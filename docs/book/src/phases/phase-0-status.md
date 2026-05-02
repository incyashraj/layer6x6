# Phase 0 Status

This page tracks the Phase 0 exit checklist from `Plan/Phase-0-Plan.md`.

## Local repository status

| Area | Status | Notes |
|------|--------|-------|
| Licenses | Done locally | `LICENSE-MIT` and `LICENSE-APACHE` are present. |
| Cargo workspace | Done locally | Root sentinel package keeps baseline Cargo commands green before runtime crates exist. |
| Toolchain | Done locally | `rust-toolchain.toml` pins Rust `1.91.1`. |
| Cargo deny config | Done locally | `cargo-deny` passes advisories, licenses, bans, and sources checks. |
| Line endings | Done locally | `.gitattributes` normalizes text files to LF. |
| Ignore rules | Done locally | `.gitignore` covers Rust, mdBook, editor, OS, and generated files. |
| README | Draft complete | Needs external reader review before Phase 0 exit. |
| CONTRIBUTING | Draft complete | Includes setup, PR flow, commit style, ADRs, and licensing. |
| SECURITY | Draft complete | Uses `security@layer36.dev`; mailbox must be confirmed externally. |
| Code of Conduct | Draft complete | Present and linked from contributor docs. |
| Changelog | Done locally | `[Unreleased]` section initialized. |
| ADRs | Done locally | ADR template and ADR-0001 are present. |
| CI | Configured | `ci.yml` runs fmt, clippy, tests, mdBook, and cargo-deny. |
| mdBook | Done locally | Book source builds with `mdbook build docs/book`; Pages publication remains external. |
| First-PR guide | Draft complete | Screenshot section is pending live GitHub/Pages URLs. |
| Legal records | Naming decision recorded | Layer36 is active project name; official clearance remains pending. |
| Launch drafts | Draft complete | Blog post and social thread drafts exist. |

## External status

| Area | Status | Next action |
|------|--------|-------------|
| Public repo settings | Pending | Confirm description, topics, social preview, and Pages URL in GitHub. |
| Branch protection | Pending | Require CI green, one review, and no force-push on `main`. |
| CI history | Pending | Need five consecutive green days on `main`. |
| GitHub Pages | Pending | Publish and verify the public docs URL. |
| Discord | Pending | Create channels, welcome/rules post, and invite link. |
| Announcement | Pending | Publish only after repo, docs, and Discord are ready. |
| Trademark search | Partially complete | Layer36 lightweight screen found no obvious exact software/runtime conflict; official searches remain. |
| Domain | Pending | Secure `layer36.dev` or chosen equivalent. |
| Founder IP note | Pending | Fill `docs/legal/founder-ip.md`. |
| Good first issues | Pending | Create the five drafts from `docs/community/good-first-issues.md`. |
| External contributor PR | Pending | Needs at least one opened and merged PR. |
| Retrospective | Drafted | Complete `docs/book/src/phases/phase-0-retro.md` after external work. |
| Phase 1 kickoff issue | Drafted | Open from `docs/governance/phase-1-kickoff-issue.md` after Phase 0 exit is approved. |

## Current local verification

The local baseline is:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
scripts/setup.sh
```

Additional checks:

```bash
mdbook build docs/book
cargo deny check advisories licenses bans sources
```
