# Benchmarks

Written by `just bench` (tools/bench) with X_eTaL abb8274; the
baseline `just bench-check` compares against. Natively, release,
through the vendored `xetal-play`, best of 7 runs. The ratio is the
time over a fixed pure-Rust reference loop (29.7 ms in this
run), so a busier or slower machine changes it less than the time.
`just bench-check` fails when a case's ratio is more than 15% above
this table's.

| Case | What | ms | Ratio | Change |
| ---- | ---- | -- | ----- | ------ |
| nbody | 50 bodies, 10 steps | 55.6 | 1.843 | +2% |
| image-pipeline | 96 x 96, three filters, pooling | 118.2 | 3.954 | -1% |
| reaction-diffusion | 64 x 64, 20 steps | 65.4 | 2.164 | +0% |
| wave-tank | 60 x 96, 4 steps | 28.0 | 0.917 | +2% |
| mandelbrot | 90 x 135, 32 steps | 232.3 | 7.643 | -1% |
| julia | 90 x 135, 32 steps | 241.4 | 7.776 | +6% |
| life-microscope | 64 x 64, one generation | 10.5 | 0.348 | +2% |
| ca-lab | 48 x 64 Life, 4 steps | 15.9 | 0.525 | +1% |
| langtons-ant | 64 x 64, 50 steps | 137.2 | 4.461 | +6% |
| elementwise | x + x * x | 24.8 | 0.804 | -1% |
| reduce | '+ r_/_2 x | 20.5 | 0.657 | -1% |
| scan | '+ s_\_2 on 4096 rows of 64 | 331.4 | 10.937 | -4% |
| each | a lambda on 16384 items | 20.0 | 0.657 | -1% |
| table | a 512 x 512 '* t_able | 151.3 | 4.953 | +1% |
| inner | (32 x 256) '+ '* i_nner (256 x 32) | 358.0 | 11.895 | +10% |
| rotate | 1 o_-_2 x | 12.2 | 0.408 | +1% |
| transpose | o_\ x | 4.3 | 0.142 | -6% |
