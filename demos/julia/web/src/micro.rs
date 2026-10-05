//! The model: the demo's own X_eTaL program (julia.xtl), whose one
//! function `c u:i_terate z0` gives both a Julia set (c one number, z0
//! the grid) and the Mandelbrot set (c the grid, z0 = 0).

use microscope::run::{lit, numbers, output, section};

/// The command-line program; the page runs its core with its own view.
pub const SOURCE: &str = include_str!("../../julia.xtl");

const CORE_START: &str = "# Numbers centered on 0";
const CORE_END: &str = "# -- end of the core";

pub fn core() -> &'static str {
    section(SOURCE, CORE_START, CORE_END)
}

/// Where a picture looks: its size in pixels, its center and width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub rows: usize,
    pub cols: usize,
    pub cx: f64,
    pub cy: f64,
    pub w: f64,
}

impl View {
    pub const JULIA: View = View { rows: 90, cols: 135, cx: 0.0, cy: 0.0, w: 3.4 };
    pub const PICKER: View = View { rows: 48, cols: 72, cx: -0.6, cy: 0.0, w: 3.0 };

    /// The point at pixel (y, x), as the program places it.
    pub fn point(&self, y: usize, x: usize) -> (f64, f64) {
        let centered = |i: usize, n: usize| i as f64 - (n as f64 - 1.0) / 2.0;
        let d = self.w / self.cols as f64;
        (self.cx + d * centered(x, self.cols), self.cy - d * centered(y, self.rows))
    }

    /// The pixel nearest the point (a, b), if it is in view.
    pub fn pixel(&self, (a, b): (f64, f64)) -> Option<(usize, usize)> {
        let d = self.w / self.cols as f64;
        let x = ((a - self.cx) / d + (self.cols as f64 - 1.0) / 2.0).round();
        let y = ((self.cy - b) / d + (self.rows as f64 - 1.0) / 2.0).round();
        let inside = (0.0..self.cols as f64).contains(&x) && (0.0..self.rows as f64).contains(&y);
        inside.then_some((y as usize, x as usize))
    }
}

/// Which set: a Julia set for c, or the Mandelbrot set.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Set {
    Julia(f64, f64),
    Mandelbrot,
}

/// The line that runs `set`, as the page writes it.
pub fn call(set: Set) -> String {
    match set {
        Set::Julia(a, b) => format!("z := ({} c_at {}) u:i_terate grid", lit(a), lit(b)),
        Set::Mandelbrot => "z := grid u:i_terate 0.0 * grid".into(),
    }
}

/// The program for `set` on view `v`, `k` steps: prints the counts.
pub fn program(v: &View, k: usize, set: Set) -> String {
    format!(
        "rows := {}\ncols := {}\ncx := {}\ncy := {}\nw := {}\nk := {k}\naspect := 1.0\n{}{}\nr_avel 3 s_elect z\n",
        v.rows, v.cols, lit(v.cx), lit(v.cy), lit(v.w), core(), call(set)
    )
}

/// How many steps each point of `v` stays inside, for `set`.
pub fn counts(v: &View, k: usize, set: Set) -> Result<Vec<f64>, String> {
    let out = output(&program(v, k, set), 1)?;
    numbers(&out[0], v.rows * v.cols)
}

/// Named values of c with well-known Julia sets.
pub const PRESETS: &[(&str, f64, f64)] = &[
    ("Dendrite-like spirals", -0.8, 0.156),
    ("Douady rabbit", -0.123, 0.745),
    ("San Marco", -0.75, 0.0),
    ("Siegel disk", -0.391, -0.587),
    ("Dust (outside the set)", 0.285, 0.535),
    ("Dragon", 0.36, 0.1),
];
