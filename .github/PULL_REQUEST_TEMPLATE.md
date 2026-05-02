## What does this PR do?

Brief summary of the change and why it's needed.

Closes: <!-- issue number, e.g. Closes #42 or task ID e.g. Closes P1-RT-02 -->

---

## Checklist

- [ ] Task ID is in the branch name (`p{N}-{area}-{n}-description`) and referenced above.
- [ ] `cargo fmt --all -- --check` passes locally.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes locally.
- [ ] `cargo test --workspace` passes locally.
- [ ] If docs changed, `mdbook build docs/book` passes locally.
- [ ] If this adds/changes a public API, WIT interface, or UAPI module — docs and examples are updated.
- [ ] If this is a significant technical decision — an ADR has been opened or referenced.
- [ ] CHANGELOG.md entry added under `[Unreleased]`.

---

## Testing notes

How did you verify this? What edge cases did you consider?

---

## Screenshots / benchmarks (if applicable)
