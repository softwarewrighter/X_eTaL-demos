//! The model: the demo's own X_eTaL program (fourier-epicycles.xtl):
//! its transform, circles, chain and errors, run on a curve the page
//! picks or the user draws. Nothing here knows about the browser.

use std::f64::consts::TAU;

use microscope::run::{lit, numbers, output, section};

/// The command-line program; the page runs its core.
pub const SOURCE: &str = include_str!("../../fourier-epicycles.xtl");

/// The angles, the transform, the circles, the chain, the errors.
pub fn core() -> &'static str {
    section(SOURCE, "# -- the angles", "# -- end of the core")
}

/// Points on a curve (and circles, and times).
pub const N: usize = 128;

/// A closed curve: N points, x and y.
#[derive(Clone, Debug, PartialEq)]
pub struct Curve {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

impl Curve {
    fn from_fn(f: impl Fn(f64) -> (f64, f64)) -> Self {
        let (x, y) = (0..N).map(|j| f(TAU * j as f64 / N as f64)).unzip();
        Curve { x, y }
    }

    /// Any closed path resampled to N points evenly spaced along it,
    /// centered and scaled to fit -1 .. 1.
    pub fn resampled(points: &[(f64, f64)]) -> Option<Self> {
        if points.len() < 3 {
            return None;
        }
        let mut pts = points.to_vec();
        pts.push(points[0]);
        let seg: Vec<f64> = pts.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt()).collect();
        let total: f64 = seg.iter().sum();
        if total <= 0.0 {
            return None;
        }
        let (mut x, mut y) = (vec![], vec![]);
        let (mut i, mut acc) = (0, 0.0);
        for j in 0..N {
            let want = total * j as f64 / N as f64;
            while i + 1 < seg.len() && acc + seg[i] < want {
                acc += seg[i];
                i += 1;
            }
            let t = if seg[i] > 0.0 { (want - acc) / seg[i] } else { 0.0 };
            x.push(pts[i].0 + t * (pts[i + 1].0 - pts[i].0));
            y.push(pts[i].1 + t * (pts[i + 1].1 - pts[i].1));
        }
        let (cx, cy) = (x.iter().sum::<f64>() / N as f64, y.iter().sum::<f64>() / N as f64);
        let r = x.iter().zip(&y).map(|(a, b)| (a - cx).abs().max((b - cy).abs())).fold(1e-9, f64::max);
        Some(Curve { x: x.iter().map(|a| (a - cx) / r).collect(), y: y.iter().map(|b| (b - cy) / r).collect() })
    }
}

/// The preset curves.
pub const PRESETS: &[&str] = &["Heart", "Square", "Five-pointed star", "Flower", "Trefoil knot"];

pub fn preset(i: usize) -> Curve {
    match i % PRESETS.len() {
        0 => Curve::from_fn(|t| {
            let x = 16.0 * t.sin().powi(3);
            let y = 13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();
            (x / 17.0, y / 17.0)
        }),
        1 => Curve::resampled(&[(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)]).unwrap(),
        2 => {
            let pts: Vec<(f64, f64)> = (0..10)
                .map(|i| {
                    let a = std::f64::consts::FRAC_PI_2 + TAU * i as f64 / 10.0;
                    let r = if i % 2 == 0 { 1.0 } else { 0.4 };
                    (r * a.cos(), r * a.sin())
                })
                .collect();
            Curve::resampled(&pts).unwrap()
        }
        3 => Curve::from_fn(|t| {
            let r = 0.7 + 0.25 * (5.0 * t).cos();
            (r * t.cos(), r * t.sin())
        }),
        _ => Curve::from_fn(|t| ((t.sin() + 2.0 * (2.0 * t).sin()) / 3.0, (t.cos() - 2.0 * (2.0 * t).cos()) / 3.0)),
    }
}

/// Everything X_eTaL computed for a curve.
#[derive(Clone, Debug, PartialEq)]
pub struct Anatomy {
    pub curve: Curve,
    /// Per frequency k: real and imaginary parts, strength, speed.
    pub re: Vec<f64>,
    pub im: Vec<f64>,
    pub amp: Vec<f64>,
    pub f: Vec<f64>,
    /// The frequencies strongest first (from 1).
    pub order: Vec<usize>,
    /// cos A: the angle table, N x N.
    pub cos: Vec<f64>,
    /// The chain: row time j, column c; where circle c ends (cx, cy).
    pub cx: Vec<f64>,
    pub cy: Vec<f64>,
    /// The mean error rebuilt from 1 .. N circles.
    pub err: Vec<f64>,
}

impl Anatomy {
    /// The end of circle c (0 = the first) at time j.
    pub fn joint(&self, j: usize, c: usize) -> (f64, f64) {
        (self.cx[j * N + c], self.cy[j * N + c])
    }

    /// The radius of circle c (strongest first).
    pub fn radius(&self, c: usize) -> f64 {
        self.amp[self.order[c] - 1]
    }
}

fn strand(v: &[f64]) -> String {
    v.iter().map(|&a| lit(a)).collect::<Vec<_>>().join(" ")
}

/// The program: the curve, the core, and the arrays the page shows.
pub fn program(c: &Curve) -> String {
    format!(
        "x := {}\ny := {}\n{}r_avel re\nr_avel im\nr_avel amp\nr_avel f\nr_avel order\nr_avel C\nr_avel cx\nr_avel cy\nr_avel err\n",
        strand(&c.x),
        strand(&c.y),
        core()
    )
}

/// Run the transform and the chain through X_eTaL.
pub fn run(c: &Curve) -> Result<Anatomy, String> {
    let out = output(&program(c), 9)?;
    let order: Vec<f64> = numbers(&out[4], N)?;
    Ok(Anatomy {
        curve: c.clone(),
        re: numbers(&out[0], N)?,
        im: numbers(&out[1], N)?,
        amp: numbers(&out[2], N)?,
        f: numbers(&out[3], N)?,
        order: order.iter().map(|&o| o as usize).collect(),
        cos: numbers(&out[5], N * N)?,
        cx: numbers(&out[6], N * N)?,
        cy: numbers(&out[7], N * N)?,
        err: numbers(&out[8], N)?,
    })
}

/// A direct discrete Fourier transform (for the tests): Z_k as (re, im).
pub fn direct_dft(c: &Curve) -> Vec<(f64, f64)> {
    let n = c.x.len();
    (0..n)
        .map(|k| {
            let (mut r, mut i) = (0.0, 0.0);
            for j in 0..n {
                let a = TAU * (k * j) as f64 / n as f64;
                r += c.x[j] * a.cos() + c.y[j] * a.sin();
                i += c.y[j] * a.cos() - c.x[j] * a.sin();
            }
            (r / n as f64, i / n as f64)
        })
        .collect()
}
