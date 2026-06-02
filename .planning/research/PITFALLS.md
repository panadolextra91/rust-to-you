# Pitfalls Research

**Domain:** Repository investigation CLI/TUI
**Researched:** 2026-06-02
**Confidence:** MEDIUM

## Critical Pitfalls

### Pitfall 1: Shallow clone blind spots

**What goes wrong:**
Branch, file-age, or most-modified-file metrics become misleading because the local clone does not contain enough history.

**Why it happens:**
Teams optimize for speed first, then accidentally compute archaeology metrics from incomplete data.

**How to avoid:**
Define which sections are allowed to use shallow history, and label estimated metrics clearly when deeper history is not fetched.

**Warning signs:**
Metrics change wildly between repeated runs, or "oldest" and "most modified" results look suspiciously recent.

**Phase to address:**
Phase 2: Collection Layer

---

### Pitfall 2: GitHub rate-limit pain

**What goes wrong:**
Public unauthenticated requests hit REST rate limits and investigations fail unpredictably.

**Why it happens:**
Branch pagination, contributors, commits, and repo metadata all seem cheap until they stack up.

**How to avoid:**
Centralize GitHub requests, cap concurrent calls, surface rate-limit errors cleanly, and make the data-collection plan explicit.

**Warning signs:**
HTTP 403/429 responses or intermittent failures on branch-heavy repositories.

**Phase to address:**
Phase 2: Collection Layer

---

### Pitfall 3: Heuristic comedy without evidence

**What goes wrong:**
Repository Vibes or Interesting Findings feel random, untrustworthy, or mean.

**Why it happens:**
The product voice is fun, so it is tempting to improvise labels without grounded signals.

**How to avoid:**
Make every vibe and finding trace back to concrete evidence bullets and keep risk language playful but fair.

**Warning signs:**
The section reads funny but cannot explain why it chose a label.

**Phase to address:**
Phase 5: Polish & Calibration

---

### Pitfall 4: TUI pretty but not readable

**What goes wrong:**
The report looks cute in screenshots but becomes annoying to read in a real terminal.

**Why it happens:**
Teams optimize for decoration over scroll behavior, spacing, and small-window resilience.

**How to avoid:**
Keep one scrollable layout, verify narrow widths early, and separate styling from section content.

**Warning signs:**
Wrapped labels, clipped sections, or keyboard scrolling that loses context.

**Phase to address:**
Phase 4: Presentation Layer

---

### Pitfall 5: Over-collecting before proving value

**What goes wrong:**
The project drifts into PR analysis, issue mining, security scanning, or full repository intelligence before the MVP report is solid.

**Why it happens:**
Repository tooling has infinite tempting side quests.

**How to avoid:**
Keep the V1 contract strict: public GitHub repos, read-only investigation, one report, nine sections.

**Warning signs:**
New collectors appear that do not directly power a V1 section.

**Phase to address:**
Phase 1: Intake & Boundaries

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hard-coding file detectors inline | Fast to ship | Detector logic becomes brittle and hard to extend | Acceptable only for the first few signatures |
| Computing findings directly in widgets | Faster demo | Impossible to test or reuse for JSON later | Never |
| Using one giant analyzer module | Less file setup | Becomes unreadable as sections multiply | Acceptable only during spike code |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| GitHub REST | Treating every public call as "free" | Track rate-limit impact and paginate intentionally |
| `git2` clone/fetch | Assuming default clone settings are good for archaeology | Make depth and branch behavior explicit in collection code |
| Terminal UI | Assuming terminal size is generous | Test narrow terminals and preserve scroll state |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Full-history traversal on every run | Slow investigations | Cache intermediate facts later; keep passes bounded now | Large or long-lived repos |
| Loading giant trees eagerly | High memory usage | Stream or summarize where possible | Big monorepos |
| Repeated API calls for the same repo facts | Network overhead | Normalize remote metadata into one snapshot | Re-runs and branch-heavy repos |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Expanding to private repo support casually | Token leakage and auth complexity | Stay public-only in V1 |
| Following write-capable GitHub paths later without design review | Violates read-only trust model | Keep write actions explicitly out of scope |
| Logging full URLs or auth material | Accidental secret exposure if auth arrives later | Sanitize input and logs from the start |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Report feels like a monitoring dashboard | Loses the promised playful experience | Keep the case-file narrative structure |
| Too many icons or jokes | Harder to scan real facts | Use humor to frame sections, not replace evidence |
| Hidden section ordering | User has to learn navigation | Use a fixed top-to-bottom reading order |

## "Looks Done But Isn't" Checklist

- [ ] **First Impressions:** Verify contributor and activity figures are sourced consistently.
- [ ] **Commit Crimes:** Verify bus factor is labeled as an estimate if heuristic-based.
- [ ] **Branch Jungle:** Verify stale vs active branch rules use explicit time thresholds.
- [ ] **Ancient Relics:** Verify file-history logic handles renames or documents that it does not.
- [ ] **Language Soup:** Verify percentages sum correctly and ASCII bars align with values.
- [ ] **Repository Vibes:** Verify every label has evidence bullets.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Shallow clone blind spots | MEDIUM | Add targeted deeper fetches for affected metrics and label changed behavior in release notes |
| Rate-limit failures | LOW | Retry after reset window, reduce concurrency, and fail gracefully with clear messaging |
| Weak vibe heuristics | LOW | Re-rank evidence weights and add fixture repos for calibration |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Shallow clone blind spots | Phase 2 | Fixture repos produce stable archaeology metrics |
| GitHub rate-limit pain | Phase 2 | Repeated public runs fail gracefully under limit pressure |
| Heuristic comedy without evidence | Phase 5 | Each vibe/finding shows supporting bullets |
| TUI pretty but not readable | Phase 4 | Narrow-terminal manual checks pass |
| Over-collecting before proving value | Phase 1 | Every collector maps to a V1 section requirement |

## Sources

- https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api
- https://docs.github.com/en/rest/branches/branches?apiVersion=latest
- https://docs.github.com/en/rest/git/trees
- https://docs.rs/crate/git2/latest
- Product constraints and exclusions from PROJECT.md

---
*Pitfalls research for: Repository investigation CLI/TUI*
*Researched: 2026-06-02*
