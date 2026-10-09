//! The page: the cube as a net of 54 stickers, the turn buttons, the
//! moves made, and the program X_eTaL ran.

use yew::prelude::*;

use microscope::chrome::{about, footer, header, notice};
use microscope::source::{between, listing};

use crate::micro::name;
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
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    let moves = if model.moves.is_empty() {
        "none: the solved cube".to_string()
    } else {
        model.moves.iter().map(|&m| name(m)).collect::<Vec<_>>().join(" ")
    };
    html! {
        <>
        <header>
            { header("Eigencube", "A Rubik's cube as linear algebra: each of the 26 cubelets is a point of {-1, 0, 1}^3 carrying a rotation matrix, and a turn moves the cubelets on the turning side (a dot product) by one matrix product. X_eTaL turns every cubelet at once, then reads the stickers back from the matrices. Turn the faces, or scramble.", about(include_str!("../../demo.toml"))) }
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
                        <p class="moves"><b>{format!("{} moves: ", model.moves.len())}</b>{moves}</p>
                        <p class="note">{format!("X_eTaL turned the cube in {:.0} ms.", model.ms)}</p>
                    </section>
                </div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program"}</h2>
                        <p class="note">{"Exactly the program X_eTaL ran in your browser for this cube: the core of eigencube.xtl, then the moves made and the stickers it prints. Highlighted: every turn of a batch of cubes, as two matrix products."}</p>
                        { listing(&model.program, between(&model.program, "u:c_hildren := ", "## u:t_urn m S")) }
                    </section>
                </div>
            </div>
        </main>
        { footer() }
        </>
    }
}
