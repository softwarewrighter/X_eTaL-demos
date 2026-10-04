//! The page shows exactly the program X_eTaL runs (micro's `program`,
//! the same text `run` passes to X_eTaL), folding only the data the
//! page wrote in, never a line of the demo's own .xtl.

use fourier_epicycles_web::micro::{core, preset, program};
use microscope::source::lines;

#[test]
fn the_listing_is_the_program_run_and_folds_only_data() {
    let p = program(&preset(2));
    assert!(p.contains(core()));
    let folded: Vec<&str> = lines(&p).iter().filter(|l| l.folded()).map(|l| &p[l.raw.0..l.raw.1]).collect();
    assert_eq!(folded.len(), 2, "the curve's x and y");
    assert!(folded.iter().all(|l| !core().contains(l.trim())));
}
