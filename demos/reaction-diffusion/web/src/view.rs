//! The stages of a step and the part of the program computing each.

use microscope::source::{between, listing, Range};
use yew::Html;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Spread,
    React,
    Update,
}

pub const STAGES: [Stage; 3] = [Stage::Spread, Stage::React, Stage::Update];

/// The byte range of the program that computes `stage`.
pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Spread => between(src, "u:l_ap := ", "u:l_ap := "),
        Stage::React => between(src, "uvv := ", "uvv := "),
        Stage::Update => between(src, "(u:p_lane u + ", "(u:p_lane u + "),
    }
}

/// The program X_eTaL ran, exactly, decorated, `focus` highlighted,
/// the grids' data folded.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(program, focus))
}
