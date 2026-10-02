//! The model: the demo's own X_eTaL program (nbody.xtl): the pairs'
//! displacement cube, the pull, the reduce to accelerations and the
//! leapfrog step, run on bodies the page keeps, with the constants of
//! the preset it picks. Nothing here knows about the browser.

use std::f64::consts::PI;

use microscope::run::{lit, numbers, output, section};

/// The command-line program; the page runs its core.
pub const SOURCE: &str = include_str!("../../nbody.xtl");

/// The masses spread over the pairs, the cube, the pull, the reduce
/// and the step.
pub fn core() -> &'static str {
    section(SOURCE, "# -- the bodies' masses", "# -- end of the core")
}

/// The constants of a run: G, the softening length squared and the
/// time step.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Physics {
    pub g: f64,
    pub eps2: f64,
    pub dt: f64,
}

/// The bodies: masses, positions and velocities, one item per body.
#[derive(Clone, Debug, PartialEq)]
pub struct Bodies {
    pub m: Vec<f64>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub vx: Vec<f64>,
    pub vy: Vec<f64>,
}

impl Bodies {
    pub fn len(&self) -> usize {
        self.m.len()
    }

    pub fn is_empty(&self) -> bool {
        self.m.is_empty()
    }

    /// The total momentum, sum of m v.
    pub fn momentum(&self) -> (f64, f64) {
        let px = (0..self.len()).map(|i| self.m[i] * self.vx[i]).sum();
        let py = (0..self.len()).map(|i| self.m[i] * self.vy[i]).sum();
        (px, py)
    }

    /// Kinetic plus (softened) potential energy.
    pub fn energy(&self, ph: &Physics) -> f64 {
        let n = self.len();
        let mut e = 0.0;
        for i in 0..n {
            e += 0.5 * self.m[i] * (self.vx[i] * self.vx[i] + self.vy[i] * self.vy[i]);
            for j in i + 1..n {
                let (dx, dy) = (self.x[i] - self.x[j], self.y[i] - self.y[j]);
                e -= ph.g * self.m[i] * self.m[j] / (dx * dx + dy * dy + ph.eps2).sqrt();
            }
        }
        e
    }

    /// Move to the centre-of-mass frame: the centre at 0, still.
    fn centred(mut self) -> Self {
        let mt: f64 = self.m.iter().sum();
        let c = |v: &[f64]| (0..v.len()).map(|i| self.m[i] * v[i]).sum::<f64>() / mt;
        let (cx, cy, cvx, cvy) = (c(&self.x), c(&self.y), c(&self.vx), c(&self.vy));
        for i in 0..self.len() {
            self.x[i] -= cx;
            self.y[i] -= cy;
            self.vx[i] -= cvx;
            self.vy[i] -= cvy;
        }
        self
    }
}

/// A starting scene: its bodies, constants, the half-width of the
/// view and how many steps a frame runs.
#[derive(Clone, Copy, Debug)]
pub struct Preset {
    pub name: &'static str,
    pub physics: Physics,
    pub view: f64,
    pub per_frame: usize,
    pub bodies: fn() -> Bodies,
    pub about: &'static str,
}

/// Three equal masses on one figure-eight curve (Chenciner and
/// Montgomery, 2000).
pub fn figure_eight() -> Bodies {
    let (x, y) = (0.97000436, -0.24308753);
    let (vx, vy) = (-0.93240737, -0.86473146);
    Bodies {
        m: vec![1.0, 1.0, 1.0],
        x: vec![x, -x, 0.0],
        y: vec![y, -y, 0.0],
        vx: vec![-vx / 2.0, -vx / 2.0, vx],
        vy: vec![-vy / 2.0, -vy / 2.0, vy],
    }
}

/// Two stars of mass 0.5 circling each other 0.5 apart, and light
/// planets on circular orbits around the pair.
pub fn binary_planets() -> Bodies {
    let v = (0.5f64 / (2.0 * 0.5)).sqrt();
    let mut b = Bodies { m: vec![0.5, 0.5], x: vec![-0.25, 0.25], y: vec![0.0; 2], vx: vec![0.0; 2], vy: vec![-v, v] };
    for (k, r) in [1.3f64, 1.7, 2.1, 2.6, 3.1].into_iter().enumerate() {
        let a = 2.4 * k as f64;
        let s = (1.0 / r).sqrt();
        b.m.push(0.00001);
        b.x.push(r * a.cos());
        b.y.push(r * a.sin());
        b.vx.push(-s * a.sin());
        b.vy.push(s * a.cos());
    }
    b.centred()
}

/// A small random number generator, so a preset is the same each time.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

pub const CLUSTER: usize = 50;

/// 50 equal bodies scattered over a disk, turning slower than an orbit
/// would need, so the disk collapses into clumps that swing past each
/// other.
pub fn cluster() -> Bodies {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let n = CLUSTER;
    let mut b = Bodies { m: vec![1.0 / n as f64; n], x: vec![], y: vec![], vx: vec![], vy: vec![] };
    for _ in 0..n {
        let r = rng.next().sqrt();
        let a = 2.0 * PI * rng.next();
        let s = 0.6 * r.sqrt() + 0.1 * (rng.next() - 0.5);
        b.x.push(r * a.cos());
        b.y.push(r * a.sin());
        b.vx.push(-s * a.sin());
        b.vy.push(s * a.cos());
    }
    b.centred()
}

/// The two-body problem: masses 1 and 0.2 on ellipses with
/// eccentricity 0.5 and semi-major axis 1 (relative orbit), starting
/// at the closest point.
pub fn kepler() -> Bodies {
    let (m1, m2, a, e): (f64, f64, f64, f64) = (1.0, 0.2, 1.0, 0.5);
    let mt = m1 + m2;
    let r = a * (1.0 - e);
    let v = (mt * (1.0 + e) / r).sqrt();
    Bodies {
        m: vec![m1, m2],
        x: vec![-m2 / mt * r, m1 / mt * r],
        y: vec![0.0; 2],
        vx: vec![0.0; 2],
        vy: vec![-m2 / mt * v, m1 / mt * v],
    }
}

/// Kepler's third law: the period of the two-body preset, 2 pi
/// sqrt(a^3 / (G M)).
pub fn kepler_period() -> f64 {
    2.0 * PI * (1.0f64 / 1.2).sqrt()
}

pub const PRESETS: &[Preset] = &[
    Preset {
        name: "Figure-eight (three bodies)",
        physics: Physics { g: 1.0, eps2: 0.000001, dt: 0.005 },
        view: 1.3,
        per_frame: 4,
        bodies: figure_eight,
        about: "Three equal masses chasing each other round one figure-eight curve, a periodic orbit found in 2000.",
    },
    Preset {
        name: "Binary star and planets",
        physics: Physics { g: 1.0, eps2: 0.0001, dt: 0.005 },
        view: 3.4,
        per_frame: 8,
        bodies: binary_planets,
        about: "Two stars circling each other; five light planets orbit the pair, the inner ones wobbling with the binary's pull.",
    },
    Preset {
        name: "Cluster (50 bodies)",
        physics: Physics { g: 1.0, eps2: 0.0025, dt: 0.004 },
        view: 1.6,
        per_frame: 2,
        bodies: cluster,
        about: "50 equal bodies in a slowly turning disk: too slow to orbit, it collapses into clumps that fling bodies outwards.",
    },
    Preset {
        name: "Two bodies (Kepler ellipse)",
        physics: Physics { g: 1.0, eps2: 0.00000001, dt: 0.002 },
        view: 1.1,
        per_frame: 10,
        bodies: kepler,
        about: "Masses 1 and 0.2 on ellipses round their centre of mass; Kepler's third law gives the period, 2 pi sqrt(a^3 / G M).",
    },
];

/// Every array of a run's end state, as X_eTaL computed them.
#[derive(Clone, Debug, PartialEq)]
pub struct Anatomy {
    /// The bodies after the run.
    pub next: Bodies,
    /// `u:c_ube p`: 2 x N x N, item [k][i][j] = p_k,i - p_k,j.
    pub cube: Vec<f64>,
    /// `u:p_ull d`: N x N, g m_j / (r_ij^2 + eps2)^1.5.
    pub pull: Vec<f64>,
    /// `u:a_cc p`: 2 x N, the x row then the y row.
    pub acc: Vec<f64>,
}

impl Anatomy {
    pub fn n(&self) -> usize {
        self.next.len()
    }

    /// Body j's contribution to body i's acceleration:
    /// -(p_i - p_j) * pull_ij.
    pub fn part(&self, i: usize, j: usize) -> (f64, f64) {
        let n = self.n();
        let w = self.pull[i * n + j];
        (-self.cube[i * n + j] * w, -self.cube[n * n + i * n + j] * w)
    }
}

fn strand(xs: &[f64]) -> String {
    xs.iter().map(|&x| lit(x)).collect::<Vec<_>>().join(" ")
}

/// The program: the constants, the core, `steps` steps from `b`, and
/// the arrays of the end state printed.
pub fn program(b: &Bodies, ph: &Physics, steps: usize) -> String {
    let n = b.len();
    let params = format!("g := {}\neps2 := {}\ndt := {}\nm := {}\n", lit(ph.g), lit(ph.eps2), lit(ph.dt), strand(&b.m));
    let state = [&b.x[..], &b.y[..], &b.vx[..], &b.vy[..]].concat();
    format!(
        "{params}{}s0 := (4 c_at {n}) r_eshape {}\ns := {steps} 'u:s_tep p_ower s0\n\
         p := 2 t_ake s\nd := u:c_ube p\nr_avel s\nr_avel d\nr_avel u:p_ull d\nr_avel u:a_cc p\n",
        core(),
        strand(&state)
    )
}

/// Run `steps` steps from `b` through X_eTaL.
pub fn run(b: &Bodies, ph: &Physics, steps: usize) -> Result<Anatomy, String> {
    let n = b.len();
    let out = output(&program(b, ph, steps), 4)?;
    let s = numbers::<f64>(&out[0], 4 * n)?;
    let row = |k: usize| s[k * n..(k + 1) * n].to_vec();
    Ok(Anatomy {
        next: Bodies { m: b.m.clone(), x: row(0), y: row(1), vx: row(2), vy: row(3) },
        cube: numbers(&out[1], 2 * n * n)?,
        pull: numbers(&out[2], n * n)?,
        acc: numbers(&out[3], 2 * n)?,
    })
}

/// The accelerations by a direct double loop, summed in the order
/// X_eTaL's reduce sums them (from the right).
pub fn direct_acc(b: &Bodies, ph: &Physics) -> (Vec<f64>, Vec<f64>) {
    let n = b.len();
    let (mut ax, mut ay) = (vec![0.0; n], vec![0.0; n]);
    for i in 0..n {
        let (mut sx, mut sy) = (0.0, 0.0);
        for j in (0..n).rev() {
            let (dx, dy) = (b.x[i] - b.x[j], b.y[i] - b.y[j]);
            let w = ph.g * b.m[j] / (dx * dx + dy * dy + ph.eps2).powf(1.5);
            sx = dx * w + sx;
            sy = dy * w + sy;
        }
        ax[i] = -sx;
        ay[i] = -sy;
    }
    (ax, ay)
}

/// One kick-drift-kick step by direct loops.
pub fn direct_step(b: &Bodies, ph: &Physics) -> Bodies {
    let h = ph.dt / 2.0;
    let (ax, ay) = direct_acc(b, ph);
    let n = b.len();
    let hx: Vec<f64> = (0..n).map(|i| b.vx[i] + h * ax[i]).collect();
    let hy: Vec<f64> = (0..n).map(|i| b.vy[i] + h * ay[i]).collect();
    let x: Vec<f64> = (0..n).map(|i| b.x[i] + ph.dt * hx[i]).collect();
    let y: Vec<f64> = (0..n).map(|i| b.y[i] + ph.dt * hy[i]).collect();
    let mid = Bodies { m: b.m.clone(), x, y, vx: hx, vy: hy };
    let (ax, ay) = direct_acc(&mid, ph);
    Bodies {
        vx: (0..n).map(|i| mid.vx[i] + h * ax[i]).collect(),
        vy: (0..n).map(|i| mid.vy[i] + h * ay[i]).collect(),
        ..mid
    }
}
