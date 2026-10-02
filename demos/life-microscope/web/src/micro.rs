//! The microscope's model: a board, the X_eTaL program that takes it one
//! generation on while printing every array it builds, and those arrays
//! read back. Nothing here knows about the browser.

/// The command-line program; the page runs its rule.
pub const SOURCE: &str = include_str!("../../life-microscope.xtl");

/// The Life rule as one line of X_eTaL, read from `life-microscope.xtl`.
pub fn rule() -> &'static str {
    SOURCE.lines().find(|l| l.starts_with("u:l_ife := ")).unwrap_or("")
}

/// The offsets the board is rotated by, along each axis.
pub const OFFSETS: [i32; 3] = [-1, 0, 1];

/// A board of 0 (dead) and 1 (alive) cells, row by row; its edges wrap.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<u8>,
}

impl Board {
    pub fn empty(rows: usize, cols: usize) -> Self {
        Board { rows, cols, cells: vec![0; rows * cols] }
    }

    /// A board with `pattern` (rows of '.' and 'O') placed at (top, left).
    pub fn with(rows: usize, cols: usize, top: usize, left: usize, pattern: &[&str]) -> Self {
        let mut b = Board::empty(rows, cols);
        for (y, line) in pattern.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == 'O' {
                    b.cells[((top + y) % rows) * cols + (left + x) % cols] = 1;
                }
            }
        }
        b
    }

    pub fn get(&self, y: usize, x: usize) -> u8 {
        self.cells[y * self.cols + x]
    }

    pub fn toggle(&mut self, y: usize, x: usize) {
        let c = &mut self.cells[y * self.cols + x];
        *c = 1 - *c;
    }

    pub fn population(&self) -> usize {
        self.cells.iter().filter(|&&c| c == 1).count()
    }
}

/// Every array one generation builds, as X_eTaL computed them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Anatomy {
    /// `-1 0 1 o_-_12 b`: nine boards, the one for offsets (dy, dx) at
    /// index 3 * (dy + 1) + (dx + 1); its cell (y, x) holds b[y+dy][x+dx].
    pub shifted: Vec<Vec<u8>>,
    /// `'+ r_/_12 r`: S, each cell plus its eight neighbours.
    pub sum: Vec<u8>,
    /// `s = 3`: alive next whatever the cell is now.
    pub three: Vec<u8>,
    /// `b * s = 4`: alive now with three neighbours.
    pub four: Vec<u8>,
    /// `u:l_ife b`: the next board, as the one line computes it.
    pub next: Vec<u8>,
}

/// The program that steps `b` once, printing r, S, S = 3, b * S = 4 and
/// the next board, each flattened (`r_avel`) on a line of its own.
pub fn program(b: &Board) -> String {
    let cells: Vec<String> = b.cells.iter().map(u8::to_string).collect();
    format!(
        "{}\nb := {} {} r_eshape {}\nr := -1 0 1 o_-_12 b\ns := '+ r_/_12 r\n\
         r_avel r\nr_avel s\nr_avel s = 3\nr_avel b * s = 4\nr_avel u:l_ife b\n",
        rule(),
        b.rows,
        b.cols,
        cells.join(" ")
    )
}

fn numbers(line: &str, want: usize) -> Result<Vec<u8>, String> {
    let v: Result<Vec<u8>, _> = line.split_whitespace().map(str::parse).collect();
    let v = v.map_err(|e| format!("unexpected output {line:?}: {e}"))?;
    match v.len() == want {
        true => Ok(v),
        false => Err(format!("expected {want} numbers, got {}", v.len())),
    }
}

/// Run one generation of `b` through X_eTaL and read back its arrays.
pub fn examine(b: &Board) -> Result<Anatomy, String> {
    let run = xetal_play::run(&program(b), 1);
    if !run.err.is_empty() {
        return Err(run.err);
    }
    let n = b.rows * b.cols;
    let lines: Vec<&str> = run.out.lines().collect();
    if lines.len() != 5 {
        return Err(format!("expected 5 lines of output, got {}", lines.len()));
    }
    let r = numbers(lines[0], 9 * n)?;
    Ok(Anatomy {
        shifted: r.chunks(n).map(<[u8]>::to_vec).collect(),
        sum: numbers(lines[1], n)?,
        three: numbers(lines[2], n)?,
        four: numbers(lines[3], n)?,
        next: numbers(lines[4], n)?,
    })
}

/// The patterns the page offers: name and rows ('O' alive).
pub const PATTERNS: &[(&str, &[&str])] = &[
    ("Glider", &[".O.", "..O", "OOO"]),
    ("Blinker", &["OOO"]),
    ("Toad", &[".OOO", "OOO."]),
    ("Lightweight spaceship", &[".O..O", "O....", "O...O", "OOOO."]),
    ("R-pentomino", &[".OO", "OO.", ".O."]),
    ("Pulsar quarter", &["..OOO", "", "O....O", "O....O", "O....O", "..OOO"]),
];

/// Pattern `i` near the middle of a rows x cols board.
pub fn pattern(i: usize, rows: usize, cols: usize) -> Board {
    let (_, p) = PATTERNS[i % PATTERNS.len()];
    let h = p.len();
    let w = p.iter().map(|l| l.len()).max().unwrap_or(0);
    Board::with(rows, cols, (rows - h) / 2, (cols - w) / 2, p)
}
