//! The page shows exactly the program X_eTaL runs (micro's `program`,
//! the same text `run` passes to X_eTaL), folding only the data the
//! page wrote in, never a line of the demo's own .xtl.

use nbody_web::micro::{core, program, PRESETS};
use microscope::source::lines;

fn check(program: &str, code: &str) {
    assert!(program.contains(code), "the program holds the demo's code");
    for l in lines(program).iter().filter(|l| l.folded()) {
        let text = &program[l.raw.0..l.raw.1];
        assert!(!code.contains(text.trim()), "a line of the demo's code is folded: {}", &text[..text.len().min(60)]);
    }
}

#[test]
fn the_listing_is_the_program_run_and_folds_only_data() {
    check(&program(&(PRESETS[2].bodies)(), &PRESETS[2].physics, 1), core());
}
