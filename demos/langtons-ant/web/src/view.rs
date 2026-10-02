//! The stages of a step, the part of the core computing each, colours.

use microscope::colour::pixels;
use microscope::source::{between, block, Range};
use yew::Html;

use crate::micro::core;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Look,
    Turn,
    Flip,
    Move,
}

pub const STAGES: [Stage; 4] = [Stage::Look, Stage::Turn, Stage::Flip, Stage::Move];

pub fn range(stage: Stage) -> Range {
    let src = core();
    match stage {
        Stage::Look => between(src, "cell := ", "cell := "),
        Stage::Turn => between(src, "turn := ", "turn := "),
        Stage::Flip => between(src, "flip := ", "flip := "),
        Stage::Move => between(src, "dy := ", "step := "),
    }
}

pub fn source(focus: Stage) -> Html {
    block(core(), range(focus))
}

/// The board: white cells dark, black cells cream, the ant violet.
pub fn board(cells: &[i64], ant: usize) -> Vec<u8> {
    let v: Vec<f64> = cells.iter().map(|&c| c as f64).collect();
    let mut px = pixels(&v, |c| if c == 1.0 { [255, 244, 214] } else { [20, 18, 40] });
    px[4 * ant..4 * ant + 3].copy_from_slice(&[151, 117, 250]);
    px
}
