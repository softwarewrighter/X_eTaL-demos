//! Around every demo page: the header (logo, title, lede), stage chips,
//! panels, notices and the footer (as the X_eTaL live demo shows it).

use yew::prelude::*;

use crate::source::{code, shape};

pub const REPO: &str = "https://github.com/softwarewrighter/X_eTaL-demos";

/// Where a demo's title leads: the Wikipedia article on its subject
/// (the game, puzzle or mathematics), or, when there is none, a short
/// story (its history and how it plays) shown in a dialog.
#[derive(Clone, Debug, PartialEq)]
pub enum About {
    Wiki(String),
    Story(String),
    Nothing,
}

/// The value of `key = "..."` in a demo.toml (a basic TOML string).
fn toml_string(toml: &str, key: &str) -> Option<String> {
    let line = toml.lines().find(|l| l.split('=').next().is_some_and(|k| k.trim() == key))?;
    let v = line.split_once('=')?.1.trim();
    let v = v.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = v.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => out.push(match chars.next()? {
                'n' => '\n',
                't' => '\t',
                other => other,
            }),
            '"' => break,
            c => out.push(c),
        }
    }
    Some(out).filter(|s| !s.is_empty())
}

/// What a demo's title leads to, from its demo.toml (`wiki`, else `story`).
pub fn about(demo_toml: &str) -> About {
    match (toml_string(demo_toml, "wiki"), toml_string(demo_toml, "story")) {
        (Some(w), _) => About::Wiki(w),
        (None, Some(s)) => About::Story(s),
        _ => About::Nothing,
    }
}

/// Two overlapping pages: the mark of a link to an article.
fn pages_glyph() -> Html {
    html! {
        <svg class="pages" viewBox="0 0 16 16" width="0.8em" height="0.8em" aria-hidden="true">
            <path d="M5 1.5h6l3 3v8h-9z" fill="none" stroke="currentColor" stroke-width="1.3" />
            <path d="M3 4.5v10h8" fill="none" stroke="currentColor" stroke-width="1.3" />
        </svg>
    }
}

#[derive(Properties, PartialEq)]
struct TitleProps {
    title: String,
    about: About,
}

/// The title: a link to the article (new tab, with the pages mark), or a
/// button opening the story's dialog (closed by Escape, a click outside
/// it, or its X).
#[function_component(Title)]
fn title(p: &TitleProps) -> Html {
    let open = use_state(|| false);
    {
        let open = open.clone();
        use_effect_with(*open, move |&is_open| {
            let listener = is_open.then(|| {
                let open = open.clone();
                let f = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
                    if e.key() == "Escape" {
                        open.set(false);
                    }
                });
                if let Some(w) = web_sys::window() {
                    let _ = w.add_event_listener_with_callback("keydown", wasm_bindgen::JsCast::unchecked_ref(f.as_ref()));
                }
                f
            });
            move || {
                if let (Some(f), Some(w)) = (listener, web_sys::window()) {
                    let _ = w.remove_event_listener_with_callback("keydown", wasm_bindgen::JsCast::unchecked_ref(f.as_ref()));
                }
            }
        });
    }
    match &p.about {
        About::Wiki(url) => html! {
            <h1><a class="wiki" href={url.clone()} target="_blank" rel="noopener noreferrer"
                title={format!("{} on Wikipedia (opens in a new tab)", p.title)}>{p.title.clone()}{" "}{pages_glyph()}</a></h1>
        },
        About::Story(story) => {
            let show = { let open = open.clone(); Callback::from(move |_: MouseEvent| open.set(true)) };
            let hide = { let open = open.clone(); Callback::from(move |_: MouseEvent| open.set(false)) };
            let keep = Callback::from(|e: MouseEvent| e.stop_propagation());
            html! { <>
                <h1><button class="story" onclick={show} title={format!("About {}", p.title)}>{p.title.clone()}</button></h1>
                { for open.then(|| html! {
                    <div class="dialog-backdrop" onclick={hide.clone()}>
                        <div class="dialog" role="dialog" aria-modal="true" aria-label={format!("About {}", p.title)} onclick={keep}>
                            <button class="dialog-x" aria-label="Close" onclick={hide.clone()}>{"\u{00d7}"}</button>
                            <h2>{p.title.clone()}</h2>
                            { for story.split("\n\n").map(|para| html! { <p>{para.to_string()}</p> }) }
                        </div>
                    </div>
                }) }
            </> }
        }
        About::Nothing => html! { <h1>{p.title.clone()}</h1> },
    }
}

/// The logo (linking back to the catalog), the title (leading to what
/// the demo is about, `about`) and a lede.
pub fn header(title: &str, lede: &str, about: About) -> Html {
    html! { <>
        <div class="brand">
            <a href="../" title="All demos"><img class="logo" src="modern-xetal-logo.jpg" alt="X_eTaL" /></a>
            <Title title={title.to_string()} {about} />
        </div>
        <p class="lede">{lede}</p>
    </> }
}

/// One stage of the computation: its name, its code, its result's shape.
pub fn chip(name: &str, src: &str, dims: &[usize], meaning: &str, active: bool, onclick: Callback<MouseEvent>) -> Html {
    html! {
        <button class={classes!("stage", active.then_some("active"))} {onclick}>
            <span class="sname">{name}</span>
            { code(src) }
            { shape(dims, meaning) }
        </button>
    }
}

/// A panel: a title followed by its code, a note, a body; outlined when
/// it shows the stage in focus.
pub fn panel(title: &str, src: &str, note: &str, focus: bool, body: Html) -> Html {
    html! {
        <section class={classes!("panel", focus.then_some("focus"))}>
            <h2>{title}{" "}{code(src)}</h2>
            <p class="note">{note}</p>
            {body}
        </section>
    }
}

/// Why the last change was not shown, if there is a reason.
pub fn notice(text: &Option<String>) -> Html {
    html! { for text.iter().map(|n| html! { <p class="notice" role="status">{n}</p> }) }
}

fn sep() -> Html {
    html! { <span class="sep">{ "\u{00b7}" }</span> }
}

/// The footer, as the X_eTaL live demo shows it, plus the vendored
/// X_eTaL commit and the way back to the catalog.
pub fn footer() -> Html {
    html! {
        <footer>
            <span>{ "Copyright (c) 2026 Michael A Wright" }</span>{ sep() }
            <span>{ "MIT License" }</span>{ sep() }
            <a href={REPO} target="_blank">{ "Repository" }</a>{ sep() }
            <a href="../">{ "All demos" }</a>{ sep() }
            <span>{ format!("X_eTaL {}", env!("XETAL_SHA")) }</span>{ sep() }
            <span>{ format!("build (host {}, sha {}, {})", env!("BUILD_HOST"), env!("BUILD_SHA"), env!("BUILD_TIMESTAMP")) }</span>
        </footer>
    }
}
