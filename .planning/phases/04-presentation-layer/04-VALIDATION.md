---
phase: 4
slug: presentation-layer
status: planned
nyquist_compliant: true
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

| Behavior | Req | Test Type | Automated Command | File | Plan/Task | Status |
|----------|-----|-----------|-------------------|------|-----------|--------|
| ASCII bar fill for fraction/width | PRES-02 | unit (pure) | `cargo test --lib tui::format` | tui/format.rs | 04-01 T1 | ⬜ |
| thousands sep (12442→"12,442"), None→"—" | PRES-02 | unit (pure) | `cargo test --lib tui::format` | tui/format.rs | 04-01 T1 | ⬜ |
| relative_date with injected `now` (days/months/years) | PRES-02 | unit (pure) | `cargo test --lib tui::format` | tui/format.rs | 04-01 T1 | ⬜ |
| Section abstraction renders header + body | PRES-02 | unit | `cargo test --lib tui::section` | tui/section.rs | 04-01 T2 | ⬜ |
| Header has repo identity, date, Case ID | PRES-01 | unit (TestBackend buffer substring) | `cargo test --lib tui::report` | tui/report.rs | 04-01 T2 | ⬜ |
| All 6 sections appear in fixed order | PRES-01 | unit (TestBackend row order) | `cargo test --lib tui::report` | tui/report.rs | 04-01 T2 | ⬜ |
| Bar alignment under emoji-bearing rows | PRES-02 | unit (TestBackend cells) | `cargo test --lib tui::report` | tui/report.rs | 04-01 T2 | ⬜ |
| Sections 7-9 NOT rendered (no placeholder) | PRES-01/D-01 | unit (TestBackend negative assert) | `cargo test --lib tui::report` | tui/report.rs | 04-01 T2 | ⬜ |
| Non-TTY plain renderer emits section labels | D-06 | unit (capture Write into Vec<u8>) | `cargo test --lib tui::plain` | tui/plain.rs | 04-01 T3 | ⬜ |
| Scroll clamps to max_scroll; G→max, g→0 | PRES-01 | unit (pure key handler over TuiState) | `cargo test --lib tui::app` | tui/app.rs | 04-02 T1 | ⬜ |
| render_tui compiles (panic-safe loop, wrap, Press filter) | PRES-01/D-02 | build | `cargo build --lib` | tui/app.rs | 04-02 T2 | ⬜ |
| IsTerminal branch + run wiring (no regressions) | D-06 | build + full suite | `cargo test` | tui/mod.rs, app/run.rs | 04-02 T3 | ⬜ |

*Status: ⬜ pending · ✅ green · ❌ red*

---

## Wave 0 Requirements

- [ ] `cargo add ratatui@0.30 crossterm@0.29` (only new deps; thousands-sep + relative-date hand-rolled from chrono) — **verified via `--dry-run`: ratatui 0.30.0 / crossterm 0.29.0 resolve cleanly**
- [ ] `src/tui/format.rs` — pure helpers + tests (bar, thousands sep, dash_or, relative_date w/ injected now) — 04-01 T1
- [ ] `src/tui/section.rs` — reusable Section abstraction (D-01) — 04-01 T2
- [ ] `src/tui/report.rs` + tests — `build_report_lines(&session, &FactualSections)` + TestBackend smoke tests — 04-01 T2
- [ ] `src/tui/plain.rs` + tests — non-TTY renderer (writes to a generic `impl Write`) — 04-01 T3
- [ ] `src/tui/app.rs` + tests — `TuiState`, `max_scroll`, pure key handler — 04-02 T1; `render_tui` loop — 04-02 T2
- [ ] `src/tui/mod.rs` — `render()` TTY/non-TTY branch via `IsTerminal`; `pub mod tui;` in lib.rs — 04-01 T1 (module) + 04-02 T3 (branch)
- [ ] Wire `app::run` to call `tui::render` (replace inline println! block) — 04-02 T3
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

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] No 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 10s
- [x] `nyquist_compliant: true` set (planner finalizes)

**Approval:** approved (planner, 2026-06-03)
