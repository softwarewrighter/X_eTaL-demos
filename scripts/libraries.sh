#!/usr/bin/env bash
# Get X_eTaL-libraries at the known-good commit in LIBRARIES_COMMIT (as
# scripts/xetal.sh gets X_eTaL): clone it into work/libraries
# (gitignored), check the commit out, and print XETAL_PATH for it,
# every libs/<Name>/src. A demo imports a library with u_se< and runs
# with that path (scripts/run-demo.sh, test-demos.sh, doc-site.sh); a
# page reads the library's text from work/libraries at build time.
# Safe to run at any time: with nothing to do it only prints the path.
#   scripts/libraries.sh
#   XETAL_LIBRARIES_SOURCE=../X_eTaL-libraries scripts/libraries.sh   # first clone from a local checkout
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
commit="$(tr -d '[:space:]' < "$root/LIBRARIES_COMMIT")"
source="${XETAL_LIBRARIES_SOURCE:-https://github.com/softwarewrighter/X_eTaL-libraries.git}"
clone="$root/work/libraries"

if [ ! -d "$clone/.git" ]; then
    mkdir -p "$root/work"
    echo "libraries: cloning $source into work/libraries" >&2
    git clone --quiet "$source" "$clone"
fi
if ! git -C "$clone" cat-file -e "$commit^{commit}" 2>/dev/null; then
    git -C "$clone" fetch --quiet origin
fi
if [ "$(git -C "$clone" rev-parse HEAD)" != "$commit" ]; then
    git -C "$clone" checkout --quiet --detach "$commit"
fi
# work/libraries is a pristine checkout of the known-good commit: never edit it.
[ -z "$(git -C "$clone" status --porcelain --untracked-files=no)" ] || { echo "libraries: work/libraries has local changes; it must match LIBRARIES_COMMIT" >&2; exit 1; }
ls -d "$clone"/libs/*/src | paste -sd: -
