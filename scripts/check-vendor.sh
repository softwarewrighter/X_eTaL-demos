#!/usr/bin/env bash
# Check the vendored X_eTaL: the CLI builds and answers, and a workspace
# outside vendor/xetal can use xetal-play natively and for wasm32.
#   scripts/check-vendor.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
xetal="$("$root/scripts/build-xetal.sh")"
got="$("$xetal" eval -e "'+ r_/_2 2 3 r_eshape r_ange 6")"
[ "$got" = "6 15" ] || { echo "check-vendor: eval gave '$got', expected '6 15'" >&2; exit 1; }
"$xetal" run "$root/vendor/xetal/demos/life.xtl" >/dev/null
cd "$root/tools/vendor-probe"
cargo test -q >/dev/null 2>&1 || { cargo test; exit 1; }
cargo check -q --target wasm32-unknown-unknown
echo "check-vendor: ok ($(sed -n 's/^commit = "\(.......\).*/\1/p' "$root/vendor/xetal/VENDORED"))"
