//! Coverage curve.
//!
//! An exact-area rasterizer gives a one-pixel stem that falls between two pixels two
//! half-covered pixels, which reads as a pale grey smear. Raising mid-range coverage with a
//! gamma-style curve (the idea behind FreeType's stem darkening and DirectWrite's enhanced
//! contrast) keeps thin strokes dark while leaving fully covered and empty pixels unchanged.

/// Gamma applied at contrast 100. Contrast 0 is the identity.
const MAX_GAMMA: f32 = 1.9;

/// 256-entry lookup table: input coverage (0..=255) to output coverage.
pub fn lut(contrast: u8) -> [u8; 256] {
    let g = 1.0 + (MAX_GAMMA - 1.0) * (contrast.min(100) as f32 / 100.0);
    let mut t = [0u8; 256];
    for (i, v) in t.iter_mut().enumerate() {
        let c = i as f32 / 255.0;
        *v = (c.powf(1.0 / g) * 255.0 + 0.5).clamp(0.0, 255.0) as u8;
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoints_are_fixed_and_curve_is_monotonic() {
        for contrast in [0u8, 35, 100, 250] {
            let t = lut(contrast);
            assert_eq!(t[0], 0);
            assert_eq!(t[255], 255);
            assert!(t.windows(2).all(|w| w[0] <= w[1]), "contrast {contrast} not monotonic");
        }
    }

    #[test]
    fn zero_contrast_is_identity_and_more_contrast_is_darker() {
        let id = lut(0);
        assert!(id.iter().enumerate().all(|(i, &v)| v as usize == i));
        let mid = lut(35)[128];
        let high = lut(100)[128];
        assert!(mid > 128 && high > mid, "128 -> {mid} (35) -> {high} (100)");
    }
}
