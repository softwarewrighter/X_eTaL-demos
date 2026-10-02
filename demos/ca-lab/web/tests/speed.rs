use ca_lab_web::micro::{first_row, grow, rules, start, step};

#[test]
#[ignore = "timing: cargo test --release -- --ignored --nocapture"]
fn timings() {
    let t = std::time::Instant::now();
    grow(30, &first_row(false), 10).unwrap();
    eprintln!("1-D history: {:?}", t.elapsed());
    let t = std::time::Instant::now();
    step(&rules()[0].table, &start(0), 4).unwrap();
    eprintln!("2-D, 4 steps: {:?}", t.elapsed());
}
