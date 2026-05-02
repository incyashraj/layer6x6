#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"

HELLO_WASM="$("$ROOT/scripts/build-hello-component.sh")"

echo "Running Phase 1 tests with $HELLO_WASM"
LAYER36_HELLO_WASM="$HELLO_WASM" cargo test --workspace
