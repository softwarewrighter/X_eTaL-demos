//! The model: the Eigencube library (X_eTaL-libraries, pinned in
//! LIBRARIES_COMMIT) and the demo's program, run on a list of moves.
//! Nothing here knows about the browser.

use std::sync::{Arc, Once};

use microscope::run::{numbers, output};

/// The command-line program (it imports the library).
pub const SOURCE: &str = include_str!("../../eigencube.xtl");
/// The library: the cube, its turns, stickers and solver, as pinned
/// (`just libraries` puts it in work/libraries).
pub const LIBRARY: &str = include_str!("../../../../work/libraries/libs/Eigencube/src/Eigencube.xtl");
/// The name a program imports it by (`"ec:" u_se< "Eigencube"`).
pub const LIBRARY_FILE: &str = "Eigencube.xtl";
/// The import line every program here starts with.
pub const IMPORT: &str = "\"ec:\" u_se< \"Eigencube\"\n";

/// The library where X_eTaL looks for it: the store a run's libraries
/// are read from (memory, in the browser and natively).
pub fn install() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        xetal_store::install(Arc::new(xetal_store::Memory::default()));
        let _ = xetal_store::write(LIBRARY_FILE, LIBRARY);
    });
}

/// The moves in the library's order: move 2f-1 turns face f clockwise
/// as seen facing it, move 2f counterclockwise.
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

fn strand(moves: &[u8]) -> String {
    moves.iter().map(|m| format!(" {m}")).collect()
}

/// The program that draws the cube the moves make: the stickers.
pub fn program(moves: &[u8]) -> String {
    format!("{IMPORT}ms := 1 d_rop 0{}\nr_avel ec:s_tickers ms ec:d_o ec:solved\n", strand(moves))
}

/// The 54 sticker colors (1 to 6, faces U D F B R L), face by face, each
/// face's cells row by row as a cube net shows it.
pub fn run(moves: &[u8]) -> Result<Vec<u8>, String> {
    install();
    let out = output(&program(moves), 1)?;
    numbers(&out[0], 54)
}

/// The program that solves the cube the moves make, printing the
/// solution (led by a 0, so never an empty line).
pub fn solve_program(moves: &[u8]) -> String {
    format!("{IMPORT}ms := 1 d_rop 0{}\n(sol, lens, e) := 1 ec:s_olve ms ec:d_o ec:solved\n0 c_at sol\n", strand(moves))
}

/// A solution (moves 1 to 12) of the cube the moves make.
pub fn solve(moves: &[u8]) -> Result<Vec<u8>, String> {
    install();
    let out = output(&solve_program(moves), 1)?;
    let n = out[0].split_whitespace().count();
    let v: Vec<u8> = numbers(&out[0], n)?;
    Ok(v[1..].to_vec())
}
