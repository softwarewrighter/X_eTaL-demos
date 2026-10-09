//! The stages of a run and the part of the program computing each.

use microscope::source::{between, listing, Range, NONE};
use yew::Html;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Grid,
    Step,
    Iterate,
    Measure,
}

pub const STAGES: [Stage; 4] = [Stage::Grid, Stage::Step, Stage::Iterate, Stage::Measure];

/// The byte range of the program that computes `stage`.
pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Grid => between(src, "re := ", "ci := "),
        Stage::Step => between(src, "u:s_tep := ", "\n}"),
        Stage::Iterate => between(src, "(zr, zi, counts) := k ", "(zr, zi, counts) := k "),
        Stage::Measure => between(src, "(zr, zi, counts) := k ", "(zr, zi, counts) := k "),
    }
}

/// The program X_eTaL ran, exactly, decorated, `focus` highlighted.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(program, focus))
}

/// The orbit's program, exactly as run.
pub fn orbit_source(program: &str) -> Html {
    listing(program, NONE)
}
