#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
cd "$ROOT"

OUTPUT="${1:-target/phase2-sample-evidence/sample-evidence.md}"

cargo build -p layer36-cli

CLOCK_WASM="$(scripts/build-layer36-clock-component.sh | tail -n 1)"
CAT_WASM="$(scripts/build-layer36-cat-component.sh | tail -n 1)"
CURL_WASM="$(scripts/build-layer36-curl-component.sh | tail -n 1)"

cargo run -p layer36-tools --bin record-phase2-sample-evidence -- \
  --layer36 "$ROOT/target/debug/layer36" \
  --clock "$CLOCK_WASM" \
  --cat "$CAT_WASM" \
  --curl "$CURL_WASM" \
  --output "$OUTPUT"
