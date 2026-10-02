use langtons_ant_web::micro::{run, Ant};

#[test]
#[ignore = "timing: cargo test --release -- --ignored --nocapture"]
fn timing() {
    let t = std::time::Instant::now();
    run(&Ant::start(), 200).unwrap();
    eprintln!("200 steps on 64 x 64: {:?}", t.elapsed());
}
