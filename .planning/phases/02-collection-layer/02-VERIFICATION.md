---
phase: 02-collection-layer
verified: 2026-06-03T00:00:00Z
status: passed
score: 4/4 must-haves verified
---

# Phase 2: Collection Layer Verification Report

**Phase Goal:** Collect all remote and local evidence required for the report without over-fetching, normalized into one InvestigationSnapshot (incl. a walking skeleton proving the full clone → single-metric → printed-output pipeline end-to-end).
**Verified:** 2026-06-03 (manual verification by validation partner; implementation by Antigravity; one post-review fix round)
**Status:** passed

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Tool fetches overview metadata for a public GitHub repo | ✓ VERIFIED | e2e `octocat/Hello-World` → Stars 3621 / Forks 6247 from the live API; single `GET /repos` with mandatory User-Agent |
| 2 | Tool gathers git/branch/history evidence + a local snapshot | ✓ VERIFIED | e2e: age 5606d, 3 commits, 2 contributors, bus factor 1, 3 branches; `InvestigationSnapshot` assembled in `collect()` |
| 3 | Tool detects language + infrastructure signals | ✓ VERIFIED | tokei language breakdown + 8 infra detectors (unit tests `scan::lang`, `scan::infra` green) |
| 4 | Walking skeleton proves full clone → metric → printed output e2e | ✓ VERIFIED | e2e clones the repo and prints repo age + full report via the Phase 1 `run(&session)` seam |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Status | Details |
|----------|--------|---------|
| `Cargo.toml` deps | ✓ | git2 (https+vendored-openssl), reqwest blocking (no-default), tokei (no-default), tempfile, chrono, serde — NO tokio |
| `src/lib.rs` + `src/main.rs` | ✓ | crate converted to lib+bin; main is a thin shim → `cargo test --lib` works |
| `src/github/client.rs` | ✓ | `fetch_metadata` (User-Agent, optional GITHUB_TOKEN, 1 call), pure `classify(StatusCode)`, serde `RepoMetadata` |
| `src/repo/clone.rs` | ✓ | `clone_repo` → `CloneWorkspace` (tempfile RAII auto-clean) |
| `src/repo/history.rs` | ✓ | revwalk counts, repo age, contributors+bus_factor (bot/merge/mailmap filter), bounded most-modified + time-of-day |
| `src/repo/branches.rs` | ✓ | `enumerate_branches` via `BranchType::Remote`, default from HEAD |
| `src/scan/{lang,infra}.rs` | ✓ | tokei breakdown + 8 path-presence infra detectors |
| `src/snapshot.rs` | ✓ | `InvestigationSnapshot` + `RepoMetaState::{Available,Unavailable}` (explicit degrade) |
| `src/app/collect.rs` | ✓ | API-first orchestration, 404→abort, transient→degrade |

### Key Link Verification

| From | To | Status | Details |
|------|----|--------|---------|
| main → run → collect | github + repo + scan | ✓ WIRED | full pipeline runs end-to-end (e2e) |
| collect → IntakeError | Phase 1 taxonomy | ✓ WIRED | populates RepoNotFoundOrPrivate (404, exit 3), Network (clone), + new CollectionFailed (exit 5) — Phase 1 dead_code cleared |
| collect → snapshot | run() display | ✓ WIRED | RepoMetaState handled; "based on last N commits" caveat surfaced when capped |

## Requirements Coverage

| Requirement | Status |
|-------------|--------|
| COLL-01 (overview metadata: age, default branch, stars, forks, contributors, last activity) | ✓ SATISFIED |
| COLL-02 (git/branch/history evidence + local snapshot) | ✓ SATISFIED |
| COLL-03 (language + infrastructure signals, read-only) | ✓ SATISFIED |

**Coverage:** 3/3 requirements satisfied

## Quality Gates

- `cargo build` → ✓ (git2 vendored-openssl links — build trap #1 avoided)
- `cargo test` → ✓ **16/16 passed** (incl. `test_time_of_day_buckets`)
- `cargo clippy` → ✓ **0 warnings**
- e2e live clone (`octocat/Hello-World`) → ✓ exit 0
- e2e not-found repo → ✓ clean message + exit 3
- Decision compliance → ✓ D-01..D-07 all delivered

## Decision Compliance (CONTEXT D-01..D-07)

| Decision | Verdict |
|----------|---------|
| D-01 temp + RAII auto-clean | ✓ `CloneWorkspace` owns `TempDir` |
| D-02 API-first, populate IntakeError variants | ✓ `collect()` |
| D-03 404 abort / transient degrade (stars/forks unknown) | ✓ verified e2e + `RepoMetaState::Unavailable` |
| D-04 GITHUB_TOKEN if present | ✓ `fetch_metadata` |
| D-05 bounded 1000 + "based on last N commits" | ✓ `COMMIT_WINDOW_CAP`, `CommitWindow.capped` (tested) |
| D-06 full history for cheap counts | ✓ total_commits/contributors/age full walk |
| D-07 walking skeleton via run() seam | ✓ |

Research-derived traps all mitigated: git2 `https`/`vendored-openssl` feature, GitHub `User-Agent`, fresh-clone `BranchType::Remote`, `mailmap` identity, shared bot/merge filter.

## Post-Review Fixes Applied (this round)

1. Time-of-day buckets (`night`/`weekend`/`business`) made INDEPENDENT (3 separate ifs) and aligned to METRICS (night 00:00–04:59); locked by new `test_time_of_day_buckets`.
2. `collect.rs` no longer swallows git2 errors — core collectors propagate via new `IntakeError::CollectionFailed` (exit 5); API degrade path unchanged.
3. clippy `strip_prefix` cleanup → 0 warnings.

## Human Verification Required

None — all must-haves verified programmatically + via live e2e.

## Gaps Summary

**No gaps found.** Phase goal achieved. Ready for Phase 3.

### Non-Critical Notes (deferred)
- Clone errors collapse to `IntakeError::Network` (loses some specificity vs a dedicated clone-failure variant). Acceptable for V1.
- Time-of-day bucket thresholds remain tunable during Phase 5 vibe calibration.

---

*Phase: 02-collection-layer*
*Verified: 2026-06-03 (manual, post-fix)*
