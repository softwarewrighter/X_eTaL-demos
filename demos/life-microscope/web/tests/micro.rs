use life_microscope_web::micro::{examine, pattern, program, rule, Board, PATTERNS};

fn blinker() -> Board {
    Board::with(5, 5, 1, 2, &["O", "O", "O"])
}

#[test]
fn the_page_runs_the_command_line_programs_rule() {
    assert!(rule().starts_with("u:l_ife := { ") && rule().ends_with('}'));
    assert!(program(&blinker()).starts_with(rule()));
}

#[test]
fn a_blinker_turns() {
    let a = examine(&blinker()).unwrap();
    assert_eq!(a.next, Board::with(5, 5, 2, 1, &["OOO"]).cells);
}

#[test]
fn the_parts_add_up_to_the_rule() {
    let b = pattern(0, 8, 8);
    let a = examine(&b).unwrap();
    let parts: Vec<u8> = a.three.iter().zip(&a.four).map(|(t, f)| t + f).collect();
    assert_eq!(parts, a.next);
    assert_eq!(a.shifted[4], b.cells, "offset (0, 0) is the board itself");
    for i in 0..b.cells.len() {
        let total: u8 = a.shifted.iter().map(|s| s[i]).sum();
        assert_eq!(total, a.sum[i]);
    }
}

#[test]
fn a_shifted_board_holds_the_neighbor() {
    let b = blinker();
    let a = examine(&b).unwrap();
    // Offsets (dy, dx) = (-1, 0): cell (y, x) holds b[y-1][x].
    let up = &a.shifted[1];
    for y in 0..5 {
        for x in 0..5 {
            assert_eq!(up[y * 5 + x], b.get((y + 4) % 5, x));
        }
    }
}

#[test]
fn a_glider_moves_in_four_generations() {
    let mut b = pattern(0, 8, 8);
    let start = b.clone();
    for _ in 0..4 {
        b.cells = examine(&b).unwrap().next;
    }
    let moved: Vec<u8> = (0..64).map(|i| start.cells[((i / 8 + 7) % 8) * 8 + (i % 8 + 7) % 8]).collect();
    assert_eq!(b.cells, moved);
}

#[test]
fn every_pattern_runs() {
    for i in 0..PATTERNS.len() {
        let b = pattern(i, 16, 16);
        assert!(b.population() > 0);
        examine(&b).unwrap();
    }
}

#[test]
fn the_program_prints_five_arrays() {
    assert_eq!(program(&blinker()).matches("r_avel").count(), 5);
}
