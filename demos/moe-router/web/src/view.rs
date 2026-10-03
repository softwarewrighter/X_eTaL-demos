//! The program the page shows (the router weights, the core, the
//! sentence's lines), the stages and the part computing each.

use microscope::source::{between, block, find, Range};
use yew::Html;

use crate::micro::{core, weights};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Embed,
    Scores,
    Softmax,
    Top2,
    Load,
}

pub const STAGES: [Stage; 5] = [Stage::Embed, Stage::Scores, Stage::Softmax, Stage::Top2, Stage::Load];

/// The router weights, the core, and the lines that route the sentence.
pub fn program(ids: &[usize]) -> String {
    let ids: Vec<String> = ids.iter().map(|i| i.to_string()).collect();
    format!(
        "# F, E: the 37 words' features and embeddings (not shown)\n{}{}# -- the sentence\nids := {}\nx := ids s_elect E\np := u:s_oftmax u:s_cores x\ngates := u:t_op2 p\nu:l_oad gates\n",
        weights(),
        core(),
        ids.join(" ")
    )
}

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Embed => find(src, "x := ids s_elect E"),
        Stage::Scores => between(src, "u:s_cores := ", "u:s_cores := "),
        Stage::Softmax => between(src, "u:s_oftmax := { s ->", "\n}"),
        Stage::Top2 => between(src, "u:t_op2 := { p ->", "\n}"),
        Stage::Load => between(src, "u:l_oad := ", "u:l_oad := "),
    }
}

pub fn source(ids: &[usize], focus: Stage) -> Html {
    let src = program(ids);
    block(&src, range(&src, focus))
}

/// Grid geometry of the routing picture.
pub const TOK_X: f64 = 120.0;
pub const GRID_X: f64 = 330.0;
pub const CELL_W: f64 = 112.0;
pub const CELL_H: f64 = 64.0;
pub const GAP: f64 = 8.0;
pub const TOP: f64 = 24.0;

/// The top-left corner of expert e's box.
pub fn expert_box(e: usize) -> (f64, f64) {
    (GRID_X + (e % 4) as f64 * (CELL_W + GAP), TOP + (e / 4) as f64 * (CELL_H + GAP))
}

/// The y of token t's label, spread over the grid's height.
pub fn token_y(t: usize, n: usize) -> f64 {
    let h = 4.0 * (CELL_H + GAP) - GAP;
    TOP + h * (t as f64 + 0.5) / n as f64
}
