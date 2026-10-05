#!/usr/bin/env bash
# Further tests: the program after macro expansion (xetal expand)
# against expected/stencil-macros.expanded.xtl (XETAL_BLESS=1 rewrites
# it), and the examples in Stencil.xtlm's docs (xetal doc --test).
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
xetal="${XETAL:-../../bin/xetal}"
golden=expected/stencil-macros.expanded.xtl
if [ "${XETAL_BLESS:-}" = 1 ]; then
  "$xetal" expand stencil-macros.xtl > "$golden"
fi
diff -u "$golden" <("$xetal" expand stencil-macros.xtl)
"$xetal" doc --test Stencil.xtlm >/dev/null
