# X_eTaL-demos tasks. Recipes call scripts/*.sh, which hold the logic
# and work without just too. `just` alone lists the recipes.

set positional-arguments

# List the recipes
default:
    @just --list

# Snapshot a committed ref of ../X_eTaL into vendor/xetal/ (default HEAD); commit it on its own
vendor ref="HEAD":
    scripts/vendor-xetal.sh "$1"

# Build the vendored xetal CLI into target/xetal/
xetal:
    @scripts/build-xetal.sh

# The vendored X_eTaL: what was vendored (VENDORED) and the binary's version
xetal-version:
    @cat vendor/xetal/VENDORED
    @"$(scripts/build-xetal.sh)" --version | head -1

# Evaluate an expression with the vendored xetal: just eval "'+ r_/ 1 2 3"
eval expr:
    @"$(scripts/build-xetal.sh)" eval -e "$1"

# Check the vendored X_eTaL: CLI builds and answers; xetal-play usable natively and for wasm32
check-vendor:
    scripts/check-vendor.sh

# The demos, in catalog order
demos:
    @scripts/demos.py list

# Start a demo sub-project from demos/_template: just new-demo wave-tank "Wave tank"
new-demo slug title:
    scripts/new-demo.sh "$1" "$2"

# Run a demo's program (default SLUG.xtl) with the vendored xetal: just run life-microscope
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
bench:
    cargo run --release -q --manifest-path tools/bench/Cargo.toml

# Compare the timings with docs/bench.json; fails if a case is more than 15% slower (run on every vendor refresh)
bench-check:
    cargo run --release -q --manifest-path tools/bench/Cargo.toml -- check

# Build the live site into pages/ (committed; the Pages workflow publishes it)
pages:
    scripts/build-pages.sh

# Serve the built pages/ as GitHub Pages will: http://127.0.0.1:8096/X_eTaL-demos/
serve-pages port="8096":
    scripts/serve-pages.sh "$1"

# Serve one demo's web app locally, rebuilt on change: just serve life-microscope
serve slug port="8095":
    cd demos/{{slug}}/web && trunk serve --release --port {{port}} --address 127.0.0.1

# Screenshot every demo (from the built pages/) into demos/<slug>/screenshot.png
screenshots *slugs:
    scripts/screenshots.sh "$@"

# The full pre-commit gate: vendored X_eTaL, demo tooling, demo tests, ASCII-only markdown
gate:
    scripts/gate.sh

# Show the agentrail saga state and the current step
status:
    agentrail status

# Open the saga plan
plan:
    @cat docs/plan.md
