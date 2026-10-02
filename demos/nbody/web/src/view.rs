//! The program the page shows (the preset's constants, then the core),
//! the stages and the part computing each, and the matrices' colours.

use microscope::colour::{pixels, ramp, Rgb, DIVERGE, GLOW};
use microscope::source::{between, block, Range};
use yew::Html;

use crate::micro::{core, Physics};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Masses,
    Cube,
    Pull,
    Reduce,
    Step,
}

pub const STAGES: [Stage; 5] = [Stage::Masses, Stage::Cube, Stage::Pull, Stage::Reduce, Stage::Step];

/// The preset's constants followed by the core.
pub fn program(ph: &Physics, n: usize) -> String {
    format!(
        "g := {}\neps2 := {}\ndt := {}\n# m: the {n} masses, written in by the page with the state\n{}",
        microscope::run::lit(ph.g),
        microscope::run::lit(ph.eps2),
        microscope::run::lit(ph.dt),
        core()
    )
}

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Masses => between(src, "n := t_ally m", "mj := "),
        Stage::Cube => between(src, "u:c_ube := { p ->", "\n}"),
        Stage::Pull => between(src, "u:p_ull := ", "u:p_ull := "),
        Stage::Reduce => between(src, "u:a_cc := { p ->", "\n}"),
        Stage::Step => between(src, "u:s_tep := { s ->", "\n}"),
    }
}

/// The constants and the core, decorated, `focus` highlighted.
pub fn source(ph: &Physics, n: usize, focus: Stage) -> Html {
    let src = program(ph, n);
    block(&src, range(&src, focus))
}

/// Each body's colour.
pub fn body_colour(i: usize) -> &'static str {
    const C: [&str; 8] = ["#7048e8", "#e8590c", "#1c7ed6", "#2f9e44", "#d6336c", "#f59f00", "#0c8599", "#5f3dc4"];
    C[i % C.len()]
}

/// Dim every row of an N x N picture but row `sel`.
fn spotlight(mut px: Vec<u8>, cols: usize, sel: Option<usize>) -> Vec<u8> {
    if let Some(s) = sel {
        for (k, p) in px.chunks_mut(4).enumerate() {
            if k / cols != s {
                for c in &mut p[..3] {
                    *c = (*c as f64 * 0.45 + 255.0 * 0.2) as u8;
                }
            }
        }
    }
    px
}

/// Signed values (blue below 0, red above), row `sel` in full colour.
pub fn signed(values: &[f64], cols: usize, sel: Option<usize>) -> Vec<u8> {
    spotlight(microscope::colour::signed(values), cols, sel)
}

const DIAGONAL: Rgb = [120, 120, 130];

/// Positive values on a log scale over the off-diagonal entries (the
/// diagonal, a body and itself, grey), row `sel` in full colour.
pub fn log_scaled(values: &[f64], n: usize, sel: Option<usize>) -> Vec<u8> {
    let off = |k: usize| k / n != k % n;
    let logs: Vec<f64> = values.iter().map(|&v| v.max(1e-300).log10()).collect();
    let (lo, hi) = (0..values.len())
        .filter(|&k| off(k) && values[k] > 0.0)
        .fold((f64::MAX, f64::MIN), |(lo, hi), k| (lo.min(logs[k]), hi.max(logs[k])));
    let span = (hi - lo).max(1e-9);
    let mut px = pixels(&logs, |l| ramp(&GLOW, (l - lo) / span));
    for k in (0..values.len()).filter(|&k| !off(k)) {
        px[4 * k..4 * k + 3].copy_from_slice(&DIAGONAL);
    }
    spotlight(px, n, sel)
}

/// The 2 x N accelerations, each row scaled by its largest size.
pub fn acc_strip(acc: &[f64], n: usize) -> Vec<u8> {
    let m = acc.iter().fold(1e-12_f64, |m, v| m.max(v.abs()));
    pixels(&acc[..2 * n], |v| ramp(&DIVERGE, 0.5 + v / (2.0 * m)))
}
