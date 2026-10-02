//! The page: the bodies and their trails, the forces on the body you
//! pick, the matrices of the cube and its reduction, the program.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{chip, footer, header, notice, panel};
use microscope::colour;
use microscope::source::code;

use crate::micro::{kepler_period, Anatomy, PRESETS};
use crate::model::{Action, Model};
use crate::view::{acc_strip, body_colour, log_scaled, signed, source, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

/// A number as a plain decimal with about four significant digits.
fn num(x: f64) -> String {
    if x == 0.0 || !x.is_finite() {
        return "0.0".into();
    }
    let digits = (3 - x.abs().log10().floor() as i32).clamp(1, 12) as usize;
    let s = format!("{x:.digits$}");
    if s.trim_start_matches('-').chars().all(|c| c == '0' || c == '.') {
        "0.0".into()
    } else {
        s
    }
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let n = m.bodies.len();
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Masses => ("masses", "mj", vec![n, n], "row i, column j: the mass m_j pulling on body i"),
        Stage::Cube => ("cube", "u:c_ube p", vec![2, n, n], "2 planes (x, y) of every pair's displacement p_i - p_j"),
        Stage::Pull => ("pull", "u:p_ull d", vec![n, n], "every pair's pull, g m_j / (r^2 + eps2)^1.5"),
        Stage::Reduce => ("reduce", "u:a_cc p", vec![2, n], "the x and y acceleration of every body"),
        Stage::Step => ("step", "u:s_tep s", vec![4, n], "x, y, velocity x, velocity y of every body"),
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
        d.dispatch(Action::PerFrame(v.parse().unwrap_or(4)));
    });
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
            <button onclick={act(m, || Action::Tick)}>{"Step"}</button>
            <button onclick={act(m, || Action::Restart)}>{"Restart"}</button>
            <select onchange={on_preset} aria-label="Preset">
                { for PRESETS.iter().enumerate().map(|(i, p)| html! { <option selected={i == m.preset}>{p.name}</option> }) }
            </select>
            <label class="slider">{"steps per frame"}
                <input type="range" min="1" max="40" value={m.per_frame.to_string()} onchange={on_speed} />
                <b>{m.per_frame}</b>
            </label>
            <span class="gen">{format!("step {}; X_eTaL ran {} steps in {:.0} ms", m.steps, m.per_frame, m.ms)}</span>
        </div>
    }
}

/// An arrow from (x, y) along (dx, dy), in world units.
fn arrow(x: f64, y: f64, dx: f64, dy: f64, class: &'static str) -> Html {
    let (x2, y2) = (x + dx, y + dy);
    let marker = if class == "sum" { "url(#head-sum)" } else { "url(#head-part)" };
    html! { <line class={classes!("arrow", class)} x1={x.to_string()} y1={y.to_string()} x2={x2.to_string()} y2={y2.to_string()}
        marker-end={marker} vector-effect="non-scaling-stroke" /> }
}

fn sky(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let m: &Model = model;
    let p = &PRESETS[m.preset];
    let (b, n, v) = (&a.next, a.n(), p.view);
    let mmax = b.m.iter().fold(0.0f64, |x, &y| x.max(y));
    let i = m.selected;
    let trails = m.trails.iter().enumerate().map(|(k, tr)| {
        let pts: Vec<String> = tr.iter().map(|(x, y)| format!("{x:.4},{y:.4}")).collect();
        html! { <polyline class="trail" points={pts.join(" ")} stroke={body_colour(k)} vector-effect="non-scaling-stroke" /> }
    });
    let bodies = (0..n).map(|k| {
        let r = v * (0.008 + 0.024 * (b.m[k] / mmax).cbrt()) / (n as f64 / 3.0).max(1.0).powf(0.25);
        let d = model.dispatcher();
        let onclick = Callback::from(move |_: MouseEvent| d.dispatch(Action::Select(k)));
        html! { <g class="body" {onclick}>
            <circle cx={b.x[k].to_string()} cy={b.y[k].to_string()} r={r.to_string()} fill={body_colour(k)}
                class={classes!((k == i).then_some("sel"))} vector-effect="non-scaling-stroke" />
            <circle class="hit" cx={b.x[k].to_string()} cy={b.y[k].to_string()} r={r.max(v * 0.045).to_string()} />
        </g> }
    });
    // The selected body's parts, scaled so the longest fits a third of the view.
    let parts: Vec<(f64, f64)> = (0..n).filter(|&j| j != i).map(|j| a.part(i, j)).collect();
    let sum = (a.acc[i], a.acc[n + i]);
    let len = |(x, y): (f64, f64)| (x * x + y * y).sqrt();
    let big = parts.iter().map(|&q| len(q)).fold(len(sum), f64::max).max(1e-300);
    let s = 0.35 * v / big;
    let arrows = parts.iter().map(|&(x, y)| arrow(b.x[i], b.y[i], s * x, s * y, "part"));
    let vb = format!("{} {} {} {}", -v, -v, 2.0 * v, 2.0 * v);
    let body = html! {
        <svg class="sky" viewBox={vb} role="img" aria-label="the bodies, their trails and the forces on the selected body">
            <defs>
                <marker id="head-part" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="5" markerHeight="5" orient="auto"><path d="M0,0 L10,5 L0,10 z" class="head part" /></marker>
                <marker id="head-sum" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="5" markerHeight="5" orient="auto"><path d="M0,0 L10,5 L0,10 z" class="head sum" /></marker>
            </defs>
            <g transform="scale(1,-1)">
                { for trails }
                { for arrows }
                { arrow(b.x[i], b.y[i], s * sum.0, s * sum.1, "sum") }
                { for bodies }
            </g>
        </svg>
    };
    let note = format!("{} Click a body to see the forces on it: violet, each other body's pull; orange, their sum.", p.about);
    panel("The bodies:", "p", &note, false, body)
}

fn stats(m: &Model, a: &Anatomy) -> Html {
    let ph = &PRESETS[m.preset].physics;
    let t = m.steps as f64 * ph.dt;
    let e = a.next.energy(ph);
    let (px, py) = a.next.momentum();
    let drift = if m.e0 != 0.0 { (e - m.e0) / m.e0.abs() } else { 0.0 };
    let period = match m.preset {
        3 => format!("; Kepler's period 2 pi sqrt(a^3 / G M) = {:.4}, so {:.3} orbits so far", kepler_period(), t / kepler_period()),
        _ => String::new(),
    };
    html! { <p class="note">
        { format!("time {t:.3}; energy {} (changed by {:.2e} of its start); total momentum ({}, {}){period}", num(e), drift, num(px), num(py)) }
    </p> }
}

fn inspector(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let m: &Model = model;
    let ph = &PRESETS[m.preset].physics;
    let (b, n, i) = (&a.next, a.n(), m.selected);
    let mut js: Vec<usize> = (0..n).filter(|&j| j != i).collect();
    let len = |(x, y): (f64, f64)| (x * x + y * y).sqrt();
    js.sort_by(|&p, &q| len(a.part(i, q)).total_cmp(&len(a.part(i, p))));
    let shown = &js[..js.len().min(6)];
    let rows = shown.iter().map(|&j| {
        let (ax, ay) = a.part(i, j);
        let d = model.dispatcher();
        let onclick = Callback::from(move |_: MouseEvent| d.dispatch(Action::Select(j)));
        html! { <tr class="pick" {onclick}>
            <td><span class="dot" style={format!("background:{}", body_colour(j))}></span>{j + 1}</td>
            <td>{code(&num(a.cube[i * n + j]))}</td><td>{code(&num(a.cube[n * n + i * n + j]))}</td>
            <td>{code(&num(a.pull[i * n + j]))}</td><td>{code(&num(ax))}</td><td>{code(&num(ay))}</td>
        </tr> }
    });
    let more = js.len() - shown.len();
    let top = shown.first().map(|&j| {
        let (dx, dy) = (a.cube[i * n + j], a.cube[n * n + i * n + j]);
        let w = format!(
            "{} * {} / ((({} * {}) + {} * {}) + {}) ^ 1.5",
            num(ph.g), num(b.m[j]), num(dx), num(dx), num(dy), num(dy), num(ph.eps2)
        );
        html! { <>
            <p class="calc">{format!("the pull of body {} on body {}: ", j + 1, i + 1)}{code("u:p_ull d")}{" = "}{code(&w)}{" = "}{code(&num(a.pull[i * n + j]))}</p>
            <p class="calc">{"its part of the acceleration: "}{code("n_eg d * w")}{" = "}{code(&format!("n_eg ({} {}) * {}", num(dx), num(dy), num(a.pull[i * n + j])))}{" = "}{code(&format!("{} {}", num(a.part(i, j).0), num(a.part(i, j).1)))}</p>
        </> }
    });
    html! {
        <section class="panel inspector">
            <h2>{format!("Body {}: mass {}, at ({}, {})", i + 1, num(b.m[i]), num(b.x[i]), num(b.y[i]))}</h2>
            <p class="note">{format!("Row {} of the cube and the pull: the other bodies, strongest first. Click a row to pick that body.", i + 1)}</p>
            <table class="zs forces">
                <tr><th>{"body"}</th><th>{"x_i - x_j"}</th><th>{"y_i - y_j"}</th><th>{"pull"}</th><th>{"part of a_x"}</th><th>{"part of a_y"}</th></tr>
                { for rows }
                { for (more > 0).then(|| html! { <tr><td colspan="6" class="more">{format!("and {more} weaker pulls")}</td></tr> }) }
                <tr class="total"><td colspan="4">{"the sum over j: "}{code("u:a_cc p")}</td><td>{code(&num(a.acc[i]))}</td><td>{code(&num(a.acc[n + i]))}</td></tr>
            </table>
            { for top }
            { stats(m, a) }
        </section>
    }
}

/// A picture of an array; a click picks the body of the row (or, for
/// the 2 x N accelerations, of the column).
fn pic(model: &UseReducerHandle<Model>, rows: usize, cols: usize, rgba: Vec<u8>, caption: &str, by_row: bool) -> Html {
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x): (usize, usize)| d.dispatch(Action::Select(if by_row { y } else { x }))));
    html! { <figure>
        <Canvas {rows} {cols} rgba={Rc::new(rgba)} {onclick} class="mid" />
        <figcaption>{code(caption)}</figcaption>
    </figure> }
}

fn arrays(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let m: &Model = model;
    let (n, i) = (a.n(), Some(m.selected));
    let mj: Vec<f64> = (0..n * n).map(|k| a.next.m[k % n]).collect();
    let nn = n * n;
    let px: Vec<f64> = (0..nn).map(|k| a.part(k / n, k % n).0).collect();
    let py: Vec<f64> = (0..nn).map(|k| a.part(k / n, k % n).1).collect();
    html! { <>
        { panel("1. The masses, over every pair:", "mj := (o_ffsets n) 'r_ight t_able m", "Row i, column j holds m_j, the mass pulling on body i: every row is the masses.",
            m.focus == Stage::Masses, html! { <div class="pair">{pic(model, n, n, colour::scaled(&mj), "mj", true)}</div> }) }
        { panel("2. The cube:", "u:c_ube p", "For each axis, a table of differences holds x_i - x_j: every pair's displacement at once (blue below 0, red above). The two tables are the planes of a 2 x N x N cube; each is minus its own mirror image.",
            m.focus == Stage::Cube, html! { <div class="pair">{pic(model, n, n, signed(&a.cube[..nn], n, i), "1 s_elect d", true)}{pic(model, n, n, signed(&a.cube[nn..], n, i), "2 s_elect d", true)}</div> }) }
        { panel("3. The pull:", "u:p_ull d", "g m_j / (r^2 + eps2)^1.5 for every pair, on a log scale: bright pairs are close. eps2 softens close passes; the grey diagonal (a body and itself) is large but multiplies a displacement of 0.",
            m.focus == Stage::Pull, html! { <div class="pair">{pic(model, n, n, log_scaled(&a.pull, n, i), "u:p_ull d", true)}</div> }) }
        { panel("4. The reduction:", "n_eg '+ r_/_3 d * (2 c_at s_hape w) r_eshape w", "Each plane of the cube times the pull is every pair's part of the acceleration; summing along j (axis 3) leaves one x and one y per body.",
            m.focus == Stage::Reduce, html! { <div class="pair">
                {pic(model, n, n, signed(&px, n, i), "n_eg 1 s_elect d * W", true)}{pic(model, n, n, signed(&py, n, i), "n_eg 2 s_elect d * W", true)}
                {pic(model, 2, n, acc_strip(&a.acc, n), "u:a_cc p", false)}
            </div> }) }
        { panel("5. The step:", "u:s_tep s", "Kick, drift, kick (leapfrog): half a step of acceleration on the velocities, a whole step of the velocities on the positions, the acceleration at the new positions for the other half. It keeps the energy close for long runs.",
            m.focus == Stage::Step, html! {}) }
    </> }
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
    let n = model.bodies.len();
    let body = match &model.last {
        Some(a) => html! {
            <div class="layout even">
                <div class="col">{ sky(&model, a) }{ inspector(&model, a) }</div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"The preset's constants, then the core of nbody.xtl, run by X_eTaL in your browser; the stage you pick is highlighted."}</p>
                        { source(&PRESETS[model.preset].physics, n, model.focus) }
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
            { header("N-body gravity", "Every pair of bodies at once, with no loops: the displacements of all pairs form a cube, the cube gives every pair's pull, and one reduce sums the pulls into each body's acceleration. Pick a preset; click a body to see its row of forces.") }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&model.notice) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
