# Setting up your environment

## Prerequisites

- **Rust** (stable, via [rustup](https://rustup.rs/))
- **Git**

Optional (needed for specific phases):
- `cargo-component`: for building WASM components (Phase 1+)
- `wasm-tools`: for WASM inspection and optimization (Phase 1+)
- `mdbook`: for building the docs site (`cargo install mdbook`)
- `tinygo` and `go`: for the Phase 2 Go binding path
- `node`, `npm`, and `jco`: for the Phase 2 TypeScript binding path

If you want GitHub to run jobs on your own machine, see
[CI and runners](ci-and-runners.md).

Run this after setup:

```bash
layer36 doctor
```

It reports the core Rust tools first, then the optional language-binding tools.
Missing Go or TypeScript tools are fine until you work on those binding tracks.

The first TypeScript SDK scaffold lives in `packages/sdk-ts`. It does not require
`jco` for normal Rust runtime work, but the TypeScript sample build will need it.

## Clone and build

```bash
git clone https://github.com/incyashraj/layer6x6.git
cd layer6x6
cargo build --workspace
scripts/test-phase1.sh
```

This should work in under 10 minutes on a fresh machine. If it doesn't,
[open an issue](https://github.com/incyashraj/layer6x6/issues/new?template=bug_report.md): that's a bug.

## Run the docs site locally

```bash
cargo install mdbook     # one-time
mdbook serve docs/book
# then open http://localhost:3000
```

## Useful commands

```bash
cargo fmt --all -- --check                 # check formatting
cargo clippy --all-targets --all-features -- -D warnings  # lint
scripts/test-phase1.sh                     # build fixture + run all tests
cargo build --workspace                    # build everything
```
