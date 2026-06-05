# Roadmap: rust-to-you

## Overview

rust-to-you moves from a strict public-GitHub intake contract to a layered investigation engine, then finishes as a polished one-scroll TUI report. Because the chosen structure mode is Horizontal Layers, the roadmap builds technical layers in order: intake, collection, analysis, presentation, and final narrative calibration.

## Phases

**Phase Numbering:**

- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Intake & Guardrails** - Lock the single-command contract and V1 boundaries.
- [x] **Phase 2: Collection Layer** - Build GitHub, git, and filesystem data acquisition.
- [x] **Phase 3: Analysis Layer** - Compute the report metrics that power the core sections.
- [x] **Phase 4: Presentation Layer** - Render the investigation as a cute scrollable TUI.
- [x] **Phase 5: Polish & Calibration** - Tune vibes, findings, and end-to-end report quality.

## Phase Details

### Phase 1: Intake & Guardrails

**Goal**: Accept one public GitHub URL, reject unsupported cases clearly, and establish the investigation session contract.
**Depends on**: Nothing (first phase)
**Requirements**: [INPT-01, INPT-02]
**Success Criteria** (what must be TRUE):

  1. User can start an investigation with `rust-to-you <repo-url>`.
  2. User sees clear errors for invalid, unsupported, or private repository inputs.
  3. The investigation path is explicitly read-only and scoped to GitHub public repos.

**Plans**: 2 plans
Plans:
**Wave 1**

- [x] 01-01-PLAN.md — Scaffold Cargo project; clap CLI surface, generous URL parser to RepoRef, and the IntakeError taxonomy with tiered exit codes (wave 1)

**Wave 2** *(blocked on Wave 1 completion)*

- [x] 01-02-PLAN.md — InvestigationSession bootstrap, deterministic case_id, and the run() seam that prints the stub line on a read-only success path (wave 2)

### Phase 2: Collection Layer

**Goal**: Collect all remote and local evidence required for the report without over-fetching.
**Depends on**: Phase 1
**Requirements**: [COLL-01, COLL-02, COLL-03]
**Success Criteria** (what must be TRUE):

  1. Tool can fetch overview metadata for a public GitHub repository.
  2. Tool can gather branch/history evidence and a local repo snapshot for downstream analyzers.
  3. Tool can detect language and infrastructure signals from repository files and config paths.
  4. A walking skeleton proves the full clone → single-metric → printed-output pipeline runs end-to-end.

**Plans**: 4 plans

Plans:

**Wave 1**

- [x] 02-01-PLAN.md — Walking skeleton: add collection deps (git2 vendored-openssl, reqwest blocking, tokei no-default, tempfile, chrono, serde, serde_json), convert to lib+bin crate, RAII-clone a repo, compute repo age, print one line via run() seam [COLL-02]

**Wave 2** *(blocked on Wave 1 — needs the deps + lib target)*

- [x] 02-02-PLAN.md — GitHub client: blocking GET /repos with mandatory User-Agent + optional GITHUB_TOKEN, RepoMetadata serde model, pure classify(StatusCode) error mapping (404→abort, 403/net→transient) [COLL-01]
- [x] 02-03-PLAN.md — git-history collectors: total commits + contributors/bus_factor (shared bot/merge/mailmap filter), branch enumeration via remote refs, bounded most-modified + time-of-day with capped caveat [COLL-01, COLL-02]

**Wave 3** *(blocked on Wave 2 — convergence)*

- [x] 02-04-PLAN.md — Filesystem scanners (tokei languages + 8 infra footprints) + InvestigationSnapshot normalization + API-first/degrade-on-transient collect() orchestrator wired through run() [COLL-01, COLL-02, COLL-03]

### Phase 3: Analysis Layer

**Goal**: Turn normalized evidence into report-ready metrics for the core factual sections.
**Depends on**: Phase 2
**Requirements**: [ANLY-01, ANLY-02, ANLY-03, ANLY-04, ANLY-05]
**Success Criteria** (what must be TRUE):

  1. Tool computes Commit Crimes, Branch Jungle, and Ancient Relics values from collected evidence.
  2. Tool computes Language Soup percentages and Infrastructure Footprints flags in a reusable form.
  3. Analyzer outputs are deterministic enough to support fixture-based tests.

**Plans**: 3 plans

Plans:

**Wave 1**

- [x] 03-01-PLAN.md — Enrich history/branch collectors (commits_this_month rolling-30d, top-contributor display name, STALE_BRANCH_DAYS=90) and emit CommitCrimes + BranchJungle pure-data analyzers [ANLY-01, ANLY-02]

**Wave 2** *(blocked on Wave 1 — shares src/repo/history.rs, src/snapshot.rs, src/app/collect.rs)*

- [x] 03-02-PLAN.md — Enrich history collector (oldest_file via first-commit-tree∩HEAD, oldest_contributor by earliest first-commit time) and emit AncientRelics + LanguageSoup + InfrastructureFootprints analyzers [ANLY-03, ANLY-04, ANLY-05]

**Wave 3** *(blocked on Waves 1+2 — convergence)*

- [x] 03-03-PLAN.md — Assemble FirstImpressions + FactualSections pure-data view model bundling all six factual sections for Phase 4 [ANLY-01, ANLY-02, ANLY-03, ANLY-04, ANLY-05]

### Phase 4: Presentation Layer

**Goal**: Render the investigation as a single scrollable Ratatui report with the desired reading flow.
**Depends on**: Phase 3
**Requirements**: [PRES-01, PRES-02]
**Success Criteria** (what must be TRUE):

  1. User can scroll through the full report in one vertical flow without tabs or mode switching.
  2. Header, case metadata, and the nine MVP sections render in a cute but readable layout.
  3. The TUI remains usable on common terminal widths and preserves scroll state cleanly.

**Plans**: 2 plans

Plans:

**Wave 1**

- [x] 04-01-PLAN.md — Add ratatui/crossterm deps + src/tui/ module; pure format helpers (ascii_bar/thousands/dash_or/relative_date), the reusable Section abstraction (D-01), build_report_lines (header + 6 factual sections), and the non-TTY plain renderer — TestBackend + unit tested [PRES-01, PRES-02]

**Wave 2** *(blocked on Wave 1 — needs build_report_lines + plain::render)*

- [x] 04-02-PLAN.md — TuiState + max_scroll + pure D-03 key handler, the ratatui::run panic-safe scroll/wrap event loop, the IsTerminal TTY-vs-plain branch, and wiring app::run → tui::render (replaces inline printer) [PRES-01, PRES-02]

### Phase 5: Polish & Calibration

**Goal**: Tune the narrative layers and verify the full investigation flow against sample repositories.
**Depends on**: Phase 4
**Requirements**: [NARR-01, NARR-02]
**Success Criteria** (what must be TRUE):

  1. Repository Vibes classifications always show evidence bullets that justify the label.
  2. Interesting Findings and Crab Verdict read coherently on real sample repositories.
  3. End-to-end runs produce stable, trustworthy reports for representative public repos.

**Plans**: 2 plans
**Spec**: Repository Vibes ruleset is defined in `research/VIBES.md` (weighted scoring, MIN_SCORE=4, Chaotic Good fallback) — plan 05-01 implements and calibrates it.

Plans:

**Wave 1**

- [x] 05-01-PLAN.md — Enrich snapshot with release_tag_count (git2 tag_names), implement vibes/findings/verdict pure analyzers (VIBES.md exactly), and render bilingual+Ferris Sections 7-9 via the Section abstraction in report.rs + plain.rs (update the negative assert) [NARR-01, NARR-02]

**Wave 2** *(blocked on Wave 1 — needs the analyzers + thresholds)*

- [x] 05-02-PLAN.md — Calibration & end-to-end verification: deterministic fixtures for every vibe/finding/verdict rule + manual real-repo spot-check (solo / tokio / dormant / corporate) + tune thresholds and write changes back into VIBES.md [NARR-01, NARR-02]

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Intake & Guardrails | 2/2 | Completed | 2026-06-02 |
| 2. Collection Layer | 4/4 | Completed | 2026-06-03 |
| 3. Analysis Layer | 3/3 | Completed | 2026-06-03 |
| 4. Presentation Layer | 2/2 | Completed | 2026-06-03 |
| 5. Polish & Calibration | 2/2 | Completed | 2026-06-03 |

---

# Milestone v1.2.0 — Robustness & Safety Hardening

**Goal:** Make rust-to-you safe to point at *any* repository (huge or hostile) and safe to interrupt at any moment — no machine hang, no orphaned temp files, no injection surface.

**Phase numbering:** Continues from v1.0 (last phase = 5). v1.1.0 added no GSD phases, so this milestone starts at **Phase 6**.

## Phases (v1.2.0)

- [x] **Phase 6: Safe Intake & Pre-flight Guard** - Refuse oversized repos before the clone starts (with a `--deep` opt-in) and harden the intake parser against injection/abuse. (completed 2026-06-05)
- [ ] **Phase 7: Interruptible Lifecycle & Temp Hygiene** - Guarantee the clone temp dir is always cleaned up — on Ctrl-C, on crash recovery, and on every exit path.

## Phase Details (v1.2.0)

### Phase 6: Safe Intake & Pre-flight Guard

**Goal**: Before any clone happens, the tool refuses repositories that would hang the machine and rejects malformed/malicious inputs — protecting the user with clear bilingual messaging and an explicit `--deep` escape hatch.
**Depends on**: Phase 5 (existing intake + collection pipeline)
**Requirements**: [GUARD-01, GUARD-02, GUARD-03, SEC-01, SEC-02]
**Success Criteria** (what must be TRUE):

  1. User pointing the tool at a repository larger than the safe size threshold is stopped *before* the clone begins, via the pre-flight GitHub API size check (metadata is already fetched pre-clone in `src/app/collect.rs`) — their machine never starts an oversized clone unexpectedly.
  2. User sees a clear bilingual (VI+EN), Ferris-narrated message that states the repo is too large, shows its actual size, and explains how to proceed with `--deep`.
  3. User can pass `--deep` to opt into full analysis of a large repository, accepting the longer runtime.
  4. User input with an owner/repo segment beginning with `-`, or exceeding GitHub's segment-length limits, is rejected safely at the parser (`src/cli/parse.rs`) before any network or git operation runs.
  5. The intake threat model is documented and backed by explicit injection/abuse tests that prove malformed inputs cannot reach the git2 / network surfaces.

**Plans**: 3 plans

Plans:

**Wave 1**

- [x] 06-01-PLAN.md — Intake contract + parser hardening: RepoTooLarge (exit 6) + UnsafeInput (exit 2) variants, leading-dash + length-cap guards in parse_repo_ref with injection reject table, --deep flag on Args, RepoMetadata.size field [SEC-01, SEC-02, GUARD-02, GUARD-03]

**Wave 2** *(blocked on Wave 1 — needs the error variants + size field + --deep flag)*

- [x] 06-02-PLAN.md — Pre-flight size guard: pure size_decision helper + MAX_REPO_KB constant + guard branch before clone, --deep threaded via InvestigationSession, fail-open on unknown size [GUARD-01, GUARD-03]
- [x] 06-03-PLAN.md — docs/THREAT-MODEL.md: version-controlled intake threat model (STRIDE + no-shell-spawn rationale) backed by the parser injection tests [SEC-02]

### Phase 7: Interruptible Lifecycle & Temp Hygiene

**Goal**: No run — interrupted, crashed, or completed — ever leaves an orphaned clone temp directory on the user's machine.
**Depends on**: Phase 6
**Requirements**: [CLEAN-01, CLEAN-02, CLEAN-03]
**Success Criteria** (what must be TRUE):

  1. User pressing Ctrl-C (SIGINT) or sending SIGTERM mid-run never leaves an orphaned clone temp directory behind — the live `CloneWorkspace`/TempDir (`src/repo/clone.rs`) is cleaned up before the process exits.
  2. On startup, the tool sweeps away orphaned temp directories left by previously crashed or killed runs, so prior failures self-heal.
  3. No code path exits the process — including panic/abort — while a clone workspace is alive without first cleaning it up.

**Plans**: TBD

## Progress (v1.2.0)

**Execution Order:**
Phases execute in numeric order: 6 → 7

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 6. Safe Intake & Pre-flight Guard | 3/3 | Complete    | 2026-06-05 |
| 7. Interruptible Lifecycle & Temp Hygiene | 0/? | Not started | - |
