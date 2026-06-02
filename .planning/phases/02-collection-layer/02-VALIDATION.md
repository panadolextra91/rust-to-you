---
phase: 2
slug: collection-layer
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-06-02
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Derived from 02-RESEARCH.md § "Validation Architecture".

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` / `cargo test` (no external crate; matches Phase 1) |
| **Config file** | none — Cargo convention; unit tests in `#[cfg(test)] mod tests` per file |
| **Quick run command** | `cargo test --lib` |
| **Full suite command** | `cargo test` |
| **Estimated runtime** | ~5 seconds (fixture git repos built in `TempDir`, no network) |

---

## Sampling Rate

- **After every task commit:** `cargo test --lib` (+ `cargo clippy`)
- **After every plan wave:** `cargo test` (full)
- **Before `/gsd-verify-work`:** full suite green + `cargo clippy` clean + a manual end-to-end run of the 02-01 skeleton against a small public repo (e.g. `octocat/Hello-World`)
- **Max feedback latency:** ~5 seconds

---

## Per-Task Verification Map

| Task | Plan | Wave | Requirement | Threat Ref | Test Type | Automated Command | File Exists | Status |
|------|------|------|-------------|------------|-----------|-------------------|-------------|--------|
| repo age (skeleton metric) | 02-01 | 1 | COLL-02 | — | unit (fixture repo) | `cargo test --lib repo::history::tests::repo_age` | ❌ W0 | ⬜ pending |
| GitHub status→IntakeError mapping | 02-02 | — | COLL-01 | T-02 (net) | unit (pure fn over StatusCode, NO network) | `cargo test --lib github::tests::status_mapping` | ❌ W0 | ⬜ pending |
| serde decode of `/repos` JSON | 02-02 | — | COLL-01 | — | unit (static JSON fixture) | `cargo test --lib github::tests::decode` | ❌ W0 | ⬜ pending |
| total-commit count (full history) | 02-03 | — | COLL-02 | — | unit (fixture repo) | `cargo test --lib repo::history::tests::total_commits` | ❌ W0 | ⬜ pending |
| branch enumeration (ALL incl. remote refs) | 02-03 | — | COLL-02 | — | unit (clone fixture via file://) | `cargo test --lib repo::history::tests::branch_count` | ❌ W0 | ⬜ pending |
| contributors + bus_factor (bot/merge/identity filter) | 02-03 | — | COLL-01 | — | unit (fixture: bot + dup-email + merge) | `cargo test --lib repo::history::tests::bus_factor` | ❌ W0 | ⬜ pending |
| most-modified-file bounded + capped flag | 02-03 | — | COLL-02 | — | unit (fixture > N commits) | `cargo test --lib repo::history::tests::most_modified` | ❌ W0 | ⬜ pending |
| tokei language percentages (~100%) | 02-04 | — | COLL-03 | — | unit (TempDir known files) | `cargo test --lib scan::lang::tests::percentages` | ❌ W0 | ⬜ pending |
| infra footprint detection (per signal) | 02-04 | — | COLL-03 | T-traversal | unit (TempDir marker files) | `cargo test --lib scan::infra::tests::detect` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `repo/history.rs` test module + `fn make_fixture_repo() -> TempDir` helper — builds commits with controlled author name/email/`Time`, a merge commit, a `[bot]` author, duplicate-email authors, and a `.mailmap` (shared fixture for age/count/bus-factor/most-modified)
- [ ] `repo/history.rs` clone-topology test — `Repository::clone` from a local `file://` path to a second `TempDir` to exercise remote-ref branch enumeration (no network)
- [ ] `scan/lang.rs` + `scan/infra.rs` test modules with `TempDir` fixtures
- [ ] `github/` test module — static `/repos` JSON sample string + a **pure** `classify(StatusCode) -> Result<_, IntakeError>` fn testable without network
- [ ] No framework install needed — Rust built-in test harness (as in Phase 1)

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Real `fetch_metadata` HTTP round-trip | COLL-01 | Needs live network; keep out of default suite | Either inject a fake transport (preferred) or gate a real-network test behind `#[ignore]` (`cargo test -- --ignored`) |
| Real remote clone (walking skeleton e2e) | COLL-02 | Needs network + a real repo | Run the binary against a tiny public repo (e.g. `octocat/Hello-World`); confirm it clones + prints repo age |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 10s
- [ ] `nyquist_compliant: true` set in frontmatter (planner finalizes)

**Approval:** pending
