//! The stages, the part of the program (or the library) computing
//! each, and colors.

use microscope::color::{field, signed};
use microscope::source::{between, find, listing, Range};
use yew::Html;

use crate::micro::LIBRARY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Picture,
    Macro,
    Call,
    Result,
}

pub const STAGES: [Stage; 4] = [Stage::Picture, Stage::Macro, Stage::Call, Stage::Result];

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Picture => between(src, "rows := ", "img := "),
        Stage::Macro => find(src, "\"s:\" u_se< \"Stencil\""),
        Stage::Call => find(src, src.lines().find(|l| l.starts_with("u:s_tep :=")).unwrap_or("u:s_tep :=")),
        Stage::Result => find(src, src.lines().find(|l| l.starts_with("out := ")).unwrap_or("out := ")),
    }
}

/// The program X_eTaL ran, exactly, decorated, `focus` highlighted.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(program, focus))
}

/// The macro library, its macro highlighted.
pub fn library() -> Html {
    listing(LIBRARY, between(LIBRARY, "m:t_encil< := {", "\n}"))
}

/// The picture and an unsigned result: dark to bright over 0 .. 1.1.
pub fn bright(v: &[f64]) -> Vec<u8> {
    field(v, 0.0, 1.1)
}

/// A result: signed (blue, white, red) when the kernel sums to 0.
pub fn result(v: &[f64], balanced: bool) -> Vec<u8> {
    match balanced {
        true => signed(v),
        false => bright(v),
    }
}
