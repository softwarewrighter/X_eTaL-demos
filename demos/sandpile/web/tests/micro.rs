use sandpile_web::micro::{core, direct_round, run, Pile, MID, SIDE, SOURCE};

fn settled(mut p: Pile) -> (Pile, usize) {
    let mut rounds = 0;
    while !p.stable() {
        p = run(&p, 50).unwrap().next;
        rounds += 50;
        assert!(rounds < 20000, "never settled");
    }
    (p, rounds)
}

#[test]
fn the_page_runs_the_command_line_programs_core() {
    assert!(core().contains("u:t_opple := { h ->") && core().contains("u:s_tep := { s ->"));
    assert!(SOURCE.contains(core()));
}

#[test]
fn a_round_is_the_direct_round() {
    let mut p = Pile::empty();
    p.drop(MID, MID, 777);
    p.drop(10, 70, 33);
    p.everywhere(2);
    for _ in 0..5 {
        let x = run(&p, 1).unwrap().next;
        let d = direct_round(&p);
        assert_eq!(x, d);
        p = x;
    }
}

#[test]
fn a_settled_pile_is_stable_and_keeps_its_grains_inside() {
    let mut p = Pile::empty();
    p.drop(MID, MID, 2000);
    let (s, _) = settled(p);
    assert!(s.stable());
    assert_eq!(s.grains(), 2000, "2000 grains spread less far than the edge");
}

#[test]
fn grains_are_conserved_except_at_the_edge() {
    let mut p = Pile::empty();
    p.everywhere(3);
    p.drop(MID, MID, 1);
    let before = p.grains();
    let a = run(&p, 400).unwrap();
    // What the inner cells gave the edge is lost: each topple of a cell
    // next to the edge gives one grain per edge neighbor.
    let s = SIDE;
    let mut lost = 0;
    for r in 1..s - 1 {
        for c in 1..s - 1 {
            let edges = [r == 1, r == s - 2, c == 1, c == s - 2].iter().filter(|&&e| e).count() as i64;
            lost += edges * a.next.topples[r * s + c];
        }
    }
    assert_eq!(a.next.grains(), before - lost);
}

#[test]
fn the_order_of_dropping_does_not_matter() {
    let mut a = Pile::empty();
    a.drop(MID, MID, 600);
    let (a, _) = settled(a);
    let mut a2 = a.clone();
    a2.drop(20, 25, 500);
    let (a2, _) = settled(a2);
    let mut b = Pile::empty();
    b.drop(20, 25, 500);
    let (b, _) = settled(b);
    let mut b2 = b.clone();
    b2.drop(MID, MID, 600);
    let (b2, _) = settled(b2);
    assert_eq!(a2.h, b2.h);
}

#[test]
fn a_center_drop_has_four_fold_symmetry() {
    let mut p = Pile::empty();
    p.drop(MID, MID, 1500);
    let (s, _) = settled(p);
    let at = |r: usize, c: usize| s.h[r * SIDE + c];
    for r in 0..SIDE {
        for c in 0..SIDE {
            assert_eq!(at(r, c), at(c, SIDE - 1 - r));
            assert_eq!(at(r, c), at(SIDE - 1 - r, c));
        }
    }
}
