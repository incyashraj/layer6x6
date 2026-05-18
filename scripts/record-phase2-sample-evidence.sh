#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
cd "$ROOT"

OUTPUT="${1:-target/phase2-sample-evidence/sample-evidence.md}"

cargo build -p layer36-cli

absolute_path() {
  case "$1" in
    /*) printf '%s\n' "$1" ;;
    *) printf '%s/%s\n' "$ROOT" "$1" ;;
  esac
}

CLOCK_WASM="${LAYER36_CLOCK_WASM:-apps/layer36-clock/target/wasm32-wasip1/release/layer36_clock.wasm}"
CAT_WASM="${LAYER36_CAT_WASM:-apps/layer36-cat/target/wasm32-wasip1/release/layer36_cat.wasm}"
CURL_WASM="${LAYER36_CURL_WASM:-apps/layer36-curl/target/wasm32-wasip1/release/layer36_curl.wasm}"

if [ ! -f "$CLOCK_WASM" ]; then
  CLOCK_WASM="$(scripts/build-layer36-clock-component.sh | tail -n 1)"
fi
if [ ! -f "$CAT_WASM" ]; then
  CAT_WASM="$(scripts/build-layer36-cat-component.sh | tail -n 1)"
fi
if [ ! -f "$CURL_WASM" ]; then
  CURL_WASM="$(scripts/build-layer36-curl-component.sh | tail -n 1)"
fi

CLOCK_WASM="$(absolute_path "$CLOCK_WASM")"
CAT_WASM="$(absolute_path "$CAT_WASM")"
CURL_WASM="$(absolute_path "$CURL_WASM")"

cargo run -p layer36-tools --bin record-phase2-sample-evidence -- \
  --layer36 "$ROOT/target/debug/layer36" \
  --clock "$CLOCK_WASM" \
  --cat "$CAT_WASM" \
  --curl "$CURL_WASM" \
  --output "$OUTPUT"
