# Phase 5: Polish & Calibration - Context

**Gathered:** 2026-06-03
**Status:** Ready for planning

<domain>
## Phase Boundary

Implement the three narrative sections — **Repository Vibes** (Section 7, per `research/VIBES.md`),
**Interesting Findings** (Section 8), **Crab Verdict** (Section 9) — render them via the Phase 4
`Section` abstraction (after the 6 factual sections), and calibrate the whole report end-to-end
against real repos. All text bilingual VI+EN with the Ferris voice (D-08).

**In scope:** enrich the snapshot with `release_tag_count`; the vibe classifier (VIBES.md
ruleset); rule-based findings + crab verdict; wire 7-9 into the TUI + plain renderers; deterministic
rule tests + manual real-repo calibration.

**Explicitly NOT in this phase:** new factual sections; `--json`/`--deep`/caching (v2); blame-based
truck factor (v2); adding new vibes (the 7 in VIBES.md are fixed).
</domain>

<decisions>
## Implementation Decisions

### Vibe Inputs — enrichment (like Phase 3)
- **D-01:** Enrich the collector to add **`release_tag_count`** to the snapshot via git2
  `tag_names()` (cheap, no network). All other VIBES signals are ALREADY in the snapshot /
  FactualSections: night/weekend/business %, bus_factor, contributor_count, top_author_share_pct,
  commits_this_month, branch total + stale count, infra flags, repo_age_days, days-since-last-commit
  (from last_activity). The classifier reads from the snapshot ONLY (workspace is dropped) — touching
  `src/repo/*` + `src/snapshot.rs` + `src/app/collect.rs` for the tag count is expected, not creep.

### Repository Vibes (Section 7) — implement research/VIBES.md EXACTLY
- **D-02:** Implement the locked VIBES.md ruleset: weighted scoring across the 7 vibes,
  **MIN_SCORE = 4**, tie-break by the specificity order, **Chaotic Good** fallback, **evidence
  bullets = the satisfied conditions**, single PRIMARY vibe shown + **runner-up** captured for
  Section 8. Vibe names keep their iconic English + emoji (e.g. `🧙 Solo Wizard`) AND get a VI gloss
  per D-08 (e.g. `🧙 Solo Wizard / Phù thủy đơn độc`). Evidence bullets are bilingual.

### Interesting Findings (Section 8)
- **D-03:** Rule-based finding generators — each rule fires on a threshold and emits a **bilingual
  observation** (e.g. most-modified-file very high, single-dev dominance, stale-branch pile,
  missing CI, bus factor 1, heavy night-owl commits). **Rank by interestingness/severity, cap the
  shown set at ~4-6.** Include the **runner-up vibe** as one finding line ("Also gives off
  {runner-up} energy / Cũng phảng phất chất {runner-up}"). Pure functions over snapshot/FactualSections.

### Crab Verdict (Section 9)
- **D-04:** Rule-based **Strengths (✓)** + **Risks (⚠)** derived from signals — strengths:
  recently active, CI present, healthy bus factor (≥3), distributed authorship, has releases; risks:
  bus factor 1, no CI, stale-branch pile, dormant, single-dev dominance. PLUS a **one-line overall
  verdict label** from the balance of strengths vs risks (e.g. `Khỏe mạnh / Healthy`,
  `Cần chăm sóc / Needs care`, `Bị ám / Haunted`). Bilingual + Ferris.

### Rendering (cross-cutting)
- **D-06:** Sections 7-9 render via the Phase 4 `Section` abstraction and appear **after** the 6
  factual sections (fixed report order 1→9), in BOTH `report.rs` (TUI) and `plain.rs`. All text
  bilingual + Ferris. **Update the Phase 4 negative TestBackend assert** (which currently asserts
  7-9 are ABSENT) — now they must be present.

### Calibration & Verification (05-02)
- **D-05:** **Deterministic unit tests** for every vibe/finding/verdict rule against synthetic
  snapshot/FactualSections fixtures (no network, like prior phases) — e.g. a fixture that scores
  Solo Wizard, one that falls through to Chaotic Good, etc. PLUS a **manual spot-check** on ~3-4
  diverse real repos (a solo project, a large OSS like `tokio-rs/tokio`, a dormant old repo, a
  corporate-style repo) to confirm labels "feel right". Tune the VIBES **V1 threshold values** by
  judgment if one category dominates, and **record any threshold change back into research/VIBES.md**.

### Claude's Discretion
- Exact EN copy + VI gloss for each vibe name (keep the iconic name recognizable).
- Exact finding rules + interestingness weights (within the spirit of VIBES/PITFALLS).
- Overall verdict-label thresholds.
- Module placement (e.g. `src/analyze/vibes.rs`, `findings.rs`, `verdict.rs`).

### Carried Forward (locked — do NOT re-litigate)
- research/VIBES.md ruleset (scoring, MIN_SCORE=4, 7 vibes, tie-break, Chaotic Good fallback,
  evidence bullets, runner-up→Section 8) · METRICS bot/merge/mailmap filtering · **D-08 bilingual
  VI+EN + Ferris voice** (applies to 7-9) · Phase 4 `Section` abstraction · pure-fn-over-snapshot +
  deterministic/fixture-testable · analyze → view-model → render separation.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope & requirements
- `.planning/ROADMAP.md` § "Phase 5: Polish & Calibration" (incl. the **Spec** line → research/VIBES.md), plans 05-01..05-02
- `.planning/REQUIREMENTS.md` — NARR-01 (vibes from signals + evidence bullets), NARR-02 (findings + crab verdict with strengths/risks)

### THE spec + metrics
- `.planning/research/VIBES.md` — **authoritative** vibe ruleset (signals, scoring, 7 vibes, MIN_SCORE=4, tie-break, Chaotic Good fallback, evidence templates, output contract incl. runner-up→Section 8, calibration notes)
- `.planning/research/METRICS.md` — shared bot/merge/mailmap filtering (already in the snapshot inputs)
- `.planning/research/PITFALLS.md` — Pitfall 3 (heuristic comedy MUST be backed by evidence; playful but fair)

### Cross-cutting voice
- `.planning/PROJECT.md` § Key Decisions — Ferris narrator + bilingual VI+EN (D-08 from Phase 4)

### Code to build on / enrich (Phase 1-4, on `main`)
- `src/snapshot.rs` + `src/repo/*` + `src/app/collect.rs` — add `release_tag_count` (git2 `tag_names()`)
- `src/report/sections.rs` + `src/analyze/*` — the FactualSections inputs (bus_factor, branches, %, infra, etc.)
- `src/tui/report.rs` + `src/tui/plain.rs` — extend with Sections 7-9 (update the negative 7-9 assert)
- `src/tui/section.rs` (the Section abstraction), `src/i18n.rs` (bilingual helper)
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets (Phases 1-4, on `main`)
- Snapshot/FactualSections already carry almost every VIBES signal (night/weekend/business %,
  bus_factor, contributor_count, top_author_share_pct, commits_this_month, branch total + stale,
  infra flags, repo_age_days). ONLY `release_tag_count` is missing → enrich.
- `src/tui/section.rs` `Section { title, body }.into_lines()` — render 7-9 the same way as 1-6.
- `src/i18n.rs` `bi`/`two_line`/`inline_label` — reuse for bilingual vibe/findings/verdict text.
- The fixture-test pattern (synthetic FactualSections built directly in tests, e.g. in
  `src/tui/report.rs` tests) — reuse for deterministic vibe/finding/verdict rule tests.

### Established Patterns
- Pure analyzers over snapshot (analyze/*.rs) → add vibes/findings/verdict analyzers the same way.
- `build_report_lines` / `plain::render` iterate sections — append 7-9 after section 6.

### Integration Points
- The Phase 4 negative TestBackend assertion ("no Vibes/Findings/Verdict") MUST be updated now
  that 7-9 exist.
- Vibe classifier output includes a runner-up that flows into the Findings section.
</code_context>

<specifics>
## Specific Ideas

- VIBES.md is the contract — follow its scoring, thresholds, evidence templates, and tie-break
  order verbatim; only deviate via the calibration step (and write changes back to VIBES.md).
- Findings + verdict must stay **grounded** (Pitfall 3): every line traces to a real signal;
  playful but fair, never mean.
- Keep the report's gossip energy: Ferris narrates the verdict ("Theo Ferris... / In Ferris's
  opinion..."), bilingual, with the expressive emoji established in Phase 4.
</specifics>

<deferred>
## Deferred Ideas

- Blame-based truck factor / `--deep` deeper analysis → v2.
- `--json` machine output → v2.
- Additional vibe categories beyond the 7 → out of scope (VIBES.md set is fixed for V1).

None of the above is scope creep into Phase 5 — they are intentional boundaries.
</deferred>

---

*Phase: 5-Polish & Calibration*
*Context gathered: 2026-06-03*
