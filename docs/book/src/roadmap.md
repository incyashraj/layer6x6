# Roadmap

Layer36 follows an eight-phase, 24-month plan to v1.0.

Full details for each phase live in the `Plan/` directory.

---

## Phase overview

| # | Phase | Sentence | Duration | Status |
|---|-------|----------|----------|--------|
| 0 | Foundation | Get the project bones set up so real work can start. | Weeks 1–4 | **In Progress** |
| 1 | POC Runtime | Prove one binary runs identically on three desktop hosts. | Months 2–3 | Not started |
| 2 | UAPI v0.1 (CLI) | Ship the first useful cross-platform CLI app through our runtime. | Months 4–6 | Not started |
| 3 | UI + Graphics | First GUI app running natively on Win / macOS / Linux. | Months 7–10 | Not started |
| 4 | Mobile Hosts | Same app runs on iOS and Android. | Months 11–14 | Not started |
| 5 | Developer SDK | A dev can `layer36 new hello && layer36 run` in under 60 seconds. | Months 15–18 | Not started |
| 6 | Distribution & Identity | Users discover, install, update, and sign in across devices. | Months 19–22 | Not started |
| 7 | v1.0 Hardening | ParkSure migrated end-to-end; public launch. | Months 23–24 | Not started |

---

## Phase 0 — Foundation (Current)

**Objective:** A contributor can `git clone`, open their editor, and be
productive in under an hour — without asking anyone a question.

**Deliverables:**
- Monorepo with MIT + Apache-2.0 dual license
- README, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT
- Repo skeleton (licenses, CI, gitignore, gitattributes)
- ADR-0001: "We use Rust for the runtime"
- CI pipeline for Linux, macOS, and Windows
- mdBook documentation site
- Discord server (pending external setup)

**Exit criteria:** `git clone && cargo build` succeeds in ≤ 10 minutes.
CI green on `main`. One external contributor PR merged.

---

For the full build plan including architecture, technology stack, testing
strategy, security model, and go-to-market, see
[`Plan/Build-Plan.md`](https://github.com/incyashraj/layer6x6/blob/main/Plan/Build-Plan.md).
