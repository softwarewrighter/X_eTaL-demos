# Benchmarks

Written by `just bench` (tools/bench) with X_eTaL 1c1617e; the
baseline `just bench-check` compares against. Natively, release,
through the vendored `xetal-play`, best of 7 runs. The ratio is the
time over a fixed pure-Rust reference loop (29.1 ms in this
run), so a busier or slower machine changes it less than the time.
`just bench-check` fails when a case's ratio is more than 15% above
this table's.

| Case | What | ms | Ratio | Change |
| ---- | ---- | -- | ----- | ------ |
| nbody | 50 bodies, 10 steps | 26.2 | 0.900 | -1% |
| image-pipeline | 96 x 96, three filters, pooling | 55.1 | 1.870 | -1% |
| reaction-diffusion | 64 x 64, 20 steps | 60.9 | 2.063 | -3% |
| wave-tank | 60 x 96, 4 steps | 23.7 | 0.821 | +2% |
| mandelbrot | 90 x 135, 32 steps | 199.9 | 6.897 | -3% |
| julia | 90 x 135, 32 steps | 187.1 | 6.408 | -4% |
| life-microscope | 64 x 64, one generation | 10.6 | 0.367 | +1% |
| ca-lab | 48 x 64 Life, 4 steps | 15.7 | 0.545 | +1% |
| langtons-ant | 64 x 64, 50 steps | 78.1 | 2.649 | +1% |
| fourier-epicycles | 128 points: transform, chain, errors | 71.9 | 2.486 | -0% |
| sandpile | 81 x 81, 10000 grains, 20 rounds | 49.8 | 1.722 | new |
| elementwise | x + x * x, 8 times | 92.4 | 3.183 | -2% |
| reduce | '+ r_/_2 x, 8 times | 82.7 | 2.857 | +0% |
| scan | '+ s_\_2 on 4096 rows of 64, once | 181.1 | 6.128 | +0% |
| each | a lambda on 16384 items, 8 times | 64.9 | 2.189 | +3% |
| table | a 512 x 512 '* t_able, twice | 28.5 | 0.977 | -1% |
| inner | (32 x 256) '+ '* i_nner (256 x 32), once | 28.1 | 0.969 | +1% |
| rotate | 1 o_-_2 x, 16 times | 96.3 | 3.305 | -0% |
| transpose | o_\ x, 32 times | 51.5 | 1.746 | -3% |
