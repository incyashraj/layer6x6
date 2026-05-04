# Phase 2 Dispatch Benchmarks

Phase 2 adds UAPI, so the runtime does more work than Phase 1. Every app call
now passes through a policy check before the host adapter touches files,
network, time, locale, or streams. That check needs to be cheap.

These numbers are an early local read, not a release promise. They tell us
whether the current design is in the right range before we freeze UAPI v0.1.

## Reference Machine

| Field | Value |
|---|---|
| Date | 2026-05-04 |
| CPU | Apple M4 |
| OS | macOS |
| Architecture | arm64 |
| Rust | rustc 1.91.1 |

## Command

```bash
cargo bench -p layer36-runtime --bench uapi_dispatch
```

The benchmark uses a no-op host adapter. That means it measures Layer36
dispatcher and policy overhead, not disk speed, terminal speed, or network
speed.

## First Local Read

| Path | Local result | Phase 2 target | Notes |
|---|---:|---:|---|
| Default stdout grant | ~186 ns | < 1 us | Default low-risk IO capability. |
| Filesystem open with read grant | ~611 ns | < 1 us | Path grant check plus adapter call. |
| File handle read with grant re-check | ~623 ns | < 1 us | Re-checks the opened file path before read. |
| File handle write with grant re-check | ~560 ns | < 1 us | Re-checks the opened file path before write. |
| Missing filesystem read grant | ~331 ns | < 1 us | Denial path stops before adapter work. |
| HTTP fetch grant check | ~484 ns | < 1 us | URL endpoint parsing plus `net.connect` check. |

## What This Means

The first dispatcher path is fast enough for the Phase 2 target on the reference
machine. The harder work is still ahead: measuring full component startup,
cross-host variance, real adapter cost, and regressions over time.

For now, this gives us a useful line in the sand. UAPI checks are not free, but
they are small enough that safety is not fighting the basic CLI performance
goal.
