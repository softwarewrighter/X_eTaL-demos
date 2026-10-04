//! The page's state and what each control does to it.

use std::rc::Rc;

use microscope::run::now;
use yew::Reducible;

use crate::micro::{program_1d, program_2d, first_row, grow, rules, start, step, History, Rule2, Step2, COLS, GENS, ROWS, WIDTH};
use crate::view::{Mode, Stage};

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub mode: Mode,
    pub focus: Stage,
    pub ms: f64,
    pub notice: Option<String>,
    // one dimension
    pub rule: u8,
    pub random: bool,
    pub history: Option<Rc<History>>,
    /// The programs X_eTaL ran for each mode, exactly as run (the page
    /// shows the one in use).
    pub program_1d: Rc<String>,
    pub program_2d: Rc<String>,
    pub gen: usize,
    pub cell: usize,
    // two dimensions
    pub which: usize,
    pub rule2: Rule2,
    pub board: Vec<i64>,
    pub last: Option<Rc<Step2>>,
    pub steps: usize,
    pub playing: bool,
    pub selected: (usize, usize),
}

pub enum Action {
    Mode(Mode),
    Focus(Stage),
    Rule(u8),
    FlipBit(usize),
    Random(bool),
    Pick(usize, usize),
    Which(usize),
    CycleEntry(usize),
    Paint(usize, usize),
    Tick,
    TogglePlay,
    Reset,
}

/// The mode the page opens in: `#2d` in the address opens two dimensions.
fn opening_mode() -> Mode {
    #[cfg(target_arch = "wasm32")]
    {
        let hash = web_sys::window().and_then(|w| w.location().hash().ok()).unwrap_or_default();
        if hash == "#2d" {
            return Mode::Two;
        }
    }
    Mode::One
}

impl Model {
    pub fn new() -> Self {
        let rule2 = rules()[0].clone();
        Model {
            mode: opening_mode(),
            focus: Stage::Number,
            ms: 0.0,
            notice: None,
            rule: 30,
            random: false,
            history: None,
            program_1d: Rc::new(String::new()),
            program_2d: Rc::new(String::new()),
            gen: 20,
            cell: WIDTH / 2,
            which: 0,
            board: start(0),
            rule2,
            last: None,
            steps: 0,
            playing: false,
            selected: (ROWS / 2, COLS / 2),
        }
        .regrow()
        .preview()
    }

    fn regrow(self) -> Self {
        let t = now();
        let text = program_1d(self.rule, &first_row(self.random), self.gen);
        match grow(self.rule, &first_row(self.random), self.gen) {
            Ok(h) => Model { history: Some(Rc::new(h)), program_1d: Rc::new(text), ms: now() - t, notice: None, ..self },
            Err(e) => Model { notice: Some(format!("X_eTaL stopped: {e}")), ..self },
        }
    }

    /// The coming step from the board as it is: its arrays, as X_eTaL
    /// computes them (the board is not changed).
    fn preview(self) -> Self {
        let t = now();
        let text = program_2d(&self.rule2.table, &self.board, 1);
        match step(&self.rule2.table, &self.board, 1) {
            Ok(s) => Model { last: Some(Rc::new(s)), program_2d: Rc::new(text), ms: now() - t, notice: None, ..self },
            Err(e) => Model { playing: false, notice: Some(format!("X_eTaL stopped: {e}")), ..self },
        }
    }

    /// Take the step the preview shows.
    fn tick(self) -> Self {
        match &self.last {
            Some(s) => Model { board: s.next.clone(), steps: self.steps + 1, ..self.clone() }.preview(),
            None => self.preview(),
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
            Action::Mode(mode) => Model { mode, playing: false, ..m },
            Action::Focus(s) => Model { focus: s, ..m },
            Action::Rule(rule) => Model { rule, ..m }.regrow(),
            Action::FlipBit(k) => Model { rule: m.rule ^ (1 << k), ..m }.regrow(),
            Action::Random(random) => Model { random, ..m }.regrow(),
            Action::Pick(g, x) => Model { gen: g.min(GENS - 2), cell: x, ..m }.regrow(),
            Action::Which(i) => {
                let rule2 = rules()[i].clone();
                Model { which: i, rule2, board: start(i), steps: 0, ..m }.preview()
            }
            Action::CycleEntry(i) => {
                let mut rule2 = m.rule2.clone();
                let n = rule2.states.len() as i64;
                rule2.table[i] = (rule2.table[i] + 1) % n;
                Model { rule2, ..m }.preview()
            }
            Action::Paint(y, x) => {
                let mut board = m.board.clone();
                let n = m.rule2.states.len() as i64;
                let i = y * COLS + x;
                board[i] = (board[i] + 1) % n;
                Model { board, selected: (y, x), ..m }.preview()
            }
            Action::Tick => m.tick(),
            Action::TogglePlay => Model { playing: !m.playing, ..m },
            Action::Reset => {
                let i = m.which;
                Model { board: start(i), rule2: rules()[i].clone(), steps: 0, ..m }.preview()
            }
        })
    }
}
