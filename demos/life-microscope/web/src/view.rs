//! The part of the Life line that computes each stage, and the line
//! drawn decorated with it highlighted.

use microscope::source::{find, line as decorated, listing, Range, NONE};
use yew::Html;

use crate::micro::rule;
use crate::model::Stage;

/// The byte range of the rule's line that computes `stage`.
pub fn range(stage: Stage) -> Range {
    match stage {
        Stage::Board => rule().rfind("_r").map_or(NONE, |i| (i, i + 2)),
        Stage::Rotate => find(rule(), "-1 0 1 o_-_12 _r"),
        Stage::Sum => find(rule(), "'+ r_/_12"),
        Stage::Masks => find(rule(), "(_l = 3) + _r * _l = 4"),
        Stage::Next => find(rule(), "{ (_l = 3) + _r * _l = 4 } _r"),
    }
}

/// The line drawn decorated, the part computing `focus` highlighted.
pub fn line(focus: Stage) -> Html {
    decorated(rule(), range(focus))
}

/// The whole program X_eTaL ran for this board, exactly (it starts with
/// the rule's line, so the stage ranges hold), the board's data folded.
pub fn source(program: &str, focus: Stage) -> Html {
    listing(program, range(focus))
}
