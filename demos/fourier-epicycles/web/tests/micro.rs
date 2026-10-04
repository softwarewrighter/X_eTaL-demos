use fourier_epicycles_web::micro::{core, direct_dft, preset, run, Curve, N, PRESETS, SOURCE};

#[test]
fn the_page_runs_the_command_line_programs_core() {
    assert!(core().contains("A := ") && core().contains("err := "));
    assert!(SOURCE.contains(core()));
}

#[test]
fn all_the_circles_give_the_curve_back() {
    for i in 0..PRESETS.len() {
        let a = run(&preset(i)).unwrap_or_else(|e| panic!("{}: {e}", PRESETS[i]));
        assert!(a.err[N - 1] < 1e-9, "{}: {}", PRESETS[i], a.err[N - 1]);
        for j in [0, N / 3, N - 1] {
            let (x, y) = a.joint(j, N - 1);
            assert!((x - a.curve.x[j]).abs() < 1e-9 && (y - a.curve.y[j]).abs() < 1e-9);
        }
    }
}

#[test]
fn a_circle_is_one_term() {
    let c = Curve { x: (0..N).map(|j| (std::f64::consts::TAU * j as f64 / N as f64).cos()).collect(), y: (0..N).map(|j| (std::f64::consts::TAU * j as f64 / N as f64).sin()).collect() };
    let a = run(&c).unwrap();
    assert_eq!(a.f[a.order[0] - 1], 1.0);
    assert!((a.radius(0) - 1.0).abs() < 1e-12);
    assert!((1..N).all(|c| a.radius(c) < 1e-12));
    assert!(a.err[0] < 1e-12);
}

#[test]
fn parseval_energy_is_kept() {
    let c = preset(0);
    let a = run(&c).unwrap();
    let energy: f64 = (0..N).map(|j| c.x[j] * c.x[j] + c.y[j] * c.y[j]).sum::<f64>() / N as f64;
    let spectrum: f64 = a.amp.iter().map(|v| v * v).sum();
    assert!((energy - spectrum).abs() < 1e-12, "{energy} {spectrum}");
}

#[test]
fn x_etal_agrees_with_a_direct_dft() {
    let c = preset(2);
    let a = run(&c).unwrap();
    for (k, (r, i)) in direct_dft(&c).into_iter().enumerate() {
        assert!((a.re[k] - r).abs() < 1e-12 && (a.im[k] - i).abs() < 1e-12, "k {k}");
    }
}

#[test]
fn circles_come_strongest_first_and_the_error_falls() {
    // The square: its series never ends, so each circle helps.
    let a = run(&preset(1)).unwrap();
    assert!((1..N).all(|c| a.radius(c - 1) >= a.radius(c)));
    assert!(a.err[0] > a.err[3] && a.err[3] > a.err[15] && a.err[15] > a.err[63]);
    // Speeds: k below the middle, k - N above.
    assert_eq!((a.f[1], a.f[N / 2 - 1], a.f[N / 2], a.f[N - 1]), (1.0, (N / 2 - 1) as f64, -((N / 2) as f64), -1.0));
}

#[test]
fn a_drawn_path_is_resampled_evenly() {
    let c = Curve::resampled(&[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)]).unwrap();
    assert_eq!(c.x.len(), N);
    assert!(c.x.iter().chain(&c.y).all(|v| v.abs() <= 1.0 + 1e-12));
    assert!(Curve::resampled(&[(0.0, 0.0), (1.0, 1.0)]).is_none());
}
