//! The model: the demo's own X_eTaL program (sandpile.xtl): its edge,
//! its toppling round and the round that counts topples, run on a pile
//! the page keeps. Nothing here knows about the browser.

use microscope::run::{matrix, numbers, output, section};

/// The command-line program; the page runs its core.
pub const SOURCE: &str = include_str!("../../sandpile.xtl");

/// The edge, the toppling round, stability, the counting round.
pub fn core() -> &'static str {
    section(SOURCE, "# -- the edge", "# -- end of the core")
}

/// The grid is SIDE x SIDE; its outer ring is the edge (a sink).
pub const SIDE: usize = 81;
pub const MID: usize = SIDE / 2;

/// A pile: grains per cell, and how many times each cell has toppled.
#[derive(Clone, Debug, PartialEq)]
pub struct Pile {
    pub h: Vec<i64>,
    pub topples: Vec<i64>,
}

impl Pile {
    pub fn empty() -> Self {
        Pile { h: vec![0; SIDE * SIDE], topples: vec![0; SIDE * SIDE] }
    }

    pub fn on_edge(r: usize, c: usize) -> bool {
        r == 0 || c == 0 || r == SIDE - 1 || c == SIDE - 1
    }

    /// Drop `n` grains on cell (r, c) (none on the edge).
    pub fn drop(&mut self, r: usize, c: usize, n: i64) {
        if !Pile::on_edge(r, c) {
            self.h[r * SIDE + c] += n;
        }
    }

    /// Add `n` grains to every cell inside the edge.
    pub fn everywhere(&mut self, n: i64) {
        for r in 1..SIDE - 1 {
            for c in 1..SIDE - 1 {
                self.h[r * SIDE + c] += n;
            }
        }
    }

    pub fn grains(&self) -> i64 {
        self.h.iter().sum()
    }

    pub fn stable(&self) -> bool {
        self.h.iter().all(|&v| v < 4)
    }
}

/// What X_eTaL computed for some rounds.
#[derive(Clone, Debug, PartialEq)]
pub struct Rounds {
    /// The pile after the rounds.
    pub next: Pile,
    /// Who topples in the coming round, and how often: h d_iv 4.
    pub q: Vec<i64>,
    pub stable: bool,
    pub grains: i64,
}

fn ints(name: &str, v: &[i64]) -> String {
    matrix(name, SIDE, SIDE, v.iter().map(|x| x.to_string()))
}

/// The program: the grid's side, the core, the pile, `rounds` rounds.
pub fn program(p: &Pile, rounds: usize) -> String {
    format!(
        "side := {SIDE}\n{}{}{}s := {rounds} 'u:s_tep p_ower (u:p_lane h0) c_at u:p_lane c0\nh := 1 s_elect s\n\
         r_avel h\nr_avel 2 s_elect s\nr_avel h d_iv 4\nu:s_table h\n'+ r_/_12 h\n",
        core(),
        ints("h0", &p.h),
        ints("c0", &p.topples)
    )
}

/// Run `rounds` rounds through X_eTaL.
pub fn run(p: &Pile, rounds: usize) -> Result<Rounds, String> {
    let out = output(&program(p, rounds), 5)?;
    let n = SIDE * SIDE;
    Ok(Rounds {
        next: Pile { h: numbers(&out[0], n)?, topples: numbers(&out[1], n)? },
        q: numbers(&out[2], n)?,
        stable: numbers::<i64>(&out[3], 1)?[0] == 1,
        grains: numbers::<i64>(&out[4], 1)?[0],
    })
}

/// One round by a direct loop (for the tests).
pub fn direct_round(p: &Pile) -> Pile {
    let s = SIDE;
    let q: Vec<i64> = p.h.iter().map(|v| v / 4).collect();
    let mut h = vec![0; s * s];
    let mut t = p.topples.clone();
    for r in 0..s {
        for c in 0..s {
            let i = r * s + c;
            t[i] += q[i];
            if Pile::on_edge(r, c) {
                continue;
            }
            h[i] = p.h[i] - 4 * q[i] + q[i - 1] + q[i + 1] + q[i - s] + q[i + s];
        }
    }
    Pile { h, topples: t }
}
