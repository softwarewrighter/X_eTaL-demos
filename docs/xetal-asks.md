# Asks for X_eTaL

Features the demos need that X_eTaL does not have yet, and bugs the
demos uncovered. Asks marked X_eTaL-ML came from the machine-learning
demos, which moved there; they stay here as the record until X_eTaL-ML
carries them. This repo does not change X_eTaL: each ask is filed
here (and taken to `../X_eTaL`), the demo uses the workaround noted
below or waits, and the workaround is removed when the ask lands in the
pinned release (`XETAL_COMMIT`, now X_eTaL 1998414, 206 commits
after v0.1.0, pinned 2026-10-08 for tuple patterns; every ask was
re-checked against 9c667a3 on 2026-10-07, and only the tuple ask
against 1998414). X_eTaL keeps its own
ledger of these asks (its `docs/asks.md`, D1 to D9).

Each entry: status (open, filed, landed, dropped), kind (feature, bug
or speed), which demo(s) need it, why, a minimal repro or example, and
the workaround in use. The last section lists language properties the
demos work with that are not asks.

| Status | Kind | Ask | Demos | Workaround |
| ------ | ---- | --- | ----- | ---------- |
| landed (Saga 30: 16b710d, 8456e45) | speed (regression) | `t_able` and `i_nner` were 2.7x and 1.5x slower at abb8274 than at 06d39fa (and `i_nner` 20% slower again at 081fb3f); at 1c1617e they are faster than ever: against the 081fb3f baseline `t_able` -82%, `i_nner` -86%, nbody -52%, image-pipeline -55% | nbody, image-pipeline, langtons-ant (and X_eTaL-ML's) | none needed now |
| landed (abb8274) | bug | Reading a long strand of Int literals took quadratic time (8000 Ints: 2.1 s; now 0.00 s) | ca-lab, langtons-ant (boards passed in each frame), moe-router (X_eTaL-ML) | removed in ca-lab and langtons-ant: boards go in as Int literals (moe-router (X_eTaL-ML)'s word numbers are X_eTaL-ML's) |
| partly landed (abb8274) | speed | Whole-array arithmetic was about 50 ns per element per operation; elementwise arithmetic is now about 8x faster (see the regression above for `t_able` and `i_nner`) | langtons-ant (the highway needs ~10,000 steps), reaction-diffusion, mandelbrot, wave-tank | small grids, a few steps per frame, the page shows each run's time |
| partly landed (between 1c1617e and 882aa76) | speed | A scan with a built-in operand was quadratic in the axis length (each running reduce afresh). At 882aa76 an Int scan is linear (64 x 4096 `'+ s_\\_2`: 0.05 s), but a Float scan is still quadratic (64 x 512 0.22 s, 64 x 4096 14.5 s) | (none yet; `tools/bench`) | short axes |
| landed (Saga 30) | speed | `i_nner` (matrix product) cost about 370 ns per multiply-add; at 1c1617e the bench's (32 x 256) by (256 x 32) product takes 29 ms, 7x faster | X_eTaL-ML's | none needed now |
| open | feature | Grade along an axis per row (`g_rade_2 M` grades the columns as items, one vector, not each row), for top-k per row | moe-router (X_eTaL-ML) (top-2 experts per token) | the largest by `'m_ax r_/_2`, masked out, then the largest again |
| open | feature | `xetal-play`: pass arrays into a program and read them back without text, or keep a session between runs | every page that keeps state (reaction-diffusion, wave-tank, ca-lab, langtons-ant, nbody) | each frame writes the state as literal matrices and parses the printed `r_avel` lines |
| landed (abb8274) | feature | Number literals with an exponent (`1.5e-7`) | mandelbrot (deep zoom), any page writing small or large Floats into a program | removed: `microscope::run::lit` writes the shortest form, with an exponent when small |
| open | feature | Complex numbers (planned upstream) | mandelbrot, julia | two Float planes (or two numbers) for the real and imaginary parts |
| partly landed (abb8274) | feature | Nested arrays: a vector per item. `m_ap` now gives each result boxed, but nothing turns a list of boxes back into a matrix (APL's mix) | mandelbrot (the orbit table) | still two `e_ach` passes, one per part |
| landed (175acf5, X_eTaL PR #111; pinned at 1998414) | feature | A state of several arrays for `p_ower`: tuples `(board, time)`, taken apart by patterns in bindings and lambda parameters | wave-tank (time), langtons-ant (direction); removed from mandelbrot and julia (counts) | still in place in the first two (extra planes of one rank-3 array, a scalar stored in every cell), to be removed demo by demo |
| open | feature | A per-operation evaluation trace (the planned stepping debugger) exposed through `xetal-play` | the microscope shell (all pages) | pages print chosen intermediate arrays with `r_avel` |
| landed (abb8274) | feature | Transpose (`o_\`, and `t_ranspose` with a permutation) | image-pipeline (ky from kx), attention and embedding-explorer (X_eTaL-ML) | removed in image-pipeline: `ky := o_\ kx` |
| landed (ffd5526) | bug | A condition bound to a name could not be used in arithmetic (`a := 1 2 > 0` then `1 * a`, `f_loat a` or `'+ r_/ a`); a bound condition now stays Bool and converts as T1 says | image-pipeline (masks) | removed: masks are bound as conditions and made Floats where used (`0.7 * f_loat disk`) |
| landed (4a31ba9; moot since 2026-10-05) | bug | A vendored build reported the outer repo's commit as its own | `xetal --version` from `just xetal` | removed: X_eTaL is now built from a real clone (`work/xetal/` at `XETAL_COMMIT`), which reports its own commit; `scripts/check-xetal.sh` checks it |
| open | feature | `xetal-play`: a page's own macro library and the expansion: no way to give a run a library of the page's own (`.xtl` or `.xtlm`), nor to get the program after expansion (`xetal expand`) | stencil-macros (its page expands the kernel's call with `Stencil.xtlm`) | the page depends on `xetal-store` (installs memory, writes the library), `xetal-macro` (`StoreLibraries`) and `xetal-program` (`expanded_with`) directly |
| open | bug | `xetal run --context CTX FILE` reads standard input twice: FILE's `[]R_EAD` runs once silently and again for the shown run (with input `hello`, `world` the program `l := []R_EAD @` then `l` prints `world`; with one line it fails: no more input) | xetal-pipes (it would carry the stage's arguments) | a copy of the stage with the arguments' line put in front |
| open | feature | End of input: `[]R_EAD @` at the end of standard input fails (`error[io]: []R_EAD: no more input`) and nothing can test for it or recover, so a program cannot read a stream of unknown length | xetal-pipes (every stage reads standard input) | the stage's wrapper appends an end line after the input; the stage reads until it |
| open | feature | Program arguments: `xetal run FILE` takes nothing after the file, and a program cannot read its arguments | xetal-pipes (`head -n 3`, `grep NEEDLE`, `cat FILE`) | the wrapper runs a copy of the stage with `args := "..."` put in front, its library found through `XETAL_PATH` (`--context` reads standard input twice, below) |
| open | feature | `s_ort` (and `g_rade`) of a list of boxed strings: `expected a number or Char, found Box Char`; `u_nique` and `m_atch` take boxes, sorting does not | xetal-pipes (`sort`, `uniq`) | lines padded into a Char matrix (built by `t_able` indexing, as mix, D6, is missing); its rows graded |
| landed (24ec3bae, D127; checked at 1998414) | bug | The `deprecated-private` lint (PN2) now warns on a bare top-level function of a macro library too, not only an ordinary one; X_eTaL's own `lib/Combinators.xtlm` and `lib/System.xtlm` are migrated to `h:` with it | stencil-macros (`Stencil.xtlm`; already migrated here, on the convention, before the lint asked for it) | none needed |

## For the X_eTaL agent: ledger corrections

Checked on 2026-10-05 against X_eTaL 882aa76, whose `docs/asks.md`
(generated from `docs/asks.toml`) lists this repo's asks as D1 to
D14. Its D rows disagree with this file as follows:

| Ask | Ledger says | Should say |
| --- | ----------- | ---------- |
| D11 (`t_able` / `i_nner` regression) | planned (Saga 30, started) | landed: 16b710d, 8456e45; at 1c1617e `t_able` -82%, `i_nner` -86% against the 081fb3f baseline |
| D12 (`i_nner` 370 ns per multiply-add) | planned (Saga 30, then Saga 22) | landed for this repo: the (32 x 256) by (256 x 32) product takes 29 ms, 7x faster; Saga 22's vector kernels are welcome but no longer asked |
| D10 (provenance) | landed (4a31ba9) | landed; also moot here: since 2026-10-05 this repo pins a commit and builds a clone (`docs/vendoring.md`), which reports its own commit |
| (missing) D15 | - | new or partly landed: a scan with a built-in operand is quadratic. Ints are linear at 882aa76, Floats are not (64 x 4096 `'+ s_\_2`: 14.5 s). Repro: `x := (64 c_at 4096) r_eshape 0.5 0.25` then `'+ s_\_2 x`. See "Scan is quadratic" below |
| M3 (X_eTaL-ML, same as D11) | planned | landed, as D11 |

With those, the summary row for X_eTaL-demos becomes 7 landed,
3 partly landed (D2, D6, D15), 5 planned (D3, D5, D7, D8, D13),
0 declined, 0 new. D13 (grade per row) and the mix half of D6 are
still "to decide with the user": those need the user's call in
X_eTaL, not more work here.

## For the X_eTaL agent: a page's own macro library

Found 2026-10-05 against 512b3ee (v0.1.0), building the stencil-macros
page.

What: `xetal-play`, the engine for pages, runs a program whose
libraries come from the store the host installed, but it neither
installs one nor offers a way to add a library to it, and it has no
counterpart of `xetal expand`. A page with a macro library of its own
(here `Stencil.xtlm`, beside the demo's `.xtl`) cannot run or show
its expansion through `xetal-play` alone.

Why: a macro is only half visible when it runs: what makes the demo
is seeing what the call became. Every downstream page that ships a
library (the demos, X_eTaL-libraries' site, the games) needs the same
two calls.

Suggested: `xetal_play::add_library(name, text)` (into the installed
store, or a store of its own if none) and `xetal_play::expanded(src)
-> Result<String, String>`. Workaround here: the page depends on
`xetal-store`, `xetal-macro` and `xetal-program` by path, installs a
`Memory` store once, writes `Stencil.xtlm` into it, and calls
`xetal_program::expanded_with(name, src, &StoreLibraries)`.

## For the X_eTaL agent: four asks for programs in a Unix pipe

Found 2026-10-05 against 882aa76 (still so at 512b3ee and at 9c667a3),
building
`xetal-pipes` (now live at the command line; each workaround is in its
README): X_eTaL
programs as Unix pipeline stages (`xetalcat FILE | xetaluniq |
xetalwc -l`, `xetalcat FILE | xetalgrep NEEDLE | xetalsort |
xetalhead -n 3`), each stage an `.xtl` program reading standard input
and printing lines. X_eTaL's plan has none of the three (D50, the
steppable evaluator, is about waiting for typed lines, not the end of
a stream).

### 1. End of input (feature)

What: `[]R_EAD @` gives the next line of standard input; after the
last one it fails, and the failure cannot be caught or tested first.

```
$ printf 'abc\nde\n' | xetal run read3.xtl    # three []R_EAD @ in a row
abc
de
error[io]: []R_EAD: no more input at 72..81
```

Why: a filter (cat, grep, sort, uniq, wc) reads until its input ends,
and does not know how many lines are coming. Every Unix-style use of
X_eTaL needs this.

Suggested (X_eTaL's to decide): a value at the end of input that a
program can test (an empty box, or a `[]E_OF` niladic), or a reader of
all of standard input as one text (`[]R_EAD_ALL @`, then `p_artition`
on newlines), which also suits arrays better than a line at a time.
Workaround: each stage's wrapper (`bin/xetal-stage`) writes an end
line after the real input, and the stage reads until it.

### 2. Program arguments (feature)

What: `xetal run FILE` accepts no arguments after the file, and no
system name gives a program its arguments.

```
$ xetal run head.xtl -n 3
error: unexpected argument '-n' found
```

Why: `head -n 3`, `grep NEEDLE` and `cat FILE` are parameterized by
their arguments; so is every script with a shebang (`#!/usr/bin/env
xetal`), which the CLI already supports.

Suggested: `xetal run FILE ARGS...` (and `xetal FILE ARGS...`), the
arguments read as a list of boxed texts (`[]A_RGS`, say). Workaround:
the wrapper runs a copy of the stage with `args := "-n 3"` put in
front of it, and sets `XETAL_PATH` to the stages' directory so the
copy finds their library. (`xetal run --context` was the plan, but it
reads standard input twice: ask 4.)

### 3. Sorting text (feature)

What: `s_ort` and `g_rade` refuse a list of boxed strings, the natural
form of a list of lines (`p_artition` gives it, and `u_nique` and
`m_atch` accept it).

```
$ xetal eval -e 's_ort ("b a" != f_irst " ") p_artition "b a"'
error[type-mismatch]: expected a number or Char, found Box Char
```

Why: sorting lines (`sort`, then `uniq -c`, top-k words) is the core
of text processing.

Suggested: order boxes by their contents (lexicographically, as APL2
and Dyalog order nested arrays). Workaround: pad the lines into a Char
matrix (by `t_able` indexing into the text, since mix is missing, D6)
and grade its rows, which `g_rade` does.

### 4. `--context` reads standard input twice (bug)

What: `xetal run --context CTX FILE` runs CTX silently, then FILE as
its continuation, but FILE's reads of standard input happen twice.

```
$ echo 'args := ""' > ctx.xtl
$ printf 'l := []R_EAD @\nl\n' > read.xtl
$ printf 'hello\n' | xetal run read.xtl
hello
$ printf 'hello\n' | xetal run --context ctx.xtl read.xtl
error[io]: []R_EAD: no more input at 16..25
$ printf 'hello\nworld\n' | xetal run --context ctx.xtl read.xtl
world
```

Why: the continuation seems to be run once silently along with the
context, then again for its output, so every effect in it (input,
and likely files written with `[]N_PUT`) happens twice. Org-babel
sessions, its purpose, rarely read input, which may be why it was not
seen.

Suggested: run the context alone silently, then the file once.
Workaround: not using `--context` (see ask 2).

## The deprecated-private lint now covers .xtlm (landed)

Found 2026-10-07 against 9c667a3 (Saga 38 step 4, PN2), migrating
`Stencil.xtlm` to `h:` on the new convention; landed 2026-10-08
(24ec3bae, D127, checked at 1998414): a bare top-level function in a
macro library now warns `deprecated-private` too, the same as an
ordinary library, and X_eTaL's own `lib/Combinators.xtlm` and
`lib/System.xtlm` are migrated to `h:` with it. The ask as filed:

What: a bare top-level function in an ordinary library (`.xtl`) warns
`deprecated-private`; the same in a macro library (`.xtl` text loaded
as `.xtlm`) does not.

```
$ cat Stencil.xtlm
s_ide := { n k -> n <= k * k ? k; n s_ide k + 2 }
m:t_encil< := { kernel name -> ... n s_ide 1 ... }
$ echo '"s:" u_se< "Stencil"' | xetal run -
(no warning)
$ xetal type Stencil.xtlm
m:t_encil< : Char -> Char -> Char
(no warning either, though xetal migrate Stencil.xtlm rewrites s_ide to h:s_ide correctly)
```

Why: the two kinds of library disagreed on whether a bare helper is
flagged; a macro library's authors got no nudge toward `h:`.

Workaround while open: none needed -- `Stencil.xtlm` was migrated to
`h:` on the convention alone, not because the lint asked for it.

## For the X_eTaL agent: four asks not in X_eTaL's plan yet

Checked against `../X_eTaL/docs/plan.md` at 2bd6e7d (2026-10-03): the
speed regression (Saga 30), records (Saga 29), host arrays, the trace
and complex numbers are planned; these four are not. Each says what,
why, a repro at abb8274, and a suggested shape (the design is
X_eTaL's to decide). The user has relayed them as well.

### 1. A bound condition in arithmetic (bug, or a doc fix)

Landed in ffd5526 (vendored here at 081fb3f): a bound condition stays
Bool and converts in arithmetic; image-pipeline binds its masks as
conditions again. The ask as filed:

What: a comparison bound to a name cannot be used where a number is
expected, though the same comparison written inline can.

```
$ xetal eval -e "a := 1 2 > 0
1 * a"
error[type-mismatch]: expected a number, found Bool at 13..18
$ xetal eval -e "a := 1 2 > 0
f_loat a"
error[type-mismatch]: expected a number, found Bool at 13..21
$ xetal eval -e "f_loat 1 2 > 0"
1.0 1.0
```

Why: lang-choices T1 says a Bool converts to Int in arithmetic, and
T5 says a top-level condition defaults to Bool when bound; together a
reader expects `1 * a` to work. Masks are the array idiom (the image
pipeline builds its picture from named masks: a disk, a square, a
triangle), and naming them is natural.

Suggested: let a bound Bool convert in arithmetic (and `f_loat`) as
T1 describes; or, if the default is meant to stop it, say so in T5
and in the error ("a Bool bound by name: write `f_loat (...)` when
binding"). Workaround here: `disk := f_loat (...) < 300`.

### 2. Grade (and sort) of each row along an axis (feature)

What: `g_rade_2 M` grades the columns as whole items and returns one
vector; there is no grade of each row.

```
$ xetal eval -e "M := 2 4 r_eshape 0.1 0.5 0.3 0.9 0.7 0.2 0.8 0.1
g_rade_2 M"
1 3 2 4
```

Why: top-k along a row is everyday in array ML (a router's top-2
experts per token, nearest neighbors, beam search). Today it takes
masks: the largest of each row (`'m_ax r_/_2`), taken out, then the
largest again; k passes for top-k.

Suggested: a rank-preserving grade under an axis subscript (each
vector along that axis graded on its own: `g_rade_2 M` is 2 x 4, so
`2 t_ake_2 g_rade_2 n_eg M` is each row's top 2), or a rank operator
that applies `g_rade` to each row. Needed by X_eTaL-ML's moe-router.

### 3. Mix: boxes back into one array (feature)

What: `'f m_ap v` (new at abb8274) gives each result boxed, but
nothing turns a list of boxes into one array; `d_isclose` opens one
box only.

```
$ xetal eval -e "d_isclose 'r_ange m_ap 1 2 3"
error[rank]: d_isclose opens one box, got shape 3
```

Why: the Mandelbrot demo's orbit table is z0 .. zn for one point, each
z a pair (real, imaginary). With mix it is one pass:
`mix '{ n -> n 'u:o_rbit p_ower 0.0 0.0 } m_ap o_ffsets 11` (an 11 x 2
table). Without it the demo runs the orbit twice, selecting the real
parts and then the imaginary parts with `e_ach`.

Suggested: APL2's mix (disclose of a list of boxes into an array one
rank higher, padding shorter items), perhaps as `d_isclose` on a list
or a new name, and its inverse split (each row boxed). Workaround
here: two `e_ach` passes.

### 4. A vendored build reports the outer repository's commit (bug)

Landed in 4a31ba9 (`XETAL_BUILD_SHA`), and moot here since 2026-10-05:
this repo no longer tracks a copy of X_eTaL but builds a clone of it at
`XETAL_COMMIT` (X_eTaL's `docs/vendoring.md`), and a clone reports its
own commit. The ask as filed:

What: `xetal-cli`'s `build.rs` takes the commit from
`git rev-parse --short HEAD` in the directory it builds in. Built from
`vendor/xetal/` inside this repository, `xetal --version` reports this
repository's commit, not X_eTaL's.

```
$ just xetal && target/xetal/release/xetal --version
X_eTaL 0.1.0
...
  Commit: d93828a        <- X_eTaL-demos' commit, not abb8274
```

Why: every consumer vendors X_eTaL (demos, games, libraries,
extensions, ML); a bug report from any of them quotes the wrong
commit.

Suggested: let an environment variable (for example
`XETAL_BUILD_SHA`) override the git lookup, and/or read a `VENDORED`
file next to the sources when present (the vendor scripts write one).
Workaround here: `just xetal-version` prints `vendor/xetal/VENDORED`
beside the binary's version; the pages' footers read `VENDORED`.

## Details

### Scan is quadratic

Partly landed at 882aa76: an Int scan with a built-in operand is now
linear (64 x 4096 `'+ s_\_2` in 0.05 s), but a Float scan is still
quadratic (64 x 4096 in 14.5 s, 64 x 512 in 0.22 s). Float `+` is
not exactly associative, so a running fold can differ from APL's
right-to-left definition in the last bits; this repo would accept
that (or a documented choice either way). The ask as filed:

`'f s_\ v` is every running reduce, and each is computed from the
start (right to left, as APL defines a scan for any function), so a
scan along an axis of length n costs about n * n / 2 applications.
For `+`, `*`, `m_ax`, `m_in`, `&` and `|` (associative), a running
fold gives the same results in n steps. At abb8274 (06d39fa is the
same):

| `'+ s_\_2` on | Time |
| -------------- | ---- |
| 64 x 128 | 0.04 s |
| 64 x 256 | 0.06 s |
| 64 x 512 | 0.22 s |
| 512 x 512, twice | 2.5 s |

Repro: `x := (64 c_at 512) r_eshape 0.5 0.25` then `'+ s_\_2 x`.
Ask: a linear scan for the associative built-ins (the result is
identical). Found by `tools/bench`, whose scan case now uses short
rows.

### Speed regression in `t_able` and `i_nner` (06d39fa to abb8274)

Landed in X_eTaL's Saga 30 (16b710d: a built-in operand is computed
directly; 8456e45: lean kernels for function operands), vendored here
at 1c1617e: `just bench-check` against the 081fb3f baseline gave
`t_able` -82%, `i_nner` -86%, nbody -52%, image-pipeline -55%,
langtons-ant -41%, nothing slower; nbody's 10 steps take 27 ms (38 ms
at 06d39fa). The ask as filed:

Refreshing the vendored X_eTaL from 06d39fa to abb8274 (2026-10-03)
made the higher-order built-ins `t_able` and `i_nner` slower, while
elementwise arithmetic got much faster (the upstream commits include
cd80454, "hof: higher-order built-ins as kernels, steps of the machine
(D50)"). Both builds release, the same machine, best of runs:

| Program (each line run 4 times) | 06d39fa | abb8274 |
| ------------------------------- | ------- | ------- |
| `y := x + x * x` on 512 x 512 | 0.41 s | 0.05 s |
| `y := (o_ffsets 512) 'r_ight t_able r_avel 1 t_ake x` (512 x 512) | 0.11 s | 0.30 s |
| `y := (r_avel 1 t_ake x) '* t_able r_avel 1 t_ake x` (512 x 512) | 0.12 s | 0.30 s |
| `y := ((64 c_at 512) r_eshape x) '+ '* i_nner (512 c_at 64) r_eshape x` | 4.08 s | 5.98 s |
| `'+ r_/_2 x`, `'+ r_/ x`, `1 o_-_2 x` | same | same |

with `x := (512 c_at 512) r_eshape 0.5 0.25 0.125`. Effect on the
demos (natively, through `xetal-play`):

| Demo run | 06d39fa | abb8274 |
| -------- | ------- | ------- |
| ternary-net (X_eTaL-ML), the FP32/FP16/INT8 maps | 0.64 s | 1.40 s |
| ternary-net (X_eTaL-ML), the ternary map | 0.22 s | 0.44 s |
| nbody, 50 bodies, 10 steps | 38 ms | 67 ms |
| image-pipeline, 96 x 96 | 70 ms | 122 ms |

In the browser (the live pages, Chrome, WebAssembly) the slow-down is
larger and erratic: image-pipeline took a steady 130 ms at 06d39fa,
and at abb8274 971 ms on the first run, then 136, 570 and 616 ms for
three threshold changes; ternary-net's formats went from about 550 ms
to 2224 ms. Something beyond the native regression (memory growth,
or a path the WebAssembly build takes) seems to be involved.

At 081fb3f (vendored here 2026-10-03, after the higher-order cost
guard, e3e9c3d) `i_nner` is about 20% slower again: four
(32 x 256) by (256 x 32) products took 0.71 s at abb8274 and 0.85 s at
081fb3f (`xetal run`, best of three); `just bench-check` flags `inner`
(+16%) and `transpose` (+23%, on a 5 ms case, so less certain), the
pages within 10%. Saga 30 is expected to remove both.

Repro: save the lines above as a program, run it with `xetal run`
built at each commit. Ask: bring `t_able` and `i_nner` back to (or
past) their 06d39fa speed; a benchmark of both in the speed saga would
keep them there.

### Long Int strands are slow to read

Landed in abb8274 (8000 Ints read in 0.00 s); ca-lab and langtons-ant
now pass their boards as Int literals. The history:

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

Landed in abb8274 (`1.5e-7`, `2.5E3` read); `microscope::run::lit`
now writes Rust's shortest form. The history:

A page that writes values into an X_eTaL program (a view's center and
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

Partly landed in abb8274: `'f m_ap v` gives each result boxed
(`'r_ange m_ap 1 2 3`), but there is no mix to turn the boxes back into
a matrix, so the Mandelbrot orbit still takes two passes. Ask: a mix
(disclose every box into one array, padding as APL2 does). The
history:

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

Landed in 175acf5 (X_eTaL PR #111), pinned here at 1998414: a tuple
`(board, time)` is one value, `p_ower` iterates it, and a pattern
takes it apart, in a binding (`(b, t) := s`) or a lambda's parameter
(`{ (b, t) -> ... }`). Mandelbrot and Julia now carry their state as
`(zr, zi, counts)`, taken apart by patterns on both sides of
`u:s_tep`/`u:i_terate`; wave-tank and Langton's ant still carry their
planes.

### Per-operation trace

The microscope pages show the intermediate arrays of a step by
printing chosen expressions (`r_avel` of each), which needs the page
to know which expressions matter. A trace of each Core application
with its value, type, shape and source span, exposed through
`xetal-play`, would let a page step inside any line.

### Transpose

Landed in abb8274: `o_\` reverses the axes, `t_ranspose` permutes them;
image-pipeline's `ky` is `o_\ kx`. The history:

Attention is `S <- Q x transpose(K)`; PCA needs the covariance
`X^T X`. Both read naturally only with a transpose; the two demos are
deferred until it exists (`docs/plan.md`, saga 4).

### Grade per row

A router keeps each token's top two experts: a top-k along each row
of a matrix. `g_rade` grades the items along the first axis, and with
a subscript moves that axis first, so `g_rade_2 M` grades the columns
as whole items and returns one vector:

```
$ xetal eval -e "M := 2 4 r_eshape 0.1 0.5 0.3 0.9 0.7 0.2 0.8 0.1
g_rade_2 M"
1 3 2 4
```

Ask: a grade (and sort) of each row along an axis (a rank-preserving
`g_rade` under a subscript, or a rank operator), so top-k is `k t_ake_2`
of a per-row grade. Workaround in moe-router: the largest of each row
(`'m_ax r_/_2`) as a mask, taken out, then the largest again.

### Inner product speed

The 1.58-bit network runs a 2 -> 16 -> 16 -> 3 network (padded to
16 wide) over a map of points. Measured natively (release):

| Program | Time |
| ------- | ---- |
| 4 x (1024 x 16) `'+ '* i_nner` (16 x 16) | 0.39 s (about 370 ns per multiply-add) |
| the same as `'+ r_/_2` of two spread arrays multiplied | 0.27 s |
| one forward pass of 1024 points (three layers) | 0.31 s |

Repro: `x := (1024 c_at 16) r_eshape 0.1 0.2 -0.3 0.5 0.7`,
`w := (16 c_at 16) r_eshape 0.3 -0.1 0.2 0.0 0.4`, then time
`x '+ '* i_nner w` four times. A specialized kernel for `'+ '* i_nner`
on Floats (and Ints) would make the demo's map finer and its page
quicker. Workaround: a 24 x 24 map; the page keeps the FP32, FP16 and
INT8 maps and reruns only the ternary pass when the threshold moves.

### A bound condition in arithmetic

Landed in ffd5526 (vendored at 081fb3f). The ask as filed:

lang-choices T1 says a Bool converts to Int implicitly in arithmetic,
and T5 says a top-level condition binding defaults to Bool. Together
they suggest a bound mask still works in arithmetic, but it does not:

```
$ xetal eval -e "a := 1 2 > 0
1 * a"
error[type-mismatch]: expected a number, found Bool at 20..25
$ xetal eval -e "a := 1 2 > 0
f_loat a"
error[type-mismatch]: expected a number, found Bool at 13..21
$ xetal eval -e "f_loat 1 2 > 0"
1.0 1.0
```

The image pipeline builds its picture from masks (a disk, a square, a
triangle) bound by name. Workaround: bind each mask as a Float,
`disk := f_loat (...) < 300`. Ask: let a bound Bool convert in
arithmetic as T1 describes (or say in T5 that it does not).

### Build provenance in a vendored build

Landed (4a31ba9), and moot since this repo builds a clone of X_eTaL
(see ask 4 above). As it stood at abb8274:

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
  error; the demos write `f_loat (x > 1)`. Comparisons written inline
  mix with Ints (`1 * (b = 1)` is an Int array) and sum (`'+ r_/`
  over them), and an Int state needs Int, not Bool, values. (A
  condition bound to a name does not convert: see the ask above.)
- **`m_od` takes the dividend on the left** (`a m_od b` is a mod b),
  unlike APL's residue.
- **Only a single value extends.** Arithmetic pairs arrays of the same
  shape, and a single value extends over any shape; a vector does not
  extend over a matrix's rows (`M * 1 2 3` is a shape error), so the
  demos spread it with `t_able` (`(o_ffsets n) 'r_ight t_able v`).
- **Rotation wraps.** `o_-` rotates, so stencils see a torus; the wave
  tank damps its edges (a sponge) so waves leave instead of wrapping.
