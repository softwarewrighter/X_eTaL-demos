//! The program the page shows (the chosen scene, then the step), the
//! stages and the part computing each, and the surface's colours.

use microscope::colour::{pixels, ramp, DIVERGE};
use microscope::source::{between, block, Range};
use yew::Html;

use crate::micro::{core, SCENES};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Scene,
    Spread,
    Step,
}

pub const STAGES: [Stage; 3] = [Stage::Scene, Stage::Spread, Stage::Step];

/// The scene's X_eTaL followed by the core.
pub fn program(scene: usize) -> String {
    format!("# -- the scene: {}\n{}{}", SCENES[scene].0, SCENES[scene].1, core())
}

pub fn range(src: &str, stage: Stage) -> Range {
    match stage {
        Stage::Scene => (src.find('\n').map_or(0, |i| i + 1), src.find("# -- the Laplacian").unwrap_or(0)),
        Stage::Spread => between(src, "u:l_ap := ", "u:l_ap := "),
        Stage::Step => between(src, "drive := ", "nxt := "),
    }
}

/// The scene and the core, decorated, `focus` highlighted.
pub fn source(scene: usize, focus: Stage) -> Html {
    let src = program(scene);
    block(&src, range(&src, focus))
}

/// The surface: troughs blue, crests red, walls dark, the source violet.
pub fn surface(u: &[f64], wall: &[f64], src: &[f64]) -> Vec<u8> {
    let mut px = pixels(u, |v| ramp(&DIVERGE, 0.5 + v / 1.2));
    for i in 0..u.len() {
        let c: Option<[u8; 3]> = match (wall[i], src[i]) {
            (w, _) if w == 0.0 => Some([40, 44, 56]),
            (_, s) if s == 1.0 => Some([151, 117, 250]),
            _ => None,
        };
        if let Some(c) = c {
            px[4 * i..4 * i + 3].copy_from_slice(&c);
        }
    }
    px
}
