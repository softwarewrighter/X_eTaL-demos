//! The model: the demo's own X_eTaL program (eigencube.xtl): its cube,
//! turns and stickers, run on a list of moves. Nothing here knows about
//! the browser.

use microscope::run::{numbers, output, section};

/// The command-line program; the page runs its core.
pub const SOURCE: &str = include_str!("../../eigencube.xtl");

/// The cube, the twelve turns and the stickers: the part of the program
/// a turn needs (the search tables and the solver come after it).
pub fn core() -> &'static str {
    section(SOURCE, "### The cube", "### The 24 orientations")
}

/// The whole model and solver: everything but the command line's lines.
pub fn solver() -> &'static str {
    section(SOURCE, "### The cube", "### On the command line")
}

/// The moves in X_eTaL's order: move 2f-1 turns face f clockwise as seen
/// facing it, move 2f counterclockwise.
pub const FACES: [char; 6] = ['U', 'D', 'F', 'B', 'R', 'L'];

/// Move m (1 to 12) in the usual notation: U, U', D, ...
pub fn name(m: u8) -> String {
    let f = FACES[((m - 1) / 2) as usize];
    if m % 2 == 1 { f.to_string() } else { format!("{f}'") }
}

/// The move that undoes move m.
pub fn inverse(m: u8) -> u8 {
    if m % 2 == 1 { m + 1 } else { m - 1 }
}

/// The program: the core, the moves, and the stickers it prints.
pub fn program(moves: &[u8]) -> String {
    let ms: Vec<String> = moves.iter().map(|m| m.to_string()).collect();
    format!("{}ms := 1 d_rop 0 {}\nS := ms u:d_o solved\nr_avel u:s_tickers S\n", core(), ms.join(" "))
}

/// The program that solves the cube the moves made: the solver, the
/// moves, and the solution it prints (led by a 0, so never empty).
pub fn solve_program(moves: &[u8]) -> String {
    let ms: Vec<String> = moves.iter().map(|m| m.to_string()).collect();
    format!("{}ms := 1 d_rop 0 {}\n(sol, lens, e) := 1 u:s_olve ms u:d_o solved\n0 c_at sol\n", solver(), ms.join(" "))
}

/// A solution (moves 1 to 12) of the cube the moves made.
pub fn solve(moves: &[u8]) -> Result<Vec<u8>, String> {
    let out = output(&solve_program(moves), 1)?;
    let n = out[0].split_whitespace().count();
    let v: Vec<u8> = numbers(&out[0], n)?;
    Ok(v[1..].to_vec())
}

/// The 54 sticker colors (1 to 6, faces U D F B R L), face by face, each
/// face's cells row by row as a cube net shows it.
pub fn run(moves: &[u8]) -> Result<Vec<u8>, String> {
    let out = output(&program(moves), 1)?;
    numbers(&out[0], 54)
}
