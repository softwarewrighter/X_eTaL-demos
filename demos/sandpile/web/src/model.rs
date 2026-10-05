//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{program, run, Pile, Rounds, MID};
use crate::view::Stage;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub pile: Pile,
    pub last: Option<Rc<Rounds>>,
    /// The program X_eTaL ran for `last`, exactly as run (the page shows it).
    pub program: Rc<String>,
    /// Grains dropped so far (those not on the grid fell off the edge).
    pub dropped: i64,
    pub rounds: usize,
    pub per_frame: usize,
    pub playing: bool,
    pub selected: (usize, usize),
    /// Whether a click drops grains (else it only inspects).
    pub click_drops: bool,
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
}

pub enum Action {
    Tick,
    TogglePlay,
    DropCentre(i64),
    Everywhere(i64),
    Clear,
    Click(usize, usize),
    ClickDrops(bool),
    PerFrame(usize),
    Focus(Stage),
}

impl Model {
    pub fn new() -> Self {
        let mut pile = Pile::empty();
        pile.drop(MID, MID, 10000);
        Model {
            pile,
            last: None,
            program: Rc::new(String::new()),
            dropped: 10000,
            rounds: 0,
            per_frame: 20,
            playing: true,
            selected: (MID, MID),
            click_drops: false,
            focus: Stage::Keep,
            ms: 0.0,
            notice: None,
        }
        .advance(0)
    }

    /// Run `n` rounds; on an X_eTaL error keep the last good pile.
    fn advance(self, n: usize) -> Self {
        let t = now();
        let text = program(&self.pile, n);
        match run(&self.pile, n) {
            Ok(r) => Model {
                pile: r.next.clone(),
                playing: self.playing && !r.stable,
                last: Some(Rc::new(r)),
                program: Rc::new(text),
                rounds: self.rounds + n,
                ms: now() - t,
                notice: None,
                ..self
            },
            Err(e) => Model { playing: false, notice: Some(format!("X_eTaL stopped: {e} (showing the last good pile)")), ..self },
        }
    }

    /// Change the pile, then show it (0 rounds) and play until it settles.
    fn change(self, f: impl FnOnce(&mut Pile) -> i64) -> Self {
        let mut pile = self.pile.clone();
        let added = f(&mut pile);
        Model { pile, dropped: self.dropped + added, playing: true, ..self }.advance(0)
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Reducible for Model {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Action) -> Rc<Self> {
        let m = (*self).clone();
        Rc::new(match action {
            Action::Tick => {
                let n = m.per_frame;
                m.advance(n)
            }
            Action::TogglePlay => Model { playing: !m.playing, ..m },
            Action::DropCentre(n) => m.change(|p| {
                p.drop(MID, MID, n);
                n
            }),
            Action::Everywhere(n) => m.change(|p| {
                p.everywhere(n);
                n * ((crate::micro::SIDE - 2) * (crate::micro::SIDE - 2)) as i64
            }),
            Action::Clear => Model { pile: Pile::empty(), dropped: 0, rounds: 0, playing: false, ..m }.advance(0),
            Action::Click(r, c) => {
                let m = Model { selected: (r, c), ..m };
                if m.click_drops && !Pile::on_edge(r, c) {
                    m.change(|p| {
                        p.drop(r, c, 200);
                        200
                    })
                } else {
                    m
                }
            }
            Action::ClickDrops(b) => Model { click_drops: b, ..m },
            Action::PerFrame(n) => Model { per_frame: n.clamp(1, 200), ..m },
            Action::Focus(f) => Model { focus: f, ..m },
        })
    }
}
