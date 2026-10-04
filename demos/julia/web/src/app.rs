//! The page: the Julia set, the Mandelbrot map c is picked on, the
//! program, and the two calls of the one function.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{about, chip, footer, header, notice, panel};
use microscope::source::{code, line, NONE};

use crate::colour;
use crate::micro::{call, Set, View, PRESETS};
use crate::model::{Action, Model, K};
use crate::view::{picker_source, source, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (r, c) = (m.view.rows, m.view.cols);
    let (name, src, dims, meaning): (&str, String, Vec<usize>, String) = match s {
        Stage::Grid => ("z0: the grid", "grid".into(), vec![2, r, c], format!("2 planes (real, imaginary) of {r} by {c} points")),
        Stage::Iterate => ("k steps", "c u:i_terate z0".into(), vec![3, r, c], "3 planes: z real, z imaginary, the count".into()),
        Stage::Call => ("c: one number", format!("{:.3} c_at {:.3}", m.c.0, m.c.1), vec![2], "2 numbers: c's real and imaginary parts".into()),
    };
    chip(name, &src, &dims, &meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let onchange = Callback::from(move |e: Event| {
        let i = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::Preset(i.parse().unwrap_or(0)));
    });
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play: walk c around the set" } }</button>
            <select {onchange} aria-label="c">
                { for PRESETS.iter().enumerate().map(|(i, (n, a, b))| html! {
                    <option value={i.to_string()} selected={i == m.preset}>{format!("{n}: c = {a} {b:+}i")}</option> }) }
            </select>
            <button onclick={act(m, || Action::ZoomOut)}>{"Zoom out"}</button>
            <button onclick={act(m, || Action::Reset)}>{"Reset view"}</button>
            <span class="gen">{format!("c = {:+.5} {:+.5}i; zoom {:.0}x; X_eTaL ran in {:.0} ms", m.c.0, m.c.1, View::JULIA.w / m.view.w, m.ms)}</span>
        </div>
    }
}

fn picture(model: &UseReducerHandle<Model>) -> Html {
    let m: &Model = model;
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::ZoomIn(y, x))));
    let body = match &m.julia {
        Ok(j) => html! { <Canvas rows={m.view.rows} cols={m.view.cols} rgba={Rc::new(colour::escape(j, K))} {onclick} class="big" /> },
        Err(e) => html! { <pre class="error">{e}</pre> },
    };
    let title = "The Julia set for this c:";
    panel(title, &call(Set::Julia(m.c.0, m.c.1)), "Black: points z0 that stay within 2 for all the steps. Click to zoom in (2x).", m.focus != Stage::Grid && m.focus != Stage::Iterate, body)
}

fn picker(model: &UseReducerHandle<Model>) -> Html {
    let m: &Model = model;
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Pick(y, x))));
    let v = View::PICKER;
    let body = match &m.map {
        Ok(map) => html! { <Canvas rows={v.rows} cols={v.cols} rgba={Rc::new(colour::escape(map, 32))} mark={v.pixel(m.c)} {onclick} class="big" /> },
        Err(e) => html! { <pre class="error">{e}</pre> },
    };
    panel("Pick c on the Mandelbrot set:", "grid u:i_terate 0.0 * grid", "The same function with c the grid and z0 = 0. Click a point to use it as c: inside the black set the Julia set is connected; outside it falls apart into dust.", false, body)
}

fn compare(m: &Model) -> Html {
    let body = html! { <div class="calls">
        <p>{"Mandelbrot: c is the grid, z0 is 0"}</p>
        { line("grid u:i_terate 0.0 * grid", NONE) }
        <p>{"Julia: c is one number, z0 is the grid"}</p>
        { line(&call(Set::Julia(m.c.0, m.c.1))["z := ".len()..], NONE) }
        <p class="note">{"Inside the step, "}{code("inside * cr + ...")}{" works whether "}{code("cr")}{" is a plane of the grid or a single number: X_eTaL extends a scalar over an array, so one step function serves both sets."}</p>
    </div> };
    panel("One function, two sets:", "c u:i_terate z0", "", m.focus == Stage::Call, body)
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let timer = playing.then(|| Interval::new(60, move || d.dispatch(Action::Tick)));
            move || drop(timer)
        });
    }
    html! {
        <>
        <header>
            { header("Julia sets", "The Mandelbrot set's companions. One X_eTaL function iterates z \u{00d7} z + c: give it the grid as c and you get the Mandelbrot set; give it one number as c and the grid as the starting z, and you get that c's Julia set.", about(include_str!("../../demo.toml"))) }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&model.notice) }
        </header>
        <main>
            <div class="layout even">
                <div class="col">{ picture(&model) }{ picker(&model) }</div>
                <div class="col">
                    { compare(&model) }
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran in your browser for the Julia set: the view and c (written in by the page), the core of julia.xtl, the call and the line that prints the counts. The stage you pick is highlighted."}</p>
                        { source(&model.program, model.focus) }
                        <p class="note">{"And the program for the picker, the same function with c the grid:"}</p>
                        { picker_source(&model.picker_text) }
                    </section>
                </div>
            </div>
        </main>
        { footer() }
        </>
    }
}
