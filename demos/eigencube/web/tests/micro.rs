//! The page's model against what a physical cube must do: the stickers
//! X_eTaL reads back from the rotation matrices, checked by identities
//! that hold for every move, not by one remembered picture.

use eigencube_web::micro::{inverse, name, run, IMPORT, LIBRARY, SOURCE};

/// The solved cube: nine stickers of each face's own color.
fn solved() -> Vec<u8> {
    (1..=6).flat_map(|f| std::iter::repeat(f).take(9)).collect()
}

#[test]
fn the_page_and_the_command_line_use_one_library() {
    assert!(SOURCE.contains(IMPORT.trim_end()), "eigencube.xtl imports the library");
    for f in ["h:c_hildren := { S ->", "l:s_tickers := { S ->", "l:s_olve := { hybrid S ->"] {
        assert!(LIBRARY.contains(f), "the library defines {f}");
    }
}

#[test]
fn no_moves_is_the_solved_cube() {
    assert_eq!(run(&[]).unwrap(), solved());
}

#[test]
fn every_move_has_order_four_and_its_inverse_undoes_it() {
    for m in 1..=12u8 {
        assert_eq!(run(&[m; 4]).unwrap(), solved(), "{} four times", name(m));
        assert_ne!(run(&[m; 2]).unwrap(), solved(), "{} twice moves stickers", name(m));
        assert_eq!(run(&[m, inverse(m)]).unwrap(), solved(), "{} then {}", name(m), name(inverse(m)));
    }
}

#[test]
fn a_turn_moves_twelve_side_stickers_and_keeps_every_color_count() {
    for m in 1..=12u8 {
        let s = run(&[m]).unwrap();
        let moved = s.iter().zip(solved()).filter(|(a, b)| **a != *b).count();
        assert_eq!(moved, 12, "{}: the turning face keeps its color, its four neighbors trade a row each", name(m));
        for c in 1..=6u8 {
            assert_eq!(s.iter().filter(|&&x| x == c).count(), 9);
        }
    }
}

#[test]
fn turns_follow_the_usual_notation() {
    // U (up, clockwise seen from above) brings the right face's top row
    // to the front; R brings the front's right column up.
    let u = run(&[1]).unwrap();
    assert_eq!(&u[18..21], &[5, 5, 5]);
    let r = run(&[9]).unwrap();
    assert_eq!([r[2], r[5], r[8]], [3, 3, 3]);
}

#[test]
fn sexy_move_has_order_six_and_a_scramble_undoes_in_reverse() {
    let sexy = [9u8, 1, 10, 2]; // R U R' U'
    let six: Vec<u8> = sexy.iter().copied().cycle().take(24).collect();
    assert_eq!(run(&six).unwrap(), solved());
    assert_ne!(run(&sexy).unwrap(), solved());
    let scramble = [3u8, 8, 11, 2, 6, 9, 4, 12, 1, 5, 7, 10, 3, 3];
    let back: Vec<u8> = scramble.iter().chain(scramble.iter().rev().map(|&m| inverse(m)).collect::<Vec<_>>().iter()).copied().collect();
    assert_ne!(run(&scramble).unwrap(), solved());
    assert_eq!(run(&back).unwrap(), solved());
}

#[test]
fn the_pages_solve_ends_at_the_solved_cube() {
    use eigencube_web::micro::solve;
    for scramble in [vec![1u8, 9, 5, 12, 3, 8, 6, 2, 11, 4, 7, 10, 9, 1, 5, 3, 12, 6, 8, 2], vec![9u8, 1, 10, 2], vec![]] {
        let sol = solve(&scramble).unwrap();
        let all: Vec<u8> = scramble.iter().chain(sol.iter()).copied().collect();
        assert_eq!(run(&all).unwrap(), solved(), "scramble {:?}", scramble.iter().map(|&m| name(m)).collect::<Vec<_>>());
    }
}
