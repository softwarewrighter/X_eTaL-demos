use wave_tank_web::micro::{core, prelude, run, Surface, COLS, ROWS, SCENES, SOURCE};

#[test]
fn the_page_runs_the_command_line_programs_sections() {
    assert!(prelude().contains("damp := "));
    assert!(core().contains("u:s_tep := { (u, p, t) ->"));
    assert!(SOURCE.contains(prelude()) && SOURCE.contains(core()));
}

#[test]
fn every_scene_runs() {
    for i in 0..SCENES.len() {
        let a = run(&Surface::flat(), i, 3).unwrap_or_else(|e| panic!("{}: {e}", SCENES[i].0));
        assert!(a.wall.iter().all(|&w| w == 0.0 || w == 1.0));
        assert!(a.c2.iter().all(|&c| c > 0.0 && c <= 0.5), "{}: c2 must keep the step stable", SCENES[i].0);
    }
}

#[test]
fn a_still_tank_stays_still() {
    let open = SCENES.len() - 1;
    let a = run(&Surface::flat(), open, 5).unwrap();
    assert!(a.next.u.iter().all(|&u| u == 0.0));
}

#[test]
fn one_step_is_the_wave_equation() {
    let mut s = Surface::flat();
    s.drop(30, 40);
    let open = SCENES.len() - 1;
    let a = run(&s, open, 2).unwrap();
    for i in 0..ROWS * COLS {
        let want = a.damp[i] * a.wall[i] * ((2.0 * a.u[i] - a.p[i]) + a.c2[i] * a.lap[i]);
        assert!((a.next.u[i] - want).abs() < 1e-12);
        assert_eq!(a.next.p[i], a.u[i]);
    }
}

#[test]
fn a_ripple_spreads_and_walls_stay_dry() {
    let mut s = Surface::flat();
    s.drop(30, 20);
    let a = run(&s, 0, 40).unwrap();
    assert!(a.next.u[30 * COLS + 30].abs() > 1e-3, "the ripple reached 10 cells away");
    for i in 0..ROWS * COLS {
        if a.wall[i] == 0.0 {
            assert_eq!(a.next.u[i], 0.0);
        }
    }
}

#[test]
fn time_moves_on() {
    let a = run(&Surface::flat(), 0, 7).unwrap();
    assert_eq!(a.next.t, 7.0);
}
