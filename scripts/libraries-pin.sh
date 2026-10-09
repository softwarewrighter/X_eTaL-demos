#!/usr/bin/env bash
# Pin a newer X_eTaL-libraries: write the full SHA of a COMMITTED ref of
# ../X_eTaL-libraries into LIBRARIES_COMMIT, then fetch it. Commit
# LIBRARIES_COMMIT on its own, after the goldens pass.
#   scripts/libraries-pin.sh            # HEAD of ../X_eTaL-libraries
#   scripts/libraries-pin.sh REF
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
repo="${XETAL_LIBRARIES_REPO:-$root/../X_eTaL-libraries}"
sha="$(git -C "$repo" rev-parse --verify "${1:-HEAD}^{commit}")"
echo "$sha" > "$root/LIBRARIES_COMMIT"
clone="$root/work/libraries"
if [ -d "$clone/.git" ] && ! git -C "$clone" cat-file -e "$sha^{commit}" 2>/dev/null; then
    git -C "$clone" fetch --quiet "$repo" "$sha" 2>/dev/null || true
fi
XETAL_LIBRARIES_SOURCE="${XETAL_LIBRARIES_SOURCE:-$repo}" "$root/scripts/libraries.sh" >/dev/null
echo "pinned X_eTaL-libraries ${sha:0:7}: $(git -C "$repo" log -1 --format=%s "$sha")"
echo "libraries-pin: now run the goldens (just test)" >&2
