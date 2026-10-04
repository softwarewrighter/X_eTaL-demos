//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{program, run, Anatomy, Setup, BLURS, EDGES, SCENES};
use crate::view::Stage;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub setup: Setup,
    pub scene: usize,
    /// The preset each kernel came from, None once edited.
    pub blur: Option<usize>,
    pub edge: Option<usize>,
    pub last: Option<Rc<Anatomy>>,
    /// The program X_eTaL ran for `last`, exactly as run (the page shows it).
    pub program: Rc<String>,
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
}

pub enum Action {
    Scene(usize),
    Blur(usize),
    Edge(usize),
    BlurCell(usize, f64),
    EdgeCell(usize, f64),
    Thresh(f64),
    Pixel(usize, usize),
    Focus(Stage),
}

impl Model {
    pub fn new() -> Self {
        let m = Model { setup: Setup::new(0), scene: 0, blur: Some(0), edge: Some(0), last: None, program: Rc::new(String::new()), focus: Stage::Smooth, ms: 0.0, notice: None };
        m.rerun(Setup::new(0))
    }

    /// Run `setup`; on an X_eTaL error keep the last good state and say why.
    fn rerun(self, setup: Setup) -> Self {
        let t = now();
        match run(&setup) {
            Ok(a) => Model { program: Rc::new(program(&setup)), setup, last: Some(Rc::new(a)), ms: now() - t, notice: None, ..self },
            Err(e) => Model { notice: Some(format!("X_eTaL stopped: {e} (showing the last good run)")), ..self },
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
        let s = m.setup.clone();
        Rc::new(match action {
            Action::Scene(i) => {
                let i = i.min(SCENES.len() - 1);
                Model { scene: i, ..m }.rerun(Setup { scene: SCENES[i].1.to_string(), ..s })
            }
            Action::Blur(i) => Model { blur: Some(i), ..m }.rerun(Setup { blur: BLURS[i.min(BLURS.len() - 1)].1, ..s }),
            Action::Edge(i) => Model { edge: Some(i), ..m }.rerun(Setup { kx: EDGES[i.min(EDGES.len() - 1)].1, ..s }),
            Action::BlurCell(k, v) => {
                let mut blur = s.blur;
                blur[k] = v;
                Model { blur: None, ..m }.rerun(Setup { blur, ..s })
            }
            Action::EdgeCell(k, v) => {
                let mut kx = s.kx;
                kx[k] = v;
                Model { edge: None, ..m }.rerun(Setup { kx, ..s })
            }
            Action::Thresh(t) => m.rerun(Setup { thresh: t, ..s }),
            Action::Pixel(y, x) => m.rerun(Setup { pixel: (y, x), ..s }),
            Action::Focus(f) => Model { focus: f, ..m },
        })
    }
}
