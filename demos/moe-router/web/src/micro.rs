//! The model: the demo's own X_eTaL program (moe-router.xtl): its
//! vocabulary and router weights, and its scores, softmax, top-2 and
//! load, run on a sentence the page picks. Nothing here knows about the
//! browser.

use microscope::run::{numbers, output, section};

/// The command-line program; the page runs its sections.
pub const SOURCE: &str = include_str!("../../moe-router.xtl");

/// The vocabulary's features, the embeddings and the router weights.
pub fn tables() -> &'static str {
    section(SOURCE, "# -- the vocabulary", "# -- the scores")
}

/// The router weights (shown on the page; the vocabulary is long).
pub fn weights() -> &'static str {
    section(SOURCE, "# -- the router weights", "# -- the scores")
}

/// Scores, softmax, top-2, load.
pub fn core() -> &'static str {
    section(SOURCE, "# -- the scores", "# -- end of the core")
}

pub const EXPERTS: usize = 16;
pub const FEATURES: [&str; 8] = ["animal", "number", "colour", "action", "place", "food", "function", "time"];

/// The vocabulary, in order, from the program's `# words:` line; the
/// last, `?`, stands for any word not listed.
pub fn words() -> Vec<&'static str> {
    SOURCE.lines().find_map(|l| l.strip_prefix("# words: ")).map_or(vec![], |w| w.split_whitespace().collect())
}

/// Expert e (from 0) is on grid row e / 4 (it likes feature e / 4) and
/// column e % 4 (feature 4 + e % 4).
pub fn expert_name(e: usize) -> String {
    format!("{} \u{00d7} {}", FEATURES[e / 4], FEATURES[4 + e % 4])
}

pub const SENTENCES: &[&str] = &[
    "the red fox eats fish in the park",
    "two brown horses run to the river today",
    "a cat sees seven blue birds at night",
    "the dog sleeps at home and eats bread",
    "one green apple and three fish",
];

/// A sentence's tokens: its words in the vocabulary (lowercase, without
/// punctuation; a plural "s" dropped when only the singular is listed),
/// any other word as `?`. Returns the shown words and their numbers
/// (from 1).
pub fn tokens(sentence: &str) -> Vec<(String, usize)> {
    let vocab = words();
    let find = |w: &str| vocab.iter().position(|v| *v == w);
    sentence
        .split_whitespace()
        .map(|raw| {
            let w: String = raw.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
            let i = find(&w).or_else(|| w.strip_suffix('s').and_then(find)).unwrap_or(vocab.len() - 1);
            (w, i + 1)
        })
        .filter(|(w, _)| !w.is_empty())
        .collect()
}

/// Every array of the routing, as X_eTaL computed them.
#[derive(Clone, Debug, PartialEq)]
pub struct Anatomy {
    pub words: Vec<String>,
    /// n x 8: each token's embedding.
    pub x: Vec<f64>,
    /// n x 16: scores, probabilities, gates.
    pub scores: Vec<f64>,
    pub p: Vec<f64>,
    pub gates: Vec<f64>,
    /// 16: how many tokens each expert got.
    pub load: Vec<f64>,
}

impl Anatomy {
    pub fn n(&self) -> usize {
        self.words.len()
    }

    /// Token t's two experts, the first (larger gate) first.
    pub fn top2(&self, t: usize) -> [usize; 2] {
        let row = &self.gates[t * EXPERTS..(t + 1) * EXPERTS];
        let mut picked: Vec<usize> = (0..EXPERTS).filter(|&e| row[e] > 0.0).collect();
        picked.sort_by(|&a, &b| row[b].total_cmp(&row[a]));
        [picked[0], picked.get(1).copied().unwrap_or(picked[0])]
    }
}

/// The program: the tables, the core, the sentence, and the arrays.
pub fn program(ids: &[usize]) -> String {
    let lits: Vec<String> = ids.iter().map(|&i| format!("{i}.0")).collect();
    format!(
        "{}{}ids := f_loor {}\nx := ids s_elect E\np := u:s_oftmax u:s_cores x\ngates := u:t_op2 p\n\
         r_avel x\nr_avel u:s_cores x\nr_avel p\nr_avel gates\nr_avel u:l_oad gates\n",
        tables(),
        core(),
        lits.join(" ")
    )
}

/// Route `sentence` through X_eTaL.
pub fn run(sentence: &str) -> Result<Anatomy, String> {
    let toks = tokens(sentence);
    if toks.len() < 2 {
        return Err("type a sentence of at least two words".into());
    }
    let ids: Vec<usize> = toks.iter().map(|t| t.1).collect();
    let n = ids.len();
    let out = output(&program(&ids), 5)?;
    Ok(Anatomy {
        words: toks.into_iter().map(|t| t.0).collect(),
        x: numbers(&out[0], n * 8)?,
        scores: numbers(&out[1], n * EXPERTS)?,
        p: numbers(&out[2], n * EXPERTS)?,
        gates: numbers(&out[3], n * EXPERTS)?,
        load: numbers(&out[4], EXPERTS)?,
    })
}
