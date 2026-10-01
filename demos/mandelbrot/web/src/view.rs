//! The stages of a run and the part of the core computing each.

use microscope::source::{between, block, Range};
use yew::Html;

use crate::micro::core;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Grid,
    Step,
    Iterate,
    Measure,
}

pub const STAGES: [Stage; 4] = [Stage::Grid, Stage::Step, Stage::Iterate, Stage::Measure];

/// The byte range of the core that computes `stage`.
pub fn range(stage: Stage) -> Range {
    let src = core();
    match stage {
        Stage::Grid => between(src, "re := ", "ci := "),
        Stage::Step => between(src, "u:s_tep := ", "\n}"),
        Stage::Iterate => between(src, "z := k ", "z := k "),
        Stage::Measure => between(src, "zr := 1 s_elect z", "counts := "),
    }
}

/// The core, decorated, `focus` highlighted.
pub fn source(focus: Stage) -> Html {
    block(core(), range(focus))
}
