//! Shelf packer for the glyph texture atlas.

#[derive(Clone, Debug)]
struct Shelf {
    y: u32,
    height: u32,
    x: u32,
}

/// Packs rectangles into rows ("shelves"). Glyphs of one size have similar heights, so shelves
/// fill densely; a full atlas is reset as a whole.
#[derive(Clone, Debug)]
pub struct ShelfPacker {
    width: u32,
    height: u32,
    shelves: Vec<Shelf>,
}

/// Empty pixels kept between glyphs so neighbours never bleed into each other.
pub const PADDING: u32 = 1;

impl ShelfPacker {
    pub fn new(width: u32, height: u32) -> Self {
        ShelfPacker { width, height, shelves: Vec::new() }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Top-left corner for a `w` x `h` rectangle, or None when the atlas is full.
    pub fn allocate(&mut self, w: u32, h: u32) -> Option<(u32, u32)> {
        let (pw, ph) = (w + PADDING, h + PADDING);
        if pw > self.width || ph > self.height {
            return None;
        }
        // Best fit: the lowest shelf that is tall enough without wasting more than half its height.
        let best = self
            .shelves
            .iter_mut()
            .filter(|s| s.height >= ph && s.height <= ph + ph / 2 + 2 && s.x + pw <= self.width)
            .min_by_key(|s| s.height);
        if let Some(shelf) = best {
            let pos = (shelf.x, shelf.y);
            shelf.x += pw;
            return Some(pos);
        }
        let y = self.shelves.last().map_or(0, |s| s.y + s.height);
        if y + ph > self.height {
            return None;
        }
        self.shelves.push(Shelf { y, height: ph, x: pw });
        Some((0, y))
    }

    pub fn reset(&mut self) {
        self.shelves.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocations_never_overlap_and_stay_inside() {
        let mut p = ShelfPacker::new(256, 256);
        let mut rects: Vec<(u32, u32, u32, u32)> = vec![];
        let mut seed = 7u32;
        loop {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            let w = 2 + (seed >> 16) % 20;
            let h = 4 + (seed >> 8) % 18;
            match p.allocate(w, h) {
                Some((x, y)) => {
                    assert!(x + w <= 256 && y + h <= 256);
                    for &(rx, ry, rw, rh) in &rects {
                        let apart = x + w <= rx || rx + rw <= x || y + h <= ry || ry + rh <= y;
                        assert!(apart, "({x},{y},{w},{h}) overlaps ({rx},{ry},{rw},{rh})");
                    }
                    rects.push((x, y, w, h));
                }
                None => break,
            }
        }
        assert!(rects.len() > 100, "only {} glyphs fit", rects.len());
    }

    #[test]
    fn too_big_fails_and_reset_frees_space() {
        let mut p = ShelfPacker::new(32, 32);
        assert!(p.allocate(40, 4).is_none());
        assert!(p.allocate(30, 30).is_some());
        assert!(p.allocate(30, 30).is_none());
        p.reset();
        assert_eq!(p.allocate(30, 30), Some((0, 0)));
    }
}
