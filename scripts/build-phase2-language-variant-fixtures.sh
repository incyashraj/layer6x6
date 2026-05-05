#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
MODE="${LAYER36_LANGUAGE_VARIANTS_MODE:-optional}"
OUT_DIR="$ROOT/test/integration/language-variants"
SRC_DIR="$ROOT/test/integration/language-variants-src"

has_file() {
  [ -f "$1" ]
}

has_complete_set() {
  prefix="$1"
  has_file "$OUT_DIR/${prefix}_clock.wasm" \
    && has_file "$OUT_DIR/${prefix}_cat.wasm" \
    && has_file "$OUT_DIR/${prefix}_curl.wasm"
}

jco_runner() {
  if command -v jco >/dev/null 2>&1; then
    jco "$@"
    return
  fi

  if command -v npx >/dev/null 2>&1; then
    npx --no-install jco "$@"
    return
  fi

  return 127
}

can_build_ts() {
  command -v node >/dev/null 2>&1 || return 1
  jco_runner --version >/dev/null 2>&1 || return 1
  return 0
}

build_ts_fixtures() {
  mkdir -p "$OUT_DIR"

  jco_runner componentize \
    "$SRC_DIR/layer36-ts-clock.mjs" \
    --wit "$ROOT/wit/layer36/phase2" \
    --world-name cli \
    --out "$OUT_DIR/layer36_ts_clock.wasm"

  jco_runner componentize \
    "$SRC_DIR/layer36-ts-cat.mjs" \
    --wit "$ROOT/wit/layer36/phase2" \
    --world-name cli \
    --out "$OUT_DIR/layer36_ts_cat.wasm"

  jco_runner componentize \
    "$SRC_DIR/layer36-ts-curl.mjs" \
    --wit "$ROOT/wit/layer36/phase2" \
    --world-name cli \
    --out "$OUT_DIR/layer36_ts_curl.wasm"
}

case "$MODE" in
  optional|any|both|go|ts)
    ;;
  *)
    echo "Phase 2 language-variant build error: unknown LAYER36_LANGUAGE_VARIANTS_MODE='$MODE'." >&2
    echo "Allowed values: optional, any, both, go, ts" >&2
    exit 1
    ;;
esac

mkdir -p "$OUT_DIR"

ts_ready=0
go_ready=0

if has_complete_set "layer36_ts"; then
  ts_ready=1
fi
if has_complete_set "layer36_go"; then
  go_ready=1
fi

if [ "$ts_ready" -eq 0 ]; then
  if can_build_ts; then
    echo "Building TypeScript language-variant fixtures with jco"
    build_ts_fixtures
    ts_ready=1
  else
    echo "TypeScript language-variant fixtures not built: jco path is unavailable."
  fi
fi

if [ "$go_ready" -eq 0 ]; then
  echo "Go language-variant fixtures not built: TinyGo build pipeline is not wired yet in this script."
fi

case "$MODE" in
  optional)
    ;;
  any)
    if [ "$go_ready" -eq 0 ] && [ "$ts_ready" -eq 0 ]; then
      echo "Phase 2 language-variant build error: mode '$MODE' requires at least one complete language fixture set." >&2
      exit 1
    fi
    ;;
  both)
    if [ "$go_ready" -eq 0 ] || [ "$ts_ready" -eq 0 ]; then
      echo "Phase 2 language-variant build error: mode '$MODE' requires complete Go and TypeScript fixture sets." >&2
      exit 1
    fi
    ;;
  go)
    if [ "$go_ready" -eq 0 ]; then
      echo "Phase 2 language-variant build error: mode '$MODE' requires a complete Go fixture set." >&2
      exit 1
    fi
    ;;
  ts)
    if [ "$ts_ready" -eq 0 ]; then
      echo "Phase 2 language-variant build error: mode '$MODE' requires a complete TypeScript fixture set." >&2
      exit 1
    fi
    ;;
esac

echo "Language fixture availability after build step:"
echo "  Go: ${go_ready}"
echo "  TypeScript: ${ts_ready}"
