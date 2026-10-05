# Saga: macros

Goal: move to X_eTaL's current main and prove "extensible" with a demo
built on a user macro library, showing what the macros expand to.

1. repin: pin X_eTaL main (882aa76) in XETAL_COMMIT; goldens, web
   tests and `just bench-check`; re-check every ask against it; write
   down what X_eTaL's asks ledger (its docs/asks.md, asks.toml) needs
   to change for this repo.
2. stencil-macros-cli: a demo whose user macro library
   (Stencil.xtlm) turns a kernel written as a picture of numbers into
   rotation arithmetic at expansion time (zero entries vanish); its
   .xtl applies several kernels (blur, edges, Laplacian diffusion);
   CLI goldens, including the `xetal expand` output; README; asks for
   anything missing.
3. stencil-macros-web: the page: edit the kernel, see the macro call,
   its expansion and the result image update; browser baseline,
   screenshot, catalog, pages; release notes in README and plan.
