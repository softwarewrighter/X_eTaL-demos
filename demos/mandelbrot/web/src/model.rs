//! The page's state and what each control does to it.

use std::rc::Rc;

use yew::Reducible;

use microscope::run::now;

use crate::micro::{orbit, run, Frame, View, MIN_WIDTH};
use crate::view::Stage;

pub const K_MAX: usize = 64;
pub const K_START: usize = 32;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub view: View,
    pub k: usize,
    pub frame: Result<Rc<Frame>, String>,
    pub selected: (usize, usize),
    pub orbit: Result<Vec<(f64, f64)>, String>,
    pub playing: bool,
    pub zooming: bool,
    pub focus: Stage,
    /// How long the last X_eTaL run took, in milliseconds.
    pub ms: f64,
    /// Why the last change was not shown (the page keeps the last good picture).
    pub notice: Option<String>,
}

pub enum Action {
    SetK(usize),
    Tick,
    TogglePlay,
    Click(usize, usize),
    ZoomOut,
    Reset,
    Zooming(bool),
    Focus(Stage),
}

impl Model {
    pub fn new() -> Self {
        let v = View::HOME;
        Model {
            view: v,
            k: K_START,
            frame: Err(String::new()),
            selected: (v.rows / 2, v.cols / 3),
            orbit: Ok(Vec::new()),
            playing: false,
            zooming: false,
            focus: Stage::Iterate,
            ms: 0.0,
            notice: None,
        }
        .rerun()
    }

    fn rerun(mut self) -> Self {
        let t = now();
        self.frame = run(&self.view, self.k).map(Rc::new);
        self.ms = now() - t;
        self.playing &= self.frame.is_ok();
        self.reorbit()
    }

    fn reorbit(mut self) -> Self {
        let (y, x) = self.selected;
        self.orbit = orbit(self.view.point(y, x), self.k.max(1));
        self
    }

    /// `next`, if X_eTaL ran it; else this model, saying why not.
    fn or_keep(self, next: Model) -> Model {
        match &next.frame {
            Ok(_) => Model { notice: None, ..next },
            Err(e) => Model { notice: Some(format!("X_eTaL could not run that view: {e}")), playing: false, ..self },
        }
    }

    fn click(self, y: usize, x: usize) -> Self {
        if !self.zooming {
            return Model { selected: (y, x), notice: None, ..self }.reorbit();
        }
        let view = self.view.zoom(y, x, 2.0);
        if view.w < MIN_WIDTH {
            let notice = "That is as deep as 64-bit floats can go: zoom out or reset.".to_string();
            return Model { notice: Some(notice), ..self };
        }
        let selected = (view.rows / 2, view.cols / 2);
        let next = Model { view, selected, ..self.clone() }.rerun();
        self.or_keep(next)
    }

    fn tick(self) -> Self {
        match self.k >= K_MAX {
            true => Model { playing: false, ..self },
            false => self.clone().or_keep(Model { k: self.k + 1, ..self }.rerun()),
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
            Action::SetK(k) => m.clone().or_keep(Model { k: k.min(K_MAX), ..m }.rerun()),
            Action::Tick => m.tick(),
            Action::TogglePlay if !m.playing && m.k >= K_MAX => Model { k: 0, playing: true, ..m }.rerun(),
            Action::TogglePlay => Model { playing: !m.playing, ..m },
            Action::Click(y, x) => m.click(y, x),
            Action::ZoomOut => m.clone().or_keep(Model { view: View { w: m.view.w * 2.0, ..m.view }, ..m }.rerun()),
            Action::Reset => Model { zooming: m.zooming, ..Model::new() },
            Action::Zooming(z) => Model { zooming: z, ..m },
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
