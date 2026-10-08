# X_eTaL-demos -- Implementation Plan

A gallery of small, visual, topical X_eTaL programs, each runnable from
the command line and live in the browser. The source of the ideas is
`docs/research.txt` (archival, not normative); this plan turns every
suggestion there into sagas and steps.

Development is driven by agentrail sagas (one active saga in
`.agentrail/`, finished sagas archived to `.agentrail-archive/`), as in
`../X_eTaL`. Every step ends with the gate (`just gate`), docs updated,
a detailed commit to `main` (with the `.agentrail/` changes), a push,
and then `agentrail complete`.

## Guiding principle

Every demo follows the same arc:

> small X_eTaL program -> visually striking result -> step through the
> array transformations -> reveal something technically interesting.

and contains at least one moment where a reader sees an animated loop
nest and realizes the X_eTaL program states the whole operation as one
array transformation. That, not character count, is the argument for
the language.

## Architecture decisions

| # | Decision | Why |
| - | -------- | --- |
| A1 | X_eTaL is **pinned**: `XETAL_COMMIT` holds the full SHA of a committed X_eTaL ref (`just xetal-pin [REF]`, default `HEAD` of `../X_eTaL`). `scripts/xetal.sh` clones X_eTaL into `work/xetal/` (gitignored) and checks that commit out (X_eTaL's `docs/vendoring.md`). Until 2026-10-05 it was a tracked source snapshot in `vendor/xetal/` (602 files, binaries included). | X_eTaL is developed in parallel; demos need a recent but stable interpreter, moved deliberately, never under a step. One tracked line instead of a copy; the clone is a real repository, so the binary reports X_eTaL's commit. |
| A2 | The pinned CLI is built into `target/xetal/` and linked as `bin/xetal` (`just xetal`); every recipe runs that binary, not one on the PATH. | Reproducible: a demo's goldens are tied to `XETAL_COMMIT`. |
| A3 | Each demo is its own sub-project, `demos/<slug>/`: `demo.toml` (title, one-line summary, concepts, status), `README.md` (the per-demo doc), `<slug>.xtl` programs, `reg/` reg-rs baselines (each `.xtl` at the CLI, the built page in headless Chrome), and `web/` (its own Cargo workspace: a Yew app depending on the pinned `xetal-play` in `work/xetal/` by path). | Demos evolve independently; one broken demo never blocks another. |
| A4 | The shared browser shell (built in saga 2 step 3 from the first three pages; see `shared/microscope/README.md`) (the "array-language microscope": source, array/shape panel, visual world, execution timeline) lives in `shared/microscope/`, a Cargo workspace the demos depend on by path. | The four synchronized views are the same for every demo (research: "one visual execution architecture"). |
| A5 | The live site is built **locally** into `pages/` (`just pages`): a catalog page `pages/index.html` generated from every `demos/*/demo.toml`, plus `pages/<slug>/` from trunk. Since 2026-10-05 `pages/` is not tracked on `main`: `just publish` builds it from a clean commit and force-replaces the `gh-pages` branch (one commit, no history), which GitHub Pages serves; nothing is built on GitHub. Until then `pages/` was committed and a workflow uploaded it. | Simple, fast, deterministic deploys, and built files never accumulate in git (as X_eTaL-games and X_eTaL-libraries do). |
| A6 | A feature X_eTaL lacks, or a bug a demo uncovers, is **not** worked around silently and not fixed here: it is recorded in `docs/xetal-asks.md` (what, why, which demo, a minimal repro), and the demo uses a documented workaround or waits. | X_eTaL owns its language decisions; this repo is a consumer. |
| A7 | `just` is the entry point for everything (build, test, run each demo, serve, pages, gate); recipes call `scripts/*.sh`. | One way to do things, usable without `just` too. |

## Gallery order

Implementable demos come first; a demo that needs an X_eTaL feature
that does not exist yet (`docs/xetal-asks.md`) is **deferred** to the
last saga, until its asks land in a vendored release. Each demo's
first step starts with a short feasibility check against the vendored
interpreter; if it turns out to be blocked, the ask is filed and the
step moves to the deferred saga (`agentrail reorder` / `insert`).

The research's progression is kept within what can be built now:

```
boolean grid -> numeric grid -> dynamic system -> neural net -> sparse model
   Life          Mandelbrot     reaction-diffusion   CNN        MoE router
```

| Demo (slug) | Visual payoff | X_eTaL concepts | Now? | Saga |
| ----------- | ------------- | --------------- | ---- | ---- |
| life-microscope | Life, every rotation and sum visible | rotate, reduce, masks | yes | 1 |
| mandelbrot | the set grows iteration by iteration; click a pixel for its orbit | broadcasting, masks | yes (two Float planes) | 2 |
| julia | Julia sets, c picked on the Mandelbrot view | broadcasting, masks | yes | 2 |
| reaction-diffusion | Gray-Scott organic textures; click a pixel for its stencil | stencils, iteration | yes | 2 |
| wave-tank | ripples, interference, double slit | finite-difference stencil | yes | 2 |
| ca-lab | Rule 30/90/110, Life, Brian's Brain, Wireworld; edit the rule | lookup tables, neighborhoods | yes | 2 |
| langtons-ant | emergent highway | state arrays, masks | yes (one-hot masks, no amend) | 2 |
| nbody | 50 bodies; the 2 x N x N displacement cube reduced to forces | pairwise broadcasting (table), reduce | yes | 3 |
| image-pipeline | blur, edges, threshold on a picture | reshape, convolution, masks | yes (synthetic images) | 3 |
| ternary-net | 1.58-bit weights; FP32/FP16/INT8/ternary compared | ternary arrays, inner product | yes | 3 |
| moe-router | tokens routed to 16 experts; epsilon slider shows routing discontinuities | top-k (grade), masks, select | yes | 3 |
| cnn-digits | draw a digit, see every conv/ReLU/pool stage | windows, convolution, tensors | likely (weights read from a file; speed to check) | 3 |
| attention | Q, K, V, softmax heatmap; rows meeting columns | matrix product, softmax, transpose | deferred: transpose | 4 |
| embedding-explorer | PCA 64 -> 3 point cloud, every stage inspectable | covariance, eigenvectors, transpose | deferred: transpose | 4 |
| world-model | predict frame t+1; change gravity | recurrence, learning | deferred: training speed, records | 4 |
| diffusion | noise -> prediction -> reconstruction panels | tensor transforms, iteration | deferred: a learned denoiser | 4 |

The shared microscope (saga 2, step 3) works at statement
granularity, which works today; stepping inside a line waits on the per-operation trace
ask.

## Saga 1 -- foundation  [DONE]

Goal: the process, the vendored interpreter, the demo sub-project
layout, the live-site pipeline, and one demo published end to end.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | scaffold | agentrail saga, CLAUDE.md/AGENTS.md, README (intro, summary, demo list, build, status, copyright, license), COPYRIGHT, LICENSE, .gitignore, justfile, `scripts/gate.sh`, this plan, `docs/xetal-asks.md` |
| 2 | vendor-xetal | DONE: vendored 06d39fa; `scripts/vendor-xetal.sh` takes components, lib, userlibs, demos (xetal-libs and xetal-web embed the last three), .cargo, LICENSE, COPYRIGHT; `tools/vendor-probe` proves an outside workspace can use xetal-play natively and for wasm32; the root `.cargo/config.toml` carries X_eTaL's wasm stack size. Planned: `scripts/vendor-xetal.sh` + `just vendor [REF]` (git archive of a committed ref of `../X_eTaL`: components, lib, userlibs, .cargo), `vendor/xetal/VENDORED`, `just xetal` builds the CLI into `target/xetal/`, `just xetal-version`; gate checks the vendored build |
| 3 | demo-layout | DONE: `demos/_template`, `scripts/demos.py` (list, check, json over `demo.toml`), `scripts/test-demos.sh` (goldens stdout+stderr, web/ cargo test, test.sh; `XETAL_BLESS=1`), `scripts/new-demo.sh`, `scripts/run-demo.sh`, `scripts/selftest-demos.sh` in the gate; recipes demos, new-demo, run, show, test, test-demo, bless. Planned: the sub-project template (`demos/_template/`), `just new-demo SLUG`, `demo.toml` schema, golden runner (`scripts/test-demos.sh`: run each `*.xtl` with the vendored binary, diff against `expected/`), `just run SLUG`, `just test` |
| 4 | pages-pipeline | DONE: `scripts/build-catalog.py` (cards from `demos.py json`, light/dark, footer with this repo's and the vendored X_eTaL commit), `scripts/build-pages.sh` (trunk per demo web app into `pages/<slug>/`, stale ones removed), `scripts/serve-pages.sh`, `.github/workflows/pages.yml` (upload only), Pages enabled with build type workflow; recipes pages, serve-pages, serve. Planned: `scripts/build-catalog.sh` (catalog `pages/index.html` from `demo.toml`), `scripts/build-pages.sh`, `.github/workflows/pages.yml`, footer with build provenance (commit, vendored X_eTaL SHA), README link to the catalog; verify the deploy on GitHub |
| 5 | life-microscope | DONE: `life-microscope.xtl` (+ golden), web app (model tested natively: the line matches the CLI program, a blinker turns, the parts add up, shifted boards hold neighbors, a glider moves; Yew page with decorated line, stage timeline, nine shifted boards, S heatmap, masks, next, cell inspector, play/step/patterns/draw), X_eTaL logo and footer style, live. Planned: first demo end to end: Life programs + goldens, a web app showing the board and the nine rotated boards and their sum, published and linked |

### Saga 1 retrospective

Delivered the process, the vendored X_eTaL with a gate check that an
outside workspace can use `xetal-play` natively and for wasm32, the
demo sub-project layout with a self-tested golden runner, the live
catalog (built locally, published by an upload-only workflow), and
the Life microscope, live. Learned: the X_eTaL engine is fast enough to
re-run a whole program per generation in the browser (16 x 16 board,
a few ms), so a demo can get every intermediate array by printing
`r_avel` of each and parsing the output, with no X_eTaL changes; that
is the model for saga 2's statement-level trace. The page's footer and
logo follow the X_eTaL live demo; a "Literate docs" footer link waits
until this repo has its own literate documents.

## Saga 2 -- grids: demos first, then the shared microscope  [DONE]

Reordered at the user's request (2026-10-01): two more demos before
the shared shell, so the shell is extracted from three working pages
rather than designed ahead of them. The microscope works at statement
granularity (each intermediate array printed with `r_avel` and read
back, as the Life page does); stepping inside a line waits on the
per-operation trace ask.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | mandelbrot | DONE: the page includes `mandelbrot.xtl` and runs its marked core (no copy); 90 x 135 at k = 32 in ~300 ms in Chrome; canvas drawing; k slider and play re-run X_eTaL; orbit by a second program (two `e_ach` passes: no nested arrays); zoom. Planned: complex grid by broadcasting (two Float planes until X_eTaL has complex numbers), escape masks per iteration, the set appearing iteration by iteration, click a pixel to see its orbit z0, z1, ...; zoom by clicking |
| 2 | reaction-diffusion | DONE: five-point Laplacian by four rotations, du 0.2 / dv 0.1 (stable explicit step; du 1.0 diverged), presets maze/coral/ripples/spots/worms; the page keeps the grid and passes it in as literal matrices each frame (64 x 64, 20 steps: ~80 ms natively), X_eTaL prints the last step's arrays; autoplay; cell inspector with the update arithmetic; click drops V. Also: one continuous highlight block per run of tokens in all three demos. Planned: Gray-Scott on U and V; the Laplacian as four shifts and a weighted sum, shown step by step; click a pixel for its neighborhood and arithmetic; feed/kill presets |
| 3 | microscope-shell | DONE: `shared/microscope` (run, source, canvas, color, cells, chrome, microscope.css; 7 tests; README how-to); the three demos moved onto it with their tests unchanged and no visible change; the gate tests it and checks every web app for wasm32; fixed `between` stopping before a closing `}`. Planned: `shared/microscope/`: what the three pages share pulled out (X_eTaL runner + `r_avel` parsing, array views: boards, heatmaps, numbers; decorated source with highlighted stage; stage timeline with shapes; inspector frame; header, footer, logo, help); the three demos moved onto it with their tests unchanged |
| 4 | julia | DONE: its own demo; one dyadic function `c u:i_terate z0` gives both sets (scalar extension makes one step serve a grid c and a single c); Mandelbrot picker, presets, Play walks c along the cardioid, zoom; tests: symmetry, c = 0 is the unit disk. Planned: Julia sets on the Mandelbrot program, c picked by clicking the Mandelbrot view |
| 5 | wave-tank | click for ripples; interference; barriers, double slit, obstacles, speeds; the stencil at any point |
| 6 | ca-lab | DONE: every rule a lookup table (s_elect): elementary rules with an editable 8-bit table and a history grown by rotation, 2-D rules (Life, Brian's Brain, Wireworld) as state x neighbors tables, editable, previewing the coming step; tests against direct computations; found the quadratic Int-strand read (ask filed; Float workaround 17x faster). Planned: elementary CA (Rule 30, 90, 110) growing downward, the rule as an editable lookup array; Life, Brian's Brain, Wireworld |
| 7 | langtons-ant | the ant as state arrays and masks; the highway emerges |
| 8 | gallery-1-release | DONE: `just screenshots` (headless Chrome with a watchdog) into each demo's README and the catalog cards; docs/xetal-asks.md reviewed against every workaround (10 asks, and the language properties that are not asks); retrospective. Planned: catalog, README, per-demo docs, screenshots, retrospective |

### Saga 2 retrospective

Delivered six more live demos (Mandelbrot, reaction-diffusion, Julia
sets, wave tank, cellular automata lab, Langton's ant) and the shared
page shell, extracted from the first three pages as planned. Every
page runs its demo's own `.xtl` (included, not copied), and every
model is tested natively against a direct computation or the
mathematics (Gray-Scott arithmetic, the wave equation, Julia symmetry,
elementary rules, a direct ant).

Learned:
- The "print with `r_avel`, parse, draw" model carries every demo so
  far; the text round trip is the main cost, which led to two asks
  (arrays in and out of `xetal-play`; quadratic Int strands, worked
  around with Floats for a 17x speed-up).
- The demos meet X_eTaL's speed limit (about 50 ns per element per
  operation): fine for pictures, slow for long runs like Langton's
  ant. Asked, with measurements.
- Explicit numerical steps need care: reaction-diffusion diverged at
  du 1.0 (fixed by stable rates); rotation wraps, so the wave tank
  needed a sponge edge and Langton's ant pauses before its highway
  wraps.
- User feedback shaped the shell: decorated code everywhere, s_hape
  labels, a bold highlight block, a compact footer, error notices
  that keep the last good state, and the deep-zoom literal fix.

## Saga 3 -- physics and ML (implementable now)  [DONE]

The machine-learning demos move to `../X_eTaL-ML`
(<https://github.com/softwarewrighter/X_eTaL-ML>; the user's decision,
2026-10-03, after `../X_eTaL/docs/research3.txt`): ternary-net,
moe-router (with the nudge) and cnn-digits are copied there, and
X_eTaL-ML plans its own sagas for the rest of the ML work. This repo
starts no new ML demo and deletes none until X_eTaL-ML has them; the
handoff is `docs/xetal-ml-asks.md`. Done the same day: X_eTaL-ML's
live pages run the three demos, and their copies here were removed.


| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | nbody | 2 x N x N displacements by broadcasting (component axis first: `c_at` stacks along the first axis), the cube shown, reduced to 2 x N accelerations; leapfrog; no loops in the program. Done: four presets, tests for momentum, F_ij = -F_ji, Kepler's period and a direct loop; about 3 ms per 50-body step natively |
| 2 | image-pipeline | reshape, blur, edge detection, threshold on an image, every stage shown. Done: windows as `-1 0 1 o_-_2 -1 0 1 o_-_2 x` (3 x 3 x R x C), one filter function for blur and Sobel, max-pooling by reshape; editable kernels; about 70 ms per 96 x 96 run natively; bound-Bool ask filed |
| 3 | cli-reg | Inserted after an audit: are the pages real? Yes: every page runs the vendored X_eTaL engine compiled to wasm on its demo's own `.xtl` (life-microscope now reads its rule from the file too). CLI goldens moved to reg-rs baselines in each demo's `reg/`; every page is also tested in headless Chrome (`scripts/browser-check.sh`, a reg-rs baseline per demo); the logo with the corrected name |
| 4 | ternary-net | weights as -1/0/+1 glyphs, activation x ternary weights -> accumulators -> activation; FP32/FP16/INT8/1.58-bit storage, ops and error compared. Done: a 2-16-16-3 spiral classifier trained offline (`demos/ternary-net/train/`, `just ternary-train`) in full precision and fine-tuned for ternary; the model is one 17 x 3 x 16 array; formats as array expressions; the page runs three programs from the core so only what changed reruns; `i_nner` speed ask filed |
| 5 | moe-router | 16 experts, a sentence's tokens routed by `token x router_weights`, top-2; animated routes and the routing vector. Done: a 37-word vocabulary of 8 features, experts on a 4 x 4 feature grid so words route by meaning; top-2 by masks (per-row grade ask filed); typed sentences |
| 6 | moe-epsilon | perturb an embedding x + epsilon with a slider and show where the selected experts jump (the routing-discontinuity regions); link to moe-microscope. Done in moe-router (one program, one page): x0 + eps d along a path and x0 + a d1 + b d2 over a plane, by outer products; the strip of gates along eps, the boundaries, a slice map by pair (straight-edged regions); boundaries tested against a direct computation on a 1000x finer grid |
| 7 | cnn-weights | a tiny MNIST CNN trained offline (script in `demos/cnn-digits/train/`), weights exported as X_eTaL-readable data. Done: MNIST fetched by `scripts/mnist.sh` (MD5-checked, gitignored work/mnist/, the user agreed to the download); conv 3x3x8 -> ReLU -> pool -> dense 10 trained by std-only Rust in 12 s, 97.82% on the 10,000 test digits; weights and ten sample digits as literals in cnn-digits.xtl, whose X_eTaL forward pass reads all ten right (`just cnn-train`) |
| 8 | asks-sweep | Inserted (research3: downstream asks lag upstream). Done: vendored X_eTaL abb8274 (its own commit; every CLI baseline and web test unchanged); every ask re-run: landed Int strands, exponent literals, transpose (workarounds removed in ca-lab, langtons-ant, `microscope::run::lit`, image-pipeline's `ky := o_\ kx`), partly landed elementwise speed (about 8x) and nested results (`m_ap`, no mix yet); filed a regression: `t_able` 2.7x and `i_nner` 1.5x slower, pages 1.7 to 2.2x slower (kept, the user's choice; not blocking) |
| 9 | gallery-2-release | Done: every screenshot retaken (the corrected logo); the ML demos removed once X_eTaL-ML's live pages ran them (recipes, mnist script, ML CSS too), the catalog and README linking X_eTaL-ML; README status; per-demo READMEs checked (timings with the regression, nbody's transpose note, mandelbrot's mix); asks reviewed against every workaround (one language property added: only a single value extends); retrospective |
| 10 | cnn-digits | MOVED to X_eTaL-ML (blocked here): draw a digit, every CNN stage shown; see `docs/xetal-ml-asks.md` |

### Saga 3 retrospective

- Delivered: N-body gravity and the image pipeline (here), the 1.58-bit
  network, the MoE routing microscope with its nudge, and the CNN's
  weights (moved to X_eTaL-ML the same day, at the user's decision
  after research3), plus two inserted steps.
- The user's audit question ("are these real?") was the turning point:
  the pages did run the vendored engine on the demos' own .xtl (one
  page held a copied rule, fixed), but tests were golden files and
  native model tests only. Now each demo has reg-rs baselines for its
  CLI programs and for its built page in headless Chrome; a bless
  guard stops a failing browser check from becoming a baseline (it
  happened once, caught before commit).
- Array idioms that worked: windows as two list rotations (Life,
  the image pipeline, the CNN), outer products for pairs and paths
  (N-body's cube, the router's nudge), one model array to stay within
  two arguments (ternary-net), masks for top-k.
- X_eTaL asks filed: bound conditions in arithmetic, `i_nner` speed,
  per-row grade; then the vendor refresh landed transpose, exponent
  literals and fast Int strands (workarounds removed) and brought a
  `t_able` / `i_nner` speed regression (filed; pages slower, not
  blocking).
- Process: parallel sessions share the machine (ports, Chrome, CPU):
  free ports for servers, private Chrome profiles, and timings checked
  twice before they are believed.

## Saga 5 -- start-here (an entry page, a release gate)  [DONE]

Asked for by the user (2026-10-03) after research3: first a
start-here section and an ecosystem overview for this repository,
then visual demos X_eTaL can do today. Reprioritized the same day
after `../X_eTaL/docs/research4.txt`: stop adding breadth; make the
entry page answer, in under 30 seconds, what am I looking at, why is
it easier as an array expression, and show me the X_eTaL that did it;
add performance regression gates; audit and tag a known-compatible
snapshot for the six-repository release. The ecosystem front door
belongs to core X_eTaL's site (this repo carries a copy of the
overview until then). New demos wait until after the launch.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | start-here | README and catalog: what X_eTaL is and why, the three meanings of extensible, the six repositories with links, a 5-minute path; each catalog card answers what, why, and the X_eTaL line (rendered by `xetal render`) |
| 2 | perf-gate | Done: `tools/bench` times each page's own `run` and eight built-ins natively through `xetal-play` (best of 7, as a ratio to a pure-Rust reference loop timed just before each case, a failing case re-measured once); `just bench` writes `docs/bench.md` and the baseline `docs/bench.json`, `just bench-check` fails on a case more than 15% slower (verified with a doctored baseline; three runs on an unchanged build within 5%); required on every vendor refresh (CLAUDE.md rule 2), not in the default gate (40 s, noisy on a shared machine), which builds the tool; found a quadratic scan (ask filed) |
| 3 | promotion-audit | Done without the speed fix (X_eTaL's Saga 30 has not landed): vendored 081fb3f (it lands the bound-condition fix: image-pipeline binds its masks as conditions again); every ask re-run; bench-check: pages within 10%, `inner` about 20% slower again (filed); a fresh clone runs `just run nbody` in about 17 s; the promotion blockers below. The tag moved to its own step, after Saga 30 |
| 3a | listing-is-program | Done (the user's question: the nbody listing had no `m`): every page's program panel shows exactly the program X_eTaL ran for what is on screen (constants, core, the data written in, the run and print lines), long data lines folded by `microscope::source::listing` (decorated, with a count, open and close); the models keep the text of their last run; mandelbrot and julia show their second program too; life-microscope gained a program panel; a test per demo that the program holds the demo's code and folds only data |
| 4 | gallery-3-release | Done: screenshots retaken (listings, Langton's colors, the title glyph); READMEs say the program panel shows the whole program run; the start-here text updated for `.xtlm` macros, which landed in X_eTaL (not yet vendored); asks reviewed; retrospective below |
| 4a | release-tag | Done 2026-10-04: X_eTaL's Saga 30 fixes vendored (1c1617e, with `.xtlm` macros); `just bench-check` nothing slower (table -82%, inner -86%), new baseline; asks re-run (the regression and inner-product asks landed; scan narrowed to built-in operands); docs and timings updated; tagged v0.1.0 (the known-compatible snapshot) |
| 5 | fourier-epicycles | Done (post-launch): the transform as an outer product of angles and two matrix products; circles strongest first by grade; one scan gives every reconstruction (the slider needs no rerun); presets and drawing; tests (inverse, one circle, Parseval, a direct DFT, ordering, resampling) |
| 6 | sandpile | Done (post-launch): every cell topples at once (h d_iv 4, four rotations, an edge mask), a second plane counts topples; the page animates rounds; tests (a direct round, stability, conservation at the edge, the abelian property, symmetry) caught a mask bug in the first .xtl (one side of the edge kept its grains) |
| - | macro demo | now Saga 6 below |

### Saga 5 retrospective (to the gallery-3 release)

- The entry point exists: README start-here and catalog cards that
  answer what, why and show the X_eTaL line (verified against the
  .xtl by the gate); every title leads to the Wikipedia article on its
  subject (or a story dialog when there is none).
- Honesty fixes from the user's eye: the nbody listing hid `m` and the
  run (now every page shows exactly the program it ran, data folded);
  Langton's board was drawn inverted (now black on white).
- Tooling found its own bugs: the vendor script kept git-archive
  timestamps, so cargo could reuse stale builds after a refresh (now
  touched); 5 ms bench cases were too noisy for a 15% gate (now
  repeated); a listing that decorated every number stalled a page
  (now only what is shown is decorated), caught by the browser test.
- `just bench` / `just bench-check` measure every refresh; they found
  a quadratic scan and a further `i_nner` slow-down, filed upstream.
- Still waiting: X_eTaL's speed fix (Saga 30) for the release tag;
  the macros that just landed open a macro demo after the launch.

### Promotion blockers for this repository (2026-10-03)

What a newcomer would hit, in order:

1. Resolved 2026-10-04: the `t_able` / `i_nner` regression (X_eTaL's
   Saga 30, vendored at 1c1617e; the pages are faster than ever).
2. Resolved 2026-10-05: `xetal --version` reports X_eTaL's commit
   (the repo now pins a commit and builds a clone of X_eTaL).
3. Not blocking, noted: a scan with a built-in operand is quadratic in
   its axis (no demo depends on a long scan); per-row grade and mix are open (no demo
   here needs them now).

Checked: the README's start-here path against the live catalog and
pages (stage chips, inspectors, the line on each card), a fresh clone
running `just run nbody`, every page in headless Chrome (the gate's
browser baselines).

## Saga 6 -- macros  [DONE]

Goal: move to X_eTaL's current main and prove "extensible" (research3:
the claim no demo showed) with a demo built on a user macro library,
showing what its macros expand to.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | repin | Done 2026-10-05: X_eTaL pinned by commit, not copied (vendor/xetal removed, `XETAL_COMMIT`, `just xetal-pin`); pinned 882aa76 (88 commits: hygienic macros, `Repeat.xtlm`, system values, the clock, docs); every golden and web test unchanged; `just bench-check` all within +7% (at load 12 to 17, so the baseline was kept); asks re-run: the built-in scan is linear for Ints, still quadratic for Floats; the corrections X_eTaL's ledger needs are in `docs/xetal-asks.md` |
| 2 | stencil-macros-cli | Done 2026-10-05: `Stencil.xtlm` (`m:s_tencil<`, X_eTaL code on text: `n_umbers`, offsets by `d_iv`/`m_od`, one parenthesized rotation term per nonzero weight, `[]R_EJECT` for a non-square kernel; doc examples pass `xetal doc --test`); `stencil-macros.xtl` (blur, edges, sharpen, emboss, 60 heat steps by `p_ower`; totals kept: blur and heat 325 and 1000, edges 0); `check.xtl` (the expansion equals the kernel applied as data for five kernels, exactly); goldens: both runs and the `xetal expand` output (`test.sh`). Learned: X_eTaL's right-to-left reading forces parentheses in generated arithmetic, and `cond ? a; b` only as a whole body (helpers per branch) |
| 3 | repin-v010 | Done 2026-10-05: X_eTaL v0.1.0 pinned (512b3ee, the commit its tag names; 23 commits after 882aa76); every CLI golden, web test and browser baseline unchanged (the gate rebuilt the site on it); `just bench-check` within +9% at load 15 (baseline kept); every open ask re-run, none changed |
| 4 | stencil-macros-web | Done 2026-10-05: the page writes the kernel into `u:s_tep`'s macro call; X_eTaL (wasm) expands it with `Stencil.xtlm` from an in-memory store and runs it; panels: result (signed when the kernel sums to 0), picture, kernel editor (10 presets, 3 x 3 / 5 x 5, divisor, steps), a cell's terms, the call and its expansion with counts, the macro, the program; tests (every preset and edits against a direct loop, term and multiply counts, the heat expansion exactly, a cell's terms, a non-square kernel refused); browser baseline (its marker is a term of the expansion as the page draws it: `*` is drawn as a times sign); screenshot; live. New ask: `xetal-play` has no way to add a page's own library or expand (workaround: three crates by path) |
| 5 | xetal-pipes | Done 2026-10-05: seven stages (`stages/*.xtl`) and their shared `Pipes.xtl` (a halving reader: 20,000 lines in 0.3 s, not 9 s; exact output by `[]N_PUT "/dev/stdout"`; per-character line numbers by a scan; a padded line matrix); `bin/xetal-stage` and its seven links; `test.sh` compares every stage and both pipelines with the Unix tools on six inputs (empty, blank lines, no final newline, 300 lines), stderr empty; `xetal-pipes.xtl` shows the arrays; `demo.tape` (vhs, `just tape`) -> `demo.gif`, also the catalog card's picture for a command-line demo. Found an X_eTaL bug: `--context` reads standard input twice (filed; the arguments go in a copy of the stage instead). Fixed on the way: the catalog's key lines were undecorated since the pin change (`build-catalog.py` still called the removed `build-xetal.sh`) |

## Saga 7 -- private names (X_eTaL's h: namespace)  [DONE]

Goal: follow X_eTaL's new `h:` namespace (PN1-PN7, decided 2026-10-07
in `../X_eTaL/docs/private-names.md`): a library's helper functions,
formerly bare, are now file-private and spelled `h:name`.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | repin | Done 2026-10-07: X_eTaL pinned at 9c667a3 (175 commits after v0.1.0: tuples, the Rosetta stone, errors, quads, and the macros lane's `h:` namespace, PN1-PN7 all implemented, including `xetal migrate FILE`); every golden, web test and browser baseline unchanged; `just bench-check` within +11% at load 18 (baseline kept); every open ask re-run, none changed (confirmed: `xetal run --context` still double-reads standard input) |
| 2 | migrate-h | Done 2026-10-07: `Stencil.xtlm`'s and `stages/Pipes.xtl`'s bare helper functions (`s_ide`, `t_erms`, `t_erm`, `c_ols`, `r_ows`, `n_um`; `e_nded`, `r_ead1`, `r_eadn`, `r_eadall`) rewritten to `h:name` by `xetal migrate` (uses included); both ran bare with a new `deprecated-private` warning at the repin, now clean. CLAUDE.md rule 10 records the convention (apps keep `u:` for their shown stages; `h:` there is legal but unused so far). Found: the pinned build's `deprecated-private` lint does not yet fire for a bare helper in a macro library (`.xtlm`), only an ordinary library (`.xtl`) -- filed as a minor ask. Side effect, fixed: a bare private helper decorated as `Builtin` (misleadingly, as if it were a real built-in); `h:` decorates it `LibFunc` (the same green as `u:`/`l:`/`m:`), so stencil-macros' library panel and screenshot changed |

## Saga 8 -- doc-site (a cross-reference, the mechanical part)  [DONE]

Goal: a cross-reference site for this repo, as X_eTaL's own
`scripts/doc-site.sh` builds for itself (`xetal doc --out`), checked
against `https://softwarewrighter.github.io/X_eTaL/doc/`: every
demo's program and the two library files, typed, linked to their
definitions and uses. The user, 2026-10-08. Mechanical only: no
`##`/`###` doc comments added to the demos' own `.xtl` (Saga 9).

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | doc-site | Done 2026-10-08: `scripts/doc-site.sh` (modeled on X_eTaL's own) documents `Stencil.xtlm`, `stages/Pipes.xtl` and each demo's `<slug>.xtl` (13); left out: `xetal-pipes/stages/{cat,wc,...}.xtl` (an undefined `args`, the wrapper's job) and `stencil-macros/check.xtl` (a correctness check, not a demo). Wired into `build-pages.sh` (`just pages` rebuilds it, `just doc` alone; the demo-cleanup loop now skips `pages/doc/`, which is not a slug). README links the live page (`## Documentation`, `## The live site`), as X_eTaL's README links its own. 32 pages; published and checked live |

## Saga 9 -- doc content (`##`/`###` comments on the demos' own `.xtl`)

Goal: the demos' own programs are undocumented by `xetal doc` (plain
`#` throughout; Saga 8 built the site, but a demo's page there is
structure only, no prose), unlike X_eTaL's own `demos/*.xtl`, every
one of which carries real `##`/`###` documentation (3 to 49 lines
each) specifically so its cross-reference is worth reading. The user,
2026-10-08, high priority: this repository's purpose is demos, which
includes demonstrating X_eTaL's tools, their output, and its
commenting conventions -- content, not structure, is what is missing
and wanted.

Scope: `##` prose (what the program shows, why as arrays) and `###`
section headings on each of the 13 demos' `<slug>.xtl` (matching each
demo's README, not duplicating it line for line). Checked against
X_eTaL's own `demos/*.xtl`: none of them use `## >>` doctests either
(that is a library convention, seen on `.xtl`/`.xtlm` files meant to
be imported); an app's doc comment says in prose what running it
prints, as X_eTaL's own `demos/square.xtl` does ("it prints 49"). Re-run
`xetal doc --test` on each after anyway (0 examples is correct for an
app), and `just gate` (comments do not change a program's output, so
no golden should move); rebuild `pages/doc` (`just doc`) and spot
check a page or two in a browser.

Two gotchas found in step 1, the first serious, both worth knowing
before step 2 (also now CLAUDE.md rule 1 for the first):

- **The live page can silently break.** `web/src/micro.rs` usually
  cuts "the core" out of the `.xtl` by finding exact `#`-comment
  substrings (`str::find`, no error if the text moves); almost every
  demo shares the marker `# -- end of the core`
  (`grep -n "section(SOURCE" demos/*/web/src/micro.rs` lists every
  demo's own markers). Step 1 broke `mandelbrot.xtl` and `julia.xtl`
  this way (their `# Numbers centered on 0` start marker reworded,
  their `# -- end of the core` end marker deleted outright): the CLI
  `diff` against the golden still matched (the marker text is outside
  what the CLI prints), so only the browser check caught it, failing
  `just gate` with the live page showing an X_eTaL error (a name
  defined twice, since the wrong half of the file became "the core").
  Fixed by keeping both markers verbatim somewhere in the file; a `##`
  line can still contain one as a substring (`## Numbers centered on
  0 (...)` contains "# Numbers centered on 0", since the search is
  substring, not whole-line), and a deleted end marker just needs its
  plain `#` line put back. A second, subtler case bit `wave-tank.xtl`
  (two `section()` calls, `prelude` and `core`): writing its `prelude`
  END marker as `##` left that slice's own first `#` dangling with no
  newline after it (the match starts at the SECOND `#`), and the page
  concatenates `prelude` directly with the scene code it injects, so
  the stray `#` commented out the scene's first line (`c2 := 0.3`),
  one further step removed from the marker text itself -- caught the
  same way, a browser-check error ("c2 is not defined"), traced by
  reconstructing the exact concatenation by hand and feeding it to
  `bin/xetal run`. That one marker stayed a plain `#` (CLAUDE.md rule
  1 has the general form: a marker ending a slice the page then
  concatenates with something else must stay plain). Rebuild
  `pages/doc` AND `just pages` (the demo's own web app) and run
  `just gate`, not only `xetal doc --test`, before calling a file
  done; `image-pipeline.xtl` (step 4) has two `section()` calls too.
- **Quoting in prose.** The doc renderer decorates a `##` comment's
  prose the same way it decorates code, so a bare mention of a path or
  a command with a `/` in it comes out with the `/` drawn as a
  division sign. X_eTaL's own comments avoid this by double-quoting
  such text ("xetal run demos/life.xtl"), which renders as plain text;
  a short genuine code mention (a name, `_r`, `-1 0 1`) is written
  bare or in backticks and is meant to be decorated.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | doc-grids-1 | Done 2026-10-08: `##`/`###` on `life-microscope.xtl`, `mandelbrot.xtl`, `julia.xtl` (file intro, section headings matching the existing `# -- stage --` blocks, a `##` above each top-level name); `s_tep` in `julia.xtl`'s `u:i_terate` stays a plain `#` note (a local, bare, scoped to the lambda, PN6 -- not a top-level item `xetal doc` would list on its own). Every output byte-identical (`diff` against the CLI goldens); broke and then fixed the two live pages (the gotchas above); `pages/doc` rebuilt and spot-checked in Chrome (section nav, cross-references, decoration all correct) |
| 2 | doc-grids-2 | `##`/`###` on `reaction-diffusion.xtl`, `wave-tank.xtl`, `ca-lab.xtl`, `langtons-ant.xtl` |
| 3 | doc-start-here | `##`/`###` on `sandpile.xtl`, `nbody.xtl`, `fourier-epicycles.xtl` |
| 4 | doc-macros | `##`/`###` on `image-pipeline.xtl`, `stencil-macros.xtl`, `xetal-pipes.xtl` (its stage files stay out, as Saga 8 left them out of the site) |

## Saga 4 -- deferred (blocked on asks): MOVED to X_eTaL-ML

All four demos below are machine learning: X_eTaL-ML plans them
(`docs/xetal-ml-asks.md`); this repo will not start them. The table is
kept as the record of what they wait on.

Not started until the asks each demo needs have landed in a vendored
X_eTaL release; the saga opens with a vendor refresh and a
feasibility re-check, and the microscope gains stepping inside a line
if the per-operation trace has landed.

| # | Step slug | Delivers | Waits on |
| - | --------- | -------- | -------- |
| 1 | attention | Q = X Wq, K, V, S = Q K^T, A = softmax(S / sqrt d), Y = A V, each line inspectable; the heatmap for "the animal didn't cross the street because it was tired"; rows meeting columns animated | transpose |
| 2 | embedding-explorer | 1000 x 64 -> center -> covariance -> eigenvectors -> projection -> rotatable 3-D cloud | transpose |
| 3 | world-model | ball under gravity as an array world; a tiny model predicts frame t+1 from t-2..t; actual vs predicted with error; change gravity and watch it adapt | training speed, maybe records |
| 4 | diffusion | noise / prediction / reconstruction panels with noise level, signal estimate and error plotted | a learned denoiser (weights + speed) |
| 5 | gallery-3-release | catalog, docs, final retrospective | |

## Cross-cutting

- When X_eTaL lands the terminal request (`../X_eTaL-games/docs/xetal-terminal-request.md`: `xetal-cli` buildable for `wasm32-wasip1`, a browser terminal), pin it and consider running the pages on the real `xetal` binary instead of linking `xetal-play`; the reg-rs CLI and browser baselines stay as they are.

- Pin a newer X_eTaL (`just xetal-pin`) at the start of a saga, or
  when an ask in `docs/xetal-asks.md` has landed upstream; never in the
  middle of a step. The new `XETAL_COMMIT` is its own commit, with the
  goldens and timings re-run.
- When an ask lands, remove the workaround in the same step that
  pins it, and mark the ask done.
