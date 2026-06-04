# Phase 3: Analysis Layer - Context

**Gathered:** 2026-06-03
**Status:** Ready for planning

<domain>
## Phase Boundary

Turn the `InvestigationSnapshot` into **pure-data view models for the 6 FACTUAL report
sections**: First Impressions, Commit Crimes, Branch Jungle, Ancient Relics, Language Soup,
Infrastructure Footprints. Where a section needs evidence the snapshot doesn't yet hold,
**enrich the Phase 2 collectors** to capture it (the cloned workspace is dropped after
collection, so analyzers consume the snapshot only — they cannot re-query git).

**In scope:** compute the missing factual metrics (commits-this-month, top-contributor name,
active/stale branches, oldest branch, oldest file, oldest contributor, longest-living branch),
assemble per-section data structs, and add the fields they need to the snapshot + collectors.

**Explicitly NOT in this phase:** Sections 7-9 (Repository Vibes, Interesting Findings, Crab
Verdict) → Phase 5. The TUI / all rendering → Phase 4. Vibe classification (the snapshot
already carries the input signals — night/weekend/business %, bus factor — Phase 3 does NOT
classify).
</domain>

<decisions>
## Implementation Decisions

### Data Sourcing (the architectural crux)
- **D-01:** Missing section data is sourced by **enriching the Phase 2 collectors** to capture
  it into the snapshot during their existing git walks, then adding the fields to
  `InvestigationSnapshot`. Analyzers/view-models read ONLY from the snapshot (the
  `CloneWorkspace` is dropped after `collect()`; the snapshot stays self-contained). This means
  Phase 3 legitimately TOUCHES Phase 2 code (`src/repo/history.rs`, `src/repo/branches.rs`,
  `src/snapshot.rs`, `src/app/collect.rs`) — expected, not scope creep.

### Ancient Relics — cheap definitions (no expensive per-file history)
- **D-02:**
  - `oldest_file` = a path present in the **first commit's tree** that **still exists in HEAD**
    (O(1) tree intersection — no per-file `git log`). Pick deterministically (lexicographically
    first such path). Empty intersection → `None`.
  - `oldest_contributor` = the (mailmap-resolved) identity with the **earliest first-commit
    author time**, tracked during the existing full-history contributor walk.
  - `longest_living_branch` = the branch with the **oldest tip commit time** (reuse the branch
    tips already collected). ("Longest living" ≈ oldest still-present branch.)

### Branch Jungle & Stale Threshold (resolves the OPEN item from METRICS/STATE)
- **D-03:** `STALE_BRANCH_DAYS = 90`. A branch is **stale** if its tip is > 90 days old (vs
  now), **active** if ≤ 90. Branch Jungle reports: total branches, active count, stale count,
  oldest branch (oldest tip). Computed from `tip_time_secs` already in the snapshot.

### Commit Crimes Gaps
- **D-04:** `commits_this_month` = commits within the **rolling last 30 days** (vs now). No
  calendar-month boundary edge.
- **D-05:** Top contributor is shown by **git author display NAME** (mailmap-resolved),
  fallback to email/login. Capture the name alongside identity in the contributor walk. Reuse
  the existing bot/merge/identity filtering — do not recompute differently.

### View-Model Boundary
- **D-06:** Phase 3 emits **pure data structs** per section (numbers, strings, enums,
  `Option`/`None` for missing). **NO formatting** in Phase 3 — ASCII progress bars, relative
  dates ("3 days ago"), crab labels/icons, and color all live in the **Phase 4 renderer**. Keeps
  analyzers deterministic, unit-testable, and JSON-friendly later.
  - Note: ANLY-04 mentions "ASCII progress bars" — Phase 3 produces the language **percentages**;
    the bar string is rendered in Phase 4.

### Scope Notes
- **D-07:** ANLY-05 (infrastructure detection) was already delivered by Phase 2's `detect_infra`
  (COLL-03). Phase 3's only ANLY-05 work is **assembling the Infra Footprints view model** from
  the existing flags. First Impressions / Language Soup view models are likewise mostly
  assembly from existing snapshot fields.

### Claude's Discretion
- **"Last activity" source:** prefer the **most recent branch tip (git)** as the canonical
  "last commit" time — it is always present; `pushed_at` (API) may be unavailable when metadata
  degraded. Use git as the reliable source, API `pushed_at` only as a fallback/cross-check.
- Exact per-section struct shapes — planner's call (pure data per D-06).
- Module placement (e.g., `src/analyze/` for analyzers, `src/report/` for view models per
  ARCHITECTURE.md) — planner's call.

### Carried Forward (locked — do NOT re-litigate)
- Shared bot/merge/mailmap identity filtering (research/METRICS.md) · bounded 1000-commit window
  for expensive passes · no rename tracking · snapshot is the data carrier · separation
  analyze → view-model → render · deterministic + fixture-testable.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope & requirements
- `.planning/ROADMAP.md` § "Phase 3: Analysis Layer" — goal, success criteria, plans 03-01..03-03
- `.planning/REQUIREMENTS.md` — ANLY-01 (Commit Crimes), ANLY-02 (Branch Jungle), ANLY-03 (Ancient Relics), ANLY-04 (Language Soup), ANLY-05 (Infra — mostly done in Phase 2)

### Metrics & signal contracts
- `.planning/research/METRICS.md` — shared bot/merge/mailmap identity filtering (reuse for top-contributor name + oldest-contributor)
- `.planning/research/VIBES.md` — the signal inputs (night/weekend/business %, bus_factor) Phase 3 must keep intact for Phase 5 vibe scoring

### Architecture
- `.planning/research/ARCHITECTURE.md` — `report/` view model vs `analyze/` analyzers; pure-functions-over-snapshot pattern

### Phase 2 artifacts to build ON / enrich
- `.planning/phases/02-collection-layer/02-CONTEXT.md` — snapshot decisions
- `.planning/phases/02-collection-layer/02-RESEARCH.md` — git2 patterns (tree intersection, revwalk) for the enrichment
- `src/snapshot.rs` (extend), `src/repo/history.rs` + `src/repo/branches.rs` (enrich walks), `src/app/collect.rs` (wire new fields)
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets (Phase 2, on `main`)
- `InvestigationSnapshot` already carries: total_commits, repo_age_days, contributor_count,
  bus_factor, top_author_share_pct, window, most_modified_file, night/weekend/business %,
  infra flags + `InfraFootprints`, `languages: Vec<(String,f64)>`, `BranchFacts` (default_branch,
  branches[name, tip_time_secs], last_activity_secs), `RepoMetaState` (stars/forks/desc/topics/
  default_branch/pushed_at/created_at).
- `collect_contributors` (full-history walk) — extend to also track per-identity **display name**
  and **earliest first-commit time** (top-contributor name + oldest contributor).
- `enumerate_branches` — tips already collected; derive active/stale/oldest + longest-living.
- The first-commit tree is reachable (revwalk Sort::TIME|REVERSE already used in `repo_age_days`)
  — reuse to compute `oldest_file` via tree∩HEAD.

### Established Patterns
- `#[cfg(test)] mod tests` + `make_fixture_repo()` in `repo/history.rs` — extend the fixture to
  cover new metrics (oldest file/contributor, stale branches, commits-this-month).

### Integration Points
- Snapshot is the single data carrier — every new metric lands there.
- `src/app/run.rs` currently prints raw snapshot fields; Phase 4 replaces this with section
  rendering from the view models Phase 3 produces.
</code_context>

<specifics>
## Specific Ideas

- Section view models map 1:1 to the vision's Sections 1-6 (First Impressions, Commit Crimes,
  Branch Jungle, Ancient Relics, Language Soup, Infrastructure Footprints).
- Missing-data fields are `Option`/`None`, not fabricated zeros (e.g., `oldest_file: None` when
  the first-commit∩HEAD set is empty) — honest, and Phase 4 can show "—".
- Keep every new metric deterministic so the extended `make_fixture_repo()` tests lock them.
</specifics>

<deferred>
## Deferred Ideas

- **True per-file history** (precise oldest-file via per-path `git log`, most-modified across
  ALL history) → `--deep` / v2. V1 uses the cheap tree-intersection + bounded-window approach.
- **Calendar-month commits** → not chosen (rolling 30 days).
- **Pre-formatted strings** (ASCII bars, relative dates) → Phase 4 renderer.
- **Vibes/Findings/Verdict** (Sections 7-9) → Phase 5.

None of the above is scope creep into Phase 3 — they are intentional boundaries.
</deferred>

---

*Phase: 3-Analysis Layer*
*Context gathered: 2026-06-03*
