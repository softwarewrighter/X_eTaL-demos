use stencil_macros_web::micro::{
    call, direct, expanded_step, num, picture, program, run, terms, Kernel, COLS, PRESETS, ROWS, SOURCE,
};

fn close(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f64::max)
}

#[test]
fn the_page_runs_the_command_line_programs_picture() {
    assert!(picture().starts_with("rows := 24") && picture().contains("img := "));
    assert!(SOURCE.contains(picture()));
    assert!(SOURCE.contains("\"s:\" u_se< \"Stencil\""));
}

#[test]
fn every_preset_is_the_direct_loop() {
    for (i, p) in PRESETS.iter().enumerate() {
        let k = Kernel::preset(i);
        let r = run(&k).unwrap_or_else(|e| panic!("{}: {e}", p.name));
        assert_eq!(r.img.len(), ROWS * COLS);
        let d = direct(&k, &r.img);
        assert!(close(&r.out, &d) < 1e-9, "{}: {}", p.name, close(&r.out, &d));
    }
}

#[test]
fn an_edited_kernel_and_its_resizing_run_too() {
    let mut k = Kernel::preset(0);
    k.w[0] = 0.5;
    k.w[8] = -3.25;
    k.divide = 2.0;
    k.steps = 3;
    for k in [k.clone(), k.resized(5), k.resized(5).resized(3)] {
        let r = run(&k).unwrap();
        assert!(close(&r.out, &direct(&k, &r.img)) < 1e-9);
    }
    assert_eq!(k.resized(5).resized(3), k);
}

#[test]
fn the_expansion_writes_one_term_per_nonzero_number() {
    for (i, p) in PRESETS.iter().enumerate() {
        let k = Kernel::preset(i);
        let line = expanded_step(&k).unwrap();
        assert!(!line.contains("s:s_tencil<"), "{}: {line}", p.name);
        let (terms, ones, _, _) = k.counts();
        assert_eq!(line.matches(" + ").count(), terms - 1, "{}: {line}", p.name);
        // A 1 writes no multiply: the only multiplies are the other weights.
        let others = k.w.iter().filter(|&&x| x != 0.0 && x != 1.0 && x != -1.0).count();
        assert_eq!(line.matches(" * ").count(), others, "{}: {line}", p.name);
        assert!(ones <= terms);
    }
}

#[test]
fn the_heat_step_expands_as_the_readme_says() {
    let k = Kernel { side: 3, w: vec![0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0], divide: 1.0, steps: 1 };
    assert_eq!(
        expanded_step(&k).unwrap(),
        "u:s_tep := { p -> ((-1 o_-_1 p) + (-1 o_-_2 p) + (-4 * p) + (1 o_-_2 p) + (1 o_-_1 p)) }"
    );
}

#[test]
fn a_cells_terms_sum_to_its_result() {
    let k = Kernel::preset(3);
    let r = run(&k).unwrap();
    for (y, x) in [(0, 0), (11, 12), (23, 47), (5, 30)] {
        let s: f64 = terms(&k, &r.img, y, x).iter().map(|(w, _, _, v)| w * v).sum();
        assert!((s / k.divide - r.out[y * COLS + x]).abs() < 1e-9);
    }
}

#[test]
fn a_kernel_that_is_not_square_is_refused_by_the_macro() {
    let k = Kernel { side: 2, w: vec![1.0, 1.0, 1.0, 1.0], divide: 1.0, steps: 1 };
    let e = run(&k).unwrap_err();
    assert!(e.contains("square kernel"), "{e}");
}

#[test]
fn numbers_are_written_as_written() {
    assert_eq!(num(2.0), "2");
    assert_eq!(num(-1.0), "-1");
    assert_eq!(num(0.2), "0.2");
    assert_eq!(call(&Kernel::preset(0)), "u:s_tep := { p -> (\"1 2 1  2 4 2  1 2 1\" s:s_tencil< \"p\") / 16 }");
    assert!(program(&Kernel::preset(1)).contains("u:s_tep := { p -> \"-1 -1 -1  -1 8 -1  -1 -1 -1\" s:s_tencil< \"p\" }"));
}
