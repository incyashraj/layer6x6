#!/usr/bin/env bash
set -euo pipefail

cargo test -p layer36-layout
cargo bench -p layer36-layout --bench layout --no-run
