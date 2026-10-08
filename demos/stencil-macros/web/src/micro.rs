//! The model: the demo's own X_eTaL program (stencil-macros.xtl) and
//! macro library (Stencil.xtlm). The page writes the kernel it keeps
//! into a call of `s:s_tencil<`; X_eTaL expands the call (the library
//! is read from the store the page installs) and runs the program on
//! the demo's picture. Nothing here knows about the browser.

use std::sync::{Arc, Once};

use microscope::run::{numbers, output, section};
use xetal_macro::StoreLibraries;

/// The command-line program; the page runs its picture.
pub const SOURCE: &str = include_str!("../../stencil-macros.xtl");
/// The macro library, as the command line finds it beside the program.
pub const LIBRARY: &str = include_str!("../../Stencil.xtlm");
/// The name the program imports it by (`"s:" u_se< "Stencil"`).
pub const LIBRARY_FILE: &str = "Stencil.xtlm";

pub const ROWS: usize = 24;
pub const COLS: usize = 48;

/// The library where X_eTaL looks for it: in the store a run's
/// libraries are read from (in the browser, memory; natively the disk
/// until this installs memory too).
pub fn install() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        xetal_store::install(Arc::new(xetal_store::Memory::default()));
        let _ = xetal_store::write(LIBRARY_FILE, LIBRARY);
    });
}

/// The picture: its size, coordinates, masks and noise.
pub fn picture() -> &'static str {
    section(SOURCE, "rows := ", "# -- the stencils")
}

/// A kernel: its side (3 or 5), its numbers row by row, what the sum
/// is divided by, and how many times it is applied.
#[derive(Clone, Debug, PartialEq)]
pub struct Kernel {
    pub side: usize,
    pub w: Vec<f64>,
    pub divide: f64,
    pub steps: usize,
}

/// A preset: its name, side, numbers, divisor and steps.
pub struct Preset {
    pub name: &'static str,
    pub side: usize,
    pub w: &'static [f64],
    pub divide: f64,
    pub steps: usize,
}

pub const PRESETS: &[Preset] = &[
    Preset { name: "blur", side: 3, w: &[1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0], divide: 16.0, steps: 1 },
    Preset { name: "edges", side: 3, w: &[-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0], divide: 1.0, steps: 1 },
    Preset { name: "sharpen", side: 3, w: &[0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0], divide: 1.0, steps: 1 },
    Preset { name: "emboss", side: 3, w: &[-2.0, -1.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 2.0], divide: 1.0, steps: 1 },
    Preset { name: "Sobel across", side: 3, w: &[-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0], divide: 1.0, steps: 1 },
    Preset { name: "heat", side: 3, w: &[0.0, 0.2, 0.0, 0.2, 0.2, 0.2, 0.0, 0.2, 0.0], divide: 1.0, steps: 30 },
    Preset { name: "move right", side: 3, w: &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0], divide: 1.0, steps: 1 },
    Preset { name: "identity", side: 3, w: &[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0], divide: 1.0, steps: 1 },
    Preset {
        name: "blur 5 x 5",
        side: 5,
        w: &[
            1.0, 4.0, 6.0, 4.0, 1.0, 4.0, 16.0, 24.0, 16.0, 4.0, 6.0, 24.0, 36.0, 24.0, 6.0, 4.0, 16.0, 24.0, 16.0, 4.0, 1.0,
            4.0, 6.0, 4.0, 1.0,
        ],
        divide: 256.0,
        steps: 1,
    },
    Preset {
        name: "motion blur 5 x 5",
        side: 5,
        w: &[
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0,
        ],
        divide: 5.0,
        steps: 1,
    },
];

impl Kernel {
    pub fn preset(i: usize) -> Kernel {
        let p = &PRESETS[i];
        Kernel { side: p.side, w: p.w.to_vec(), divide: p.divide, steps: p.steps }
    }

    /// The same numbers on another side: the center kept, the rest
    /// cropped or padded with 0.
    pub fn resized(&self, side: usize) -> Kernel {
        let (a, b) = (self.side as i64 / 2, side as i64 / 2);
        let mut w = vec![0.0; side * side];
        for r in 0..side as i64 {
            for c in 0..side as i64 {
                let (sr, sc) = (r - b + a, c - b + a);
                if (0..self.side as i64).contains(&sr) && (0..self.side as i64).contains(&sc) {
                    w[(r * side as i64 + c) as usize] = self.w[(sr * self.side as i64 + sc) as usize];
                }
            }
        }
        Kernel { side, w, ..self.clone() }
    }

    /// The kernel as the macro reads it: rows apart by two spaces.
    pub fn text(&self) -> String {
        let row = |r: usize| (0..self.side).map(|c| num(self.w[r * self.side + c])).collect::<Vec<_>>().join(" ");
        (0..self.side).map(row).collect::<Vec<_>>().join("  ")
    }

    /// Terms the macro writes (one per number that is not 0), how many
    /// of those are a 1 (no multiply) or a -1 (a negation), and how
    /// many numbers it skips.
    pub fn counts(&self) -> (usize, usize, usize, usize) {
        let n = |f: &dyn Fn(f64) -> bool| self.w.iter().filter(|&&x| f(x)).count();
        (n(&|x| x != 0.0), n(&|x| x == 1.0), n(&|x| x == -1.0), n(&|x| x == 0.0))
    }

    /// Whether the numbers sum to 0 (an edge finder: the result is signed).
    pub fn balanced(&self) -> bool {
        self.w.iter().sum::<f64>().abs() < 1e-9
    }
}

/// A number as written: 2 for 2.0, 0.2 as it is.
pub fn num(x: f64) -> String {
    match x == x.trunc() && x.abs() < 1e15 {
        true => format!("{}", x as i64),
        false => format!("{x:?}"),
    }
}

/// The macro call the page writes (the line the expansion replaces).
pub fn call(k: &Kernel) -> String {
    let c = format!("\"{}\" s:s_tencil< \"p\"", k.text());
    match k.divide == 1.0 {
        true => format!("u:s_tep := {{ p -> {c} }}"),
        false => format!("u:s_tep := {{ p -> ({c}) / {} }}", num(k.divide)),
    }
}

/// The program: the import, the picture, the step with the kernel's
/// call, the steps applied, and the lines that print.
pub fn program(k: &Kernel) -> String {
    format!(
        "\"s:\" u_se< \"Stencil\"\n\n{}{}\nout := {} 'u:s_tep p_ower img\nr_avel img\nr_avel out\n",
        picture(),
        call(k),
        k.steps
    )
}

/// What X_eTaL computed: the picture and the kernel applied.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub img: Vec<f64>,
    pub out: Vec<f64>,
}

/// Run the program through X_eTaL.
pub fn run(k: &Kernel) -> Result<Run, String> {
    install();
    let out = output(&program(k), 2)?;
    let n = ROWS * COLS;
    Ok(Run { img: numbers(&out[0], n)?, out: numbers(&out[1], n)? })
}

/// The program after X_eTaL expanded its macro calls (what
/// `xetal expand` prints).
pub fn expanded(k: &Kernel) -> Result<String, String> {
    install();
    xetal_program::expanded_with("stencil-macros.xtl", &program(k), &StoreLibraries).map_err(|d| d.to_string())
}

/// The step's line after expansion: the code the kernel became.
pub fn expanded_step(k: &Kernel) -> Result<String, String> {
    let text = expanded(k)?;
    text.lines()
        .find(|l| l.starts_with("u:s_tep :="))
        .map(str::to_string)
        .ok_or_else(|| "the expansion has no u:s_tep line".to_string())
}

/// The kernel applied by a direct loop (for the tests): each cell the
/// sum of its neighbors times the kernel (the edges wrap), divided.
pub fn direct(k: &Kernel, img: &[f64]) -> Vec<f64> {
    let (rows, cols, h) = (ROWS as i64, COLS as i64, k.side as i64 / 2);
    let mut a = img.to_vec();
    for _ in 0..k.steps {
        let mut b = vec![0.0; a.len()];
        for r in 0..rows {
            for c in 0..cols {
                let mut s = 0.0;
                for i in 0..k.side as i64 {
                    for j in 0..k.side as i64 {
                        let (rr, cc) = ((r + i - h).rem_euclid(rows), (c + j - h).rem_euclid(cols));
                        s += k.w[(i * k.side as i64 + j) as usize] * a[(rr * cols + cc) as usize];
                    }
                }
                b[(r * cols + c) as usize] = s / k.divide;
            }
        }
        a = b;
    }
    a
}

/// One cell's terms for a single application: (weight, row offset,
/// column offset, the neighbor's value), for each weight not 0.
pub fn terms(k: &Kernel, img: &[f64], r: usize, c: usize) -> Vec<(f64, i64, i64, f64)> {
    let (rows, cols, h) = (ROWS as i64, COLS as i64, k.side as i64 / 2);
    let mut t = Vec::new();
    for i in 0..k.side as i64 {
        for j in 0..k.side as i64 {
            let w = k.w[(i * k.side as i64 + j) as usize];
            if w != 0.0 {
                let (dr, dc) = (i - h, j - h);
                let (rr, cc) = ((r as i64 + dr).rem_euclid(rows), (c as i64 + dc).rem_euclid(cols));
                t.push((w, dr, dc, img[(rr * cols + cc) as usize]));
            }
        }
    }
    t
}
