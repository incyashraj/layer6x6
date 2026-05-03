# ADR process

Architecture Decision Records (ADRs) capture the significant technical
decisions made in Layer36: what was decided, why, and what the consequences are.

## When to write an ADR

Write an ADR when you make a decision that:

- Affects multiple crates, phases, or contributors
- Is difficult or costly to reverse
- Would otherwise be invisible in code

Examples that need an ADR:
- Choosing a new dependency for the runtime
- Changing the WIT versioning strategy
- Adding a new UAPI module
- Changing the bundle format

Examples that do **not** need an ADR:
- Bug fixes
- Test additions
- Documentation updates
- Refactors that don't change observable behavior

## Process

1. Copy `docs/adr/template.md` to `docs/adr/NNNN-short-title.md` (next sequential number).
2. Fill out every section. Be honest about alternatives rejected.
3. Open a PR titled `ADR: <title>`.
4. Minimum 2 maintainers approve, or 1 approve + 7 days open.
5. **Merged ADRs are immutable.** To supersede an ADR, write a new one that
   references the old one with `Supersedes: ADR-NNNN`.

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [ADR-0001](../../../adr/0001-rust-for-runtime.md) | Rust for the Layer36 runtime | Accepted |

*(Expand this table as ADRs are merged.)*
