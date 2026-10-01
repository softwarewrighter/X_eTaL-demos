# foundation

Saga 1 of X_eTaL-demos (docs/plan.md): the process, the vendored
interpreter, the demo sub-project layout, the live-site pipeline, and
one demo (life-microscope) published end to end.

Model: ../X_eTaL (agentrail process, justfile -> scripts/*.sh, pages/
built locally and published by .github/workflows/pages.yml which only
uploads the committed folder).

Rules: each demo is its own sub-project under demos/<slug>/; X_eTaL is
used only through the vendored snapshot in vendor/xetal/ (refreshed
deliberately with `just vendor`); missing X_eTaL features and bugs go in
docs/xetal-asks.md, never worked around silently; demos that cannot be
built with today's X_eTaL are deferred (docs/plan.md saga 5). Every
step: tests pass (`just gate`), docs updated, .gitignore valid, a
detailed commit to main including .agentrail/, push, then
`agentrail complete`.

## Steps

1. scaffold -- process, CLAUDE.md, README, license files, justfile,
   gate, docs/plan.md, docs/xetal-asks.md.
2. vendor-xetal -- `just vendor [REF]`, vendor/xetal/VENDORED,
   `just xetal` builds the vendored CLI into target/xetal/.
3. demo-layout -- demos/_template, `just new-demo SLUG`, demo.toml,
   golden runner, `just run SLUG`.
4. pages-pipeline -- catalog page from demo.toml, build-pages,
   pages.yml workflow, footer provenance, README link; deploy verified.
5. life-microscope -- first demo end to end, published.
