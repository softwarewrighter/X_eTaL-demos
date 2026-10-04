use microscope::chrome::{about, About};

#[test]
fn a_title_leads_to_wikipedia_else_a_story() {
    let wiki = "slug = \"x\"\nwiki = \"https://en.wikipedia.org/wiki/Langton%27s_ant\"\nstory = \"ignored\"\n";
    assert_eq!(about(wiki), About::Wiki("https://en.wikipedia.org/wiki/Langton%27s_ant".into()));
    let story = "slug = \"x\"\nwiki = \"\"\nstory = \"A \\\"quoted\\\" game.\\n\\nSecond part.\"\n";
    assert_eq!(about(story), About::Story("A \"quoted\" game.\n\nSecond part.".into()));
    assert_eq!(about("slug = \"x\"\n"), About::Nothing);
}
