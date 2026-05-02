# Layer36 (from layer6x6)

> Write once. Run on everything. Natively.

**Naming note:** This repository lives at `incyashraj/layer6x6` while the
project is still proving the 6x6 portability matrix. The product name is
**Layer36**: layer6x6 becomes Layer36 once the matrix is solved.

Layer36 is a universal application platform — a portable runtime, a universal
standard library (UAPI), and a capability-based permission model (UCap) — that
lets you ship one binary and run it natively on Windows, macOS, Linux, iOS,
Android, and the web.

It is built on WebAssembly and its Component Model, with a thin per-OS adapter
layer that translates UAPI calls to native OS APIs.

**Status:** Pre-alpha. Phase 1 (POC Runtime). Local hello-world execution works.  
See [the roadmap](https://incyashraj.github.io/layer6x6/roadmap.html).

---

## Why

Every app today is written six times: once for each operating system it runs
on. That is a tax on every developer and a ceiling on every idea. Layer36 removes
the tax by making the developer's target a portable runtime, not any particular
OS.

Read [the full vision](https://incyashraj.github.io/layer6x6/vision.html).

## Current phase

**Phase 1 — POC Runtime.** The current slice proves a Rust WebAssembly
component can run through the Layer36 CLI and temporary WIT host imports.
Cross-host CI and release artifacts are still being hardened.

## Quickstart

Phase 1 has started. The development CLI can run the Phase 1 hello component
after the fixture is built:

```bash
scripts/build-hello-component.sh
cargo run -p layer36-cli -- run test/integration/hello-world/target/wasm32-wasip1/release/hello_world.wasm
```

Expected output:

```text
Hello, Layer36!
```

Current local runtime benchmarks are recorded in the
[Phase 1 benchmark page](docs/book/src/phase1/benchmarks.md).

Run the full Phase 1 local test harness with:

```bash
scripts/test-phase1.sh
```

For the full walkthrough, read the
[Quickstart](https://incyashraj.github.io/layer6x6/quickstart.html).

## Security

Layer36 is pre-alpha. Do not run untrusted WASM through `layer36` in Phase 1.
Treat `layer36 run foo.wasm` like running a local developer executable. The
sandbox is real, but the platform is not adversarially hardened yet.

See the [Phase 1 threat model](docs/book/src/phase1/threat-model.md).

## Project structure

```
crates/         # Rust crates (appears in Phase 1)
wit/            # WebAssembly Interface Types definitions (Phase 1)
apps/           # Sample and dogfood apps (Phase 2)
docs/           # Documentation, ADRs, mdBook site source
  adr/          # Architecture Decision Records
  book/         # mdBook source
  legal/        # Trademark, legal notes
  rfc/          # Proposals
Plan/           # Phase-by-phase build plans (living documents)
src/            # Phase 0 workspace sentinel; runtime crates start in Phase 1
test/           # Integration tests (Phase 1)
scripts/        # Dev tooling scripts
```

## Contributing

We want you. Read [CONTRIBUTING.md](CONTRIBUTING.md) and start with
[GitHub Discussions](https://github.com/incyashraj/layer6x6/discussions).
The Discord invite will be added once the Phase 0 community server is live.

Good first issues are labeled
[`good first issue`](https://github.com/incyashraj/layer6x6/labels/good%20first%20issue).

## License

Dual-licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option. Contributions are dual-licensed under the same terms.

## Acknowledgements

Layer36 stands on the shoulders of the
[Bytecode Alliance](https://bytecodealliance.org/), the
[Rust Foundation](https://foundation.rust-lang.org/), and everyone else
building the open WebAssembly ecosystem.
