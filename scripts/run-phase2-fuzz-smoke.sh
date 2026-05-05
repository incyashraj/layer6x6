#!/usr/bin/env sh
set -eu

if ! command -v cargo-fuzz >/dev/null 2>&1; then
  echo "cargo-fuzz is not installed. Install with:"
  echo "  cargo install cargo-fuzz --locked"
  exit 1
fi

cargo fuzz run manifest_parse -- -max_total_time=30
cargo fuzz run logical_path_parse -- -max_total_time=30
cargo fuzz run policy_match -- -max_total_time=30
