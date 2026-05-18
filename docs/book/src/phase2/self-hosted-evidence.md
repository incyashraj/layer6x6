# Self-Hosted Evidence

Phase 2 needs one recent local full-gate run before exit.

Hosted CI tells us the normal push checks are healthy. The self-hosted full gate
answers a different question: can the heavier local path still build fixtures,
run language checks, run benchmarks, run fuzz smoke, build docs, and record UAPI
freeze evidence on the macOS ARM64 runner.

```bash
scripts/record-phase2-self-hosted-evidence.sh
```

Default output:

```text
target/phase2-self-hosted-evidence/self-hosted-evidence.md
```

For final exit review, require at least one completed green self-hosted run in
the inspected history:

```bash
scripts/record-phase2-self-hosted-evidence.sh --require-success
```

When the final candidate commit is ready, narrow the report to the review window
too:

```bash
scripts/record-phase2-self-hosted-evidence.sh --created '>=2026-05-18' --require-success
```

You can also include this report in the Phase 2 exit bundle:

```bash
scripts/record-phase2-exit-bundle.sh --strict --include-self-hosted
```

The exit bundle uses the stricter success check. If the latest completed
self-hosted history does not show a green run, the strict bundle fails instead
of hiding the problem in a report.

## What It Records

The report includes:

- repository and branch
- workflow file or workflow name
- git commit at recording time
- optional GitHub creation-date filter
- latest completed self-hosted run
- recent run history
- completed success streak
- required success streak when strict checking is enabled

It uses GitHub CLI, so it needs a logged-in `gh` session with repository access.

## What It Does Not Prove

This report records GitHub workflow history. It does not replace the uploaded
artifacts from the workflow itself.

For final Phase 2 review, read it beside:

- the self-hosted workflow run page
- the UAPI freeze review artifact
- benchmark output
- fuzz smoke output
- the exit bundle

## Current Reading

A recent green self-hosted full gate is still one of the final proof items for
Phase 2. This recorder gives that proof a stable markdown shape so the final
review does not rely on a screenshot.
