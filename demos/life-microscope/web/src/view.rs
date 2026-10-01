//! The part of the Life line that computes each stage, and the line
//! drawn decorated with it highlighted.

use microscope::source::{find, line as decorated, Range, NONE};
use yew::Html;

use crate::micro::LINE;
use crate::model::Stage;

/// The byte range of LINE that computes `stage`.
pub fn range(stage: Stage) -> Range {
    match stage {
        Stage::Board => LINE.rfind("_r").map_or(NONE, |i| (i, i + 2)),
        Stage::Rotate => find(LINE, "-1 0 1 o_-_12 _r"),
        Stage::Sum => find(LINE, "'+ r_/_12"),
        Stage::Masks => find(LINE, "(_l = 3) + _r * _l = 4"),
        Stage::Next => find(LINE, "{ (_l = 3) + _r * _l = 4 } _r"),
    }
}

/// The line drawn decorated, the part computing `focus` highlighted.
pub fn line(focus: Stage) -> Html {
    decorated(LINE, range(focus))
}
