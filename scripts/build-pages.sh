#!/usr/bin/env bash
# Build the live site into pages/, which is committed: the Pages
# workflow publishes that folder as it is (nothing is built on GitHub).
#   - every demo with a web app (demos/<slug>/web/) is built with trunk
#     into pages/<slug>/, served under /X_eTaL-demos/<slug>/;
#   - pages/index.html, the catalog, from every demo.toml.
# A demo removed from demos/ loses its pages/<slug>/.
#   scripts/build-pages.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
base="/X_eTaL-demos"
"$root/scripts/demos.py" check
mkdir -p "$root/pages"
touch "$root/pages/.nojekyll"
keep=()
while IFS= read -r slug; do
  [ -n "$slug" ] || continue
  web="$root/demos/$slug/web"
  [ -f "$web/Cargo.toml" ] || continue
  keep+=("$slug")
  dist="$root/target/pages-dist/$slug"
  echo "==> trunk build $slug"
  # The app's own index.html is the trunk entry: demos/<slug>/web/index.html.
  (cd "$web" && trunk build --release --public-url "$base/$slug/" --dist "$dist")
  rsync -a --delete "$dist/" "$root/pages/$slug/"
done < <("$root/scripts/demos.py" list)
# Drop pages/<slug>/ of demos that no longer have a web app.
for d in "$root"/pages/*/; do
  [ -d "$d" ] || continue
  s="$(basename "$d")"
  printf '%s\n' ${keep[@]+"${keep[@]}"} | grep -qx "$s" || { echo "removing pages/$s/"; rm -rf "$d"; }
done
"$root/scripts/build-catalog.py"
echo "pages/ built; commit it (git add pages/) and push to publish."
