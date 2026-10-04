//! The page: the tank, the arrays of a step, the inspector, the program.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{chip, footer, header, notice, panel};
use microscope::colour;
use microscope::source::code;

use crate::micro::{Anatomy, COLS, ROWS, SCENES};
use crate::model::{Action, Model};
use crate::view::{source, surface, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Scene => ("scene", "wall ; src ; c2", vec![ROWS, COLS], "a mask or map per cell"),
        Stage::Spread => ("spread", "u:l_ap u", vec![ROWS, COLS], "one number per cell"),
        Stage::Step => ("step", "u:s_tep s", vec![3, ROWS, COLS], "3 planes: the surface now, a step ago, and the time"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let on_scene = Callback::from(move |e: Event| {
        let i = e.target_unchecked_into::<HtmlSelectElement>().selected_index();
        d.dispatch(Action::Scene(i.max(0) as usize));
    });
    let d = m.dispatcher();
    let on_speed = Callback::from(move |e: Event| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::PerFrame(v.parse().unwrap_or(4)));
    });
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
            <button onclick={act(m, || Action::Tick)}>{"Step"}</button>
            <button onclick={act(m, || Action::Clear)}>{"Calm the water"}</button>
            <select onchange={on_scene} aria-label="Scene">
                { for SCENES.iter().enumerate().map(|(i, (n, _))| html! { <option selected={i == m.scene}>{*n}</option> }) }
            </select>
            <label class="slider">{"steps per frame"}
                <input type="range" min="1" max="20" value={m.per_frame.to_string()} onchange={on_speed} />
                <b>{m.per_frame}</b>
            </label>
            <span class="gen">{format!("step {}; X_eTaL ran {} steps in {:.0} ms", m.steps, m.per_frame, m.ms)}</span>
        </div>
    }
}

fn small(m: &Model, rgba: Vec<u8>, caption: &str) -> Html {
    html! { <figure>
        <Canvas rows={ROWS} cols={COLS} rgba={Rc::new(rgba)} mark={Some(m.selected)} class="mid" />
        <figcaption>{code(caption)}</figcaption>
    </figure> }
}

fn arrays(m: &Model, a: &Anatomy) -> Html {
    html! { <>
        { panel("1. The scene, as arrays:", "wall ; src ; c2", "Walls (0) hold the surface at 0; the source cells are shaken up and down; c2 is the wave speed squared (slower water bends waves).",
            m.focus == Stage::Scene, html! { <div class="pair">{small(m, colour::mask(&a.wall.iter().map(|w| 1.0 - w).collect::<Vec<_>>()), "wall")}{small(m, colour::mask(&a.src), "src")}{small(m, colour::field(&a.c2, 0.0, 0.5), "c2")}</div> }) }
        { panel("2. Spread:", "u:l_ap u", "Four shifted copies of the surface minus four times it: how far each cell is from its neighbours' level.",
            m.focus == Stage::Spread, html! { <div class="pair">{small(m, colour::signed(&a.lap), "u:l_ap u")}</div> }) }
        { panel("3. Step:", "nxt := damp * wall * ((2.0 * u) - p) + (c2 * u:l_ap u) + drive", "Each cell keeps its momentum (2u minus where it was a step ago) and is pulled towards its neighbours; walls stay at 0, the sponge damps the edges.",
            m.focus == Stage::Step, html! { <div class="pair">{small(m, colour::signed(&a.p), "p")}{small(m, colour::signed(&a.u), "u")}{small(m, colour::signed(&a.next.u), "nxt")}</div> }) }
    </> }
}

fn n4(x: f64) -> String {
    format!("{x:.4}")
}

fn inspector(m: &Model, a: &Anatomy) -> Html {
    let (y, x) = m.selected;
    let i = y * COLS + x;
    let at = |dy: i64, dx: i64| {
        let (yy, xx) = ((y as i64 + dy).rem_euclid(ROWS as i64) as usize, (x as i64 + dx).rem_euclid(COLS as i64) as usize);
        a.u[yy * COLS + xx]
    };
    let lap = format!("({} + {} + {} + {}) - 4.0 * {}", n4(at(-1, 0)), n4(at(1, 0)), n4(at(0, -1)), n4(at(0, 1)), n4(at(0, 0)));
    let drive = 0.5 * a.src[i] * (0.6 * (a.next.t - 1.0)).sin();
    let nxt = format!("{} * {} * ((2.0 * {}) - {}) + ({} * {}) + {}", n4(a.damp[i]), a.wall[i], n4(a.u[i]), n4(a.p[i]), n4(a.c2[i]), n4(a.lap[i]), n4(drive));
    html! {
        <section class="panel inspector">
            <h2>{format!("Cell row {}, column {}, in the last step", y + 1, x + 1)}</h2>
            <p class="calc">{code("u:l_ap u")}{" = "}{code(&lap)}{" = "}{code(&n4(a.lap[i]))}</p>
            <p class="calc">{code("nxt")}{" = "}{code(&nxt)}{" = "}{code(&n4(a.next.u[i]))}</p>
            <p class="note">{"Every number is from the arrays X_eTaL printed. Click the tank to drop a ripple there and inspect that cell."}</p>
        </section>
    }
}

fn tank(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let m: &Model = model;
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Click(y, x))));
    let rgba = Rc::new(surface(&a.next.u, &a.wall, &a.src));
    let body = html! { <Canvas rows={ROWS} cols={COLS} {rgba} mark={Some(m.selected)} {onclick} class="big" /> };
    panel("The tank: the surface", "u", "Red crests, blue troughs, dark walls, the source in violet. Click to drop a ripple.", false, body)
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
    let body = match &model.last {
        Some(a) => html! {
            <div class="layout even">
                <div class="col">{ tank(&model, a) }{ inspector(&model, a) }</div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran in your browser for the last frame: the tank's size, the coordinates and sponge, the scene written as masks, the core of wave-tank.xtl, the surface written in by the page (folded: click to show the numbers), and the lines that step and print. The stage you pick is highlighted."}</p>
                        { source(&model.program, model.scene, model.focus) }
                    </section>
                    { arrays(&model, a) }
                </div>
            </div>
        },
        None => html! {},
    };
    html! {
        <>
        <header>
            { header("Wave tank", "The wave equation on a grid: each step, every cell carries on moving and is pulled towards its neighbours, all at once. Waves spread, pass through slits, interfere and bend in slow water. Click to drop a ripple.") }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&model.notice) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
