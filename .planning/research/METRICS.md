# Metric Definitions

**Domain:** Precise, executor-ready definitions for the fiddly rust-to-you metrics
**Defined:** 2026-06-02 (brainstorm)
**Status:** Bus factor locked; other entries tracked as open where noted
**Why this file exists:** Some metrics are taste/heuristic calls that must be pinned down once
so the analyzer layer (Phase 3) and the vibe classifier (research/VIBES.md) agree exactly.

---

## Shared: Author Identity & Commit Filtering

These rules apply to **every** author-based metric — `contributor_count`, `top_author_share`,
and `bus_factor` — so the whole report stays internally consistent.

**Identity normalization:**
- If the repo has a `.mailmap`, use it (git2 supports mailmap resolution).
- Otherwise group authors by **lowercased email**.
- Rationale: one human with multiple emails ("alice@work" + "alice@home") must count as one
  person, or every author-based metric skews.

**Commit filtering (before any author counting):**
- **Exclude merge commits** (commits with >1 parent) — noise, not authored work.
- **Exclude bot authors** — reuse the same bot list as Infrastructure Footprints
  (`dependabot[bot]`, `renovate[bot]`, `github-actions[bot]`, and `*[bot]` by convention).
  Bots can be a repo's top committer and would wreck bus factor / contributor stats.

**Fallback:** if filtering removes *all* authors (e.g. a bot-only repo), recompute the affected
metric **without** the bot filter so the report still shows something sensible.

---

## Bus Factor

**Definition (V1):**
> The smallest number of authors who together account for **≥ 50%** of commits, after applying
> the shared identity normalization and commit filtering above.

It answers: "how few people could leave before half the project's commit knowledge walks out
the door?" It is a true **integer** count of people.

### Method (chosen: commit-count)

| Considered | Approach | Decision |
|------------|----------|----------|
| **A. Commit-count** | Count commits per author, accumulate to ≥50% | ✅ **V1** — one O(commits) pass, no diffs, cheap on full history |
| B. Lines-authored | Weight by lines changed | ✗ requires diff-walking every commit |
| C. Blame/ownership (academic truck factor) | git-blame files, count orphaned files | → deferred to `--deep` (v2) |

Bus factor uses **full history** (not the bounded commit window) — counting is cheap because it
never diffs. Only blame/lines passes are expensive, and those are deferred.

### Algorithm

```
1. Walk all commits; drop merge commits and bot authors; normalize identity.
2. Build author -> commit_count.
3. total = sum of all counts.
4. Sort authors by commit_count DESC, then by name ASC (stable tie-break → deterministic tests).
5. Accumulate counts; bus_factor = number of authors needed to reach >= 0.5 * total.
```

**Threshold:** 50% (standard).

**Edge cases:**
- Empty repo or single author → `bus_factor = 1`.
- Filtering leaves 0 authors → apply the bot-filter fallback above, then recompute.

### Presentation (Section 2 — Commit Crimes)

Show the integer plus the concentration share for flavor (replaces the vision's fictional `1.9`):
```
Bus Factor: 2 ☠️ — top 2 devs own 63% of commits
```
The ☠️ is a display flourish: emphasize it when `bus_factor` is low (1–2). Exact icon thresholds
are a report-layer concern (Phase 4), not part of this definition.

### Consumers

- Section 2 (Commit Crimes) — primary display.
- research/VIBES.md → 🧙 Solo Wizard condition `bus_factor == 1`.

---

## Related Definitions (tracked elsewhere / still open)

| Metric | Where defined | Status |
|--------|---------------|--------|
| `night_pct` / `weekend_pct` / `business_hours_pct` | research/VIBES.md (Input Signals) | Defined: author-local time over the bounded commit window |
| Bounded commit window size `N` (for most-modified-file + time-of-day) | Phase 2/3 planning | **OPEN** — pick a concrete `N` during planning |
| Stale-branch threshold (days) | Phase 3 planning | **OPEN** — Branch Jungle needs an explicit cutoff |

---
*Metric definitions for: rust-to-you*
*Defined: 2026-06-02 (brainstorm)*
