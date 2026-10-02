//! The stages and the part of the program computing each.

use microscope::source::{between, block, Range, NONE};
use yew::Html;

use crate::micro::core;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Grid,
    Iterate,
    Call,
}

pub const STAGES: [Stage; 3] = [Stage::Grid, Stage::Iterate, Stage::Call];

pub fn range(stage: Stage) -> Range {
    let src = core();
    match stage {
        Stage::Grid => between(src, "re := ", "grid := "),
        Stage::Iterate => between(src, "u:i_terate := ", "\n}"),
        Stage::Call => NONE,
    }
}

/// The core, decorated, `focus` highlighted.
pub fn source(focus: Stage) -> Html {
    block(core(), range(focus))
}
