# Phase 5: Polish & Calibration - Discussion Log

> **Audit trail only.** Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-03
**Phase:** 5-Polish & Calibration
**Areas discussed:** Vibe inputs / missing data, Interesting Findings, Crab Verdict, Calibration

---

## Vibe inputs / missing data

| Option | Description | Selected |
|--------|-------------|----------|
| Enrich collector for git tags | add release_tag_count via git2 tag_names() | ✓ |
| Drop tag-dependent conditions | weaken vibe rules | |
| Tags via GitHub API | inconsistent with git-only | |

**User's choice:** Enrich collector to count git tags (release_tag_count)

---

## Interesting Findings (Section 8)

| Option | Description | Selected |
|--------|-------------|----------|
| Rule-based + rank + cap ~4-6 + runner-up vibe | threshold rules, interestingness ranking, bilingual | ✓ |
| Fixed set, no ranking | always-same handful | |
| Runner-up + 1-2 findings | minimal | |

**User's choice:** Rule-based + rank + cap ~4-6 + runner-up vibe line

---

## Crab Verdict (Section 9)

| Option | Description | Selected |
|--------|-------------|----------|
| Strengths/Risks rules + 1-line overall verdict | ✓/⚠ from signals + health label | ✓ |
| Strengths/Risks only | no overall label | |
| Free-form summary | unstructured | |

**User's choice:** Strengths/Risks rules + one-line overall verdict (bilingual + Ferris)

---

## Calibration (05-02)

| Option | Description | Selected |
|--------|-------------|----------|
| Fixture deterministic + spot-check ~3-4 real repos | unit-test rules + manual sanity | ✓ |
| Fixture only | no real-repo pass | |
| Manual only | no auto tests | |

**User's choice:** Deterministic fixture tests + manual spot-check on diverse real repos; tune VIBES thresholds, write changes back to VIBES.md

---

## Claude's Discretion
- EN copy + VI gloss for each vibe name (keep iconic name recognizable).
- Finding rules + interestingness weights.
- Overall verdict-label thresholds.
- Module placement (analyze/vibes.rs, findings.rs, verdict.rs).

## Deferred Ideas
- Blame-based truck factor / --deep → v2.
- --json output → v2.
- More vibe categories → out of scope (7 fixed).
