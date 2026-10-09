# Eigencube

A Rubik's cube with no sticker tables and no permutation tricks: 26
cubelets, each a point c of {-1, 0, 1}^3 carrying a 3-by-3 rotation
matrix R, so that it sits at R c. A quarter turn picks the cubelets on
its side with a dot product, v . (R c) > 0, and moves them with one
matrix product; the stickers are read back from the matrices. It is a
port of Steffen Smolka's
[eigencube.py](https://github.com/smolkaj/eigencube) (MIT), whose
README tells the idea at length. In X_eTaL a whole batch of cubes is
one n x 26 x 3 x 3 array, so every turn of every cube in a search is
two matrix products, one for the masks and one for the rotations.

Live: [the Eigencube page](https://softwarewrighter.github.io/X_eTaL-demos/eigencube/):
turn the faces, scramble, undo, and solve, then step through the
solution or play it. Each click runs the program below in your
browser and draws the stickers it prints; a solve takes a few seconds.

In 3D: the same library solves a voxel cube in X_eTaL's native scene
window in
[X_eTaL-extensions](https://softwarewrighter.github.io/X_eTaL-extensions/#scene-voxels-rubik-solve)
([recording](https://softwarewrighter.github.io/X_eTaL-extensions/scene/voxels-rubik-solve.webm)),
which the page plays.

[![Eigencube: the live page](screenshot.png)](https://softwarewrighter.github.io/X_eTaL-demos/eigencube/)

## The program

The cube and its solver are the
[Eigencube library](https://github.com/softwarewrighter/X_eTaL-libraries/tree/main/libs/Eigencube)
of X_eTaL-libraries, pinned in `LIBRARIES_COMMIT` (`just libraries`
fetches it into `work/libraries`); this demo's program imports it,
scrambles a cube and solves it, and the page imports the same file.
The library's core: the twelve turns as one 36 by 3 matrix `M` (each
move's 3-by-3 rotation, a quarter turn about its face's normal v, is
v v^T - [v]x, Rodrigues' formula at -90 degrees), every turn of a
batch of n 3-by-3 matrices at once:

```
h:t_urns := { R ->
  n := t_ally R
  1 3 2 4 t_ranspose (12 3 c_at n c_at 3) r_eshape M '+ '* i_nner 2 1 3 t_ranspose R
}
```

All twelve turns of every cube in a batch S (n 26 3 3): the masks are
one matrix product of the move vectors with where each cubelet sits,
the rotations the other:

```
h:c_hildren := { S ->
  n := t_ally S
  sh := 12 c_at n c_at 26 3 3
  sel := 0 < V '+ '* i_nner o_\ ((n * 26) c_at 3) r_eshape h:p_os S
  T := sh r_eshape h:t_urns ((n * 26) c_at 3 3) r_eshape S
  S12 := sh r_eshape S
  ((12 * n) c_at 26 3 3) r_eshape S12 + (sh r_eshape 9 r_eplicate r_avel sel) * T - S12
}
```

The demo's own program (`eigencube.xtl`), after the import:

```
sc := r_oll! 20 r_eshape 12
x := sc ec:d_o ec:solved
(sol, lens, e) := 1 ec:s_olve x
ec:s_olved? sol ec:d_o x
```

## How it works

Read right to left.

- `pts` is the 27 integer points of {-1, 0, 1}^3 (`e_ncode` of 0 to
  26 in base 3, less 1); `C`, the 26 cubelets, drops the center. A
  cubelet's 1-norm is its number of stickers: 1 a center, 2 an edge,
  3 a corner.
- A cube is 26 rotation matrices; the solved cube is 26 identities,
  shape 1 26 3 3, a batch of one.
- `h:p_os S` is where each cubelet sits, R c, for every cube of a
  batch at once: `'+ r_/_4` of the matrices times the names spread
  along the rows.
- `h:c_hildren S` is every turn of every cube: `sel`, 12 by 26n, is the
  move vectors times the positions (a cubelet turns when the product
  is positive), `T` is every matrix turned every way (`h:t_urns`), and
  the result keeps a matrix where `sel` is 0 and takes the turned one
  where it is 1.
- `ec:s_tickers S` reads the colors back: cubelet k shows a sticker on
  face v when v . (R c) = 1, and the sticker's color is the face it
  faced when solved, R^T v, one more matrix product (`N '+ '* i_nner`
  the transposed matrices). The 54 stickers are sorted by face and
  place, giving the 6 by 9 the page draws.

The solver follows eigencube.py's stages: the top and middle layers a
cubelet at a time (17 searches), the bottom edges (8) and the bottom
corners' places (4), each an A* search, then eigencube.py's corner
twist, (L' U' L U) twice, for the endgame. Its goals and heuristics are
matrix products too: a stage counts solved cubelets in chosen sets
(`h:s_core`), and its heuristic sums square roots of each cubelet's
distance from home (eigencube.py's p-norm with p = 1/2). The distances
come from the cube's 24 rotations (`G`, what the turns reach from the
identity) and their Cayley table (`Tm`), both made from the matrices
once, so the search moves 26 orientation numbers per cube (`h:n_ext`,
checked equal to the matrix turns), and a stage searches only the
cubelets it looks at (`h:p_roject`: each cubelet moves by its own
position alone).

eigencube.py knows no move sequences: it finds every maneuver by
search. That is quick for the top layer, but a middle or bottom
cubelet needs a maneuver that breaks the solved layers and mends them,
about ten moves during which the heuristic only gets worse, and the
search wanders through hundreds of thousands of cubes (minutes in
X_eTaL). So `1 ec:s_olve` (the page and the command line) searches the
middle and bottom stages over macros instead of single turns: D turns
and the sequences a person solving by hand knows (an edge inserted into
the middle layer, a bottom edge flip, Sune, a corner cycle), each
checked to keep the top layer whole. The goals, the heuristic and the
search are eigencube.py's; a stage now needs one to four macros.
`0 ec:s_olve` is eigencube.py's search alone.

## Status

- The model, the turns, the stickers and the solver are complete. A
  20- to 30-move scramble solves in 2 to 3 seconds at the command line
  (24 seeds, 112 to 176 moves) and in about 3 seconds in the page.
- The solutions are long: eigencube.py's are too (a layer method with a
  cubelet at a time), and the macros are joined as they are, so a D
  next to a D' is not canceled.
- `0 ec:s_olve`, eigencube.py's search on every stage, is correct but
  takes minutes on the bottom layer.

## Run it

```bash
just run eigencube          # a new scramble, its stickers, the solution, the solved cube
just show eigencube         # as a notebook: each statement, then its output
just test-demo eigencube    # its CLI and browser baselines and the web app's tests
just serve eigencube        # the web app at http://127.0.0.1:8413/
```

## Workarounds

- The page imports the library through `xetal-store` directly (it
  installs a store in memory and writes `Eigencube.xtl` into it):
  `xetal-play` has no way to give a run a library of the page's own
  ([`docs/xetal-asks.md`](../../docs/xetal-asks.md), "a page's own
  macro library", as stencil-macros).
