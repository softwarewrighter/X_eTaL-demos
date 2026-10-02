//! N-body gravity in the browser: the demo's own X_eTaL program (the
//! displacement cube, the pull, the reduce, the leapfrog step) run on
//! bodies the page keeps. `micro` is the model (tested natively); the
//! rest is the page.

pub mod app;
pub mod micro;
pub mod model;
pub mod view;
