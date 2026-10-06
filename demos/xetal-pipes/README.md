# Unix pipes in X_eTaL

`cat`, `wc`, `grep`, `uniq`, `sort`, `head` and `tail`, each a small
X_eTaL program (`stages/*.xtl`) run as a Unix filter, chained with
ordinary pipes:

```bash
cd demos/xetal-pipes && export PATH="$PWD/bin:$PATH"
xetalcat sample.txt | xetaluniq | xetalwc -l
xetalcat sample.txt | xetalgrep the | xetalsort | xetalhead -n 3
```

Each stage gives the same bytes as the tool it imitates (the tests
compare them on six inputs). Command line only: there is no page.

![Unix pipes in X_eTaL: a terminal recording](demo.gif)

## The stages

| Command | Stage | What it does |
| ------- | ----- | ------------ |
| `xetalcat [FILE]` | `stages/cat.xtl` | the file (X_eTaL reads it with `[]N_GET`), or standard input |
| `xetalwc [-l] [-w] [-c]` | `stages/wc.xtl` | lines, words, characters, in BSD wc's columns |
| `xetalgrep NEEDLE` | `stages/grep.xtl` | the lines holding NEEDLE (a fixed string) |
| `xetaluniq` | `stages/uniq.xtl` | without repeated adjacent lines |
| `xetalsort` | `stages/sort.xtl` | the lines in byte order (as `LC_ALL=C sort`) |
| `xetalhead [-n N]` | `stages/head.xtl` | the first N lines (10) |
| `xetaltail [-n N]` | `stages/tail.xtl` | the last N lines (10) |

`stages/Pipes.xtl` is what they share: reading standard input,
writing standard output exactly, and the arrays below. `bin/xetal<cmd>`
are links to `bin/xetal-stage`, which runs the stage.

## How it works

A stage reads its input as one text and never splits it into lines.
Every character is numbered by its line, a running sum of the
newlines before it (`'+ s_\`); choosing lines is then a mask, and
reordering them a grade:

```
ln := p:l_ines t
starts := (t_ally t) t_ake '& r_/_1 ((o_ffsets k) o_- tt) = needle 'l_eft t_able tt
hits := u_nique starts r_eplicate ln
differs := 1 c_at 1 d_rop '| r_/_2 m != -1 o_-_1 m
rank := g_rade g_rade m
sorted := (g_rade ln s_elect rank) s_elect t
```

| Stage | The array idea |
| ----- | -------------- |
| grep | the text's k rotations (k the needle's length) stacked, compared with the needle's characters, and-reduced: 1 wherever the needle starts; the lines of those characters are the hits, and a line is kept when its number is a member of them |
| uniq | the lines as the rows of a matrix padded with character 0; a line stays when its row differs from the row above (the matrix rotated down one row) |
| sort | the grade of the padded rows gives each line its rank; every character takes its line's rank, and a stable grade of those ranks moves whole lines, in order, at once |
| head, tail | the mask of line numbers up to N, or past the count less N |
| wc | the count of newlines, of word starts (a character that is not blank after one that is), and of characters |

`xetal-pipes.xtl` shows these arrays on `sample.txt`:
`just show xetal-pipes`.

Reading is the one place that is not an array operation: X_eTaL reads
standard input a line at a time. Appending each line to the text
would copy it once per line (20,000 lines took 9 s), so the reader
reads halves one after the other and copies about log n times: each
stage runs on 20,000 lines in about 0.3 s.

## Run it

```bash
just run xetal-pipes             # the arrays inside the stages, on sample.txt
just show xetal-pipes            # the same as a notebook
just test-demo xetal-pipes       # its CLI baseline, and every stage against the Unix tool
just tape xetal-pipes            # record demo.gif again (vhs)
```

## Workarounds

Four X_eTaL asks (in [`docs/xetal-asks.md`](../../docs/xetal-asks.md));
`bin/xetal-stage` and `stages/Pipes.xtl` hold the workarounds:

- End of input: `[]R_EAD` fails at the end of standard input, and the
  failure cannot be caught, so the wrapper writes an end line after
  the input (`<<xetal-end-of-input>>`; `awk 1` first ends a last line
  that has no newline) and the stage reads until it. Input holding
  that exact line would end early.
- Program arguments: `xetal run FILE` takes none, so the wrapper runs
  a copy of the stage with one line put in front of it,
  `args := "..."` (one argument per line), and `XETAL_PATH` points at
  `stages/` so the copy still finds `Pipes.xtl`.
- `xetal run --context` would carry that line without a copy, but it
  reads standard input twice (the program's `[]R_EAD` runs once
  silently, then again).
- Sorting text: `s_ort` and `g_rade` refuse a list of boxed strings,
  so the lines are padded into a character matrix, whose rows they do
  sort.
