//! The page: the cube as a net of 54 stickers, the turn buttons, the
//! moves made, and the program X_eTaL ran.

use gloo_timers::callback::{Interval, Timeout};
use yew::prelude::*;

use microscope::chrome::{about, footer, header, notice};
use microscope::source::{between, listing};

use crate::micro::{name, LIBRARY};
use crate::model::{Action, Model};

/// Where each face sits in the net, in sticker cells: U D F B R L.
const AT: [(usize, usize); 6] = [(3, 0), (3, 6), (3, 3), (9, 3), (6, 3), (0, 3)];
/// Each face's color: the color of its center in the solved cube.
const COLOR: [&str; 6] = ["white", "yellow", "green", "blue", "red", "orange"];

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn net(stickers: &[u8]) -> Html {
    let cells = stickers.iter().enumerate().map(|(i, &c)| {
        let (f, k) = (i / 9, i % 9);
        let (x, y) = (AT[f].0 + k % 3, AT[f].1 + k / 3);
        let class = format!("sticker {}", COLOR[(c - 1) as usize]);
        html! { <rect class={class} x={(x * 10 + 1).to_string()} y={(y * 10 + 1).to_string()} width="8.6" height="8.6" rx="1.4" /> }
    });
    html! {
        <svg class="net" viewBox="0 0 121 91" role="img" aria-label="the cube unfolded: up above, then left, front, right and back, then down">
            { for cells }
        </svg>
    }
}

fn turns(m: &UseReducerHandle<Model>) -> Html {
    let button = |t: u8| html! { <button class="turn" onclick={act(m, move || Action::Turn(t))}>{name(t)}</button> };
    html! {
        <div class="controls">
            { for (1..=12).map(button) }
            <button onclick={act(m, || Action::Undo)} disabled={m.moves.is_empty()}>{"Undo"}</button>
            <button onclick={act(m, || Action::Scramble)}>{"Scramble"}</button>
            <button onclick={act(m, || Action::Reset)}>{"Reset"}</button>
            <button class="solve" onclick={act(m, || Action::StartSolve)} disabled={m.solving}>{ if m.solving { "Solving..." } else { "Solve" } }</button>
        </div>
    }
}

fn solution(m: &UseReducerHandle<Model>) -> Html {
    if m.solution.is_empty() {
        return html! {};
    }
    let moves = m.solution.iter().enumerate().map(|(i, &t)| {
        let class = if i + 1 == m.step { "mv now" } else if i < m.step { "mv done" } else { "mv" };
        html! { <span class={class}>{name(t)}</span> }
    });
    html! {
        <section class="panel">
            <h2>{format!("The solution: {} moves", m.solution.len())}</h2>
            <p class="note">{format!("X_eTaL solved the cube in {:.1} s: eigencube's search for the top layer, a cubelet at a time; the same search over known sequences (macros) for the middle and bottom layers; then eigencube's corner twists. Step through it, or play it.", m.solve_ms / 1000.0)}</p>
            <div class="controls">
                <button onclick={act(m, || Action::Step(-(isize::MAX / 2)))} disabled={m.step == 0}>{"|<"}</button>
                <button onclick={act(m, || Action::Step(-1))} disabled={m.step == 0}>{"<"}</button>
                <button onclick={act(m, || Action::TogglePlay)}>{ if m.playing { "Pause" } else { "Play" } }</button>
                <button onclick={act(m, || Action::Step(1))} disabled={m.step == m.solution.len()}>{">"}</button>
                <button onclick={act(m, || Action::Step(isize::MAX / 2))} disabled={m.step == m.solution.len()}>{">|"}</button>
                <span class="gen">{format!("move {} of {}", m.step, m.solution.len())}</span>
            </div>
            <p class="moves">{ for moves }</p>
        </section>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    {
        let d = model.dispatcher();
        use_effect_with(model.solving, move |&solving| {
            let t = solving.then(|| Timeout::new(30, move || d.dispatch(Action::Solve)));
            move || drop(t)
        });
    }
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let t = playing.then(|| Interval::new(350, move || d.dispatch(Action::Tick)));
            move || drop(t)
        });
    }
    let moves = if model.moves.is_empty() {
        "none: the solved cube".to_string()
    } else {
        model.moves.iter().map(|&m| name(m)).collect::<Vec<_>>().join(" ")
    };
    let solving = model.program.contains("ec:s_olve");
    let ran = if solving { between(&model.program, "(sol, lens, e)", "0 c_at sol") } else { between(&model.program, "r_avel ec:s_tickers", "ec:solved") };
    let lib = if solving { between(LIBRARY, "l:s_olve := { hybrid S ->", "### Notation") } else { between(LIBRARY, "h:c_hildren := { S ->", "## l:t_urn m S") };
    let what = if solving { "Highlighted: the solver, 29 search stages and the endgame." } else { "Highlighted: every turn of a batch of cubes, as two matrix products." };
    html! {
        <>
        <header>
            { header("Eigencube", "A Rubik's cube as linear algebra: each of the 26 cubelets is a point of {-1, 0, 1}^3 carrying a rotation matrix, and a turn moves the cubelets on the turning side (a dot product) by one matrix product. X_eTaL turns every cubelet at once, then reads the stickers back from the matrices. Turn the faces, scramble, and solve.", about(include_str!("../../demo.toml"))) }
            { turns(&model) }
            { notice(&model.notice) }
        </header>
        <main>
            <div class="layout even">
                <div class="col">
                    <section class="panel">
                        <h2>{"The cube"}</h2>
                        <p class="note">{"Unfolded: up on top, then left, front, right and back, then down. A face turns clockwise as seen facing it; a prime (') turns it back."}</p>
                        { net(&model.stickers) }
                        <p class="moves"><b>{format!("{} moves made: ", model.moves.len())}</b>{moves}</p>
                        <p class="note">{format!("X_eTaL turned the cube in {:.0} ms.", model.ms)}</p>
                    </section>
                    { solution(&model) }
                </div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran last in your browser: it imports the Eigencube library, writes in the moves made, and prints the stickers (or, to solve, the solution)."}</p>
                        { listing(&model.program, ran) }
                    </section>
                    <section class="panel code">
                        <h2>{"The library"}</h2>
                        <p class="note">{format!("Eigencube.xtl, from X_eTaL-libraries at the pinned commit: the cube, its turns, its stickers and the solver. {what}")}</p>
                        { listing(LIBRARY, lib) }
                    </section>
                </div>
            </div>
        </main>
        { footer() }
        </>
    }
}
