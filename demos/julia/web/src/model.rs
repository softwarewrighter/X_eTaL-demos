//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{program, counts, Set, View, PRESETS};
use crate::view::Stage;

pub const K: usize = 48;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub c: (f64, f64),
    pub preset: usize,
    pub view: View,
    /// The programs X_eTaL ran for the Julia set and for the picker,
    /// exactly as run (the page shows them).
    pub program: Rc<String>,
    pub picker_text: Rc<String>,
    pub julia: Result<Rc<Vec<f64>>, String>,
    /// The Mandelbrot map c is picked on (computed once).
    pub map: Result<Rc<Vec<f64>>, String>,
    pub playing: bool,
    /// Where Play is on its walk around the main cardioid (radians).
    pub t: f64,
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
}

pub enum Action {
    Pick(usize, usize),
    Preset(usize),
    ZoomIn(usize, usize),
    ZoomOut,
    Reset,
    TogglePlay,
    Tick,
    Focus(Stage),
}

/// A point just outside the main cardioid's edge, at angle t: the Julia
/// sets there change shape quickly as t moves.
pub fn cardioid(t: f64) -> (f64, f64) {
    let (s, c) = t.sin_cos();
    let (s2, c2) = (2.0 * t).sin_cos();
    let r = 1.02;
    (r * (c / 2.0 - c2 / 4.0), r * (s / 2.0 - s2 / 4.0))
}

impl Model {
    pub fn new() -> Self {
        let (_, a, b) = PRESETS[0];
        Model {
            c: (a, b),
            preset: 0,
            view: View::JULIA,
            program: Rc::new(String::new()),
            picker_text: Rc::new(program(&View::PICKER, 32, Set::Mandelbrot)),
            julia: Err(String::new()),
            map: counts(&View::PICKER, 32, Set::Mandelbrot).map(Rc::new),
            playing: false,
            t: 0.4,
            focus: Stage::Call,
            ms: 0.0,
            notice: None,
        }
        .rerun()
    }

    fn rerun(self) -> Self {
        let t = now();
        match counts(&self.view, K, Set::Julia(self.c.0, self.c.1)) {
            Ok(j) => Model { julia: Ok(Rc::new(j)), program: Rc::new(program(&self.view, K, Set::Julia(self.c.0, self.c.1))), ms: now() - t, notice: None, ..self },
            Err(e) if self.julia.is_err() => Model { julia: Err(e), ..self },
            Err(e) => Model { playing: false, notice: Some(format!("X_eTaL could not run that: {e}")), ..self },
        }
    }

    fn with_c(self, c: (f64, f64)) -> Self {
        Model { c, view: View::JULIA, ..self }.rerun()
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
            Action::Pick(y, x) => {
                let c = View::PICKER.point(y, x);
                Model { playing: false, ..m }.with_c(c)
            }
            Action::Preset(i) => {
                let (_, a, b) = PRESETS[i % PRESETS.len()];
                Model { preset: i, playing: false, ..m }.with_c((a, b))
            }
            Action::ZoomIn(y, x) if m.view.w / 2.0 > 1e-9 => {
                let (cx, cy) = m.view.point(y, x);
                Model { view: View { cx, cy, w: m.view.w / 2.0, ..m.view }, ..m }.rerun()
            }
            Action::ZoomIn(..) => Model { notice: Some("Zoomed in as far as this page goes: zoom out or reset.".into()), ..m },
            Action::ZoomOut => Model { view: View { w: m.view.w * 2.0, ..m.view }, ..m }.rerun(),
            Action::Reset => Model { view: View::JULIA, ..m }.rerun(),
            Action::TogglePlay => Model { playing: !m.playing, ..m },
            Action::Tick => {
                let t = m.t + 0.03;
                Model { t, ..m }.with_c(cardioid(t))
            }
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
