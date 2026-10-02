//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{run, Ant, Last};
use crate::view::Stage;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub ant: Ant,
    pub last: Option<Last>,
    pub per_frame: usize,
    pub playing: bool,
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
}

/// Play pauses here: the highway has formed, and on this small board it
/// would soon wrap around the edges into its own trail.
pub const PAUSE_AT: usize = 11_500;

pub enum Action {
    Tick,
    One,
    TogglePlay,
    Reset,
    PerFrame(usize),
    Focus(Stage),
}

impl Model {
    pub fn new() -> Self {
        Model { ant: Ant::start(), last: None, per_frame: 100, playing: true, focus: Stage::Turn, ms: 0.0, notice: None }
    }

    fn advance(self, n: usize) -> Self {
        let t = now();
        match run(&self.ant, n) {
            Ok((ant, last)) => Model { ant, last: Some(last), ms: now() - t, notice: None, ..self },
            Err(e) => Model { playing: false, notice: Some(format!("X_eTaL stopped: {e}")), ..self },
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::new()
    }
}

impl Reducible for Model {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Action) -> Rc<Self> {
        let m = (*self).clone();
        Rc::new(match action {
            Action::Tick => {
                let n = m.per_frame;
                let before = m.ant.steps;
                let next = m.advance(n);
                match before < PAUSE_AT && next.ant.steps >= PAUSE_AT {
                    true => Model {
                        playing: false,
                        notice: Some("Paused: the highway has formed. On this 64 by 64 board it will wrap around the edges and run into its own trail; press Play to watch that.".into()),
                        ..next
                    },
                    false => next,
                }
            }
            Action::One => Model { playing: false, ..m }.advance(1),
            Action::TogglePlay => Model { playing: !m.playing, ..m },
            Action::Reset => Model { per_frame: m.per_frame, playing: false, ..Model::new() },
            Action::PerFrame(n) => Model { per_frame: n.clamp(1, 1000), ..m },
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
