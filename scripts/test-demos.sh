#!/usr/bin/env bash
# Test the demos (every demos/<slug>/ but the template), or the named ones:
#   - each top-level <slug>/*.xtl runs with the vendored xetal (--seed 1,
#     pictures to work/draw/<slug>/) and its stdout must equal
#     expected/<name>.out and its stderr expected/<name>.err (empty when
#     there is no .err file);
#   - web/ (a Cargo workspace), when present: cargo test;
#   - test.sh, when present and executable: run it.
# XETAL_BLESS=1 rewrites the expected files instead (review the diff!).
# XETAL_DEMOS_DIR overrides demos/ (scripts/selftest-demos.sh uses it).
#   scripts/test-demos.sh [SLUG...]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
"$root/scripts/demos.py" check
xetal="$("$root/scripts/build-xetal.sh")"
if [ $# -gt 0 ]; then slugs=("$@"); else
  slugs=(); while IFS= read -r s; do [ -n "$s" ] && slugs+=("$s"); done < <("$root/scripts/demos.py" list)
fi
# same EXPECTED_STEM DIR: DIR/out and DIR/err match the expected files;
# the differences go to DIR/diff.
same() {
  local ok=0
  diff -u "$1.out" "$2/out" > "$2/diff" || ok=1
  if [ -f "$1.err" ]; then
    diff -u "$1.err" "$2/err" >> "$2/diff" || ok=1
  elif [ -s "$2/err" ]; then
    { echo "unexpected stderr:"; cat "$2/err"; } >> "$2/diff"; ok=1
  fi
  return $ok
}
fail=0; n=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
for slug in ${slugs[@]+"${slugs[@]}"}; do
  d="${XETAL_DEMOS_DIR:-$root/demos}/$slug"
  [ -f "$d/demo.toml" ] || { echo "test: no demo $slug" >&2; exit 1; }
  n=$((n + 1))
  for prog in "$d"/*.xtl; do
    [ -e "$prog" ] || continue
    name="$(basename "$prog" .xtl)"
    exp="$d/expected/$name"
    mkdir -p "$root/work/draw/$slug"
    (cd "$d" && "$xetal" run --seed 1 --draw "$root/work/draw/$slug" "$prog" \
      >"$tmp/out" 2>"$tmp/err") || true
    if [ "${XETAL_BLESS:-}" = 1 ]; then
      mkdir -p "$d/expected"; cp "$tmp/out" "$exp.out"
      if [ -s "$tmp/err" ]; then cp "$tmp/err" "$exp.err"; else rm -f "$exp.err"; fi
      echo "blessed: $slug/$name"; continue
    fi
    if [ ! -f "$exp.out" ]; then
      echo "FAIL: $slug/$name: no expected/$name.out (XETAL_BLESS=1 to create)"; fail=1
    elif same "$exp" "$tmp"; then
      echo "ok: $slug/$name"
    else
      echo "FAIL: $slug/$name"; cat "$tmp/diff"; fail=1
    fi
  done
  if [ -f "$d/web/Cargo.toml" ]; then
    (cd "$d/web" && cargo test -q --workspace >/dev/null 2>&1) \
      && echo "ok: $slug/web" || { echo "FAIL: $slug/web"; (cd "$d/web" && cargo test -q --workspace) || true; fail=1; }
  fi
  if [ -x "$d/test.sh" ]; then
    "$d/test.sh" && echo "ok: $slug/test.sh" || { echo "FAIL: $slug/test.sh"; fail=1; }
  fi
done
echo "test-demos: $n demo(s)$([ $fail = 0 ] && echo ', all passed' || echo ', FAILURES')"
exit $fail
