//! X_eTaL source drawn decorated (as X_eTaL renders it, colored), with
//! a range highlighted as one continuous block, and shapes as s_hape.

use xetal_play::{decorate, Class};
use yew::prelude::*;

/// A byte range of a source; `NONE` highlights nothing.
pub type Range = (usize, usize);
pub const NONE: Range = (1, 0);

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

fn segments(src: &str, (lo, hi): Range) -> Html {
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

/// Any X_eTaL snippet, drawn decorated, inline.
pub fn code(src: &str) -> Html {
    html! { <code class="xtl">{ segments(src, NONE) }</code> }
}

/// One line of X_eTaL, large, `range` highlighted.
pub fn line(src: &str, range: Range) -> Html {
    html! { <code class="line">{ segments(src, range) }</code> }
}

/// A program (several lines, with comments), `range` highlighted.
pub fn block(src: &str, range: Range) -> Html {
    html! { <pre class="source">{ segments(src, range) }</pre> }
}

/// The range of `src` from `from` to the end of the line holding the
/// first `to` after it.
pub fn between(src: &str, from: &str, to: &str) -> Range {
    let a = src.find(from).unwrap_or(0);
    let b = src[a..].find(to).map_or(a, |i| a + i + to.len());
    let end = src[b..].find('\n').map_or(src.len(), |i| b + i);
    (a, end)
}

/// The range of the first `part` in `src` (`NONE` if it is not there).
pub fn find(src: &str, part: &str) -> Range {
    src.find(part).map_or(NONE, |i| (i, i + part.len()))
}

/// A shape as X_eTaL's s_hape gives it, decorated, with a tooltip
/// saying what the axes are; a scalar's shape is empty.
pub fn shape(dims: &[usize], meaning: &str) -> Html {
    let text: Vec<String> = dims.iter().map(usize::to_string).collect();
    let shown = match text.is_empty() {
        true => html! { <>{ code("s_hape") }{ " is empty: a scalar" }</> },
        false => code(&format!("s_hape = {}", text.join(" "))),
    };
    html! { <span class="shape" title={format!("the array's shape (s_hape): {meaning}")}>{ shown }</span> }
}

/// A line of a program longer than this, holding more numbers than
/// `FOLD_NUMBERS`, is data the page wrote in: it is folded.
const FOLD_CHARS: usize = 160;
const FOLD_NUMBERS: usize = 24;
/// How many numbers a folded line shows before its "...".
const SHOWN_NUMBERS: usize = 6;

/// One line of a program as a listing draws it: its byte span in the
/// program, how many numbers it holds, and its decorated segments (for
/// a folded line, only those of its first few numbers).
pub struct Line {
    pub raw: Range,
    pub numbers: usize,
    pub folded: bool,
    pub segments: Vec<(String, Class, Range)>,
}

impl Line {
    /// Whether the line is folded in a listing.
    pub fn folded(&self) -> bool {
        self.folded
    }
}

fn number_words(line: &str) -> usize {
    line.split_whitespace().filter(|w| w.parse::<f64>().is_ok()).count()
}

/// The end of `line`'s prefix that shows its first `SHOWN_NUMBERS`
/// numbers (cut at a space).
fn prefix_end(line: &str) -> usize {
    let mut seen = 0;
    let mut end = line.len();
    for (i, w) in line.split(' ').scan(0usize, |pos, w| { let at = *pos; *pos += w.len() + 1; Some((at, w)) }) {
        if w.parse::<f64>().is_ok() {
            seen += 1;
            if seen > SHOWN_NUMBERS {
                end = i;
                break;
            }
        }
    }
    line[..end].trim_end().len()
}

/// The program split into lines; long data lines folded. Only what a
/// listing shows is decorated: the whole program with each folded line
/// cut to its first numbers (so a page that writes in thousands of
/// numbers each frame stays fast).
pub fn lines(src: &str) -> Vec<Line> {
    let mut out = vec![];
    let mut skeleton = String::new();
    let mut map = vec![]; // (skeleton start, skeleton end, program start)
    let mut start = 0;
    for raw in src.split('\n') {
        let numbers = number_words(raw);
        let folded = raw.len() > FOLD_CHARS && numbers > FOLD_NUMBERS;
        let shown = if folded { &raw[..prefix_end(raw)] } else { raw };
        if !skeleton.is_empty() || !out.is_empty() {
            skeleton.push('\n');
        }
        let k = skeleton.len();
        skeleton.push_str(shown);
        map.push((k, skeleton.len(), start));
        out.push(Line { raw: (start, start + raw.len()), numbers, folded, segments: vec![] });
        start += raw.len() + 1;
    }
    let mut li = 0;
    for seg in decorate(&skeleton) {
        let mut pos = seg.raw.start;
        for (j, piece) in skeleton[seg.raw.start..seg.raw.end].split('\n').enumerate() {
            if j > 0 {
                pos += 1;
            }
            while li + 1 < map.len() && pos >= map[li + 1].0 {
                li += 1;
            }
            if !piece.is_empty() {
                let (k, _, p) = map[li];
                let text = if seg.raw.end - seg.raw.start == piece.len() { seg.text.clone() } else { piece.to_string() };
                out[li].segments.push((text, seg.class, (pos - k + p, pos - k + p + piece.len())));
            }
            pos += piece.len();
        }
    }
    out
}

fn draw(segments: &[(String, Class, Range)], (lo, hi): Range) -> Html {
    let mut out: Vec<Html> = Vec::new();
    let mut run: Vec<Html> = Vec::new();
    for (text, class, (a, b)) in segments {
        let seg = html! { <span class={css(*class)}>{text.clone()}</span> };
        if *a >= lo && *b <= hi && *class != Class::Comment {
            run.push(seg);
        } else {
            if !run.is_empty() {
                let inner = std::mem::take(&mut run);
                out.push(html! { <span class="hl">{ for inner }</span> });
            }
            out.push(seg);
        }
    }
    if !run.is_empty() {
        out.push(html! { <span class="hl">{ for run }</span> });
    }
    html! { <>{ for out }</> }
}

#[derive(Properties, PartialEq)]
struct FoldProps {
    head: Html,
    line: String,
    numbers: usize,
}

/// A folded data line: its start and a count; opened, the whole line,
/// decorated only while open.
#[function_component(Fold)]
fn fold(p: &FoldProps) -> Html {
    let open = use_state(|| false);
    let toggle = {
        let open = open.clone();
        Callback::from(move |_: MouseEvent| open.set(!*open))
    };
    if *open {
        let body = segments(&p.line, NONE);
        html! { <div class="fold open">
            <span class="foldn" onclick={toggle}>{format!("({} numbers: hide them)", p.numbers)}</span>{"\n"}{ body }
        </div> }
    } else {
        html! { <div class="fold">
            { p.head.clone() }<span class="foldn" onclick={toggle}>{format!(" ... ({} numbers: show them)", p.numbers)}</span>
        </div> }
    }
}

/// The whole program a page ran, drawn decorated, `range` highlighted;
/// its long data lines (what the page wrote in: a board, the bodies)
/// folded, each showing its start and how many numbers it holds, the
/// whole line one click away.
pub fn listing(src: &str, range: Range) -> Html {
    let all = lines(src);
    let n = all.len();
    let rows = all.into_iter().enumerate().map(|(i, l)| {
        let nl = if i + 1 < n { "\n" } else { "" };
        if l.folded {
            let head = draw(&l.segments, range);
            let line = src[l.raw.0..l.raw.1].to_string();
            html! { <Fold {head} {line} numbers={l.numbers} /> }
        } else {
            html! { <>{ draw(&l.segments, range) }{nl}</> }
        }
    });
    html! { <pre class="source listing">{ for rows }</pre> }
}
