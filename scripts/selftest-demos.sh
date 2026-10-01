#!/usr/bin/env bash
# Test the demo tooling itself in a scratch demos directory: new-demo
# makes a demo that passes; a wrong expected output fails; an unexpected
# stderr fails; XETAL_BLESS=1 repairs it; a bad demo.toml is rejected.
#   scripts/selftest-demos.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
XETAL_DEMOS_DIR="$(mktemp -d)"; export XETAL_DEMOS_DIR
trap 'rm -rf "$XETAL_DEMOS_DIR"' EXIT
t="$root/scripts/test-demos.sh"
expect() { # expect pass|fail DESCRIPTION
  if "$t" probe >/dev/null 2>&1; then got=pass; else got=fail; fi
  [ "$got" = "$1" ] || { echo "selftest: $2: expected $1, got $got" >&2; "$t" probe || true; exit 1; }
}
"$root/scripts/new-demo.sh" probe "Probe & Co" >/dev/null
grep -q 'title = "Probe & Co"' "$XETAL_DEMOS_DIR/probe/demo.toml"
expect pass "a fresh demo"
echo 56 > "$XETAL_DEMOS_DIR/probe/expected/probe.out"
expect fail "a wrong expected output"
XETAL_BLESS=1 "$t" probe >/dev/null
expect pass "after blessing"
printf '#!/usr/bin/env xetal\n1 +\n' > "$XETAL_DEMOS_DIR/probe/probe.xtl"
printf '' > "$XETAL_DEMOS_DIR/probe/expected/probe.out"
expect fail "an unexpected error"
XETAL_BLESS=1 "$t" probe >/dev/null
[ -s "$XETAL_DEMOS_DIR/probe/expected/probe.err" ]
expect pass "an expected error"
sed -i '' 's/^status = .*/status = "bogus"/' "$XETAL_DEMOS_DIR/probe/demo.toml"
expect fail "a bad status"
echo "selftest-demos: ok"
