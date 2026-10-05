//! The page: the board, the last step's arithmetic, the program.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{about, chip, footer, header, notice, panel};
use microscope::source::code;

use crate::micro::{Last, N};
use crate::model::{Action, Model};
use crate::view::{board, source, Stage, STAGES};

const DIRS: [&str; 4] = ["up", "right", "down", "left"];

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Look => ("look", "'+ r_/ r_avel a * b", vec![], "one number: the color under the ant"),
        Stage::Turn => ("turn", "(d + 1 + 2 * cell) m_od 4", vec![], "one number: the new direction"),
        Stage::Flip => ("flip", "b + a * 1 - 2 * b", vec![N, N], "the board"),
        Stage::Move => ("move", "dy o_-_1 dx o_-_2 a", vec![N, N], "the ant's mask, rotated one cell"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let on_speed = Callback::from(move |e: Event| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::PerFrame(v.parse().unwrap_or(100)));
    });
    let per_step = if m.per_frame > 0 { m.ms / m.per_frame as f64 } else { 0.0 };
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
            <button onclick={act(m, || Action::One)}>{"One step"}</button>
            <button onclick={act(m, || Action::Reset)}>{"Reset"}</button>
            <label class="slider">{"steps per frame"}
                <input type="range" min="1" max="1000" value={m.per_frame.to_string()} onchange={on_speed} />
                <b>{m.per_frame}</b>
            </label>
            <span class="gen">{format!("step {}; {:.2} ms a step", m.ant.steps, per_step)}</span>
        </div>
    }
}

fn last_step(m: &Model, l: &Last) -> Html {
    let turn = (l.dir + 1 + 2 * l.cell) % 4;
    let color = if l.cell == 0 { "white" } else { "black" };
    let side = if l.cell == 0 { "right" } else { "left" };
    let (dy, dx) = ((turn == 0) as i64 - (turn == 2) as i64, (turn == 3) as i64 - (turn == 1) as i64);
    let body = html! { <>
        <p class="calc">{format!("The ant stood at row {}, column {}, facing {}, on a {color} cell.", l.y + 1, l.x + 1, DIRS[l.dir as usize])}</p>
        <p class="calc">{code("cell")}{" = "}{code(&l.cell.to_string())}{format!(" ({color}), so it turns {side}: ")}{code(&format!("({} + 1 + 2 * {}) m_od 4", l.dir, l.cell))}{" = "}{code(&turn.to_string())}{format!(" ({})", DIRS[turn as usize])}</p>
        <p class="calc">{"It flips the cell: "}{code(&format!("{} + 1 * 1 - 2 * {}", l.cell, l.cell))}{" = "}{code(&(1 - l.cell).to_string())}</p>
        <p class="calc">{"It moves by rotating its mask: "}{code(&format!("{dy} o_-_1 {dx} o_-_2 a"))}</p>
        <p class="note">{"The ant is a mask with a single 1, so looking, flipping and moving are whole-array operations: the same code would move a thousand ants."}</p>
    </> };
    panel("The last step:", "u:s_tep s", "", m.focus == Stage::Turn || m.focus == Stage::Look, body)
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let timer = playing.then(|| Interval::new(30, move || d.dispatch(Action::Tick)));
            move || drop(timer)
        });
    }
    let m: &Model = &model;
    let rgba = Rc::new(board(&m.ant.board, m.ant.y * N + m.ant.x));
    let black = m.ant.board.iter().filter(|&&c| c == 1).count();
    let pic = html! { <Canvas rows={N} cols={N} {rgba} class="big" /> };
    let note = format!("{black} black cells. The ant (violet) wanders chaotically for about 10,000 steps, then builds a diagonal highway forever (the board wraps around at its edges).");
    html! {
        <>
        <header>
            { header("Langton's ant", "On a white cell, turn right; on a black cell, turn left; flip the cell and step forward. From two rules comes chaos, and then, after about 10,000 steps, order: a highway. Here the ant is arrays: a one-hot mask and a direction.", about(include_str!("../../demo.toml"))) }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&m.notice) }
        </header>
        <main>
            <div class="layout even">
                <div class="col">
                    { panel("The board:", "b", &note, m.focus == Stage::Flip || m.focus == Stage::Move, pic) }
                </div>
                <div class="col">
                    { for m.last.iter().map(|l| last_step(m, l)) }
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran in your browser for the last frame: the board's size, the core of langtons-ant.xtl, the board written in by the page (folded: click to show the numbers), the ant as a mask, and the lines that step and print. The stage you pick is highlighted."}</p>
                        { source(&m.program, m.focus) }
                    </section>
                </div>
            </div>
        </main>
        { footer() }
        </>
    }
}
