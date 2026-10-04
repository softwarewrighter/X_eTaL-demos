//! The model: the demo's own X_eTaL program (image-pipeline.xtl): its
//! coordinates and its pipeline (windows, filter, Sobel edges,
//! threshold, max-pooling), run on a picture the page picks, with the
//! kernels and threshold the page sets. Nothing here knows about the
//! browser.

use microscope::run::{lit, numbers, output, section};

/// The command-line program; the page runs its sections.
pub const SOURCE: &str = include_str!("../../image-pipeline.xtl");

/// Each pixel's row and column.
pub fn prelude() -> &'static str {
    section(SOURCE, "# -- coordinates", "# -- the picture")
}

/// The windows, the filter, the pipeline and the pooling.
pub fn core() -> &'static str {
    section(SOURCE, "# -- the windows", "# -- end of the core")
}

/// The picture is SIZE x SIZE pixels; pooling halves it.
pub const SIZE: usize = 96;
pub const HALF: usize = SIZE / 2;

/// The pictures: a name and the X_eTaL that defines `img` (brightness
/// 0 to 1) from `row`, `col`, `rows` and `cols`.
pub const SCENES: &[(&str, &str)] = &[
    ("Shapes", "disk := (((row - 30) * row - 30) + (col - 26) * col - 26) < 300\nsq := ((row >= 14) & row < 44) & (col >= 52) & col < 84\ntri := ((row >= 56) & row < 88) & (a_bs col - 48) <= row - 56\nimg := 0.15 + (0.65 * f_loat disk) + (0.45 * f_loat sq) + (0.3 * f_loat tri) + 0.002 * f_loat r_oll! (s_hape row) r_eshape 50\n"),
    ("Quadrants and a diagonal", "dr := row - rows d_iv 2\ndc := col - cols d_iv 2\nimg := 0.15 + (0.35 * f_loat (dr * dc) > 0) + 0.4 * f_loat dr > dc\n"),
    ("Rings", "dr := row - rows d_iv 2\ndc := col - cols d_iv 2\nr := (f_loat (dr * dr) + dc * dc) ^ 0.5\nimg := 0.5 + 0.35 * s_in r / 2.5\n"),
    ("Noisy checkerboard", "check := 1 = ((row d_iv 16) + col d_iv 16) m_od 2\nimg := 0.2 + (0.5 * f_loat check) + 0.003 * f_loat r_oll! (s_hape row) r_eshape 100\n"),
    ("Half and half", "img := 0.2 + 0.6 * f_loat col >= cols d_iv 2\n"),
];

/// A 3 x 3 kernel, row by row.
pub type Kernel = [f64; 9];

pub const BLURS: &[(&str, Kernel)] = &[
    ("Gaussian", [0.0625, 0.125, 0.0625, 0.125, 0.25, 0.125, 0.0625, 0.125, 0.0625]),
    ("Box", [1.0 / 9.0; 9]),
    ("None (identity)", [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]),
    ("Sharpen", [0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0]),
];

pub const EDGES: &[(&str, Kernel)] = &[
    ("Sobel", [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0]),
    ("Prewitt", [-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0]),
    ("Central difference", [0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0]),
];

/// `k` turned a quarter (its transpose), as `o_\ kx` gives `ky`: for
/// showing ky and for the tests.
pub fn turn(k: &Kernel) -> Kernel {
    let mut t = [0.0; 9];
    for a in 0..3 {
        for b in 0..3 {
            t[3 * a + b] = k[3 * b + a];
        }
    }
    t
}

/// What the page sets: the picture, the kernels, the threshold, and
/// the pixel whose windows are shown.
#[derive(Clone, Debug, PartialEq)]
pub struct Setup {
    pub scene: String,
    pub blur: Kernel,
    pub kx: Kernel,
    pub thresh: f64,
    pub pixel: (usize, usize),
}

impl Setup {
    pub fn new(scene: usize) -> Self {
        Setup { scene: SCENES[scene].1.to_string(), blur: BLURS[0].1, kx: EDGES[0].1, thresh: 0.8, pixel: (30, 26 + 17) }
    }
}

/// Every stage of the pipeline, as X_eTaL computed them.
#[derive(Clone, Debug, PartialEq)]
pub struct Anatomy {
    pub img: Vec<f64>,
    pub smooth: Vec<f64>,
    pub gx: Vec<f64>,
    pub gy: Vec<f64>,
    pub mag: Vec<f64>,
    pub edges: Vec<f64>,
    /// HALF x HALF.
    pub pool: Vec<f64>,
    /// The windows of `img` and of `smooth` at the chosen pixel, 3 x 3.
    pub win_img: Kernel,
    pub win_smooth: Kernel,
}

fn kernel(name: &str, k: &Kernel) -> String {
    let items: Vec<String> = k.iter().map(|&x| lit(x)).collect();
    format!("{name} := (3 c_at 3) r_eshape {}\n", items.join(" "))
}

/// The page's settings: the kernels and the threshold, as X_eTaL.
pub fn settings(s: &Setup) -> String {
    format!("{}{}ky := o_\\ kx\nthresh := {}\n", kernel("blur", &s.blur), kernel("kx", &s.kx), lit(s.thresh))
}

/// The program: the size, coordinates, picture, settings and core,
/// then every stage printed.
pub fn program(s: &Setup) -> String {
    let (y, x) = (s.pixel.0.min(SIZE - 1) + 1, s.pixel.1.min(SIZE - 1) + 1);
    format!(
        "rows := {SIZE}\ncols := {SIZE}\n{}{}{}{}\
         r_avel img\nr_avel smooth\nr_avel gx\nr_avel gy\nr_avel mag\nr_avel edges\nr_avel pool\n\
         r_avel {y} s_elect_3 {x} s_elect_4 u:w_indows img\nr_avel {y} s_elect_3 {x} s_elect_4 u:w_indows smooth\n",
        prelude(),
        s.scene,
        settings(s),
        core()
    )
}

fn nine(v: Vec<f64>) -> Kernel {
    let mut k = [0.0; 9];
    k.copy_from_slice(&v);
    k
}

/// Run the pipeline through X_eTaL.
pub fn run(s: &Setup) -> Result<Anatomy, String> {
    let out = output(&program(s), 9)?;
    let n = SIZE * SIZE;
    let a = |i: usize| numbers::<f64>(&out[i], n);
    Ok(Anatomy {
        img: a(0)?,
        smooth: a(1)?,
        gx: a(2)?,
        gy: a(3)?,
        mag: a(4)?,
        edges: a(5)?,
        pool: numbers(&out[6], HALF * HALF)?,
        win_img: nine(numbers(&out[7], 9)?),
        win_smooth: nine(numbers(&out[8], 9)?),
    })
}

/// A direct 3 x 3 correlation, wrapping at the edges as rotation does:
/// out[r][c] = sum over a, b of k[a][b] * x[r + a - 1][c + b - 1].
pub fn direct(k: &Kernel, x: &[f64], rows: usize, cols: usize) -> Vec<f64> {
    let mut out = vec![0.0; rows * cols];
    for r in 0..rows {
        for c in 0..cols {
            let mut s = 0.0;
            for a in 0..3 {
                for b in 0..3 {
                    let rr = (r + rows + a - 1) % rows;
                    let cc = (c + cols + b - 1) % cols;
                    s += k[3 * a + b] * x[rr * cols + cc];
                }
            }
            out[r * cols + c] = s;
        }
    }
    out
}
