//! The page shows exactly the program X_eTaL runs (micro's `program`,
//! the same text `run` passes to X_eTaL), folding only the data the
//! page wrote in, never a line of the demo's own .xtl.

use microscope::source::lines;
use sandpile_web::micro::{core, program, Pile, MID};

#[test]
fn the_listing_is_the_program_run_and_folds_only_data() {
    let mut p = Pile::empty();
    p.drop(MID, MID, 100);
    let text = program(&p, 3);
    assert!(text.contains(core()));
    let folded: Vec<&str> = lines(&text).iter().filter(|l| l.folded()).map(|l| &text[l.raw.0..l.raw.1]).collect();
    assert_eq!(folded.len(), 2, "the grains and the topple counts");
    assert!(folded.iter().all(|l| !core().contains(l.trim())));
}
