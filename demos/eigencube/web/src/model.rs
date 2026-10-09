//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{inverse, program, run, solve, solve_program};

/// How many random moves a scramble makes.
pub const SCRAMBLE: usize = 25;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    /// Every move made since the solved cube, in order.
    pub moves: Vec<u8>,
    /// A solution of `moves`, and how many of its moves are shown made.
    pub solution: Vec<u8>,
    pub step: usize,
    /// The 54 sticker colors X_eTaL computed for the cube shown.
    pub stickers: Vec<u8>,
    /// The program X_eTaL ran last, exactly as run.
    pub program: Rc<String>,
    pub ms: f64,
    pub solve_ms: f64,
    pub solving: bool,
    pub playing: bool,
    pub notice: Option<String>,
    seed: u64,
}

pub enum Action {
    Turn(u8),
    Undo,
    Scramble,
    Reset,
    /// Show "Solving" first, then `Solve` runs (the page repaints between).
    StartSolve,
    Solve,
    Step(isize),
    TogglePlay,
    Tick,
}

impl Model {
    pub fn new() -> Self {
        let m = Model {
            moves: vec![],
            solution: vec![],
            step: 0,
            stickers: vec![],
            program: Rc::new(String::new()),
            ms: 0.0,
            solve_ms: 0.0,
            solving: false,
            playing: false,
            notice: None,
            seed: 0x9e37_79b9_7f4a_7c15 ^ now().to_bits(),
        };
        m.with(vec![])
    }

    /// The moves of the cube shown: those made, then the solution so far.
    pub fn shown(&self) -> Vec<u8> {
        let mut v = self.moves.clone();
        v.extend_from_slice(&self.solution[..self.step]);
        v
    }

    /// Draw the cube shown through X_eTaL; on an error keep the last one.
    fn draw(self) -> Self {
        let t = now();
        let shown = self.shown();
        match run(&shown) {
            Ok(s) => Model { program: Rc::new(program(&shown)), stickers: s, ms: now() - t, notice: None, ..self },
            Err(e) => Model { notice: Some(format!("X_eTaL stopped: {e} (showing the last cube)")), playing: false, ..self },
        }
    }

    /// New moves made: any solution is dropped.
    fn with(self, moves: Vec<u8>) -> Self {
        Model { moves, solution: vec![], step: 0, playing: false, ..self }.draw()
    }

    /// A turn made by hand keeps the part of a solution already shown.
    fn turned(self, t: u8) -> Self {
        let mut ms = self.shown();
        ms.push(t);
        self.with(ms)
    }

    /// The next random moves (xorshift), never undoing the one before.
    fn scramble(mut self) -> Self {
        let mut ms = vec![];
        while ms.len() < SCRAMBLE {
            self.seed ^= self.seed << 13;
            self.seed ^= self.seed >> 7;
            self.seed ^= self.seed << 17;
            let m = (self.seed % 12) as u8 + 1;
            if ms.last().map_or(true, |&l| l != inverse(m)) {
                ms.push(m);
            }
        }
        self.with(ms)
    }

    fn solved(self) -> Self {
        let moves = self.shown();
        let t = now();
        match solve(&moves) {
            Ok(sol) => Model { moves: moves.clone(), solution: sol, step: 0, solving: false, solve_ms: now() - t, program: Rc::new(solve_program(&moves)), notice: None, ..self },
            Err(e) => Model { solving: false, notice: Some(format!("X_eTaL stopped: {e}")), ..self },
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
        if m.solving && !matches!(action, Action::Solve) {
            return self;
        }
        Rc::new(match action {
            Action::Turn(t) => m.turned(t),
            Action::Undo => {
                let mut ms = m.shown();
                ms.pop();
                m.with(ms)
            }
            Action::Scramble => m.scramble(),
            Action::Reset => m.with(vec![]),
            Action::StartSolve => Model { solving: true, playing: false, ..m },
            Action::Solve => m.solved(),
            Action::Step(d) => {
                let step = (m.step as isize + d).clamp(0, m.solution.len() as isize) as usize;
                Model { step, playing: false, ..m }.draw()
            }
            Action::TogglePlay => {
                let restart = m.step == m.solution.len();
                Model { playing: !m.playing && !m.solution.is_empty(), step: if restart { 0 } else { m.step }, ..m }.draw()
            }
            Action::Tick => match m.step < m.solution.len() {
                true => Model { step: m.step + 1, ..m }.draw(),
                false => Model { playing: false, ..m },
            },
        })
    }
}
