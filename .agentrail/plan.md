# Saga: tuples-retrofit

Goal: remove the "extra plane of a rank-3 array" workaround (an ask,
docs/xetal-asks.md "A state of several arrays") from the four demos
that used it, now that X_eTaL has tuples through p_ower (landed
175acf5, pinned 1998414): wave-tank (time), langtons-ant (direction),
mandelbrot and julia (counts). u:p_lane goes away in each; the state
is a tuple, taken apart by a pattern in the step function and in the
binding that reads p_ower's result; 1/2/3 s_elect indexing becomes
named destructuring.

Per demo: rewrite the .xtl's step function and state; update the web
app's micro.rs (the program it writes, how it reads back p_ower's
tuple result -- likely simpler, each component r_avel'd and printed
on its own, no more unstacking a plane); update tests and reg-rs
goldens; rebuild and browser-check the live page; update the README
(the Workarounds section this resolves, the How-it-works table);
rebuild pages/doc. Mind CLAUDE.md rule 1's marker-string rules, since
the step function's definition text changes shape.

1. tuples-mandelbrot-julia: the simplest case (zr, zi, counts: three
   same-shaped planes, no wasted scalar-as-plane) and the two demos
   share the pattern closely.
2. tuples-wave-tank: time as a scalar tuple component, not a plane.
3. tuples-langtons-ant: direction as a scalar tuple component.
4. tuples-release: docs/xetal-asks.md's ask marked fully landed;
   docs/plan.md a saga entry; README status line if it mentions the
   workaround count.
