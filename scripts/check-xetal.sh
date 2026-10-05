#!/usr/bin/env bash
# Check the pinned X_eTaL: the CLI builds, answers and reports the
# commit in XETAL_COMMIT, and a workspace outside work/xetal can use
# xetal-play natively and for wasm32.
#   scripts/check-xetal.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
xetal="$("$root/scripts/xetal.sh")"
short="$(cut -c1-7 "$root/XETAL_COMMIT")"
got="$("$xetal" eval -e "'+ r_/_2 2 3 r_eshape r_ange 6")"
[ "$got" = "6 15" ] || { echo "check-xetal: eval gave '$got', expected '6 15'" >&2; exit 1; }
"$xetal" --version | grep -q "$short" || { echo "check-xetal: xetal --version does not report $short" >&2; exit 1; }
"$xetal" run "$root/work/xetal/demos/life.xtl" >/dev/null
cd "$root/tools/xetal-probe"
cargo test -q >/dev/null 2>&1 || { cargo test; exit 1; }
cargo check -q --target wasm32-unknown-unknown
echo "check-xetal: ok ($short)"
