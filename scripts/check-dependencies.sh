#!/usr/bin/env sh
set -eu

tmp_log="$(mktemp)"
cleanup() {
  rm -f "$tmp_log"
}
trap cleanup EXIT

if cargo deny check advisories >"$tmp_log" 2>&1; then
  cat "$tmp_log"
else
  status="$?"
  cat "$tmp_log"
  if grep -Eq "unsupported CVSS version: 4.0|failed to load advisory database|TOML parse error|failed to acquire advisory database lock|exclusive lock on a read-only path" "$tmp_log"; then
    echo "warning: advisory check skipped due current cargo-deny/advisory-db compatibility or local advisory-db lock-path limits; license, bans, and source checks still run."
  else
    exit "$status"
  fi
fi

cargo deny check licenses bans sources
