//! X_eTaL source drawn decorated (as X_eTaL renders it), with the part
//! computing the selected stage highlighted.

use xetal_play::{decorate, Class};
use yew::prelude::*;

use crate::micro::core;

/// The stages of one run, in the order the program computes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Spread,
    React,
    Update,
}

pub const STAGES: [Stage; 3] = [Stage::Spread, Stage::React, Stage::Update];

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
        Stage::Spread => between(src, "u:l_ap := ", "u:l_ap := "),
        Stage::React => between(src, "uvv := ", "uvv := "),
        Stage::Update => between(src, "(u:p_lane u + ", "(u:p_lane u + "),
    }
}

fn segments(src: &str, lo: usize, hi: usize) -> Html {
    // Runs of segments inside lo..hi are wrapped in one highlight block.
    let mut out: Vec<Html> = Vec::new();
    let mut run: Vec<Html> = Vec::new();
    let flush = |run: &mut Vec<Html>, out: &mut Vec<Html>| {
        if !run.is_empty() {
            let inner = std::mem::take(run);
            out.push(html! { <span class="hl">{ for inner }</span> });
        }
    };
    for s in decorate(src) {
        let inside = s.raw.start >= lo && s.raw.end <= hi && s.class != Class::Comment;
        let seg = html! { <span class={css(s.class)}>{s.text}</span> };
        if inside {
            run.push(seg);
        } else {
            flush(&mut run, &mut out);
            out.push(seg);
        }
    }
    flush(&mut run, &mut out);
    html! { <>{ for out }</> }
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
