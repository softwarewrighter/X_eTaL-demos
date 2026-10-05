#!/usr/bin/env bash
# Check the published site: every demo page with a web app, loaded from
# https://softwarewrighter.github.io/X_eTaL-demos/ in headless Chrome,
# must run X_eTaL and show its browser.txt text (scripts/browser-check.sh
# with XETAL_PAGES_URL). Run after just publish (GitHub Pages takes a
# minute or so to serve a new gh-pages commit).
#   scripts/check-live.sh [SLUG...]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export XETAL_PAGES_URL="${XETAL_PAGES_URL:-https://softwarewrighter.github.io/X_eTaL-demos}"
curl -sf -o /dev/null "$XETAL_PAGES_URL/" || { echo "check-live: $XETAL_PAGES_URL/ does not load" >&2; exit 1; }
slugs=("$@")
if [ ${#slugs[@]} -eq 0 ]; then
  for f in "$root"/demos/*/web/browser.txt; do slugs+=("$(basename "$(dirname "$(dirname "$f")")")"); done
fi
fail=0
for s in "${slugs[@]}"; do
  if out="$("$root/scripts/browser-check.sh" "$s" 2>&1)"; then echo "ok: live $s"
  else echo "FAIL: live $s"; echo "$out" | grep -v "^found:"; fail=1; fi
done
[ "$fail" = 0 ] && echo "check-live: ok (${#slugs[@]} pages at $XETAL_PAGES_URL)"
exit "$fail"
