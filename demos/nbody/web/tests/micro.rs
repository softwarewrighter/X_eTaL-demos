use nbody_web::micro::{
    core, direct_acc, direct_step, figure_eight, kepler, kepler_period, run, Physics, CLUSTER, PRESETS, SOURCE,
};

fn close(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol * (1.0 + a.abs().max(b.abs()))
}

#[test]
fn the_page_runs_the_command_line_programs_core() {
    assert!(core().contains("u:c_ube := { p ->"));
    assert!(core().contains("u:s_tep := { s ->"));
    assert!(SOURCE.contains(core()));
}

#[test]
fn every_preset_runs_and_the_cube_has_its_shape() {
    for p in PRESETS {
        let b = (p.bodies)();
        let n = b.len();
        let a = run(&b, &p.physics, 2).unwrap_or_else(|e| panic!("{}: {e}", p.name));
        assert_eq!((a.cube.len(), a.pull.len(), a.acc.len()), (2 * n * n, n * n, 2 * n));
        for i in 0..n {
            assert_eq!(a.cube[i * n + i], 0.0, "a body's displacement from itself is 0");
        }
    }
    assert_eq!(cluster_len(), CLUSTER);
}

fn cluster_len() -> usize {
    (PRESETS[2].bodies)().len()
}

#[test]
fn the_cube_is_every_pairs_displacement() {
    let p = &PRESETS[2];
    let a = run(&(p.bodies)(), &p.physics, 0).unwrap();
    let (b, n) = (&a.next, a.n());
    for i in 0..n {
        for j in 0..n {
            assert_eq!(a.cube[i * n + j], b.x[i] - b.x[j]);
            assert_eq!(a.cube[n * n + i * n + j], b.y[i] - b.y[j]);
        }
    }
}

#[test]
fn forces_are_equal_and_opposite() {
    // F_ij = m_i * (body j's part of a_i) must be -F_ji.
    let p = &PRESETS[2];
    let a = run(&(p.bodies)(), &p.physics, 3).unwrap();
    let (m, n) = (&a.next.m, a.n());
    for i in 0..n {
        for j in 0..n {
            let (fx, fy) = a.part(i, j);
            let (gx, gy) = a.part(j, i);
            assert!(close(m[i] * fx, -m[j] * gx, 1e-12) && close(m[i] * fy, -m[j] * gy, 1e-12), "{i} {j}");
        }
    }
}

#[test]
fn the_reduce_sums_each_row_of_parts() {
    let p = &PRESETS[1];
    let a = run(&(p.bodies)(), &p.physics, 1).unwrap();
    let n = a.n();
    for i in 0..n {
        let sx: f64 = (0..n).map(|j| a.part(i, j).0).sum();
        let sy: f64 = (0..n).map(|j| a.part(i, j).1).sum();
        assert!(close(a.acc[i], sx, 1e-12) && close(a.acc[n + i], sy, 1e-12));
    }
}

#[test]
fn momentum_is_conserved() {
    let p = &PRESETS[2];
    let b = (p.bodies)();
    let (px0, py0) = b.momentum();
    let a = run(&b, &p.physics, 40).unwrap();
    let (px, py) = a.next.momentum();
    assert!((px - px0).abs() < 1e-13 && (py - py0).abs() < 1e-13, "{px} {py}");
}

#[test]
fn x_etal_agrees_with_a_direct_loop() {
    let p = &PRESETS[2];
    let b = (p.bodies)();
    let (ax, ay) = direct_acc(&b, &p.physics);
    let a = run(&b, &p.physics, 0).unwrap();
    let n = b.len();
    for i in 0..n {
        assert!(close(a.acc[i], ax[i], 1e-12) && close(a.acc[n + i], ay[i], 1e-12));
    }
    let mut d = b.clone();
    for _ in 0..10 {
        d = direct_step(&d, &p.physics);
    }
    let a = run(&b, &p.physics, 10).unwrap();
    for i in 0..n {
        assert!(close(a.next.x[i], d.x[i], 1e-10) && close(a.next.vy[i], d.vy[i], 1e-10), "body {i}");
    }
}

#[test]
fn two_bodies_return_after_keplers_period() {
    let ph = Physics { g: 1.0, eps2: 0.00000001, dt: 0.002 };
    let b = kepler();
    let t = kepler_period();
    let half = run(&b, &ph, (t / 2.0 / ph.dt).round() as usize).unwrap().next;
    // Half a period on: the far point, a(1 + e) = 1.5 apart, on the other side.
    assert!(close(half.x[1] - half.x[0], -1.5, 2e-3), "{}", half.x[1] - half.x[0]);
    let full = run(&b, &ph, (t / ph.dt).round() as usize).unwrap().next;
    for i in 0..2 {
        assert!((full.x[i] - b.x[i]).abs() < 5e-3 && (full.y[i] - b.y[i]).abs() < 5e-3, "body {i}: {full:?}");
    }
}

#[test]
fn the_figure_eight_keeps_its_energy_and_comes_round() {
    let p = &PRESETS[0];
    let b = figure_eight();
    let e0 = b.energy(&p.physics);
    // The figure-eight's period is about 6.326.
    let a = run(&b, &p.physics, (6.3259 / p.physics.dt).round() as usize).unwrap();
    assert!(close(a.next.energy(&p.physics), e0, 1e-5));
    for i in 0..3 {
        assert!((a.next.x[i] - b.x[i]).abs() < 1e-2 && (a.next.y[i] - b.y[i]).abs() < 1e-2, "body {i}");
    }
}
