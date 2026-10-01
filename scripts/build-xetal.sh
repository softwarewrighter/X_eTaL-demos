#!/usr/bin/env bash
# Build the vendored xetal CLI (optimized) into target/xetal/, and print
# its path. Cargo only rebuilds when a vendored source changed.
#   scripts/build-xetal.sh           # quiet; prints the binary's path
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
[ -f "$root/vendor/xetal/VENDORED" ] || { echo "no vendor/xetal: run just vendor" >&2; exit 1; }
export CARGO_TARGET_DIR="$root/target/xetal"
(cd "$root/vendor/xetal/components/cli" && cargo build -q --release -p xetal-cli >&2)
echo "$CARGO_TARGET_DIR/release/xetal"
