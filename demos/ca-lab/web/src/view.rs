//! The stages, the part of the program computing each, and colors.

use microscope::color::{pixels, Rgb};
use microscope::source::{between, listing, Range};
use yew::Html;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Neighbors,
    Number,
    Lookup,
}

pub const STAGES: [Stage; 3] = [Stage::Neighbors, Stage::Number, Stage::Lookup];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    One,
    Two,
}

pub fn range(src: &str, mode: Mode, stage: Stage) -> Range {
    let (a, b) = match mode {
        Mode::One => ("u:r_ow := ", "u:r_ow := "),
        Mode::Two => ("u:c_ount := ", "u:c_ount := "),
    };
    match (mode, stage) {
        (Mode::One, Stage::Lookup) => between(src, "u:g_row := ", "\n}"),
        (Mode::Two, Stage::Lookup) | (Mode::Two, Stage::Number) => between(src, "u:l_ook := ", "u:l_ook := "),
        _ => between(src, a, b),
    }
}

/// The program X_eTaL ran for the mode shown, exactly, decorated,
/// `focus` highlighted, the board's data folded.
pub fn source(program: &str, mode: Mode, focus: Stage) -> Html {
    listing(program, range(program, mode, focus))
}

/// The colors of the states 0, 1, 2, 3.
pub const STATES: [Rgb; 4] = [[20, 18, 40], [255, 244, 214], [177, 151, 252], [255, 179, 102]];
/// Wireworld's: empty, head, tail, wire.
pub const WIRE: [Rgb; 4] = [[20, 18, 40], [120, 200, 255], [255, 110, 90], [255, 190, 90]];

pub fn states(cells: &[i64], wire: bool) -> Vec<u8> {
    let pal = if wire { &WIRE } else { &STATES };
    let v: Vec<f64> = cells.iter().map(|&c| c as f64).collect();
    pixels(&v, |c| pal[(c as usize).min(3)])
}
