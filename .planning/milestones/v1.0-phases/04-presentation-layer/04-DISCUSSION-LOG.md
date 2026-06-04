# Phase 4: Presentation Layer - Discussion Log

> **Audit trail only.** Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-03
**Phase:** 4-Presentation Layer
**Areas discussed:** Sections 7-9 handling, Scroll/keys/screen mode, Formatting conventions, Visual style + resilience

---

## Sections 7-9 in Phase 4

| Option | Description | Selected |
|--------|-------------|----------|
| 6 + reusable abstraction; 7-9 in Phase 5 | Build section renderer, render 6 factual; Phase 5 adds 7-9 (logic+render) | ✓ |
| 6 + placeholder stubs | "coming..." lines for 7-9 now | |
| Scaffold 9 empty widgets | empty 7-9 widgets | |

**User's choice:** 6 sections + reusable abstraction; 7-9 deferred to Phase 5 (no placeholders)

---

## Scroll + keys + screen mode

| Option | Description | Selected |
|--------|-------------|----------|
| Alt-screen + full keymap | full-screen, ↓↑/j/k/PgUp/PgDn/g/G/q-Esc, restore on exit | ✓ |
| Inline (no alt-screen) | terminal scrollback | |
| Arrows + q minimal | ↓↑ + q only | |

**User's choice:** Alternate screen + full keymap

---

## Formatting conventions

| Option | Description | Selected |
|--------|-------------|----------|
| Full set | ASCII bars, relative dates, missing→—, thousands separators | ✓ |
| Bars + relative date, no comma | numbers raw | |
| Minimal | raw numbers, ISO dates, no bars | |

**User's choice:** Full set

---

## Visual style + resilience

| Option | Description | Selected |
|--------|-------------|----------|
| Color+crab, wrap narrow, plain on non-TTY | + EXPRESSIVE emoji 🦀😭😋 per user | ✓ |
| Color+crab, error on non-TTY | require TTY | |
| Monochrome minimal | no color | |

**User's choice:** Option 1 + explicit request for expressive emoji (🦀, 😭, 😋, and friends) for personality

---

## Claude's Discretion
- Per-section colors + emoji choices (keep readable).
- Event-loop tick rate / input handling.
- TUI testing = pure format helpers (unit) + ratatui TestBackend smoke test; interactive loop manual.
- run.rs plain printer → non-TTY fallback (reuse or rewrite).
- src/tui/ module placement.

## Deferred Ideas
- Sections 7-9 rendering → Phase 5.
- Mouse-wheel scroll → optional/if cheap.
- --json export → v2.
