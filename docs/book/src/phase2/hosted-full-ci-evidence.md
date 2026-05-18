# Hosted Full CI Evidence

Normal push CI is intentionally small. It checks the fast Linux path, docs, UAPI
freshness, the Rust SDK package shape, formatting, and linting.

Phase 2 exit also needs the heavier hosted full CI path. That path runs the
Linux, macOS, and Windows lanes and records the cross-host evidence artifacts.

Use this recorder to prove that a recent hosted CI run was a full run, not only
the fast push run:

```bash
scripts/record-phase2-hosted-full-ci-evidence.sh
```

Default output:

```text
target/phase2-hosted-full-ci-evidence/hosted-full-ci-evidence.md
```

For final review, require a completed full run with every required full job
green:

```bash
scripts/record-phase2-hosted-full-ci-evidence.sh --require-success
```

To limit the report to the final review window:

```bash
scripts/record-phase2-hosted-full-ci-evidence.sh \
  --created '>=2026-05-18' \
  --require-success
```

You can include it in the exit bundle:

```bash
scripts/record-phase2-exit-bundle.sh --strict --include-hosted-full-ci
```

The final review shortcut includes it too:

```bash
scripts/record-phase2-exit-bundle.sh --final-review
```

## Required Full Jobs

The recorder checks these hosted CI jobs:

- Phase 2 bindings
- Build shared component fixtures
- Full test on Linux
- Full test on macOS
- Full test on Windows
- Language variant evidence compare
- UCap enforcement evidence compare
- Adapter evidence compare
- Sample evidence compare
- Phase 2 benchmark check
- Dependency audit

If those jobs are missing or skipped, the run is not counted as hosted full CI
proof.

## How To Run Hosted Full CI

Use one of these paths:

```bash
gh workflow run CI --ref main -f full=true -f language_variants_mode=ts
```

or include `[full-ci]` in a commit message.

The manual workflow path is cleaner for final review because it does not require
a documentation-only commit just to trigger the heavy matrix.

## What This Does Not Prove

This is hosted full CI proof only.

It does not replace:

- normal hosted CI and Pages stability history
- self-hosted macOS ARM64 full-gate proof
- long fuzz soak proof
- the outside Rust walkthrough
- the final UAPI freeze decision

Each track answers a different question, so the final Phase 2 packet should
keep them separate.
