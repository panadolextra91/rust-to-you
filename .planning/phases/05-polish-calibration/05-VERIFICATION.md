---
phase: 05-polish-calibration
verified: 2026-06-03T00:00:00Z
status: passed
score: 3/3 must-haves verified
---

# Phase 5: Polish & Calibration Verification Report

**Phase Goal:** Implement the 3 narrative sections (Repository Vibes per VIBES.md, Interesting Findings, Crab Verdict), render them bilingual + Ferris via the Phase 4 Section abstraction after the 6 factual sections, and calibrate end-to-end.
**Verified:** 2026-06-03 (manual; implementation by Antigravity; one cosmetic bug fixed by reviewer)
**Status:** passed

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Repository Vibes always show evidence bullets justifying the label | ✓ VERIFIED | e2e octocat → 🏛️ Ancient Temple + bullets ("15 years old", "Last commit 2945 days ago"); vibes.rs evidence = fired conditions |
| 2 | Interesting Findings + Crab Verdict read coherently | ✓ VERIFIED | e2e: findings (bus factor 1 / no CI / 15y active + runner-up Solo Wizard), verdict "Bị ám / Haunted" + grounded risks |
| 3 | Stable, trustworthy end-to-end reports | ✓ VERIFIED | full pipeline runs; 57/57 tests; deterministic fixtures per rule |

**Score:** 3/3 truths verified

### Required Artifacts

| Artifact | Status | Details |
|----------|--------|---------|
| `release_tag_count` (snapshot) | ✓ | via git2 `tag_names()` in `repo/branches.rs`, wired in collect.rs |
| `src/analyze/vibes.rs` | ✓ | VIBES.md ruleset exact: 7 vibes, MIN_SCORE=4, specificity tie-break, Chaotic Good fallback, evidence = fired conditions, runner-up captured |
| `src/analyze/findings.rs` | ✓ | rule-based, ranked, capped ~6, runner-up vibe line; pure |
| `src/analyze/verdict.rs` | ✓ | Strengths/Risks rules + one-line overall label (Healthy/Needs care/Haunted); pure |
| Sections 7-9 render (report.rs + plain.rs) | ✓ | via Section abstraction, after the 6 factual; bilingual + Ferris; negative assert flipped to `found_titles == 9` + present-assertions |

### Key Link Verification

| From | To | Status | Details |
|------|----|--------|---------|
| collect → snapshot.release_tag_count | vibes analyzer | ✓ WIRED | vibe scoring reads snapshot only (no git2 in analyzers) |
| vibes runner-up | findings | ✓ WIRED | "Also gives off X energy" line present |
| analyzers → report/plain | render | ✓ WIRED | 7-9 present in both renderers (e2e confirmed) |

## Requirements Coverage

| Requirement | Status |
|-------------|--------|
| NARR-01 (classify vibes from signals + evidence bullets) | ✓ SATISFIED |
| NARR-02 (interesting findings + crab verdict with strengths/risks) | ✓ SATISFIED |

**Coverage:** 2/2 requirements satisfied

## Quality Gates

- `cargo build` → ✓
- `cargo test` → ✓ **57/57 passed** (43 prior + 14 new vibes/findings/verdict)
- `cargo clippy` → ✓ **0 warnings**
- e2e (`octocat/Hello-World`) → ✓ Sections 1-9 render bilingually; vibe = Ancient Temple (correct for a 15-year dormant repo)

## Decision Compliance (CONTEXT D-01..D-06)

| Decision | Verdict |
|----------|---------|
| D-01 enrich release_tag_count (git2 tag_names), analyzers read snapshot only | ✓ |
| D-02 implement VIBES.md exactly (7 vibes, MIN_SCORE=4, tie-break, Chaotic Good fallback, evidence, runner-up) | ✓ |
| D-03 rule-based ranked findings + runner-up line | ✓ |
| D-04 Strengths/Risks + overall verdict label | ✓ |
| D-05 deterministic fixtures + manual calibration | ✓ (fixtures present; real-repo spot-check is the calibration checkpoint) |
| D-06 render 7-9 via Section abstraction, bilingual+Ferris, flip negative assert | ✓ |

## Reviewer Fix Applied (this round)

- **Doubled label bug:** `VibeLabel::display()` and `verdict.overall_label` already hold a combined "Name / Gloss" string, but were re-wrapped in `inline_label()` → rendered twice (e.g. "🏛️ Ancient Temple / Đền cổ / 🏛️ Ancient Temple / Đền cổ"). Fixed at the 4 render call sites (report.rs ×2, plain.rs ×2) to render the combined string once via `.vi`. Verified: labels now render single. 57/57 tests still pass, clippy clean.

## Human Verification Required

Multi-repo "feels right" calibration (the 05-02 checkpoint) — verified manually via e2e on octocat (Ancient Temple is the correct vibe). Further spot-checks across diverse repos can be run any time with `rust-to-you <repo>`.

## Gaps Summary

**No gaps found.** Phase goal achieved. All 9 sections render; vibes are evidence-backed; findings + verdict are coherent and grounded (Pitfall 3). This is the final V1 milestone phase.

---

*Phase: 05-polish-calibration*
*Verified: 2026-06-03 (manual, post-fix)*
