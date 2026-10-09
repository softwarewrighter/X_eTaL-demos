//! The page shows exactly the program X_eTaL runs (micro's `program`,
//! the same text `run` passes to X_eTaL), folding only the data the
//! page wrote in, never a line of the demo's own .xtl.

use eigencube_web::micro::{core, program};
use microscope::source::lines;

#[test]
fn the_listing_is_the_program_run_and_folds_only_data() {
    let p = program(&[1, 9, 4, 12, 7]);
    assert!(p.contains(core()), "the program holds the demo's code");
    for l in lines(&p).iter().filter(|l| l.folded()) {
        let text = &p[l.raw.0..l.raw.1];
        assert!(!core().contains(text.trim()), "a line of the demo's code is folded: {}", &text[..text.len().min(60)]);
    }
}
