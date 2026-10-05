# X_eTaL-demos tasks. Recipes call scripts/*.sh, which hold the logic
# and work without just too. `just` alone lists the recipes.

set positional-arguments

# List the recipes
default:
    @just --list

# Get and build xetal at the known-good commit in XETAL_COMMIT (clone in work/xetal, binary bin/xetal)
xetal:
    @scripts/xetal.sh

# Pin a committed ref of ../X_eTaL (default HEAD) in XETAL_COMMIT and build it; commit it on its own
xetal-pin ref="HEAD":
    scripts/xetal-pin.sh "$1"

# The pinned X_eTaL: XETAL_COMMIT and the binary's version
xetal-version:
    @cat XETAL_COMMIT
    @"$(scripts/xetal.sh)" --version

# Evaluate an expression with the pinned xetal: just eval "'+ r_/ 1 2 3"
eval expr:
    @"$(scripts/xetal.sh)" eval -e "$1"

# Check the pinned X_eTaL: CLI builds, answers, reports its commit; xetal-play usable natively and for wasm32
check-xetal:
    scripts/check-xetal.sh

# The demos, in catalog order
demos:
    @scripts/demos.py list

# Start a demo sub-project from demos/_template: just new-demo wave-tank "Wave tank"
new-demo slug title:
    scripts/new-demo.sh "$1" "$2"

# Run a demo's program (default SLUG.xtl) with the pinned xetal: just run life-microscope
run slug file="":
    @scripts/run-demo.sh "$1" ${2:+"$2"}

# Run a demo's program as a notebook: each statement drawn, then its output
show slug file="":
    @scripts/run-demo.sh --echo "$1" ${2:+"$2"}

# Test every demo: reg-rs baselines (CLI runs, the built page in headless Chrome), web/ tests (XETAL_BROWSER=0 skips the browser)
test:
    scripts/test-demos.sh

# Test one demo: just test-demo life-microscope
test-demo slug:
    scripts/test-demos.sh "$1"

# Accept one demo's current output as its reg-rs baselines, creating missing ones (review the diff!)
bless slug:
    XETAL_BLESS=1 scripts/test-demos.sh "$1"

# Load one demo's built page in headless Chrome and check it shows X_eTaL's results
browser-check slug:
    scripts/browser-check.sh "$1"

# Time the pages' X_eTaL programs and the showcase built-ins; write docs/bench.md and docs/bench.json (the new baseline)
bench: xetal
    cargo run --release -q --manifest-path tools/bench/Cargo.toml

# Compare the timings with docs/bench.json; fails if a case is more than 15% slower (run on every X_eTaL pin)
bench-check: xetal
    cargo run --release -q --manifest-path tools/bench/Cargo.toml -- check

# Build the live site into pages/ (not tracked; the gate builds it too)
pages:
    scripts/build-pages.sh

# Publish the site: build pages/ from this commit and make it the gh-pages branch's only commit (needs a clean work tree)
publish:
    scripts/publish-pages.sh

# Check the published site in headless Chrome: every page runs X_eTaL (after just publish)
check-live *slugs:
    scripts/check-live.sh "$@"

# Serve the built pages/ as GitHub Pages will: http://127.0.0.1:8413/X_eTaL-demos/ (8413 is this repo's port)
serve-pages port="8413":
    scripts/serve-pages.sh "$1"

# Serve one demo's web app locally, rebuilt on change: just serve life-microscope
serve slug port="8413": xetal
    cd demos/{{slug}}/web && trunk serve --release --port {{port}} --address 127.0.0.1

# Screenshot every demo (from the built pages/) into demos/<slug>/screenshot.png
screenshots *slugs:
    scripts/screenshots.sh "$@"

# The full pre-commit gate: pinned X_eTaL, demo tooling, demo tests, ASCII-only markdown
gate:
    scripts/gate.sh

# Show the agentrail saga state and the current step
status:
    agentrail status

# Open the saga plan
plan:
    @cat docs/plan.md
