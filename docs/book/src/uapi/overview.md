# UAPI Overview

> The UAPI modules will be defined as WIT interfaces starting in Phase 2.

The Universal API (UAPI) is the standard library every Layer36 app calls.
It is defined as [WIT](https://component-model.bytecodealliance.org/design/wit.html)
interfaces, and each module is implemented natively per host platform.

## Target modules for v1.0

```
layer36:
├── io/            # stdio, files, pipes
├── net/           # TCP, UDP, QUIC, HTTP, WebSocket, DNS
├── time/          # clocks, timers, scheduling
├── fs/            # filesystem, paths, metadata
├── ui/            # window, widgets, layout, input, text rendering
├── gfx/           # 2D canvas, 3D GPU (WebGPU-compat), shaders
├── audio/         # playback, capture, mixing
├── sensors/       # accelerometer, gyro, GPS, camera, mic
├── storage/       # key-value, SQL (SQLite), object store
├── crypto/        # hash, symmetric, asymmetric, random, PQ-safe
├── identity/      # DID-based user identity, signing, attestation
├── ipc/           # intra-device messaging, cross-app calls
├── notify/        # system notifications, toasts, badges
├── locale/        # i18n, l10n, formatting
├── accessibility/ # screen reader, high-contrast, reduced motion
├── ai/            # local inference, model loading, embeddings
└── platform/      # device info, capabilities query, power state
```

**Phase 2 scope:** `io`, `fs`, `net` (HTTP client), `time`, `locale`.
