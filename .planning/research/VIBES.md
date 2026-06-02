# Repository Vibes Ruleset

**Domain:** Section 7 "Repository Vibes" classifier for rust-to-you
**Defined:** 2026-06-02 (brainstorm)
**Status:** Spec ready — thresholds are V1 starting values, to be tuned in Phase 5
**Implements:** NARR-01 (classify vibes from signals + show evidence bullets)
**Mitigates:** PITFALLS Pitfall 3 (heuristic comedy without evidence)

## Purpose

Repository Vibes assigns one playful "personality" label to a repo, backed by concrete
evidence bullets. It is the soul of the report: funny but never ungrounded. Every label must
trace back to observed signals, and every shown bullet is a condition that actually fired.

## Design Approach: Weighted Scoring

Not first-match-wins. Each vibe is a set of weighted conditions; the engine sums the weights of
satisfied conditions per vibe and the highest score wins. This is chosen because:

- Real repos overlap multiple vibes (solo + ancient, startup + crunch).
- Each satisfied condition emits one evidence bullet — evidence falls out for free.
- It naturally yields a runner-up vibe (see Output Contract → Section 8 reuse).
- Deterministic and fixture-testable (satisfies ROADMAP Phase 3/5 testability criteria).

## Input Signals

All signals come from already-planned collectors/analyzers — no extra GitHub API calls.
Time-based signals are computed over the **same bounded commit window** used for "most modified
file" (see PITFALLS Pitfall 1 — expensive full-history passes).

| Signal | Source | Notes |
|--------|--------|-------|
| `age_years` | First Impressions | Repo age from first commit |
| `days_since_last_commit` | First Impressions | Recency of activity |
| `commits_last_30d` | Commit Crimes | Recent velocity |
| `contributor_count` | Commit Crimes | Distinct authors (bot-filtered, identity-normalized — see research/METRICS.md) |
| `top_author_share` | Commit Crimes | % of commits by #1 author (same filtering as above) |
| `bus_factor` | Commit Crimes | Integer — fewest authors making ≥50% of commits. Full definition in research/METRICS.md |
| `branch_count` | Branch Jungle | Total branches |
| `stale_branch_count` | Branch Jungle | Branches past stale threshold |
| `release_tag_count` | git tags via git2 | **No API needed** — tags are git objects |
| `has_ci` | Infrastructure Footprints | GitHub Actions / GitLab CI / CircleCI / Jenkins |
| `infra_count` | Infrastructure Footprints | Count of detected infra signals |
| `has_bot` | Infrastructure Footprints | Dependabot or Renovate present |
| `night_pct` | Commit Crimes | % commits 00:00–05:00 in the commit's **own author-local time** (git stores the tz offset) |
| `weekend_pct` | Commit Crimes | % commits on Sat/Sun (author-local) |
| `business_hours_pct` | derived | % commits Mon–Fri ~09:00–18:00 author-local |

## Scoring Algorithm

```
1. Score all 7 vibes (sum weights of satisfied conditions, including Chaotic Good's positives).
2. winner = vibe with the highest score.
   - On a tie, break by SPECIFICITY ORDER (below).
3. If winner.score < MIN_SCORE (4) → force Chaotic Good (fallback role).
4. runner_up = 2nd-highest distinct vibe with score ≥ 2 (excluding winner); else None.
   - runner_up is NOT shown in Section 7. It is passed to Section 8 (Interesting Findings).
```

**MIN_SCORE = 4** — guards against weak/accidental classifications.

**Specificity order (most specific → most general), used for tie-breaks:**
Sleep-Deprived Startup → Solo Wizard → Startup Goblin → Corporate Fortress →
Open Source Kingdom → Ancient Temple → Chaotic Good

## Vibe Definitions

Each condition lists its weight `(+n)` and the evidence bullet it emits when satisfied.
Thresholds are **V1 starting values** — calibrate in Phase 5 against sample repos.

### 🧙 Solo Wizard — "one crab built the whole temple"
| Condition | Weight | Evidence bullet |
|-----------|--------|-----------------|
| `top_author_share ≥ 65%` | +3 | "{author} authored {share}% of all commits" |
| `contributor_count ≤ 5` | +2 | "Only {n} contributors ever" |
| `bus_factor == 1` | +2 | "Bus factor 1 — {author} owns {share}% ☠️" |

### 🏛️ Ancient Temple — "old, slow, and quietly revered"
| Condition | Weight | Evidence bullet |
|-----------|--------|-----------------|
| `age_years ≥ 5` | +3 | "{age_years} years old" |
| `days_since_last_commit > 120` | +2 | "Last commit {days} days ago" |
| `release_tag_count ≥ 5` | +1 | "{n} release tags" |

### 🏢 Corporate Fortress — "process-heavy, business hours only"
| Condition | Weight | Evidence bullet |
|-----------|--------|-----------------|
| `has_ci` | +2 | "CI/CD pipeline present" |
| `infra_count ≥ 3` | +2 | "{n} infrastructure signals detected" |
| `has_bot` | +1 | "Dependency bot configured" |
| `business_hours_pct ≥ 80%` | +2 | "{pct}% of commits during Mon–Fri business hours" |
| `contributor_count ≥ 10` | +1 | "{n} contributors" |

### 👑 Open Source Kingdom — "a big, healthy community"
| Condition | Weight | Evidence bullet |
|-----------|--------|-----------------|
| `contributor_count ≥ 30` | +3 | "{n} contributors" |
| `top_author_share < 40%` | +2 | "No single dev owns more than {share}% of commits" |
| `days_since_last_commit ≤ 30` | +1 | "Active within the last {days} days" |
| `has_ci` | +1 | "CI/CD pipeline present" |

### 🧌 Startup Goblin — "small, fast, gloriously messy"
| Condition | Weight | Evidence bullet |
|-----------|--------|-----------------|
| `age_years < 3` | +2 | "Only {age_years} years old" |
| `commits_last_30d ≥ 50` | +2 | "{n} commits in the last 30 days" |
| `release_tag_count == 0` | +1 | "0 release tags" |
| `2 ≤ contributor_count ≤ 8` | +1 | "{n} contributors" |

### 😵 Sleep-Deprived Startup — "Goblin, but it's 3am"
Shares ALL Startup Goblin conditions (same weights) PLUS:
| Condition | Weight | Evidence bullet |
|-----------|--------|-----------------|
| `branch_count ≥ 25` | +1 | "{n} branches in flight" |
| `night_pct ≥ 25%` | +3 | "{pct}% of commits between midnight and 5am" |

> Because it is a superset of Goblin, it naturally outscores Goblin whenever the
> crunch signals fire — no manual priority needed.

### 🔥 Chaotic Good — "useful but feral" (fallback + positive)
Wins by fallback (no vibe ≥ MIN_SCORE) OR on its own positive conditions:
| Condition | Weight | Evidence bullet |
|-----------|--------|-----------------|
| `days_since_last_commit ≤ 30` | +1 | "Still active" |
| `stale_branch_count ≥ 15` | +2 | "{n} stale branches abandoned in the wild" |
| `release_tag_count == 0` | +1 | "Ships with 0 release tags" |
| `!has_ci` | +1 | "No CI/CD safety net" |

## Output Contract

```rust
struct VibeResult {
    primary: VibeLabel,        // shown in Section 7 with its icon
    primary_score: u32,
    evidence: Vec<String>,     // bullets = satisfied conditions of `primary`
    runner_up: Option<VibeLabel>, // NOT shown in Section 7 — fed to Section 8
}
```

- Section 7 renders: icon + `primary` label + `evidence` bullets.
- Section 8 (Interesting Findings) may use `runner_up` for a line like
  "Also gives off {runner_up} energy."

## Calibration Notes (Phase 5)

- All numeric thresholds above are **V1 starting guesses**. Run against a fixture set
  (e.g. axum, a solo side-project, a corporate repo, a dormant old repo) and adjust until
  labels feel right.
- Keep risk/insult language playful but fair (Pitfall 3).
- If many real repos land on Chaotic Good, MIN_SCORE or thresholds are likely too strict.

## Cross-References

- ROADMAP Phase 5 → plan 05-01 (implements & tunes this ruleset)
- REQUIREMENTS → NARR-01
- PITFALLS → Pitfall 1 (bounded commit window), Pitfall 3 (grounded evidence)
- research/METRICS.md → bus factor + shared author identity/bot filtering (now locked)

---
*Vibe ruleset for: rust-to-you Section 7*
*Defined: 2026-06-02 (brainstorm)*
