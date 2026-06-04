# Phase 6: Safe Intake & Pre-flight Guard - Context

**Gathered:** 2026-06-04
**Status:** Ready for planning

<domain>
## Phase Boundary

Before any clone happens, the tool must (1) refuse repositories larger than a safe
size threshold via a pre-flight GitHub API size check, and (2) reject malformed /
malicious inputs at the parser before any network or git2 operation runs. Refusals
are communicated with clear bilingual (VI+EN), Ferris-narrated messages, and the
size guard has an explicit `--deep` escape hatch. The intake threat model is
documented and backed by injection/abuse tests.

**In scope:** pre-flight size guard + `--deep` flag, parser hardening (leading-dash
rejection + segment length caps), bilingual refusal messaging, threat-model doc +
injection tests.

**Out of scope (own phases / deferred):** SIGINT/SIGTERM cleanup and orphaned-temp
hygiene (Phase 7 / CLEAN-*), time/commit budget bounding of `--deep` runs (GUARD-04,
deferred), any new analysis behavior gated behind `--deep` beyond the size bypass.

</domain>

<decisions>
## Implementation Decisions

### Size threshold (GUARD-01)
- **D-01:** Threshold is **500 MB**. GitHub's API `size` field is in **KB**, so the
  comparison is `size_kb > 500 * 1024`.
- **D-02:** Threshold is a **hardcoded constant** in code — no env var, no flag to
  tune it. `--deep` is the only escape hatch. Keeps the surface minimal ("tighten +
  document, not a new subsystem").
- **D-03:** `RepoMetadata` (src/github/client.rs) currently has **no `size` field** —
  add `pub size: u64` (GitHub returns it as an integer KB count) so the guard can read it.

### Pre-flight check placement (GUARD-01)
- **D-04:** The size check runs in `src/app/collect.rs` **after `fetch_metadata`
  succeeds and before `clone_repo`** — the machine never starts an oversized clone.
- **D-05:** The check only fires when metadata is `Available`. The existing
  abort-on-404 / degrade-on-transient structure is preserved.

### `--deep` behavior (GUARD-03)
- **D-06:** Add a `--deep` boolean flag to clap `Args` (src/cli/args.rs), threaded
  through to the collection path (e.g., via `InvestigationSession` or a `collect`
  parameter — planner's discretion on the exact plumbing).
- **D-07:** On a large repo **with** `--deep`: print a one-shot **bilingual warning**
  ("repo lớn (X MB), Ferris vẫn đào vì --deep — sẽ lâu đó" / EN line) then proceed
  to clone. Not a silent bypass — the user is told they're in for a long run.
- **D-08:** `--deep` in this phase **only** bypasses the size guard. It does not
  unlock any other behavior (the deferred blame-based truck factor stays out of scope).

### Unknown-size behavior (GUARD-01 edge)
- **D-09:** When size is unavailable (metadata fetch failed → `RepoMetaState::Unavailable`,
  e.g. rate-limit / no token / transient network), **warn then clone** (fail-open with
  notice): print a bilingual line ("không biết kích thước repo, Ferris cứ đào nha" / EN)
  and proceed. This matches the existing degrade-on-transient path in collect.rs and
  preserves the tool's core value when the API is flaky. The guard never blocks on
  missing data.

### Parser hardening (SEC-01)
- **D-10:** Reject any owner/repo **segment beginning with `-`** (argument-injection
  guard). Note: `is_valid_segment` currently *allows* `-` anywhere, so a `-foo` segment
  passes today — this must be tightened.
- **D-11:** Cap segment lengths to **GitHub's documented limits**: owner ≤ 39 chars,
  repo ≤ 100 chars. Reject longer segments before any network/git2 call.
- **D-12:** Keep all existing parser guards (`..`, `\`, control chars, charset
  allowlist, 2048 overall cap).

### Error UX & exit codes (GUARD-02, SEC-01)
- **D-13:** Add a dedicated `IntakeError::RepoTooLarge { size_mb, threshold_mb }`
  variant with a **new exit code 6**. Its bilingual message states the repo is too
  large, shows the **actual size**, and explains how to proceed with `--deep`.
- **D-14:** Add a dedicated variant for **unsafe input segments** (leading-dash and/or
  over-length) — distinct from the generic `MalformedRepoPath` — so Ferris gives a clear
  "input này kỳ lạ / không an toàn, Ferris không nhận" message. Exit code stays in the
  input-error class (2), consistent with other parse errors. (Exact variant name and
  whether over-length shares this variant vs `MalformedRepoPath` is planner's discretion,
  but leading-dash MUST get the distinct message.)

### Threat model documentation (SEC-02)
- **D-15:** Document the intake threat model in **`docs/THREAT-MODEL.md`** (standalone,
  version-controlled). It must cover the intake attack surface: arg-injection via
  leading `-`, path traversal (`..`), backslash, control chars, and oversize-clone DoS —
  and explicitly note that `git2` is libgit2 FFI (no shell spawn), so the injection
  surface is narrow by construction.
- **D-16:** Back the threat model with **explicit injection/abuse tests** proving that
  malformed inputs are rejected at the parser before reaching the git2 / network surfaces
  (i.e., `parse_repo_ref` rejects them; they never reach `fetch_metadata` / `clone_repo`).

### Claude's Discretion
- Exact plumbing of `--deep` from `Args` to `collect` (session field vs function param).
- Exact name of the unsafe-input error variant and whether over-length reuses it or
  `MalformedRepoPath`.
- Whether the warning/notice lines (D-07, D-09) go through the existing `i18n::two_line`
  helper (preferred for consistency) — confirm at plan time.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope & requirements
- `.planning/ROADMAP.md` — Phase 6 section (goal, 5 success criteria, GUARD/SEC reqs)
- `.planning/REQUIREMENTS.md` §Guard, §Security — GUARD-01/02/03, SEC-01/02 text
- `.planning/PROJECT.md` §Key Decisions — locked: refuse-by-default + `--deep` (2026-06-04);
  intake security = "tighten + document", git2 is libgit2 FFI no shell spawn (2026-06-04);
  bilingual VI+EN two-line; Ferris third-person narrator

No external specs/ADRs beyond the planning docs — requirements fully captured in the
decisions above.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/i18n.rs` — `bi()`, `two_line()`, `inline_label()` bilingual helpers; use
  `two_line` for every new VI+EN message (size refusal, --deep warning, unknown-size notice).
- `src/error.rs` — `IntakeError` enum (thiserror) with bilingual `#[error(...)]` strings
  and `exit_code()`; new variants follow this exact pattern. Existing codes: 2 (input),
  3 (not found), 4 (network/rate-limit), 5 (collection).
- `src/cli/parse.rs` — `parse_repo_ref` + `is_valid_segment`; already blocks `..`, `\`,
  control chars, non-github hosts, applies charset allowlist. Has accept/reject test tables
  to extend.

### Established Patterns
- Pre-clone GitHub metadata fetch already exists: `src/app/collect.rs` calls
  `fetch_metadata(&session.repo)` first, with abort-on-404 / degrade-on-transient, THEN
  `clone_repo`. The size guard slots into this gap — no new fetch needed.
- `RepoMetadata` (src/github/client.rs) is serde `Deserialize`; adding `size` is a
  one-field change + a decode test update.
- clap `Args` (src/cli/args.rs) is a single-field struct today; `--deep` is the first flag.
  `main.rs` parses args → `parse_repo_ref` → `InvestigationSession::new` → `app::run`.

### Integration Points
- `Args.deep` (new) → `InvestigationSession` / `collect` → size-guard branch in `collect.rs`.
- `RepoMetadata.size` (new) → read in `collect.rs` size check.
- New `IntakeError` variants → surfaced via existing `eprintln!("{}", e)` + `exit_code()`
  paths in `main.rs` and `app::run`.

</code_context>

<specifics>
## Specific Ideas

- Size refusal message must show the **actual size in MB** and name the `--deep` flag
  explicitly (GUARD-02 success criterion).
- All new user-facing lines stay in Ferris's voice: third-person "Ferris", never "tôi/mình".

</specifics>

<deferred>
## Deferred Ideas

- **GUARD-04** — bounding `--deep` runs by a time/commit budget so even an enormous repo
  cannot hang indefinitely. Explicitly deferred in REQUIREMENTS.md (refuse-by-default
  already bounds the common case).
- **CLEAN-01/02/03** — SIGINT/SIGTERM cleanup + orphaned-temp sweep belong to Phase 7,
  not this phase.

</deferred>

---

*Phase: 06-safe-intake-pre-flight-guard*
*Context gathered: 2026-06-04*
