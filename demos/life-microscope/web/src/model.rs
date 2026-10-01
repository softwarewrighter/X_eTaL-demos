//! The page's state and what each control does to it.

use std::rc::Rc;

use yew::Reducible;

use crate::micro::{examine, pattern, Anatomy, Board};

pub const ROWS: usize = 16;
pub const COLS: usize = 16;

/// The stages of one generation, in the order the line computes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Board,
    Rotate,
    Sum,
    Masks,
    Next,
}

pub const STAGES: [Stage; 5] = [Stage::Board, Stage::Rotate, Stage::Sum, Stage::Masks, Stage::Next];

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub board: Board,
    pub anatomy: Result<Anatomy, String>,
    pub generation: usize,
    pub pattern: usize,
    pub selected: (usize, usize),
    pub playing: bool,
    pub drawing: bool,
    pub focus: Stage,
}

pub enum Action {
    Step,
    TogglePlay,
    Reset,
    Clear,
    Pattern(usize),
    Cell(usize, usize),
    Drawing(bool),
    Focus(Stage),
}

impl Model {
    pub fn new(pattern_index: usize) -> Self {
        let board = pattern(pattern_index, ROWS, COLS);
        Model {
            anatomy: examine(&board),
            board,
            generation: 0,
            pattern: pattern_index,
            selected: (ROWS / 2, COLS / 2),
            playing: false,
            drawing: false,
            focus: Stage::Board,
        }
    }

    fn with_board(mut self, board: Board) -> Self {
        self.anatomy = examine(&board);
        self.playing &= self.anatomy.is_ok();
        self.board = board;
        self
    }

    fn step(self) -> Self {
        let Ok(a) = &self.anatomy else { return self };
        let board = Board { cells: a.next.clone(), ..self.board.clone() };
        let generation = self.generation + 1;
        Model { generation, ..self.with_board(board) }
    }

    fn cell(mut self, y: usize, x: usize) -> Self {
        self.selected = (y, x);
        if !self.drawing {
            return self;
        }
        let mut board = self.board.clone();
        board.toggle(y, x);
        self.with_board(board)
    }
}

impl Reducible for Model {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Action) -> Rc<Self> {
        let m = (*self).clone();
        Rc::new(match action {
            Action::Step => m.step(),
            Action::TogglePlay => Model { playing: !m.playing && m.anatomy.is_ok(), ..m },
            Action::Reset => Model { drawing: m.drawing, ..Model::new(m.pattern) },
            Action::Clear => Model { generation: 0, playing: false, ..m.with_board(Board::empty(ROWS, COLS)) },
            Action::Pattern(i) => Model { drawing: m.drawing, ..Model::new(i) },
            Action::Cell(y, x) => m.cell(y, x),
            Action::Drawing(d) => Model { drawing: d, ..m },
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
