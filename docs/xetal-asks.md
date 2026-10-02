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
| open | bug | Reading a long strand of Int literals takes quadratic time (8000 ints: 2.1 s; the same as Floats: 7 ms) | ca-lab (a 48 x 64 board per frame), any page passing Int arrays in | write Int arrays as Float literals and `f_loor` them |
| open | feature | Evaluator speed for whole-array arithmetic (vector kernels, planned upstream) | langtons-ant (2.2 ms a step on 64 x 64; the highway needs ~10,000 steps), reaction-diffusion | fewer steps per frame; smaller grids |
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

### Long Int strands are slow to read

Passing a board into a program as a literal strand is the demos' way
to hand state to each run. Strands of Int literals are read in
quadratic time; Float strands are not:

| Literal strand | Time (`xetal run`, release) |
| -------------- | --------------------------- |
| 1000 Ints | 0.03 s |
| 2000 Ints | 0.11 s |
| 4000 Ints | 0.49 s |
| 8000 Ints | 2.10 s |
| 8000 Floats (`0.0 1.0 ...`) | 0.007 s |

Repro: `python3 -c "print('b := ' + ' '.join(str(i % 2) for i in
range(8000)) + '\nt_ally b')" > ints.xtl; time xetal run ints.xtl`.

The cellular automata lab's 2-D step (a 48 x 64 board, 4 steps) took
283 ms through `xetal-play` with an Int strand and 16 ms with the
board written as Floats and floored (`b0 := f_loor b0f`), which is the
workaround in use.

### Evaluator speed for whole-array arithmetic

The demos' steps are a handful of whole-array operations (rotations,
elementwise arithmetic, a reduce). Measured natively (release) through
`xetal-play`:

| Demo | Array | Per step |
| ---- | ----- | -------- |
| Langton's ant | 64 x 64, about 10 whole-board operations | 2.2 ms |
| Reaction-diffusion | 64 x 64, 2 Laplacians and the update | 3.3 ms |
| Mandelbrot | 90 x 135, one z * z + c step | about 8 ms |

That is roughly 50 ns per element per operation. Langton's ant needs
about 10,000 steps before its highway appears: 20 to 30 seconds
natively, longer in the browser. Vector kernels for the elementwise
and rotate primitives (already planned upstream) would make these
demos much more responsive.
