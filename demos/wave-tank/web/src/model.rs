//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{run, Anatomy, Surface, COLS, ROWS};
use crate::view::Stage;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub surface: Surface,
    pub scene: usize,
    pub last: Option<Rc<Anatomy>>,
    pub steps: usize,
    pub per_frame: usize,
    pub playing: bool,
    pub selected: (usize, usize),
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
}

pub enum Action {
    Tick,
    TogglePlay,
    Scene(usize),
    Clear,
    Click(usize, usize),
    PerFrame(usize),
    Focus(Stage),
}

impl Model {
    pub fn new(scene: usize) -> Self {
        Model {
            surface: Surface::flat(),
            scene,
            last: None,
            steps: 0,
            per_frame: 4,
            playing: true,
            selected: (ROWS / 2, COLS / 2),
            focus: Stage::Step,
            ms: 0.0,
            notice: None,
        }
        .advance(1)
    }

    fn advance(self, steps: usize) -> Self {
        let t = now();
        match run(&self.surface, self.scene, steps) {
            Ok(a) => Model {
                surface: a.next.clone(),
                last: Some(Rc::new(a)),
                steps: self.steps + steps,
                ms: now() - t,
                notice: None,
                ..self
            },
            Err(e) => Model { playing: false, notice: Some(format!("X_eTaL stopped: {e}")), ..self },
        }
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
            Action::Scene(i) => Model { per_frame: m.per_frame, ..Model::new(i) },
            Action::Clear => Model { per_frame: m.per_frame, playing: m.playing, ..Model::new(m.scene) },
            Action::Click(y, x) => {
                let mut surface = m.surface.clone();
                surface.drop(y, x);
                Model { surface, selected: (y, x), ..m }.advance(1)
            }
            Action::PerFrame(n) => Model { per_frame: n.clamp(1, 20), ..m },
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
