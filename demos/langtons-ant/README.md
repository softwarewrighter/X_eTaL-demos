# Langton's ant

On a white cell, turn right; on a black cell, turn left; flip the cell
and step forward. From two rules comes about 10,000 steps of chaos,
and then order: the ant builds a diagonal "highway" forever.

Here the ant is arrays. Its position is a mask with a single 1 and
its direction is a number, so looking at its cell, flipping it and
moving are whole-array operations with no indexing: the same code
would move a thousand ants.

Live: [Langton's ant](https://softwarewrighter.github.io/X_eTaL-demos/langtons-ant/)

[![Langton's ant: the live page](screenshot.png)](https://softwarewrighter.github.io/X_eTaL-demos/langtons-ant/)

## The program

The core of `langtons-ant.xtl` (the live page runs exactly this):

```
u:s_tep := { s ->
  b := 1 s_elect s
  a := 2 s_elect s
  d := f_irst r_avel 3 s_elect s
  cell := '+ r_/ r_avel a * b
  turn := (d + 1 + 2 * cell) m_od 4
  flip := b + a * 1 - 2 * b
  dy := (1 * turn = 0) - 1 * turn = 2
  dx := (1 * turn = 3) - 1 * turn = 1
  step := dy o_-_1 dx o_-_2 a
  (u:p_lane flip) c_at (u:p_lane step) c_at u:p_lane turn + 0 * b
}
```

## How it works

| Part | Code | Shape | What it is |
| ---- | ---- | ----- | ---------- |
| look | `'+ r_/ r_avel a * b` | (one number) | the board times the ant's mask, summed: the colour under the ant |
| turn | `(d + 1 + 2 * cell) m_od 4` | (one number) | right (+1) on white, left (+3, that is -1) on black |
| flip | `b + a * 1 - 2 * b` | 64 64 | 0 becomes 1 and 1 becomes 0 where the mask is 1, nothing elsewhere |
| move | `dy o_-_1 dx o_-_2 a` | 64 64 | the mask rotated one row or column: up, right, down or left |
| state | `u:s_tep s` | 3 64 64 | the board, the mask, and the direction (in every cell) |

The page runs a number of steps per frame (100 by default) on the
board it keeps, and pauses at step 11,500, when the highway has formed
(on this 64 by 64 board, which wraps at its edges, the highway soon
runs into its own trail), and shows the last step's look, turn, flip and move
with X_eTaL's numbers. Its tests check the X_eTaL ant against a direct
simulation, step for step.

Speed: about 2.2 ms a step natively on the 64 by 64 board (each step
is about ten whole-board operations), so the highway, at about step
10,000, takes 20 to 30 seconds natively and longer in the browser.
Faster array kernels in X_eTaL (planned upstream) would shorten that;
the measurement is in [`docs/xetal-asks.md`](../../docs/xetal-asks.md).

## Run it

```bash
just run langtons-ant          # 400 steps on a 21 by 21 board, as characters
just show langtons-ant         # the same as a notebook
just serve langtons-ant        # the web app at http://127.0.0.1:8095/
just test-demo langtons-ant    # the expected output and the web app's tests
```

## Workarounds

The board goes into each run as Float literals floored to Int (Int
strands are slow to read). The direction is kept as a plane of the
state, the same in every cell, because the state must be one array.
There is no functional update (amend) in X_eTaL yet; here the mask
makes one unnecessary, which is the point of the demo.
