//! The model: the demo's own X_eTaL program (mandelbrot.xtl), run on a
//! view, and the arrays it prints read back. Nothing here knows about
//! the browser.

/// The command-line program; the page runs its core with its own view.
pub const SOURCE: &str = include_str!("../../mandelbrot.xtl");

const CORE_START: &str = "# Numbers centred on 0";
const CORE_END: &str = "# -- end of the core";
const ORBIT_START: &str = "u:o_rbit :=";

/// The core of the program: helpers, c by broadcasting, the step, k
/// steps, |z|^2 and the counts.
pub fn core() -> &'static str {
    let a = SOURCE.find(CORE_START).unwrap_or(0);
    let b = SOURCE.find(CORE_END).unwrap_or(SOURCE.len());
    &SOURCE[a..b]
}

/// The definition of one step of a single point's orbit.
pub fn orbit_def() -> &'static str {
    let a = SOURCE.find(ORBIT_START).unwrap_or(0);
    let b = SOURCE[a..].find('\n').map_or(SOURCE.len(), |i| a + i);
    &SOURCE[a..b]
}

/// Where the picture looks: its size in pixels, its centre and width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub rows: usize,
    pub cols: usize,
    pub cx: f64,
    pub cy: f64,
    pub w: f64,
}

impl View {
    pub const HOME: View = View { rows: 90, cols: 135, cx: -0.6, cy: 0.0, w: 3.0 };

    /// The point c at pixel (y, x), as the program places it.
    pub fn point(&self, y: usize, x: usize) -> (f64, f64) {
        let centred = |i: usize, n: usize| i as f64 - (n as f64 - 1.0) / 2.0;
        let d = self.w / self.cols as f64;
        (self.cx + d * centred(x, self.cols), self.cy - d * centred(y, self.rows))
    }

    /// The view zoomed by `f` (2.0 halves the width) around pixel (y, x).
    pub fn zoom(&self, y: usize, x: usize, f: f64) -> View {
        let (cx, cy) = self.point(y, x);
        View { cx, cy, w: self.w / f, ..*self }
    }
}

/// Every array of one run, as X_eTaL computed them (row by row).
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub k: usize,
    /// `re`, one per column, and `im`, one per row: what t_able spreads.
    pub re: Vec<f64>,
    pub im: Vec<f64>,
    /// `cr`, `ci`: c's real and imaginary parts at every point.
    pub cr: Vec<f64>,
    pub ci: Vec<f64>,
    /// `m2`: |z|^2 after k steps (a point that left keeps its z).
    pub m2: Vec<f64>,
    /// `4 >= m2`: still within 2 of the origin after k steps.
    pub inside: Vec<f64>,
    /// `counts`: the steps each point spent inside, 0..k.
    pub counts: Vec<f64>,
}

fn params(v: &View, k: usize) -> String {
    format!(
        "rows := {}\ncols := {}\ncx := {:?}\ncy := {:?}\nw := {:?}\nk := {k}\naspect := 1.0\n",
        v.rows, v.cols, v.cx, v.cy, v.w
    )
}

/// The program the page runs for `v` and `k` steps.
pub fn program(v: &View, k: usize) -> String {
    let prints = "r_avel re\nr_avel im\nr_avel cr\nr_avel ci\nr_avel m2\n\
                  r_avel f_loat 4 >= m2\nr_avel counts\n";
    format!("{}{}{prints}", params(v, k), core())
}

fn floats(line: &str, want: usize) -> Result<Vec<f64>, String> {
    let v: Result<Vec<f64>, _> = line.split_whitespace().map(str::parse).collect();
    let v = v.map_err(|e| format!("unexpected output {:.60}: {e}", line))?;
    match v.len() == want {
        true => Ok(v),
        false => Err(format!("expected {want} numbers, got {}", v.len())),
    }
}

fn output(src: &str, lines: usize) -> Result<Vec<String>, String> {
    let run = xetal_play::run(src, 1);
    if !run.err.is_empty() {
        return Err(run.err);
    }
    let out: Vec<String> = run.out.lines().map(str::to_string).collect();
    match out.len() == lines {
        true => Ok(out),
        false => Err(format!("expected {lines} lines of output, got {}", out.len())),
    }
}

/// Run `k` steps of the view `v` through X_eTaL.
pub fn run(v: &View, k: usize) -> Result<Frame, String> {
    let out = output(&program(v, k), 7)?;
    let n = v.rows * v.cols;
    Ok(Frame {
        k,
        re: floats(&out[0], v.cols)?,
        im: floats(&out[1], v.rows)?,
        cr: floats(&out[2], n)?,
        ci: floats(&out[3], n)?,
        m2: floats(&out[4], n)?,
        inside: floats(&out[5], n)?,
        counts: floats(&out[6], n)?,
    })
}

/// The program for the orbit z0 .. zn of the point c = (cr, ci).
pub fn orbit_program(c: (f64, f64), n: usize) -> String {
    let each = |part: u8| {
        format!("'{{ n -> {part} s_elect n 'u:o_rbit p_ower 0.0 0.0 }} e_ach o_ffsets {}\n", n + 1)
    };
    format!("cr0 := {:?}\nci0 := {:?}\n{}\n{}{}", c.0, c.1, orbit_def(), each(1), each(2))
}

/// The orbit z0 .. zn of c, as X_eTaL computes it (it may overflow to
/// infinity after the point escapes).
pub fn orbit(c: (f64, f64), n: usize) -> Result<Vec<(f64, f64)>, String> {
    let out = output(&orbit_program(c, n), 2)?;
    let re = floats(&out[0], n + 1)?;
    let im = floats(&out[1], n + 1)?;
    Ok(re.into_iter().zip(im).collect())
}

/// The first step at which the orbit is outside the disk of radius 2.
pub fn escape(orbit: &[(f64, f64)]) -> Option<usize> {
    orbit.iter().position(|(a, b)| !(a * a + b * b <= 4.0))
}
