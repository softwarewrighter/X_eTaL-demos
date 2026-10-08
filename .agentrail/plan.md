# Saga: doc-content (HIGH PRIORITY, user-requested 2026-10-08)

Goal: write real ##/### documentation into the demos' own .xtl
programs, as X_eTaL writes its own demos/*.xtl (every one of them,
3 to 49 ## lines), so pages/doc/ (Saga 8) is worth reading, not just
structure. The user: "This is a repo for demos and that includes
demonstrating X_eTaL tools, tool output, and commenting conventions.
Very valuable and desirable content needed badly."

Scope per file: a file-level ## intro (what the program shows, why
it is an array expression -- draw from the demo's own README, do not
duplicate it line for line), ### section headings matching the
program's own "# -- stage --" markers, and ## on each top-level
u:/bare name worth a reader's pause. A ## >> runnable example only
where one is short, self-contained and low-risk; most of these
programs print large arrays, which do not make good doctests -- skip
it rather than force one. After each demo: `xetal doc --test FILE`
(if it has an example) and `just gate` (comments must not move any
golden).

1. doc-grids-1: life-microscope.xtl, mandelbrot.xtl, julia.xtl.
2. doc-grids-2: reaction-diffusion.xtl, wave-tank.xtl, ca-lab.xtl,
   langtons-ant.xtl.
3. doc-start-here: sandpile.xtl, nbody.xtl, fourier-epicycles.xtl.
4. doc-macros: image-pipeline.xtl, stencil-macros.xtl,
   xetal-pipes.xtl (its stage files stay out of pages/doc/, as Saga 8
   left them: an undefined args, the wrapper's job).
