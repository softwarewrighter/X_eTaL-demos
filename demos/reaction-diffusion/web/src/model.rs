//! The page's state and what each control does to it.

use std::rc::Rc;

use yew::Reducible;

use microscope::run::now;

use crate::micro::{preset, run, Anatomy, Grid, Rates};
use crate::view::Stage;

pub const N: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    /// The grid the next run starts from.
    pub grid: Grid,
    pub rates: Rates,
    pub preset: usize,
    /// The arrays of the last step X_eTaL ran.
    pub last: Rc<Anatomy>,
    pub steps: usize,
    pub per_frame: usize,
    pub playing: bool,
    pub seeding: bool,
    pub selected: (usize, usize),
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
}

pub enum Action {
    Tick,
    TogglePlay,
    Preset(usize),
    Reset,
    Click(usize, usize),
    Seeding(bool),
    PerFrame(usize),
    Focus(Stage),
}

impl Model {
    pub fn new(preset_index: usize) -> Self {
        let grid = Grid::seeded(N);
        let rates = preset(preset_index);
        let last = Rc::new(run(&grid, &rates, 1).unwrap_or_else(|_| empty(&grid)));
        Model {
            grid: last.next.clone(),
            rates,
            preset: preset_index,
            last,
            steps: 1,
            per_frame: 20,
            playing: true,
            seeding: true,
            selected: (N * 3 / 10, N * 35 / 100),
            focus: Stage::Update,
            ms: 0.0,
            notice: None,
        }
    }

    /// Run `steps` steps from the grid; on an error keep everything and say why.
    fn advance(self, steps: usize) -> Self {
        let t = now();
        match run(&self.grid, &self.rates, steps) {
            Ok(a) => Model {
                grid: a.next.clone(),
                last: Rc::new(a),
                steps: self.steps + steps,
                ms: now() - t,
                notice: None,
                ..self
            },
            Err(e) => Model { playing: false, notice: Some(format!("X_eTaL stopped: {e}")), ..self },
        }
    }

    fn click(mut self, y: usize, x: usize) -> Self {
        self.selected = (y, x);
        if !self.seeding {
            return self;
        }
        self.grid.drop(y, x, 2);
        self.advance(1)
    }
}

fn empty(g: &Grid) -> Anatomy {
    let z = vec![0.0; g.n * g.n];
    Anatomy { u: g.u.clone(), v: g.v.clone(), lap_u: z.clone(), lap_v: z.clone(), uvv: z, next: g.clone() }
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
            Action::Preset(i) => Model { playing: m.playing, per_frame: m.per_frame, ..Model::new(i) },
            Action::Reset => Model { per_frame: m.per_frame, ..Model::new(m.preset) },
            Action::Click(y, x) => m.click(y, x),
            Action::Seeding(s) => Model { seeding: s, ..m },
            Action::PerFrame(n) => Model { per_frame: n.clamp(1, 60), ..m },
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
