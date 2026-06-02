# Feature Research

**Domain:** Repository investigation CLI/TUI
**Researched:** 2026-06-02
**Confidence:** MEDIUM

## Feature Landscape

### Table Stakes (Users Expect These)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Single-command repo intake | The tool should feel faster than opening browser tabs | LOW | `rust-to-you <repo-url>` is the core contract. |
| Reliable repo summary | Users expect overview stats before any clever commentary | MEDIUM | Powers First Impressions and report credibility. |
| Git-history-based metrics | Without commit/branch/relic data, the premise falls flat | HIGH | Needs careful data collection and clearly labeled estimates. |
| Filesystem and infra detection | Repo archaeology needs structure clues, not just commits | MEDIUM | Look for workflow files, Dockerfiles, Terraform, bots, etc. |
| Readable terminal report | Terminal-native tools live or die on presentation | MEDIUM | One long scroll is simpler than a navigation-heavy TUI. |

### Differentiators (Competitive Advantage)

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Repository Vibes classification | Gives the tool personality and memorability | MEDIUM | Must be backed by concrete evidence bullets. |
| Interesting Findings bullets | Turns raw stats into gossip-worthy observations | MEDIUM | Works best when generated from ranked heuristics. |
| Crab Verdict summary | Gives closure and a strong final takeaway | LOW | Best when it balances useful strengths/risks with humor. |
| Case ID and report framing | Makes the investigation feel like a filed report | LOW | Pure delight feature that reinforces tone without changing architecture. |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Multi-screen dashboard | Sounds powerful | Adds navigation complexity and weakens the "read one report" concept | Keep one vertical case-file report |
| PR and issue analysis in V1 | Feels like "more complete" GitHub coverage | Explodes API surface and distracts from repo archaeology | Defer to v2+ if the core report lands well |
| Private repo auth from day one | Common enterprise expectation | Adds setup friction, token handling, and rate-limit complexity | Stay public-repo-only for launch |
| AI architecture review | Sounds smart on paper | High risk of low-trust fluff in a tool whose charm comes from grounded observations | Keep findings deterministic and evidence-based |

## Feature Dependencies

```text
Single-command repo intake
    └──requires──> URL validation + investigation session
                          └──requires──> minimal API metadata + full clone

Commit/branch/relic metrics
    └──requires──> normalized git snapshot

Repository vibes
    └──requires──> metrics + evidence ranking

Interesting findings
    └──enhances──> summary, archaeology, and verdict sections

Multi-screen dashboard
    ──conflicts──> one-scroll report philosophy
```

### Dependency Notes

- **Metrics require a normalized snapshot:** the tool should collect once, analyze many times.
- **Vibes require evidence:** classification without observable signals turns into empty comedy.
- **One-scroll report conflicts with tabs:** introducing navigation would dilute the main reading experience.

## MVP Definition

### Launch With (v1)

- [ ] Single-command public GitHub intake — essential to beat manual tab clicking
- [ ] Nine-section read-only report — the promised user-facing artifact
- [ ] Commit, branch, archaeology, language, and infra analyzers — core evidence set
- [ ] Repository Vibes, Interesting Findings, and Crab Verdict — the product's voice

### Add After Validation (v1.x)

- [ ] `--json` output — add when users want piping or automation
- [ ] Simple local caching — add when repeated investigations become common
- [ ] Optional `--deep` mode — lift the default commit caps for unbounded history analysis when users want slower-but-richer archaeology

### Future Consideration (v2+)

- [ ] PR analysis — defer until core report proves valuable
- [ ] Issue analysis — defer until repo-level archaeology is solid
- [ ] Private repo support — defer until auth/setup trade-offs are worth it
- [ ] GitLab/Bitbucket support — defer until GitHub-only flow is stable

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Public GitHub repo intake | HIGH | LOW | P1 |
| First Impressions section | HIGH | MEDIUM | P1 |
| Commit Crimes section | HIGH | HIGH | P1 |
| Branch Jungle section | HIGH | HIGH | P1 |
| Ancient Relics section | HIGH | HIGH | P1 |
| Language Soup section | MEDIUM | MEDIUM | P1 |
| Infrastructure Footprints section | MEDIUM | MEDIUM | P1 |
| Repository Vibes section | HIGH | MEDIUM | P1 |
| Interesting Findings section | HIGH | MEDIUM | P1 |
| Crab Verdict section | HIGH | LOW | P1 |
| JSON export | MEDIUM | LOW | P2 |
| Cache/offline/deep flags | MEDIUM | MEDIUM | P2 |
| PR analysis | LOW for MVP | HIGH | P3 |

**Priority key:**
- P1: Must have for launch
- P2: Should have, add when possible
- P3: Nice to have, future consideration

## Competitor Feature Analysis

| Feature | Typical analytics tools | Typical git dashboards | Our Approach |
|---------|-------------------------|------------------------|--------------|
| Repository summary | Often present but dry | Often present but operational | Keep it readable and case-file styled |
| Commit and branch metrics | Usually numeric only | Usually chart-heavy | Use metrics as evidence for narrative sections |
| Personality/tone | Usually absent | Usually absent | Make the report playful without sacrificing grounding |
| Navigation model | Filters, tabs, charts | Panels and views | One vertical scroll from start to verdict |

## Sources

- User-provided product vision and MVP list
- GitHub public repository inspection capabilities documented in the REST API:
  - https://docs.github.com/en/rest/repos?apiVersion=2022-11-28
  - https://docs.github.com/en/rest/commits
  - https://docs.github.com/en/rest/branches/branches?apiVersion=latest
  - https://docs.github.com/en/rest/git/trees
  - https://docs.github.com/en/rest/repos/contents?apiversion=2022-11-28

---
*Feature research for: Repository investigation CLI/TUI*
*Researched: 2026-06-02*
