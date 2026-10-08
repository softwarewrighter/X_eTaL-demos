# X_eTaL demos

<p align="center">
  <b><a href="https://softwarewrighter.github.io/X_eTaL-demos/">The live demo catalog</a></b>
  -- every demo running in your browser (WebAssembly)
</p>

Small programs in [X_eTaL](https://github.com/softwarewrighter/X_eTaL),
the eXperimental Extensible Typed Array Language, that produce
something worth watching: a cellular automaton, a fractal, an organic
reaction-diffusion texture, waves through a double slit, gravity
between every pair of bodies.

## Start here

**X_eTaL** asks what an APL-family array language would look like if
it were designed today: whole-array programming and terse composition
as in APL, J and BQN, but with inferred static types and typed
functional composition from Haskell, explicit, checked interfaces in
the spirit of Rust, and plain ASCII source (`'+ r_/ v`) drawn as
readable typography. Programs stay short; the type checker answers
"can these pieces actually compose?" before anything runs.

**Extensible** has three meanings: libraries extend the vocabulary,
macros extend what the language can say, native extensions extend the
machine.

- `.xtl` libraries: reusable, typed X_eTaL functions
  (`"mx:" u_se< "Matrix"`). Ready today.
- `.xtlm` macro libraries: new notation that expands into ordinary,
  visible X_eTaL (`xetal expand` prints it), which is then
  type-checked. New in X_eTaL (and in this repository's bundled copy).
- Native extensions: Rust libraries behind typed X_eTaL facades, for
  what the language should not reinvent (a database, a clock). Working
  through a bridge today (hello, clock, sqlite); core support coming.

**The ecosystem**, one question per repository:

| Repository | What it holds | Live |
| ---------- | ------------- | ---- |
| [X_eTaL](https://github.com/softwarewrighter/X_eTaL) | the language: interpreter, REPL, editor, notebook, browser playground | [playground](https://softwarewrighter.github.io/X_eTaL/) |
| [X_eTaL-demos](https://github.com/softwarewrighter/X_eTaL-demos) (here) | visual array and scientific programs | [catalog](https://softwarewrighter.github.io/X_eTaL-demos/) |
| [X_eTaL-ML](https://github.com/softwarewrighter/X_eTaL-ML) | machine learning: ternary networks, MoE routing, a CNN, attention | [catalog](https://softwarewrighter.github.io/X_eTaL-ML/) |
| [X_eTaL-games](https://github.com/softwarewrighter/X_eTaL-games) | interactive games and puzzles: state, input, ordinary programs | [games](https://softwarewrighter.github.io/X_eTaL-games/) |
| [X_eTaL-libraries](https://github.com/softwarewrighter/X_eTaL-libraries) | reusable `.xtl` libraries and `.xtlm` macro libraries | [libraries](https://softwarewrighter.github.io/X_eTaL-libraries/) |
| [X_eTaL-extensions](https://github.com/softwarewrighter/X_eTaL-extensions) | native Rust extensions and their ABI | [extensions](https://softwarewrighter.github.io/X_eTaL-extensions/) |

**Five minutes here:**

1. Open the [live catalog](https://softwarewrighter.github.io/X_eTaL-demos/)
   and pick a demo; Life or N-body gravity are good first ones.
2. Read its program beside the result: each stage chip highlights the
   code that computes it and shows the array's shape (`s_hape = ...`).
3. Click a cell or a body: the inspector shows the arithmetic with
   the numbers X_eTaL printed.
4. Run it at the command line: `just run nbody` (it builds the
   bundled X_eTaL the first time; see Build below).
5. Change it: edit `demos/nbody/nbody.xtl` (a softening, a time step)
   and run it again, or `just serve nbody` to see the page change.

## What this is

Each demo follows the same arc:

> a small X_eTaL program -> a visually striking result -> the array
> transformations stepped through -> something technically
> interesting revealed.

The point is the moment where a loop nest you would write in another
language turns out to be one array expression: rotate the board by
every offset and sum (Life), iterate `z <- z*z + c` over a whole grid
at once (Mandelbrot), build the N by N displacement cube and reduce it
(N-body). Each demo shows its program beside the result, with the
shape of every intermediate array, so you can see the whole operation
happen.

Every demo is a separate sub-project under `demos/<name>/`, runnable
from the command line with `just` and published as a page of the live
catalog.

## Demos

The machine-learning demos (the 1.58-bit network, the MoE routing
microscope, the tiny CNN, and future ones such as attention) are in
their own repository, [X_eTaL-ML](https://github.com/softwarewrighter/X_eTaL-ML)
([live](https://softwarewrighter.github.io/X_eTaL-ML/)); they were made
here and moved there (`docs/xetal-ml-asks.md`).

| Demo | What you see | Array ideas | Status |
| ---- | ------------ | ----------- | ------ |
| [Life microscope](demos/life-microscope/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/life-microscope/)) | Conway's Life, with the nine shifted boards and their sum | rotate, reduce, masks | live |
| [Mandelbrot](demos/mandelbrot/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/mandelbrot/)) | the set appearing step by step; a point's orbit; zoom | broadcasting, masks | live |
| [Julia sets](demos/julia/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/julia/)) | one function, two sets; pick c on the Mandelbrot map; c walking its edge | scalar extension | live |
| [Reaction-diffusion](demos/reaction-diffusion/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/reaction-diffusion/)) | Gray-Scott mazes, coral and spots growing; one cell's stencil arithmetic | stencils, iteration | live |
| [Wave tank](demos/wave-tank/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/wave-tank/)) | a double slit, a lens, ripples where you click | stencils, masks | live |
| [Cellular automata lab](demos/ca-lab/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/ca-lab/)) | Rule 30, 90, 110 with an editable table; Life, Brian's Brain, Wireworld as tables | lookup tables, rotations | live |
| [Langton's ant](demos/langtons-ant/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/langtons-ant/)) | a highway emerging from chaos | one-hot masks, rotation | live |
| [Abelian sandpile](demos/sandpile/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/sandpile/)) | avalanches settling into a fractal; drop grains anywhere | rotations, masks, integer division | live |
| [N-body gravity](demos/nbody/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/nbody/)) | a figure-eight three-body orbit, a binary with planets, a collapsing cluster; the pairwise force cube | pairwise broadcasting, reduce | live |
| [Fourier epicycles](demos/fourier-epicycles/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/fourier-epicycles/)) | circles on circles tracing a star, a heart or your drawing; a slider for how many | outer product, matrix product, scan | live |
| [Image pipeline](demos/image-pipeline/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/image-pipeline/)) | blur, Sobel edges, threshold, pooling; edit the kernels, click a pixel for its window | windows by rotation, reduce, reshape | live |
| [Stencils by macro](demos/stencil-macros/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/stencil-macros/)) | kernels written as pictures of numbers, turned into code by a macro library of our own; edit one and see what the program expands to | macro libraries, expansion, rotations | live |
| [Unix pipes in X_eTaL](demos/xetal-pipes/README.md) (command line) | `xetalcat sample.txt \| xetalgrep the \| xetalsort \| xetalhead -n 3`: cat, wc, grep, uniq, sort, head, tail as X_eTaL programs, byte for byte the real tools | text as arrays, scan, grade, compress | live |

A demo's name links to its own page (`demos/<name>/README.md`, with a
screenshot) once it exists. What the "waiting" demos need from X_eTaL is listed in
[`docs/xetal-asks.md`](docs/xetal-asks.md).

## Build

Prerequisites:

- [Rust](https://rustup.rs) (stable) and
  [`just`](https://github.com/casey/just)
  (`brew install just` or `cargo install just`)
- for the browser demos: `rustup target add wasm32-unknown-unknown`
  and [trunk](https://trunkrs.dev) (`brew install trunk` or
  `cargo install trunk`)
- for the gate (maintainers): `sw-markdown-checker`, `reg-rs` and
  Google Chrome (the browser tests)

```bash
just                                 # list the tasks
just xetal                           # build the bundled X_eTaL interpreter
just eval "'+ r_/_2 2 3 r_eshape r_ange 6"   # try it: row sums, 6 15
just test                            # every demo's tests
just gate                            # the pre-commit gate
```

The demos run a known-good X_eTaL commit, pinned in `XETAL_COMMIT`
(one line, the full SHA), so they do not change under you as X_eTaL
develops. `just xetal` clones X_eTaL into `work/xetal/` (gitignored),
checks that commit out, builds the CLI and links it as `bin/xetal`;
the web apps build on the crates in the same clone. The first build
takes a few minutes; `XETAL_SOURCE=../X_eTaL just xetal` clones from a
sibling checkout instead of GitHub. `just xetal-version` shows the
commit (the binary reports it too). Maintainers move to a newer X_eTaL
with `just xetal-pin` (the latest commit of `../X_eTaL`) or
`just xetal-pin REF`, then commit `XETAL_COMMIT` on its own after
`just gate` and `just bench-check` pass.

## Running and adding demos

```bash
just demos                           # the demos, in catalog order
just run SLUG                        # run a demo's program
just show SLUG                       # the same as a notebook: each statement, then its output
just test-demo SLUG                  # its reg-rs baselines (CLI and browser) and web tests
just new-demo wave-tank "Wave tank"  # start a new demo from demos/_template
just bless SLUG                      # accept its current output as the baselines (review the diff)
just bench                           # time the pages' programs and the built-ins (docs/bench.md)
just bench-check                     # compare with the committed timings (on every X_eTaL pin)
```

Each demo is a sub-project, `demos/<slug>/`:

| File | What it is |
| ---- | ---------- |
| `demo.toml` | title, one-line summary, why it is an array expression (`idea`) and its key line (`line`, shown on its catalog card), where its title leads (`wiki`: the Wikipedia article on its subject, or `story`: a short history for a dialog), concepts, status (draft, live, deferred), catalog order, the X_eTaL asks it needs |
| `README.md` | the demo's own page: what you see, the program, how it works |
| `*.xtl` | its X_eTaL programs; each is run by the tests (seed 1) |
| `reg/` | its reg-rs baselines: `cli-NAME` runs `NAME.xtl` with the bundled `xetal` CLI, `browser-SLUG` loads the built page in headless Chrome (`.rgt` command and exit code, `.out` / `.err`; the `.tdb` cache is not committed) |
| `web/` | its browser app (a Cargo workspace), when it has one, with `browser.txt`: text the page shows only after X_eTaL has run |
| `test.sh` | any further tests, when it has them |

Pictures a program shows (`[]S_HOW`) are written to `work/draw/<slug>/`.

Every page runs the real thing: the bundled X_eTaL engine (the
pinned `xetal-play` crate: the same parser, type checker and
evaluator the `xetal` CLI is built from) compiled to WebAssembly,
running the demo's own `.xtl` file (included in the app at build
time). The tests check it three ways: each `.xtl` at the command line
with the bundled CLI against its reg-rs baseline; the web app's model
natively (the same engine, the same `.xtl`) against direct
computations; and the built page in headless Chrome
(`scripts/browser-check.sh`), which must show results only a
successful X_eTaL run produces and no X_eTaL error. Running them
needs `reg-rs` (the regression-test CLI) and Google Chrome.

The web apps share one shell, `shared/microscope/`: running X_eTaL
and reading arrays back, the decorated source with the current stage
highlighted, canvases, stage chips, panels, header and footer. Its
[README](shared/microscope/README.md) walks through adding a demo's
web app.

## The live site

```bash
just serve SLUG       # one demo's web app at http://127.0.0.1:8413/, rebuilt on change
just pages            # build the whole site into pages/
just publish          # publish it (the gh-pages branch)
just check-live       # check every published page runs X_eTaL
just serve-pages      # preview pages/ at http://127.0.0.1:8413/X_eTaL-demos/
just screenshots      # capture each demo (headless Chrome) into demos/<slug>/screenshot.png
```

The site is built locally: `just pages` builds every demo that has a
web app into `pages/<slug>/` and writes the catalog, `pages/index.html`,
from the demos' `demo.toml` files. `pages/` is not tracked on `main`:
`just publish` builds it from the current commit and pushes it as the
only commit of the `gh-pages` branch, which GitHub Pages serves at
<https://softwarewrighter.github.io/X_eTaL-demos/>.

## Status

Twelve demos are live on a shared page shell, each runnable at the
command line and in the browser on the pinned X_eTaL (9c667a3): the
Life microscope, Mandelbrot and Julia sets, reaction-diffusion, the
wave tank, the cellular automata lab, Langton's ant, the abelian
sandpile, N-body gravity, Fourier epicycles, the image pipeline and
stencils by macro (the first built on a macro library of its own,
expanded in the browser). A thirteenth runs at the command line only:
Unix pipes, seven X_eTaL programs as pipeline stages. Every demo's programs are tested at the command line and its page in
headless Chrome, both as reg-rs baselines, with the page's model
tested natively. The machine- learning demos moved to X_eTaL-ML
(above). Timings of every page's program are kept in
[`docs/bench.md`](docs/bench.md) and checked on every new pin of
X_eTaL. See [`docs/plan.md`](docs/plan.md) for the roadmap.

## Documentation

- [`docs/plan.md`](docs/plan.md) -- architecture decisions, the
  gallery, the roadmap
- [`docs/xetal-asks.md`](docs/xetal-asks.md) -- features and fixes the
  demos need from X_eTaL
- `docs/research.txt` -- the archival list of demo ideas
- [`CLAUDE.md`](CLAUDE.md) (also `AGENTS.md`) -- the agent workflow
  (agentrail sagas) and rules

## Development

Development is tracked with agentrail sagas, as in X_eTaL: `agentrail
status` shows the current step, `agentrail next` its instructions.
Every step ends with the gate passing, docs updated, a commit to `main`
and a push.

## Related Projects

- [X_eTaL](https://github.com/softwarewrighter/X_eTaL) -- the language
  ([try it live](https://softwarewrighter.github.io/X_eTaL/))
- [X_eTaL-ML](https://github.com/softwarewrighter/X_eTaL-ML) -- the
  machine-learning demos ([live](https://softwarewrighter.github.io/X_eTaL-ML/))
- [sw-mlpl](https://github.com/sw-ml-study/sw-mlpl) -- Software
  Wrighter's Machine Learning Programming Language, a Rust array
  language inspired by APL, APL2, J, and BQN.
- [sw-apl](https://github.com/sw-vibe-coding/sw-apl) -- a clean-room
  APL interpreter in Rust modeled on APL\360 and IBM 5100 APL.

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright

## License

MIT. See [`LICENSE`](LICENSE) and [`COPYRIGHT`](COPYRIGHT).
