//! X_eTaL source drawn decorated (as X_eTaL renders it), with the part
//! computing the selected stage highlighted.

use xetal_play::{decorate, Class};
use yew::prelude::*;

use crate::micro::core;

/// The stages of one run, in the order the program computes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Grid,
    Step,
    Iterate,
    Measure,
}

pub const STAGES: [Stage; 4] = [Stage::Grid, Stage::Step, Stage::Iterate, Stage::Measure];

fn css(class: Class) -> &'static str {
    match class {
        Class::Builtin => "t-builtin",
        Class::UserFunc | Class::LibFunc | Class::Macro => "t-user",
        Class::LambdaArg => "t-arg",
        Class::Number | Class::Exponent => "t-num",
        Class::Symbol | Class::Quote => "t-sym",
        Class::Comment => "t-comment",
        _ => "t-plain",
    }
}

fn between(src: &str, from: &str, to_line_with: &str) -> (usize, usize) {
    let a = src.find(from).unwrap_or(0);
    let b = src[a..].find(to_line_with).map_or(a, |i| a + i);
    let end = src[b..].find('\n').map_or(src.len(), |i| b + i);
    (a, end)
}

/// The byte range of the core that computes `stage`.
pub fn range(stage: Stage) -> (usize, usize) {
    let src = core();
    match stage {
        Stage::Grid => between(src, "re := ", "ci := "),
        Stage::Step => between(src, "u:s_tep := ", "\n}"),
        Stage::Iterate => between(src, "z := k ", "z := k "),
        Stage::Measure => between(src, "zr := 1 s_elect z", "counts := "),
    }
}

fn segments(src: &str, lo: usize, hi: usize) -> Html {
    let segs = decorate(src).into_iter().map(|s| {
        let mut class = classes!(css(s.class));
        if s.raw.start >= lo && s.raw.end <= hi && s.class != Class::Comment {
            class.push("hl");
        }
        html! { <span {class}>{s.text}</span> }
    });
    html! { <>{ for segs }</> }
}

/// Any X_eTaL snippet, drawn decorated.
pub fn code(src: &str) -> Html {
    html! { <code class="xtl">{ segments(src, 1, 0) }</code> }
}

/// The program's core, decorated, `focus` highlighted.
pub fn source(focus: Stage) -> Html {
    let (lo, hi) = range(focus);
    html! { <pre class="source">{ segments(core(), lo, hi) }</pre> }
}

/// A shape as X_eTaL's s_hape gives it, labelled, with a tooltip.
pub fn shape(dims: &[usize], meaning: &str) -> Html {
    let text: Vec<String> = dims.iter().map(usize::to_string).collect();
    html! {
        <span class="shape" title={format!("the array's shape (s_hape): {meaning}")}>
            { code(&format!("s_hape = {}", text.join(" "))) }
        </span>
    }
}
