//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{expanded_step, program, run, Kernel, Run};
use crate::view::Stage;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub kernel: Kernel,
    /// The preset the kernel is, until a number is edited.
    pub preset: Option<usize>,
    pub last: Option<Rc<Run>>,
    /// The kernel the last good run used (the inspector's terms).
    pub ran: Kernel,
    /// The program X_eTaL ran for `last`, exactly as run.
    pub program: Rc<String>,
    /// The step's line after X_eTaL expanded the macro call.
    pub expansion: Rc<String>,
    pub selected: (usize, usize),
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
}

pub enum Action {
    Preset(usize),
    Side(usize),
    Cell(usize, f64),
    Divide(f64),
    Steps(usize),
    Click(usize, usize),
    Focus(Stage),
}

impl Model {
    pub fn new() -> Self {
        let kernel = Kernel::preset(0);
        Model {
            ran: kernel.clone(),
            kernel,
            preset: Some(0),
            last: None,
            program: Rc::new(String::new()),
            expansion: Rc::new(String::new()),
            selected: (11, 24),
            focus: Stage::Call,
            ms: 0.0,
            notice: None,
        }
        .rerun()
    }

    /// Expand and run the kernel; on an X_eTaL error keep the last good
    /// result and say why.
    fn rerun(self) -> Self {
        let t = now();
        let k = self.kernel.clone();
        match (run(&k), expanded_step(&k)) {
            (Ok(r), Ok(line)) => Model {
                last: Some(Rc::new(r)),
                ran: k.clone(),
                program: Rc::new(program(&k)),
                expansion: Rc::new(line),
                ms: now() - t,
                notice: None,
                ..self
            },
            (Err(e), _) | (_, Err(e)) => Model { notice: Some(format!("X_eTaL stopped: {e} (showing the last good result)")), ..self },
        }
    }

    fn edited(self, kernel: Kernel) -> Self {
        Model { kernel, preset: None, ..self }.rerun()
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
            Action::Preset(i) => Model { kernel: Kernel::preset(i), preset: Some(i), ..m }.rerun(),
            Action::Side(s) => {
                let k = m.kernel.resized(s);
                m.edited(k)
            }
            Action::Cell(i, v) => {
                let mut k = m.kernel.clone();
                if i < k.w.len() {
                    k.w[i] = v;
                }
                m.edited(k)
            }
            Action::Divide(v) => {
                let k = Kernel { divide: if v == 0.0 { 1.0 } else { v }, ..m.kernel.clone() };
                m.edited(k)
            }
            Action::Steps(n) => {
                let k = Kernel { steps: n.clamp(1, 60), ..m.kernel.clone() };
                Model { kernel: k, ..m }.rerun()
            }
            Action::Click(r, c) => Model { selected: (r, c), ..m },
            Action::Focus(f) => Model { focus: f, ..m },
        })
    }
}
