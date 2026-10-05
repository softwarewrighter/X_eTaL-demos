//! The model: the demo's own X_eTaL program (reaction-diffusion.xtl),
//! stepped on a grid the page keeps, and the arrays of the last step
//! read back. Nothing here knows about the browser.

/// The command-line program; the page runs its core on its own grid.
pub const SOURCE: &str = include_str!("../../reaction-diffusion.xtl");

const CORE_START: &str = "# A matrix as one plane";
const CORE_END: &str = "# -- end of the core";

/// The core: the Laplacian and the step.
pub fn core() -> &'static str {
    section(SOURCE, CORE_START, CORE_END)
}

use microscope::run::{lit_or_zero, matrix, numbers, output, section};

/// `x` as an X_eTaL Float literal; sizes under 1e-15 are written 0
/// (V decays towards 0, and the grid goes into every run as text).
pub fn lit(x: f64) -> String {
    lit_or_zero(x, 1e-15)
}

/// The rates: how fast U and V spread, the feed f and the kill k.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rates {
    pub du: f64,
    pub dv: f64,
    pub f: f64,
    pub k: f64,
}

/// Named feed and kill rates, with du = 0.2, dv = 0.1.
pub const PRESETS: &[(&str, f64, f64)] = &[
    ("Maze", 0.029, 0.057),
    ("Coral", 0.0545, 0.062),
    ("Ripples", 0.022, 0.051),
    ("Spots", 0.035, 0.065),
    ("Worms", 0.046, 0.063),
];

pub fn preset(i: usize) -> Rates {
    let (_, f, k) = PRESETS[i % PRESETS.len()];
    Rates { du: 0.2, dv: 0.1, f, k }
}

/// The two chemicals on an n x n grid, row by row.
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub n: usize,
    pub u: Vec<f64>,
    pub v: Vec<f64>,
}

impl Grid {
    /// U everywhere, no V.
    pub fn empty(n: usize) -> Self {
        Grid { n, u: vec![1.0; n * n], v: vec![0.0; n * n] }
    }

    /// Add a square of V (and take U down) centered on (y, x).
    pub fn drop(&mut self, y: usize, x: usize, r: usize) {
        let n = self.n;
        for dy in 0..2 * r {
            for dx in 0..2 * r {
                let i = ((y + n + dy - r) % n) * n + (x + n + dx - r) % n;
                self.u[i] = 0.5;
                self.v[i] = 0.25;
            }
        }
    }

    /// The starting grid: a few squares of V, placed off-center so the
    /// pattern is not symmetric.
    pub fn seeded(n: usize) -> Self {
        let mut g = Grid::empty(n);
        for (fy, fx) in [(0.3, 0.35), (0.62, 0.7), (0.7, 0.25)] {
            g.drop((fy * n as f64) as usize, (fx * n as f64) as usize, (n / 16).max(2));
        }
        g
    }
}

/// Every array of the last step of a run, as X_eTaL computed them.
#[derive(Clone, Debug, PartialEq)]
pub struct Anatomy {
    /// U and V before the last step.
    pub u: Vec<f64>,
    pub v: Vec<f64>,
    /// `u:l_ap u`, `u:l_ap v`: four shifts minus four times the center.
    pub lap_u: Vec<f64>,
    pub lap_v: Vec<f64>,
    /// `u * v * v`: the reaction.
    pub uvv: Vec<f64>,
    /// The grid after the last step.
    pub next: Grid,
}

/// The program that runs `steps` (at least 1) steps from `g`, printing
/// the arrays of the last one.
pub fn program(g: &Grid, r: &Rates, steps: usize) -> String {
    let params = format!(
        "n := {}\ndu := {}\ndv := {}\nf := {}\nk := {}\n",
        g.n, lit(r.du), lit(r.dv), lit(r.f), lit(r.k)
    );
    let run = format!(
        "s0 := (u:p_lane u0) c_at u:p_lane v0\ns := {} 'u:s_tep p_ower s0\n\
         u := 1 s_elect s\nv := 2 s_elect s\ns1 := u:s_tep s\n\
         r_avel u\nr_avel v\nr_avel u:l_ap u\nr_avel u:l_ap v\nr_avel u * v * v\n\
         r_avel 1 s_elect s1\nr_avel 2 s_elect s1\n",
        steps.max(1) - 1
    );
    let m = |name: &str, xs: &[f64]| matrix(name, g.n, g.n, xs.iter().map(|&x| lit(x)));
    format!("{params}{}{}{}{run}", core(), m("u0", &g.u), m("v0", &g.v))
}

fn floats(line: &str, want: usize) -> Result<Vec<f64>, String> {
    numbers(line, want)
}

/// Run `steps` steps of `g` through X_eTaL.
pub fn run(g: &Grid, r: &Rates, steps: usize) -> Result<Anatomy, String> {
    let lines = output(&program(g, r, steps), 7)?;
    let nn = g.n * g.n;
    let a = |i: usize| floats(&lines[i], nn);
    Ok(Anatomy {
        u: a(0)?,
        v: a(1)?,
        lap_u: a(2)?,
        lap_v: a(3)?,
        uvv: a(4)?,
        next: Grid { n: g.n, u: a(5)?, v: a(6)? },
    })
}
