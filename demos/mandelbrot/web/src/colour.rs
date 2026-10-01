//! This demo's palettes, on the shared ramps.

use microscope::colour::{pixels, ramp, GLOW};

const MAGNITUDE: [[u8; 3]; 4] = [[20, 20, 40], [88, 44, 160], [255, 179, 102], [255, 250, 230]];

/// Escape counts 0..k: the points still inside after k steps are black.
pub fn escape(counts: &[f64], k: usize) -> Vec<u8> {
    let k = k.max(1) as f64;
    pixels(counts, |c| if c >= k { [0, 0, 0] } else { ramp(&GLOW, (c / k).sqrt()) })
}

/// |z|^2: 0 dark, 4 (the edge of the disk) bright, beyond it white.
pub fn magnitude(values: &[f64]) -> Vec<u8> {
    pixels(values, |v| if v > 4.0 { [255, 255, 255] } else { ramp(&MAGNITUDE, v / 4.0) })
}

pub use microscope::colour::{mask, signed};
