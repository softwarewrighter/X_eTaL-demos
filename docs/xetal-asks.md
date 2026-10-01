# Asks for X_eTaL

Features the demos need that X_eTaL does not have yet, and bugs the
demos uncovered. This repo does not change X_eTaL: each ask is filed
here (and taken to `../X_eTaL`), the demo uses the workaround noted
below or waits, and the workaround is removed when the ask lands in a
vendored release (`vendor/xetal/VENDORED`).

Each entry: status (open, filed, landed, dropped), kind (feature or
bug), which demo(s) need it, why, a minimal repro or example, and the
workaround in use.

| Status | Kind | Ask | Demos | Workaround |
| ------ | ---- | --- | ----- | ---------- |
| open | feature | Complex numbers (already planned upstream) | mandelbrot, julia | two Float planes for the real and imaginary parts |
| open | feature | Transpose (already planned upstream) | attention, embedding-explorer | to be found when the demo is written |
| open | feature | A per-operation evaluation trace (the planned stepping debugger) exposed through `xetal-play` | microscope (all) | trace per statement / named binding only |

## Details

### Complex numbers

The Mandelbrot and Julia demos iterate `z <- z*z + c` over a grid. The
research's program is `C <- complex-grid(w, h)`, `Z <- 0`, then
`Z <- Z*Z + C` with escaped cells masked. Without a complex type the
program carries the real and imaginary parts as two planes (see
`../X_eTaL/demos/classics/mandelbrot.xtl`), which hides the point of the
demo: broadcasting one arithmetic expression over the whole grid.

### Transpose

Attention is `S <- Q x transpose(K)`; PCA needs the covariance
`X^T X`. Both read naturally only with a transpose.

### Per-operation trace

The microscope shows every intermediate array of a line (for Life:
the nine rotations, their sum, the comparison masks). Statement-level
capture gives only named results. A trace of each Core application
with its value, type and shape (the source span it came from) would
let the timeline step inside a line.
