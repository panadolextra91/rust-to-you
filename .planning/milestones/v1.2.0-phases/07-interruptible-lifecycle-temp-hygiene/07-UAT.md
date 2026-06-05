---
status: complete
phase: 07-interruptible-lifecycle-temp-hygiene
source: [07-01-SUMMARY.md, 07-02-SUMMARY.md]
started: 2026-06-05T08:29:24Z
updated: 2026-06-05T08:29:24Z
---

## Current Test

[testing complete]

## Tests

### 1. Ctrl-C / SIGINT mid-clone leaves no orphan, exits 130 (CLEAN-01)
expected: Interrupting an in-progress clone cleans the live temp dir before exit, prints a bilingual Ferris goodbye, and exits with code 130.
result: pass
note: Verified live. Ran `rust-to-you https://github.com/torvalds/linux --deep`, waited for the `rust-to-you-clone-*` dir to appear (~2.2s, clone in flight), sent SIGINT. Output: "🦀 Ferris dọn dẹp rồi rút nha / Ferris cleaned up — bye"; exit 130; the live temp dir (`rust-to-you-clone-FUkcDC`) was removed, no orphan. This is the exact 5 MB leak from Phase 6 UAT — now fixed.

### 2. SIGTERM mid-clone leaves no orphan, exits 130 (CLEAN-01)
expected: SIGTERM mid-clone behaves identically to SIGINT (cleanup + exit 130).
result: pass
note: Verified via the `tests/interrupt.rs` integration test `sigterm_cleans` (rewritten to clone a large repo so the signal lands inside the leak window). `cargo test --test interrupt -- --ignored` → 2 passed (sigint + sigterm), no orphans left behind.

### 3. Interrupt before the clone starts still exits 130 cleanly (CLEAN-01 edge, D-03)
expected: SIGINT during metadata fetch (no CloneWorkspace alive yet) still prints the goodbye and exits 130, leaving nothing behind.
result: pass
note: Verified live. SIGINT ~150ms in (before any temp dir was created) → exit 130 + bilingual goodbye + no orphan (cleanup takes the empty slot harmlessly).

### 4. Startup sweep self-heals orphans, age- and prefix-selective (CLEAN-02)
expected: On startup the tool removes stale `rust-to-you-clone-*` dirs (>60 min), keeps fresh matching dirs and non-matching dirs, and prints a bilingual notice ONLY when ≥1 dir was removed.
result: pass
note: Verified live. Seeded a stale matching dir (mtime backdated to Jan 1), a fresh matching dir, and a non-matching dir, then ran the tool. Output: "🦀 Ferris dọn 1 temp cũ từ lần trước nha / Ferris swept 1 stale temp dir(s) from a previous run". Stale → removed; fresh → kept (the concurrent-instance safety case); non-matching → kept. Re-running with nothing stale prints no notice.

### 5. Panic cleans the live temp + friendly message, backtrace hidden by default (CLEAN-03, D-04)
expected: On panic while a workspace is alive, the panic hook cleans the temp dir and prints a friendly bilingual Ferris line; the default backtrace is suppressed unless RUST_BACKTRACE is set.
result: pass
note: Verified by unit test `repo::hygiene::tests::panic_cleans_temp` (registers a temp, installs the hook, `catch_unwind(panic)`, asserts the dir is gone and the slot is None). Backtrace suppression is code-confirmed: the hook only calls the default hook when `std::env::var_os("RUST_BACKTRACE").is_some()`.

### 6. Cleanup is idempotent — Drop + handler/hook never double-remove or error (CLEAN-03)
expected: `cleanup_live_temp()` uses `.take()` so a second invocation (e.g. panic hook runs, then Drop runs on unwind) is a silent no-op.
result: pass
note: Verified by unit test `cleanup_idempotent` (second call no-ops, slot None). Mutex is poison-resilient (`unwrap_or_else(|e| e.into_inner())`). The two global-slot tests are serialized with a guard mutex to keep the parallel runner from racing them.

### 7. No regression — full test suite + lint green
expected: All prior + new tests pass; lint clean.
result: pass
note: `cargo test --lib` → 70/70 (65 prior + 5 hygiene). `cargo clippy --all-targets` → clean. `cargo test --test interrupt -- --ignored` → 2/2.

## Summary

total: 7
passed: 7
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none — all 3 CLEAN requirements verified; no code issues found]

## Verification Method Notes

- Build: `cargo build` (working tree, uncommitted Phase 7 changes incl. 3 post-review test cleanups) → binary OK.
- CLI behaviors 1, 3, 4 exercised end-to-end against the built binary; behavior 2 via the rewritten integration test; behaviors 5, 6 via unit tests.
- Post-review cleanups applied before this UAT: (#1) integration test rewritten to use a large repo so it reliably hits the leak window; (#2) two global-slot unit tests serialized; (#3) `clone.rs` clears the live-temp slot on clone failure.
- All test-created temp dirs were cleaned (manually for the interrupted live runs; automatically by the tests).
