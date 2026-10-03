//! The page: the stage you pick, large; the kernels to edit; a pixel's
//! window times each kernel; every stage small; the program.

use std::rc::Rc;

use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{chip, footer, header, notice, panel};
use microscope::colour;
use microscope::source::code;

use crate::micro::{turn, Anatomy, Kernel, BLURS, EDGES, HALF, SCENES, SIZE};
use crate::model::{Action, Model};
use crate::view::{direction, edges, grey, source, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

/// A number as a plain decimal with about four significant digits.
fn num(x: f64) -> String {
    if x == 0.0 || !x.is_finite() {
        return "0.0".into();
    }
    let digits = (3 - x.abs().log10().floor() as i32).clamp(1, 8) as usize;
    let s = format!("{x:.digits$}");
    if s.trim_start_matches('-').chars().all(|c| c == '0' || c == '.') {
        "0.0".into()
    } else {
        s
    }
}

/// A kernel item: the number with trailing zeros dropped (0.0625, 2.0).
fn short(x: f64) -> String {
    let s = num(x);
    match s.contains('.') {
        true => {
            let t = s.trim_end_matches('0');
            if t.ends_with('.') { format!("{t}0") } else { t.to_string() }
        }
        false => s,
    }
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Picture => ("picture", "img", vec![SIZE, SIZE], "one brightness per pixel"),
        Stage::Windows => ("windows", "u:w_indows img", vec![3, 3, SIZE, SIZE], "the 3 x 3 shifted copies: row offset, column offset, then the picture"),
        Stage::Smooth => ("smooth", "blur u:f_ilter img", vec![SIZE, SIZE], "the blurred picture"),
        Stage::Gradients => ("gradients", "gx ; gy", vec![SIZE, SIZE], "the change across (gx) and down (gy) at each pixel"),
        Stage::Magnitude => ("magnitude", "mag", vec![SIZE, SIZE], "the edge strength at each pixel"),
        Stage::Edges => ("edges", "edges", vec![SIZE, SIZE], "1.0 where the strength passes the threshold"),
        Stage::Pool => ("pool", "pool", vec![HALF, HALF], "the strongest edge in each 2 x 2 block"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

/// A preset picker; once edited it shows "edited" (not a preset).
fn select(label: &str, names: Vec<&'static str>, chosen: Option<usize>, on: Callback<usize>) -> Html {
    let n = names.len();
    let onchange = Callback::from(move |e: Event| {
        let i = e.target_unchecked_into::<HtmlSelectElement>().selected_index();
        if i >= 0 && (i as usize) < n {
            on.emit(i as usize);
        }
    });
    html! {
        <select {onchange} aria-label={label.to_string()}>
            { for names.iter().enumerate().map(|(i, n)| html! { <option selected={Some(i) == chosen}>{*n}</option> }) }
            { for chosen.is_none().then(|| html! { <option selected=true disabled=true>{"edited"}</option> }) }
        </select>
    }
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let on_scene = Callback::from(move |i| d.dispatch(Action::Scene(i)));
    let d = m.dispatcher();
    let on_thresh = Callback::from(move |e: Event| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::Thresh(v.parse().unwrap_or(0.8)));
    });
    html! {
        <div class="controls">
            { select("Picture", SCENES.iter().map(|s| s.0).collect(), Some(m.scene), on_scene) }
            <label class="slider">{"threshold"}
                <input type="range" min="0.05" max="3" step="0.05" value={m.setup.thresh.to_string()} onchange={on_thresh} />
                <b>{format!("{:.2}", m.setup.thresh)}</b>
            </label>
            <span class="gen">{format!("X_eTaL ran the pipeline on {SIZE} x {SIZE} pixels in {:.0} ms", m.ms)}</span>
        </div>
    }
}

/// A 3 x 3 kernel as number fields; `on` gets (item, value).
fn editor(k: &Kernel, on: Option<Callback<(usize, f64)>>) -> Html {
    let cell = |i: usize| {
        let on = on.clone();
        let onchange = on.map(|on| {
            Callback::from(move |e: Event| {
                if let Ok(v) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<f64>() {
                    on.emit((i, v));
                }
            })
        });
        let disabled = onchange.is_none();
        html! { <input type="number" step="0.0625" value={short(k[i])} {onchange} {disabled} aria-label={format!("kernel item {}", i + 1)} /> }
    };
    html! { <div class="kgrid">{ for (0..9).map(cell) }</div> }
}

fn kernels(model: &UseReducerHandle<Model>) -> Html {
    let m: &Model = model;
    let d = model.dispatcher();
    let on_blur = Callback::from(move |i| d.dispatch(Action::Blur(i)));
    let d = model.dispatcher();
    let on_edge = Callback::from(move |i| d.dispatch(Action::Edge(i)));
    let d = model.dispatcher();
    let blur_cell = Callback::from(move |(k, v)| d.dispatch(Action::BlurCell(k, v)));
    let d = model.dispatcher();
    let edge_cell = Callback::from(move |(k, v)| d.dispatch(Action::EdgeCell(k, v)));
    html! {
        <section class="panel">
            <h2>{"The kernels"}</h2>
            <p class="note">{"Edit any number, or pick a preset; X_eTaL reruns the whole pipeline. ky is kx turned a quarter: ky := o_\\ kx, the transpose."}</p>
            <div class="kernels">
                <div><div class="kname">{code("blur")}{" "}{ select("Blur kernel", BLURS.iter().map(|b| b.0).collect(), m.blur, on_blur) }</div>{ editor(&m.setup.blur, Some(blur_cell)) }</div>
                <div><div class="kname">{code("kx")}{" "}{ select("Edge kernel", EDGES.iter().map(|b| b.0).collect(), m.edge, on_edge) }</div>{ editor(&m.setup.kx, Some(edge_cell)) }</div>
                <div><div class="kname">{code("ky")}</div>{ editor(&turn(&m.setup.kx), None) }</div>
            </div>
        </section>
    }
}

fn grid3(k: &Kernel) -> Html {
    html! { <table class="nbhd num">{ for (0..3).map(|r| html! { <tr>{ for (0..3).map(|c| html! { <td>{num(k[3 * r + c])}</td> }) }</tr> }) }</table> }
}

/// One filter at the pixel: its window times the kernel, summed.
fn product(name: &str, win: &Kernel, wname: &str, k: &Kernel, kname: &str, value: f64) -> Html {
    let sum: f64 = (0..9).map(|i| win[i] * k[i]).sum();
    html! { <div class="product">
        <p class="calc">{code(name)}{" = "}{code(&format!("'+ r_/_12 ({wname}) * {kname}"))}</p>
        <div class="crosses">{grid3(win)}<span class="op">{"\u{00d7}"}</span>{grid3(k)}<span class="op">{"="}</span><span class="val">{code(&num(value))}</span></div>
        { for ((sum - value).abs() > 1e-9).then(|| html! { <p class="error">{"the page's sum differs from X_eTaL's"}</p> }) }
    </div> }
}

fn inspector(m: &Model, a: &Anatomy) -> Html {
    let (y, x) = m.setup.pixel;
    let i = y * SIZE + x;
    let s = &m.setup;
    let mag = format!("(({} * {}) + {} * {}) ^ 0.5", num(a.gx[i]), num(a.gx[i]), num(a.gy[i]), num(a.gy[i]));
    let edge = format!("f_loat {} > {}", num(a.mag[i]), num(s.thresh));
    html! {
        <section class="panel inspector">
            <h2>{format!("Pixel row {}, column {}", y + 1, x + 1)}</h2>
            <p class="note">{"Its window (the nine numbers of the windows stack at this pixel, from X_eTaL) times each kernel, summed. Click any picture to pick another pixel."}</p>
            { product("smooth", &a.win_img, "u:w_indows img", &s.blur, "blur", a.smooth[i]) }
            { product("gx", &a.win_smooth, "u:w_indows smooth", &s.kx, "kx", a.gx[i]) }
            { product("gy", &a.win_smooth, "u:w_indows smooth", &turn(&s.kx), "ky", a.gy[i]) }
            <p class="calc">{code("mag")}{" = "}{code(&mag)}{" = "}{code(&num(a.mag[i]))}</p>
            <p class="calc">{code("edges")}{" = "}{code(&edge)}{" = "}{code(&num(a.edges[i]))}</p>
        </section>
    }
}

/// The picture of a stage, its size and its pixels.
fn picture(stage: Stage, a: &Anatomy) -> (usize, Vec<u8>) {
    match stage {
        Stage::Picture | Stage::Windows => (SIZE, grey(&a.img)),
        Stage::Smooth => (SIZE, grey(&a.smooth)),
        Stage::Gradients => (SIZE, direction(&a.gx, &a.gy, &a.mag)),
        Stage::Magnitude => (SIZE, colour::scaled(&a.mag)),
        Stage::Edges => (SIZE, edges(&a.edges)),
        Stage::Pool => (HALF, colour::scaled(&a.pool)),
    }
}

fn pic(model: &UseReducerHandle<Model>, size: usize, rgba: Vec<u8>, class: &'static str, caption: Html) -> Html {
    let d = model.dispatcher();
    let k = SIZE / size;
    let onclick = Some(Callback::from(move |(y, x): (usize, usize)| d.dispatch(Action::Pixel(y * k, x * k))));
    let (y, x) = model.setup.pixel;
    let mark = Some((y / k, x / k));
    html! { <figure>
        <Canvas rows={size} cols={size} rgba={Rc::new(rgba)} {mark} {onclick} {class} />
        <figcaption>{caption}</figcaption>
    </figure> }
}

fn big(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let (size, rgba) = picture(model.focus, a);
    let note = match model.focus {
        Stage::Gradients => "The gradients' direction as a colour (the angle of gx, gy), as bright as the edge is strong.",
        Stage::Pool => "Half the size: each pixel is the strongest edge of a 2 x 2 block.",
        _ => "The stage you picked, as X_eTaL computed it. Click a pixel to see its window times the kernels.",
    };
    let body = html! { <Canvas rows={size} cols={size} rgba={Rc::new(rgba)} mark={Some((model.setup.pixel.0 * size / SIZE, model.setup.pixel.1 * size / SIZE))}
        onclick={Some({ let d = model.dispatcher(); let k = SIZE / size; Callback::from(move |(y, x): (usize, usize)| d.dispatch(Action::Pixel(y * k, x * k))) })} class="big" /> };
    panel("The stage:", stage_code(model.focus), note, false, body)
}

fn stage_code(s: Stage) -> &'static str {
    match s {
        Stage::Picture => "img",
        Stage::Windows => "u:w_indows img",
        Stage::Smooth => "smooth",
        Stage::Gradients => "gx ; gy",
        Stage::Magnitude => "mag",
        Stage::Edges => "edges",
        Stage::Pool => "pool",
    }
}

/// The picture shifted as the windows plane (a, b) holds it: the
/// pixel at row r + a, column c + b.
fn shifted(img: &[f64], a: i64, b: i64) -> Vec<f64> {
    let n = SIZE as i64;
    (0..SIZE * SIZE)
        .map(|k| {
            let (r, c) = (k as i64 / n, k as i64 % n);
            img[(((r + a).rem_euclid(n)) * n + (c + b).rem_euclid(n)) as usize]
        })
        .collect()
}

fn arrays(model: &UseReducerHandle<Model>, a: &Anatomy) -> Html {
    let f = model.focus;
    let small = |rgba: Vec<u8>, cap: &str| pic(model, SIZE, rgba, "mid", code(cap));
    let windows = (0..9).map(|k| {
        let (da, db) = (k as i64 / 3 - 1, k as i64 % 3 - 1);
        pic(model, SIZE, grey(&shifted(&a.img, da, db)), "tiny", code(&format!("{} s_elect {} s_elect w", db + 2, da + 2)))
    });
    let pool = format!("'m_ax r_/_2 'm_ax r_/_4 ({HALF} c_at 2 c_at {HALF} c_at 2) r_eshape mag");
    html! { <>
        { panel("1. The picture:", "img", "Built from masks of the pixels' coordinates (a disk, a square, a triangle...) and a little noise from r_oll!.",
            f == Stage::Picture, html! { <div class="pair">{small(grey(&a.img), "img")}</div> }) }
        { panel("2. The windows:", "u:w_indows img", "Rotating by a list of amounts gives one shifted copy per amount on a new first axis; doing it twice gives the 3 x 3 stack of copies. Plane (a, b) holds each pixel's neighbour at row offset a, column offset b.",
            f == Stage::Windows, html! { <div class="nine">{ for windows }</div> }) }
        { panel("3. Smooth:", "blur u:f_ilter img", "The kernel spread over the picture, times the windows, summed over the kernel's two axes: every pixel's weighted average of its neighbours.",
            f == Stage::Smooth, html! { <div class="pair">{small(grey(&a.smooth), "smooth")}</div> }) }
        { panel("4. The gradients:", "gx := kx u:f_ilter smooth", "The same filter with the edge kernels: gx is the change across (blue darker to the right, red brighter), gy the change down. The colour picture shows their direction.",
            f == Stage::Gradients, html! { <div class="pair">{small(colour::signed(&a.gx), "gx")}{small(colour::signed(&a.gy), "gy")}{small(direction(&a.gx, &a.gy, &a.mag), "gx ; gy")}</div> }) }
        { panel("5. The magnitude:", "mag := ((gx * gx) + gy * gy) ^ 0.5", "How strong the edge is at each pixel, whichever way it runs.",
            f == Stage::Magnitude, html! { <div class="pair">{small(colour::scaled(&a.mag), "mag")}</div> }) }
        { panel("6. The edges:", "edges := f_loat mag > thresh", "A mask: 1.0 where the strength passes the threshold.",
            f == Stage::Edges, html! { <div class="pair">{small(edges(&a.edges), "edges")}</div> }) }
        { panel("7. Pooling:", &pool, "Reshaped so each 2 x 2 block has its own two axes (rows, row in block, columns, column in block), then the largest over those two axes: half the size, the strongest edge kept.",
            f == Stage::Pool, html! { <div class="pair">{pic(model, HALF, colour::scaled(&a.pool), "mid", code("pool"))}</div> }) }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    let body = match &model.last {
        Some(a) => html! {
            <div class="layout even">
                <div class="col">{ big(&model, a) }{ kernels(&model) }{ inspector(&model, a) }</div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"The picture, the page's settings, then the core of image-pipeline.xtl, run by X_eTaL in your browser; the stage you pick is highlighted."}</p>
                        { source(&model.setup, model.focus) }
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
            { header("Image pipeline", "Blur, edges, a threshold and pooling, each an array program over the whole picture. Every 3 x 3 filter is the same two steps: the stack of the picture's nine shifted copies, then the kernel times the stack, summed. Edit the kernels; click a pixel to see its window times each one.") }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&model.notice) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
