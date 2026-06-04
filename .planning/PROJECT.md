# rust-to-you

## What This Is

rust-to-you is a playful Rust CLI/TUI that investigates a public GitHub repository from one command: `rust-to-you <repo-url>`. It clones just enough data, inspects history, structure, CI/CD and infra clues, then renders a cute read-only report that feels more like repository archaeology and gossip than a dashboard.

It is being built for the project owner herself: a fast way to understand an unfamiliar repo without clicking through a pile of GitHub tabs.

## Core Value

Given one public GitHub repository URL, produce a cute, readable TUI investigation report faster than manually digging through the GitHub UI.

## Current Milestone: v1.2.0 Robustness & Safety Hardening

**Goal:** Make rust-to-you safe to point at *any* repository (huge or hostile) and safe to interrupt at any moment — no machine hang, no orphaned temp files, no injection surface.

**Target features:**
- Pre-flight size guard via the GitHub API: refuse oversized repos with a clear message, opt in to the long path with `--deep`.
- Hardened intake parser: block argument-injection (leading `-`), cap owner/repo segment lengths, document the threat model.
- Interruptible lifecycle: SIGINT/SIGTERM cleanup so a mid-run Ctrl-C never leaks the clone temp dir.
- Self-healing temp hygiene: startup sweep of orphaned temp dirs from prior crashed runs.

## Requirements

### Validated

- ✓ User can investigate a public GitHub repository with a single command. — v1.0
- ✓ User gets a scrollable TUI report with the nine MVP sections and crab-flavored presentation. — v1.0
- ✓ Investigation stays read-only and focused on public GitHub repositories only. — v1.0

### Active

<!-- v1.2.0 — Robustness & Safety Hardening. Detailed REQ-IDs in REQUIREMENTS.md -->

- [ ] User is protected from accidentally hanging their machine on an oversized repo.
- [ ] User can opt into deep analysis of large repos with an explicit `--deep` flag.
- [ ] User's machine is never left with orphaned temp files, even if the run is interrupted.
- [ ] User is protected from malicious or malformed repo inputs (injection-hardened intake).

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
- The preferred implementation stack is Rust with `clap`, `git2`, `reqwest` (blocking), `serde`, `serde_json`, `ratatui`, `crossterm`, and `chrono`. No async runtime in V1 — the pipeline is sequential and the GitHub API surface is a single call.
- V1 does a **full clone** (not shallow): archaeology metrics (oldest file, most-modified file, bus factor, branch ages) need complete git history. Expensive history passes are bounded instead (e.g. cap commits scanned for most-modified-file).
- The GitHub REST API is used **only** for facts git cannot provide locally — stars, forks, and description/topics. Everything else comes from the local clone: `git2` for commits/branches/contributors/file history, plus local scanners for languages and infra, so rate limits are a non-issue.

## Constraints

- **Domain**: Public GitHub repositories only — avoids auth, private-access, and multi-host complexity in V1.
- **Interaction**: Single command only — V1 must start from `rust-to-you <repo-url>` without requiring extra flags.
- **Presentation**: One scrollable vertical report — no tabs, no multi-screen flows, no dashboard sprawl.
- **Safety**: Read-only investigation — the tool must never mutate the remote repository.
- **Tone**: Cute and playful output — the crab identity is part of the product value, not garnish.
- **Narrator/voice**: All user-facing text is narrated by **Ferris** (the Rust crab mascot 🦀). Ferris NEVER refers to itself as "tôi/mình" — always speaks of itself in the third person as "Ferris".
- **Bilingual**: All user-facing text (errors, status, section titles, narrative) is **Vietnamese + English**. Messages/errors/section-titles/narrative render as two lines (Vietnamese line, then English line); dense data-row labels render bilingually inline (e.g. "Tổng commit / Total commits: 12,442").
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
| V1 uses a full clone, not a shallow clone | Archaeology sections need complete history; expensive passes are bounded instead | ✅ Decided 2026-06-02 |
| GitHub API limited to stars/forks/description; git2 supplies everything else | Shrinks API to one call → rate limits stop being a concern | ✅ Decided 2026-06-02 |
| Drop `tokio`; use `reqwest` blocking | One sequential API call needs no async runtime; simpler for a first Rust project | ✅ Decided 2026-06-02 |
| Ancient Relics does not track file renames in V1 | Rename-following is tricky and redundant for the playful report; keep it simple | ✅ Decided 2026-06-02 |
| Add a walking-skeleton plan first in Phase 2 | Prove the clone→metric→output pipeline end-to-end early to de-risk integration | ✅ Decided 2026-06-02 |
| Bus factor = integer, commit-count to ≥50% of commits; bots/merges excluded, identities normalized; blame-based truck factor deferred to `--deep` | Cheap on full history, deterministic/testable, honest integer; full def in research/METRICS.md | ✅ Decided 2026-06-02 |
| Narrator is **Ferris** (Rust mascot); never "tôi/mình", always third-person "Ferris" | Gives the tool a consistent on-brand personality (Rust + crab) | ✅ Decided 2026-06-03 |
| All user-facing text is **bilingual VI+EN** — two lines (VI then EN) for messages/errors/titles/narrative; inline "VI / EN" for dense data labels. Retrofit Phase 1-3 strings too | One playful, accessible voice in both languages; centralized in Phase 4's i18n helper | ✅ Decided 2026-06-03 |
| Bot + identity filtering is shared across `contributor_count`, `top_author_share`, `bus_factor` | Keeps every author-based number in the report internally consistent | ✅ Decided 2026-06-02 |
| v1.2.0: oversized repos are **refused by default**, with `--deep` to opt into the long path | Refuse-mode bounds clone size and history walks for free; never surprises the user with a machine hang | ✅ Decided 2026-06-04 |
| v1.2.0: intake security is **tighten + document**, not a new subsystem | `git2` is libgit2 FFI (no shell spawn) so the injection surface is already narrow; charset allowlist exists | ✅ Decided 2026-06-04 |

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
*Last updated: 2026-06-04 after starting milestone v1.2.0 (Robustness & Safety Hardening)*
