//! The model: the demo's own X_eTaL program (ca-lab.xtl): elementary
//! rules grown as a history, and 2-D rules as a table on (state, live
//! neighbors). Nothing here knows about the browser.

use microscope::run::{matrix, numbers, output, section};

/// The command-line program; the page runs its core.
pub const SOURCE: &str = include_str!("../../ca-lab.xtl");

pub fn core() -> &'static str {
    section(SOURCE, "# -- one dimension", "# -- end of the core")
}

fn ints(xs: &[i64]) -> String {
    let v: Vec<String> = xs.iter().map(i64::to_string).collect();
    v.join(" ")
}

// -- one dimension ---------------------------------------------------

pub const WIDTH: usize = 129;
pub const GENS: usize = 96;

/// The 8 bits of a rule: the next state for neighborhood numbers 0..7.
pub fn bits(rule: u8) -> [i64; 8] {
    std::array::from_fn(|i| ((rule >> i) & 1) as i64)
}

/// The first row: one live cell in the middle, or a seeded scatter.
pub fn first_row(random: bool) -> Vec<i64> {
    let mut seed: u64 = 0x2545F4914F6CDD1D;
    (0..WIDTH)
        .map(|i| match random {
            false => (i == WIDTH / 2) as i64,
            true => {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                (seed % 2) as i64
            }
        })
        .collect()
}

/// A history and, for one generation, the arrays of its step.
#[derive(Clone, Debug, PartialEq)]
pub struct History {
    /// GENS rows of WIDTH cells, the first row at the top.
    pub rows: Vec<i64>,
    /// For generation `g`: its left neighbors, the row, its right
    /// neighbors, the neighborhood numbers, and the next row.
    pub left: Vec<i64>,
    pub row: Vec<i64>,
    pub right: Vec<i64>,
    pub number: Vec<i64>,
    pub next: Vec<i64>,
}

/// The program growing `first` under `rule`, and showing generation `g`.
pub fn program_1d(rule: u8, first: &[i64], g: usize) -> String {
    format!(
        "{}tbl := {}\nr0 := {}\nh := {} '{{ h -> tbl u:g_row h }} p_ower ({GENS} c_at {WIDTH}) r_eshape r0\n\
         r := {} s_elect h\nr_avel h\nr_avel -1 o_- r\nr\nr_avel 1 o_- r\n\
         r_avel (4 * -1 o_- r) + (2 * r) + 1 o_- r\ntbl u:r_ow r\n",
        core(),
        ints(&bits(rule)),
        ints(first),
        GENS - 1,
        g.min(GENS - 1) + 1
    )
}

pub fn grow(rule: u8, first: &[i64], g: usize) -> Result<History, String> {
    let out = output(&program_1d(rule, first, g), 6)?;
    let w = |i: usize| numbers::<i64>(&out[i], WIDTH);
    Ok(History { rows: numbers(&out[0], GENS * WIDTH)?, left: w(1)?, row: w(2)?, right: w(3)?, number: w(4)?, next: w(5)? })
}

// -- two dimensions --------------------------------------------------

pub const ROWS: usize = 48;
pub const COLS: usize = 64;

/// A 2-D rule: its states' names and the table, state * 9 + live
/// neighbors -> next state.
#[derive(Clone, Debug, PartialEq)]
pub struct Rule2 {
    pub name: &'static str,
    pub states: &'static [&'static str],
    pub table: Vec<i64>,
}

pub fn rules() -> Vec<Rule2> {
    let rule = |name, states, t: &[i64]| Rule2 { name, states, table: t.to_vec() };
    vec![
        rule("Life", &["dead", "alive"], &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0]),
        rule("Brian's Brain", &["off", "firing", "dying"], &[0, 0, 1, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        rule("Wireworld", &["empty", "head", "tail", "wire"], &[0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3]),
    ]
}

/// One step's arrays, as X_eTaL computed them.
#[derive(Clone, Debug, PartialEq)]
pub struct Step2 {
    /// The board before the last step, its live-neighbor counts, the
    /// table numbers (state * 9 + count), and the board after it.
    pub board: Vec<i64>,
    pub count: Vec<i64>,
    pub number: Vec<i64>,
    pub next: Vec<i64>,
}

/// The board goes in as a matrix of Int literals.
pub fn program_2d(table: &[i64], board: &[i64], steps: usize) -> String {
    format!(
        "{}tbl := {}\n{}b := {} '{{ b -> tbl u:l_ook b }} p_ower b0\n\
         r_avel b\nr_avel u:c_ount b\nr_avel (9 * b) + u:c_ount b\nr_avel tbl u:l_ook b\n",
        core(),
        ints(table),
        matrix("b0", ROWS, COLS, board.iter().map(|v| v.to_string())),
        steps.max(1) - 1
    )
}

pub fn step(table: &[i64], board: &[i64], steps: usize) -> Result<Step2, String> {
    let out = output(&program_2d(table, board, steps), 4)?;
    let n = ROWS * COLS;
    let a = |i: usize| numbers::<i64>(&out[i], n);
    Ok(Step2 { board: a(0)?, count: a(1)?, number: a(2)?, next: a(3)? })
}

/// A starting board for rule `i`: Life and Brian's Brain a seeded
/// scatter; Wireworld a loop of wire with an electron, feeding a line.
pub fn start(i: usize) -> Vec<i64> {
    let mut b = vec![0i64; ROWS * COLS];
    let mut seed: u64 = 0x9E3779B97F4A7C15 ^ i as u64;
    let mut rnd = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    match i {
        2 => {
            let mut set = |y: usize, x: usize, v: i64| b[y * COLS + x] = v;
            for x in 8..20 {
                set(10, x, 3);
                set(16, x, 3);
            }
            for y in 10..=16 {
                set(y, 8, 3);
                set(y, 20, 3);
            }
            for x in 20..56 {
                set(13, x, 3);
            }
            set(10, 12, 1);
            set(10, 11, 2);
        }
        1 => {
            for c in b.iter_mut() {
                *c = (rnd() % 10 == 0) as i64;
            }
        }
        _ => {
            for c in b.iter_mut() {
                *c = (rnd() % 4 == 0) as i64;
            }
        }
    }
    b
}
