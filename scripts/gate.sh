#!/usr/bin/env bash
# The pre-commit gate: every demo's tests, then ASCII-only markdown for
# the docs we own. Later steps add the vendored build and the goldens.
#   scripts/gate.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
"$root/scripts/test-demos.sh"
md=(README.md docs/plan.md docs/xetal-asks.md)
for f in demos/*/README.md; do [ -e "$f" ] && md+=("$f"); done
for f in "${md[@]}"; do sw-markdown-checker -f "$f" >/dev/null || { sw-markdown-checker -f "$f"; exit 1; }; done
echo "gate: ok"
