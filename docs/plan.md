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
| A1 | X_eTaL is **vendored** into `vendor/xetal/` as a source snapshot of a committed ref of `../X_eTaL` (`just vendor [REF]`, default `HEAD`), recorded in `vendor/xetal/VENDORED` (SHA, date, subject). Uncommitted work in `../X_eTaL` is never vendored. | X_eTaL is developed in parallel; demos need a recent but stable interpreter, refreshed deliberately, never moving under a step. |
| A2 | The vendored CLI is built into `target/xetal/` (`just xetal`), and every recipe runs that binary, not one on the PATH. | Reproducible: a demo's goldens are tied to `VENDORED`. |
| A3 | Each demo is its own sub-project, `demos/<slug>/`: `demo.toml` (title, one-line summary, concepts, status), `README.md` (the per-demo doc), `<slug>.xtl` programs, `reg/` reg-rs baselines (each `.xtl` at the CLI, the built page in headless Chrome), and `web/` (its own Cargo workspace: a Yew app depending on the vendored `xetal-play` by path). | Demos evolve independently; one broken demo never blocks another. |
| A4 | The shared browser shell (built in saga 2 step 3 from the first three pages; see `shared/microscope/README.md`) (the "array-language microscope": source, array/shape panel, visual world, execution timeline) lives in `shared/microscope/`, a Cargo workspace the demos depend on by path. | The four synchronized views are the same for every demo (research: "one visual execution architecture"). |
| A5 | The live site is built **locally** into `pages/` (`just pages`): a catalog page `pages/index.html` generated from every `demos/*/demo.toml`, plus `pages/<slug>/` from trunk. `pages/` is committed; `.github/workflows/pages.yml` only uploads it (nothing is built on GitHub). | Same model as `../X_eTaL`: simple, fast, deterministic deploys. |
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
| 5 | life-microscope | DONE: `life-microscope.xtl` (+ golden), web app (model tested natively: the line matches the CLI program, a blinker turns, the parts add up, shifted boards hold neighbours, a glider moves; Yew page with decorated line, stage timeline, nine shifted boards, S heatmap, masks, next, cell inspector, play/step/patterns/draw), X_eTaL logo and footer style, live. Planned: first demo end to end: Life programs + goldens, a web app showing the board and the nine rotated boards and their sum, published and linked |

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
| 2 | reaction-diffusion | DONE: five-point Laplacian by four rotations, du 0.2 / dv 0.1 (stable explicit step; du 1.0 diverged), presets maze/coral/ripples/spots/worms; the page keeps the grid and passes it in as literal matrices each frame (64 x 64, 20 steps: ~80 ms natively), X_eTaL prints the last step's arrays; autoplay; cell inspector with the update arithmetic; click drops V. Also: one continuous highlight block per run of tokens in all three demos. Planned: Gray-Scott on U and V; the Laplacian as four shifts and a weighted sum, shown step by step; click a pixel for its neighbourhood and arithmetic; feed/kill presets |
| 3 | microscope-shell | DONE: `shared/microscope` (run, source, canvas, colour, cells, chrome, microscope.css; 7 tests; README how-to); the three demos moved onto it with their tests unchanged and no visible change; the gate tests it and checks every web app for wasm32; fixed `between` stopping before a closing `}`. Planned: `shared/microscope/`: what the three pages share pulled out (X_eTaL runner + `r_avel` parsing, array views: boards, heatmaps, numbers; decorated source with highlighted stage; stage timeline with shapes; inspector frame; header, footer, logo, help); the three demos moved onto it with their tests unchanged |
| 4 | julia | DONE: its own demo; one dyadic function `c u:i_terate z0` gives both sets (scalar extension makes one step serve a grid c and a single c); Mandelbrot picker, presets, Play walks c along the cardioid, zoom; tests: symmetry, c = 0 is the unit disk. Planned: Julia sets on the Mandelbrot program, c picked by clicking the Mandelbrot view |
| 5 | wave-tank | click for ripples; interference; barriers, double slit, obstacles, speeds; the stencil at any point |
| 6 | ca-lab | DONE: every rule a lookup table (s_elect): elementary rules with an editable 8-bit table and a history grown by rotation, 2-D rules (Life, Brian's Brain, Wireworld) as state x neighbours tables, editable, previewing the coming step; tests against direct computations; found the quadratic Int-strand read (ask filed; Float workaround 17x faster). Planned: elementary CA (Rule 30, 90, 110) growing downward, the rule as an editable lookup array; Life, Brian's Brain, Wireworld |
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

## Saga 3 -- physics and ML (implementable now)  [ACTIVE]

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | nbody | 2 x N x N displacements by broadcasting (component axis first: `c_at` stacks along the first axis), the cube shown, reduced to 2 x N accelerations; leapfrog; no loops in the program. Done: four presets, tests for momentum, F_ij = -F_ji, Kepler's period and a direct loop; about 3 ms per 50-body step natively |
| 2 | image-pipeline | reshape, blur, edge detection, threshold on an image, every stage shown. Done: windows as `-1 0 1 o_-_2 -1 0 1 o_-_2 x` (3 x 3 x R x C), one filter function for blur and Sobel, max-pooling by reshape; editable kernels; about 70 ms per 96 x 96 run natively; bound-Bool ask filed |
| 3 | cli-reg | Inserted after an audit: are the pages real? Yes: every page runs the vendored X_eTaL engine compiled to wasm on its demo's own `.xtl` (life-microscope now reads its rule from the file too). CLI goldens moved to reg-rs baselines in each demo's `reg/`; every page is also tested in headless Chrome (`scripts/browser-check.sh`, a reg-rs baseline per demo); the logo with the corrected name |
| 4 | ternary-net | weights as -1/0/+1 glyphs, activation x ternary weights -> accumulators -> activation; FP32/FP16/INT8/1.58-bit storage, ops and error compared. Done: a 2-16-16-3 spiral classifier trained offline (`demos/ternary-net/train/`, `just ternary-train`) in full precision and fine-tuned for ternary; the model is one 17 x 3 x 16 array; formats as array expressions; the page runs three programs from the core so only what changed reruns; `i_nner` speed ask filed |
| 5 | moe-router | 16 experts, a sentence's tokens routed by `token x router_weights`, top-2; animated routes and the routing vector |
| 6 | moe-epsilon | perturb an embedding x + epsilon with a slider and show where the selected experts jump (the routing-discontinuity regions); link to moe-microscope |
| 7 | cnn-weights | a tiny MNIST CNN trained offline (script in `demos/cnn-digits/train/`), weights exported as X_eTaL-readable data |
| 8 | cnn-digits | draw a 28 x 28 digit; conv -> ReLU -> pool -> dense -> softmax in X_eTaL; click any conv output to see input patch x kernel = value |
| 9 | gallery-2-release | catalog, docs, retrospective |

## Saga 4 -- deferred (blocked on asks)

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

- When X_eTaL lands the terminal request (`../X_eTaL-games/docs/xetal-terminal-request.md`: `xetal-cli` buildable for `wasm32-wasip1`, a browser terminal), refresh the vendor and consider running the pages on the real `xetal` binary instead of linking `xetal-play`; the reg-rs CLI and browser baselines stay as they are.

- Refresh the vendored X_eTaL (`just vendor`) at the start of a saga, or
  when an ask in `docs/xetal-asks.md` has landed upstream; never in the
  middle of a step. The refresh is its own commit, with the goldens
  re-run.
- When an ask lands, remove the workaround in the same step that
  refreshes the vendor, and mark the ask done.
