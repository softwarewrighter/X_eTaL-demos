use julia_web::micro::{call, core, counts, program, Set, View, SOURCE};

const SMALL: View = View { rows: 10, cols: 16, cx: 0.0, cy: 0.0, w: 3.2 };

#[test]
fn the_page_runs_the_command_line_programs_core() {
    assert!(core().contains("u:i_terate := { c z0 ->"));
    assert!(SOURCE.contains(core()));
    assert!(SOURCE.contains("julia := (-0.8 c_at 0.156) u:i_terate grid"));
    assert!(SOURCE.contains("mandel := grid u:i_terate 0.0 * grid"));
    assert!(call(Set::Mandelbrot).ends_with("grid u:i_terate 0.0 * grid"));
}

#[test]
fn a_julia_set_is_symmetric_through_the_center() {
    // z -> -z maps a Julia set onto itself: the counts read the same backwards.
    let c = counts(&SMALL, 30, Set::Julia(-0.8, 0.156)).unwrap();
    let rev: Vec<f64> = c.iter().rev().cloned().collect();
    assert_eq!(c, rev);
}

#[test]
fn c_zero_gives_the_unit_disk() {
    // c = 0: z -> z^2 stays bounded exactly when |z0| <= 1.
    let v = View { rows: 9, cols: 9, cx: 0.0, cy: 0.0, w: 4.5 };
    let c = counts(&v, 30, Set::Julia(0.0, 0.0)).unwrap();
    for y in 0..9 {
        for x in 0..9 {
            let (a, b) = v.point(y, x);
            let r2 = a * a + b * b;
            if (r2 - 1.0).abs() > 0.05 {
                assert_eq!(c[y * 9 + x] == 30.0, r2 < 1.0, "z0 = {a} {b}");
            }
        }
    }
}

#[test]
fn the_mandelbrot_set_from_the_same_function() {
    let v = View { rows: 3, cols: 3, cx: 0.0, cy: 0.0, w: 3.0 };
    let c = counts(&v, 20, Set::Mandelbrot).unwrap();
    assert_eq!(c[4], 20.0, "c = 0 is in the set");
    assert_eq!(c[5], 3.0, "c = 1: 0, 1, 2, then 5");
}

#[test]
fn pixels_and_points_agree() {
    let v = View::PICKER;
    for (y, x) in [(0, 0), (10, 20), (47, 71)] {
        assert_eq!(v.pixel(v.point(y, x)), Some((y, x)));
    }
    assert_eq!(v.pixel((5.0, 5.0)), None);
}

#[test]
fn the_program_writes_c_as_decimals() {
    let p = program(&SMALL, 5, Set::Julia(-0.123, 0.745));
    assert!(p.contains("z := (-0.123 c_at 0.745) u:i_terate grid"));
}
