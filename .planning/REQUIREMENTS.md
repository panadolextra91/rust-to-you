# Requirements: rust-to-you

**Defined:** 2026-06-04
**Milestone:** v1.2.0 — Robustness & Safety Hardening
**Core Value:** Given one public GitHub repository URL, produce a cute, readable TUI investigation report faster than manually digging through the GitHub UI.

> v1.0 requirements (INPT / COLL / ANLY / NARR / PRES) shipped and are archived in
> `.planning/milestones/v1.0-REQUIREMENTS.md`. This file scopes the v1.2.0 milestone.

## v1.2.0 Requirements

Requirements for the Robustness & Safety Hardening milestone. Each maps to a roadmap phase.

### Guard

- [x] **GUARD-01**: User is blocked (with a warning) before the tool clones a repository larger than a safe size threshold, using a pre-flight check against the GitHub API so their machine never hangs unexpectedly.
- [x] **GUARD-02**: User sees a clear bilingual (VI+EN) message stating the repository is too large, showing its actual size, and explaining how to proceed with `--deep`.
- [x] **GUARD-03**: User can pass `--deep` to opt into full analysis of a large repository, accepting the longer runtime.

### Security

- [x] **SEC-01**: User's malformed or malicious repo input is rejected safely — owner/repo segments beginning with `-` are blocked and segment lengths are capped to GitHub's limits.
- [x] **SEC-02**: The intake threat model is documented and covered by explicit injection/abuse tests.

### Cleanup

- [x] **CLEAN-01**: User interrupting a run (Ctrl-C / SIGINT / SIGTERM) never leaves an orphaned clone temp directory behind.
- [x] **CLEAN-02**: On startup, the tool sweeps away orphaned temp directories left by previously crashed or killed runs.
- [x] **CLEAN-03**: No code path exits the process (or aborts on panic) while a clone workspace is alive without cleaning it up first.

## Future Requirements

Deferred to a future release. Tracked but not in the current roadmap.

### Output & Runtime Modes

- **MODE-01**: User can request `--json` output for machine-readable investigations.
- **MODE-02**: User can reuse cached investigations for repeat runs.
- **MODE-04**: User can run an offline mode against already-fetched local data.
- **GUARD-04** *(deferred)*: `--deep` runs are bounded by a time/commit budget so even an enormous repo cannot hang indefinitely. Deferred — refuse-by-default already bounds the common case.

### Expanded Coverage

- **EXPD-01**: Tool can analyze pull requests.
- **EXPD-02**: Tool can analyze issues.
- **EXPD-03**: Tool can authenticate to inspect private repositories.
- **EXPD-04**: Tool can support GitLab and Bitbucket repositories.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Shallow/blobless clone in default mode | Archaeology metrics need full history; refuse-by-default + `--deep` handles size instead |
| PR / issue analysis | Still deferred — milestone is hardening, not expanded coverage |
| Security scanning of target repos | This milestone hardens rust-to-you itself, not the repos it inspects |
| Auth / private repos / other hosts | Public-GitHub-only scope unchanged from v1.0 |
| Any write action | Tool remains read-only |

## Traceability

Populated during roadmap creation. Phase numbering continues from v1.0 (last phase = 5).

| Requirement | Phase | Status |
|-------------|-------|--------|
| GUARD-01 | Phase 6 | Complete |
| GUARD-02 | Phase 6 | Complete |
| GUARD-03 | Phase 6 | Complete |
| SEC-01 | Phase 6 | Complete |
| SEC-02 | Phase 6 | Complete |
| CLEAN-01 | Phase 7 | Complete |
| CLEAN-02 | Phase 7 | Complete |
| CLEAN-03 | Phase 7 | Complete |

**Coverage:**

- v1.2.0 requirements: 8 total
- Mapped to phases: 8
- Unmapped: 0 ✓

---
*Requirements defined: 2026-06-04*
*Last updated: 2026-06-04 after milestone v1.2.0 definition*
