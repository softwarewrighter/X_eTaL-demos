//! Drawing the arrays: boards, the sum heatmap, and the decorated line.

use xetal_play::{decorate, Class};
use yew::prelude::*;

use crate::micro::LINE;
use crate::model::Stage;

/// How a grid's cells are coloured.
#[derive(Clone, Copy, PartialEq)]
pub enum Paint {
    /// 0 / 1 cells.
    Cells,
    /// 0..9 sums, shaded, numbers shown.
    Sum,
}

pub struct Grid<'a> {
    pub cells: &'a [u8],
    pub rows: usize,
    pub cols: usize,
    pub paint: Paint,
    pub selected: Option<(usize, usize)>,
    pub size: &'static str,
    pub onclick: Option<Callback<(usize, usize)>>,
}

fn cell(g: &Grid, y: usize, x: usize) -> Html {
    let v = g.cells[y * g.cols + x];
    let mut class = classes!("cell");
    if g.selected == Some((y, x)) {
        class.push("sel");
    }
    let (style, text) = match g.paint {
        Paint::Cells => {
            class.push(if v == 1 { "on" } else { "off" });
            (String::new(), String::new())
        }
        Paint::Sum => {
            let text = if v == 0 { String::new() } else { v.to_string() };
            (format!("--heat:{}", v.min(9)), text)
        }
    };
    if g.paint == Paint::Sum {
        class.push("heat");
        class.push(match v { 3 => "s3", 4 => "s4", _ => "" });
        if v >= 5 {
            class.push("hot");
        }
    }
    let onclick = g.onclick.clone().map(|cb| Callback::from(move |_: MouseEvent| cb.emit((y, x))));
    html! { <div {class} {style} {onclick}>{text}</div> }
}

pub fn grid(g: Grid) -> Html {
    let style = format!("grid-template-columns: repeat({}, 1fr)", g.cols);
    let cells = (0..g.rows).flat_map(|y| (0..g.cols).map(move |x| (y, x)));
    html! {
        <div class={classes!("grid", g.size)} {style}>
            { for cells.map(|(y, x)| cell(&g, y, x)) }
        </div>
    }
}

fn css(class: Class) -> &'static str {
    match class {
        Class::Builtin => "t-builtin",
        Class::UserFunc | Class::LibFunc | Class::Macro => "t-user",
        Class::LambdaArg => "t-arg",
        Class::Number | Class::Exponent => "t-num",
        Class::Symbol | Class::Quote => "t-sym",
        _ => "t-plain",
    }
}

/// The byte range of LINE that computes `stage`.
pub fn range(stage: Stage) -> (usize, usize) {
    let find = |s: &str| LINE.find(s).map(|i| (i, i + s.len())).unwrap_or((0, 0));
    match stage {
        Stage::Board => LINE.rfind("_r").map(|i| (i, i + 2)).unwrap_or((0, 0)),
        Stage::Rotate => find("-1 0 1 o_-_12 _r"),
        Stage::Sum => find("'+ r_/_12"),
        Stage::Masks => find("(_l = 3) + _r * _l = 4"),
        Stage::Next => find("{ (_l = 3) + _r * _l = 4 } _r"),
    }
}

/// The line drawn decorated, the part computing `focus` highlighted.
pub fn line(focus: Stage) -> Html {
    let (lo, hi) = range(focus);
    let segs = decorate(LINE).into_iter().map(|s| {
        let mut class = classes!(css(s.class));
        if s.raw.start >= lo && s.raw.end <= hi && s.class != Class::Space {
            class.push("hl");
        }
        html! { <span {class}>{s.text}</span> }
    });
    html! { <code class="line">{ for segs }</code> }
}
