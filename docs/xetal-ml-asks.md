# Machine-learning demos: the move to X_eTaL-ML

Done (2026-10-03): X_eTaL-ML took ternary-net, moe-router and
cnn-digits (its live pages checked: the same results), and this
repository removed its copies, the recipes `ternary-train` and
`cnn-train`, `scripts/mnist.sh` and the ML-only CSS. The rest of this
file is the handoff as it was written.

The machine-learning demos move to their own repository,
[X_eTaL-ML](https://github.com/softwarewrighter/X_eTaL-ML) (`../X_eTaL-ML`; the user's decision, 2026-10-03, following
`../X_eTaL/docs/research3.txt`): X_eTaL-demos keeps the visual array
and scientific programs; X_eTaL-ML takes the neural networks, ternary
models, routing, attention and the experiments after them. Its agent
copies the demos below and plans its own sagas for the rest. This file
says what moves, what it depends on, what is still to be built, and
what X_eTaL-demos removes once X_eTaL-ML has them. Nothing is deleted
here until then.

## What moves

| Demo | State here | Files |
| ---- | ---------- | ----- |
| ternary-net | live | `demos/ternary-net/` (program, README, `train/` with its own Cargo workspace, `web/`, `reg/`, screenshot); the recipe `just ternary-train` |
| moe-router | live, with the nudge (moe-epsilon) | `demos/moe-router/` (one program and one page for the router and the nudge; `web/tests/micro.rs` and `web/tests/nudge.rs`) |
| cnn-digits | draft: weights trained, CLI program done, no web app | `demos/cnn-digits/` (program with the weights and ten MNIST test digits, README, `train/`, `reg/`); `scripts/mnist.sh`; the recipe `just cnn-train` |

Each demo is self-contained (rule 1 of `CLAUDE.md`): program, README,
reg-rs baselines (`reg/cli-*`, and `reg/browser-*` for a page), web
app with its native tests and `web/browser.txt`. The pages use
`include_str!` on the demo's own `.xtl`.

## What they depend on (copy, or share)

- `vendor/xetal/` and `vendor/xetal/VENDORED`: the X_eTaL snapshot
  (06d39fa) the goldens were made with; `scripts/vendor-xetal.sh`,
  `scripts/build-xetal.sh`, `scripts/check-vendor.sh`,
  `tools/vendor-probe/`.
- `shared/microscope/`: the page shell (`run`, `source`, `canvas`,
  `color`, `chrome`) and `microscope.css`. The ML pages use these CSS
  blocks: `.formats`, `.stats`, `.glyphs`, `.glyphrow`, `.terms`,
  `.outs` (ternary-net); `.routes`, `input.sentence`, `.bars`,
  `.egrid`, `canvas.pic.strip`, `.changes`, `figure.wide` (moe-router).
- `.cargo/config.toml`: one shared target directory, and the
  WebAssembly stack size (64 MB) the evaluator needs in the browser.
- The test and site tooling: `scripts/test-demos.sh` (reg-rs CLI and
  browser baselines, bless with the browser-check guard),
  `scripts/browser-dom.sh`, `scripts/browser-check.sh`,
  `scripts/selftest-demos.sh`, `scripts/demos.py`, `scripts/new-demo.sh`
  with `demos/_template/`, `scripts/build-pages.sh`,
  `scripts/build-catalog.py`, `scripts/screenshots.sh`,
  `scripts/serve-pages.sh`, `scripts/gate.sh`, the `justfile`, and
  `.github/workflows/pages.yml`.
- `images/modern-xetal-logo.jpg` (the corrected name) and
  `images/favicon.ico`.
- The page conventions in `shared/microscope/README.md`: decorated
  code everywhere, `s_hape` labels, a notice that keeps the last good
  state, the footer.

## Still to build (for X_eTaL-ML's sagas)

1. **cnn-digits web app** (this repo's planned step, not started):
   draw a 28 x 28 digit; X_eTaL runs the convolution (windows by
   rotation, one matrix product), ReLU, pooling, dense and softmax;
   every stage shown with its shape (28 28 -> 8 26 26 -> 8 13 13 ->
   10); click a convolution output cell to see its input patch times
   the filter; a probability bar chart; the ten sample digits to pick.
   Tests: the X_eTaL forward pass equals a Rust forward pass of the
   same weights (the trainer's `Net::forward` is that pass); the
   samples classified. Measure speed (the CLI does 11 passes in about
   0.4 s natively). Then `status = "live"`, screenshot, browser
   baseline. Notes: the program is 158 KB (the 1352 x 10 dense weights
   as literals); a page that sends a drawn digit writes 784 Floats,
   which reads fast (the slow-strand bug is for Ints only).
2. **attention**: Q = X Wq, K, V, S = Q K^T, A = softmax(S / sqrt d),
   Y = A V, each line inspectable; the heatmap for "the animal didn't
   cross the street because it was tired". It waited here for a
   transpose, which has landed (`o_\` and `t_ranspose`, vendored here
   at abb8274).
3. **embedding-explorer**: 1000 x 64 -> center -> covariance ->
   eigenvectors -> projection -> a rotatable 3-D point cloud (waited
   for transpose; eigenvectors by power iteration in X_eTaL).
4. **world-model**: a ball under gravity as an array world; a tiny
   model predicts frame t+1 from t-2..t; actual against predicted;
   change gravity and watch it adapt (waits on training speed, perhaps
   records).
5. **diffusion**: noise, prediction and reconstruction panels (waits
   on a learned denoiser: weights and speed).
6. Later candidates from research3: a tiny transformer, training in
   X_eTaL, quantization beyond ternary, and an ML macro library once
   `.xtlm` lands (for example a network-description macro).

## X_eTaL asks the ML demos carry

From `docs/xetal-asks.md`, re-checked against X_eTaL abb8274 (vendored
here 2026-10-03): transpose, exponent literals and fast Int strands
have landed; `t_able` and `i_nner` got slower (a regression, filed).

| Ask | ML demo | Workaround now |
| --- | ------- | -------------- |
| `i_nner` speed (about 370 ns per multiply-add) | ternary-net, cnn-digits | ternary-net's map is 24 x 24 and its page reruns only the program whose inputs changed |
| Grade per row (`g_rade_2` grades columns as items) | moe-router (top-2) | the largest of each row as a mask, taken out, the largest again |
| A bound condition cannot be used in arithmetic | (image-pipeline; any mask) | bind masks as Floats |
| `xetal-play` arrays in and out without text, or a session | every page that keeps state | literal matrices in, printed `r_avel` lines out |
| Number literals with an exponent | any page writing small Floats | landed in abb8274; `microscope::run::lit` uses them |
| Transpose | attention, embedding-explorer | landed in abb8274 (`o_\`, `t_ranspose`) |
| Long Int strands read slowly | moe-router (word numbers) | landed in abb8274: moe-router can pass Ints |
| `t_able` and `i_nner` regression (2.7x, 1.5x slower than 06d39fa) | ternary-net (formats 0.64 -> 1.40 s), moe-router, cnn-digits | fixed in X_eTaL's Saga 30 (1c1617e: `i_nner` 7x faster than at abb8274) |

## What X_eTaL-demos does once X_eTaL-ML has them

- Remove `demos/ternary-net`, `demos/moe-router`, `demos/cnn-digits`,
  the recipes `ternary-train` and `cnn-train`, `scripts/mnist.sh`, and
  the ML-only CSS blocks listed above (if nothing else uses them).
- Rebuild `pages/` (the catalog loses the three cards) and link
  X_eTaL-ML's live site from the README, the catalog and
  `docs/plan.md`; move the ML asks' "demos" entries to X_eTaL-ML.
- Keep image-pipeline here (an array and image-processing demo, a
  bridge towards ML).
