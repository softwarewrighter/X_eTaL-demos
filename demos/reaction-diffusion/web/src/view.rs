//! The stages of a step and the part of the core computing each.

use microscope::source::{between, block, Range};
use yew::Html;

use crate::micro::core;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Spread,
    React,
    Update,
}

pub const STAGES: [Stage; 3] = [Stage::Spread, Stage::React, Stage::Update];

/// The byte range of the core that computes `stage`.
pub fn range(stage: Stage) -> Range {
    let src = core();
    match stage {
        Stage::Spread => between(src, "u:l_ap := ", "u:l_ap := "),
        Stage::React => between(src, "uvv := ", "uvv := "),
        Stage::Update => between(src, "(u:p_lane u + ", "(u:p_lane u + "),
    }
}

/// The core, decorated, `focus` highlighted.
pub fn source(focus: Stage) -> Html {
    block(core(), range(focus))
}
