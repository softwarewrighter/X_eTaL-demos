//! The page: controls, stages, the picture, the arrays, the orbit.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{chip, footer, header, notice, panel as frame};
use microscope::source::code;

use crate::colour;
use crate::micro::{escape, Frame};
use crate::model::{Action, Model, K_MAX};
use crate::view::{source, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (r, c) = (m.view.rows, m.view.cols);
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, String) = match s {
        Stage::Grid => ("broadcast c", "(o_ffsets rows) 'r_ight t_able re", vec![r, c], format!("{r} rows by {c} columns of points")),
        Stage::Step => ("one step", "u:s_tep", vec![3, r, c], "3 planes (z real, z imaginary, count) of the grid".into()),
        Stage::Iterate => ("k steps", "k 'u:s_tep p_ower", vec![3, r, c], "the state after k steps".into()),
        Stage::Measure => ("measure", "counts := 3 s_elect z", vec![r, c], "one number per point".into()),
    };
    chip(name, src, &dims, &meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let onchange = Callback::from(move |e: Event| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::SetK(v.parse().unwrap_or(1)));
    });
    let zooming = m.zooming;
    html! {
        <div class="controls">
            <label class="slider">{ code("k :=") }
                <input type="range" min="0" max={K_MAX.to_string()} value={m.k.to_string()} {onchange} />
                <b>{m.k}</b>
            </label>
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play steps" } }</button>
            <button onclick={act(m, || Action::ZoomOut)}>{"Zoom out"}</button>
            <button onclick={act(m, || Action::Reset)}>{"Reset"}</button>
            <label class="toggle">
                <input type="checkbox" checked={zooming} onclick={act(m, move || Action::Zooming(!zooming))} />
                {"Click zooms in"}
            </label>
            <span class="gen">{format!("centre {:.6} {:+.6}i, width {:.3e}, zoom {:.0}x; X_eTaL ran in {:.0} ms", m.view.cx, m.view.cy, m.view.w, 3.0 / m.view.w, m.ms)}</span>
        </div>
    }
}

fn panel(m: &Model, stage: Option<Stage>, title: &str, src: &str, note: &str, body: Html) -> Html {
    frame(title, src, note, stage == Some(m.focus), body)
}

fn picture(model: &UseReducerHandle<Model>, f: &Frame) -> Html {
    let m: &Model = model;
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Click(y, x))));
    let rgba = Rc::new(colour::escape(&f.counts, f.k));
    let hint = if m.zooming { "Click to zoom in there (2x)." } else { "Click a point to see its orbit; tick \"Click zooms in\" to zoom." };
    let body = html! { <Canvas rows={m.view.rows} cols={m.view.cols} {rgba} mark={Some(m.selected)} {onclick} class="big" /> };
    panel(m, Some(Stage::Measure), "The picture: steps each point stayed inside,", "counts", &format!("Black: still within 2 of the origin after k = {} steps (the set, so far). {hint}", f.k), body)
}

fn small(m: &Model, rgba: Vec<u8>) -> Html {
    html! { <Canvas rows={m.view.rows} cols={m.view.cols} rgba={Rc::new(rgba)} mark={Some(m.selected)} class="mid" /> }
}

fn arrays(m: &Model, f: &Frame) -> Html {
    let grid = html! { <div class="pair">
        <figure>{small(m, colour::signed(&f.cr))}<figcaption>{code("cr")}{" real parts"}</figcaption></figure>
        <figure>{small(m, colour::signed(&f.ci))}<figcaption>{code("ci")}{" imaginary parts"}</figcaption></figure>
    </div> };
    let after = html! { <div class="pair">
        <figure>{small(m, colour::magnitude(&f.m2))}<figcaption>{code("m2")}{" = |z|"}<sup>{"2"}</sup></figcaption></figure>
        <figure>{small(m, colour::mask(&f.inside))}<figcaption>{code("4 >= m2")}{" inside"}</figcaption></figure>
    </div> };
    html! { <>
        { panel(m, Some(Stage::Grid), "1. c for every point, by broadcasting:", "(o_ffsets rows) 'r_ight t_able re", "One row of real parts and one column of imaginary parts, spread over the grid by t_able. No loop over points.", grid) }
        { panel(m, Some(Stage::Step), "2. After k steps, for every point at once:", "k 'u:s_tep p_ower", "Each step computes z * z + c on the whole grid; points outside the disk keep their z. Left: |z| squared (white past 4). Right: the mask of points still inside.", after) }
    </> }
}

fn orbit_svg(points: &[(f64, f64)], esc: Option<usize>) -> Html {
    let s = |v: f64| 120.0 + v.clamp(-2.4, 2.4) * 50.0;
    let shown = &points[..esc.map_or(points.len(), |e| (e + 1).min(points.len()))];
    let path: Vec<String> = shown.iter().map(|&(a, b)| format!("{:.1},{:.1}", s(a), s(-b))).collect();
    let dots = shown.iter().enumerate().map(|(i, &(a, b))| {
        let class = if Some(i) == esc { "dot out" } else { "dot" };
        html! { <circle {class} cx={format!("{:.1}", s(a))} cy={format!("{:.1}", s(-b))} r="3" /> }
    });
    html! {
        <svg class="orbit" viewBox="0 0 240 240" role="img" aria-label="the orbit in the complex plane">
            <line class="axis" x1="0" y1="120" x2="240" y2="120" />
            <line class="axis" x1="120" y1="0" x2="120" y2="240" />
            <circle class="disk" cx="120" cy="120" r="100" />
            <polyline class="path" points={path.join(" ")} />
            { for dots }
        </svg>
    }
}

fn orbit_panel(m: &Model) -> Html {
    let (y, x) = m.selected;
    let (a, b) = m.view.point(y, x);
    let body = match &m.orbit {
        Err(e) => html! { <pre class="error">{e}</pre> },
        Ok(o) => {
            let esc = escape(o);
            let verdict = match esc {
                Some(n) => format!("left the disk at step {n}"),
                None => format!("still inside after {} steps", o.len().saturating_sub(1)),
            };
            let rows = o.iter().take(12).enumerate().map(|(n, &(re, im))| html! {
                <tr class={(Some(n) == esc).then_some("out")}><td>{n}</td><td>{format!("{re:+.5}")}</td><td>{format!("{im:+.5}i")}</td><td>{format!("{:.4}", re * re + im * im)}</td></tr>
            });
            html! { <>
                <p class="note">{format!("c = {a:+.6} {b:+.6}i: {verdict}.")}</p>
                <div class="orbitbox">
                    { orbit_svg(o, esc) }
                    <table class="zs"><tr><th>{"n"}</th><th>{"re"}</th><th>{"im"}</th><th>{"|z|\u{b2}"}</th></tr>{ for rows }</table>
                </div>
            </> }
        }
    };
    panel(m, None, "The orbit of the marked point:", "k 'u:o_rbit p_ower 0.0 0.0", "z0, z1, ... for this one c, computed by a second X_eTaL program. The circle is |z| = 2.", body)
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let timer = playing.then(|| Interval::new(120, move || d.dispatch(Action::Tick)));
            move || drop(timer)
        });
    }
    let body = match &model.frame {
        Ok(f) => html! {
            <div class="layout even">
                <div class="col">{ picture(&model, f) }{ orbit_panel(&model) }</div>
                <div class="col">
                    <section class={classes!("panel", "code")}>
                        <h2>{"The program"}</h2>
                        <p class="note">{"The core of mandelbrot.xtl, run by X_eTaL in your browser; the stage you pick is highlighted."}</p>
                        { source(model.focus) }
                    </section>
                    { arrays(&model, f) }
                </div>
            </div>
        },
        Err(e) => html! { <pre class="error">{e}</pre> },
    };
    html! {
        <>
        <header>
            { header("Mandelbrot", "Every point c of the picture iterated at once: z becomes z \u{00d7} z + c, over the whole grid, as one array expression. Step k up and watch the set appear.") }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&model.notice) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
