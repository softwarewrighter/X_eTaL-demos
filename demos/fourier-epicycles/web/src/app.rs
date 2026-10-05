//! The page: circles on circles tracing a curve, the spectrum, the
//! angle table, the error, the program.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{about, chip, footer, header, notice, panel};
use microscope::color;
use microscope::source::code;

use crate::micro::{Anatomy, N, PRESETS};
use crate::model::{Action, Model};
use crate::view::{source, Stage, STAGES};

const VIEW: f64 = 1.35;

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn num(x: f64) -> String {
    if x == 0.0 || !x.is_finite() {
        return "0.0".into();
    }
    let digits = (3 - x.abs().log10().floor() as i32).clamp(1, 8) as usize;
    format!("{x:.digits$}")
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Angles => ("angles", "k '* t_able k", vec![N, N], "the angle of frequency k at sample j"),
        Stage::Transform => ("transform", "re ; im", vec![N], "each frequency's real and imaginary part"),
        Stage::Circles => ("circles", "vx ; vy", vec![N, N], "each circle's vector at each time, strongest first"),
        Stage::Chain => ("chain", "'+ s_\\_2 vx", vec![N, N], "where the first c circles end, at each time"),
        Stage::Error => ("error", "err", vec![N], "the mean error rebuilt from 1, 2, ... circles"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let on_preset = Callback::from(move |e: Event| {
        let i = e.target_unchecked_into::<HtmlSelectElement>().selected_index();
        if i >= 0 && (i as usize) < PRESETS.len() {
            d.dispatch(Action::Preset(i as usize));
        }
    });
    let d = m.dispatcher();
    let on_k = Callback::from(move |e: InputEvent| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::Circles(v.parse().unwrap_or(12)));
    });
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
            <select key={format!("{:?}", m.preset)} onchange={on_preset} aria-label="Curve">
                { for PRESETS.iter().enumerate().map(|(i, p)| html! { <option selected={m.preset == Some(i)}>{*p}</option> }) }
                { for m.preset.is_none().then(|| html! { <option selected=true disabled=true>{"your drawing"}</option> }) }
            </select>
            <label class="slider">{"circles"}
                <input type="range" min="1" max={N.to_string()} value={m.circles.to_string()} oninput={on_k} />
                <b>{m.circles}</b>
            </label>
            <span class="gen">{format!("X_eTaL transformed {N} points in {:.0} ms", m.ms)}</span>
        </div>
    }
}

/// Mouse position in the picture's coordinates (y up).
fn world(e: &MouseEvent) -> Option<(f64, f64)> {
    let el: web_sys::Element = e.current_target()?.dyn_into().ok()?;
    let r = el.get_bounding_client_rect();
    let x = ((e.client_x() as f64 - r.left()) / r.width() * 2.0 - 1.0) * VIEW;
    let y = -((e.client_y() as f64 - r.top()) / r.height() * 2.0 - 1.0) * VIEW;
    Some((x, y))
}

use wasm_bindgen::JsCast;

fn points(xs: impl Iterator<Item = (f64, f64)>) -> String {
    xs.map(|(x, y)| format!("{x:.4},{y:.4}")).collect::<Vec<_>>().join(" ")
}

fn picture(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let m: &Model = model;
    let (k, j) = (m.circles, m.frame);
    let original = points((0..N).map(|i| (a.curve.x[i], a.curve.y[i])));
    let rebuilt = points((0..=N).map(|i| a.joint(i % N, k - 1)));
    let traced = points((0..=j).map(|i| a.joint(i, k - 1)));
    let mut prev = (0.0, 0.0);
    let circles = (0..k).map(|c| {
        let center = prev;
        let end = a.joint(j, c);
        prev = end;
        let r = a.radius(c);
        html! { <g>
            { for (r > 0.004).then(|| html! { <circle class="ring" cx={center.0.to_string()} cy={center.1.to_string()} r={r.to_string()} vector-effect="non-scaling-stroke" /> }) }
            <line class="arm" x1={center.0.to_string()} y1={center.1.to_string()} x2={end.0.to_string()} y2={end.1.to_string()} vector-effect="non-scaling-stroke" />
        </g> }
    });
    let tip = a.joint(j, k - 1);
    let drawing = m.drawing.as_ref().map(|d| html! { <polyline class="drawn" points={points(d.iter().copied())} vector-effect="non-scaling-stroke" /> });
    let d = model.dispatcher();
    let down = Callback::from(move |e: MouseEvent| {
        if let Some((x, y)) = world(&e) {
            d.dispatch(Action::DrawStart(x, y));
        }
    });
    let d = model.dispatcher();
    let drawing_now = m.drawing.is_some();
    let mv = Callback::from(move |e: MouseEvent| {
        if drawing_now {
            if let Some((x, y)) = world(&e) {
                d.dispatch(Action::DrawTo(x, y));
            }
        }
    });
    let up = act(model, || Action::DrawEnd);
    let vb = format!("{} {} {} {}", -VIEW, -VIEW, 2.0 * VIEW, 2.0 * VIEW);
    let body = html! {
        <svg class="epicycles" viewBox={vb} onmousedown={down} onmousemove={mv} onmouseup={up.clone()} onmouseleave={up}
            role="img" aria-label="circles on circles tracing the curve">
            <g transform="scale(1,-1)">
                <polygon class="original" points={original} vector-effect="non-scaling-stroke" />
                <polyline class="rebuilt" points={rebuilt} vector-effect="non-scaling-stroke" />
                <polyline class="traced" points={traced} vector-effect="non-scaling-stroke" />
                { for circles }
                <circle class="tip" cx={tip.0.to_string()} cy={tip.1.to_string()} r="0.025" />
                { for drawing }
            </g>
        </svg>
    };
    let note = format!(
        "The {k} strongest circles, each turning at its own whole-number speed, chained end to end: the last one's end traces the curve (orange; gray, the curve itself). Mean error with {k}: {}. Press and drag in the picture to draw your own closed curve.",
        num(a.err[k - 1])
    );
    panel("Circles on circles:", "cx ; cy", &note, m.focus == Stage::Chain, body)
}

fn spectrum(m: &Model, a: &Anatomy) -> Html {
    // Bars by speed, -N/2 .. N/2 - 1; the circles in use colored.
    let used: Vec<usize> = a.order[..m.circles].iter().map(|o| o - 1).collect();
    let top = a.amp.iter().cloned().fold(1e-12, f64::max);
    let w = 2.0;
    let bars = (0..N).map(|i| {
        let f = i as i64 - (N / 2) as i64;
        let k = f.rem_euclid(N as i64) as usize;
        let h = 60.0 * (a.amp[k] / top).sqrt();
        html! { <rect class={classes!("bar", used.contains(&k).then_some("on"))} x={(i as f64 * w).to_string()} y={(62.0 - h).to_string()} width={(w * 0.8).to_string()} height={h.to_string()}>
            <title>{format!("speed {f}: strength {}", num(a.amp[k]))}</title>
        </rect> }
    });
    html! { <svg class="spectrum" viewBox={format!("0 0 {} 64", N as f64 * w)} preserveAspectRatio="none">{ for bars }</svg> }
}

fn error_curve(m: &Model, a: &Anatomy) -> Html {
    let lo = a.err.iter().cloned().filter(|e| *e > 0.0).fold(f64::MAX, f64::min).max(1e-6).log10();
    let hi = a.err[0].max(1e-6).log10().max(lo + 1e-9);
    let pt = |c: usize| {
        let y = 60.0 - 56.0 * (a.err[c].max(1e-6).log10() - lo) / (hi - lo);
        (c as f64 * 2.0, y)
    };
    let line = (0..N).map(|c| { let (x, y) = pt(c); format!("{x:.1},{y:.1}") }).collect::<Vec<_>>().join(" ");
    let (cx, cy) = pt(m.circles - 1);
    html! { <svg class="errcurve" viewBox={format!("0 0 {} 64", N * 2)} preserveAspectRatio="none">
        <polyline points={line} /><circle cx={cx.to_string()} cy={cy.to_string()} r="2.5" />
    </svg> }
}

fn arrays(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let m: &Model = model;
    let f = m.focus;
    html! { <>
        { panel("1. The angles:", "A := (2.0 * (p_i @) / f_loat n) * k '* t_able k", "One outer product: row k, column j holds 2 pi k j / n, the angle of frequency k at sample j. Drawn: its cosines.",
            f == Stage::Angles, html! { <div class="pair"><figure><Canvas rows={N} cols={N} rgba={Rc::new(color::signed(&a.cos))} class="mid" /><figcaption>{code("C := c_os A")}</figcaption></figure></div> }) }
        { panel("2. The transform:", "re := ((C '+ '* i_nner x) + S '+ '* i_nner y) / f_loat n", "Two matrix products give each frequency's part of the curve: the strength of each speed, colored where its circle is drawn.",
            f == Stage::Transform, spectrum(m, a)) }
        { panel("3. The circles, strongest first:", "order := g_rade n_eg amp", "Sorted by strength; at time j, circle c is its frequency's part turned by its speed times the angle of time j.",
            f == Stage::Circles, html! { <p class="calc">{format!("the three strongest: speeds {}, {}, {}; radii {}, {}, {}", a.f[a.order[0] - 1], a.f[a.order[1] - 1], a.f[a.order[2] - 1], num(a.radius(0)), num(a.radius(1)), num(a.radius(2)))}</p> }) }
        { panel("4. The chain:", "cx := '+ s_\\_2 vx", "A running sum along the circles: column c is where the first c circles end, so every reconstruction (1, 2, ... 128 circles) comes from one scan.",
            f == Stage::Chain, html! {}) }
        { panel("5. The error:", "err := ('+ r_/ ((dx * dx) + dy * dy) ^ 0.5) / f_loat n", "The mean distance from the curve for each number of circles (log scale); the dot is the number drawn.",
            f == Stage::Error, error_curve(m, a)) }
    </> }
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
    let body = match &model.last {
        Some(a) => html! {
            <div class="layout even">
                <div class="col">{ picture(&model, a) }{ arrays(&model, a) }</div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran in your browser for this curve: its points (folded: click to show them), the core of fourier-epicycles.xtl, and the lines that print what is drawn. The stage you pick is highlighted."}</p>
                        { source(&model.program, model.focus) }
                    </section>
                </div>
            </div>
        },
        None => html! {},
    };
    html! {
        <>
        <header>
            { header("Fourier epicycles", "Any closed curve is a sum of circles turning at whole-number speeds. X_eTaL finds them with a discrete Fourier transform written as two matrix products, sorts them by size, and chains them end to end; one running sum gives the curve rebuilt from any number of circles. Pick a curve or draw one.", about(include_str!("../../demo.toml"))) }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&model.notice) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
