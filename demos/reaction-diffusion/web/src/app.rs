//! The page: controls, stages, the picture, the arrays, the inspector.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{about, chip, footer, header, notice, panel as frame};
use microscope::colour;
use microscope::source::code;

use crate::micro::{Anatomy, PRESETS};
use crate::model::{Action, Model, N};
use crate::view::{source, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Spread => ("spread", "u:l_ap u", vec![N, N], "one number per cell"),
        Stage::React => ("react", "u * v * v", vec![N, N], "one number per cell"),
        Stage::Update => ("update", "u:s_tep s", vec![2, N, N], "two planes, U and V, of the grid"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let on_preset = Callback::from(move |e: Event| {
        let i = e.target_unchecked_into::<HtmlSelectElement>().selected_index();
        d.dispatch(Action::Preset(i.max(0) as usize));
    });
    let d = m.dispatcher();
    let on_speed = Callback::from(move |e: Event| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::PerFrame(v.parse().unwrap_or(20)));
    });
    let seeding = m.seeding;
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
            <button onclick={act(m, || Action::Tick)}>{"Step"}</button>
            <button onclick={act(m, || Action::Reset)}>{"Reset"}</button>
            <select onchange={on_preset} aria-label="Pattern">
                { for PRESETS.iter().enumerate().map(|(i, (n, f, k))| html! {
                    <option selected={i == m.preset}>{format!("{n} (f {f}, k {k})")}</option> }) }
            </select>
            <label class="slider">{"steps per frame"}
                <input type="range" min="1" max="60" value={m.per_frame.to_string()} onchange={on_speed} />
                <b>{m.per_frame}</b>
            </label>
            <label class="toggle">
                <input type="checkbox" checked={seeding} onclick={act(m, move || Action::Seeding(!seeding))} />
                {"Click adds V"}
            </label>
            <span class="gen">{format!("step {}; X_eTaL ran {} steps in {:.0} ms", m.steps, m.per_frame, m.ms)}</span>
        </div>
    }
}

fn panel(m: &Model, stage: Option<Stage>, title: &str, src: &str, note: &str, body: Html) -> Html {
    frame(title, src, note, stage == Some(m.focus), body)
}

fn small(m: &Model, rgba: Vec<u8>, caption: &str, cap_note: &str) -> Html {
    html! { <figure>
        <Canvas rows={N} cols={N} rgba={Rc::new(rgba)} mark={Some(m.selected)} class="mid" />
        <figcaption>{code(caption)}{cap_note}</figcaption>
    </figure> }
}

fn arrays(m: &Model, a: &Anatomy) -> Html {
    html! { <>
        { panel(m, Some(Stage::Spread), "1. Spread:", "u:l_ap u", "Four shifted copies of the grid minus four times the grid: how much each cell differs from its neighbours. Blue below 0, red above.",
            html! { <div class="pair">{small(m, colour::signed(&a.lap_u), "u:l_ap u", "")}{small(m, colour::signed(&a.lap_v), "u:l_ap v", "")}</div> }) }
        { panel(m, Some(Stage::React), "2. React:", "uvv := u * v * v", "Where V meets U, U turns into more V: the product, for every cell at once.",
            html! { <div class="pair">{small(m, colour::scaled(&a.uvv), "u * v * v", "")}</div> }) }
        { panel(m, Some(Stage::Update), "3. Update:", "u:s_tep s", "U spreads, is used up and is fed (f); V spreads, grows and is removed (f + k).",
            html! { <div class="pair">{small(m, colour::field(&a.next.u, 0.2, 1.0), "u", " after the step")}{small(m, colour::field(&a.next.v, 0.0, 0.4), "v", " after the step")}</div> }) }
    </> }
}

fn picture(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let m: &Model = model;
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Click(y, x))));
    let hint = if m.seeding { "Click to drop V there (and inspect that cell)." } else { "Click a cell to inspect it." };
    let body = html! { <Canvas rows={N} cols={N} rgba={Rc::new(colour::field(&a.next.v, 0.0, 0.4))} mark={Some(m.selected)} {onclick} class="big" /> };
    panel(m, None, "The pattern: the chemical", "v", &format!("{N} by {N} cells; the edges wrap around. {hint}"), body)
}

fn num(x: f64) -> String {
    format!("{x:.4}")
}

fn inspector(m: &Model, a: &Anatomy) -> Html {
    let (y, x) = m.selected;
    let at = |g: &[f64], dy: usize, dx: usize| g[((y + N + dy - 1) % N) * N + (x + N + dx - 1) % N];
    let i = y * N + x;
    let r = &m.rates;
    let cross = |g: &[f64]| html! {
        <table class="nbhd num">
            <tr><td></td><td>{num(at(g, 0, 1))}</td><td></td></tr>
            <tr><td>{num(at(g, 1, 0))}</td><td class="sel">{num(at(g, 1, 1))}</td><td>{num(at(g, 1, 2))}</td></tr>
            <tr><td></td><td>{num(at(g, 2, 1))}</td><td></td></tr>
        </table>
    };
    let lap = |g: &[f64]| format!("({} + {} + {} + {}) - 4.0 * {}", num(at(g, 0, 1)), num(at(g, 2, 1)), num(at(g, 1, 0)), num(at(g, 1, 2)), num(at(g, 1, 1)));
    let nu = format!("{} + (({} * {}) - {}) + {} * 1.0 - {}", num(a.u[i]), r.du, num(a.lap_u[i]), num(a.uvv[i]), r.f, num(a.u[i]));
    let nv = format!("{} + (({} * {}) + {}) - ({} + {}) * {}", num(a.v[i]), r.dv, num(a.lap_v[i]), num(a.uvv[i]), r.f, r.k, num(a.v[i]));
    html! {
        <section class="panel inspector">
            <h2>{format!("Cell row {}, column {}, in the last step", y + 1, x + 1)}</h2>
            <div class="crosses"><div>{code("u")}{cross(&a.u)}</div><div>{code("v")}{cross(&a.v)}</div></div>
            <p class="calc">{code("u:l_ap u")}{" = "}{code(&lap(&a.u))}{" = "}{code(&num(a.lap_u[i]))}</p>
            <p class="calc">{code("u:l_ap v")}{" = "}{code(&lap(&a.v))}{" = "}{code(&num(a.lap_v[i]))}</p>
            <p class="calc">{code("u * v * v")}{" = "}{code(&num(a.uvv[i]))}</p>
            <p class="calc">{"new "}{code("u")}{" = "}{code(&nu)}{" = "}{code(&num(a.next.u[i]))}</p>
            <p class="calc">{"new "}{code("v")}{" = "}{code(&nv)}{" = "}{code(&num(a.next.v[i]))}</p>
            <p class="note">{"Every number here is from the arrays X_eTaL printed; the formulas show how the step combined them."}</p>
        </section>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(|| Model::new(0));
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let timer = playing.then(|| Interval::new(30, move || d.dispatch(Action::Tick)));
            move || drop(timer)
        });
    }
    let a = model.last.clone();
    html! {
        <>
        <header>
            { header("Reaction-diffusion", "Two chemicals on a grid: U is fed in, V turns U into more V, both spread to their neighbours. Each step is a few array expressions over the whole grid, and patterns grow by themselves.", about(include_str!("../../demo.toml"))) }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&model.notice) }
        </header>
        <main>
            <div class="layout even">
                <div class="col">{ picture(&model, &a) }{ inspector(&model, &a) }</div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran in your browser for the last frame: the rates, the core of reaction-diffusion.xtl, the two grids written in by the page (folded: click to show the numbers), and the lines that step and print. The stage you pick is highlighted."}</p>
                        { source(&model.program, model.focus) }
                    </section>
                    { arrays(&model, &a) }
                </div>
            </div>
        </main>
        { footer() }
        </>
    }
}
