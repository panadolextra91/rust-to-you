# Phase 1: Intake & Guardrails - Context

**Gathered:** 2026-06-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Accept exactly one public GitHub repo URL via `rust-to-you <repo-url>`, validate it
**from the string alone** (no network), reject unsupported/malformed input with clear
crab-voiced errors, and bootstrap a read-only investigation session.

**In scope:** CLI surface (single positional arg), URL parsing → normalized `RepoRef`,
the full error taxonomy (including network-dependent variants defined but not yet wired),
exit-code mapping, and a successful-intake "session + stub line" seam.

**Explicitly NOT in this phase:** any network call, cloning, GitHub API access, or
collection/analysis logic. Those are Phase 2. Phase 1 must run fully offline and sync.
</domain>

<decisions>
## Implementation Decisions

### URL Intake
- **D-01:** Generous URL acceptance. Parse and normalize ALL of:
  - `https://github.com/owner/repo` (+ optional `.git`, + optional trailing `/`)
  - scheme-less `github.com/owner/repo` (normalize to https)
  - deep links like `https://github.com/owner/repo/tree/main/src/...` → take the first
    two path segments (`owner`, `repo`)
  - bare shorthand `owner/repo` (assume github.com)
- **D-02:** All accepted forms normalize to a single `RepoRef { owner, repo }`; host is
  always github.com in V1. The canonical clone URL `https://github.com/{owner}/{repo}.git`
  is derived from `RepoRef` (consumed in Phase 2, not used in Phase 1).

### Error Handling & Scope
- **D-03:** Phase 1 is **syntactic-only — ZERO network calls.** The error enum DEFINES the
  network-dependent variants now (`RepoNotFoundOrPrivate`, `Network`, `RateLimited`) so the
  taxonomy is stable, but Phase 2 populates them at runtime. Phase 1 only ever emits the
  syntactic variants (`EmptyInput`, `NotAUrl`, `UnsupportedHost`, `MalformedRepoPath`).
- **D-04:** Not-found vs private uses ONE combined, honest message ("private **or** doesn't
  exist"). Never claim to distinguish them — unauthenticated GitHub returns 404 for both, so
  distinguishing is not possible and would be a lie.

### Failure UX
- **D-05:** Tiered exit codes: `0` success · `2` input/syntactic error · `3`
  not-found/private (Phase 2) · `4` network/rate-limit (Phase 2). Lets scripts/CI tell error
  classes apart and is stable for v2.
- **D-06:** Error messages are crab-voiced **and actionable** (include a fix example, e.g.
  "try: `rust-to-you github.com/owner/repo`"), written to **stderr**.

### Success Seam
- **D-07:** A successful intake builds `InvestigationSession { repo, case_id, started_at }`,
  prints ONE stub line — `🦀 Investigation opened: {owner}/{repo} — Case {CASE_ID}` — and
  exits 0. This printed `run(session)` path is the explicit seam Phase 2 (walking skeleton)
  plugs collection into.
- **D-08:** `case_id` is **deterministic**: `{REPO_UPPER truncated ~4}-{4 hex chars of a
  stable hash of "owner/repo"}` (e.g. `AXUM-7F42`). Same repo → same id → unit-testable; no
  randomness, no time input.

### Carried Forward (locked earlier — do NOT re-litigate)
- Public GitHub only · read-only · single command `rust-to-you <url>` (PROJECT.md).
- **No `tokio`** — Phase 1 is fully synchronous; `reqwest` (blocking) only appears in Phase 2.
- Module layout follows research/ARCHITECTURE.md (`src/cli/`, `src/app/`).

### Claude's Discretion
- Ergonomic crate choices are the planner/executor's call, kept **minimal and sync**:
  `thiserror` for the error enum and the `url` crate for robust parsing are recommended but
  not mandated (hand-rolling is acceptable if deps are a concern).
- Exact non-crypto hash function for `case_id` (any stable hash is fine).
- Internal file split within `src/cli/` and `src/app/` (e.g. `error.rs` placement).
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope & requirements
- `.planning/ROADMAP.md` § "Phase 1: Intake & Guardrails" — goal, success criteria, plans 01-01 / 01-02
- `.planning/REQUIREMENTS.md` — INPT-01 (single-command intake), INPT-02 (clear read-only errors)

### Locked project decisions
- `.planning/PROJECT.md` § "Key Decisions" — public-only, read-only, no-tokio, single-command (all ✅ Decided)

### Architecture & stack (constrain HOW)
- `.planning/research/ARCHITECTURE.md` § "Recommended Project Structure" — `src/cli/`, `src/app/`, pipeline pattern, the `run()` seam
- `.planning/research/STACK.md` — `clap` (derive); confirms NO `tokio`; `reqwest` blocking is Phase 2 only

### Downstream seam (informational — Phase 2 consumes Phase 1)
- `.planning/research/SUMMARY.md` § "Phase 2" — walking skeleton plugs into the Phase 1 success seam (D-07)
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- None — greenfield. No `src/`, no `Cargo.toml` yet. Plan 01-01 scaffolds the Cargo project.

### Established Patterns
- Target layout is pre-decided in research/ARCHITECTURE.md: `src/main.rs` (entrypoint) →
  `src/cli/` (clap + URL parsing) → `src/app/` (session + `run()` seam). Errors centralized
  (e.g. `src/error.rs`) so Phase 2+ can extend the same taxonomy.

### Integration Points
- `run(session)` (D-07) is the single seam Phase 2 collection attaches to.
- The error enum (D-03) is the shared taxonomy Phase 2 extends with live network variants.
</code_context>

<specifics>
## Specific Ideas

Crab-voiced error wording agreed during discussion (starting drafts, tunable):
- Empty input → "🦀 Cho mình cái URL repo nào: `rust-to-you https://github.com/owner/repo`"
- Not a URL → "🦀 Cái này không giống URL GitHub. Thử `github.com/owner/repo` nha"
- Unsupported host → "🦀 V1 chỉ hóng repo GitHub thôi — `{host}` chưa có trong danh sách khách mời"
- Malformed path → "🦀 Mình cần `owner/repo`, kiểu `github.com/tokio-rs/axum`"
- Not-found/private (Phase 2) → "🦀 Không với tới `{owner}/{repo}` — private hoặc không tồn tại. Mình chỉ làm repo public"

Success stub: `🦀 Investigation opened: tokio-rs/axum — Case AXUM-7F42`

URL parser is the highest-value test surface → table-driven unit tests covering every
accept/reject case above.
</specifics>

<deferred>
## Deferred Ideas

- **SSH `git@github.com:owner/repo.git` URL form** → v2. Public HTTPS clone covers V1; SSH
  adds parsing surface without V1 value.
- **Distinguishing private vs. not-found** → not feasible unauthenticated (both 404);
  revisit only if/when auth lands (v2 EXPD-03).
- **Actual network/existence validation** → Phase 2 (Collection Layer) populates the
  pre-defined error variants.

None of the above are scope creep into Phase 1 — they are intentional boundaries.
</deferred>

---

*Phase: 1-Intake & Guardrails*
*Context gathered: 2026-06-02*
