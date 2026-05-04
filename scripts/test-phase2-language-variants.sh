#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"

set_if_exists() {
  key="$1"
  path="$2"
  eval "current=\${$key:-}"
  if [ -n "$current" ]; then
    return
  fi
  if [ -f "$ROOT/$path" ]; then
    eval "$key=\$ROOT/$path"
    export "$key"
  fi
}

set_if_exists "LAYER36_GO_CLOCK_WASM" "test/integration/language-variants/layer36_go_clock.wasm"
set_if_exists "LAYER36_GO_CAT_WASM" "test/integration/language-variants/layer36_go_cat.wasm"
set_if_exists "LAYER36_GO_CURL_WASM" "test/integration/language-variants/layer36_go_curl.wasm"
set_if_exists "LAYER36_TS_CLOCK_WASM" "test/integration/language-variants/layer36_ts_clock.wasm"
set_if_exists "LAYER36_TS_CAT_WASM" "test/integration/language-variants/layer36_ts_cat.wasm"
set_if_exists "LAYER36_TS_CURL_WASM" "test/integration/language-variants/layer36_ts_curl.wasm"

has_variant=""
for key in \
  LAYER36_GO_CLOCK_WASM \
  LAYER36_GO_CAT_WASM \
  LAYER36_GO_CURL_WASM \
  LAYER36_TS_CLOCK_WASM \
  LAYER36_TS_CAT_WASM \
  LAYER36_TS_CURL_WASM
do
  eval "value=\${$key:-}"
  if [ -n "$value" ]; then
    has_variant="yes"
    break
  fi
done

if [ -z "$has_variant" ]; then
  echo "Skipping Phase 2 language-variant runtime tests (no LAYER36_GO_* or LAYER36_TS_* vars set, and no test/integration/language-variants/*.wasm fixtures found)."
  exit 0
fi

echo "Running Phase 2 language-variant runtime tests"
cd "$ROOT"

cargo test -p layer36-cli --test cli configured_layer36_go_
cargo test -p layer36-cli --test cli configured_layer36_ts_
