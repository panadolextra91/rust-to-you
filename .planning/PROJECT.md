# rust-to-you

## What This Is

rust-to-you is a playful Rust CLI/TUI that investigates a public GitHub repository from one command: `rust-to-you <repo-url>`. It clones just enough data, inspects history, structure, CI/CD and infra clues, then renders a cute read-only report that feels more like repository archaeology and gossip than a dashboard.

It is being built for the project owner herself: a fast way to understand an unfamiliar repo without clicking through a pile of GitHub tabs.

## Core Value

Given one public GitHub repository URL, produce a cute, readable TUI investigation report faster than manually digging through the GitHub UI.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] User can investigate a public GitHub repository with a single command.
- [ ] User gets a scrollable TUI report with the nine MVP sections and crab-flavored presentation.
- [ ] Investigation stays read-only and focused on public GitHub repositories only.

### Out of Scope

- PR analysis — explicitly deferred from V1 to keep scope on repository archaeology.
- Issue analysis — not required for the first playful report flow.
- Security scanning — too broad for the initial read-only gossip experience.
- Dependency graphing — valuable later, but not core to the first report.
- AI-generated architecture review — too subjective and heavyweight for V1.
- Authentication and private repository access — public-repo-only keeps setup friction low.
- GitLab and Bitbucket support — GitHub-only keeps acquisition logic narrow for launch.
- Tabbed or multi-screen TUI navigation — user wants one vertical report, not app complexity.
- Any write action against the remote repository — V1 is inspection only.

## Context

- The tool exists because manually opening GitHub tabs to inspect a repo is annoying and slow.
- The desired tone is intentionally playful: "repository archaeology & gossip," not enterprise analytics.
- V1 input is exactly `rust-to-you <repo-url>` with no required flags.
- The report should highlight both useful signals and funny observations, especially in Repository Vibes, Interesting Findings, and Crab Verdict.
- The preferred implementation stack is Rust with `clap`, `git2`, `reqwest`, `serde`, `serde_json`, `tokio`, `ratatui`, `crossterm`, and `chrono`.

## Constraints

- **Domain**: Public GitHub repositories only — avoids auth, private-access, and multi-host complexity in V1.
- **Interaction**: Single command only — V1 must start from `rust-to-you <repo-url>` without requiring extra flags.
- **Presentation**: One scrollable vertical report — no tabs, no multi-screen flows, no dashboard sprawl.
- **Safety**: Read-only investigation — the tool must never mutate the remote repository.
- **Tone**: Cute and playful output — the crab identity is part of the product value, not garnish.
- **Architecture**: Horizontal-layer roadmap — implementation should be planned by technical layers because that is the chosen project structure mode.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| V1 supports public GitHub repositories only | Narrow scope keeps repo acquisition, metadata, and rate-limit handling tractable | — Pending |
| V1 command surface is only `rust-to-you <repo-url>` | Fastest path to a focused first release | — Pending |
| TUI is a single vertical report | Reading the report should feel like scrolling a case file, not navigating an app | — Pending |
| Product positioning is "repository archaeology & gossip" | Distinguishes the tool from dashboards and dry analytics | — Pending |
| V1 is read-only | Prevents surprise side effects and keeps trust high | — Pending |
| Not in V1: PRs, issues, security scans, dependency graphs, AI architecture review, auth | Protects MVP scope and keeps attention on the core report | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-06-02 after initialization*
