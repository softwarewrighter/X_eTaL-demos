# Julia sets

The Mandelbrot set's companions. One X_eTaL function iterates
z * z + c and counts the steps z stays within 2 of the origin. Give it
the grid of points as c, starting from z = 0, and you get the
Mandelbrot set; give it one number as c and the grid as the starting
z, and you get that c's Julia set. Pick c by clicking the Mandelbrot
map, or press Play and watch the Julia set change shape as c walks
around the edge of the Mandelbrot set.

Live: [Julia sets](https://softwarewrighter.github.io/X_eTaL-demos/julia/)

[![Julia sets: the live page](screenshot.png)](https://softwarewrighter.github.io/X_eTaL-demos/julia/)

## The program

The function at the heart of `julia.xtl`, the core the live page runs.
The page's program panel shows the whole program it runs: this core,
its settings, the data it writes in (folded) and the lines that print
the arrays it draws.

```
u:i_terate := { (cr, ci) (zr0, zi0) ->
  s_tep := { (zr, zi, counts) ->
    inside := f_loat 4 >= (zr * zr) + zi * zi
    a := (inside * cr + (zr * zr) - zi * zi) + (1 - inside) * zr
    b := (inside * ci + 2 * zr * zi) + (1 - inside) * zi
    (a, b, inside + counts)
  }
  k 's_tep p_ower (zr0, zi0, 0.0 * zr0)
}
```

and the two sets:

```
(mzr, mzi, mandel) := grid u:i_terate (0.0 * gr, 0.0 * gi)
(jzr, jzi, julia) := (-0.8, 0.156) u:i_terate grid
```

## How it works

| Part | Code | Shape | What it is |
| ---- | ---- | ----- | ---------- |
| the grid | `grid` | rows cols, a pair | a pair (real, imaginary) of every point, built by broadcasting a row and a column with `t_able` |
| c for a Julia set | `(-0.8, 0.156)` | a pair | one number's real and imaginary parts |
| c for the Mandelbrot set | `grid` | rows cols, a pair | every point is its own c |
| k steps | `c u:i_terate z0` | rows cols, a 3-tuple | z's real part, its imaginary part, and the count of steps inside, after k steps |

`(cr, ci)` destructures to a pair of numbers for a Julia set and a
pair of matrices for the Mandelbrot set; `inside * cr + ...` works
either way, because X_eTaL extends a scalar over an array (scalar
extension). So the step is written once.

The page computes the Mandelbrot map once and the Julia set whenever c
or the view changes (90 by 135 points, 48 steps). Clicking the Julia
picture zooms in; Play walks c just outside the main cardioid of the
Mandelbrot set, where Julia sets change quickly.

Checked by the page's tests: a Julia set is symmetric through its
center (z and -z behave alike); for c = 0 the set is exactly the unit
disk; the same function gives the Mandelbrot set's known points.

## Run it

```bash
just run julia          # a Julia set and the Mandelbrot set, as characters
just show julia         # the same as a notebook
just serve julia        # the web app at http://127.0.0.1:8413/
just test-demo julia    # its CLI and browser baselines and the web app's tests
```

## Workarounds

X_eTaL has no complex numbers yet, so z and c are each two parts
(planes of the grid, or two numbers), carried as tuples; with complex
numbers the step would be `z * z + c`. A function takes at most two
arguments (left and right), so c and z0 are the two arguments of
`u:i_terate`, each taken apart by a pattern, and k is a variable bound
before it (functions see only names bound before them). (The state
used to stack z's parts and the count as planes of one rank-3 array
for `p_ower`, a workaround of its own; removed now that X_eTaL has
tuples, landed D7: the state is `(zr, zi, counts)`.) Listed in
[`docs/xetal-asks.md`](../../docs/xetal-asks.md).
