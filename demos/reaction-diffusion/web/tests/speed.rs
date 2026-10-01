use reaction_diffusion_web::micro::{preset, run, Grid};

#[test]
#[ignore = "timing: cargo test --release -- --ignored --nocapture"]
fn frame_cost() {
    for (n, steps) in [(64, 1), (64, 20), (80, 20)] {
        let g = Grid::seeded(n);
        let t = std::time::Instant::now();
        run(&g, &preset(0), steps).unwrap();
        eprintln!("{n} x {n}, {steps} steps: {:?}", t.elapsed());
    }
}
