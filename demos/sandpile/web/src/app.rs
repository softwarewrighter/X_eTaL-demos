//! The page: the pile settling, round after round; the topple counts;
//! who topples next; a cell's arithmetic; the program.

use std::rc::Rc;

use gloo_timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use microscope::canvas::Canvas;
use microscope::chrome::{about, chip, footer, header, notice, panel};
use microscope::source::code;

use crate::micro::{Pile, Rounds, SIDE};
use crate::model::{Action, Model};
use crate::view::{counts, pile, source, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Edge => ("edge", "inside", vec![SIDE, SIDE], "1 inside, 0 on the ring where grains fall off"),
        Stage::Topple => ("topple", "h d_iv 4", vec![SIDE, SIDE], "how many times each cell topples this round"),
        Stage::Give => ("give", "g", vec![SIDE, SIDE], "what each cell gets from its four neighbors"),
        Stage::Keep => ("keep", "u:t_opple h", vec![SIDE, SIDE], "the grains after the round"),
        Stage::Count => ("count", "u:s_tep s", vec![2, SIDE, SIDE], "the grains, and each cell's topples so far"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let d = m.dispatcher();
    let on_speed = Callback::from(move |e: InputEvent| {
        let v = e.target_unchecked_into::<HtmlInputElement>().value();
        d.dispatch(Action::PerFrame(v.parse().unwrap_or(20)));
    });
    let d = m.dispatcher();
    let on_drops = Callback::from(move |e: Event| {
        d.dispatch(Action::ClickDrops(e.target_unchecked_into::<HtmlInputElement>().checked()));
    });
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
            <button onclick={act(m, || Action::DropCenter(1000))}>{"+1,000 at the center"}</button>
            <button onclick={act(m, || Action::DropCenter(10000))}>{"+10,000 at the center"}</button>
            <button onclick={act(m, || Action::Everywhere(1))}>{"+1 everywhere"}</button>
            <button onclick={act(m, || Action::Clear)}>{"Clear"}</button>
            <label class="toggle"><input type="checkbox" checked={m.click_drops} onchange={on_drops} />{"click drops 200 grains"}</label>
            <label class="slider">{"rounds per frame"}
                <input type="range" min="1" max="200" value={m.per_frame.to_string()} oninput={on_speed} />
                <b>{m.per_frame}</b>
            </label>
            <span class="gen">{format!("round {}; X_eTaL ran {} rounds in {:.0} ms", m.rounds, m.per_frame, m.ms)}</span>
        </div>
    }
}

fn picture(model: &UseReducerHandle<Model>, r: &Rounds) -> Html {
    let m: &Model = model;
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Click(y, x))));
    let total: i64 = r.next.topples.iter().sum();
    let status = if r.stable { "stable: every cell has fewer than 4 grains" } else { "toppling" };
    let note = format!(
        "{} grains on the grid, {} fell off the edge; {} topples so far; {status}. Dark 0, blue 1, gold 2, red 3, white 4 or more.",
        r.grains,
        m.dropped - r.grains,
        total
    );
    let body = html! { <Canvas rows={SIDE} cols={SIDE} rgba={Rc::new(pile(&r.next.h))} mark={Some(m.selected)} {onclick} class="big" /> };
    panel("The pile:", "h", &note, m.focus == Stage::Keep, body)
}

fn inspector(m: &Model, r: &Rounds) -> Html {
    let (y, x) = m.selected;
    let i = y * SIDE + x;
    let h = &r.next.h;
    if Pile::on_edge(y, x) {
        return html! { <section class="panel inspector"><h2>{format!("Cell row {}, column {}: the edge", y + 1, x + 1)}</h2>
            <p class="note">{"Grains given to the edge fall off: inside is 0 here, so it always holds 0."}</p></section> };
    }
    let q = |j: usize| r.q[j];
    let (n, s, w, e) = (q(i - SIDE), q(i + SIDE), q(i - 1), q(i + 1));
    let g = n + s + w + e;
    let next = h[i] - 4 * q(i) + g;
    html! {
        <section class="panel inspector">
            <h2>{format!("Cell row {}, column {}, in the coming round", y + 1, x + 1)}</h2>
            <p class="calc">{"grains "}{code("h")}{" = "}{code(&h[i].to_string())}{"; it topples "}{code("h d_iv 4")}{" = "}{code(&q(i).to_string())}{" times"}</p>
            <p class="calc">{"its neighbors give "}{code("g")}{" = "}{code(&format!("{n} + {s} + {w} + {e}"))}{" = "}{code(&g.to_string())}</p>
            <p class="calc">{"it keeps "}{code("inside * (h - 4 * q) + g")}{" = "}{code(&format!("1 * ({} - 4 * {}) + {g}", h[i], q(i)))}{" = "}{code(&next.to_string())}</p>
            <p class="note">{format!("It has toppled {} times so far. Click a cell to inspect it (tick \"click drops\" to drop grains there instead).", r.next.topples[i])}</p>
        </section>
    }
}

fn small(model: &UseReducerHandle<Model>, rgba: Vec<u8>, caption: &str) -> Html {
    let d = model.dispatcher();
    let onclick = Some(Callback::from(move |(y, x)| d.dispatch(Action::Click(y, x))));
    html! { <figure>
        <Canvas rows={SIDE} cols={SIDE} rgba={Rc::new(rgba)} mark={Some(model.selected)} {onclick} class="mid" />
        <figcaption>{code(caption)}</figcaption>
    </figure> }
}

fn arrays(model: &UseReducerHandle<Model>, r: &Rounds) -> Html {
    let f = model.focus;
    html! { <>
        { panel("1. Topple:", "q := h d_iv 4", "Every cell with 4 or more grains topples, as many times as it can, all at once: the order does not matter (the pile is abelian).",
            f == Stage::Topple, html! { <div class="pair">{small(model, counts(&r.q), "h d_iv 4")}</div> }) }
        { panel("2. Give and keep:", "inside * (h - 4 * q) + g", "Each topple gives one grain to each neighbor (four rotations of q); a cell keeps what it did not give, plus what it got. On the edge, inside is 0: the grains fall off.",
            f == Stage::Give, html! { <p class="note">{"See the cell's arithmetic in the inspector."}</p> }) }
        { panel("3. The avalanche:", "u:s_tep s", "The second plane counts each cell's topples so far (log scale): the shape of the avalanches.",
            f == Stage::Count, html! { <div class="pair">{small(model, counts(&r.next.topples), "2 s_elect s")}</div> }) }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let timer = playing.then(|| Interval::new(40, move || d.dispatch(Action::Tick)));
            move || drop(timer)
        });
    }
    let body = match &model.last {
        Some(r) => html! {
            <div class="layout even">
                <div class="col">{ picture(&model, r) }{ inspector(&model, r) }</div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran in your browser for the last frame: the grid's side, the core of sandpile.xtl, the pile and its topple counts written in by the page (folded: click to show them), and the lines that run the rounds and print. The stage you pick is highlighted."}</p>
                        { source(&model.program, model.focus) }
                    </section>
                    { arrays(&model, r) }
                </div>
            </div>
        },
        None => html! {},
    };
    html! {
        <>
        <header>
            { header("Abelian sandpile", "Drop grains on a grid; a cell with 4 or more topples, giving one to each neighbor, and grains that reach the edge fall off. Every cell topples at once, round after round, until the pile is stable: a fractal grows from a single heap.", about(include_str!("../../demo.toml"))) }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&model.notice) }
        </header>
        <main>{body}</main>
        { footer() }
        </>
    }
}
