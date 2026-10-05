//! The model: the demo's own X_eTaL program (langtons-ant.xtl), whose
//! step moves the ant as a one-hot mask, run on a board the page
//! keeps. Nothing here knows about the browser.

use microscope::run::{matrix, numbers, output, section};

/// The command-line program; the page runs its core.
pub const SOURCE: &str = include_str!("../../langtons-ant.xtl");

pub fn core() -> &'static str {
    section(SOURCE, "# A matrix as one plane", "# -- end of the core")
}

pub const N: usize = 64;

/// The board (0 white, 1 black), where the ant stands, its direction
/// (0 up, 1 right, 2 down, 3 left), and how many steps it has taken.
#[derive(Clone, Debug, PartialEq)]
pub struct Ant {
    pub board: Vec<i64>,
    pub y: usize,
    pub x: usize,
    pub dir: i64,
    pub steps: usize,
}

impl Ant {
    pub fn start() -> Self {
        Ant { board: vec![0; N * N], y: N / 2, x: N / 2, dir: 0, steps: 0 }
    }
}

/// The last step, as X_eTaL computed it: where the ant stood, facing
/// which way, on which color.
#[derive(Clone, Debug, PartialEq)]
pub struct Last {
    pub y: usize,
    pub x: usize,
    pub dir: i64,
    pub cell: i64,
}

/// The program: `steps` (at least 1) steps from `ant`; prints the board
/// after, the ant's place and direction, and the last step's.
pub fn program(ant: &Ant, steps: usize) -> String {
    format!(
        "rows := {N}\ncols := {N}\n{}row := (o_ffsets rows) 'l_eft t_able o_ffsets cols\n\
         col := (o_ffsets rows) 'r_ight t_able o_ffsets cols\n{}\
         a0 := 1 * (row = {}) & col = {}\nd0 := {} + 0 * row\n\
         prev := {} 'u:s_tep p_ower (u:p_lane b0) c_at (u:p_lane a0) c_at u:p_lane d0\ns := u:s_tep prev\n\
         r_avel 1 s_elect s\nw_here r_avel 2 s_elect s\nf_irst r_avel 3 s_elect s\n\
         w_here r_avel 2 s_elect prev\nf_irst r_avel 3 s_elect prev\n'+ r_/ r_avel (2 s_elect prev) * 1 s_elect prev\n",
        core(),
        matrix("b0", N, N, ant.board.iter().map(|v| v.to_string())),
        ant.y,
        ant.x,
        ant.dir,
        steps.max(1) - 1
    )
}

fn place(i: i64) -> (usize, usize) {
    let i = (i - 1) as usize;
    (i / N, i % N)
}

/// Run `steps` steps of `ant` through X_eTaL.
pub fn run(ant: &Ant, steps: usize) -> Result<(Ant, Last), String> {
    let out = output(&program(ant, steps), 6)?;
    let one = |i: usize| numbers::<i64>(&out[i], 1).map(|v| v[0]);
    let (y, x) = place(one(1)?);
    let (ly, lx) = place(one(3)?);
    let next = Ant { board: numbers(&out[0], N * N)?, y, x, dir: one(2)?, steps: ant.steps + steps.max(1) };
    Ok((next, Last { y: ly, x: lx, dir: one(4)?, cell: one(5)? }))
}
