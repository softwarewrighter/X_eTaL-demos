# physics-ml

Saga 3 of X_eTaL-demos (docs/plan.md): physics and machine-learning
demos that X_eTaL can do today: N-body gravity, an image pipeline, a
1.58-bit (ternary) network, an MoE routing microscope (with the
routing-discontinuity slider) and a tiny CNN.

Every demo follows shared/microscope/README.md: its own .xtl with a
marked core and golden; a web/ app on shared/microscope that
include_str!s the .xtl core, prints intermediate arrays with r_avel
and draws them; native tests against a direct computation or the
mathematics; decorated code everywhere, s_hape labels, notices that
keep the last good state; README with screenshot (just screenshots);
catalog entry. Missing features or bugs go to docs/xetal-asks.md
(check the "not asks" section first). Pass Int arrays as Floats
floored (the Int-strand ask). Measure speed and size grids to it.

Every step: just gate, docs, .gitignore, detailed commit to main with
.agentrail/, just pages, push, verify the deploy and the live page.

## Steps

1. nbody
2. image-pipeline
3. ternary-net
4. moe-router
5. moe-epsilon
6. cnn-weights
7. cnn-digits
8. gallery-2-release
