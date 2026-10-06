#!/usr/bin/env bash
# Every X_eTaL stage against the Unix tool it imitates, on the same
# inputs (byte order: LC_ALL=C), and the two pipelines of the README.
# wc's columns are compared as BSD wc prints them (this is macOS).
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
export PATH="$PWD/bin:$PATH" LC_ALL=C
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
printf '' > "$tmp/empty.txt"
printf 'one line\n' > "$tmp/one.txt"
printf 'b\n\na\n\n\nb\nb\nc d  e\n a\n' > "$tmp/blanks.txt"
printf 'last line has no newline\nsecond\nlast' > "$tmp/nonl.txt"
seq 1 300 | awk '{ print ($1 * 7919) % 101 " w" ($1 % 13) }' > "$tmp/many.txt"
inputs=(sample.txt "$tmp/empty.txt" "$tmp/one.txt" "$tmp/blanks.txt" "$tmp/nonl.txt" "$tmp/many.txt")
fail=0
same() { # name, our command, their command (input on standard input)
  local name="$1" ours="$2" theirs="$3" f
  for f in "${inputs[@]}"; do
    local rc=0
    awk 1 "$f" | eval "$ours" > "$tmp/ours" 2> "$tmp/err" || rc=$?
    [ "$rc" = 0 ] || { echo "FAIL: $name on $(basename "$f"): exit $rc"; fail=1; }
    awk 1 "$f" | eval "$theirs" > "$tmp/theirs"
    [ ! -s "$tmp/err" ] || { echo "FAIL: $name on $(basename "$f"): $(head -1 "$tmp/err")"; fail=1; }
    diff "$tmp/ours" "$tmp/theirs" >/dev/null || { echo "FAIL: $name on $(basename "$f")"; diff "$tmp/ours" "$tmp/theirs" | head -5; fail=1; }
  done
}
same cat "xetalcat" "cat"
same wc "xetalwc" "wc"
same "wc -l" "xetalwc -l" "wc -l"
same "wc -w -c" "xetalwc -w -c" "wc -w -c"
same "grep the" "xetalgrep the" "grep -F the || true"
same "grep w1" "xetalgrep w1" "grep -F w1 || true"
same "grep 'a b'" "xetalgrep 'c d'" "grep -F 'c d' || true"
same uniq "xetaluniq" "uniq"
same sort "xetalsort" "sort"
same head "xetalhead" "head"
same "head -n 3" "xetalhead -n 3" "head -n 3"
same tail "xetaltail" "tail"
same "tail -n 2" "xetaltail -n 2" "tail -n 2"
same "sort | uniq | wc -l" "xetalsort | xetaluniq | xetalwc -l" "sort | uniq | wc -l"
# cat with a file: X_eTaL reads the file itself (no trailing newline added).
for f in "${inputs[@]}"; do
  diff <(xetalcat "$f") "$f" >/dev/null || { echo "FAIL: cat FILE on $(basename "$f")"; fail=1; }
done
# The README's pipelines.
diff <(xetalcat sample.txt | xetaluniq | xetalwc -l) <(cat sample.txt | uniq | wc -l) >/dev/null \
  || { echo "FAIL: cat | uniq | wc -l"; fail=1; }
diff <(xetalcat sample.txt | xetalgrep the | xetalsort | xetalhead -n 3) <(cat sample.txt | grep -F the | sort | head -n 3) >/dev/null \
  || { echo "FAIL: cat | grep | sort | head"; fail=1; }
exit "$fail"
