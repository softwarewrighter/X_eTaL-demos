use microscope::source::{between, find, NONE};

#[test]
fn ranges() {
    let src = "a := 1\nb := { x ->\n  x\n}\nc := 2\n";
    assert_eq!(&src[between(src, "b := ", "\n}").0..between(src, "b := ", "\n}").1], "b := { x ->\n  x\n}");
    assert_eq!(&src[find(src, "c := 2").0..find(src, "c := 2").1], "c := 2");
    assert_eq!(find(src, "zzz"), NONE);
}

#[test]
fn a_range_to_a_marker_on_its_own_line_ends_after_it() {
    let src = "x := 1\ny := 2\nz := 3\n";
    let (a, b) = between(src, "x := ", "y := ");
    assert_eq!(&src[a..b], "x := 1\ny := 2");
    let (a, b) = between(src, "z := ", "z := ");
    assert_eq!(&src[a..b], "z := 3");
}

#[test]
fn a_listing_keeps_every_byte_and_folds_only_data() {
    use microscope::source::lines;
    let data: Vec<String> = (0..60).map(|i| format!("{}.5", i)).collect();
    let src = format!("g := 1.0\nm := {}\nu:f := {{ x -> x + 1 }}\nu:f 2\n", data.join(" "));
    let ls = lines(&src);
    // The lines' spans (and the newlines) give the program back; an
    // unfolded line's segments cover it exactly, a folded one's its start.
    let back: Vec<&str> = ls.iter().map(|l| &src[l.raw.0..l.raw.1]).collect();
    assert_eq!(back.join("\n"), src);
    for l in &ls {
        let text: String = l.segments.iter().map(|(_, _, (a, b))| &src[*a..*b]).collect();
        let line = &src[l.raw.0..l.raw.1];
        if l.folded() {
            assert!(line.starts_with(&text) && text.len() < 80, "{text}");
        } else {
            assert_eq!(text, line);
        }
    }
    let folded: Vec<bool> = ls.iter().map(|l| l.folded()).collect();
    assert_eq!(folded, vec![false, true, false, false, false]);
    assert_eq!(ls[1].numbers, 60);
}
