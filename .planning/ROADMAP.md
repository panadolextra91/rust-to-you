# Roadmap: rust-to-you

## Overview

rust-to-you turns one public-GitHub URL into a cute, bilingual one-scroll TUI investigation
report. v1.0 built the layered engine (intake → collection → analysis → presentation →
narrative); v1.2.0 hardened it to be safe against huge/hostile repos and safe to interrupt.

## Milestones

- ✅ **v1.0 MVP** — Phases 1–5 (shipped 2026-06-03) → [archive](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1.0 Release & Polish** — ad-hoc release, no GSD phases (shipped 2026-06-03)
- ✅ **v1.2.0 Robustness & Safety Hardening** — Phases 6–7 (shipped 2026-06-05) → [archive](milestones/v1.2.0-ROADMAP.md)

## Phases

<details>
<summary>✅ v1.0 MVP (Phases 1–5) — SHIPPED 2026-06-03</summary>

- [x] Phase 1: Intake & Guardrails (2/2 plans) — completed 2026-06-02
- [x] Phase 2: Collection Layer (4/4 plans) — completed 2026-06-03
- [x] Phase 3: Analysis Layer (3/3 plans) — completed 2026-06-03
- [x] Phase 4: Presentation Layer (2/2 plans) — completed 2026-06-03
- [x] Phase 5: Polish & Calibration (2/2 plans) — completed 2026-06-03

Full detail: [milestones/v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)

</details>

<details>
<summary>✅ v1.2.0 Robustness & Safety Hardening (Phases 6–7) — SHIPPED 2026-06-05</summary>

- [x] Phase 6: Safe Intake & Pre-flight Guard (3/3 plans) — completed 2026-06-05
      Refuse oversized repos before the clone (500 MB guard + `--deep`), harden the intake
      parser against injection/abuse, document the threat model.
- [x] Phase 7: Interruptible Lifecycle & Temp Hygiene (2/2 plans) — completed 2026-06-05
      Clean the clone temp dir on every exit path (Ctrl-C / crash / panic, exit 130) and
      sweep orphaned temp dirs on startup.

Full detail: [milestones/v1.2.0-ROADMAP.md](milestones/v1.2.0-ROADMAP.md)

</details>

## Progress

| Phase | Milestone | Plans | Status | Completed |
|-------|-----------|-------|--------|-----------|
| 1. Intake & Guardrails | v1.0 | 2/2 | Complete | 2026-06-02 |
| 2. Collection Layer | v1.0 | 4/4 | Complete | 2026-06-03 |
| 3. Analysis Layer | v1.0 | 3/3 | Complete | 2026-06-03 |
| 4. Presentation Layer | v1.0 | 2/2 | Complete | 2026-06-03 |
| 5. Polish & Calibration | v1.0 | 2/2 | Complete | 2026-06-03 |
| 6. Safe Intake & Pre-flight Guard | v1.2.0 | 3/3 | Complete | 2026-06-05 |
| 7. Interruptible Lifecycle & Temp Hygiene | v1.2.0 | 2/2 | Complete | 2026-06-05 |

---

*Next milestone: run `/gsd-new-milestone` to define fresh requirements and phases.*
