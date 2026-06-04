---
phase: 04-presentation-layer
verified: 2026-06-03T00:00:00Z
status: passed
score: 3/3 must-haves verified
---

# Phase 4: Presentation Layer Verification Report

**Phase Goal:** Render the investigation as a single scrollable ratatui TUI report (header + case metadata + the 6 factual sections), bilingual VI+EN with the Ferris voice, plus a non-TTY plain fallback.
**Verified:** 2026-06-03 (manual; implementation by Antigravity; one post-review refinement round)
**Status:** passed

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Single scrollable vertical report: header + Case ID + sections in fixed order | ✓ VERIFIED | `report.rs` TestBackend test asserts subtitle + Case ID + 6 section titles in monotonic order; single `Paragraph` + scroll |
| 2 | Cute/readable: crab + emoji, bilingual VI+EN, Ferris voice | ✓ VERIFIED | e2e plain output: bilingual header/labels/narrative, `🌱/☠️/🔥/🏺/🌿/⚙️` section titles, Ferris voice, no "mình" |
| 3 | Usable on common widths + clean scroll state | ✓ VERIFIED | `Paragraph.wrap` + `max_scroll` clamp (unit-tested); headless scroll-contract test; interactive verified manually |

**Score:** 3/3 truths verified

### Required Artifacts

| Artifact | Status | Details |
|----------|--------|---------|
| `src/i18n.rs` | ✓ | `Bilingual{vi,en}` + `bi` + `two_line` + `inline_label`, pure + tested; shared by TUI + plain |
| `src/tui/format.rs` | ✓ | `ascii_bar`, `thousands`, `dash_or`, `relative_date` (now with >2yr absolute "%b %Y" fallback) |
| `src/tui/section.rs` | ✓ | reusable Section abstraction (D-01) |
| `src/tui/report.rs` | ✓ | `build_report_lines` — header + 6 factual sections, bilingual; TestBackend tests incl. negative 7-9 assert |
| `src/tui/app.rs` | ✓ | `TuiState`/`max_scroll`/pure `handle_key`; `ratatui::run` panic-safe loop; headless scroll-contract test |
| `src/tui/plain.rs` | ✓ | non-TTY bilingual renderer over `impl Write` |
| `src/tui/mod.rs` | ✓ | `render()` `IsTerminal` TTY/plain branch; `now_secs` threaded in |
| `src/error.rs` (retrofit) | ✓ | all 8 `IntakeError` variants bilingual VI\nEN, Ferris voice, no "mình" |
| `src/app/run.rs`, `src/app/collect.rs` | ✓ | bilingual status/degrade; single `now_secs` computed once and threaded |

### Key Link Verification

| From | To | Status | Details |
|------|----|--------|---------|
| run → tui::render | TUI / plain | ✓ WIRED | `IsTerminal` branch; one `now_secs` passed to build_factual_sections AND render |
| renderers → i18n | Bilingual | ✓ WIRED | both TUI and plain use the same `two_line`/`inline_label` |
| main → IntakeError Display | stderr | ✓ WIRED | bilingual two-line errors (verified e2e: gitlab → VI + EN, exit 2) |

## Requirements Coverage

| Requirement | Status |
|-------------|--------|
| PRES-01 (single scrollable report, header/identity/date/Case ID, sections in order) | ✓ SATISFIED (9-section completion finishes in Phase 5 per D-01) |
| PRES-02 (cute/readable, crab iconography, no tabs/multi-screen) | ✓ SATISFIED |

**Coverage:** 2/2 requirements satisfied

## Quality Gates

- `cargo build` → ✓ (ratatui 0.30 + crossterm 0.29 link; no tokio)
- `cargo test` → ✓ **43/43 passed**
- `cargo clippy` → ✓ **0 warnings**
- e2e plain (`octocat/Hello-World | cat`) → ✓ bilingual report, thousands separators, "May 2018" absolute date
- e2e error (`gitlab.com/x/y`) → ✓ bilingual stderr, exit 2

## Decision Compliance (CONTEXT D-01..D-08)

| Decision | Verdict |
|----------|---------|
| D-01 reusable Section abstraction; 6 sections; NO 7-9 placeholders | ✓ negative TestBackend assert |
| D-02 panic-safe `ratatui::run` (no hand-rolled lifecycle) | ✓ |
| D-03 keymap ↓↑/j/k/PgDn/PgUp/g/G/q/Esc; single-Paragraph scroll | ✓ pure key handler + max_scroll tested |
| D-04 ASCII bars / relative dates / missing→"—" / thousands | ✓ pure helpers |
| D-05 color + expressive emoji (🦀😭☠️🔥👑…) | ✓ |
| D-06 wrap narrow; non-TTY → plain text | ✓ `IsTerminal` branch |
| D-07 header case-file identity (crab/title/repo/date/Case ID) | ✓ |
| D-08 bilingual VI+EN + Ferris voice; i18n helper; retrofit Phase 1-3 | ✓ all 8 errors bilingual, report bilingual, shared helper |

## Post-Review Refinements Applied (this round)

1. `relative_date` absolute-date fallback for >2-year ages (`"%b %Y"`) — tested; e2e shows "May 2018".
2. `now_secs` computed ONCE in `run.rs` and threaded into both `build_factual_sections` and `tui::render` (consistent now across stale-branch math and displayed dates).
3. Headless `test_headless_scroll_contract` (TestBackend, scroll((0,0)) vs ((1,0)), `assert_ne!`).

## Human Verification Required

Interactive scroll/keys + real-terminal color/resize — verified manually (run `rust-to-you <repo>` in a TTY). All other behavior verified programmatically + via the non-TTY plain e2e.

## Gaps Summary

**No gaps found.** Phase goal achieved. The 6 factual sections render bilingually in a scrollable TUI with a plain fallback; Sections 7-9 (Vibes/Findings/Verdict) land in Phase 5 via the same Section abstraction. Ready for Phase 5.

---

*Phase: 04-presentation-layer*
*Verified: 2026-06-03 (manual, post-refinement)*
