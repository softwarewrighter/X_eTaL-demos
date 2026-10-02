//! The model: the demo's own X_eTaL program (wave-tank.xtl): its
//! coordinates and sponge, and its step, run with a scene the page
//! picks, on a surface the page keeps. Nothing here knows about the
//! browser.

use microscope::run::{lit_or_zero, matrix, numbers, output, section};

/// The command-line program; the page runs its sections.
pub const SOURCE: &str = include_str!("../../wave-tank.xtl");

/// `u:p_lane`, coordinates and the sponge (`row`, `col`, `edge`, `damp`).
pub fn prelude() -> &'static str {
    section(SOURCE, "# A matrix as one plane", "# -- the scene")
}

/// The Laplacian and the step.
pub fn core() -> &'static str {
    section(SOURCE, "# -- the Laplacian", "# -- end of the core")
}

/// `x` as an X_eTaL literal; sizes under 1e-12 are written 0.
pub fn lit(x: f64) -> String {
    lit_or_zero(x, 1e-12)
}

pub const ROWS: usize = 60;
pub const COLS: usize = 96;

/// The scenes: a name and the X_eTaL that defines `c2` (the wave speed
/// squared, a number or a map), `wall` (1 water, 0 wall) and `src` (the
/// cells the source shakes), from `row`, `col`, `rows` and `cols`.
pub const SCENES: &[(&str, &str)] = &[
    ("Double slit", "c2 := 0.3\nmid := rows d_iv 2\nslits := ((row >= mid - 8) & row < mid - 5) | (row >= mid + 5) & row < mid + 8\nwall := f_loat n_ot (col = 36) & n_ot slits\nsrc := f_loat col = 12\n"),
    ("Single slit", "c2 := 0.3\nmid := rows d_iv 2\nslit := (row >= mid - 3) & row < mid + 3\nwall := f_loat n_ot (col = 36) & n_ot slit\nsrc := f_loat col = 12\n"),
    ("Two sources", "c2 := 0.3\nmid := rows d_iv 2\nwall := f_loat row >= 0\nsrc := f_loat ((row = mid - 9) | row = mid + 9) & col = 28\n"),
    ("Lens", "mid := rows d_iv 2\nr2 := ((row - mid) * row - mid) + (col - 44) * col - 44\nc2 := 0.32 - 0.16 * f_loat r2 < 225\nwall := f_loat row >= 0\nsrc := f_loat col = 12\n"),
    ("Open tank (click for ripples)", "c2 := 0.3\nwall := f_loat row >= 0\nsrc := 0.0 * wall\n"),
];

/// The surface now and a step ago, and the time.
#[derive(Clone, Debug, PartialEq)]
pub struct Surface {
    pub u: Vec<f64>,
    pub p: Vec<f64>,
    pub t: f64,
}

impl Surface {
    pub fn flat() -> Self {
        Surface { u: vec![0.0; ROWS * COLS], p: vec![0.0; ROWS * COLS], t: 0.0 }
    }

    /// A drop at (y, x): a smooth bump, still (the same a step ago).
    pub fn drop(&mut self, y: usize, x: usize) {
        for dy in -4i64..=4 {
            for dx in -4i64..=4 {
                let (yy, xx) = (y as i64 + dy, x as i64 + dx);
                if (0..ROWS as i64).contains(&yy) && (0..COLS as i64).contains(&xx) {
                    let h = 1.2 * (-((dy * dy + dx * dx) as f64) / 4.0).exp();
                    let i = yy as usize * COLS + xx as usize;
                    self.u[i] += h;
                    self.p[i] += h;
                }
            }
        }
    }
}

/// Every array of the last step of a run, as X_eTaL computed them.
#[derive(Clone, Debug, PartialEq)]
pub struct Anatomy {
    /// The surface before the last step, and a step before that.
    pub u: Vec<f64>,
    pub p: Vec<f64>,
    /// `u:l_ap u`.
    pub lap: Vec<f64>,
    /// The scene: wave speed squared, walls, source, sponge.
    pub c2: Vec<f64>,
    pub wall: Vec<f64>,
    pub src: Vec<f64>,
    pub damp: Vec<f64>,
    /// The surface after the last step, ready for the next run.
    pub next: Surface,
}

/// The program: scene `scene`, `steps` (at least 1) steps from `s`,
/// printing the arrays of the last one.
pub fn program(s: &Surface, scene: usize, steps: usize) -> String {
    let params = format!("rows := {ROWS}\ncols := {COLS}\namp := 0.5\nomega := 0.6\n");
    let m = |name: &str, xs: &[f64]| matrix(name, ROWS, COLS, xs.iter().map(|&x| lit(x)));
    let run = format!(
        "s0 := (u:p_lane u0) c_at (u:p_lane p0) c_at u:p_lane {} + 0.0 * u0\n\
         s := {} 'u:s_tep p_ower s0\ns1 := u:s_tep s\nu := 1 s_elect s\n\
         r_avel u\nr_avel 2 s_elect s\nr_avel u:l_ap u\nr_avel c2 + 0.0 * wall\nr_avel wall\n\
         r_avel src\nr_avel damp\nr_avel 1 s_elect s1\nr_avel 2 s_elect s1\nf_irst r_avel 3 s_elect s1\n",
        lit(s.t),
        steps.max(1) - 1
    );
    let scene = SCENES[scene % SCENES.len()].1;
    format!("{params}{}{scene}{}{}{}{run}", prelude(), core(), m("u0", &s.u), m("p0", &s.p))
}

/// Run `steps` steps of scene `scene` from `s` through X_eTaL.
pub fn run(s: &Surface, scene: usize, steps: usize) -> Result<Anatomy, String> {
    let out = output(&program(s, scene, steps), 10)?;
    let n = ROWS * COLS;
    let a = |i: usize| numbers::<f64>(&out[i], n);
    let t = numbers::<f64>(&out[9], 1)?[0];
    Ok(Anatomy {
        u: a(0)?,
        p: a(1)?,
        lap: a(2)?,
        c2: a(3)?,
        wall: a(4)?,
        src: a(5)?,
        damp: a(6)?,
        next: Surface { u: a(7)?, p: a(8)?, t },
    })
}
