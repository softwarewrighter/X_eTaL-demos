# X_eTaL demos

<p align="center">
  <b><a href="https://softwarewrighter.github.io/X_eTaL-demos/">The live demo catalog</a></b>
  -- every demo running in your browser (WebAssembly)
</p>

Small programs in [X_eTaL](https://github.com/softwarewrighter/X_eTaL),
the eXperimental eXtensible Typed Array Language, that produce
something worth watching: a cellular automaton, a fractal, an organic
reaction-diffusion texture, a neural network seeing a digit, tokens
being routed to experts in a sparse model.

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

| Demo | What you see | Array ideas | Status |
| ---- | ------------ | ----------- | ------ |
| [Life microscope](demos/life-microscope/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/life-microscope/)) | Conway's Life, with the nine shifted boards and their sum | rotate, reduce, masks | live |
| [Mandelbrot](demos/mandelbrot/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/mandelbrot/)) | the set appearing step by step; a point's orbit; zoom | broadcasting, masks | live |
| [Julia sets](demos/julia/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/julia/)) | one function, two sets; pick c on the Mandelbrot map; c walking its edge | scalar extension | live |
| [Reaction-diffusion](demos/reaction-diffusion/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/reaction-diffusion/)) | Gray-Scott mazes, coral and spots growing; one cell's stencil arithmetic | stencils, iteration | live |
| [Wave tank](demos/wave-tank/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/wave-tank/)) | a double slit, a lens, ripples where you click | stencils, masks | live |
| [Cellular automata lab](demos/ca-lab/README.md) ([live](https://softwarewrighter.github.io/X_eTaL-demos/ca-lab/)) | Rule 30, 90, 110 with an editable table; Life, Brian's Brain, Wireworld as tables | lookup tables, rotations | live |
| Langton's ant | a highway emerging from chaos | state arrays, masks | planned |
| N-body gravity | 50 bodies and the pairwise force cube | pairwise broadcasting | planned |
| Image pipeline | blur, edges, threshold, every stage | convolution, masks | planned |
| 1.58-bit network | ternary weights versus FP32/FP16/INT8 | ternary arrays, dot products | planned |
| MoE routing microscope | tokens routed to 16 experts; where routing jumps | top-k, masks | planned |
| Tiny CNN | draw a digit, see each layer | windows, convolution | planned |
| Attention microscope | the attention heatmap, rows meeting columns | matrix algebra, softmax | waiting on X_eTaL |
| Embedding explorer | PCA from 64 dimensions to a 3-D cloud | covariance, projection | waiting on X_eTaL |
| Tiny world model | predicting the next frame of a ball's world | recurrence, prediction | waiting on X_eTaL |
| Diffusion from noise | an image denoised step by step | tensor transforms | waiting on X_eTaL |

A demo's name links to its own page (`demos/<name>/README.md`) once it
exists. What the "waiting" demos need from X_eTaL is listed in
[`docs/xetal-asks.md`](docs/xetal-asks.md).

## Build

Prerequisites:

- [Rust](https://rustup.rs) (stable) and
  [`just`](https://github.com/casey/just)
  (`brew install just` or `cargo install just`)
- for the browser demos: `rustup target add wasm32-unknown-unknown`
  and [trunk](https://trunkrs.dev) (`brew install trunk` or
  `cargo install trunk`)
- for the gate (maintainers): `sw-markdown-checker`

```bash
just                                 # list the tasks
just xetal                           # build the bundled X_eTaL interpreter
just eval "'+ r_/_2 2 3 r_eshape r_ange 6"   # try it: row sums, 6 15
just test                            # every demo's tests
just gate                            # the pre-commit gate
```

The demos run a copy of X_eTaL kept in this repository under
`vendor/xetal/` (a snapshot of a known-good commit, recorded in
`vendor/xetal/VENDORED`), so they do not change under you as X_eTaL
develops. `just xetal-version` shows which commit it is. Maintainers
refresh it from a sibling checkout with `just vendor` (the latest
commit of `../X_eTaL`) or `just vendor REF`; only committed X_eTaL
work is ever copied, and the refresh is committed on its own after
`just gate` passes.

## Running and adding demos

```bash
just demos                           # the demos, in catalog order
just run SLUG                        # run a demo's program
just show SLUG                       # the same as a notebook: each statement, then its output
just test-demo SLUG                  # check its output against expected/
just new-demo wave-tank "Wave tank"  # start a new demo from demos/_template
just bless SLUG                      # rewrite its expected output (review the diff)
```

Each demo is a sub-project, `demos/<slug>/`:

| File | What it is |
| ---- | ---------- |
| `demo.toml` | title, one-line summary, concepts, status (draft, live, deferred), catalog order, the X_eTaL asks it needs |
| `README.md` | the demo's own page: what you see, the program, how it works |
| `*.xtl` | its X_eTaL programs; each is run by the tests (seed 1) |
| `expected/` | each program's expected output (`NAME.out`, and `NAME.err` when it should fail) |
| `web/` | its browser app (a Cargo workspace), when it has one |
| `test.sh` | any further tests, when it has them |

Pictures a program shows (`[]S_HOW`) are written to `work/draw/<slug>/`.

The web apps share one shell, `shared/microscope/`: running X_eTaL
and reading arrays back, the decorated source with the current stage
highlighted, canvases, stage chips, panels, header and footer. Its
[README](shared/microscope/README.md) walks through adding a demo's
web app.

## The live site

```bash
just serve SLUG       # one demo's web app at http://127.0.0.1:8095/, rebuilt on change
just pages            # build the whole site into pages/
just serve-pages      # preview pages/ at http://127.0.0.1:8096/X_eTaL-demos/
```

The site is built locally: `just pages` builds every demo that has a
web app into `pages/<slug>/` and writes the catalog, `pages/index.html`,
from the demos' `demo.toml` files. `pages/` is committed, and pushing it
to `main` runs a GitHub Actions workflow
(`.github/workflows/pages.yml`) that only publishes the folder, at
<https://softwarewrighter.github.io/X_eTaL-demos/>.

## Status

Early. The project process, plan and build scaffolding are in place,
and the bundled X_eTaL builds and is checked by the gate (its
command-line interpreter, and its library natively and for
WebAssembly). The demo layout, its test runner and the live catalog
are in place, and the Life microscope, Mandelbrot and
reaction-diffusion, Julia set, wave tank and cellular automata lab
demos are live, on a shared page shell. Next: Langton's ant. See
[`docs/plan.md`](docs/plan.md) for the roadmap.

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
- [sw-mlpl](https://github.com/sw-ml-study/sw-mlpl) -- Software
  Wrighter's Machine Learning Programming Language, a Rust array
  language inspired by APL, APL2, J, and BQN.
- [sw-apl](https://github.com/sw-vibe-coding/sw-apl) -- a clean-room
  APL interpreter in Rust modelled on APL\360 and IBM 5100 APL.

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright

## License

MIT. See [`LICENSE`](LICENSE) and [`COPYRIGHT`](COPYRIGHT).
