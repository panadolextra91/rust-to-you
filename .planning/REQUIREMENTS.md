# Requirements: rust-to-you

**Defined:** 2026-06-02
**Core Value:** Given one public GitHub repository URL, produce a cute, readable TUI investigation report faster than manually digging through the GitHub UI.

## v1 Requirements

### Intake

- [ ] **INPT-01**: User can run `rust-to-you <repo-url>` against a public GitHub repository and start an investigation from that single command.
- [ ] **INPT-02**: User gets a clear read-only error when the input URL is invalid, unsupported, private, or otherwise unusable for V1.

### Collection

- [ ] **COLL-01**: Tool collects repository metadata needed for First Impressions, including age, default branch, stars, forks, contributors, and last activity.
- [ ] **COLL-02**: Tool gathers enough git and branch evidence to support Commit Crimes, Branch Jungle, and Ancient Relics calculations.
- [ ] **COLL-03**: Tool inspects repository files and config locations needed for Language Soup and Infrastructure Footprints without mutating the remote repository.

### Analysis

- [ ] **ANLY-01**: Tool computes Commit Crimes values for total commits, commits this month, top contributor, and estimated bus factor.
- [ ] **ANLY-02**: Tool computes Branch Jungle values for total branches, active branches, stale branches, and oldest branch.
- [ ] **ANLY-03**: Tool computes Ancient Relics values for oldest file, most modified file, oldest contributor, and longest living branch.
- [ ] **ANLY-04**: Tool computes Language Soup percentages and renders them as report-ready values with ASCII progress bars.
- [ ] **ANLY-05**: Tool detects Docker, Terraform, GitHub Actions, GitLab CI, CircleCI, Jenkins, Dependabot, and Renovate signals for Infrastructure Footprints.

### Narrative

- [ ] **NARR-01**: Tool classifies Repository Vibes from observed repository signals and shows evidence bullets that justify the classification.
- [ ] **NARR-02**: Tool generates Interesting Findings bullets and a Crab Verdict summary with strengths and risks derived from computed evidence.

### Presentation

- [ ] **PRES-01**: Tool renders a single scrollable vertical TUI report with header, repository identity, investigation date, case ID, and the nine MVP sections in order.
- [ ] **PRES-02**: Report styling stays cute and readable in a terminal, including crab iconography, without tabs or multi-screen navigation.

## v2 Requirements

### Output & Runtime Modes

- **MODE-01**: User can request `--json` output for machine-readable investigations.
- **MODE-02**: User can reuse cached investigations for repeat runs.
- **MODE-03**: User can opt into deeper history analysis when they want slower but richer archaeology.
- **MODE-04**: User can run an offline mode against already-fetched local data.

### Expanded Coverage

- **EXPD-01**: Tool can analyze pull requests.
- **EXPD-02**: Tool can analyze issues.
- **EXPD-03**: Tool can authenticate to inspect private repositories.
- **EXPD-04**: Tool can support GitLab and Bitbucket repositories.

## Out of Scope

| Feature | Reason |
|---------|--------|
| PR analysis | Explicitly excluded from V1 to keep scope on repository-level investigation |
| Issue analysis | Explicitly excluded from V1 to avoid expanding API surface too early |
| Security scanning | Not core to the playful read-only report experience |
| Dependency graph generation | Valuable later, but not required for launch validation |
| AI-generated architecture review | Too subjective for a grounded V1 report |
| Authentication/private repos | Public-repo-only launch keeps setup and trust simple |
| GitLab/Bitbucket support | GitHub-only scope reduces collector complexity |
| Tabbed or multi-screen TUI | Conflicts with the desired one-scroll report experience |
| Any write action | V1 must remain read-only |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| INPT-01 | Phase 1 | Pending |
| INPT-02 | Phase 1 | Pending |
| COLL-01 | Phase 2 | Pending |
| COLL-02 | Phase 2 | Pending |
| COLL-03 | Phase 2 | Pending |
| ANLY-01 | Phase 3 | Pending |
| ANLY-02 | Phase 3 | Pending |
| ANLY-03 | Phase 3 | Pending |
| ANLY-04 | Phase 3 | Pending |
| ANLY-05 | Phase 3 | Pending |
| PRES-01 | Phase 4 | Pending |
| PRES-02 | Phase 4 | Pending |
| NARR-01 | Phase 5 | Pending |
| NARR-02 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 14 total
- Mapped to phases: 14
- Unmapped: 0 ✓

---
*Requirements defined: 2026-06-02*
*Last updated: 2026-06-02 after initial definition*
