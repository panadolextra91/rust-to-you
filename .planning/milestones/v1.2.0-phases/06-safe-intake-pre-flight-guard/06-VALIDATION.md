---
phase: 6
slug: safe-intake-pre-flight-guard
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-06-04
---

# Phase 6 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Built-in Rust test harness (`#[cfg(test)]` + `#[test]`) |
| **Config file** | none — Cargo convention |
| **Quick run command** | `cargo test --lib` |
| **Full suite command** | `cargo test` |
| **Estimated runtime** | ~5–15 seconds (no network in unit tests) |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --lib`
- **After every plan wave:** Run `cargo test`
- **Before `/gsd-verify-work`:** `cargo test` green + `cargo clippy` clean
- **Max feedback latency:** ~15 seconds

---

## Per-Task Verification Map

| Req ID | Behavior | Wave | Test Type | Automated Command | File Exists |
|--------|----------|------|-----------|-------------------|-------------|
| GUARD-01 | Repo > 500 MB without `--deep` → `RepoTooLarge`, clone not started | 0→1 | unit (mocked meta) | `cargo test --lib app::collect` | ❌ W0 (collect.rs has no tests) |
| GUARD-01 edge | `Unavailable` size → warn + proceed (fail-open) | 0→1 | unit | `cargo test --lib app::collect` | ❌ W0 |
| GUARD-02 | `RepoTooLarge` message contains actual MB + names `--deep`, bilingual VI+EN | 1 | unit | `cargo test --lib error` | ❌ W0 |
| GUARD-03 | `--deep` on large repo → warning printed, falls through to clone | 1 | unit | `cargo test --lib app::collect` | ❌ W0 |
| GUARD-03 | `--deep` flag parses on `Args` | 1 | unit | `cargo test --lib cli::args` | ❌ W0 (Args has no tests) |
| SEC-01 | leading-dash owner/repo rejected with distinct `UnsafeInput` | 1 | unit | `cargo test --lib cli::parse` | ✅ extend reject table |
| SEC-01 | owner > 39 / repo > 100 rejected | 1 | unit | `cargo test --lib cli::parse` | ✅ extend reject table |
| SEC-01 | existing guards (`..`, `\`, control, host) still pass | 1 | unit | `cargo test --lib cli::parse` | ✅ existing tests |
| SEC-02 | injection/abuse table proves rejection at parser before fetch/clone | 1 | unit | `cargo test --lib cli::parse` | ✅ extend |
| (decode) | `RepoMetadata` decodes `size` (KB) | 1 | unit | `cargo test --lib github::client` | ✅ extend existing `decode()` |
| exit codes | `RepoTooLarge` → 6; `UnsafeInput` → 2 | 1 | unit | `cargo test --lib error` | ❌ W0 (add exit_code assertions) |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `src/app/collect.rs` `#[cfg(test)]` module — size-guard branches (GUARD-01/03 + fail-open). **Recommended enabler:** extract a pure `fn size_decision(meta_state, deep) -> Decision` so GUARD-01/03 are unit-testable without a live clone or network.
- [ ] `src/cli/args.rs` `#[cfg(test)]` — assert `Args::parse_from(["bin","o/r","--deep"]).deep == true`.
- [ ] `src/error.rs` `#[cfg(test)]` — assert `RepoTooLarge.exit_code() == 6`, `UnsafeInput.exit_code() == 2`, and `to_string()` contains both VI and EN lines + the MB value + `--deep`.
- [ ] Extend `src/cli/parse.rs` reject table (leading-dash, over-length) — file exists.
- [ ] Extend `src/github/client.rs` `decode()` fixtures with `"size"` — file exists.
- [ ] Framework install: none — Rust test harness is built in.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| End-to-end refusal against a real >500 MB public repo | GUARD-01/02 | Requires live GitHub API + a known-large repo; not run in CI to keep unit tests network-free | Run `rust-to-you torvalds/linux` (no `--deep`) → expect exit 6 + bilingual too-large message showing actual MB; then re-run with `--deep` → expect bilingual warning then clone proceeds |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
