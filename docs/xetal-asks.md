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
| open | feature | Nested arrays: `e_ach` returning a vector per item | mandelbrot (orbit table) | two `e_ach` passes, one per part |
| open | feature | Number literals with an exponent (`1.5e-7`) | mandelbrot (deep zoom), any demo passing small or large Floats into a program | the host writes plain decimals with 17 significant digits |
| open | feature | Transpose (already planned upstream) | attention, embedding-explorer | to be found when the demo is written |
| open | bug | A vendored build reports the outer repo's commit as its own | all (`xetal --version`, the web footer) | `just xetal-version` prints `vendor/xetal/VENDORED` beside it |
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

### Build provenance in a vendored build

`xetal-cli`'s and `xetal-chrome`'s `build.rs` take the commit from
`git rev-parse --short HEAD` in the directory being built. Built from
`vendor/xetal/` inside this repo, `xetal --version` says
`Commit: 486edcd` (this repo's commit), not the X_eTaL commit it was
built from (06d39fa). The same would show in a demo's footer.

Repro: copy a committed X_eTaL tree into another git repository, build
`xetal-cli`, run `xetal --version`.

Ask: let an environment variable (for example `XETAL_BUILD_SHA`)
override the git lookup in both `build.rs` files, so a vendoring repo
can pass the vendored commit.

### Nested arrays from `e_ach`

The Mandelbrot orbit is z0 .. zn for one c, each z a pair. The natural
program is `'{ n -> n 'u:o_rbit p_ower 0.0 0.0 } e_ach o_ffsets 11`,
but `e_ach` rejects a vector per item: `e_ach needs a single value
from each call (nested arrays come later)`. The demo runs the orbit
twice, selecting the real parts and then the imaginary parts. Nested
arrays are already planned upstream.

### Number literals with an exponent

A page that writes values into an X_eTaL program (a view's centre and
width) must spell every Float as an X_eTaL literal. Rust's shortest
float formatting switches to exponent form for small values, and
X_eTaL's lexer rejects it:

```
$ xetal eval -e "1.5e-5 + 0"
error[bad-number]: a number must be followed by a space, symbol or bracket at 3..4
```

The Mandelbrot page hit this after about 16 zooms (width 3 / 2^16).
Workaround: the page writes plain decimals (`0.000045776367187500`),
which X_eTaL reads correctly at any size. Ask: accept `e`/`E`
exponents in Float literals (and print very large or small Floats
the same way, so output can be read back).
