# Adapter Evidence

This page tracks repeatable cross-host proof for the Phase 2 adapter boundary
gate.

The rule is simple:

- runtime policy checks stay in runtime code
- host-facing calls go through per-OS adapter crates

## Record One Host Report

Run this on Linux, macOS, and Windows:

```bash
scripts/record-phase2-adapter-evidence.sh --strict
```

Default output path:

`target/phase2-adapter-evidence/adapter-evidence.md`

Optional custom output:

```bash
scripts/record-phase2-adapter-evidence.sh --strict --output /tmp/adapter-linux.md
```

## Compare Three Host Reports

After collecting one report per host:

```bash
scripts/compare-phase2-adapter-evidence.sh /tmp/adapter-linux.md /tmp/adapter-macos.md /tmp/adapter-windows.md
```

The compare step checks:

- same commit metadata across all reports
- host labels match expected OS lanes
- `scripts/check-adapter-boundary.sh` passed on each host

## Hosted CI Evidence

Full hosted CI now uploads one adapter evidence artifact per OS:

- `adapter-evidence-ubuntu-latest`
- `adapter-evidence-macos-latest`
- `adapter-evidence-windows-latest`

Then CI runs:

- `Adapter evidence compare`

This gives a direct cross-host evidence path for `P2E-02`.
