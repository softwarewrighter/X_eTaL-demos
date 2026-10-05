//! The page shows exactly the program X_eTaL runs (micro's `program`,
//! the same text `run` passes to X_eTaL); nothing in it is folded, as
//! the page writes in no data, only the kernel's call.

use microscope::source::lines;
use stencil_macros_web::micro::{call, picture, program, Kernel, PRESETS};

#[test]
fn the_listing_is_the_program_run() {
    for i in 0..PRESETS.len() {
        let k = Kernel::preset(i);
        let text = program(&k);
        assert!(text.contains(picture()) && text.contains(&call(&k)));
        assert!(lines(&text).iter().all(|l| !l.folded()));
    }
}
