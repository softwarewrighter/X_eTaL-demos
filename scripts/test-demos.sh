#!/usr/bin/env bash
# Run every demo sub-project's tests (demos/<slug>/). A demo with no
# tests yet is listed and skipped; no demos at all passes.
#   scripts/test-demos.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
n=0
for d in "$root"/demos/*/; do
  [ -f "$d/demo.toml" ] || continue
  n=$((n + 1))
  slug="$(basename "$d")"
  if [ -x "$d/test.sh" ]; then
    echo "test: $slug"
    "$d/test.sh"
  else
    echo "test: $slug (no tests yet)"
  fi
done
echo "test-demos: $n demo(s)"
