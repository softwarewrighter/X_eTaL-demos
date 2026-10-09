//! The stages and the part of the program computing each.

use microscope::source::{between, listing, Range, NONE};
use yew::Html;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Grid,
    Iterate,
    Call,
}

pub const STAGES: [Stage; 3] = [Stage::Grid, Stage::Iterate, Stage::Call];

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Grid => between(src, "re := ", "grid := "),
        Stage::Iterate => between(src, "u:i_terate := ", "\n}"),
        Stage::Call => between(src, "(zr, zi, counts) := ", "(zr, zi, counts) := "),
    }
}

/// The program X_eTaL ran, exactly, decorated, `focus` highlighted.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(program, focus))
}

/// The picker's program (the Mandelbrot map), exactly as run.
pub fn picker_source(program: &str) -> Html {
    listing(program, NONE)
}
