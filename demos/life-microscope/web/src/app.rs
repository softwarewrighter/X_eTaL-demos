//! The page: controls, the stage timeline, the arrays, the inspector.

use gloo_timers::callback::Interval;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use microscope::cells::{grid, Grid, Paint};
use microscope::chrome::{about, chip, footer, header, panel as frame};
use microscope::source::code;

use crate::micro::{Anatomy, OFFSETS, PATTERNS};
use crate::model::{Action, Model, Stage, COLS, ROWS, STAGES};
use crate::view::{line, source};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_info(s: Stage) -> (&'static str, &'static str, Vec<usize>, String) {
    let board = format!("{ROWS} rows by {COLS} columns");
    match s {
        Stage::Board => ("board", "_r", vec![ROWS, COLS], board),
        Stage::Rotate => ("rotate", "-1 0 1 o_-_12", vec![3, 3, ROWS, COLS],
            format!("3 row offsets by 3 column offsets by a {ROWS} by {COLS} board: nine shifted boards")),
        Stage::Sum => ("sum", "'+ r_/_12", vec![ROWS, COLS], format!("{board}: the two offset axes summed away")),
        Stage::Masks => ("compare", "s = 3 ; b * s = 4", vec![ROWS, COLS], format!("two boards, each {board}")),
        Stage::Next => ("add", "+", vec![ROWS, COLS], board),
    }
}

fn timeline(m: &UseReducerHandle<Model>) -> Html {
    let chips = STAGES.iter().map(|&s| {
        let (name, src, dims, meaning) = stage_info(s);
        chip(name, src, &dims, &meaning, m.focus == s, act(m, move || Action::Focus(s)))
    });
    html! { <nav class="timeline">{ for chips }</nav> }
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let onchange = Callback::from(move |e: Event| {
        let i = e.target_unchecked_into::<HtmlSelectElement>().selected_index();
        d.dispatch(Action::Pattern(i.max(0) as usize));
    });
    let draw = m.drawing;
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::Step)}>{"Step"}</button>
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
            <button onclick={act(m, || Action::Reset)}>{"Reset"}</button>
            <button onclick={act(m, || Action::Clear)}>{"Clear"}</button>
            <select {onchange} aria-label="Pattern">
                { for PATTERNS.iter().enumerate().map(|(i, (n, _))| html! {
                    <option selected={i == m.pattern}>{*n}</option> }) }
            </select>
            <label class="toggle">
                <input type="checkbox" checked={draw} onclick={act(m, move || Action::Drawing(!draw))} />
                {"Click draws cells"}
            </label>
            <span class="gen">{format!("generation {}, {} alive", m.generation, m.board.population())}</span>
        </div>
    }
}

fn panel(m: &Model, stage: Stage, title: &str, src: &str, note: &str, body: Html) -> Html {
    frame(title, src, note, m.focus == stage, body)
}

fn small(m: &Model, cells: &[u8], paint: Paint) -> Html {
    let selected = Some(m.selected);
    grid(Grid { cells, rows: ROWS, cols: COLS, paint, selected, size: "mid", onclick: None })
}

fn shifted(m: &Model, a: &Anatomy) -> Html {
    let minis = a.shifted.iter().enumerate().map(|(k, cells)| {
        let (dy, dx) = (OFFSETS[k / 3], OFFSETS[k % 3]);
        let g = Grid { cells, rows: ROWS, cols: COLS, paint: Paint::Cells, selected: Some(m.selected), size: "mini", onclick: None };
        html! { <figure>{grid(g)}<figcaption>{format!("{dy:+} {dx:+}")}</figcaption></figure> }
    });
    html! { <div class="nine">{ for minis }</div> }
}

fn inspector(m: &Model, a: &Anatomy) -> Html {
    let (y, x) = m.selected;
    let i = y * COLS + x;
    let nb = a.shifted.iter().map(|s| html! { <td class={if s[i] == 1 {"on"} else {"off"}}>{s[i]}</td> });
    let nb: Vec<Html> = nb.collect();
    let rows = nb.chunks(3).map(|r| html! { <tr>{ for r.iter().cloned() }</tr> });
    let (s, alive) = (a.sum[i], m.board.get(y, x));
    let why = match (a.three[i], a.four[i]) {
        (1, _) => "S = 3: alive next (born, or alive with 2 neighbours)",
        (_, 1) => "alive and S = 4: stays alive (3 neighbours)",
        _ if alive == 1 => "alive, but S is not 3 or 4: dies",
        _ => "dead, and S is not 3: stays dead",
    };
    html! {
        <section class="panel inspector">
            <h2>{format!("Cell row {}, column {}", y + 1, x + 1)}</h2>
            <p class="note">{"Its value in each of the nine shifted boards: its 3 by 3 neighbourhood."}</p>
            <table class="nbhd">{ for rows }</table>
            <p class="calc">{"S is "}{code(&s.to_string())}{", and "}{code(&format!("({s} = 3) + {alive} * {s} = 4"))}{" is "}{code(&a.next[i].to_string())}</p>
            <p class="note">{why}</p>
        </section>
    }
}

fn arrays(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let m: &Model = model;
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Cell(y, x))));
    let board = grid(Grid { cells: &m.board.cells, rows: ROWS, cols: COLS, paint: Paint::Cells, selected: Some(m.selected), size: "big", onclick });
    html! {
        <div class="layout">
            <div class="col">
                { panel(m, Stage::Board, "The board", "b", "Click a cell to inspect it (or tick \"Click draws cells\"). The edges wrap around.", board) }
                { inspector(m, a) }
                <section class="panel code">
                    <h2>{"The program"}</h2>
                    <p class="note">{"Exactly the program X_eTaL ran in your browser for this board: the rule's one line, the board written in by the page (folded: click to show the cells), and the lines that print every array shown. The stage you pick is highlighted."}</p>
                    { source(&m.program, m.focus) }
                </section>
            </div>
            <div class="col">
                { panel(m, Stage::Rotate, "1. Rotate by every offset:", "-1 0 1 o_-_12 b", "Nine copies of the board, shifted by -1, 0 or +1 rows and columns: a 3 by 3 by 16 by 16 array. In each copy the selected cell holds one of its neighbours.", shifted(m, a)) }
                { panel(m, Stage::Sum, "2. Sum over the two offset axes:", "s := '+ r_/_12 r", "S: each cell plus its eight neighbours, the nine copies added together at once. No loop over cells, no loop over neighbours.", small(m, &a.sum, Paint::Sum)) }
                { panel(m, Stage::Masks, "3. Compare:", "s = 3 ; b * s = 4", "Two boolean boards: alive next whatever it is now (S = 3), and alive now with three neighbours (S = 4).", html!{ <div class="pair">{small(m, &a.three, Paint::Cells)}{small(m, &a.four, Paint::Cells)}</div> }) }
                { panel(m, Stage::Next, "4. Add, the next board:", "(s = 3) + b * s = 4", "The whole rule, for every cell at once.", small(m, &a.next, Paint::Cells)) }
            </div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(|| Model::new(0));
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let timer = playing.then(|| Interval::new(350, move || d.dispatch(Action::Step)));
            move || drop(timer)
        });
    }
    let body = match &model.anatomy {
        Ok(a) => arrays(&model, a),
        Err(e) => html! { <pre class="error">{e}</pre> },
    };
    html! {
        <>
        <header>
            { header("Life microscope", "Conway's Life is one line of X_eTaL. Step it and watch every array that line builds; read it right to left.", about(include_str!("../../demo.toml"))) }
            { line(model.focus) }
            { timeline(&model) }
            { controls(&model) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
