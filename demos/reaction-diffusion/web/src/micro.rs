//! The model: the demo's own X_eTaL program (reaction-diffusion.xtl),
//! stepped on a grid the page keeps, and the arrays of the last step
//! read back. Nothing here knows about the browser.

/// The command-line program; the page runs its core on its own grid.
pub const SOURCE: &str = include_str!("../../reaction-diffusion.xtl");

const CORE_START: &str = "# A matrix as one plane";
const CORE_END: &str = "# -- end of the core";

/// The core: the Laplacian and the step.
pub fn core() -> &'static str {
    let a = SOURCE.find(CORE_START).unwrap_or(0);
    let b = SOURCE.find(CORE_END).unwrap_or(SOURCE.len());
    &SOURCE[a..b]
}

/// `x` as an X_eTaL Float literal: plain decimal (X_eTaL literals have
/// no exponent), 17 significant digits; sizes under 1e-15 are written 0.
pub fn lit(x: f64) -> String {
    if x.abs() < 1e-15 || !x.is_finite() {
        return "0.0".into();
    }
    let digits = (16 - x.abs().log10().floor() as i64).clamp(1, 32) as usize;
    let s = format!("{x:.digits$}");
    let s = s.trim_end_matches('0');
    if s.ends_with('.') { format!("{s}0") } else { s.to_string() }
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

    /// Add a square of V (and take U down) centred on (y, x).
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

    /// The starting grid: a few squares of V, placed off-centre so the
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
    /// `u:l_ap u`, `u:l_ap v`: four shifts minus four times the centre.
    pub lap_u: Vec<f64>,
    pub lap_v: Vec<f64>,
    /// `u * v * v`: the reaction.
    pub uvv: Vec<f64>,
    /// The grid after the last step.
    pub next: Grid,
}

fn matrix(name: &str, n: usize, xs: &[f64]) -> String {
    let body: Vec<String> = xs.iter().map(|&x| lit(x)).collect();
    format!("{name} := ({n} c_at {n}) r_eshape {}\n", body.join(" "))
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
    format!("{params}{}{}{}{run}", core(), matrix("u0", g.n, &g.u), matrix("v0", g.n, &g.v))
}

fn floats(line: &str, want: usize) -> Result<Vec<f64>, String> {
    let v: Result<Vec<f64>, _> = line.split_whitespace().map(str::parse).collect();
    let v = v.map_err(|e| format!("unexpected output {:.60}: {e}", line))?;
    match v.len() == want {
        true => Ok(v),
        false => Err(format!("expected {want} numbers, got {}", v.len())),
    }
}

/// Run `steps` steps of `g` through X_eTaL.
pub fn run(g: &Grid, r: &Rates, steps: usize) -> Result<Anatomy, String> {
    let out = xetal_play::run(&program(g, r, steps), 1);
    if !out.err.is_empty() {
        return Err(out.err);
    }
    let lines: Vec<&str> = out.out.lines().collect();
    if lines.len() != 7 {
        return Err(format!("expected 7 lines of output, got {}", lines.len()));
    }
    let nn = g.n * g.n;
    let a = |i: usize| floats(lines[i], nn);
    Ok(Anatomy {
        u: a(0)?,
        v: a(1)?,
        lap_u: a(2)?,
        lap_v: a(3)?,
        uvv: a(4)?,
        next: Grid { n: g.n, u: a(5)?, v: a(6)? },
    })
}
