//! The page shows exactly the program X_eTaL runs (micro's `program`,
//! the same text `run` passes to X_eTaL), folding only the data the
//! page wrote in, never a line of code.

use eigencube_web::micro::{program, solve_program, IMPORT};
use microscope::source::lines;

fn check(p: &str) {
    assert!(p.starts_with(IMPORT), "the program imports the library");
    for l in lines(p).iter().filter(|l| l.folded()) {
        let text = &p[l.raw.0..l.raw.1];
        assert!(text.trim_start().starts_with("ms := "), "a line of code is folded: {}", &text[..text.len().min(60)]);
    }
}

#[test]
fn the_listing_is_the_program_run_and_folds_only_data() {
    check(&program(&[1, 9, 4, 12, 7]));
    check(&solve_program(&[1, 9, 4, 12, 7]));
}
