use mandelbrot_web::micro::{core, escape, lit, orbit, orbit_def, run, View, MIN_WIDTH, SOURCE};

const SMALL: View = View { rows: 12, cols: 18, cx: -0.6, cy: 0.0, w: 3.0 };

#[test]
fn the_page_runs_the_command_line_programs_core() {
    assert!(core().starts_with("# Numbers centered on 0"));
    assert!(core().contains("u:s_tep := { s ->"));
    assert!(core().contains("counts := 3 s_elect z"));
    assert!(orbit_def().starts_with("u:o_rbit := { s ->"));
    assert!(SOURCE.contains(core()));
}

#[test]
fn shapes_and_ranges() {
    let f = run(&SMALL, 10).unwrap();
    assert_eq!((f.re.len(), f.im.len(), f.counts.len()), (18, 12, 216));
    assert!(f.counts.iter().all(|&c| (0.0..=10.0).contains(&c)));
    assert!(f.inside.iter().all(|&c| c == 0.0 || c == 1.0));
}

#[test]
fn c_is_broadcast_from_a_row_and_a_column() {
    let f = run(&SMALL, 1).unwrap();
    for y in 0..12 {
        for x in 0..18 {
            assert_eq!(f.cr[y * 18 + x], f.re[x]);
            assert_eq!(f.ci[y * 18 + x], f.im[y]);
            let (a, b) = SMALL.point(y, x);
            assert!((a - f.re[x]).abs() < 1e-12 && (b - f.im[y]).abs() < 1e-12);
        }
    }
}

#[test]
fn the_set_appears_step_by_step() {
    // Inside after k steps is exactly "stayed inside for all k + 1 counted steps".
    let k = 6;
    let a = run(&SMALL, k).unwrap();
    let b = run(&SMALL, k + 1).unwrap();
    let from_counts: Vec<f64> = b.counts.iter().map(|&c| (c == (k + 1) as f64) as u8 as f64).collect();
    assert_eq!(a.inside, from_counts);
    // Fewer points are inside after more steps.
    let n = |f: &mandelbrot_web::micro::Frame| f.inside.iter().filter(|&&v| v == 1.0).count();
    assert!(n(&b) <= n(&a));
}

#[test]
fn known_points() {
    let v = View { rows: 3, cols: 3, cx: 0.0, cy: 0.0, w: 3.0 };
    let f = run(&v, 20).unwrap();
    assert_eq!(f.counts[4], 20.0, "c = 0 never leaves");
    assert_eq!(f.counts[5], 3.0, "c = 1: z is 0, 1, 2 (|2|^2 = 4 is still inside), then 5");
}

#[test]
fn the_orbit_matches_the_command_line() {
    let o = orbit((-0.75, 0.1), 3).unwrap();
    assert_eq!(o[1], (-0.75, 0.1));
    assert_eq!(o[2], (-0.1975, -0.05000000000000002));
    assert_eq!(escape(&o), None);
    let out = orbit((1.0, 0.0), 6).unwrap();
    assert_eq!(escape(&out), Some(3), "c = 1: 0, 1, 2, then 5 is outside");
}

#[test]
fn the_orbit_agrees_with_the_grid() {
    let k = 5;
    let f = run(&SMALL, k).unwrap();
    let (y, x) = (6, 9);
    let o = orbit(SMALL.point(y, x), k).unwrap();
    let (a, b) = o[k];
    if escape(&o).is_none() {
        assert!((a * a + b * b - f.m2[y * 18 + x]).abs() < 1e-9);
    }
}

#[test]
fn literals_round_trip() {
    // The shortest form, with an exponent when small (X_eTaL reads it).
    for x in [0.0, 1.0, -0.6, 3.0 / 65536.0, -1.234e-11, 2.5e-300, 123456.789, 1e-12] {
        let s = lit(x);
        assert_eq!(s.parse::<f64>().unwrap(), x, "{x} as {s}");
    }
}

#[test]
fn a_deep_zoom_still_runs() {
    // Zoomed in 40 times (width 3 / 2^40 = 2.7e-12) around a point of the edge.
    let v = View { rows: 4, cols: 6, cx: -0.743643887037151, cy: 0.131825904205330, w: 3.0 / 2f64.powi(40) };
    assert!(v.w > MIN_WIDTH);
    run(&v, 10).unwrap();
    orbit(v.point(1, 1), 10).unwrap();
}
