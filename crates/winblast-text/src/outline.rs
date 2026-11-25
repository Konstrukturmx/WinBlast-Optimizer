//! Glyph outlines to line segments.

use crate::raster::{Flattener, Point};
use skrifa::outline::OutlinePen;

/// Collects a glyph outline (font pixel space, y up) as flattened line segments.
#[derive(Default)]
pub struct SegmentPen {
    pub segments: Vec<(Point, Point)>,
    start: Option<Point>,
    current: Point,
}

impl SegmentPen {
    fn push(&mut self, to: Point) {
        if to != self.current {
            self.segments.push((self.current, to));
        }
        self.current = to;
    }

    fn close_contour(&mut self) {
        if let Some(start) = self.start.take() {
            self.push(start);
        }
    }

    /// Bounding box (min_x, min_y, max_x, max_y), or None for an empty outline.
    pub fn bounds(&self) -> Option<(f32, f32, f32, f32)> {
        let mut it = self.segments.iter().flat_map(|(a, b)| [*a, *b]);
        let first = it.next()?;
        Some(it.fold((first.x, first.y, first.x, first.y), |(x0, y0, x1, y1), p| {
            (x0.min(p.x), y0.min(p.y), x1.max(p.x), y1.max(p.y))
        }))
    }
}

impl OutlinePen for SegmentPen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.close_contour();
        self.current = Point::new(x, y);
        self.start = Some(self.current);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.push(Point::new(x, y));
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        let p0 = self.current;
        let mut segs = Vec::new();
        Flattener::quad(p0, Point::new(cx0, cy0), Point::new(x, y), &mut |a, b| segs.push((a, b)));
        self.segments.extend(segs);
        self.current = Point::new(x, y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let p0 = self.current;
        let mut segs = Vec::new();
        Flattener::cubic(p0, Point::new(cx0, cy0), Point::new(cx1, cy1), Point::new(x, y), &mut |a, b| segs.push((a, b)));
        self.segments.extend(segs);
        self.current = Point::new(x, y);
    }

    fn close(&mut self) {
        self.close_contour();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_contours_are_closed_and_bounds_cover_all_points() {
        let mut pen = SegmentPen::default();
        pen.move_to(0.0, 0.0);
        pen.line_to(4.0, 0.0);
        pen.line_to(4.0, 3.0);
        pen.move_to(10.0, 10.0); // implicit close of the first contour
        pen.quad_to(12.0, 14.0, 14.0, 10.0);
        pen.close();
        let firsts = pen.segments.iter().filter(|(_, b)| *b == Point::new(0.0, 0.0)).count();
        assert_eq!(firsts, 1, "first contour closed back to its start");
        let (x0, y0, x1, y1) = pen.bounds().unwrap();
        assert_eq!((x0, y0, x1), (0.0, 0.0, 14.0));
        assert!((y1 - 12.0).abs() < 0.1, "curve top {y1}");
    }
}
