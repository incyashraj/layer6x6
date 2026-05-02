#!/usr/bin/env sh
set -eu

echo "Checking Rust toolchain..."
rustc -V
cargo -V

echo "Building workspace..."
cargo build --workspace

echo "Running Phase 1 tests..."
scripts/test-phase1.sh

echo "Checking formatting..."
cargo fmt --all -- --check

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Setup check complete."
