#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"

if [ -n "${LAYER36_HELLO_WASM:-}" ]; then
  HELLO_WASM="$LAYER36_HELLO_WASM"
else
  HELLO_WASM="$("$ROOT/scripts/build-hello-component.sh")"
fi

echo "Running Phase 1 tests with $HELLO_WASM"
if [ -n "${LAYER36_HELLO_SHA256:-}" ]; then
  echo "Expecting hello component sha256: $LAYER36_HELLO_SHA256"
fi

LAYER36_HELLO_WASM="$HELLO_WASM" cargo test --workspace
