# Benchmarks

Written by `just bench` (tools/bench) with X_eTaL 081fb3f; the
baseline `just bench-check` compares against. Natively, release,
through the vendored `xetal-play`, best of 7 runs. The ratio is the
time over a fixed pure-Rust reference loop (29.4 ms in this
run), so a busier or slower machine changes it less than the time.
`just bench-check` fails when a case's ratio is more than 15% above
this table's.

| Case | What | ms | Ratio | Change |
| ---- | ---- | -- | ----- | ------ |
| nbody | 50 bodies, 10 steps | 55.6 | 1.894 | -0% |
| image-pipeline | 96 x 96, three filters, pooling | 123.8 | 4.185 | +1% |
| reaction-diffusion | 64 x 64, 20 steps | 63.1 | 2.120 | -0% |
| wave-tank | 60 x 96, 4 steps | 26.8 | 0.904 | -0% |
| mandelbrot | 90 x 135, 32 steps | 215.4 | 7.250 | -2% |
| julia | 90 x 135, 32 steps | 208.2 | 7.008 | -8% |
| life-microscope | 64 x 64, one generation | 10.4 | 0.352 | -3% |
| ca-lab | 48 x 64 Life, 4 steps | 15.3 | 0.519 | -3% |
| langtons-ant | 64 x 64, 50 steps | 133.5 | 4.542 | +0% |
| elementwise | x + x * x, 8 times | 95.8 | 3.229 | +1% |
| reduce | '+ r_/_2 x, 8 times | 79.6 | 2.700 | -2% |
| scan | '+ s_\_2 on 4096 rows of 64, once | 166.6 | 5.605 | -4% |
| each | a lambda on 16384 items, 8 times | 79.1 | 2.687 | +1% |
| table | a 512 x 512 '* t_able, twice | 157.7 | 5.356 | -1% |
| inner | (32 x 256) '+ '* i_nner (256 x 32), once | 203.8 | 6.894 | -4% |
| rotate | 1 o_-_2 x, 16 times | 95.8 | 3.253 | +2% |
| transpose | o_\ x, 32 times | 50.8 | 1.726 | -3% |
