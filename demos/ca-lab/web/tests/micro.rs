use ca_lab_web::micro::{bits, core, first_row, grow, rules, start, step, COLS, GENS, ROWS, SOURCE, WIDTH};

#[test]
fn the_page_runs_the_command_line_programs_core() {
    assert!(core().contains("u:g_row := { tbl h ->") && core().contains("u:l_ook := { tbl b ->"));
    assert!(SOURCE.contains(core()));
}

#[test]
fn rule_bits() {
    assert_eq!(bits(30), [0, 1, 1, 1, 1, 0, 0, 0]);
    assert_eq!(bits(110), [0, 1, 1, 1, 0, 1, 1, 0]);
}

/// The same rule, run directly in Rust, for comparison.
fn next_row(rule: u8, r: &[i64]) -> Vec<i64> {
    let n = r.len();
    (0..n).map(|i| {
        let k = 4 * r[(i + n - 1) % n] + 2 * r[i] + r[(i + 1) % n];
        ((rule >> k) & 1) as i64
    }).collect()
}

#[test]
fn elementary_rules_match_a_direct_computation() {
    for rule in [30u8, 90, 110, 184] {
        let first = first_row(rule == 184);
        let h = grow(rule, &first, 10).unwrap();
        let mut r = first.clone();
        for g in 0..GENS {
            assert_eq!(&h.rows[g * WIDTH..(g + 1) * WIDTH], &r[..], "rule {rule}, generation {g}");
            r = next_row(rule, &r);
        }
    }
}

#[test]
fn a_generations_arrays() {
    let h = grow(30, &first_row(false), 5).unwrap();
    assert_eq!(&h.row[..], &h.rows[5 * WIDTH..6 * WIDTH]);
    assert_eq!(&h.next[..], &h.rows[6 * WIDTH..7 * WIDTH]);
    for i in 0..WIDTH {
        assert_eq!(h.left[i], h.row[(i + WIDTH - 1) % WIDTH]);
        assert_eq!(h.number[i], 4 * h.left[i] + 2 * h.row[i] + h.right[i]);
    }
}

#[test]
fn every_2d_rule_runs_and_life_is_life() {
    for (i, r) in rules().iter().enumerate() {
        assert_eq!(r.table.len(), 9 * r.states.len());
        let s = step(&r.table, &start(i), 3).unwrap();
        assert!(s.next.iter().all(|&v| (0..r.states.len() as i64).contains(&v)), "{}", r.name);
    }
    let life = &rules()[0];
    let b = start(0);
    let s = step(&life.table, &b, 1).unwrap();
    for i in 0..ROWS * COLS {
        let (y, x) = (i / COLS, i % COLS);
        let mut n = 0;
        for dy in [ROWS - 1, 0, 1] {
            for dx in [COLS - 1, 0, 1] {
                if (dy, dx) != (0, 0) {
                    n += b[((y + dy) % ROWS) * COLS + (x + dx) % COLS];
                }
            }
        }
        assert_eq!(s.count[i], n);
        let alive = (n == 3) || (b[i] == 1 && n == 2);
        assert_eq!(s.next[i], alive as i64);
    }
}

#[test]
fn wireworld_electrons_move() {
    let ww = &rules()[2];
    let b = start(2);
    let s = step(&ww.table, &b, 4).unwrap();
    assert!(s.next.iter().any(|&v| v == 1), "a head is still running");
    assert_ne!(s.next, b);
}
