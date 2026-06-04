# Phase 3: Analysis Layer - Discussion Log

> **Audit trail only.** Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-03
**Phase:** 3-Analysis Layer
**Areas discussed:** Ancient Relics sourcing, Stale-branch threshold, Commit Crimes gaps, View-model boundary

---

## Ancient Relics + missing data sourcing

| Option | Description | Selected |
|--------|-------------|----------|
| Enrich collectors + cheap defs | Add snapshot fields, extend Phase 2 walks; oldest file = first-commit∩HEAD, oldest contributor = earliest first commit, longest-living branch = oldest tip | ✓ |
| Keep workspace alive | Pass cloned repo into analyzers | |
| Cut from V1 | Drop oldest-file/contributor/branch | |

**User's choice:** Enrich collectors + cheap definitions

---

## Stale-branch threshold

| Option | Description | Selected |
|--------|-------------|----------|
| 90 days | tip > 90d = stale | ✓ |
| 180 days | more lenient | |
| 365 days | very lenient | |

**User's choice:** 90 days

---

## Commit Crimes gaps

| Option | Description | Selected |
|--------|-------------|----------|
| Rolling 30 days | commits in last 30d; top contributor = mailmap author name | ✓ |
| Calendar month | commits since 1st of month | |

**User's choice:** Rolling 30 days + git author display name (mailmap-resolved)

---

## View-model boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Pure data structs | Phase 3 = data only; ASCII bars / relative dates / crab labels → Phase 4 | ✓ |
| Analyzer pre-formats | Phase 3 builds bar strings + relative dates | |

**User's choice:** Pure data structs (formatting deferred to Phase 4)

---

## Claude's Discretion
- "Last activity" = most recent branch tip (git, always present); API `pushed_at` fallback only.
- Per-section struct shapes — planner's call (pure data).
- Module placement (analyze/ vs report/) — planner's call.

## Deferred Ideas
- True per-file history (precise oldest-file, all-history most-modified) → --deep/v2.
- Calendar-month commits → not chosen.
- Pre-formatted strings → Phase 4.
- Vibes/Findings/Verdict (Sections 7-9) → Phase 5.
