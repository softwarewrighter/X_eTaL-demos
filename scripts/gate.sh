#!/usr/bin/env bash
# The pre-commit gate: the pinned X_eTaL (scripts/check-xetal.sh),
# the demo tooling (scripts/selftest-demos.sh), the shared page shell,
# every demo's tests, the benchmark tool's build, then ASCII-only
# markdown for the docs we own.
#   scripts/gate.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
"$root/scripts/check-xetal.sh"
"$root/scripts/selftest-demos.sh"
# The shared shell of the demo pages (shared/microscope).
(cd "$root/shared/microscope" && cargo test -q >/dev/null 2>&1 && cargo check -q --target wasm32-unknown-unknown) \
  || { (cd "$root/shared/microscope" && cargo test -q); echo "FAIL: shared/microscope"; exit 1; }
echo "ok: shared/microscope"
"$root/scripts/test-demos.sh"
# The benchmark tool builds (running it, just bench-check, takes about
# 40 s and is noisy on a shared machine: it is run on every X_eTaL pin).
(cd "$root/tools/bench" && cargo check -q --release) || { echo "FAIL: tools/bench"; exit 1; }
echo "ok: tools/bench (builds)"
# American spellings only (and the checker checks itself first).
"$root/scripts/check-spelling.py" --self-test
"$root/scripts/check-spelling.py"
md=(README.md docs/plan.md docs/xetal-asks.md docs/xetal-ml-asks.md docs/bench.md shared/microscope/README.md)
for f in demos/*/README.md; do [ -e "$f" ] && md+=("$f"); done
for f in "${md[@]}"; do sw-markdown-checker -f "$f" >/dev/null || { sw-markdown-checker -f "$f"; exit 1; }; done
echo "gate: ok"
