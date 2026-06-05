# Milestones

## v1.2.0 Robustness & Safety Hardening (Shipped: 2026-06-05)

**Phases completed:** 2 phases (6–7), 5 plans, 6 tasks
**Git range:** `82d11e5` (milestone start) → `57026c6` (v1.2.0 bump)

**Delivered:** Make rust-to-you safe to point at *any* repository (huge or hostile) and safe to interrupt at any moment — no machine hang, no orphaned temp files, no injection surface.

**Key accomplishments:**

- **Pre-flight size guard (Phase 6):** repos larger than 500 MB are refused *before* the clone starts via the GitHub API `size` field, with an explicit `--deep` opt-in and a bilingual too-large message (new exit code 6).
- **Intake parser hardening (Phase 6):** leading-dash argument injection and over-length owner/repo segments are rejected at `parse_repo_ref` before any network/git call (distinct `UnsafeInput` message, exit 2); existing `..`/`\`/host guards preserved.
- **Documented intake threat model (Phase 6):** `docs/THREAT-MODEL.md` (STRIDE) backed by injection/abuse tests; notes git2 = libgit2 FFI (no shell spawn).
- **Interruptible lifecycle (Phase 7):** SIGINT/SIGTERM handler + panic hook clean the live clone temp dir on every exit path (exit 130), tracked via an idempotent global registry — verified live against a 6 GB clone.
- **Self-healing temp hygiene (Phase 7):** age-based startup sweep removes orphaned `rust-to-you-clone-*` dirs (>60 min) left by prior crashes, with a one-line notice only when something was swept.

**Quality:** every requirement (GUARD-01/02/03, SEC-01/02, CLEAN-01/02/03) verified via UAT; both phases threat-secured (`*-SECURITY.md`, 0 open). 70 unit tests + interrupt integration tests, clippy clean.

---

## v1.0 v1.0 (Shipped: 2026-06-03)

**Phases completed:** 5 phases, 13 plans, 5 tasks

**Key accomplishments:**

- Scaffolded Cargo project and CLI surface with generous URL parsing and centralized IntakeError taxonomy.
- Built the investigation session contract, deterministic case_id generator, and the run integration seam.

---

## v1.1.0 — Release & Polish (Shipped: 2026-06-03)

**Type:** Ad-hoc release (làm ngoài GSD milestone flow; ghi nhận hồi tố)

**Key accomplishments:**

- cargo-dist: phát hành prebuilt binary (shell installer + Homebrew tap).
- TUI: cuộn bằng trackpad/mouse-wheel; tinh chỉnh title "Branch Jungle" bản VI.
- Docs: README song ngữ (VI primary + EN), ARCHITECTURE.md, Ferris hero image.

**Note:** Tag `v1.1.0` đóng trước 2 commit docs (93bb231, 15b469e); chúng vẫn thuộc đợt v1.1.0 này.

---
