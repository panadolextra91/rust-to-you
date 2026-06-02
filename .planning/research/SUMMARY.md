# Project Research Summary

**Project:** rust-to-you
**Domain:** Rust CLI/TUI for GitHub repository investigation
**Researched:** 2026-06-02
**Confidence:** MEDIUM

## Executive Summary

rust-to-you is best treated as a staged investigation pipeline, not a dashboard. The recommended build is: validate one public GitHub URL, collect a mixed remote/local snapshot, compute deterministic section metrics, then render one scrollable Ratatui case file with playful copy layered on top of grounded evidence.

The strongest technical shape is a horizontal-layer architecture because that is the chosen project mode: intake and validation first, collection second, analyzers third, presentation fourth, and calibration/polish fifth. The biggest risks are slow full-history passes on large repos (the local clone is full, so completeness is not the problem — bounding cost is) and vibe heuristics drifting away from evidence.

## Key Findings

### Recommended Stack

The core stack should stay close to the original vision: `clap` for command parsing, `git2` for repository archaeology, `reqwest` (blocking) for the single GitHub metadata call, and `ratatui` with `crossterm` for the report UI. Supporting libraries (`serde`, `serde_json`, `chrono`) round out API modeling, JSON handling, and date-based metrics. No async runtime (`tokio`) in V1 — the pipeline is sequential and the API surface is one request.

**Core technologies:**
- `clap` 4.6.1: CLI contract and validation — ideal for a single-command tool
- `git2` 0.21.0: local git inspection — supports archaeology-style history analysis
- `reqwest` 0.13.4: GitHub HTTP integration — keeps remote metadata structured
- `ratatui` 0.30.0: terminal rendering — best fit for the one-scroll report concept

### Expected Features

The must-have set is unusually clear because the product vision already lists the nine launch sections. The only real question is discipline: avoid adding adjacent "repo intelligence" features before the report itself lands cleanly.

**Must have (table stakes):**
- Single-command public GitHub intake
- Read-only collection of repo metadata, git history, and filesystem signals
- Nine-section vertical report with overview, archaeology, infra, vibes, findings, and verdict

**Should have (competitive):**
- Repository Vibes with evidence bullets
- Interesting Findings that turn stats into narrative
- Case-file framing with crab personality

**Defer (v2+):**
- JSON export, caching, and deep/offline modes
- PR analysis, issue analysis, private repo auth, multi-host support

### Architecture Approach

The project should be built as four functional layers plus a calibration pass: intake, collection, analysis, presentation, and polish. The key architectural decision is to normalize remote API data, git facts, and filesystem findings into one `InvestigationSnapshot` before any section analyzer runs.

**Major components:**
1. CLI intake — validates URL and starts an investigation session
2. Collection services — fetch minimal API metadata (stars/forks/description), full-clone the repo, scan files, enumerate branches and history via git2
3. Analyzer suite — computes section metrics, findings, and verdict inputs
4. Report/TUI layer — renders one scrollable case file from a prepared view model

### Critical Pitfalls

1. **Expensive full-history passes** — full clone gives complete history, so bound costly walks (e.g. cap commits for most-modified-file) and label estimated metrics clearly
2. **GitHub rate limits** — minimized by design (one call for stars/forks/description); still surface limit/network errors cleanly
3. **Ungrounded vibes** — require evidence bullets for every classification and finding
4. **Pretty-but-fragile TUI** — optimize for reading flow and narrow terminals, not screenshots only

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Intake & Guardrails
**Rationale:** Establish the command contract and scope boundaries first.
**Delivers:** Public-GitHub-only validation and read-only investigation setup.
**Addresses:** Input and safety requirements.
**Avoids:** Scope creep into non-V1 collectors.

### Phase 2: Collection Layer
**Rationale:** Every section depends on reliable data acquisition.
**Delivers:** Walking skeleton (clone→metric→print), minimal GitHub metadata client, full clone flow, branch/history enumeration, and filesystem scanning.
**Uses:** `reqwest` (blocking), `git2`, `chrono`.
**Implements:** Data-collection components.

### Phase 3: Analysis Layer
**Rationale:** Once a snapshot exists, metrics and heuristics can be built in isolation.
**Delivers:** First Impressions, Commit Crimes, Branch Jungle, Ancient Relics, Language Soup, and Infrastructure Footprints data.
**Implements:** Analyzer components and reusable finding inputs.

### Phase 4: Presentation Layer
**Rationale:** The report UX should sit on top of stable data, not drive it.
**Delivers:** Scrollable Ratatui report and final section layout.
**Uses:** `ratatui` and `crossterm`.
**Implements:** Report model and TUI components.

### Phase 5: Polish & Calibration
**Rationale:** Vibes, findings, and verdict quality depend on real sample runs.
**Delivers:** Tuned heuristics, integrated end-to-end run, and V1 acceptance checks.
**Implements:** Narrative layers and final QA.

### Phase Ordering Rationale

- Collection must come before analysis because all nine sections depend on shared evidence.
- Analysis must come before presentation so the TUI can stay simple and testable.
- Narrative sections are safest to calibrate at the end, after real metrics exist.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** Branch/history acquisition details for large repos and rate-limit handling
- **Phase 5:** Bus factor heuristic definition (vibe ruleset now specced in research/VIBES.md; remaining work is calibrating its thresholds against sample repos)

Phases with standard patterns (skip research-phase):
- **Phase 1:** CLI validation and guardrails are straightforward
- **Phase 4:** Single-scroll Ratatui layout is established territory once the view model is stable

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Backed by current crate documentation and releases |
| Features | HIGH | Product vision is unusually explicit |
| Architecture | MEDIUM | Layering is clear, but some collection details still need implementation validation |
| Pitfalls | MEDIUM | Strongly grounded, but branch/history edge cases need real fixtures |

**Overall confidence:** MEDIUM

### Gaps to Address

- RESOLVED: bus factor is now defined in `research/METRICS.md` as a deterministic integer metric (commit-count, >=50%, bots/merges excluded, identities normalized).
- RESOLVED: renames are intentionally NOT tracked in V1; "most modified file" and "oldest file" use simple path-based history.
- Large-repo handling thresholds (commit caps for expensive passes) should be decided during phase planning.

## Sources

### Primary (HIGH confidence)
- https://docs.rs/crate/clap/latest — CLI stack
- https://docs.rs/crate/git2/latest — git inspection stack
- https://docs.rs/crate/reqwest/latest — HTTP stack
- https://docs.rs/crate/ratatui/latest — TUI stack
- https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api — GitHub API constraints
- https://docs.github.com/en/rest/branches/branches?apiVersion=latest — branch collection
- https://docs.github.com/en/rest/git/trees — large-tree constraints

### Secondary (MEDIUM confidence)
- User-provided MVP brief and exclusions — feature and roadmap shaping

### Tertiary (LOW confidence)
- Inferred heuristics for vibe and verdict sections — must be validated during implementation

---
*Research completed: 2026-06-02*
*Ready for roadmap: yes*
