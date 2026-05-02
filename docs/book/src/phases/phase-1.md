# Phase 1 — POC Runtime

**Status:** In Progress  
**Duration:** Months 2–3  
**Sentence:** *Prove one `.wasm` binary runs identically on three desktop hosts.*

## Current slice

Phase 1 has started with the runtime and CLI scaffold:

- `crates/runtime` initializes Wasmtime with Component Model support.
- `crates/cli` builds the `layer36` binary.
- `layer36 --help`, `layer36 version`, and `layer36 doctor` work locally.
- `layer36 run <file>` validates input paths and routes component execution
  through the runtime crate.
- `scripts/build-hello-component.sh` builds the Rust hello-world component.
- `layer36 run test/integration/hello-world/target/wasm32-wasip1/release/hello_world.wasm`
  prints `Hello, Layer36!` locally.
- `layer36 run --fuel 1 ...` and `layer36 run --mem-limit 0 ...` fail
  cleanly with exit code 4 and a limit-exceeded message.
- The CI test matrix now builds the hello-world component, verifies its
  SHA-256 fixture hash, and runs it through the real `layer36` binary on each
  desktop host.
- `scripts/test-phase1.sh` builds the hello-world fixture and runs the full
  workspace test suite with `LAYER36_HELLO_WASM` configured.
- `.github/workflows/release.yml` packages the five planned Phase 1 release
  artifacts on `v*` tags and publishes a `SHA256SUMS` file.
- The Phase 1 quickstart is published at `docs/book/src/quickstart.md` and
  walks from a fresh checkout to `Hello, Layer36!`.
- Threat Model v0.1 is published at `docs/book/src/phase1/threat-model.md`.
- Baseline runtime benchmarks are published at
  `docs/book/src/phase1/benchmarks.md` and checked in CI as warning-only
  regression signals.

The runtime now registers the temporary `layer36:phase1/host` WIT interface for
`print` and `exit`. Remote CI/release results, cross-host benchmark runs, and
RSS measurements are still pending.

See [`Plan/Phase-1-Plan.md`](https://github.com/incyashraj/layer6x6/blob/main/Plan/Phase-1-Plan.md).
