# MoE routing microscope

A mixture-of-experts layer sends each token to only a few of its
experts. Here a sentence's tokens are scored against 16 experts by one
matrix product, each token's scores become probabilities (softmax),
each token keeps its two most likely experts (top-2), and their
probabilities, renormalized, are the gates. The experts sit on a 4 x 4
grid of feature pairs (animal, number, colour, action by place, food,
function word, time), so words route by meaning: "red" to colour
experts, "park" to place experts, "fish" (an animal and a food) to the
animal x food expert. Type your own sentence.

Live: [MoE routing microscope](https://softwarewrighter.github.io/X_eTaL-demos/moe-router/)

[![MoE routing microscope: the live page](screenshot.png)](https://softwarewrighter.github.io/X_eTaL-demos/moe-router/)

## The program

From `moe-router.xtl` (the live page runs these sections on your
sentence):

```
u:s_cores := { x -> 2.0 * x '+ '* i_nner W }
u:s_oftmax := { s ->
  e := e_xp s - ('m_ax r_/_2 s) 'l_eft t_able o_ffsets 16
  e / ('+ r_/_2 e) 'l_eft t_able o_ffsets 16
}
u:t_op2 := { p ->
  m1 := 'm_ax r_/_2 p
  s1 := f_loat p = m1 'l_eft t_able o_ffsets 16
  p2 := p - 2.0 * s1
  m2 := 'm_ax r_/_2 p2
  s2 := f_loat p2 = m2 'l_eft t_able o_ffsets 16
  (p * s1 + s2) / (m1 + m2) 'l_eft t_able o_ffsets 16
}
u:l_oad := { g -> '+ r_/ f_loat g > 0.0 }
```

## How it works

| Stage | Code | Shape | What it is |
| ----- | ---- | ----- | ---------- |
| embed | `ids s_elect E` | tokens 8 | each word's row of the embedding table: 8 features plus a small fixed ripple |
| scores | `u:s_cores x` | tokens 16 | one matrix product with the router weights `W` (8 x 16) |
| softmax | `u:s_oftmax` | tokens 16 | each row as probabilities; the row's largest is taken off first so nothing overflows |
| top-2 | `u:t_op2 p` | tokens 16 | the gates: the two largest probabilities of each row over their sum, 0 elsewhere |
| load | `u:l_oad gates` | 16 | how many tokens each expert got |

Row-wise operations are reductions along axis 2 (`'m_ax r_/_2`,
`'+ r_/_2`) spread back over the row with `t_able`. The vocabulary (37
words, the last `?` for any other word) and the router weights are
written in the program: `W` gives expert (r, c) on the grid weight 2
for feature r and feature 4 + c, plus a ripple.

On the page, the routing picture draws a curve from each token to its
two experts, as wide as the gate; experts are shaded by load and the
selected token (cycling, or clicked) is drawn in colour. The arrays
are shown as pictures (embeddings, scores, probabilities, gates) with
the load as bars, and the inspector shows a token's embedding, its
scores and probabilities on the expert grid, and its gates' arithmetic.

The page's tests check that words are looked up (plurals, unknown
words as `?`), that each row of probabilities sums to 1 and follows
the scores (p_e / p_f = exp(s_e - s_f)), that the top-2 is the two
largest by a direct sort, that the gates sum to 1, that the load counts
each expert's tokens, and that experts specialise by feature.

## Run it

```bash
just run moe-router          # "the red fox eats fish in the park": experts, gates, load
just show moe-router         # the same as a notebook
just serve moe-router        # the web app at http://127.0.0.1:8095/
just test-demo moe-router    # its CLI and browser baselines and the web app's tests
```

## Workarounds

The top-2 is found with masks (the largest of each row, taken out,
then the largest again) because `g_rade` cannot grade each row of a
matrix (an ask in [`docs/xetal-asks.md`](../../docs/xetal-asks.md)).
The page passes the word numbers as Floats, floored, because long Int
strands are slow to read (the same file).
