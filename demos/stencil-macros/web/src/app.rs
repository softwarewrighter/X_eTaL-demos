//! The page: the kernel applied to the picture; the kernel to edit; a
//! cell's terms; the macro call and what X_eTaL expanded it to; the
//! macro; the program.

use std::rc::Rc;

use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{about, chip, footer, header, notice, panel};
use microscope::source::code;

use crate::micro::{num, terms, Run, COLS, PRESETS, ROWS};
use crate::model::{Action, Model};
use crate::view::{bright, library, result, source, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let kernel = m.ran.text();
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Picture => ("picture", "img", vec![ROWS, COLS], "one brightness per cell"),
        Stage::Macro => ("macro", "\"s:\" u_se< \"Stencil\"", vec![m.expansion.len()], "m:s_tencil<, our own: text in, the program text it writes out (Char)"),
        Stage::Call => ("call", "s:s_tencil<", vec![kernel.len()], "the kernel, as text (Char); expanded before the program is type-checked"),
        Stage::Result => ("result", "out", vec![ROWS, COLS], "the expanded step, applied steps times"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn number(label: String, value: f64, step: &str, on: Callback<f64>) -> Html {
    let onchange = Callback::from(move |e: Event| {
        if let Ok(v) = e.target_unchecked_into::<HtmlInputElement>().value().trim().parse::<f64>() {
            on.emit(v);
        }
    });
    html! { <input type="number" step={step.to_string()} value={num(value)} {onchange} aria-label={label} /> }
}

fn kernel(model: &UseReducerHandle<Model>) -> Html {
    let m: &Model = model;
    let k = &m.kernel;
    let d = model.dispatcher();
    let n = PRESETS.len();
    let on_preset = Callback::from(move |e: Event| {
        let i = e.target_unchecked_into::<HtmlSelectElement>().selected_index();
        if i >= 0 && (i as usize) < n {
            d.dispatch(Action::Preset(i as usize));
        }
    });
    let cell = |i: usize| {
        let d = model.dispatcher();
        number(format!("kernel number {}", i + 1), k.w[i], "any", Callback::from(move |v| d.dispatch(Action::Cell(i, v))))
    };
    let d = model.dispatcher();
    let on_divide = Callback::from(move |v| d.dispatch(Action::Divide(v)));
    let d = model.dispatcher();
    let on_steps = Callback::from(move |e: InputEvent| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::Steps(v.parse().unwrap_or(1)));
    });
    let side = |s: usize| {
        html! { <button class={classes!((k.side == s).then_some("active"))} onclick={act(model, move || Action::Side(s))}>{format!("{s} x {s}")}</button> }
    };
    html! {
        <section class={classes!("panel", (m.focus == Stage::Call).then_some("focus"))}>
            <h2>{"The kernel"}</h2>
            <p class="note">{"Pick a preset or edit any number; the page writes the kernel into the macro call, and X_eTaL expands it and runs the program again."}</p>
            <div class="controls">
                <select onchange={on_preset} aria-label="Kernel preset">
                    { for PRESETS.iter().enumerate().map(|(i, p)| html! { <option selected={Some(i) == m.preset}>{p.name}</option> }) }
                    { for m.preset.is_none().then(|| html! { <option selected=true disabled=true>{"edited"}</option> }) }
                </select>
                { side(3) }{ side(5) }
            </div>
            <div class={classes!("kgrid", (k.side == 5).then_some("k5"))}>{ for (0..k.side * k.side).map(cell) }</div>
            <div class="controls">
                <label class="slider">{"divide by "}{ number("divide by".into(), k.divide, "any", on_divide) }</label>
                <label class="slider">{"steps"}
                    <input type="range" min="1" max="60" value={k.steps.to_string()} oninput={on_steps} />
                    <b>{k.steps}</b>
                </label>
            </div>
        </section>
    }
}

fn picture(model: &UseReducerHandle<Model>, r: &Run) -> Html {
    let m: &Model = model;
    let k = &m.ran;
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Click(y, x))));
    let d = model.dispatcher();
    let onclick2 = Some(Callback::from(move |(y, x)| d.dispatch(Action::Click(y, x))));
    let what = if k.balanced() { "The numbers sum to 0, so the result is signed: blue below 0, red above." } else { "Dark to bright." };
    let note = format!(
        "The kernel applied {} time{} to the picture (the edges wrap). {what} Click a cell to see its terms.",
        k.steps,
        if k.steps == 1 { "" } else { "s" }
    );
    html! { <>
        { panel("The result:", "out", &note, m.focus == Stage::Result, html! {
            <Canvas rows={ROWS} cols={COLS} rgba={Rc::new(result(&r.out, k.balanced()))} mark={Some(m.selected)} {onclick} class="big" />
        }) }
        { panel("The picture:", "img", "A disk, a box and a little noise, made by the program's own masks.", m.focus == Stage::Picture, html! {
            <Canvas rows={ROWS} cols={COLS} rgba={Rc::new(bright(&r.img))} mark={Some(m.selected)} onclick={onclick2} class="big" />
        }) }
    </> }
}

fn inspector(m: &Model, r: &Run) -> Html {
    let (y, x) = m.selected;
    let k = &m.ran;
    let t = terms(k, &r.img, y, x);
    let sum: f64 = t.iter().map(|(w, _, _, v)| w * v).sum();
    let line = |(w, dr, dc, v): &(f64, i64, i64, f64)| {
        let at = match (dr, dc) {
            (0, 0) => "the cell itself".to_string(),
            _ => format!("row {:+}, column {:+}", dr, dc),
        };
        html! { <p class="calc">{code(&num(*w))}{" x "}{code(&format!("{v:.4}"))}{format!(" ({at})")}</p> }
    };
    let first = sum / k.divide;
    let shown = r.out[y * COLS + x];
    html! {
        <section class="panel inspector">
            <h2>{format!("Cell row {}, column {}", y + 1, x + 1)}</h2>
            <p class="note">{format!("Its {} terms, one per number of the kernel that is not 0 (the picture's values are X_eTaL's):", t.len())}</p>
            { for t.iter().map(line) }
            <p class="calc">{"sum "}{code(&format!("{sum:.4}"))}{ for (k.divide != 1.0).then(|| html! { <>{" / "}{code(&num(k.divide))}{" = "}{code(&format!("{first:.4}"))}</> }) }</p>
            <p class="note">{ if k.steps == 1 {
                format!("X_eTaL's result here: {shown:.4}.")
            } else {
                format!("That is the first step; after {} steps X_eTaL's result here is {shown:.4}.", k.steps)
            } }</p>
        </section>
    }
}

fn expansion(m: &Model) -> Html {
    let k = &m.ran;
    let (written, ones, negs, zeros) = k.counts();
    let mults = written - ones - negs;
    let n = k.side * k.side;
    html! { <>
        <section class={classes!("panel", "code", (m.focus == Stage::Call).then_some("focus"))}>
            <h2>{"The macro call"}</h2>
            <p class="note">{"The kernel as text, the call the page writes into the program:"}</p>
            <pre class="source">{ code(m.program.lines().find(|l| l.starts_with("u:s_tep :=")).unwrap_or("")) }</pre>
            <h2>{"What X_eTaL expanded it to"}</h2>
            <p class="note">{format!(
                "{written} terms written for {n} numbers: {zeros} zero{} skipped, {ones} one{} without a multiply, {negs} negation{}, {mults} multipl{}. Expanded in your browser by the library below, as xetal expand prints it.",
                if zeros == 1 { "" } else { "s" }, if ones == 1 { "" } else { "s" }, if negs == 1 { "" } else { "s" }, if mults == 1 { "y" } else { "ies" }
            )}</p>
            <pre class="source">{ code(&m.expansion) }</pre>
        </section>
        <section class={classes!("panel", "code", (m.focus == Stage::Macro).then_some("focus"))}>
            <h2>{"The macro: Stencil.xtlm"}</h2>
            <p class="note">{"A macro library of our own, beside the program: m:s_tencil< is an ordinary X_eTaL function from the text on each side of its call to new program text. It runs before the program is type-checked."}</p>
            { library() }
        </section>
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    let body = match &model.last {
        Some(r) => html! {
            <div class="layout even">
                <div class="col">{ picture(&model, r) }{ kernel(&model) }{ inspector(&model, r) }</div>
                <div class="col">
                    { expansion(&model) }
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran in your browser: the import, the picture from stencil-macros.xtl, the step with your kernel's macro call, and the lines that apply it and print. The stage you pick is highlighted."}</p>
                        { source(&model.program, model.focus) }
                        <p class="note">{format!("Expanded and run in {:.0} ms.", model.ms)}</p>
                    </section>
                </div>
            </div>
        },
        None => html! {},
    };
    html! {
        <>
        <header>
            { header("Stencils by macro", "An image kernel written as a picture of numbers, and a macro library of our own that turns it into code when the program is expanded: one rotation of the picture per number, times the number. A 0 writes nothing and a 1 no multiply. Edit the kernel and watch what the program becomes.", about(include_str!("../../demo.toml"))) }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { notice(&model.notice) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
