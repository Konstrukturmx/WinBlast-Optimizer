/// Fixed-capacity schedule metadata (no heap allocation during HUD execution).
pub const MAX_BATCHES: usize = 32;

#[derive(Clone, Copy, Debug, Default)]
pub struct BatchDescriptor {
    pub step_count: u16,
    pub total_weight: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct HudFrame {
    pub progress: f32,
    pub batch_index: u16,
    pub batch_total: u16,
    pub step_index: u16,
    pub step_total: u16,
    pub total_elapsed_secs: f64,
    pub completed: bool,
}

pub struct MonotonicCubicProgress {
    batches: [BatchDescriptor; MAX_BATCHES],
    batch_total: u16,
    step_total: u16,
    total_weight: f64,

    // Discrete schedule cursors
    completed_weight: f64,
    active_step_weight: f64,
    active_step_est_secs: f64,
    active_step_elapsed: f64,
    current_batch: u16,
    completed_steps: u16,
    finished: bool,

    // Monotonic cubic Hermite segment state
    seg_start: f64,
    seg_target: f64,
    seg_m0: f64,
    seg_elapsed: f64,
    seg_duration: f64,

    // Monotonic guards
    logical_target: f64,
    display_progress: f64,
    high_water_mark: f64,
    total_elapsed_secs: f64,
}

impl MonotonicCubicProgress {
    /// Initializes a new batched multi-step schedule.
    pub fn start(batches_slice: &[BatchDescriptor]) -> Self {
        let count = batches_slice.len().min(MAX_BATCHES);
        let mut batches = [BatchDescriptor::default(); MAX_BATCHES];
        let mut step_total: u16 = 0;
        let mut weight_sum: u64 = 0;

        for (i, b) in batches_slice.iter().take(count).enumerate() {
            batches[i] = *b;
            step_total = step_total.saturating_add(b.step_count);
            weight_sum = weight_sum.saturating_add(b.total_weight as u64);
        }

        Self {
            batches,
            batch_total: count as u16,
            step_total,
            total_weight: (weight_sum.max(1)) as f64,
            completed_weight: 0.0,
            active_step_weight: 0.0,
            active_step_est_secs: 1.5,
            active_step_elapsed: 0.0,
            current_batch: 0,
            completed_steps: 0,
            finished: count == 0 || step_total == 0,
            seg_start: 0.0,
            seg_target: 0.0,
            seg_m0: 0.0,
            seg_elapsed: 0.0,
            seg_duration: 0.35,
            logical_target: 0.0,
            display_progress: 0.0,
            high_water_mark: 0.0,
            total_elapsed_secs: 0.0,
        }
    }

    /// Called when a step inside `batch_idx` begins executing.
    pub fn begin_step(&mut self, batch_idx: u16, step_weight: u32, estimated_secs: f64) {
        if self.finished {
            return;
        }
        self.current_batch = batch_idx.min(self.batch_total.saturating_sub(1));
        self.active_step_weight = (step_weight.max(1)) as f64;
        self.active_step_est_secs = estimated_secs.clamp(0.15, 30.0);
        self.active_step_elapsed = 0.0;
    }

    /// Called when the active step completes. Advances the target monotonically and retargets
    /// the cubic Hermite spline with $C^1$ velocity preservation.
    pub fn complete_step(&mut self, completed_step_weight: u32) {
        if self.finished {
            return;
        }
        self.completed_steps = self.completed_steps.saturating_add(1).min(self.step_total);
        self.completed_weight =
            (self.completed_weight + (completed_step_weight.max(1)) as f64).min(self.total_weight);
        self.active_step_weight = 0.0;
        self.active_step_elapsed = 0.0;

        let raw_target = (self.completed_weight / self.total_weight).clamp(0.0, 1.0);
        self.retarget(raw_target, false);
    }

    /// Marks the entire batched schedule complete, easing cleanly to `1.0`.
    pub fn finish(&mut self) {
        self.finished = true;
        self.completed_steps = self.step_total;
        self.completed_weight = self.total_weight;
        self.active_step_weight = 0.0;
        self.retarget(1.0, true);
    }

    /// Advances the animation by `dt_secs` (e.g., 1/60 s or 1/120 s) and returns the HUD frame.
    pub fn tick(&mut self, dt_secs: f64) -> HudFrame {
        if dt_secs.is_finite() && dt_secs > 0.0 {
            let dt = dt_secs.min(0.25); // Guard against OS sleep / debugger pause spikes.
            self.total_elapsed_secs += dt;

            // Bounded asymptotic trickle for long-running active steps (capped at 85% of step weight).
            if !self.finished && self.active_step_weight > 0.0 {
                self.active_step_elapsed += dt;
                let tau = self.active_step_est_secs;
                let u = 0.85 * (self.active_step_elapsed / (self.active_step_elapsed + tau));
                let trickle_target = ((self.completed_weight + self.active_step_weight * u)
                    / self.total_weight)
                    .clamp(0.0, 0.995);

                if trickle_target > self.logical_target + 1e-4 {
                    self.retarget(trickle_target, false);
                }
            }

            self.seg_elapsed = (self.seg_elapsed + dt).min(self.seg_duration);
            let evaluated = self.eval_hermite();
            // IEEE-754 ULP floor guarantees strict monotonicity.
            self.display_progress = evaluated.clamp(self.high_water_mark, 1.0);
            self.high_water_mark = self.display_progress;
        }

        HudFrame {
            progress: self.display_progress as f32,
            batch_index: self.current_batch.saturating_add(1).min(self.batch_total),
            batch_total: self.batch_total,
            step_index: self.completed_steps,
            step_total: self.step_total,
            total_elapsed_secs: self.total_elapsed_secs,
            completed: self.finished && self.display_progress >= 0.9995,
        }
    }

    /// Batch descriptor accessor for HUD diagnostics.
    #[inline]
    pub fn batch_info(&self, idx: usize) -> Option<BatchDescriptor> {
        (idx < self.batch_total as usize).then_some(self.batches[idx])
    }

    #[inline]
    fn eval_hermite(&self) -> f64 {
        if self.seg_duration <= 1e-9 {
            return self.seg_target;
        }
        let s = (self.seg_elapsed / self.seg_duration).clamp(0.0, 1.0);
        let s2 = s * s;
        let s3 = s2 * s;
        let delta = self.seg_target - self.seg_start;
        self.seg_start + (3.0 * s2 - 2.0 * s3) * delta + (s3 - 2.0 * s2 + s) * self.seg_m0
    }

    #[inline]
    fn current_velocity(&self) -> f64 {
        if self.seg_duration <= 1e-9 || self.seg_elapsed >= self.seg_duration {
            return 0.0;
        }
        let s = (self.seg_elapsed / self.seg_duration).clamp(0.0, 1.0);
        let s2 = s * s;
        let delta = self.seg_target - self.seg_start;
        let dh_ds = (6.0 * s - 6.0 * s2) * delta + (3.0 * s2 - 4.0 * s + 1.0) * self.seg_m0;
        (dh_ds / self.seg_duration).max(0.0)
    }

    fn retarget(&mut self, new_target: f64, is_final: bool) {
        let clamped_target = new_target.clamp(self.logical_target, 1.0);
        if clamped_target <= self.logical_target + 1e-9 {
            return;
        }

        let p0 = self.eval_hermite().clamp(self.high_water_mark, 1.0);
        let v0 = self.current_velocity();
        let delta = (clamped_target - p0).max(0.0);

        let duration = if is_final {
            (0.14 + 0.30 * delta).clamp(0.14, 0.30)
        } else {
            (0.18 + 0.55 * delta).clamp(0.18, 0.55)
        };

        // Fritsch–Carlson monotonicity bound: 0 <= m0 <= 3 * delta guarantees H'(s) >= 0 on [0, 1].
        let m0 = (v0 * duration).clamp(0.0, 3.0 * delta);

        self.logical_target = clamped_target;
        self.seg_start = p0;
        self.seg_target = clamped_target;
        self.seg_m0 = m0;
        self.seg_elapsed = 0.0;
        self.seg_duration = duration;
    }
}
