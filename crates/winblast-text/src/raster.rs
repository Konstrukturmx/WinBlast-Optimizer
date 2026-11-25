//! Exact-area coverage rasterizer.
//!
//! Each line segment adds its signed area contribution to the cells it crosses; a running sum
//! along the buffer then gives every pixel's covered area. Contributions from a closed contour
//! cancel at the end of each row, so the running sum can carry across rows. Overlapping
//! contours of the same direction add up and are clamped to 1, which is non-zero filling
//! (variable fonts are drawn with overlaps, so even-odd would leave holes).

/// A point in bitmap space: x to the right, y down, 1 unit = 1 pixel.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

pub struct Rasterizer {
    width: usize,
    height: usize,
    acc: Vec<f32>,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        // Two spare cells: a segment ending exactly on the right edge writes one past the row.
        Rasterizer { width, height, acc: vec![0.0; width * height + 2] }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Add the signed area of one edge. Points outside the bitmap are clamped to it.
    pub fn line(&mut self, a: Point, b: Point) {
        let w = self.width as f32;
        let h = self.height as f32;
        let a = Point::new(a.x.clamp(0.0, w), a.y.clamp(0.0, h));
        let b = Point::new(b.x.clamp(0.0, w), b.y.clamp(0.0, h));
        if a.y == b.y {
            return; // horizontal edges enclose no area
        }
        let (dir, p0, p1) = if a.y < b.y { (1.0f32, a, b) } else { (-1.0f32, b, a) };
        let dxdy = (p1.x - p0.x) / (p1.y - p0.y);
        let mut x = p0.x;
        let row_start = p0.y as usize;
        let row_end = (p1.y.ceil() as usize).min(self.height);
        for row in row_start..row_end {
            let top = (row as f32).max(p0.y);
            let bottom = ((row + 1) as f32).min(p1.y);
            let dy = bottom - top;
            let x_next = x + dxdy * dy;
            let d = dy * dir;
            let (x0, x1) = if x < x_next { (x, x_next) } else { (x_next, x) };
            self.span(row * self.width, x0, x1, (x + x_next) * 0.5, d);
            x = x_next;
        }
    }

    /// Distribute the area `d` of one row-slice of an edge that runs from `x0` to `x1`.
    fn span(&mut self, line: usize, x0: f32, x1: f32, x_mid: f32, d: f32) {
        let x0_floor = x0.floor();
        let x0i = x0_floor as usize;
        let x1_ceil = x1.ceil();
        let x1i = x1_ceil as usize;
        let acc = &mut self.acc;
        if x1i <= x0i + 1 {
            // The slice stays inside one pixel column.
            let frac = x_mid - x0_floor;
            acc[line + x0i] += d - d * frac;
            acc[line + x0i + 1] += d * frac;
            return;
        }
        // The slice crosses several columns: split its area by the trapezoids it cuts.
        let inv = (x1 - x0).recip();
        let x0f = x0 - x0_floor;
        let a0 = 0.5 * inv * (1.0 - x0f) * (1.0 - x0f);
        let x1f = x1 - x1_ceil + 1.0;
        let a_end = 0.5 * inv * x1f * x1f;
        acc[line + x0i] += d * a0;
        if x1i == x0i + 2 {
            acc[line + x0i + 1] += d * (1.0 - a0 - a_end);
        } else {
            let a1 = inv * (1.5 - x0f);
            acc[line + x0i + 1] += d * (a1 - a0);
            for xi in x0i + 2..x1i - 1 {
                acc[line + xi] += d * inv;
            }
            let a2 = a1 + (x1i - x0i - 3) as f32 * inv;
            acc[line + x1i - 1] += d * (1.0 - a2 - a_end);
        }
        acc[line + x1i] += d * a_end;
    }

    /// Coverage per pixel (0.0 ..= 1.0), row-major.
    pub fn coverage(&self) -> Vec<f32> {
        let mut sum = 0.0f32;
        self.acc[..self.width * self.height]
            .iter()
            .map(|a| {
                sum += a;
                sum.abs().min(1.0)
            })
            .collect()
    }

    /// Coverage as 8-bit alpha through a contrast lookup table.
    pub fn alpha(&self, lut: &[u8; 256]) -> Vec<u8> {
        self.coverage().into_iter().map(|c| lut[(c * 255.0 + 0.5) as usize]).collect()
    }
}

/// Flattens curves into line segments that stay within `TOLERANCE` pixels of the curve.
pub struct Flattener;

/// Maximum distance between a curve and its line approximation, in pixels.
pub const TOLERANCE: f32 = 0.02;

impl Flattener {
    pub fn quad(p0: Point, p1: Point, p2: Point, out: &mut impl FnMut(Point, Point)) {
        let ddx = p0.x - 2.0 * p1.x + p2.x;
        let ddy = p0.y - 2.0 * p1.y + p2.y;
        let dd = (ddx * ddx + ddy * ddy).sqrt();
        // Uniform steps: |B''| = 2 dd, so each chord is off by at most dd / (4 n^2).
        let n = ((dd / (4.0 * TOLERANCE)).sqrt().ceil() as usize).clamp(1, 64);
        let mut prev = p0;
        for i in 1..=n {
            let t = i as f32 / n as f32;
            let mt = 1.0 - t;
            let p = Point::new(
                mt * mt * p0.x + 2.0 * mt * t * p1.x + t * t * p2.x,
                mt * mt * p0.y + 2.0 * mt * t * p1.y + t * t * p2.y,
            );
            out(prev, p);
            prev = p;
        }
    }

    pub fn cubic(p0: Point, p1: Point, p2: Point, p3: Point, out: &mut impl FnMut(Point, Point)) {
        let d1 = ((p0.x - 2.0 * p1.x + p2.x).powi(2) + (p0.y - 2.0 * p1.y + p2.y).powi(2)).sqrt();
        let d2 = ((p1.x - 2.0 * p2.x + p3.x).powi(2) + (p1.y - 2.0 * p2.y + p3.y).powi(2)).sqrt();
        let dd = d1.max(d2);
        // Uniform steps: the error of each chord is at most 3 dd / (4 n^2).
        let n = ((3.0 * dd / (4.0 * TOLERANCE)).sqrt().ceil() as usize).clamp(1, 96);
        let mut prev = p0;
        for i in 1..=n {
            let t = i as f32 / n as f32;
            let mt = 1.0 - t;
            let a = mt * mt * mt;
            let b = 3.0 * mt * mt * t;
            let c = 3.0 * mt * t * t;
            let d = t * t * t;
            let p = Point::new(
                a * p0.x + b * p1.x + c * p2.x + d * p3.x,
                a * p0.y + b * p1.y + c * p2.y + d * p3.y,
            );
            out(prev, p);
            prev = p;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn polygon(r: &mut Rasterizer, pts: &[(f32, f32)]) {
        for i in 0..pts.len() {
            let a = pts[i];
            let b = pts[(i + 1) % pts.len()];
            r.line(Point::new(a.0, a.1), Point::new(b.0, b.1));
        }
    }

    fn total(c: &[f32]) -> f32 {
        c.iter().sum()
    }

    #[test]
    fn pixel_aligned_square_is_fully_covered_inside_and_empty_outside() {
        let mut r = Rasterizer::new(8, 8);
        polygon(&mut r, &[(2.0, 2.0), (6.0, 2.0), (6.0, 6.0), (2.0, 6.0)]);
        let c = r.coverage();
        for y in 0..8 {
            for x in 0..8 {
                let inside = (2..6).contains(&x) && (2..6).contains(&y);
                let v = c[y * 8 + x];
                assert!((v - if inside { 1.0 } else { 0.0 }).abs() < 1e-5, "({x},{y}) = {v}");
            }
        }
    }

    #[test]
    fn coverage_equals_exact_area_for_fractional_square() {
        let mut r = Rasterizer::new(10, 10);
        polygon(&mut r, &[(1.25, 1.5), (7.75, 1.5), (7.75, 6.25), (1.25, 6.25)]);
        let area = 6.5 * 4.75;
        assert!((total(&r.coverage()) - area).abs() < 1e-3);
        // An edge at x = 1.25 covers three quarters of column 1.
        assert!((r.coverage()[3 * 10 + 1] - 0.75).abs() < 1e-4);
    }

    #[test]
    fn triangle_area_is_exact_and_winding_does_not_matter() {
        for pts in [[(1.0, 1.0), (9.0, 2.0), (4.0, 8.5)], [(4.0, 8.5), (9.0, 2.0), (1.0, 1.0)]] {
            let mut r = Rasterizer::new(12, 12);
            polygon(&mut r, &pts);
            let (a, b, c) = (pts[0], pts[1], pts[2]);
            let area = ((b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1)).abs() / 2.0;
            assert!((total(&r.coverage()) - area).abs() < 1e-3, "{} vs {area}", total(&r.coverage()));
        }
    }

    #[test]
    fn hole_with_opposite_winding_is_empty_and_overlap_is_clamped() {
        let mut r = Rasterizer::new(10, 10);
        polygon(&mut r, &[(1.0, 1.0), (9.0, 1.0), (9.0, 9.0), (1.0, 9.0)]);
        polygon(&mut r, &[(3.0, 3.0), (3.0, 7.0), (7.0, 7.0), (7.0, 3.0)]);
        let c = r.coverage();
        assert!(c[5 * 10 + 5] < 1e-5, "hole is filled");
        assert!((c[2 * 10 + 2] - 1.0).abs() < 1e-5);

        let mut r = Rasterizer::new(10, 10);
        polygon(&mut r, &[(1.0, 1.0), (6.0, 1.0), (6.0, 6.0), (1.0, 6.0)]);
        polygon(&mut r, &[(3.0, 3.0), (8.0, 3.0), (8.0, 8.0), (3.0, 8.0)]);
        let c = r.coverage();
        assert!((c[4 * 10 + 4] - 1.0).abs() < 1e-5, "same-direction overlap must stay 1");
        assert!(c.iter().all(|v| (0.0..=1.0).contains(v)));
    }

    #[test]
    fn points_outside_the_bitmap_are_clamped_without_panicking() {
        let mut r = Rasterizer::new(4, 4);
        polygon(&mut r, &[(-3.0, -2.0), (9.0, -1.0), (7.0, 9.0), (-2.0, 6.0)]);
        let c = r.coverage();
        assert!(c.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)));
        assert!((total(&c) - 16.0).abs() < 1e-3, "fully covered 4x4");
    }

    #[test]
    fn flattened_circle_area_matches_pi_r_squared() {
        // Four cubic arcs (k = 0.5523) approximate a circle of radius 20 to within 0.03 %.
        let (cx, cy, rr) = (24.0f32, 24.0f32, 20.0f32);
        let k = 0.552_284_8 * rr;
        let p = |x: f32, y: f32| Point::new(cx + x, cy + y);
        let arcs = [
            (p(rr, 0.0), p(rr, k), p(k, rr), p(0.0, rr)),
            (p(0.0, rr), p(-k, rr), p(-rr, k), p(-rr, 0.0)),
            (p(-rr, 0.0), p(-rr, -k), p(-k, -rr), p(0.0, -rr)),
            (p(0.0, -rr), p(k, -rr), p(rr, -k), p(rr, 0.0)),
        ];
        let mut r = Rasterizer::new(48, 48);
        for (a, b, c, d) in arcs {
            Flattener::cubic(a, b, c, d, &mut |s, e| r.line(s, e));
        }
        let area = std::f32::consts::PI * rr * rr;
        let got = total(&r.coverage());
        assert!((got - area).abs() / area < 0.002, "circle area {got} vs {area}");
    }

    #[test]
    fn quad_flattening_stays_within_tolerance() {
        let (p0, p1, p2) = (Point::new(0.0, 0.0), Point::new(10.0, 20.0), Point::new(20.0, 0.0));
        let mut segs = vec![];
        Flattener::quad(p0, p1, p2, &mut |a, b| segs.push((a, b)));
        assert!(segs.len() > 1);
        for (a, b) in segs {
            // midpoint of each chord vs. the curve point at the same parameter is close
            let m = Point::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
            let t = m.x / 20.0;
            let curve_y = 2.0 * (1.0 - t) * t * 20.0;
            assert!((curve_y - m.y).abs() < 0.1, "chord off by {}", (curve_y - m.y).abs());
        }
    }
}
