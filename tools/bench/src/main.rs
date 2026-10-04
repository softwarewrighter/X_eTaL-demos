//! Times the demo pages' own X_eTaL programs (each demo's `run`, the
//! code its page runs) and the showcase built-ins, natively through the
//! vendored `xetal-play`, and compares them with a committed baseline.
//!
//!   bench            run, write docs/bench.md and docs/bench.json (the new baseline)
//!   bench check      run, compare with docs/bench.json, write nothing;
//!                    exit 1 if any case is more than 15% slower
//!
//! Each case's time is the best of several runs, divided by the best
//! time of a fixed pure-Rust reference loop measured just before it,
//! so a busier or slower machine moves both and the ratio less. A case
//! that comes out slower than the tolerance is measured once more (the
//! machine is shared with other work); it is flagged only if it is
//! slower both times.

use std::time::Instant;

const RUNS: usize = 7;
const TOLERANCE: f64 = 0.15;

/// A fixed amount of plain Rust arithmetic: the yardstick.
fn reference() {
    let mut s = 0.0f64;
    for i in 0..12_000_000u64 {
        s += ((i % 1000) as f64 * 0.001).sin();
    }
    std::hint::black_box(s);
}

/// The best of `RUNS` timings of `f`, in milliseconds.
fn best(f: &dyn Fn()) -> f64 {
    f();
    (0..RUNS)
        .map(|_| {
            let t = Instant::now();
            f();
            t.elapsed().as_secs_f64() * 1000.0
        })
        .fold(f64::MAX, f64::min)
}

/// A program for one built-in on `x`, a 512 x 512 Float matrix, run
/// `reps` times (so even a fast built-in takes tens of milliseconds,
/// enough to time within the tolerance).
fn builtin(line: &str, reps: usize) -> String {
    let body: String = (0..reps).map(|_| format!("y := {line}\n")).collect();
    format!("x := (512 c_at 512) r_eshape 0.5 0.25 0.125 0.75\n{body}s_hape y\n")
}

fn run_builtin(line: &str, reps: usize) {
    microscope::run::output(&builtin(line, reps), 1).expect("the built-in program runs");
}

type Case = (&'static str, &'static str, Box<dyn Fn()>);

fn cases() -> Vec<Case> {
    let mut c: Vec<Case> = vec![];
    // The pages' programs, as each page runs them.
    c.push(("nbody", "50 bodies, 10 steps", Box::new(|| {
        use nbody_web::micro::{run, PRESETS};
        let p = &PRESETS[2];
        run(&(p.bodies)(), &p.physics, 10).unwrap();
    })));
    c.push(("image-pipeline", "96 x 96, three filters, pooling", Box::new(|| {
        use image_pipeline_web::micro::{run, Setup};
        run(&Setup::new(0)).unwrap();
    })));
    c.push(("reaction-diffusion", "64 x 64, 20 steps", Box::new(|| {
        use reaction_diffusion_web::micro::{preset, run, Grid};
        run(&Grid::seeded(64), &preset(0), 20).unwrap();
    })));
    c.push(("wave-tank", "60 x 96, 4 steps", Box::new(|| {
        use wave_tank_web::micro::{run, Surface};
        run(&Surface::flat(), 0, 4).unwrap();
    })));
    c.push(("mandelbrot", "90 x 135, 32 steps", Box::new(|| {
        use mandelbrot_web::micro::{run, View};
        run(&View::HOME, 32).unwrap();
    })));
    c.push(("julia", "90 x 135, 32 steps", Box::new(|| {
        use julia_web::micro::{counts, Set, View};
        counts(&View::JULIA, 32, Set::Julia(-0.8, 0.156)).unwrap();
    })));
    c.push(("life-microscope", "64 x 64, one generation", Box::new(|| {
        use life_microscope_web::micro::{examine, pattern};
        examine(&pattern(0, 64, 64)).unwrap();
    })));
    c.push(("ca-lab", "48 x 64 Life, 4 steps", Box::new(|| {
        use ca_lab_web::micro::{rules, start, step};
        step(&rules()[0].table, &start(0), 4).unwrap();
    })));
    c.push(("langtons-ant", "64 x 64, 50 steps", Box::new(|| {
        use langtons_ant_web::micro::{run, Ant};
        run(&Ant::start(), 50).unwrap();
    })));
    c.push(("fourier-epicycles", "128 points: transform, chain, errors", Box::new(|| {
        use fourier_epicycles_web::micro::{preset, run};
        run(&preset(2)).unwrap();
    })));
    // The showcase built-ins on a 512 x 512 matrix, each repeated.
    for (name, what, line, reps) in [
        ("elementwise", "x + x * x, 8 times", "x + x * x", 8),
        ("reduce", "'+ r_/_2 x, 8 times", "'+ r_/_2 x", 8),
        ("scan", "'+ s_\\_2 on 4096 rows of 64, once", "'+ s_\\_2 (4096 c_at 64) r_eshape x", 1),
        ("each", "a lambda on 16384 items, 8 times", "'{ v -> v * 2.0 } e_ach r_avel 32 t_ake x", 8),
        ("table", "a 512 x 512 '* t_able, twice", "(r_avel 1 t_ake x) '* t_able r_avel 1 t_ake x", 2),
        ("inner", "(32 x 256) '+ '* i_nner (256 x 32), once", "((32 c_at 256) r_eshape x) '+ '* i_nner (256 c_at 32) r_eshape x", 1),
        ("rotate", "1 o_-_2 x, 16 times", "1 o_-_2 x", 16),
        ("transpose", "o_\\ x, 32 times", "o_\\ x", 32),
    ] {
        c.push((name, what, Box::new(move || run_builtin(line, reps))));
    }
    c
}

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn vendored() -> String {
    let v = std::fs::read_to_string(root().join("vendor/xetal/VENDORED")).unwrap_or_default();
    v.lines().find_map(|l| l.strip_prefix("commit = \"")).map_or("unknown".into(), |c| c.trim_end_matches('"').chars().take(7).collect())
}

/// The baseline's ratios: `name ratio` pairs from docs/bench.json.
fn baseline() -> Vec<(String, f64)> {
    let text = std::fs::read_to_string(root().join("docs/bench.json")).unwrap_or_default();
    text.lines()
        .filter_map(|l| {
            let l = l.trim().trim_end_matches(',');
            let (k, v) = l.split_once(':')?;
            let k = k.trim().trim_matches('"');
            let ratio = v.split("\"ratio\":").nth(1)?.trim().trim_end_matches('}').trim().parse().ok()?;
            Some((k.to_string(), ratio))
        })
        .collect()
}

fn main() {
    let check = std::env::args().nth(1).as_deref() == Some("check");
    let reference_ms = best(&reference);
    let commit = vendored();
    eprintln!("reference loop: {reference_ms:.1} ms; X_eTaL {commit}; best of {RUNS}");
    let base = baseline();
    let mut rows = vec![];
    let mut slower = vec![];
    let measure = |f: &dyn Fn()| -> (f64, f64) {
        let r = best(&reference);
        let ms = best(f);
        (ms, ms / r)
    };
    for (name, what, f) in cases() {
        let (mut ms, mut ratio) = measure(f.as_ref());
        let was = base.iter().find(|(k, _)| k == name).map(|(_, r)| *r);
        if let Some(w) = was {
            if ratio / w - 1.0 > TOLERANCE {
                let (ms2, ratio2) = measure(f.as_ref());
                if ratio2 < ratio {
                    (ms, ratio) = (ms2, ratio2);
                }
            }
        }
        let change = was.map(|w| ratio / w - 1.0);
        let flag = change.is_some_and(|c| c > TOLERANCE);
        if flag {
            slower.push(name);
        }
        let shown = change.map_or("new".into(), |c| format!("{:+.0}%", 100.0 * c));
        eprintln!("{name:20} {ms:9.1} ms  ratio {ratio:8.3}  {shown}{}", if flag { "  SLOWER" } else { "" });
        rows.push((name, what, ms, ratio, shown));
    }
    if check {
        if slower.is_empty() {
            eprintln!("bench-check: ok (no case more than {:.0}% slower than docs/bench.json)", 100.0 * TOLERANCE);
            return;
        }
        eprintln!("bench-check: slower than the baseline by more than {:.0}%: {}", 100.0 * TOLERANCE, slower.join(", "));
        std::process::exit(1);
    }
    let mut json = format!("{{\n  \"xetal\": \"{commit}\",\n  \"reference_ms\": {reference_ms:.3},\n  \"cases\": {{\n");
    let n = rows.len();
    for (i, (name, _, ms, ratio, _)) in rows.iter().enumerate() {
        json += &format!("    \"{name}\": {{\"ms\": {ms:.3}, \"ratio\": {ratio:.5}}}{}\n", if i + 1 < n { "," } else { "" });
    }
    json += "  }\n}\n";
    std::fs::write(root().join("docs/bench.json"), json).expect("write docs/bench.json");
    let mut md = format!(
        "# Benchmarks\n\nWritten by `just bench` (tools/bench) with X_eTaL {commit}; the\nbaseline `just bench-check` compares against. Natively, release,\nthrough the vendored `xetal-play`, best of {RUNS} runs. The ratio is the\ntime over a fixed pure-Rust reference loop ({reference_ms:.1} ms in this\nrun), so a busier or slower machine changes it less than the time.\n`just bench-check` fails when a case's ratio is more than {:.0}% above\nthis table's.\n\n| Case | What | ms | Ratio | Change |\n| ---- | ---- | -- | ----- | ------ |\n",
        100.0 * TOLERANCE
    );
    for (name, what, ms, ratio, shown) in &rows {
        md += &format!("| {name} | {} | {ms:.1} | {ratio:.3} | {shown} |\n", what.replace('|', "\\|"));
    }
    std::fs::write(root().join("docs/bench.md"), md).expect("write docs/bench.md");
    eprintln!("wrote docs/bench.md and docs/bench.json");
}
