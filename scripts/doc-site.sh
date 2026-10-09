#!/usr/bin/env bash
# Build the cross-reference site (xetal doc --out) into pages/doc, as
# X_eTaL's own scripts/doc-site.sh does for its repository: every
# name typed, documented from its ## comments, linked to its
# definition and its uses. Run by scripts/build-pages.sh (just pages).
#
# Documented: the two library files (a macro library's helper is
# h:name, a library's export l:name) and each demo's own main
# program, demos/<slug>/<slug.xtl> (its top-level names, u: or bare).
# Left out: demos/xetal-pipes/stages/{cat,wc,grep,...}.xtl, each of
# which reads an `args` variable the wrapper (bin/xetal-stage)
# supplies at run time, so none type-checks alone; and
# demos/stencil-macros/check.xtl, a correctness check, not a demo.
#   scripts/doc-site.sh             # into pages/doc
#   XETAL_DOC_OUT=DIR scripts/doc-site.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
xetal="${XETAL:-$root/bin/xetal}"
export XETAL_PATH="$("$root/scripts/libraries.sh")"  # the pinned X_eTaL-libraries
out="${XETAL_DOC_OUT:-$root/pages/doc}"
rm -rf "$out"
files=(demos/stencil-macros/Stencil.xtlm demos/xetal-pipes/stages/Pipes.xtl)
while IFS= read -r slug; do
  [ -n "$slug" ] || continue
  f="demos/$slug/$slug.xtl"
  [ -f "$f" ] && files+=("$f")
done < <("$root/scripts/demos.py" list)
"$xetal" doc --out "$out" "${files[@]}" > /dev/null
echo "doc: $(find "$out" -name '*.html' | wc -l | tr -d ' ') pages in $out"
