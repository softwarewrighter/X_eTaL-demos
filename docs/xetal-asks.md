# Asks for X_eTaL

Features the demos need that X_eTaL does not have yet, and bugs the
demos uncovered. This repo does not change X_eTaL: each ask is filed
here (and taken to `../X_eTaL`), the demo uses the workaround noted
below or waits, and the workaround is removed when the ask lands in a
vendored release (`vendor/xetal/VENDORED`, now X_eTaL 06d39fa).

Each entry: status (open, filed, landed, dropped), kind (feature, bug
or speed), which demo(s) need it, why, a minimal repro or example, and
the workaround in use. The last section lists language properties the
demos work with that are not asks.

| Status | Kind | Ask | Demos | Workaround |
| ------ | ---- | --- | ----- | ---------- |
| open | bug | Reading a long strand of Int literals takes quadratic time (8000 Ints: 2.1 s; as Floats: 7 ms) | ca-lab, langtons-ant (boards passed in each frame) | write Int arrays as Float literals and `f_loor` them |
| open | speed | Whole-array arithmetic is about 50 ns per element per operation (vector kernels, planned upstream) | langtons-ant (the highway needs ~10,000 steps: 20 to 30 s natively), reaction-diffusion, mandelbrot, wave-tank | small grids, a few steps per frame, the page shows each run's time |
| open | feature | `xetal-play`: pass arrays into a program and read them back without text, or keep a session between runs | every page that keeps state (reaction-diffusion, wave-tank, ca-lab, langtons-ant, nbody) | each frame writes the state as literal matrices and parses the printed `r_avel` lines |
| open | feature | Number literals with an exponent (`1.5e-7`) | mandelbrot (deep zoom), any page writing small or large Floats into a program | the page writes the shortest plain decimal that reads back exactly (`microscope::run::lit`) |
| open | feature | Complex numbers (planned upstream) | mandelbrot, julia | two Float planes (or two numbers) for the real and imaginary parts |
| open | feature | Nested arrays: `e_ach` returning a vector per item (planned upstream) | mandelbrot (the orbit table) | two `e_ach` passes, one per part |
| open | feature | A state of several arrays for `p_ower` (a tuple or record; "named records" is on the upstream wish list) | wave-tank (time), langtons-ant (direction), mandelbrot and julia (counts) | extra planes of one rank-3 array, a scalar stored in every cell |
| open | feature | A per-operation evaluation trace (the planned stepping debugger) exposed through `xetal-play` | the microscope shell (all pages) | pages print chosen intermediate arrays with `r_avel` |
| open | feature | Transpose (planned upstream) | attention, embedding-explorer (deferred) | none yet: those demos wait |
| open | bug | A vendored build reports the outer repo's commit as its own | `xetal --version` from `just xetal` | `just xetal-version` prints `vendor/xetal/VENDORED` beside it (the pages' footers read `VENDORED` directly) |

## Details

### Long Int strands are slow to read

Passing a board into a program as a literal strand is how the demos
hand state to each run. Strands of Int literals are read in quadratic
time; Float strands are not:

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
board written as Floats and floored (`b0 := f_loor b0f`), the
workaround in use there and in Langton's ant.

### Evaluator speed for whole-array arithmetic

The demos' steps are a handful of whole-array operations (rotations,
elementwise arithmetic, a reduce). Measured natively (release) through
`xetal-play`:

| Demo | Array | Per step |
| ---- | ----- | -------- |
| Langton's ant | 64 x 64, about 10 whole-board operations | 2.2 ms |
| Reaction-diffusion | 64 x 64, 2 Laplacians and the update | 3.3 ms |
| Mandelbrot | 90 x 135, one z * z + c step | about 8 ms |

That is roughly 50 ns per element per operation; the browser is
slower again. Langton's ant needs about 10,000 steps before its
highway appears: 20 to 30 seconds natively. Vector kernels for the
elementwise and rotate primitives (planned upstream) would make these
demos much more responsive and allow larger grids.

### Arrays in and out of `xetal-play` without text

`xetal_play::run(src, seed)` runs a whole program from text and
returns its printed output. A page that keeps state (a grid stepped
every frame) must therefore write the state into the program as
literal matrices and read the result back from printed `r_avel`
lines: a reaction-diffusion frame on 64 x 64 writes and parses two
4096-number strands each way. An API to bind host arrays to names
before a run and read named values after it (or a session that keeps
bindings between runs) would remove the text round trip, its cost,
and the literal-format workarounds below.

### Number literals with an exponent

A page that writes values into an X_eTaL program (a view's centre and
width) must spell every Float as an X_eTaL literal. Rust's `{:?}`
formatting uses exponent form for small values, and X_eTaL's lexer
rejects it:

```
$ xetal eval -e "1.5e-5 + 0"
error[bad-number]: a number must be followed by a space, symbol or bracket at 3..4
```

The Mandelbrot page hit this after about 16 zooms (width 3 / 2^16).
Workaround: `microscope::run::lit` writes the shortest plain decimal
that reads back exactly (`0.0000457763671875`), which X_eTaL reads at
any size. (X_eTaL itself prints Floats as plain decimals, so its
output always reads back.) Ask: accept `e`/`E` exponents in Float
literals.

### Complex numbers

The Mandelbrot and Julia demos iterate z becoming z * z + c over a
grid. With a complex type the step would be `z * z + c`; without one
the programs carry real and imaginary parts as planes of the state
(and, for a Julia set's c, as two numbers), and every product is
written out by hand. That hides the point of the demo: one arithmetic
expression broadcast over the whole grid.

### Nested arrays from `e_ach`

The Mandelbrot orbit is z0 .. zn for one c, each z a pair. The natural
program is `'{ n -> n 'u:o_rbit p_ower 0.0 0.0 } e_ach o_ffsets 11`,
but `e_ach` rejects a vector per item: `e_ach needs a single value
from each call (nested arrays come later)`. The demo runs the orbit
twice, selecting the real parts and then the imaginary parts.

### A state of several arrays

`n 'f p_ower s` iterates one value, so a simulation whose state is a
grid plus a time, or a board plus a direction, packs them into one
rank-3 array: the wave tank keeps the time as a third plane (the same
number in every cell, read back with `f_irst r_avel`), Langton's ant
keeps its direction the same way, and the Mandelbrot and Julia states
carry a count plane. A tuple or record type (the wish list's "named
records") would let a state be `(board; time)`.

### Per-operation trace

The microscope pages show the intermediate arrays of a step by
printing chosen expressions (`r_avel` of each), which needs the page
to know which expressions matter. A trace of each Core application
with its value, type, shape and source span, exposed through
`xetal-play`, would let a page step inside any line.

### Transpose

Attention is `S <- Q x transpose(K)`; PCA needs the covariance
`X^T X`. Both read naturally only with a transpose; the two demos are
deferred until it exists (`docs/plan.md`, saga 4).

### Build provenance in a vendored build

`xetal-cli`'s `build.rs` takes the commit from
`git rev-parse --short HEAD` in the directory being built. Built from
`vendor/xetal/` inside this repo, `xetal --version` reports this
repo's commit, not the X_eTaL commit it was built from. Repro: copy a
committed X_eTaL tree into another git repository, build `xetal-cli`,
run `xetal --version`. Ask: let an environment variable (for example
`XETAL_BUILD_SHA`) override the git lookup. The demo pages are not
affected: their footer reads the vendored commit from `VENDORED`.

## Language properties the demos work with (not asks)

These are X_eTaL design decisions, not gaps; the demos are written
around them and say so where it shows.

- **Two arguments at most.** A function is monadic or dyadic, and
  adjacent arguments form a strand (`u:f 1 2` passes the vector
  `1 2`). Julia's iteration is therefore `c u:i_terate z0`, with the
  step count a variable bound before it.
- **Names are bound in order.** A function sees only names bound
  before it, so a program's parameters come before its functions
  (the pages put theirs first).
- **No implicit Bool or Int to Float.** `2.5 * (x > 1)` is a type
  error; the demos write `f_loat (x > 1)`. Bools mix with Ints
  (`1 * (b = 1)` is an Int array) and sum (`'+ r_/` over Bools), and
  an Int state needs Int, not Bool, values.
- **`m_od` takes the dividend on the left** (`a m_od b` is a mod b),
  unlike APL's residue.
- **Rotation wraps.** `o_-` rotates, so stencils see a torus; the wave
  tank damps its edges (a sponge) so waves leave instead of wrapping.
