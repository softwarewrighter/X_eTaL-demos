use image_pipeline_web::micro::{core, direct, prelude, run, turn, Setup, BLURS, EDGES, HALF, SCENES, SIZE, SOURCE};

const N: usize = SIZE;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn the_page_runs_the_command_line_programs_sections() {
    assert!(prelude().contains("row := "));
    assert!(core().contains("u:w_indows := { x ->"));
    assert!(SOURCE.contains(prelude()) && SOURCE.contains(core()));
}

#[test]
fn every_scene_runs_with_every_kernel() {
    for (i, (name, _)) in SCENES.iter().enumerate() {
        for (b, e) in BLURS.iter().zip(EDGES.iter().cycle()) {
            let s = Setup { blur: b.1, kx: e.1, ..Setup::new(i) };
            let a = run(&s).unwrap_or_else(|err| panic!("{name}: {err}"));
            assert!(a.img.iter().all(|&v| (0.0..=1.0).contains(&v)), "{name}: brightness 0 to 1");
        }
    }
}

#[test]
fn filters_agree_with_a_direct_convolution() {
    let a = run(&Setup::new(0)).unwrap();
    let s = Setup::new(0);
    let want = |k, x: &[f64]| direct(k, x, N, N);
    let (smooth, gx, gy) = (want(&s.blur, &a.img), want(&s.kx, &a.smooth), want(&turn(&s.kx), &a.smooth));
    for i in 0..N * N {
        assert!(close(a.smooth[i], smooth[i]) && close(a.gx[i], gx[i]) && close(a.gy[i], gy[i]), "pixel {i}");
        assert!(close(a.mag[i], (gx[i] * gx[i] + gy[i] * gy[i]).sqrt()));
        assert_eq!(a.edges[i], if a.mag[i] > s.thresh { 1.0 } else { 0.0 });
    }
}

#[test]
fn an_edited_kernel_is_used() {
    let mut s = Setup::new(0);
    s.blur = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
    let a = run(&s).unwrap();
    // The right-hand neighbour: smooth[r][c] = img[r][c + 1].
    for r in 0..N {
        for c in 0..N {
            assert_eq!(a.smooth[r * N + c], a.img[r * N + (c + 1) % N]);
        }
    }
}

#[test]
fn the_windows_are_the_pixels_neighbours() {
    let mut s = Setup::new(0);
    s.pixel = (0, 5);
    let a = run(&s).unwrap();
    for (k, &w) in a.win_img.iter().enumerate() {
        let (dy, dx) = (k / 3, k % 3);
        let (r, c) = ((N + dy - 1) % N, (5 + dx - 1) % N);
        assert_eq!(w, a.img[r * N + c], "window item {k} wraps round the top edge");
    }
    let dot: f64 = (0..9).map(|k| s.blur[k] * a.win_img[k]).sum();
    assert!(close(dot, a.smooth[5]));
}

#[test]
fn edge_orientation_on_a_known_picture() {
    // A vertical edge (dark left, bright right): gx > 0, gy = 0 on it.
    let half = SCENES.iter().position(|s| s.0 == "Half and half").unwrap();
    let a = run(&Setup::new(half)).unwrap();
    let (r, c) = (N / 2, N / 2);
    assert!(a.gx[r * N + c] > 1.0 && a.gx[r * N + c - 1] > 1.0);
    assert!(a.gy.iter().all(|&g| g.abs() < 1e-12), "no vertical change anywhere");
    // The same picture turned: a horizontal edge, gy > 0 and gx = 0.
    let s = Setup { scene: "img := 0.2 + 0.6 * f_loat row >= rows d_iv 2\n".into(), ..Setup::new(0) };
    let a = run(&s).unwrap();
    assert!(a.gy[r * N + c] > 1.0);
    assert!(a.gx.iter().all(|&g| g.abs() < 1e-12));
    // A diagonal edge: gx = -gy along it (bright below-left).
    let s = Setup { scene: "img := 0.2 + 0.6 * f_loat row > col\n".into(), ..Setup::new(0) };
    let a = run(&s).unwrap();
    let i = 40 * N + 40;
    assert!(a.mag[i] > 1.0 && close(a.gx[i], -a.gy[i]));
}

#[test]
fn pooling_takes_the_largest_of_each_two_by_two_block() {
    let a = run(&Setup::new(1)).unwrap();
    for r in 0..HALF {
        for c in 0..HALF {
            let m = [(0, 0), (0, 1), (1, 0), (1, 1)].iter().map(|(dr, dc)| a.mag[(2 * r + dr) * N + 2 * c + dc]).fold(f64::MIN, f64::max);
            assert_eq!(a.pool[r * HALF + c], m);
        }
    }
}
