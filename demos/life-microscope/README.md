# Life microscope

Conway's Game of Life is one line of X_eTaL. The microscope steps that
line on a board and draws every array it builds on the way, so you can
watch the whole rule happen to every cell at once: no loop over cells,
no loop over neighbours.

Live: [the Life microscope](https://softwarewrighter.github.io/X_eTaL-demos/life-microscope/)

## The program

```
u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }
```

## How it works

Read it right to left, with `b` the board (`_r`, the right argument):

| Step | Code | Shape | What it is |
| ---- | ---- | ----- | ---------- |
| board | `_r` | 16 16 | 0 dead, 1 alive; the edges wrap around |
| rotate | `-1 0 1 o_-_12 _r` | 3 3 16 16 | the board rotated by every offset -1, 0, 1 along axes 1 and 2: nine shifted copies. In the copy for offsets (dy, dx), each cell holds its neighbour at (dy, dx) |
| sum | `'+ r_/_12` | 16 16 | the nine copies added over the two offset axes: S, each cell plus its eight neighbours |
| compare | `(_l = 3)` and `_r * _l = 4` | 16 16, 16 16 | S = 3: alive next whatever it is now (born with 3 neighbours, or alive with 2); alive and S = 4: alive with 3 neighbours |
| add | `+` | 16 16 | the next board |

The inner function `{ (_l = 3) + _r * _l = 4 }` gets S as its left
argument `_l` and the board as its right `_r`. Because S counts the
cell itself, "alive with 2 or 3 neighbours" becomes "S = 3, or alive
and S = 4".

On the page, the line is drawn decorated (as X_eTaL renders it) and
the part computing the selected stage is highlighted. Clicking a cell
shows its value in each of the nine shifted boards: that is its 3 by 3
neighbourhood, and their sum is S.

The page runs X_eTaL itself, compiled to WebAssembly: each generation
it runs the line on the board and prints each intermediate array
(`r_avel`), and draws what X_eTaL printed.

## Run it

```bash
just run life-microscope          # the arrays of one step of a glider, then four steps on
just show life-microscope         # the same as a notebook: each statement, then its output
just serve life-microscope        # the web app at http://127.0.0.1:8095/
just test-demo life-microscope    # the expected output and the web app's tests
```

`life-microscope.xtl` is the command-line program; `web/` is the
browser app (Rust, Yew, the vendored X_eTaL `xetal-play` engine).

## Workarounds

None.
