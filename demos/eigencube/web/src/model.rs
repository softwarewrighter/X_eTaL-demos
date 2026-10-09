//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{inverse, program, run};

/// How many random moves a scramble makes.
pub const SCRAMBLE: usize = 25;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    /// Every move made since the solved cube, in order.
    pub moves: Vec<u8>,
    /// The 54 sticker colors X_eTaL computed for `moves`.
    pub stickers: Vec<u8>,
    /// The program X_eTaL ran for `stickers`, exactly as run.
    pub program: Rc<String>,
    pub ms: f64,
    pub notice: Option<String>,
    seed: u64,
}

pub enum Action {
    Turn(u8),
    Undo,
    Scramble,
    Reset,
}

impl Model {
    pub fn new() -> Self {
        let m = Model { moves: vec![], stickers: vec![], program: Rc::new(String::new()), ms: 0.0, notice: None, seed: 0x9e37_79b9_7f4a_7c15 ^ now().to_bits() };
        m.with(vec![])
    }

    /// Run `moves` through X_eTaL; on an error keep the last good cube.
    fn with(self, moves: Vec<u8>) -> Self {
        let t = now();
        match run(&moves) {
            Ok(s) => Model { program: Rc::new(program(&moves)), stickers: s, moves, ms: now() - t, notice: None, ..self },
            Err(e) => Model { notice: Some(format!("X_eTaL stopped: {e} (showing the last cube)")), ..self },
        }
    }

    /// The next of a stream of random moves (xorshift), never undoing
    /// the one before it.
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
            Action::Turn(t) => {
                let mut ms = m.moves.clone();
                ms.push(t);
                m.with(ms)
            }
            Action::Undo => {
                let mut ms = m.moves.clone();
                ms.pop();
                m.with(ms)
            }
            Action::Scramble => m.scramble(),
            Action::Reset => m.with(vec![]),
        })
    }
}
