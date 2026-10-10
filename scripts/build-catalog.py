#!/usr/bin/env python3
"""Write pages/index.html: the live catalog of the demos.

  scripts/build-catalog.py [OUT]     # default pages/index.html

One card per demo (scripts/demos.py json, catalog order): title,
summary, concepts, status, a link to its live page (pages/<slug>/, when
it has a web app) and to its README. The footer is the X_eTaL live
demo's: copyright, license, the repository, and the build's provenance
build (host, this repo's sha, yyyymmddThhmmss), plus the pinned X_eTaL commit (XETAL_COMMIT).
"""
import datetime
import html
import json
import socket
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = "https://github.com/softwarewrighter/X_eTaL-demos"
XETAL = "https://github.com/softwarewrighter/X_eTaL"
STATUS = {"live": "Live", "draft": "In progress", "deferred": "Waiting on X_eTaL"}

PAGE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>X_eTaL Demos</title>
<link rel="icon" href="favicon.ico">
<style>
:root {{ --bg:#fbfaf7; --fg:#1d1d1f; --muted:#5f6368; --card:#ffffff; --line:#e3e0d8;
  --accent:#2457c5; --chip:#eef2fb; --live:#1f7a3a; --draft:#9a6200; --deferred:#8a8a8a; }}
@media (prefers-color-scheme: dark) {{ :root:not([data-theme="light"]) {{
  --bg:#141518; --fg:#e8e6e3; --muted:#a0a4ab; --card:#1d1f23; --line:#30333a;
  --accent:#8fb0ff; --chip:#262b36; --live:#5fcf7f; --draft:#e0a84a; --deferred:#8d9097; }} }}
:root[data-theme="dark"] {{ --bg:#141518; --fg:#e8e6e3; --muted:#a0a4ab; --card:#1d1f23;
  --line:#30333a; --accent:#8fb0ff; --chip:#262b36; --live:#5fcf7f; --draft:#e0a84a; --deferred:#8d9097; }}
* {{ box-sizing: border-box; }}
body {{ margin:0; background:var(--bg); color:var(--fg);
  font: 16px/1.5 system-ui, -apple-system, "Segoe UI", sans-serif; }}
main, footer {{ max-width: 980px; margin: 0 auto; padding: 0 16px; }}
header {{ padding: 48px 0 24px; }}
h1 {{ font-size: 2rem; margin: 0 0 8px; letter-spacing: -0.01em; }}
.lede {{ color: var(--muted); max-width: 46rem; margin: 0; }}
.lede a, footer a {{ color: var(--accent); }}
.grid {{ display:grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap:16px; padding: 8px 0 40px; }}
.card {{ background:var(--card); border:1px solid var(--line); border-radius:12px; padding:18px;
  display:flex; flex-direction:column; gap:10px; }}
.card h2 {{ font-size:1.15rem; margin:0; }}
.shot img {{ width:100%; aspect-ratio: 13 / 9; object-fit: cover; object-position: top; border-radius:8px; border:1px solid var(--line); display:block; }}
.card p {{ margin:0; color:var(--muted); }}
.status {{ font-size:.8rem; font-weight:600; }}
.status.live {{ color:var(--live); }} .status.draft {{ color:var(--draft); }} .status.deferred {{ color:var(--deferred); }}
.chips {{ display:flex; flex-wrap:wrap; gap:6px; }}
.chip {{ background:var(--chip); border-radius:999px; padding:2px 10px; font-size:.8rem; }}
.links {{ margin-top:auto; display:flex; gap:16px; font-weight:600; }}
.links a {{ color:var(--accent); text-decoration:none; }}
.links a:hover {{ text-decoration:underline; }}
.empty {{ color:var(--muted); padding: 24px 0 48px; }}
footer {{ border-top:1px solid var(--line); padding-top:16px; padding-bottom:32px; color:var(--muted); font-size:.85rem; }}
footer .sep {{ margin: 0 8px; }}
.brand {{ display:flex; align-items:center; gap:16px; margin-bottom: 8px; }}
.brand h1 {{ margin: 0; color: var(--accent); }}
.logo {{ height: 56px; width: auto; border-radius: 8px; }}
code {{ font-family: ui-monospace, "JuliaMono", Menlo, monospace; }}
.about {{ background:var(--card); border:1px solid var(--line); border-radius:12px; padding:14px 18px; margin: 0 0 16px; }}
.about p {{ margin: 0 0 8px; max-width: 52rem; }}
.about a {{ color: var(--accent); }}
.eco {{ display:flex; flex-wrap:wrap; gap:6px 18px; font-weight:600; font-size:.95rem; }}
.eco a {{ text-decoration:none; }} .eco a:hover {{ text-decoration:underline; }}
.eco .here {{ color: var(--muted); }}
.card p.idea {{ color: var(--fg); font-size: .95rem; }}
.xline {{ margin: 0; padding: 8px 10px; background: var(--chip); border-radius: 8px; overflow-x: auto;
  font-size: .9rem; line-height: 1.6; white-space: pre-wrap; }}
.xline code {{ font-family: "JuliaMono", "DejaVu Sans Mono", Menlo, ui-monospace, monospace; }}
.c-builtin {{ color: #1c5fd4; }} .c-userfunc {{ color: #2b8a3e; }} .c-lambdaarg {{ color: #a61e8f; }}
.c-number {{ color: #9c6500; }} .c-symbol {{ color: #0b7285; }} .c-comment {{ color: var(--muted); }}
@media (prefers-color-scheme: dark) {{ :root:not([data-theme="light"]) .c-builtin {{ color: #8fb0ff; }}
  :root:not([data-theme="light"]) .c-userfunc {{ color: #8ce99a; }} :root:not([data-theme="light"]) .c-lambdaarg {{ color: #f783ac; }}
  :root:not([data-theme="light"]) .c-number {{ color: #ffd43b; }} :root:not([data-theme="light"]) .c-symbol {{ color: #66d9e8; }} }}
</style>
</head>
<body>
<main>
<header>
<div class="brand"><img class="logo" src="modern-xetal-logo.jpg" alt="X_eTaL"><h1>Demos</h1></div>
<p class="lede">Small programs in <a href="{xetal}">X_eTaL</a>, a typed array language,
that make something worth watching. Each one shows its program beside the result, so you
can see a whole loop nest happen as one array expression. The machine-learning demos
(a 1.58-bit network, an MoE routing microscope, a tiny CNN, attention) are in
<a href="https://softwarewrighter.github.io/X_eTaL-ML/">X_eTaL-ML</a>.</p>
</header>
<section class="about" aria-label="About X_eTaL">
<p><b>X_eTaL</b> is an APL-family array language designed today: whole-array programming
and terse composition, with inferred static types and typed functional composition from
Haskell, explicit, checked interfaces in the spirit of Rust, and plain ASCII source drawn
as readable typography.</p>
<p><b>Extensible</b> three ways: libraries extend the vocabulary (ready), macros extend
what the language can say (new), native extensions extend the machine (through a
bridge today).</p>
<nav class="eco" aria-label="The X_eTaL repositories">
<a href="https://softwarewrighter.github.io/X_eTaL/">The language (playground)</a>
<span class="here">Demos (here)</span>
<a href="https://softwarewrighter.github.io/X_eTaL-ML/">ML</a>
<a href="https://softwarewrighter.github.io/X_eTaL-games/">Games</a>
<a href="https://softwarewrighter.github.io/X_eTaL-libraries/">Libraries</a>
<a href="https://softwarewrighter.github.io/X_eTaL-extensions/">Extensions</a>
<a href="https://github.com/softwarewrighter/X_eTaL-demos#start-here">Start here</a>
</nav>
</section>
{body}
</main>
<footer>
<span>Copyright (c) 2026 Michael A Wright</span><span class="sep">&middot;</span>
<span>MIT License</span><span class="sep">&middot;</span>
<a href="{repo}" target="_blank">Repository</a><span class="sep">&middot;</span>
<span>X_eTaL <a href="{xetal}/commit/{xsha}" target="_blank">{xshort}</a></span><span class="sep">&middot;</span>
<span>build (host {host}, sha {commit}, {stamp})</span>
</footer>
</body>
</html>
"""


def rendered(line):
    """The line decorated as X_eTaL draws it (HTML spans), by the bundled CLI."""
    try:
        xetal = subprocess.run([str(ROOT / "scripts" / "xetal.sh")], capture_output=True, text=True, check=True).stdout.strip()
        out = subprocess.run([xetal, "render", "--html", "-e", line], capture_output=True, text=True, check=True).stdout
        return out.strip()
    except (subprocess.CalledProcessError, OSError):
        return html.escape(line)


def card(m):
    slug = html.escape(m["slug"])
    links = []
    if m.get("web"):
        links.append(f'<a href="{slug}/">Open the demo</a>')
    links.append(f'<a href="{REPO}/tree/main/demos/{slug}#readme">How it works</a>')
    chips = "".join(f'<span class="chip">{html.escape(c)}</span>' for c in m["concepts"])
    st = m["status"]
    pic = ""
    if m.get("web") and m.get("picture"):
        alt = html.escape(m["title"])
        pic = f'<a class="shot" href="{slug}/"><img src="{slug}/screenshot.png" alt="{alt}" loading="lazy"></a>\n'
    elif m.get("tape"):
        # A command-line demo: its terminal recording (just tape SLUG).
        alt = html.escape(m["title"] + ": a terminal recording")
        pic = f'<a class="shot" href="{REPO}/tree/main/demos/{slug}#readme"><img src="{slug}/demo.gif" alt="{alt}" loading="lazy"></a>\n'
    return (f'<article class="card" id="{slug}">\n{pic}'
            f'<span class="status {st}">{STATUS[st]}</span>\n'
            f'<h2>{html.escape(m["title"])}</h2>\n'
            f'<p>{html.escape(m["summary"])}</p>\n'
            + (f'<p class="idea"><b>Why arrays:</b> {html.escape(m["idea"])}</p>\n' if m.get("idea") else "")
            + (f'<pre class="xline"><code>{rendered(m["line"])}</code></pre>\n' if m.get("line") else "")
            + f'<div class="chips">{chips}</div>\n'
            f'<div class="links">{" ".join(links)}</div>\n</article>')


def git(*args):
    r = subprocess.run(["git", "-C", str(ROOT), *args], capture_output=True, text=True)
    return r.stdout.strip() or "unknown"


def main():
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "pages" / "index.html"
    demos = json.loads(subprocess.run([str(ROOT / "scripts" / "demos.py"), "json"],
                                      capture_output=True, text=True, check=True).stdout)
    xsha = (ROOT / "XETAL_COMMIT").read_text().strip()
    if demos:
        body = '<section class="grid">\n' + "\n".join(card(m) for m in demos) + "\n</section>"
    else:
        body = '<p class="empty">The first demo is on its way.</p>'
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(PAGE.format(
        body=body, repo=REPO, xetal=XETAL, commit=git("rev-parse", "--short", "HEAD"),
        xsha=xsha, xshort=xsha[:7], host=socket.gethostname().split(".")[0],
        stamp=datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S")))
    print(f"catalog: {out} ({len(demos)} demo(s))")


if __name__ == "__main__":
    main()
