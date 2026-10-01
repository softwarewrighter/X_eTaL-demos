//! Arrays to RGBA pixels.

fn mix(a: [u8; 3], b: [u8; 3], t: f64) -> [u8; 3] {
    let f = |i: usize| (a[i] as f64 + (b[i] as f64 - a[i] as f64) * t).round() as u8;
    [f(0), f(1), f(2)]
}

fn ramp(stops: &[[u8; 3]], t: f64) -> [u8; 3] {
    let t = t.clamp(0.0, 1.0) * (stops.len() - 1) as f64;
    let i = (t.floor() as usize).min(stops.len() - 2);
    mix(stops[i], stops[i + 1], t - i as f64)
}

const CHEM: [[u8; 3]; 5] = [[14, 12, 38], [64, 36, 140], [151, 117, 250], [255, 179, 102], [255, 248, 225]];
const DIVERGE: [[u8; 3]; 3] = [[37, 99, 235], [245, 245, 245], [220, 38, 38]];

fn pixels(values: &[f64], colour: impl Fn(f64) -> [u8; 3]) -> Vec<u8> {
    values.iter().flat_map(|&v| { let [r, g, b] = colour(v); [r, g, b, 255] }).collect()
}

/// Values from lo (dark) to hi (bright).
pub fn field(values: &[f64], lo: f64, hi: f64) -> Vec<u8> {
    pixels(values, |v| ramp(&CHEM, (v - lo) / (hi - lo)))
}

/// Signed values: blue below 0, red above, scaled by the largest size.
pub fn signed(values: &[f64]) -> Vec<u8> {
    let m = values.iter().fold(1e-12_f64, |m, v| m.max(v.abs()));
    pixels(values, |v| ramp(&DIVERGE, 0.5 + v / (2.0 * m)))
}

/// Non-negative values scaled by the largest.
pub fn scaled(values: &[f64]) -> Vec<u8> {
    let m = values.iter().fold(1e-12_f64, |m, &v| m.max(v));
    field(values, 0.0, m)
}
