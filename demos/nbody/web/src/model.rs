//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{run, Anatomy, Bodies, PRESETS};
use crate::view::Stage;

/// How many past positions a trail keeps.
pub const TRAIL: usize = 240;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub preset: usize,
    pub bodies: Bodies,
    pub last: Option<Rc<Anatomy>>,
    /// Each body's past positions, oldest first.
    pub trails: Rc<Vec<Vec<(f64, f64)>>>,
    pub steps: usize,
    pub per_frame: usize,
    pub playing: bool,
    pub selected: usize,
    pub focus: Stage,
    /// The energy at the start, to show the integrator's drift.
    pub e0: f64,
    pub ms: f64,
    pub notice: Option<String>,
}

pub enum Action {
    Tick,
    TogglePlay,
    Preset(usize),
    Restart,
    Select(usize),
    PerFrame(usize),
    Focus(Stage),
}

impl Model {
    pub fn new(preset: usize) -> Self {
        let p = &PRESETS[preset % PRESETS.len()];
        let bodies = (p.bodies)();
        Model {
            preset,
            e0: bodies.energy(&p.physics),
            trails: Rc::new(vec![Vec::new(); bodies.len()]),
            bodies,
            last: None,
            steps: 0,
            per_frame: p.per_frame,
            playing: true,
            selected: 0,
            focus: Stage::Reduce,
            ms: 0.0,
            notice: None,
        }
        .advance(0)
    }

    fn advance(self, steps: usize) -> Self {
        let ph = PRESETS[self.preset].physics;
        let t = now();
        match run(&self.bodies, &ph, steps) {
            Ok(a) => {
                let mut trails = (*self.trails).clone();
                for (i, tr) in trails.iter_mut().enumerate() {
                    tr.push((a.next.x[i], a.next.y[i]));
                    if tr.len() > TRAIL {
                        tr.remove(0);
                    }
                }
                Model {
                    bodies: a.next.clone(),
                    last: Some(Rc::new(a)),
                    trails: Rc::new(trails),
                    steps: self.steps + steps,
                    ms: now() - t,
                    notice: None,
                    ..self
                }
            }
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
            Action::Preset(i) => Model::new(i),
            Action::Restart => Model { per_frame: m.per_frame, playing: m.playing, focus: m.focus, ..Model::new(m.preset) },
            Action::Select(i) => Model { selected: i.min(m.bodies.len() - 1), ..m },
            Action::PerFrame(n) => Model { per_frame: n.clamp(1, 40), ..m },
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
