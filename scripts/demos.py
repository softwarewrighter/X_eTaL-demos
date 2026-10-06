#!/usr/bin/env python3
"""The demos' descriptions (demos/<slug>/demo.toml).

  scripts/demos.py list     # the slugs, in catalog order
  scripts/demos.py check    # validate every demo.toml (exit 1 on a problem)
  scripts/demos.py json     # every description as JSON, in catalog order

Directories starting with "_" (the template) are not demos.
XETAL_DEMOS_DIR overrides demos/ (the runner's self-test uses it).
"""
import json
import os
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STATUSES = {"draft", "live", "deferred"}
FIELDS = {"slug": str, "title": str, "summary": str, "concepts": list,
          "status": str, "order": int, "needs": list}


def demo_dirs():
    base = Path(os.environ.get("XETAL_DEMOS_DIR", ROOT / "demos"))
    return sorted(d for d in base.iterdir()
                  if d.is_dir() and not d.name.startswith(("_", ".")))


def problems(d, meta):
    out = [f"{d.name}: missing or wrong type: {k}" for k, t in FIELDS.items()
           if not isinstance(meta.get(k), t)]
    if meta.get("slug") != d.name:
        out.append(f"{d.name}: slug {meta.get('slug')!r} is not the directory name")
    if meta.get("status") not in STATUSES:
        out.append(f"{d.name}: status must be one of {sorted(STATUSES)}")
    if meta.get("status") == "live" and not (meta.get("wiki") or meta.get("story")):
        out.append(f"{d.name}: a live demo needs wiki (a Wikipedia article) or story (for its title's dialog)")
    wiki = meta.get("wiki", "")
    if wiki and not str(wiki).startswith("https://en.wikipedia.org/wiki/"):
        out.append(f"{d.name}: wiki must be an https://en.wikipedia.org/wiki/ link")
    for k in ("idea", "line", "wiki", "story"):
        if k in meta and not isinstance(meta[k], str):
            out.append(f"{d.name}: {k} must be text")
    line = meta.get("line", "")
    if isinstance(line, str) and line.strip():
        sources = "".join(f.read_text() for f in d.glob("*.xtl"))
        if line.strip() not in sources:
            out.append(f"{d.name}: line is not in the demo's .xtl: {line.strip()[:40]}")
    if not (d / "README.md").is_file():
        out.append(f"{d.name}: no README.md")
    return out


def load():
    demos, errs = [], []
    for d in demo_dirs():
        f = d / "demo.toml"
        if not f.is_file():
            errs.append(f"{d.name}: no demo.toml")
            continue
        try:
            meta = tomllib.loads(f.read_text())
        except tomllib.TOMLDecodeError as e:
            errs.append(f"{d.name}: demo.toml: {e}")
            continue
        errs += problems(d, meta)
        meta["web"] = (d / "web" / "Cargo.toml").is_file()
        meta["picture"] = (d / "screenshot.png").is_file()
        meta["tape"] = (d / "demo.gif").is_file()
        demos.append(meta)
    demos.sort(key=lambda m: (m.get("order", 0), m.get("slug", "")))
    return demos, errs


def main(cmd):
    demos, errs = load()
    if errs:
        print("\n".join(errs), file=sys.stderr)
        return 1
    if cmd == "list":
        print("\n".join(m["slug"] for m in demos))
    elif cmd == "json":
        print(json.dumps(demos, indent=2))
    elif cmd != "check":
        print(__doc__, file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "check"))
