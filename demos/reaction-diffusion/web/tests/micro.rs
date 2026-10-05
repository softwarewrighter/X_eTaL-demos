use reaction_diffusion_web::micro::{core, lit, preset, program, run, Grid, SOURCE};

#[test]
fn the_page_runs_the_command_line_programs_core() {
    assert!(core().contains("u:l_ap := { x ->"));
    assert!(core().contains("u:s_tep := { s ->"));
    assert!(SOURCE.contains(core()));
}

#[test]
fn literals_read_back_and_tiny_ones_are_zero() {
    for x in [1.0, 0.25, -0.0123, 3.3e-9] {
        assert_eq!(lit(x).parse::<f64>().unwrap(), x, "{x}: {}", lit(x));
    }
    assert_eq!(lit(1e-20), "0.0");
}

#[test]
fn a_still_grid_stays_still() {
    // All U, no V: nothing reacts, nothing spreads.
    let g = Grid::empty(8);
    let a = run(&g, &preset(0), 5).unwrap();
    assert!(a.next.u.iter().all(|&u| (u - 1.0).abs() < 1e-12));
    assert!(a.lap_u.iter().all(|&l| l.abs() < 1e-12));
}

#[test]
fn the_laplacian_is_four_neighbors_minus_four_times_the_cell() {
    let mut g = Grid::empty(8);
    g.drop(3, 4, 1);
    let a = run(&g, &preset(0), 1).unwrap();
    let n = 8;
    for y in 0..n {
        for x in 0..n {
            let at = |yy: usize, xx: usize| a.v[(yy % n) * n + xx % n];
            let want = at(y + n - 1, x) + at(y + 1, x) + at(y, x + n - 1) + at(y, x + 1) - 4.0 * at(y, x);
            assert!((a.lap_v[y * n + x] - want).abs() < 1e-12);
        }
    }
}

#[test]
fn one_step_is_the_gray_scott_update() {
    let mut g = Grid::seeded(16);
    g.drop(8, 8, 2);
    let r = preset(1);
    let a = run(&g, &r, 1).unwrap();
    for i in 0..256 {
        let (u, v, uvv) = (a.u[i], a.v[i], a.uvv[i]);
        assert!((uvv - u * v * v).abs() < 1e-12);
        let nu = u + (r.du * a.lap_u[i] - uvv) + r.f * (1.0 - u);
        let nv = v + (r.dv * a.lap_v[i] + uvv) - (r.f + r.k) * v;
        assert!((a.next.u[i] - nu).abs() < 1e-12 && (a.next.v[i] - nv).abs() < 1e-12);
    }
}

#[test]
fn many_steps_in_one_run_equal_runs_of_one() {
    let g = Grid::seeded(16);
    let r = preset(0);
    let once = run(&g, &r, 3).unwrap().next;
    let mut step = g.clone();
    for _ in 0..3 {
        step = run(&step, &r, 1).unwrap().next;
    }
    for i in 0..256 {
        assert!((once.v[i] - step.v[i]).abs() < 1e-12);
    }
}

#[test]
fn a_pattern_grows() {
    let mut g = Grid::seeded(32);
    let r = preset(0);
    let start: f64 = g.v.iter().sum();
    let t = std::time::Instant::now();
    for _ in 0..10 {
        g = run(&g, &r, 50).unwrap().next;
    }
    eprintln!("32 x 32: 500 steps in 10 runs took {:?}", t.elapsed());
    assert!(g.v.iter().sum::<f64>() > start);
}

#[test]
fn the_program_has_two_matrices() {
    let p = program(&Grid::empty(4), &preset(0), 1);
    assert!(p.contains("u0 := (4 c_at 4) r_eshape 1.0 1.0"));
    assert!(p.contains("v0 := (4 c_at 4) r_eshape 0.0"));
}
