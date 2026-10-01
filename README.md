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
| Life microscope | Conway's Life, with the nine shifted boards and their sum | rotate, reduce, masks | planned |
| Mandelbrot and Julia | the set appearing iteration by iteration; a pixel's orbit | broadcasting, masks | planned |
| Reaction-diffusion | Gray-Scott textures; one pixel's stencil arithmetic | stencils, iteration | planned |
| Wave tank | ripples, interference, a double slit | finite differences | planned |
| Cellular automata lab | Rule 30, 90, 110, Brian's Brain, Wireworld | lookup tables, neighborhoods | planned |
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

Each demo's name will link to its own page in `demos/<name>/README.md`
once it exists. What the "waiting" demos need from X_eTaL is listed in
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

## Status

Early. The project process, plan and build scaffolding are in place,
and the bundled X_eTaL builds and is checked by the gate (its
command-line interpreter, and its library natively and for
WebAssembly). The demo layout, the live catalog and the first demo
(the Life microscope) come next. See
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
