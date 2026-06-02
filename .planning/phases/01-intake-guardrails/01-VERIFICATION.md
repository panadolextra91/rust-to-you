---
phase: 01-intake-guardrails
verified: 2026-06-02T00:00:00Z
status: passed
score: 4/4 must-haves verified
---

# Phase 1: Intake & Guardrails Verification Report

**Phase Goal:** Accept one public GitHub URL, reject unsupported cases clearly, and establish the investigation session contract.
**Verified:** 2026-06-02 (manual verification by validation partner; implementation by Antigravity)
**Status:** passed

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `rust-to-you <url>` starts an investigation and prints the opened-case line | ✓ VERIFIED | `cargo run -- https://github.com/tokio-rs/axum` → `🦀 Investigation opened: tokio-rs/axum — Case AXUM-206F`, exit 0 |
| 2 | Invalid/unsupported/malformed inputs produce clear crab errors on stderr with tiered exit codes | ✓ VERIFIED | gitlab→exit 2 (UnsupportedHost), onlyowner→exit 2 (MalformedRepoPath), ssh→exit 2 (NotAUrl); all messages crab-voiced on stderr |
| 3 | Investigation path is read-only + GitHub-public scoped; ZERO network in Phase 1 | ✓ VERIFIED | No network code; `Cargo.lock` has no tokio/reqwest/git2/hyper/mio; non-github host gated |
| 4 | Deterministic case_id, robust to dotted repo names | ✓ VERIFIED | `AXUM-206F` identical across runs; `rust-lang/docs.rs → DOCS-BC41` (filter-then-truncate); REPO fallback tested |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `Cargo.toml` | clap+thiserror, no async/net | ✓ EXISTS + SUBSTANTIVE | edition 2021, bin `rust-to-you`, deps clap(derive)+thiserror only |
| `src/cli/parse.rs` | generous URL parser → RepoRef | ✓ EXISTS + SUBSTANTIVE | `parse_repo_ref`, `RepoRef`, charset/traversal guards, table-driven tests |
| `src/error.rs` | IntakeError taxonomy + exit codes | ✓ EXISTS + SUBSTANTIVE | 7 variants, `exit_code()` maps 2/3/4 |
| `src/app/session.rs` | InvestigationSession + case_id | ✓ EXISTS + SUBSTANTIVE | `generate_case_id` (FNV-1a, deterministic), 4 tests |
| `src/app/run.rs` | run() seam | ✓ EXISTS + SUBSTANTIVE | prints stub line; documented as Phase 2 seam |
| `src/main.rs` | entrypoint wiring | ✓ EXISTS + SUBSTANTIVE | parse → session → run → exit 0; Err → eprintln + exit_code |

**Artifacts:** 6/6 verified

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|----|--------|---------|
| main.rs | parse_repo_ref | `parse_repo_ref(&args.repo)` | ✓ WIRED | Ok → session, Err → stderr+exit |
| main.rs (Ok) | InvestigationSession | `InvestigationSession::new(repo_ref)` | ✓ WIRED | builds session then calls run |
| main.rs (Ok) | run seam | `app::run(&session)` + `exit(0)` | ✓ WIRED | prints stub line, exits 0 |
| main.rs (Err) | stderr + exit code | `eprintln!` + `e.exit_code()` | ✓ WIRED | tiered exit codes confirmed |

**Wiring:** 4/4 connections verified

## Requirements Coverage

| Requirement | Status | Blocking Issue |
|-------------|--------|----------------|
| INPT-01: single-command intake → investigation | ✓ SATISFIED | - |
| INPT-02: clear read-only error for invalid/unsupported/private inputs | ✓ SATISFIED | - |

**Coverage:** 2/2 requirements satisfied

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| src/error.rs | 18-24 | 3 net error variants never constructed | ⚠️ Warning | Intentional — populated in Phase 2 (D-03). Benign dead_code warning. |
| src/app/session.rs | 8 | `started_at` never read | ⚠️ Warning | Intentional — consumed in later phases. Benign dead_code warning. |

**Anti-patterns:** 0 blockers, 2 intentional warnings (both within Phase 1 scope; no stubs/TODOs in logic paths).

## Quality Gates

- `cargo build` → ✓ success (2 intentional dead_code warnings only)
- `cargo test` → ✓ 6/6 passed
- `cargo clippy` → ✓ no real lints (only the dead_code warnings)
- Decision compliance → ✓ D-01..D-08 all delivered at full scope

## Human Verification Required

None — all must-haves verified programmatically.

## Gaps Summary

**No gaps found.** Phase goal achieved. Ready to proceed to Phase 2.

### Non-Critical Nits (deferred, do not block)

1. **dead_code warnings** — add `#[allow(dead_code)] // wired in Phase 2` on the reserved net variants + `started_at` to keep the build warning-clean and document intent.
2. **Lone dotted token UX** — input like `docs.rs` (no owner) reports `UnsupportedHost` rather than `MalformedRepoPath`. Rare edge; low priority.
3. **`NotAUrl { input }`** — field captured but not shown in the message. Cosmetic; drop or display.

---

*Phase: 01-intake-guardrails*
*Verified: 2026-06-02 (manual)*
