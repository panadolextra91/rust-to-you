---
phase: 4
slug: presentation-layer
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-06-03
---

# Phase 4 — Validation Strategy

> Per-phase validation contract. Derived from 04-RESEARCH.md § "Validation Architecture".

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` (matches Phases 1-3) |
| **Config file** | none (Cargo built-in) |
| **Quick run command** | `cargo test --lib tui::` |
| **Full suite command** | `cargo test` |
| **Estimated runtime** | ~5s (pure helpers + ratatui `TestBackend`, no terminal/network) |

---

## Sampling Rate

- **After every task commit:** `cargo test --lib tui::` (+ `cargo clippy`)
- **After every plan wave:** `cargo test` (full, includes Phase 1-3 regressions)
- **Before `/gsd-verify-work`:** full suite green + clippy clean + one manual TTY run (all scroll keys, resize to ~40 cols, and `rust-to-you <repo> | less`)
- **Max feedback latency:** ~5s

---

## Per-Task Verification Map

| Behavior | Req | Test Type | Automated Command | File | Status |
|----------|-----|-----------|-------------------|------|--------|
| ASCII bar fill for fraction/width | PRES-02 | unit (pure) | `cargo test --lib tui::format` | tui/format.rs | ⬜ |
| thousands sep (12442→"12,442"), None→"—" | PRES-02 | unit (pure) | `cargo test --lib tui::format` | tui/format.rs | ⬜ |
| relative_date with injected `now` (days/months/years) | PRES-02 | unit (pure) | `cargo test --lib tui::format` | tui/format.rs | ⬜ |
| Section abstraction renders header + body | PRES-02 | unit | `cargo test --lib tui::section` | tui/section.rs | ⬜ |
| Header has repo identity, date, Case ID | PRES-01 | unit (TestBackend buffer substring) | `cargo test --lib tui::report` | tui/report.rs | ⬜ |
| All 6 sections appear in fixed order | PRES-01 | unit (TestBackend row order) | `cargo test --lib tui::report` | tui/report.rs | ⬜ |
| Bar alignment under emoji-bearing rows | PRES-02 | unit (TestBackend cells) | `cargo test --lib tui::report` | tui/report.rs | ⬜ |
| Scroll clamps to max_scroll; G→max, g→0 | PRES-01 | unit (pure key handler over TuiState) | `cargo test --lib tui::app` | tui/app.rs | ⬜ |
| Non-TTY plain renderer emits section labels | D-06 | unit (capture Write into Vec<u8>) | `cargo test --lib tui::plain` | tui/plain.rs | ⬜ |

*Status: ⬜ pending · ✅ green · ❌ red*

---

## Wave 0 Requirements

- [ ] `cargo add ratatui@0.30 crossterm@0.29` (only new deps; thousands-sep + relative-date hand-rolled from chrono)
- [ ] `src/tui/format.rs` — pure helpers + tests (bar, thousands sep, dash_or, relative_date w/ injected now)
- [ ] `src/tui/section.rs` — reusable Section abstraction (D-01)
- [ ] `src/tui/report.rs` + tests — `build_report_lines(&FactualSections, &session)` + TestBackend smoke tests
- [ ] `src/tui/app.rs` + tests — `TuiState`, `max_scroll`, pure key handler
- [ ] `src/tui/plain.rs` + tests — non-TTY renderer (writes to a generic `impl Write`)
- [ ] `src/tui/mod.rs` — `render()` TTY/non-TTY branch via `IsTerminal`; `pub mod tui;` in lib.rs
- [ ] Wire `app::run` to call `tui::render` (replace inline println! block)
- [ ] No framework install — built-in harness

---

## Manual-Only Verifications

| Behavior | Req | Why Manual | Test Instructions |
|----------|-----|------------|-------------------|
| Interactive scroll feel + all keys | PRES-01 | Real keyboard input can't be asserted headlessly | Run `rust-to-you octocat/Hello-World`; exercise ↓↑ j/k PgUp/PgDn g/G q/Esc |
| Narrow-terminal wrap + resize | PRES-02 | Real terminal rendering/resize | Resize window to ~40 cols; confirm no clipped/garbled layout |
| Color + emoji + non-TTY plain | PRES-02/D-06 | Real terminal colors / piping | Confirm color+emoji in a TTY; `rust-to-you <repo> \| less` shows plain text |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] No 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 10s
- [ ] `nyquist_compliant: true` set (planner finalizes)

**Approval:** pending
