//! The stages, the part of the program computing each, and colours.

use microscope::colour::{pixels, ramp, Rgb, GLOW};
use microscope::source::{between, find, listing, Range};
use yew::Html;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Edge,
    Topple,
    Give,
    Keep,
    Count,
}

pub const STAGES: [Stage; 5] = [Stage::Edge, Stage::Topple, Stage::Give, Stage::Keep, Stage::Count];

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Edge => between(src, "row := ", "inside := "),
        Stage::Topple => find(src, "q := h d_iv 4"),
        Stage::Give => between(src, "g := (1 o_-_1 q)", "g := (1 o_-_1 q)"),
        Stage::Keep => between(src, "inside * (h - 4 * q) + g", "inside * (h - 4 * q) + g"),
        Stage::Count => between(src, "u:s_tep := { s ->", "\n}"),
    }
}

/// The program X_eTaL ran, exactly, decorated, `focus` highlighted,
/// the pile's data folded.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(program, focus))
}

/// Grains: 0 dark, 1 blue, 2 gold, 3 red; 4 or more (about to topple) white.
const GRAINS: [Rgb; 5] = [[16, 14, 34], [28, 126, 214], [250, 176, 5], [224, 49, 49], [255, 255, 255]];

pub fn pile(h: &[i64]) -> Vec<u8> {
    let v: Vec<f64> = h.iter().map(|&x| x as f64).collect();
    pixels(&v, |x| GRAINS[(x as usize).min(4)])
}

/// Counts on a log scale, dark to bright.
pub fn counts(c: &[i64]) -> Vec<u8> {
    let top = c.iter().cloned().max().unwrap_or(0).max(1) as f64;
    let v: Vec<f64> = c.iter().map(|&x| x as f64).collect();
    pixels(&v, |x| if x <= 0.0 { [16, 14, 34] } else { ramp(&GLOW, (1.0 + x).ln() / (1.0 + top).ln()) })
}
