# grids

Saga 2 of X_eTaL-demos (docs/plan.md): grid and dynamics demos, with
the shared microscope shell extracted from the first three working
pages (Life, Mandelbrot, reaction-diffusion) rather than designed
ahead of them (user's choice, 2026-10-01).

Model for every demo: demos/life-microscope (CLI .xtl + golden; web/
Yew app on the vendored xetal-play that re-runs an X_eTaL program and
reads back each intermediate array printed with r_avel; native tests
of the model; decorated source with the stage highlighted; stage
timeline with shapes; inspector; X_eTaL logo and footer).

Rules: as saga 1 (CLAUDE.md). Missing X_eTaL features or bugs go in
docs/xetal-asks.md with the workaround named in the demo's README.
Every step: just gate, docs, .gitignore, detailed commit to main with
.agentrail/, push; for a demo with a web app also just pages, commit
pages/, verify the deploy and the live page.

## Steps

1. mandelbrot
2. reaction-diffusion
3. microscope-shell
4. julia
5. wave-tank
6. ca-lab
7. langtons-ant
8. gallery-1-release
