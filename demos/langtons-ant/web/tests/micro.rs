use langtons_ant_web::micro::{core, run, Ant, N, SOURCE};

#[test]
fn the_page_runs_the_command_line_programs_core() {
    assert!(core().contains("u:s_tep := { (b, a, d) ->"));
    assert!(SOURCE.contains(core()));
}

/// The ant, stepped directly in Rust, for comparison.
fn direct(ant: &Ant, steps: usize) -> Ant {
    let mut a = ant.clone();
    for _ in 0..steps {
        let i = a.y * N + a.x;
        a.dir = (a.dir + if a.board[i] == 0 { 1 } else { 3 }) % 4;
        a.board[i] = 1 - a.board[i];
        match a.dir {
            0 => a.y = (a.y + N - 1) % N,
            1 => a.x = (a.x + 1) % N,
            2 => a.y = (a.y + 1) % N,
            _ => a.x = (a.x + N - 1) % N,
        }
        a.steps += 1;
    }
    a
}

#[test]
fn the_ant_matches_a_direct_simulation() {
    let mut ant = Ant::start();
    let mut want = Ant::start();
    for chunk in [1, 7, 50, 300] {
        ant = run(&ant, chunk).unwrap().0;
        want = direct(&want, chunk);
        assert_eq!(ant, want, "after {} steps", want.steps);
    }
}

#[test]
fn the_last_step_is_reported() {
    let ant = Ant::start();
    let (next, last) = run(&ant, 1).unwrap();
    assert_eq!((last.y, last.x, last.dir, last.cell), (N / 2, N / 2, 0, 0));
    assert_eq!((next.y, next.x, next.dir), (N / 2, N / 2 + 1, 1), "white: turn right, step right");
    assert_eq!(next.board[(N / 2) * N + N / 2], 1, "the cell it left is black");
}
