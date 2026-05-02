#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
RUSTUP_CARGO=""

if command -v rustup >/dev/null 2>&1; then
  RUSTUP_CARGO="$(rustup which cargo 2>/dev/null || true)"
fi

if [ -n "$RUSTUP_CARGO" ]; then
  PATH="$(dirname -- "$RUSTUP_CARGO"):$HOME/.cargo/bin:$PATH"
elif [ -d "$HOME/.cargo/bin" ]; then
  PATH="$HOME/.cargo/bin:$PATH"
fi

cd "$ROOT/test/integration/hello-world"
cargo-component build --release --locked

echo "$ROOT/test/integration/hello-world/target/wasm32-wasip1/release/hello_world.wasm"
