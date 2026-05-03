# Phase 2: UAPI v0.1

**Status:** Next
**Estimate:** est. 4 to 8 weeks
**Goal:** Make Layer36 useful for small command line apps.

Phase 2 replaces the temporary Phase 1 host interface with real APIs:

- `io`
- `fs`
- `net`
- `time`
- `locale`

The proof apps are:

- `layer36-curl`
- `layer36-cat`
- `layer36-clock`

If Phase 2 works, those apps should produce the same output on Linux, macOS, and
Windows while running through the same Layer36 runtime model.

See [`Plan/Phase-2-Plan.md`](https://github.com/incyashraj/layer6x6/blob/main/Plan/Phase-2-Plan.md).
