//! The program the page shows (the picture, the settings, then the
//! core), the stages and the part computing each, and the pictures'
//! colours.

use microscope::colour::{pixels, ramp, Rgb};
use microscope::source::{between, listing, Range};
use yew::Html;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Picture,
    Windows,
    Smooth,
    Gradients,
    Magnitude,
    Edges,
    Pool,
}

pub const STAGES: [Stage; 7] =
    [Stage::Picture, Stage::Windows, Stage::Smooth, Stage::Gradients, Stage::Magnitude, Stage::Edges, Stage::Pool];

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Picture => between(src, "img := ", "img := "),
        Stage::Windows => between(src, "u:w_indows := ", "u:w_indows := "),
        Stage::Smooth => between(src, "smooth := ", "smooth := "),
        Stage::Gradients => between(src, "gx := ", "gy := "),
        Stage::Magnitude => between(src, "mag := ", "mag := "),
        Stage::Edges => between(src, "edges := ", "edges := "),
        Stage::Pool => between(src, "pool := ", "pool := "),
    }
}

/// The program X_eTaL ran, exactly, decorated, `focus` highlighted.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(program, focus))
}

const GREY: [Rgb; 2] = [[0, 0, 0], [255, 255, 255]];

/// Brightness 0 (black) to 1 (white).
pub fn grey(values: &[f64]) -> Vec<u8> {
    pixels(values, |v| ramp(&GREY, v))
}

/// The edges: 1 white, 0 dark violet.
pub fn edges(values: &[f64]) -> Vec<u8> {
    pixels(values, |v| if v == 1.0 { [255, 255, 255] } else { [24, 20, 48] })
}

fn hue(h: f64) -> Rgb {
    // h in 0..1 round the colour wheel, full saturation.
    let f = |n: f64| {
        let k = (n + h * 6.0) % 6.0;
        let v = 1.0 - (k.min(4.0 - k).clamp(0.0, 1.0));
        (255.0 * v) as u8
    };
    [f(5.0), f(3.0), f(1.0)]
}

/// The edges' direction as a hue (the angle of gx, gy), as bright as
/// the edge is strong.
pub fn direction(gx: &[f64], gy: &[f64], mag: &[f64]) -> Vec<u8> {
    let m = mag.iter().fold(1e-12_f64, |m, &v| m.max(v));
    let mut px = Vec::with_capacity(4 * gx.len());
    for i in 0..gx.len() {
        let a = gy[i].atan2(gx[i]) / std::f64::consts::TAU + 0.5;
        let [r, g, b] = hue(a);
        let t = (1.5 * mag[i] / m).min(1.0);
        px.extend([(r as f64 * t) as u8, (g as f64 * t) as u8, (b as f64 * t) as u8, 255]);
    }
    px
}
