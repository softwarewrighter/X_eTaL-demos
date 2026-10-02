//! The image pipeline in the browser: the demo's own X_eTaL program
//! (windows, filters, Sobel edges, threshold, pooling) run on pictures
//! and kernels the page sets. `micro` is the model (tested natively);
//! the rest is the page.

pub mod app;
pub mod micro;
pub mod model;
pub mod view;
