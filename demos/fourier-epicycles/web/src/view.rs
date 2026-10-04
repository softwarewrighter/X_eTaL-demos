//! The stages of the program and the part computing each.

use microscope::source::{between, listing, Range};
use yew::Html;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Angles,
    Transform,
    Circles,
    Chain,
    Error,
}

pub const STAGES: [Stage; 5] = [Stage::Angles, Stage::Transform, Stage::Circles, Stage::Chain, Stage::Error];

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Angles => between(src, "A := ", "S := "),
        Stage::Transform => between(src, "re := ", "f := "),
        Stage::Circles => between(src, "order := ", "vy := "),
        Stage::Chain => between(src, "cx := ", "cy := "),
        Stage::Error => between(src, "dx := ", "err := "),
    }
}

/// The program X_eTaL ran, exactly, decorated, `focus` highlighted,
/// the curve's points folded.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(program, focus))
}
