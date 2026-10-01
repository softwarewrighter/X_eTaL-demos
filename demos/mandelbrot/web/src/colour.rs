//! Arrays to RGBA pixels: escape counts, signed values, magnitudes, masks.

fn mix(a: [u8; 3], b: [u8; 3], t: f64) -> [u8; 3] {
    let f = |i: usize| (a[i] as f64 + (b[i] as f64 - a[i] as f64) * t).round() as u8;
    [f(0), f(1), f(2)]
}

/// A colour from stops spread evenly over 0..1.
fn ramp(stops: &[[u8; 3]], t: f64) -> [u8; 3] {
    let t = t.clamp(0.0, 1.0) * (stops.len() - 1) as f64;
    let i = (t.floor() as usize).min(stops.len() - 2);
    mix(stops[i], stops[i + 1], t - i as f64)
}

const ESCAPE: [[u8; 3]; 5] = [[12, 10, 40], [88, 44, 160], [177, 151, 252], [255, 179, 102], [255, 244, 214]];
const DIVERGE: [[u8; 3]; 3] = [[37, 99, 235], [245, 245, 245], [220, 38, 38]];
const MAGNITUDE: [[u8; 3]; 4] = [[20, 20, 40], [88, 44, 160], [255, 179, 102], [255, 250, 230]];

fn pixels(values: &[f64], colour: impl Fn(f64) -> [u8; 3]) -> Vec<u8> {
    values.iter().flat_map(|&v| { let [r, g, b] = colour(v); [r, g, b, 255] }).collect()
}

/// Escape counts 0..k: the points still inside after k steps are black.
pub fn escape(counts: &[f64], k: usize) -> Vec<u8> {
    let k = k.max(1) as f64;
    pixels(counts, |c| if c >= k { [0, 0, 0] } else { ramp(&ESCAPE, (c / k).sqrt()) })
}

/// Signed values: blue below 0, red above, scaled by the largest size.
pub fn signed(values: &[f64]) -> Vec<u8> {
    let m = values.iter().fold(1e-12_f64, |m, v| m.max(v.abs()));
    pixels(values, |v| ramp(&DIVERGE, 0.5 + v / (2.0 * m)))
}

/// |z|^2: 0 dark, 4 (the edge of the disk) and beyond bright.
pub fn magnitude(values: &[f64]) -> Vec<u8> {
    pixels(values, |v| if v > 4.0 { [255, 255, 255] } else { ramp(&MAGNITUDE, v / 4.0) })
}

/// A 0 / 1 mask: 1 dark (inside), 0 light.
pub fn mask(values: &[f64]) -> Vec<u8> {
    pixels(values, |v| if v == 1.0 { [24, 20, 48] } else { [229, 219, 255] })
}
