# Phase 2: Collection Layer - Context

**Gathered:** 2026-06-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Collect every piece of evidence the report needs: a single minimal GitHub API metadata
call (stars/forks/description), a **full clone** of the repo, git history + branch facts via
git2, and filesystem/infrastructure signals — then normalize all of it into one
`InvestigationSnapshot`. Includes a throwaway **walking skeleton** (02-01) that proves the
clone→metric→output pipeline end-to-end before the real collectors are built.

**In scope:** API metadata client (one call), full clone into a temp workspace, git-history
collectors (commits/branches/contributors/file ages), filesystem + infra scanners, language
detection, snapshot normalization, walking skeleton.

**Explicitly NOT in this phase:** the analysis/metric computation (Phase 3), the TUI (Phase 4),
narrative/vibes (Phase 5), caching/offline/--deep (v2), PR/issue/auth (v2).
</domain>

<decisions>
## Implementation Decisions

### Clone Workspace
- **D-01:** Full clone goes into a **unique temp dir** with **RAII auto-clean** — deleted on
  normal exit AND on error/panic (recommend the `tempfile` crate). **No caching in V1** (cache
  is a v2 feature). Preserves the read-only guarantee: never write to the remote; the workspace
  is owned and always cleaned up.

### Collection Flow & Failure Handling
- **D-02:** The GitHub API metadata call (`GET /repos/{owner}/{repo}`) runs **FIRST, before the
  clone.** This is where Phase 1's reserved `IntakeError` variants finally get populated:
  - HTTP 404 → `RepoNotFoundOrPrivate` → **ABORT** (exit 3) — can't clone a private/nonexistent repo anyway.
  - network error → `Network`; HTTP 403/rate-limit → `RateLimited`.
- **D-03:** Failure policy is **abort-on-404, degrade-on-transient**: a 404 aborts; a transient
  `Network`/`RateLimited` failure **on the metadata call** does NOT kill the run — proceed to
  clone and mark stars/forks/description as **unavailable/"unknown"** in the snapshot. (Rationale:
  the report is mostly git-derived; one flaky metadata call shouldn't sink an otherwise-fine
  investigation.) A failure of the **clone itself** is a hard error (no repo = nothing to report).
- **D-04:** Use a `GITHUB_TOKEN` env var **if present** (future-proof / higher limit); otherwise
  run unauthenticated. Either way rate limits are a non-issue for a single call.

### History Window (resolves the OPEN item in METRICS.md / STATE.md)
- **D-05:** Expensive passes — **most-modified-file** and **time-of-day signals** (`night_pct`,
  `weekend_pct`, `business_hours_pct`) — are bounded to the **most recent 1000 commits**. If the
  repo has ≤1000 commits, use all. When capped, the result is **labeled "based on last N commits."**
- **D-06:** Cheap full-history aggregations are **NOT** bounded — they walk the whole history
  because they never diff: total commits, contributor counts, bus factor, repo age, branch
  enumeration + branch last-commit dates.

### Walking Skeleton (02-01)
- **D-07:** Skeleton = full-clone the input repo → compute **repo age** (from the first commit
  date) → print **one plain-text line** via the existing Phase 1 `run(session)` seam. Throwaway
  output — **NOT** ratatui, not the final renderer. Purpose: prove the clone → git2 → metric →
  output pipeline runs end-to-end early. It is explicitly allowed to be replaced/refactored by
  the real collectors in 02-03 / 02-04.

### Claude's Discretion
- **Infra detection method:** path/glob **presence** checks (no content parsing in V1). The
  signal LIST is locked by REQUIREMENTS (Docker, Terraform, GitHub Actions, GitLab CI, CircleCI,
  Jenkins, Dependabot, Renovate); typical paths: `Dockerfile`/`docker-compose.y*ml`,
  `*.tf`, `.github/workflows/`, `.gitlab-ci.yml`, `.circleci/config.yml`, `Jenkinsfile`,
  `.github/dependabot.yml`, `renovate.json`/`.github/renovate.json`.
- **Language Soup:** use the **`tokei` crate** (library, line-count based) rather than a custom counter.
- **`InvestigationSnapshot` internal shape** (sub-structs like RepoMetadata / HistoryFacts /
  BranchFacts / FilesystemFacts) — planner's call, per research/ARCHITECTURE.md.
- **Default branch** comes from the clone's HEAD (git), not the API.

### Carried Forward (locked earlier — do NOT re-litigate)
- Full clone (not shallow) · git2 for ALL local data · API only for stars/forks/description ·
  `reqwest` **blocking**, NO `tokio` · no rename tracking · **bot + identity filtering shared**
  across contributor_count/top_author_share/bus_factor (research/METRICS.md) · normalize into one
  `InvestigationSnapshot` (research/ARCHITECTURE.md) · builds on Phase 1's `run()` seam, `RepoRef`,
  and `IntakeError`.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope & requirements
- `.planning/ROADMAP.md` § "Phase 2: Collection Layer" — goal, success criteria, plans 02-01..02-04
- `.planning/REQUIREMENTS.md` — COLL-01 (overview metadata), COLL-02 (git/branch evidence + snapshot), COLL-03 (language + infra signals)

### Locked project decisions & metrics
- `.planning/PROJECT.md` § "Key Decisions" — full clone, minimal API, no-tokio, bot/identity filtering
- `.planning/research/METRICS.md` — shared author identity/bot/merge filtering; bus factor; bounded-window note (now resolved here as 1000)

### Architecture & stack (constrain HOW)
- `.planning/research/ARCHITECTURE.md` — `InvestigationSnapshot` normalization, `github/`+`repo/`+`scan/` module split, the pipeline pattern
- `.planning/research/STACK.md` — `git2` (full clone), `reqwest` blocking, `chrono`, `tokei` for Language Soup; NO tokio
- `.planning/research/PITFALLS.md` — Pitfall 1 (bound expensive passes → D-05/D-06), Pitfall 2 (single API call), Pitfall 5 (no over-collecting)

### Phase 1 artifacts to build ON (seam + reused symbols)
- `.planning/phases/01-intake-guardrails/01-CONTEXT.md` — the run() seam contract, RepoRef, IntakeError taxonomy
- `src/app/run.rs` (the `run(&session)` seam), `src/app/session.rs` (`InvestigationSession`), `src/cli/parse.rs` (`RepoRef`), `src/error.rs` (`RepoNotFoundOrPrivate`/`Network`/`RateLimited` to populate)
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets (from Phase 1 — verified, on `main`)
- `RepoRef { owner, repo }` (`src/cli/parse.rs`) — the validated input; derive the clone URL `https://github.com/{owner}/{repo}.git` from it.
- `InvestigationSession { repo, case_id, started_at }` (`src/app/session.rs`) — collection output should attach to / flow alongside the session.
- `run(&session)` (`src/app/run.rs`) — the integration seam; the walking skeleton (D-07) plugs the clone+age into here.
- `IntakeError::{RepoNotFoundOrPrivate, Network, RateLimited}` + `exit_code()` (`src/error.rs`) — defined but unused in Phase 1; **Phase 2 populates them** (D-02), clearing the existing dead_code warnings.

### Established Patterns
- Module layout (`src/cli/`, `src/app/`, `src/error.rs`) — extend with collection modules per ARCHITECTURE.md (`github/`, `repo/`, `scan/`).
- Pipeline in `main.rs`: `parse → session → run`. Phase 2 inserts `collect → snapshot` before/within `run`.

### Integration Points
- `run(session)` is where collection attaches.
- The error taxonomy is the shared contract — populate, don't redefine.
- `started_at` (currently dead_code) may begin to be used for elapsed-time reporting.
</code_context>

<specifics>
## Specific Ideas

- Walking skeleton output is a single throwaway line (e.g. via the existing `🦀 Investigation
  opened: …` seam, extended to also print the computed repo age) — proof-of-pipeline, not polish.
- "based on last N commits" labeling must travel with any metric derived from the bounded window
  so Phase 3/4 can surface the caveat honestly.
- Degraded metadata (stars/forks unavailable) should be representable in the snapshot as an
  explicit "unknown", not silently zero.
</specifics>

<deferred>
## Deferred Ideas

- **Caching / offline mode (`--cache`, `--offline`)** → v2. V1 always clones fresh into temp.
- **`--deep` unbounded history** → v2 (lift the 1000-commit cap from D-05).
- **Distinguishing private vs not-found** → needs auth; v2 (EXPD-03).
- **Content-parsing infra configs** (vs presence detection) → not needed for V1 footprint flags.

None of the above is scope creep into Phase 2 — they are intentional boundaries.
</deferred>

---

*Phase: 2-Collection Layer*
*Context gathered: 2026-06-02*
