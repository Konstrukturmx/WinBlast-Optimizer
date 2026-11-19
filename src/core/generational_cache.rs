//! `generational_aot_cache.rs` — O(1) Generational Ahead-of-Time Cache for WinBlast0.
//! Edition: Rust 2024 | Dependencies: std only | Unsafe blocks: 0.

use std::array;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError, RwLock};
use std::time::{Duration, Instant};

/// Number of associative ways per set (strict O(1) upper bound of 4 probes per query).
const WAYS: usize = 4;

/// Minimum interval (1 ms) between atomic `last_used_ns` updates on reads to avoid cache-line bouncing.
const LRU_UPDATE_GRANULARITY_NS: u64 = 1_000_000;

/// Cache-line padded wrapper to eliminate false sharing across CPU cores.
#[repr(align(64))]
#[derive(Debug, Default)]
struct CachePadded<T>(T);

/// Fast, zero-allocation deterministic 64-bit hasher (multiply-rotate + SplitMix64 avalanche).
struct FastAotHasher {
    state: u64,
}

impl FastAotHasher {
    const SEED: u64 = 0x517c_c1b7_2722_0a95;

    #[inline]
    const fn new() -> Self {
        Self { state: Self::SEED }
    }

    #[inline]
    fn mix_word(&mut self, word: u64) {
        self.state = (self.state.rotate_left(5) ^ word).wrapping_mul(Self::SEED);
    }
}

impl Hasher for FastAotHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let (chunks, remainder) = bytes.as_chunks::<8>();

        for &arr in chunks {
            self.mix_word(u64::from_ne_bytes(arr));
        }

        if !remainder.is_empty() {
            let mut tail = [0u8; 8];
            if let Some(dst) = tail.get_mut(..remainder.len()) {
                dst.copy_from_slice(remainder);
            }
            self.mix_word(u64::from_ne_bytes(tail));
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^= z >> 31;
        // Reserve 0 as the empty-slot sentinel.
        if z == 0 { 1 } else { z }
    }
}

#[inline]
fn hash_key<Q: Hash + ?Sized>(key: &Q) -> u64 {
    let mut hasher = FastAotHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

/// Snapshot of runtime cache telemetry counters.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CacheMetricsSnapshot {
    pub hits: u64,
    pub misses: u64,
    pub epoch_invalidations: u64,
    pub ttl_expirations: u64,
    pub evictions: u64,
}

#[derive(Debug, Default)]
struct CacheMetrics {
    hits: CachePadded<AtomicU64>,
    misses: CachePadded<AtomicU64>,
    epoch_invalidations: CachePadded<AtomicU64>,
    ttl_expirations: CachePadded<AtomicU64>,
    evictions: CachePadded<AtomicU64>,
}

impl CacheMetrics {
    fn new() -> Self {
        Self {
            hits: CachePadded(AtomicU64::new(0)),
            misses: CachePadded(AtomicU64::new(0)),
            epoch_invalidations: CachePadded(AtomicU64::new(0)),
            ttl_expirations: CachePadded(AtomicU64::new(0)),
            evictions: CachePadded(AtomicU64::new(0)),
        }
    }

    fn snapshot(&self) -> CacheMetricsSnapshot {
        CacheMetricsSnapshot {
            hits: self.hits.0.load(Ordering::Relaxed),
            misses: self.misses.0.load(Ordering::Relaxed),
            epoch_invalidations: self.epoch_invalidations.0.load(Ordering::Relaxed),
            ttl_expirations: self.ttl_expirations.0.load(Ordering::Relaxed),
            evictions: self.evictions.0.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug)]
struct SlotEntry<K, V> {
    key: K,
    value: V,
    epoch: u64,
    expires_at_ns: u64,
}

#[repr(align(64))]
#[derive(Debug)]
struct CacheSlot<K, V> {
    /// Non-zero key fingerprint; 0 means unoccupied.
    key_hash: AtomicU64,
    /// Generation epoch at which this slot was published; 0 means empty/invalidated.
    epoch: AtomicU64,
    /// Monotonic expiry offset in nanoseconds since cache creation.
    expires_at_ns: AtomicU64,
    /// Coarse-grained monotonic timestamp of last access for 4-way LRU eviction.
    last_used_ns: AtomicU64,
    /// Slot value protected by an uncontended-fast RwLock (Windows SRWLOCK).
    payload: RwLock<Option<SlotEntry<K, V>>>,
}

impl<K, V> CacheSlot<K, V> {
    fn new() -> Self {
        Self {
            key_hash: AtomicU64::new(0),
            epoch: AtomicU64::new(0),
            expires_at_ns: AtomicU64::new(0),
            last_used_ns: AtomicU64::new(0),
            payload: RwLock::new(None),
        }
    }
}

#[derive(Debug)]
struct CacheSet<K, V> {
    /// Serializes writers within this 4-way set only; readers never touch this lock.
    write_gate: CachePadded<Mutex<()>>,
    slots: [CacheSlot<K, V>; WAYS],
}

impl<K, V> CacheSet<K, V> {
    fn new() -> Self {
        Self {
            write_gate: CachePadded(Mutex::new(())),
            slots: array::from_fn(|_| CacheSlot::new()),
        }
    }
}

/// Ultra-fast, read-optimized, O(1) generational Ahead-of-Time (AOT) cache.
///
/// - **Zero heap allocation on read hits** via [`Self::get_with`] and [`Self::get_copied`].
/// - **Atomic O(1) epoch invalidation** via [`Self::invalidate_all`].
/// - **Lock-free TTL fencing** using monotonic nanosecond offsets.
#[derive(Debug)]
pub struct GenerationalAotCache<K, V> {
    sets: Box<[CacheSet<K, V>]>,
    set_mask_u64: u64,
    current_epoch: CachePadded<AtomicU64>,
    origin: Instant,
    default_ttl_ns: u64,
    metrics: CacheMetrics,
}

impl<K, V> GenerationalAotCache<K, V>
where
    K: Eq + Hash,
{
    /// Allocates a fixed-capacity AOT cache holding at least `min_capacity` entries.
    ///
    /// Actual capacity is rounded up to `WAYS * next_power_of_two(ceil(min_capacity / WAYS))`.
    #[must_use]
    pub fn new(min_capacity: usize, default_ttl: Duration) -> Self {
        let num_sets = min_capacity.div_ceil(WAYS).max(1).next_power_of_two();
        let sets = (0..num_sets)
            .map(|_| CacheSet::new())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let set_mask_u64 = u64::try_from(num_sets - 1).unwrap_or(u64::MAX);

        Self {
            sets,
            set_mask_u64,
            current_epoch: CachePadded(AtomicU64::new(1)),
            origin: Instant::now(),
            default_ttl_ns: duration_to_ns(default_ttl),
            metrics: CacheMetrics::new(),
        }
    }

    /// Pre-warms the cache Ahead-of-Time (e.g., startup hardware telemetry or registry snapshot).
    /// Returns the number of entries inserted.
    pub fn prewarm_aot<I>(&self, entries: I, ttl: Duration) -> usize
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut inserted = 0usize;
        for (key, value) in entries {
            self.insert_with_ttl(key, value, ttl);
            inserted += 1;
        }
        inserted
    }

    /// Total fixed slot capacity of the AOT slab.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.sets.len() * WAYS
    }

    /// Returns the active atomic generation epoch.
    #[must_use]
    pub fn current_epoch(&self) -> u64 {
        self.current_epoch.0.load(Ordering::Acquire)
    }

    /// Returns a snapshot of cache hit/miss/eviction metrics.
    #[must_use]
    pub fn metrics(&self) -> CacheMetricsSnapshot {
        self.metrics.snapshot()
    }

    /// Globally invalidates all cached entries in **O(1)** via a single atomic epoch bump.
    ///
    /// Returns the new active epoch.
    pub fn invalidate_all(&self) -> u64 {
        let prev = loop {
            let cur = self.current_epoch.0.load(Ordering::Acquire);
            let next = cur.wrapping_add(1);
            let candidate = if next == 0 { 1 } else { next };
            if self
                .current_epoch
                .0
                .compare_exchange_weak(cur, candidate, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                break cur;
            }
        };

        self.metrics
            .epoch_invalidations
            .0
            .fetch_add(1, Ordering::Relaxed);

        let next = prev.wrapping_add(1);
        if next == 0 { 1 } else { next }
    }

    /// Inserts or updates a key using the cache's default TTL.
    pub fn insert(&self, key: K, value: V) {
        self.insert_ns(key, value, self.default_ttl_ns);
    }

    /// Inserts or updates a key with an explicit TTL.
    pub fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) {
        self.insert_ns(key, value, duration_to_ns(ttl));
    }

    /// Zero-allocation read query: invokes `f(&V)` on a live hit and returns its result.
    ///
    /// Accepts borrowed lookup keys (e.g., `&str` for `K = String`).
    pub fn get_with<Q, R, F>(&self, key: &Q, f: F) -> Option<R>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
        F: FnOnce(&V) -> R,
    {
        let key_hash = hash_key(key);
        let active_epoch = self.current_epoch.0.load(Ordering::Acquire);
        let now_ns = self.elapsed_ns();
        let set = self.set_for_hash(key_hash)?;

        for slot in &set.slots {
            // Fast-path Fence 1: Lock-free epoch validation.
            if slot.epoch.load(Ordering::Acquire) != active_epoch {
                continue;
            }
            // Fast-path Fence 2: Lock-free hash fingerprint validation.
            if slot.key_hash.load(Ordering::Relaxed) != key_hash {
                continue;
            }
            // Fast-path Fence 3: Lock-free monotonic TTL fencing.
            if now_ns >= slot.expires_at_ns.load(Ordering::Relaxed) {
                self.metrics
                    .ttl_expirations
                    .0
                    .fetch_add(1, Ordering::Relaxed);
                continue;
            }

            // Acquire shared slot read lock and re-verify linearizability.
            let guard = slot
                .payload
                .read()
                .unwrap_or_else(PoisonError::into_inner);
            if let Some(entry) = guard.as_ref() {
                if entry.epoch == active_epoch
                    && now_ns < entry.expires_at_ns
                    && entry.key.borrow() == key
                {
                    let prev_used = slot.last_used_ns.load(Ordering::Relaxed);
                    if now_ns.saturating_sub(prev_used) >= LRU_UPDATE_GRANULARITY_NS {
                        slot.last_used_ns.store(now_ns, Ordering::Relaxed);
                    }
                    self.metrics.hits.0.fetch_add(1, Ordering::Relaxed);
                    return Some(f(&entry.value));
                }
            }
        }

        self.metrics.misses.0.fetch_add(1, Ordering::Relaxed);
        None
    }

    /// Zero-allocation read hit for `Copy` values (hardware counters, tweak bitflags, timestamps).
    #[must_use]
    pub fn get_copied<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
        V: Copy,
    {
        self.get_with(key, |&v| v)
    }

    /// Read hit returning a cloned value when `V` is not `Copy`.
    #[must_use]
    pub fn get_cloned<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
        V: Clone,
    {
        self.get_with(key, Clone::clone)
    }

    /// Targeted O(1) invalidation of a single key. Returns `true` if a live entry was removed.
    pub fn invalidate_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let key_hash = hash_key(key);
        let Some(set) = self.set_for_hash(key_hash) else {
            return false;
        };

        let _write_guard = set
            .write_gate
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner);

        for slot in &set.slots {
            if slot.key_hash.load(Ordering::Relaxed) != key_hash {
                continue;
            }
            let mut payload = slot
                .payload
                .write()
                .unwrap_or_else(PoisonError::into_inner);
            if let Some(entry) = payload.as_ref() {
                if entry.key.borrow() == key {
                    slot.epoch.store(0, Ordering::Release);
                    slot.key_hash.store(0, Ordering::Relaxed);
                    slot.expires_at_ns.store(0, Ordering::Relaxed);
                    *payload = None;
                    return true;
                }
            }
        }

        false
    }

    fn insert_ns(&self, key: K, value: V, ttl_ns: u64) {
        let key_hash = hash_key(&key);
        let Some(set) = self.set_for_hash(key_hash) else {
            return;
        };

        let _write_guard = set
            .write_gate
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner);

        let now_ns = self.elapsed_ns();
        let expires_at_ns = now_ns.saturating_add(ttl_ns);
        let active_epoch = self.current_epoch.0.load(Ordering::Acquire);

        let mut target_idx: Option<usize> = None;
        let mut reusable_idx: Option<usize> = None;
        let mut lru_idx = 0usize;
        let mut lru_ts = u64::MAX;

        // Pass 1: Locate matching key (in-place update), empty/stale slot, or LRU victim.
        for (idx, slot) in set.slots.iter().enumerate() {
            let slot_hash = slot.key_hash.load(Ordering::Relaxed);
            let slot_epoch = slot.epoch.load(Ordering::Relaxed);
            let slot_exp = slot.expires_at_ns.load(Ordering::Relaxed);

            if slot_hash == key_hash && slot_epoch != 0 {
                let guard = slot
                    .payload
                    .read()
                    .unwrap_or_else(PoisonError::into_inner);
                if guard.as_ref().is_some_and(|e| e.key == key) {
                    target_idx = Some(idx);
                    break;
                }
            }

            let is_reusable = slot_epoch == 0 || slot_epoch != active_epoch || now_ns >= slot_exp;
            if is_reusable && reusable_idx.is_none() {
                reusable_idx = Some(idx);
            }

            let used = slot.last_used_ns.load(Ordering::Relaxed);
            if used < lru_ts {
                lru_ts = used;
                lru_idx = idx;
            }
        }

        let chosen_idx = match target_idx {
            Some(idx) => idx,
            None => match reusable_idx {
                Some(idx) => idx,
                None => {
                    self.metrics.evictions.0.fetch_add(1, Ordering::Relaxed);
                    lru_idx
                }
            },
        };

        if let Some(slot) = set.slots.get(chosen_idx) {
            let mut payload = slot
                .payload
                .write()
                .unwrap_or_else(PoisonError::into_inner);

            // Re-sample active epoch under write lock so concurrent global invalidations stay linearizable.
            let publish_epoch = self.current_epoch.0.load(Ordering::Acquire);
            *payload = Some(SlotEntry {
                key,
                value,
                epoch: publish_epoch,
                expires_at_ns,
            });

            slot.key_hash.store(key_hash, Ordering::Relaxed);
            slot.expires_at_ns.store(expires_at_ns, Ordering::Relaxed);
            slot.last_used_ns.store(now_ns, Ordering::Relaxed);
            slot.epoch.store(publish_epoch, Ordering::Release);
        }
    }

    #[inline]
    fn set_for_hash(&self, key_hash: u64) -> Option<&CacheSet<K, V>> {
        let idx = usize::try_from(key_hash & self.set_mask_u64).unwrap_or(0);
        self.sets.get(idx)
    }

    #[inline]
    fn elapsed_ns(&self) -> u64 {
        u64::try_from(self.origin.elapsed().as_nanos()).unwrap_or(u64::MAX)
    }
}

#[inline]
fn duration_to_ns(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct CpuTelemetry {
        package_temp_c: u16,
        clock_mhz: u32,
        power_mw: u32,
    }

    #[test]
    fn compile_time_send_sync_check() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<GenerationalAotCache<String, CpuTelemetry>>();
    }

    #[test]
    fn aot_prewarm_and_zero_alloc_borrowed_reads() {
        let cache = GenerationalAotCache::<String, CpuTelemetry>::new(64, Duration::from_secs(60));
        assert!(cache.capacity() >= 64);

        let inserted = cache.prewarm_aot(
            [
                (
                    String::from("HKLM\\HARDWARE\\CPU0"),
                    CpuTelemetry {
                        package_temp_c: 54,
                        clock_mhz: 5200,
                        power_mw: 65_000,
                    },
                ),
                (
                    String::from("HKLM\\HARDWARE\\CPU1"),
                    CpuTelemetry {
                        package_temp_c: 52,
                        clock_mhz: 5150,
                        power_mw: 62_000,
                    },
                ),
            ],
            Duration::from_secs(30),
        );
        assert_eq!(inserted, 2);

        // Borrowed `&str` query against `String` key: zero heap allocation on hit.
        let mhz = cache.get_with("HKLM\\HARDWARE\\CPU0", |t| t.clock_mhz);
        assert_eq!(mhz, Some(5200));

        let copied = cache.get_copied("HKLM\\HARDWARE\\CPU1");
        assert_eq!(copied.map(|t| t.package_temp_c), Some(52));
        assert_eq!(cache.metrics().hits, 2);
    }

    #[test]
    fn atomic_epoch_invalidation_fences_all_entries_in_o1() {
        let cache = GenerationalAotCache::<u64, u64>::new(32, Duration::from_secs(60));
        for k in 0..32u64 {
            cache.insert(k, k * 10);
        }
        assert_eq!(cache.get_copied(&7), Some(70));

        let new_epoch = cache.invalidate_all();
        assert_eq!(new_epoch, 2);
        assert_eq!(cache.current_epoch(), 2);

        // Every prior key is immediately fenced off without scanning slots.
        for k in 0..32u64 {
            assert_eq!(cache.get_copied(&k), None);
        }

        // Subsequent writes publish into the new generation cleanly.
        cache.insert(7, 999);
        assert_eq!(cache.get_copied(&7), Some(999));
    }

    #[test]
    fn ttl_fencing_and_targeted_invalidation() {
        let cache = GenerationalAotCache::<&'static str, bool>::new(16, Duration::from_millis(20));
        cache.insert_with_ttl("tweak.game_mode", true, Duration::from_millis(15));
        cache.insert_with_ttl("tweak.hpet", false, Duration::from_secs(30));

        assert_eq!(cache.get_copied(&"tweak.game_mode"), Some(true));
        thread::sleep(Duration::from_millis(25));
        assert_eq!(cache.get_copied(&"tweak.game_mode"), None);
        assert_eq!(cache.get_copied(&"tweak.hpet"), Some(false));

        assert!(cache.invalidate_key(&"tweak.hpet"));
        assert_eq!(cache.get_copied(&"tweak.hpet"), None);
        assert!(!cache.invalidate_key(&"tweak.hpet"));
    }

    #[test]
    fn four_way_lru_eviction_preserves_o1_bounds() {
        // Single 4-way set (capacity = 4) forces deterministic eviction on the 5th key.
        let cache = GenerationalAotCache::<u64, u64>::new(4, Duration::from_secs(60));
        assert_eq!(cache.capacity(), 4);

        for k in 0..8u64 {
            cache.insert(k, k * 2);
        }
        assert!(cache.metrics().evictions >= 4);
    }

    #[test]
    fn concurrent_reads_writes_and_epoch_bumps() {
        let cache = Arc::new(GenerationalAotCache::<u64, u64>::new(
            128,
            Duration::from_secs(10),
        ));

        let mut handles = Vec::new();
        for tid in 0..4u64 {
            let shared = Arc::clone(&cache);
            handles.push(thread::spawn(move || {
                for step in 0..1_000u64 {
                    let key = (tid * 31 + step) % 64;
                    shared.insert(key, key.wrapping_mul(3));
                    if let Some(val) = shared.get_copied(&key) {
                        assert_eq!(val, key.wrapping_mul(3));
                    }
                    if step % 250 == 0 {
                        let _ = shared.invalidate_all();
                    }
                }
            }));
        }

        for handle in handles {
            assert!(handle.join().is_ok());
        }
    }
}