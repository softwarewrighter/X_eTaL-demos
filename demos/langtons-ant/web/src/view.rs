//! The stages of a step, the part of the program computing each, colors.

use microscope::color::pixels;
use microscope::source::{between, listing, Range};
use yew::Html;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Look,
    Turn,
    Flip,
    Move,
}

pub const STAGES: [Stage; 4] = [Stage::Look, Stage::Turn, Stage::Flip, Stage::Move];

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Look => between(src, "cell := ", "cell := "),
        Stage::Turn => between(src, "turn := ", "turn := "),
        Stage::Flip => between(src, "flip := ", "flip := "),
        Stage::Move => between(src, "dy := ", "step := "),
    }
}

/// The program X_eTaL ran, exactly, decorated, `focus` highlighted,
/// the board's data folded.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(program, focus))
}

/// The board as Langton drew it: white cells white, black cells black,
/// the ant violet.
pub fn board(cells: &[i64], ant: usize) -> Vec<u8> {
    let v: Vec<f64> = cells.iter().map(|&c| c as f64).collect();
    let mut px = pixels(&v, |c| if c == 1.0 { [20, 18, 40] } else { [250, 248, 242] });
    px[4 * ant..4 * ant + 3].copy_from_slice(&[151, 117, 250]);
    px
}
