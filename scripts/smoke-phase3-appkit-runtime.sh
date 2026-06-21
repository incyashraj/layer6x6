#!/usr/bin/env bash
set -euo pipefail

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
cd "$ROOT"

if [ "$(uname -s)" != "Darwin" ]; then
  echo "Layer36 Phase 3 AppKit runtime smoke skipped: host is not macOS"
  exit 0
fi

cargo run -p layer36-runtime --example phase3_appkit_runtime_smoke
