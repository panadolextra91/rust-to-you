---
phase: 7
slug: interruptible-lifecycle-temp-hygiene
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-06-05
---

# Phase 7 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Built-in Rust test harness (`#[cfg(test)]` inline + `tests/` integration) |
| **Config file** | none — Cargo convention; `tests/interrupt.rs` created in Wave 0 |
| **Quick run command** | `cargo test --lib` |
| **Full suite command** | `cargo test` (add `--include-ignored` if the signal test is `#[ignore]`) |
| **Estimated runtime** | ~5–15s unit; integration adds a bounded subprocess clone |

---

## Sampling Rate

- **After every task commit:** `cargo test --lib`
- **After every plan wave:** `cargo test`
- **Before `/gsd-verify-work`:** full suite green + `cargo clippy` clean
- **Max feedback latency:** ~15s for unit; integration only at wave merge

---

## Per-Task Verification Map

| Req ID | Behavior | Test Type | Automated Command | File Exists |
|--------|----------|-----------|-------------------|-------------|
| CLEAN-02 | Sweep removes `rust-to-you-clone-*` older than 60 min; leaves fresh + non-matching | unit (pure `sweep_orphans` over fixture) | `cargo test --lib sweep_orphans` | ❌ W0 |
| CLEAN-02 | Sweep returns 0 (silent) when nothing stale | unit | `cargo test --lib sweep_orphans_empty` | ❌ W0 |
| CLEAN-02 | Sweep is best-effort — un-removable entry skipped, run not aborted | unit | `cargo test --lib sweep_best_effort` | ❌ W0 |
| CLEAN-03 | `cleanup_live_temp()` idempotent (2nd call no-ops, no error) | unit | `cargo test --lib cleanup_idempotent` | ❌ W0 |
| CLEAN-03 | Panic hook cleans the live temp (hook→cleanup path) | unit (`catch_unwind` + assert dir gone + slot None) | `cargo test --lib panic_cleans_temp` | ❌ W0 |
| CLEAN-01 | SIGINT mid-run leaves no orphan + exits 130 | integration (subprocess) | `cargo test --test interrupt sigint_cleans` | ❌ W0 |
| CLEAN-01 | SIGTERM mid-run leaves no orphan (exit 130 per D-03) | integration (subprocess) | `cargo test --test interrupt sigterm_cleans` | ❌ W0 |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] New hygiene module with inline `#[cfg(test)]` — pure `sweep_orphans(dir, now, max_age) -> usize` + idempotent `cleanup_live_temp()` + panic-hook cleanup test (CLEAN-02, CLEAN-03).
- [ ] `tests/interrupt.rs` — subprocess SIGINT/SIGTERM integration test (CLEAN-01). Spawns the binary, polls temp until a `rust-to-you-clone-*` dir appears, sends the signal, asserts no orphan remains + exit status 130.
- [ ] Decide mtime fixture approach: **prefer passing a synthetic `now` into the pure `sweep_orphans`** (no dev-dep) over adding `filetime`.
- [ ] Dev-dependency for sending the signal in integration test: `nix` or `libc` (`kill(pid, SIGINT)`), or shell out to `kill`.
- [ ] Framework install: none — Rust test harness built in.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Real interactive Ctrl-C leaves no temp + prints the bilingual goodbye | CLEAN-01 | True terminal SIGINT + Ferris message is best eyeballed once | Run `rust-to-you torvalds/linux --deep`, press Ctrl-C mid-clone; confirm bilingual "cleaning up, bye" line, `$?` == 130, and no `rust-to-you-clone-*` left in `$TMPDIR` |
| Startup sweep notice fires only when something was swept | CLEAN-02 | Requires pre-seeding a stale temp dir | `mkdir $TMPDIR/rust-to-you-clone-stale`, backdate its mtime >60 min, run the tool, confirm the one-line bilingual sweep notice appears and the dir is gone; run again with nothing stale → no notice |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references (incl. `tests/interrupt.rs`)
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s for unit tier
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
