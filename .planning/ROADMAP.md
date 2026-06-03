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
- [ ] **Phase 4: Presentation Layer** - Render the investigation as a cute scrollable TUI.
- [ ] **Phase 5: Polish & Calibration** - Tune vibes, findings, and end-to-end report quality.

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

- [ ] 04-01: Build the report renderer and section widgets
- [ ] 04-02: Add scrolling, keyboard handling, and terminal-resilience polish

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

- [ ] 05-01: Implement and tune vibe, findings, and verdict heuristics (vibe classifier per research/VIBES.md)
- [ ] 05-02: Run fixture and manual report verification across sample repositories

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Intake & Guardrails | 2/2 | Completed | 2026-06-02 |
| 2. Collection Layer | 4/4 | Completed | 2026-06-03 |
| 3. Analysis Layer | 3/3 | Completed | 2026-06-03 |
| 4. Presentation Layer | 0/2 | Not started | - |
| 5. Polish & Calibration | 0/2 | Not started | - |
