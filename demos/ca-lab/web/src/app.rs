//! The page: two modes. One dimension: an elementary rule's history,
//! its 8-entry table, and one generation's arrays. Two dimensions: a
//! table on (state, live neighbors), a board, and the coming step.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{about, chip, footer, header, notice, panel};
use microscope::color;
use microscope::source::code;

use crate::micro::{bits, rules, History, Step2, COLS, GENS, ROWS, WIDTH};
use crate::model::{Action, Model};
use crate::view::{source, states, Mode, Stage, STAGES};

const PRESETS_1D: [u8; 7] = [30, 90, 110, 184, 150, 54, 73];

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match (m.mode, s) {
        (Mode::One, Stage::Neighbors) => ("neighbors", "-1 o_- r ; 1 o_- r", vec![WIDTH], "one number per cell of the row"),
        (Mode::One, Stage::Number) => ("number", "(4 * l) + (2 * r) + rt", vec![WIDTH], "0 to 7 per cell"),
        (Mode::One, Stage::Lookup) => ("look up", "tbl u:r_ow r", vec![GENS, WIDTH], "the history: one row per generation"),
        (Mode::Two, Stage::Neighbors) => ("count", "u:c_ount b", vec![ROWS, COLS], "live neighbors, 0 to 8, per cell"),
        (Mode::Two, Stage::Number) => ("number", "(9 * b) + u:c_ount b", vec![ROWS, COLS], "state * 9 + count per cell"),
        (Mode::Two, Stage::Lookup) => ("look up", "tbl u:l_ook b", vec![ROWS, COLS], "the next board"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn mode_tabs(m: &UseReducerHandle<Model>) -> Html {
    let tab = |mode: Mode, label: &str| {
        let class = classes!("tab", (m.mode == mode).then_some("active"));
        html! { <button {class} onclick={act(m, move || Action::Mode(mode))}>{label.to_string()}</button> }
    };
    html! { <div class="tabs">{ tab(Mode::One, "One dimension: Wolfram's rules") }{ tab(Mode::Two, "Two dimensions: rule tables") }</div> }
}

// -- one dimension ---------------------------------------------------

fn controls_1d(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let on_rule = Callback::from(move |e: Event| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        if let Ok(r) = v.parse::<u8>() {
            d.dispatch(Action::Rule(r));
        }
    });
    let random = m.random;
    html! {
        <div class="controls">
            <label class="slider">{"rule"}
                <input class="num" type="number" min="0" max="255" value={m.rule.to_string()} onchange={on_rule} />
            </label>
            { for PRESETS_1D.iter().map(|&r| html! { <button onclick={act(m, move || Action::Rule(r))}>{format!("Rule {r}")}</button> }) }
            <label class="toggle">
                <input type="checkbox" checked={random} onclick={act(m, move || Action::Random(!random))} />
                {"Random first row"}
            </label>
            <span class="gen">{format!("X_eTaL grew {GENS} generations in {:.0} ms", m.ms)}</span>
        </div>
    }
}

/// The 8-entry table: each neighborhood (left, center, right), its
/// number, and the next state, which a click flips.
fn table_1d(m: &UseReducerHandle<Model>) -> Html {
    let b = bits(m.rule);
    let entries = (0..8).rev().map(|k| {
        let cell = |on: bool| html! { <span class={classes!("c", on.then_some("on"))}></span> };
        html! {
            <button class="entry" onclick={act(m, move || Action::FlipBit(k))} title="click to flip">
                <span class="hood">{cell(k & 4 != 0)}{cell(k & 2 != 0)}{cell(k & 1 != 0)}</span>
                <span class="k">{k}</span>
                <span class="hood">{cell(b[k] == 1)}</span>
            </button>
        }
    });
    let body = html! { <div class="table1">{ for entries }</div> };
    panel("The rule as a table:", &format!("tbl := u:b_its {}", m.rule), "Each neighborhood (left, center, right) is numbered 4 * left + 2 * center + right; the rule's bits say what each number becomes. Click an entry to flip it.", m.focus == Stage::Lookup, body)
}

fn strip(cells: &[i64], at: usize, label: &str) -> Html {
    let lo = at.saturating_sub(12).min(WIDTH - 25);
    let items = (lo..lo + 25).map(|i| html! {
        <td class={classes!(if cells[i] == 1 { "on" } else { "off" }, (i == at).then_some("at"))}>{cells[i]}</td>
    });
    html! { <tr><th>{code(label)}</th>{ for items }</tr> }
}

fn generation(m: &Model, h: &History) -> Html {
    let x = m.cell;
    let k = h.number[x] as usize;
    let body = html! { <>
        <table class="strips">
            { strip(&h.left, x, "-1 o_- r") }
            { strip(&h.row, x, "r") }
            { strip(&h.right, x, "1 o_- r") }
            { strip(&h.number, x, "number") }
            { strip(&h.next, x, "tbl u:r_ow r") }
        </table>
        <p class="calc">{"Cell "}{x + 1}{": number = "}{code(&format!("(4 * {}) + (2 * {}) + {} = {k}", h.left[x], h.row[x], h.right[x]))}{"; entry "}{k}{" of the table is "}{code(&h.next[x].to_string())}</p>
    </> };
    panel(&format!("Generation {}, around cell {}:", m.gen + 1, x + 1), "r := g s_elect h", "The row, rotated each way (every cell's left and right neighbors, all at once), the neighborhood numbers, and the next row looked up in the table.", m.focus != Stage::Lookup, body)
}

fn body_1d(model: &UseReducerHandle<Model>) -> Html {
    let m: &Model = model;
    let Some(h) = &m.history else { return html! {} };
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(g, x)| d.dispatch(Action::Pick(g, x))));
    let pic = html! { <Canvas rows={GENS} cols={WIDTH} rgba={Rc::new(states(&h.rows, false))} mark={Some((m.gen, m.cell))} {onclick} class="big" /> };
    html! {
        <div class="layout even">
            <div class="col">
                { panel(&format!("Rule {}, growing downward:", m.rule), "h", "Each row is the next generation of the row above. Click a cell to see how its generation was computed.", false, pic) }
                { table_1d(model) }
            </div>
            <div class="col">{ generation(m, h) }{ program(m) }</div>
        </div>
    }
}

// -- two dimensions --------------------------------------------------

fn controls_2d(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let on_rule = Callback::from(move |e: Event| {
        let i = e.target_unchecked_into::<HtmlSelectElement>().selected_index();
        d.dispatch(Action::Which(i.max(0) as usize));
    });
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
            <button onclick={act(m, || Action::Tick)}>{"Step"}</button>
            <button onclick={act(m, || Action::Reset)}>{"Reset"}</button>
            <select onchange={on_rule} aria-label="Rule">
                { for rules().iter().enumerate().map(|(i, r)| html! { <option selected={i == m.which}>{r.name}</option> }) }
            </select>
            <span class="gen">{format!("step {}; X_eTaL ran a step in {:.0} ms", m.steps, m.ms)}</span>
        </div>
    }
}

fn table_2d(m: &UseReducerHandle<Model>) -> Html {
    let r = &m.rule2;
    let wire = r.name == "Wireworld";
    let head = (0..9).map(|c| html! { <th>{c}</th> });
    let rows = r.states.iter().enumerate().map(|(s, name)| {
        let cells = (0..9).map(|c| {
            let i = s * 9 + c;
            let v = r.table[i] as usize;
            let [cr, cg, cb] = if wire { crate::view::WIRE[v] } else { crate::view::STATES[v] };
            let style = format!("background: rgb({cr},{cg},{cb}); color: {}", if v == 0 { "#e8e6e3" } else { "#1d1d1f" });
            html! { <td {style} onclick={act(m, move || Action::CycleEntry(i))} title={format!("number {}: click to change", s * 9 + c)}>{v}</td> }
        });
        html! { <tr><th class="state">{format!("{s} {name}")}</th>{ for cells }</tr> }
    });
    let body = html! { <table class="table2"><tr><th>{"state \\ live neighbors"}</th>{ for head }</tr>{ for rows }</table> };
    panel(&format!("{} as a table:", r.name), "tbl", "The next state for each state and count of neighbors in state 1. Click an entry to change it: the preview below changes at once.", m.focus == Stage::Lookup, body)
}

fn arrays_2d(m: &Model, s: &Step2) -> Html {
    let f = |v: &[i64]| v.iter().map(|&x| x as f64).collect::<Vec<f64>>();
    let small = |rgba: Vec<u8>, cap: &str| html! { <figure>
        <Canvas rows={ROWS} cols={COLS} rgba={Rc::new(rgba)} mark={Some(m.selected)} class="mid" />
        <figcaption>{code(cap)}</figcaption>
    </figure> };
    let wire = m.rule2.name == "Wireworld";
    let (y, x) = m.selected;
    let i = y * COLS + x;
    let body = html! { <>
        <div class="pair">
            { small(color::field(&f(&s.count), 0.0, 8.0), "u:c_ount b") }
            { small(color::scaled(&f(&s.number)), "(9 * b) + u:c_ount b") }
            { small(states(&s.next, wire), "tbl u:l_ook b") }
        </div>
        <p class="calc">{format!("Cell row {}, column {}: state ", y + 1, x + 1)}{code(&s.board[i].to_string())}{", live neighbors "}{code(&s.count[i].to_string())}{", number "}{code(&format!("(9 * {}) + {} = {}", s.board[i], s.count[i], s.number[i]))}{", next "}{code(&s.next[i].to_string())}</p>
    </> };
    panel("The coming step, for every cell at once:", "tbl u:l_ook b", "Live neighbors (nine rotated boards summed, minus the cell), each cell's table number, and the next board.", m.focus != Stage::Lookup, body)
}

fn body_2d(model: &UseReducerHandle<Model>) -> Html {
    let m: &Model = model;
    let Some(s) = &m.last else { return html! {} };
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Paint(y, x))));
    let wire = m.rule2.name == "Wireworld";
    let pic = html! { <Canvas rows={ROWS} cols={COLS} rgba={Rc::new(states(&m.board, wire))} mark={Some(m.selected)} {onclick} class="big" /> };
    let legend: Vec<String> = m.rule2.states.iter().enumerate().map(|(i, n)| format!("{i} {n}")).collect();
    html! {
        <div class="layout even">
            <div class="col">
                { panel("The board:", "b", &format!("States: {}. Click a cell to change its state; the edges wrap around.", legend.join(", ")), false, pic) }
                { table_2d(model) }
            </div>
            <div class="col">{ arrays_2d(m, s) }{ program(m) }</div>
        </div>
    }
}

fn program(m: &Model) -> Html {
    html! {
        <section class="panel code">
            <h2>{"The program"}</h2>
            <p class="note">{"Exactly the program X_eTaL ran in your browser for what you see: the core of ca-lab.xtl, the rule's table and the starting row or board written in by the page (folded: click to show the numbers), and the lines that grow or step and print. The stage you pick is highlighted."}</p>
            { source(if m.mode == Mode::One { &m.program_1d } else { &m.program_2d }, m.mode, m.focus) }
        </section>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let timer = playing.then(|| Interval::new(80, move || d.dispatch(Action::Tick)));
            move || drop(timer)
        });
    }
    let (controls, body) = match model.mode {
        Mode::One => (controls_1d(&model), body_1d(&model)),
        Mode::Two => (controls_2d(&model), body_2d(&model)),
    };
    html! {
        <>
        <header>
            { header("Cellular automata lab", "Every rule here is a lookup table. Rotations turn each cell's neighborhood into a number, for the whole array at once, and the number picks the cell's next state from the table. Change the table and you change the universe.", about(include_str!("../../demo.toml"))) }
            { mode_tabs(&model) }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls }
            { notice(&model.notice) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
