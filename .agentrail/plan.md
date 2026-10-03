# start-here

Saga 4 of X_eTaL-demos (docs/plan.md), asked for by the user
(2026-10-03) after research3: (1) a start-here page and an ecosystem
overview for this repository (README and live catalog), then (2) new
visual demos that X_eTaL can do today (no ML: those belong to
X_eTaL-ML): Fourier epicycles and an abelian sandpile.

Every demo follows shared/microscope/README.md: its own .xtl with a
marked core; reg-rs baselines (cli-NAME, browser-SLUG with
web/browser.txt markers that are not animated and lie inside <main>);
a web/ app that include_str!s the .xtl and runs its core; native tests
against a direct computation or the mathematics; decorated code,
s_hape labels, notices that keep the last good state; README with
screenshot; catalog entry. Missing features or bugs go to
docs/xetal-asks.md (check the "not asks" section first). Pages are
checked in Chrome; timings measured twice (parallel sessions share the
machine).

Every step: just gate, docs, .gitignore, detailed commit to main with
.agentrail/, just pages, push, verify the deploy and the live page.
