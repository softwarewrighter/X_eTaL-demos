# Benchmarks

Written by `just bench` (tools/bench) with X_eTaL 1c1617e; the
baseline `just bench-check` compares against. Natively, release,
through the vendored `xetal-play`, best of 7 runs. The ratio is the
time over a fixed pure-Rust reference loop (29.7 ms in this
run), so a busier or slower machine changes it less than the time.
`just bench-check` fails when a case's ratio is more than 15% above
this table's.

| Case | What | ms | Ratio | Change |
| ---- | ---- | -- | ----- | ------ |
| nbody | 50 bodies, 10 steps | 26.8 | 0.901 | -52% |
| image-pipeline | 96 x 96, three filters, pooling | 56.2 | 1.872 | -55% |
| reaction-diffusion | 64 x 64, 20 steps | 64.5 | 2.168 | +2% |
| wave-tank | 60 x 96, 4 steps | 24.3 | 0.821 | -9% |
| mandelbrot | 90 x 135, 32 steps | 212.2 | 7.109 | -2% |
| julia | 90 x 135, 32 steps | 200.3 | 6.718 | -4% |
| life-microscope | 64 x 64, one generation | 11.3 | 0.368 | +5% |
| ca-lab | 48 x 64 Life, 4 steps | 15.9 | 0.516 | -0% |
| langtons-ant | 64 x 64, 50 steps | 79.5 | 2.690 | -41% |
| elementwise | x + x * x, 8 times | 96.6 | 3.237 | +0% |
| reduce | '+ r_/_2 x, 8 times | 85.6 | 2.868 | +6% |
| scan | '+ s_\_2 on 4096 rows of 64, once | 186.7 | 6.306 | +13% |
| each | a lambda on 16384 items, 8 times | 66.8 | 2.221 | -17% |
| table | a 512 x 512 '* t_able, twice | 29.3 | 0.963 | -82% |
| inner | (32 x 256) '+ '* i_nner (256 x 32), once | 29.0 | 0.974 | -86% |
| rotate | 1 o_-_2 x, 16 times | 98.6 | 3.300 | +1% |
| transpose | o_\ x, 32 times | 53.1 | 1.774 | +3% |
