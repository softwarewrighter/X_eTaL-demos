# Benchmarks

Written by `just bench` (tools/bench) with X_eTaL 1c1617e; the
baseline `just bench-check` compares against. Natively, release,
through the vendored `xetal-play`, best of 7 runs. The ratio is the
time over a fixed pure-Rust reference loop (29.8 ms in this
run), so a busier or slower machine changes it less than the time.
`just bench-check` fails when a case's ratio is more than 15% above
this table's.

| Case | What | ms | Ratio | Change |
| ---- | ---- | -- | ----- | ------ |
| nbody | 50 bodies, 10 steps | 27.1 | 0.909 | +49% |
| image-pipeline | 96 x 96, three filters, pooling | 56.5 | 1.880 | -12% |
| reaction-diffusion | 64 x 64, 20 steps | 63.3 | 2.125 | +8% |
| wave-tank | 60 x 96, 4 steps | 24.1 | 0.806 | +1% |
| mandelbrot | 90 x 135, 32 steps | 211.9 | 7.097 | +103% |
| julia | 90 x 135, 32 steps | 200.1 | 6.702 | +1% |
| life-microscope | 64 x 64, one generation | 10.8 | 0.362 | -1% |
| ca-lab | 48 x 64 Life, 4 steps | 16.0 | 0.542 | +2% |
| langtons-ant | 64 x 64, 50 steps | 78.7 | 2.631 | -0% |
| fourier-epicycles | 128 points: transform, chain, errors | 73.4 | 2.488 | -2% |
| elementwise | x + x * x, 8 times | 95.6 | 3.247 | +12% |
| reduce | '+ r_/_2 x, 8 times | 84.0 | 2.850 | -4% |
| scan | '+ s_\_2 on 4096 rows of 64, once | 183.2 | 6.123 | +2% |
| each | a lambda on 16384 items, 8 times | 63.5 | 2.118 | -3% |
| table | a 512 x 512 '* t_able, twice | 29.1 | 0.984 | -0% |
| inner | (32 x 256) '+ '* i_nner (256 x 32), once | 28.6 | 0.958 | -1% |
| rotate | 1 o_-_2 x, 16 times | 98.3 | 3.312 | -1% |
| transpose | o_\ x, 32 times | 53.7 | 1.798 | -2% |
