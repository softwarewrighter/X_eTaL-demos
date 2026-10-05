# Stencils by macro

An image kernel is a small picture of numbers: the blur is
`1 2 1 / 2 4 2 / 1 2 1`, the edge finder `-1 -1 -1 / -1 8 -1 / -1 -1 -1`.
Each cell of the result is its neighbors times those numbers, summed.
Here a kernel is written just like that, as text, and a macro library
of our own, `Stencil.xtlm`, turns it into code when the program is
expanded: one rotation of the picture per number, times the number.
A 0 writes nothing and a 1 writes no multiply, so each stencil costs
exactly the work its kernel asks for. The program applies a blur,
edges, sharpen, emboss and sixty steps of heat flow.

This is X_eTaL's third way to extend the language (after libraries,
and before native extensions): a macro is an ordinary X_eTaL function
from the source text on each side of its call to new source text, run
before the program is type-checked. `xetal expand` shows what the
program becomes.

Live: a page is on its way (edit a kernel, see its expansion and the
result).

## The program

From `stencil-macros.xtl`:

```
"s:" u_se< "Stencil"

u:b_lur := { p -> ("1 2 1  2 4 2  1 2 1" s:t_encil< "p") / 16 }
u:e_dges := { p -> "-1 -1 -1  -1 8 -1  -1 -1 -1" s:t_encil< "p" }
u:s_harpen := { p -> "0 -1 0  -1 5 -1  0 -1 0" s:t_encil< "p" }
u:e_mboss := { p -> "-2 -1 0  -1 1 1  0 1 2" s:t_encil< "p" }
u:h_eat := { p -> p + 0.2 * "0 1 0  1 -4 1  0 1 0" s:t_encil< "p" }

warm := 60 'u:h_eat p_ower spot
```

`xetal expand stencil-macros.xtl` shows the same lines after the
macros ran (kept in `expected/stencil-macros.expanded.xtl`). The heat
step, for one:

```
u:h_eat := { p -> p + 0.2 * (((-1 o_-_1 p) + (-1 o_-_2 p) + (-4 * p) + (1 o_-_2 p) + (1 o_-_1 p))) }
```

The four zeros of its kernel are gone, its four 1s are plain
rotations, and only the center is multiplied.

## How it works

The macro, `m:t_encil<` in `Stencil.xtlm`, is X_eTaL code working on
text:

| Step | Code | What it is |
| ---- | ---- | ---------- |
| read | `n_umbers kernel` | the kernel's numbers, as Floats |
| size | `n s_ide 1` | the side k of the square (1, 3, 5, ...); any other count is refused with `[]R_EJECT` |
| offsets | `(i d_iv k) - c`, `(i m_od k) - c` | each number's row and column offset from the center |
| cells | `r_avel o_\ (3 c_at n) r_eshape ...` | weight, row offset, column offset, for each number |
| terms | `t_erms`, `t_erm` | for each weight that is not 0: the array rotated by the offsets (`o_-_1` rows, `o_-_2` columns), times the weight, as text |
| join | `" + "` | the terms, each in parentheses, summed |

Each term is parenthesized because X_eTaL reads right to left: written
out bare, `a + b - 4 * c + d` would subtract `4 * c + d`. The edges
wrap (rotation), so a blur and a heat step keep the picture's total
exactly; the program prints both totals, and the edges' total, 0.

`check.xtl` checks the macro against the kernel applied as data, the
way image-pipeline filters: the stack of the picture's nine shifted
copies times the kernel, summed. For five kernels (with fractions,
negatives, a single offset, all zeros and a 1 x 1) the two agree
exactly. The library's own doc examples run with
`xetal doc --test Stencil.xtlm`.

## Run it

```bash
just run stencil-macros          # run the program
just show stencil-macros         # as a notebook: each statement, then its output
just test-demo stencil-macros    # its CLI baselines (reg-rs), the expansion, the doc examples
bin/xetal expand demos/stencil-macros/stencil-macros.xtl   # the program after the macros ran
```

## Workarounds

None.
