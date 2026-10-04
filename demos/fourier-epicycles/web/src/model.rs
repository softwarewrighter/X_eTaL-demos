//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{preset, program, run, Anatomy, Curve, N};
use crate::view::Stage;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub preset: Option<usize>,
    pub last: Option<Rc<Anatomy>>,
    /// The program X_eTaL ran for `last`, exactly as run (the page shows it).
    pub program: Rc<String>,
    /// How many circles are drawn (the strongest first).
    pub circles: usize,
    /// The time: 0 .. N - 1.
    pub frame: usize,
    pub playing: bool,
    /// The path being drawn, while the mouse is down.
    pub drawing: Option<Vec<(f64, f64)>>,
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
}

pub enum Action {
    Preset(usize),
    Circles(usize),
    Tick,
    TogglePlay,
    DrawStart(f64, f64),
    DrawTo(f64, f64),
    DrawEnd,
    Focus(Stage),
}

impl Model {
    pub fn new() -> Self {
        Model {
            preset: Some(2),
            last: None,
            program: Rc::new(String::new()),
            circles: 12,
            frame: 0,
            playing: true,
            drawing: None,
            focus: Stage::Chain,
            ms: 0.0,
            notice: None,
        }
        .transform(preset(2))
    }

    /// Run `curve` through X_eTaL; on an error keep the last good one.
    fn transform(self, curve: Curve) -> Self {
        let t = now();
        let text = program(&curve);
        match run(&curve) {
            Ok(a) => Model { last: Some(Rc::new(a)), program: Rc::new(text), ms: now() - t, notice: None, ..self },
            Err(e) => Model { notice: Some(format!("X_eTaL stopped: {e} (showing the last curve)")), ..self },
        }
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
            Action::Preset(i) => Model { preset: Some(i), frame: 0, ..m }.transform(preset(i)),
            Action::Circles(k) => Model { circles: k.clamp(1, N), ..m },
            Action::Tick => Model { frame: (m.frame + 1) % N, ..m },
            Action::TogglePlay => Model { playing: !m.playing, ..m },
            Action::DrawStart(x, y) => Model { drawing: Some(vec![(x, y)]), playing: false, ..m },
            Action::DrawTo(x, y) => match m.drawing.clone() {
                Some(mut d) => {
                    d.push((x, y));
                    Model { drawing: Some(d), ..m }
                }
                None => m,
            },
            Action::DrawEnd => match m.drawing.clone().and_then(|d| Curve::resampled(&d)) {
                Some(c) => Model { drawing: None, preset: None, frame: 0, playing: true, ..m }.transform(c),
                None => Model { drawing: None, notice: Some("Draw a longer closed path (press, drag round, release).".into()), ..m },
            },
            Action::Focus(f) => Model { focus: f, ..m },
        })
    }
}
