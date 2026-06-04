# Phase 6: Safe Intake & Pre-flight Guard - Research

**Researched:** 2026-06-04
**Domain:** Rust CLI intake hardening — pre-flight size guard (GitHub REST), parser input validation, bilingual error UX, threat-model documentation
**Confidence:** HIGH

## Summary

This phase is a "tighten + document" hardening pass on an existing, working intake/collection
pipeline — not a new subsystem. Every integration point named in CONTEXT.md was confirmed against
the actual source: the `fetch_metadata → clone_repo` gap in `src/app/collect.rs` exists exactly as
described, `RepoMetadata` (`src/github/client.rs`) has no `size` field today, `is_valid_segment`
(`src/cli/parse.rs`) does currently allow a leading `-`, `clap Args` (`src/cli/args.rs`) is a
single-field struct, and `IntakeError` (`src/error.rs`) is a thiserror enum with bilingual
`#[error(...)]` strings and an `exit_code()` matcher using codes 2/3/4/5. The `i18n` helpers
(`bi`, `two_line`, `inline_label`) are present and already used in `run.rs` and `collect.rs`.

All five external/factual claims the planner needs were verified: GitHub's `size` field is a
**required integer in kilobytes** [CITED], owner ≤ **39** chars and repo ≤ **100** code points are
GitHub's documented limits [VERIFIED: github-limits], and `git2`/libgit2 binds via FFI with **no
shell/subprocess spawn** — confirmed both by the libgit2 design and by a `grep` of the codebase
showing zero `std::process::Command` usages [VERIFIED: codebase grep]. The 500 MB → `size_kb > 500 * 1024`
comparison in D-01 is arithmetically correct (500 MB = 512000 KB).

**Primary recommendation:** Implement as four small, independently-testable changes that follow the
existing patterns exactly: (1) add `pub size: u64` to `RepoMetadata` + decode test; (2) add a
size-guard branch in `collect.rs` between the metadata match and `clone_repo`, gated on `--deep`;
(3) tighten `is_valid_segment`/`parse_repo_ref` to reject leading-dash and over-length segments with
a dedicated error; (4) add `IntakeError::RepoTooLarge` (exit code **6**) plus an unsafe-input variant
(exit code **2**), `docs/THREAT-MODEL.md`, and an injection test suite. Use `i18n::two_line` for every
new user-facing line (D-07/D-09 notices and the `RepoTooLarge` message).

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Reject malformed/injection input | CLI parser (`src/cli/parse.rs`) | — | Must reject before any network/git2 call; pure string validation, no I/O |
| Pre-flight size check | App orchestration (`src/app/collect.rs`) | GitHub client (`src/github/client.rs`) | Decision logic lives in collect; `size` field is sourced from the metadata API client |
| `--deep` flag intake | CLI args (`src/cli/args.rs`) | App session/orchestration | clap owns flag parsing; the boolean is threaded to the collect decision point |
| Bilingual refusal/warning messaging | i18n + error (`src/i18n.rs`, `src/error.rs`) | — | Error strings live in `IntakeError`; notices use `i18n::two_line` |
| Threat-model documentation | Docs (`docs/THREAT-MODEL.md`) | Tests (`tests/` or `#[cfg(test)]`) | Standalone version-controlled doc, backed by injection tests |

## Standard Stack

This phase adds **zero new dependencies**. Everything needed already exists in `Cargo.toml`.

### Core (already present, verified in Cargo.lock)
| Library | Version (locked) | Purpose | Why Standard |
|---------|------------------|---------|--------------|
| `clap` | 4.6.1 (req `4.4`, derive) | `--deep` boolean flag on `Args` | Already the CLI parser; derive macro adds a flag in one line |
| `git2` | 0.21.0 (https, vendored-openssl) | Clone via libgit2 FFI | No shell spawn — narrow injection surface (threat-model claim) |
| `reqwest` | 0.11.27 (blocking, json, rustls-tls) | GitHub metadata fetch (already wired) | Existing `fetch_metadata` reads JSON; `size` is one more serde field |
| `serde` / `serde_json` | 1.x | Deserialize `RepoMetadata` incl. new `size` | `RepoMetadata` is already `#[derive(Deserialize)]` |
| `thiserror` | 1.0 | New `IntakeError` variants w/ bilingual `#[error]` | Existing error pattern — new variants drop straight in |

**Installation:** None. `cargo build` only.

**Version verification:** Confirmed against `Cargo.lock` (locked versions above) — no registry lookup
needed because no packages are added. `npm/pip/cargo` install steps are N/A for this phase.

## Package Legitimacy Audit

> Not applicable. This phase installs **no external packages** — it is a code/config/docs change on an
> existing dependency set. slopcheck / registry verification skipped by design (nothing to verify).

## Architecture Patterns

### System Architecture Diagram (intake → guard → clone data flow)

```
  user input (String)              --deep (bool, new)
        │                                │
        ▼                                │
  parse_repo_ref()  ◄── HARDEN HERE (SEC-01)        [src/cli/parse.rs]
   ├─ empty / control / >2048 / git@ / scheme  → IntakeError (exit 2)
   ├─ ".." / "\"                               → MalformedRepoPath (exit 2)
   ├─ NEW: segment starts with '-'             → UnsafeInput (exit 2)   ← D-10
   ├─ NEW: owner >39 / repo >100               → over-length reject     ← D-11
   └─ charset allowlist                        → MalformedRepoPath
        │ Ok(RepoRef)
        ▼
  InvestigationSession::new(repo)  [+ deep flag threaded]   [src/app/session.rs]
        │
        ▼
  app::run(session) → collect(session)                      [src/app/collect.rs]
        │
        ▼
  fetch_metadata(&repo)   [src/github/client.rs — reqwest GET /repos/{o}/{r}]
   ├─ Ok(meta)         → RepoMetaState::Available(meta)   (meta.size, KB, NEW field)
   ├─ Err(NotFound)    → abort: RepoNotFoundOrPrivate (exit 3)
   └─ Err(other)       → RepoMetaState::Unavailable  (degrade-on-transient)
        │
        ▼
  ┌─────────── SIZE GUARD (GUARD-01) — NEW BRANCH, runs BEFORE clone ──────────┐
  │  match meta_state:                                                          │
  │    Available(m) if m.size > 500*1024:                                       │
  │        if deep → two_line warning "repo lớn, vẫn đào" then fall through     │ D-07
  │        else    → return Err(RepoTooLarge{size_mb, threshold_mb}) (exit 6)   │ D-13
  │    Available(m) → proceed                                                   │
  │    Unavailable  → two_line "không biết kích thước, cứ đào" then proceed     │ D-09 (fail-open)
  └─────────────────────────────────────────────────────────────────────────────┘
        │
        ▼
  clone_repo(&repo)   [src/repo/clone.rs — git2::Repository::clone, libgit2 FFI, NO SHELL]
        │
        ▼
  history / branches / scan → InvestigationSnapshot → TUI render
```

The size guard slots into the existing gap. **No new fetch is added** — `fetch_metadata` already runs
pre-clone and its result is already matched in `collect.rs`.

### Existing Project Structure (relevant files only)
```
src/
├── cli/
│   ├── args.rs      # clap Args (single field today) — add `pub deep: bool`
│   ├── parse.rs     # parse_repo_ref + is_valid_segment — harden here
│   └── mod.rs       # re-exports Args, parse_repo_ref, RepoRef
├── app/
│   ├── session.rs   # InvestigationSession::new(repo) — candidate to carry `deep`
│   ├── collect.rs   # fetch_metadata → [GUARD] → clone_repo
│   ├── run.rs       # run(session) → collect(session); uses i18n::two_line already
│   └── mod.rs
├── github/
│   └── client.rs    # RepoMetadata (Deserialize) — add `pub size: u64`
├── repo/
│   └── clone.rs     # git2::Repository::clone — no shell (threat-model anchor)
├── error.rs         # IntakeError enum + exit_code() — add RepoTooLarge + UnsafeInput
├── i18n.rs          # bi(), two_line(), inline_label()
└── snapshot.rs      # RepoMetaState::Available(RepoMetadata) | Unavailable
docs/                # EMPTY today → create docs/THREAT-MODEL.md (D-15)
tests/               # EMPTY today → integration injection tests live here OR in #[cfg(test)]
```

### Pattern 1: Add a serde field with a default-safe decode test
**What:** Add `pub size: u64` to `RepoMetadata`. GitHub marks `size` **required** in API responses
[CITED], so a bare `pub size: u64` (no `#[serde(default)]`) is correct for real responses. However,
the **existing decode test JSON** (`src/github/client.rs` `decode()`) omits `size` — adding a
non-default `u64` will break that test. Either add `size` to the test fixtures (preferred, matches
real API) **or** mark `#[serde(default)]` for resilience. CONTEXT D-03 says "GitHub returns it as an
integer KB count" — treat as required, update the fixtures.
**When to use:** Any time a new field is added to a serde struct that has existing decode tests.
**Example:**
```rust
// src/github/client.rs — Source: existing RepoMetadata struct (verified)
#[derive(Deserialize, Debug)]
pub struct RepoMetadata {
    pub stargazers_count: u64,
    pub forks_count: u64,
    // ...
    pub created_at: Option<String>,
    pub size: u64, // NEW — kilobytes, required field per GitHub REST [CITED]
}
```

### Pattern 2: Bilingual error variant with exit code (existing pattern, verified)
**What:** New `IntakeError` variants follow the exact existing shape — bilingual two-line
`#[error("VI\nEN")]` and a `match` arm in `exit_code()`.
**Example:**
```rust
// src/error.rs — Source: existing IntakeError variants (verified)
#[error("🦀 Repo này to quá ({size_mb} MB), Ferris không đào tự động đâu — chạy lại với --deep nếu bạn chấp nhận chờ lâu\nThis repo is too large ({size_mb} MB). Ferris won't auto-dig — re-run with --deep if you accept the longer wait")]
RepoTooLarge { size_mb: u64, threshold_mb: u64 },

#[error("🦀 Input này kỳ lạ / không an toàn, Ferris không nhận\nThis input looks unsafe — Ferris won't take it")]
UnsafeInput { input: String }, // exact variant name = planner's discretion (D-14)

// in exit_code():
Self::EmptyInput | Self::NotAUrl { .. } | Self::UnsupportedHost { .. }
  | Self::MalformedRepoPath { .. } | Self::UnsafeInput { .. } => 2,  // D-14: stays in input class
Self::RepoTooLarge { .. } => 6,                                       // D-13: NEW code 6
```

### Pattern 3: Bilingual non-error notice via `i18n::two_line` (D-07, D-09)
**What:** The `--deep` warning and the unknown-size notice are NOT errors — print via `two_line`
exactly like `run.rs` does. Confirmed at plan time per CONTEXT discretion note — this is the
preferred path for consistency.
**Example:**
```rust
// src/app/collect.rs — Source: pattern from src/app/run.rs (verified)
let lines = crate::i18n::two_line(&crate::i18n::bi(
    format!("🦀 Repo lớn ({size_mb} MB), Ferris vẫn đào vì --deep — sẽ lâu đó", size_mb = size_mb),
    format!("Large repo ({size_mb} MB) — Ferris digs anyway because of --deep; this will take a while", size_mb = size_mb),
));
eprintln!("{}", lines[0]);
eprintln!("{}", lines[1]);
```
Note: `run.rs` uses `println!` for status, `collect.rs` uses `eprintln!` for the existing
degrade notice. Keep warnings/notices on **stderr** (`eprintln!`) to match the existing
degrade-on-transient line in `collect.rs`.

### Pattern 4: Harden `is_valid_segment` + length cap in `parse_repo_ref`
**What:** Two distinct tightenings. (a) Leading-dash: `is_valid_segment` today is
`s.chars().all(|c| alnum || '-' || '_' || '.')` — `-foo` passes. Add an explicit
`s.starts_with('-')` reject. (b) Over-length: cap owner ≤ 39, repo ≤ 100 in `parse_repo_ref`
(where owner vs repo identity is known — `is_valid_segment` alone can't tell which is which).
**Example:**
```rust
// src/cli/parse.rs — leading-dash guard (D-10)
fn is_valid_segment(s: &str) -> bool {
    if s.is_empty() || s.contains("..") || s.starts_with('-') { // '-' guard NEW
        return false;
    }
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

// in parse_repo_ref, after owner/repo identified (D-11):
if owner.len() > 39 { return Err(IntakeError::UnsafeInput { input: input.to_string() }); }
if repo_str.len() > 100 { return Err(IntakeError::UnsafeInput { input: input.to_string() }); }
// D-14: leading-dash MUST get the distinct UnsafeInput message; over-length may reuse
// UnsafeInput or MalformedRepoPath (planner's discretion).
```
**Caution:** Because `is_valid_segment` returning `false` currently maps to
`MalformedRepoPath`, routing the leading-dash case to the *distinct* `UnsafeInput` message (D-14
requirement) means the leading-dash check probably needs to happen in `parse_repo_ref` itself
(with owner/repo known), not buried inside `is_valid_segment` (which only returns a bool and
loses the "why"). The planner should hoist the leading-dash decision up to where it can choose
the error variant.

### Anti-Patterns to Avoid
- **Guarding inside `is_valid_segment` and losing the error reason:** that helper returns `bool`,
  so it cannot emit the distinct `UnsafeInput` message D-14 requires. Decide leading-dash at the
  `parse_repo_ref` level.
- **Adding an env var / flag to tune the 500 MB threshold:** explicitly forbidden by D-02. Hardcoded
  constant only; `--deep` is the sole escape hatch.
- **Blocking on missing size:** D-09 mandates fail-open (warn + clone) when metadata is `Unavailable`.
  Never return an error on missing-size — that would regress the tool's core value on a flaky API.
- **Letting `--deep` unlock anything beyond the size bypass:** D-08 — it only bypasses the size guard.
- **`std::process::exit()` after `clone_repo` returns a workspace:** `src/repo/clone.rs` warns that
  exit skips `Drop` and leaks the temp dir. The new `RepoTooLarge` path returns **before** clone, so
  it's safe — but the planner must ensure the guard returns `Err` (propagated up to `main.rs`'s exit)
  rather than calling exit while a `CloneWorkspace` is alive. (Full temp-hygiene is Phase 7.)

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Repo size discovery | A `git clone --depth` probe or `du` on a partial clone | The `size` field already in the metadata response | Metadata is fetched pre-clone already; the field is free, in KB, and avoids touching the network/git twice |
| Bilingual message formatting | Ad-hoc `format!("{}\n{}")` per call site | `i18n::two_line(&bi(vi, en))` | Existing helper; keeps Ferris voice consistent and testable |
| Shell-safe arg handling | Manual shell-escaping of owner/repo | Nothing — `git2` FFI never spawns a shell | Confirmed by grep (no `std::process::Command`) and libgit2 design |
| Exit-code mapping | Scattered `process::exit(N)` literals | One `IntakeError::exit_code()` match arm | Existing centralization; just add code 6 |
| Username/repo charset rules | A regex re-derived from scratch | Extend the existing allowlist + add the two new caps | Existing `is_valid_segment` already enforces charset; only `-`-leading and length are missing |

**Key insight:** Almost everything is already built. The danger in this phase is *over-building* a
"new subsystem" when the locked decisions (D-02, "tighten + document, not a new subsystem") demand a
minimal surface: one field, one branch, two guards, two error variants, one doc, one test module.

## Runtime State Inventory

> This is a hardening/refactor-adjacent phase (adds validation + a serde field), but it persists **no
> runtime state** and renames nothing. The categories are checked explicitly below.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None — verified. No DB, no cache, no persisted IDs touched by this phase. `case_id` (FNV hash in `session.rs`) is computed, not stored. | None |
| Live service config | None — verified. Only outbound call is `GET api.github.com/repos/...`; no service config holds phase state. | None |
| OS-registered state | None — verified. No tasks/daemons/units. Only OS interaction is `tempfile::TempDir` (Phase 7 territory). | None |
| Secrets/env vars | `GITHUB_TOKEN` read in `client.rs` — **unchanged** by this phase (only adds a request, not a new var). No new env var (D-02 forbids a threshold env var). | None |
| Build artifacts | None — verified. Pure source changes; `cargo build` regenerates everything. No egg-info/binary-name equivalents. | None |

**Canonical question — after every file is updated, what runtime systems still carry old behavior?**
Answer: none. This phase has no migration component; it changes *what new inputs are accepted* and
*whether a clone starts*, with no persisted state to migrate.

## Common Pitfalls

### Pitfall 1: Existing decode test breaks when `size` is added
**What goes wrong:** `RepoMetadata` gains `pub size: u64`; the two JSON fixtures in
`src/github/client.rs::tests::decode()` omit `size`, so deserialization fails the test.
**Why it happens:** `size` is required (no `#[serde(default)]`), and the fixtures predate the field.
**How to avoid:** Add `"size": <n>` to both fixtures and assert on it. (Or `#[serde(default)]`, but
that masks a real-API contract.) **Warning signs:** `cargo test` fails in `github::client::tests::decode`.

### Pitfall 2: Leading-dash guard placed where it can't emit the right message
**What goes wrong:** Putting `starts_with('-')` only inside `is_valid_segment` yields a generic
`MalformedRepoPath`, violating D-14 ("leading-dash MUST get the distinct message").
**Why it happens:** `is_valid_segment` returns `bool`, discarding the reason.
**How to avoid:** Decide the leading-dash case in `parse_repo_ref` where the variant can be chosen.
**Warning signs:** A `-foo/bar` input produces the `owner/repo` hint instead of the unsafe-input message.

### Pitfall 3: Off-by-unit on the size comparison
**What goes wrong:** Comparing `size` (KB) against a MB number, or `500 * 1000` vs `500 * 1024`.
**Why it happens:** Mixing KB/MB and decimal/binary. **How to avoid:** Constant
`const MAX_REPO_KB: u64 = 500 * 1024;` and `meta.size > MAX_REPO_KB`. For the message, convert
`size_mb = meta.size / 1024`. D-01 fixes the binary convention (500 MB = 512000 KB).
**Warning signs:** A 499 MB repo refused, or a 501 MB repo passed, in the size-guard branch test.

### Pitfall 4: Threading `--deep` cleanly without churning every signature
**What goes wrong:** Plumbing a bare `bool` through `run` → `collect` adds a positional param that's
easy to misorder. **Why it happens:** `collect(session)` takes only the session today.
**How to avoid (recommended):** Add `pub deep: bool` to `InvestigationSession` and set it in
`InvestigationSession::new` (or a `with_deep` builder), so `collect(session)` reads `session.deep`
with no new params. `main.rs` already constructs the session — pass `args.deep` there. This matches
the existing "everything hangs off `session`" pattern and is CONTEXT D-06's first-listed option.
**Warning signs:** Multiple function signatures changing; positional-bool call sites.

### Pitfall 5: `--deep` warning printed but clone still skipped (or vice versa)
**What goes wrong:** Control flow prints the D-07 warning then forgets to fall through to clone, or
returns `RepoTooLarge` even with `--deep`. **How to avoid:** Structure as: large + deep → warn, do
NOT return (fall through); large + !deep → return `Err(RepoTooLarge)`. Unit-test both branches with
mocked size. **Warning signs:** `--deep` on a big repo exits 6; or non-`--deep` clones a huge repo.

## Code Examples

### Size-guard branch in collect.rs (the core change)
```rust
// src/app/collect.rs — inserted between the meta_state match and clone_repo
// Source: composed from existing collect.rs structure + i18n/run.rs patterns (verified)
const MAX_REPO_KB: u64 = 500 * 1024; // 500 MB, D-01/D-02 hardcoded

match &meta_state {
    RepoMetaState::Available(m) if m.size > MAX_REPO_KB => {
        let size_mb = m.size / 1024;
        if session.deep {
            let w = crate::i18n::two_line(&crate::i18n::bi(
                format!("🦀 Repo lớn ({} MB), Ferris vẫn đào vì --deep — sẽ lâu đó", size_mb),
                format!("Large repo ({} MB) — Ferris digs anyway (--deep); this will take a while", size_mb),
            ));
            eprintln!("{}", w[0]); eprintln!("{}", w[1]);
            // fall through to clone
        } else {
            return Err(IntakeError::RepoTooLarge { size_mb, threshold_mb: 500 });
        }
    }
    RepoMetaState::Unavailable => {
        let n = crate::i18n::two_line(&crate::i18n::bi(
            "🦀 Không biết kích thước repo, Ferris cứ đào nha",
            "Repo size unknown — Ferris digs anyway",
        ));
        eprintln!("{}", n[0]); eprintln!("{}", n[1]); // D-09 fail-open
    }
    _ => {} // Available within threshold → proceed
}

let ws = clone_repo(&session.repo).map_err(|_| IntakeError::Network)?;
```

### Injection / abuse reject test table (extends parse.rs tests, D-16)
```rust
// src/cli/parse.rs tests — extend test_parse_repo_ref_reject (verified table exists)
("-foo/bar", IntakeError::UnsafeInput { input: "-foo/bar".to_string() }),       // leading dash, owner
("foo/-bar", IntakeError::UnsafeInput { input: "foo/-bar".to_string() }),       // leading dash, repo
("--upload-pack=evil/x", /* unsafe */),                                          // arg-injection shape
(&format!("{}/repo", "a".repeat(40)), /* over-length owner */),                 // owner > 39
(&format!("owner/{}", "b".repeat(101)), /* over-length repo */),                // repo > 100
// already covered by existing tests (keep): "..", "\\", control chars, git@, gitlab host
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Shell out to `git clone` (fork/exec) | `git2`/libgit2 FFI in-process | Already the project's choice | No shell → narrow injection surface; the threat model can claim "no shell spawn" truthfully |
| Refuse-after-clone-starts | Pre-flight metadata size check before clone | This phase | Machine never starts an oversized clone (GUARD-01 success criterion) |

**Deprecated/outdated:** Nothing in this phase relies on deprecated APIs. GitHub REST `2022-11-28`
(already pinned via `X-GitHub-Api-Version` header in `client.rs`) remains current.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | GitHub `size` is **always present** in `GET /repos/{o}/{r}` responses (so `pub size: u64` without `#[serde(default)]` is safe for real responses) | Standard Stack / Pattern 1 | LOW — docs mark it required; if a response ever omits it, decode fails → falls into `Unavailable` (fail-open D-09 still protects the user). Mitigation: `#[serde(default)]` if paranoid. |
| A2 | `size` excludes Git LFS objects and may under-report true on-disk size | (note) | LOW — a repo just over 500 MB in LFS could slip past, but `--deep` and the clone itself still proceed; the guard is a heuristic safety net, not a hard quota. |
| A3 | Threading `deep` via `InvestigationSession` is the cleanest plumbing | Pitfall 4 | LOW — CONTEXT D-06 explicitly leaves plumbing to planner discretion; the function-param alternative is equally valid. |

**Note:** A1–A3 are LOW-risk and most are already de-risked by the fail-open design. No HIGH-risk
assumptions — the load-bearing external facts (KB unit, 39/100 limits, no-shell) are all VERIFIED/CITED.

## Open Questions

1. **Does the over-length case (D-11) reuse `UnsafeInput` or `MalformedRepoPath`?**
   - What we know: D-14 says leading-dash MUST get the distinct `UnsafeInput` message; over-length is
     "planner's discretion."
   - Recommendation: route both leading-dash AND over-length to `UnsafeInput` for a consistent
     "this input is unsafe" story; both are exit code 2.

2. **Do injection tests live in `tests/` (integration) or `#[cfg(test)]` (unit)?**
   - What we know: `tests/` is empty today; existing parser tests are inline `#[cfg(test)]`.
   - Recommendation: keep parser-level injection tests inline (fast, no I/O) extending the existing
     accept/reject tables; reserve `tests/` for a future end-to-end exit-code test if desired. Inline
     is sufficient to satisfy SEC-02 "explicit injection/abuse tests."

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain / cargo | build + test | ✓ (project already builds) | edition 2021 | — |
| Network to `api.github.com` | size-guard integration (live) test only | runtime-dependent | — | Unit-test the guard with mocked `RepoMetadata` (no network needed) |

**Missing dependencies with no fallback:** None — all guard/parser logic is unit-testable with
in-memory `RepoMetadata` and string inputs; no live GitHub call required for the test suite.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Built-in Rust test harness (`#[cfg(test)]` + `#[test]`) |
| Config file | none — Cargo convention |
| Quick run command | `cargo test --lib` |
| Full suite command | `cargo test` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| GUARD-01 | Repo > 500 MB without `--deep` → `RepoTooLarge`, clone not started | unit (mocked meta) | `cargo test --lib app::collect` | ❌ Wave 0 (collect.rs has no tests today) |
| GUARD-01 edge | `Unavailable` size → warn + proceed (fail-open) | unit | `cargo test --lib app::collect` | ❌ Wave 0 |
| GUARD-02 | `RepoTooLarge` message contains actual MB + names `--deep`, bilingual (VI+EN two lines) | unit | `cargo test --lib error` | ❌ Wave 0 |
| GUARD-03 | `--deep` on large repo → warning printed, falls through to clone | unit | `cargo test --lib app::collect` | ❌ Wave 0 |
| GUARD-03 | `--deep` flag parses on `Args` | unit | `cargo test --lib cli::args` | ❌ Wave 0 (Args has no tests today) |
| SEC-01 | leading-dash owner/repo rejected with distinct `UnsafeInput` | unit | `cargo test --lib cli::parse` | ✅ extend existing reject table |
| SEC-01 | owner > 39 / repo > 100 rejected | unit | `cargo test --lib cli::parse` | ✅ extend existing reject table |
| SEC-01 | existing guards (`..`, `\`, control, host) still pass | unit | `cargo test --lib cli::parse` | ✅ existing tests |
| SEC-02 | injection/abuse table proves rejection at parser before fetch/clone | unit | `cargo test --lib cli::parse` | ✅ extend |
| (decode) | `RepoMetadata` decodes `size` (KB) | unit | `cargo test --lib github::client` | ✅ extend existing `decode()` |
| exit codes | `RepoTooLarge` → 6; `UnsafeInput` → 2 | unit | `cargo test --lib error` | ❌ Wave 0 (add exit_code assertions) |

### Sampling Rate
- **Per task commit:** `cargo test --lib` (fast, no network)
- **Per wave merge:** `cargo test` (full suite incl. any integration tests)
- **Phase gate:** `cargo test` green + `cargo clippy` clean before `/gsd-verify-work`

### Wave 0 Gaps
- [ ] `src/app/collect.rs` `#[cfg(test)]` module — size-guard branches (GUARD-01/03 + fail-open).
      Requires a way to construct `RepoMetadata` in tests (it has public fields → constructible) and
      a `session` carrying `deep`. Plan must make the guard logic testable without a live clone
      (e.g., extract a pure `fn size_decision(meta_state, deep) -> Decision` helper, or test via a
      `RepoMetadata` fixture + assert on returned `Result`/printed branch).
- [ ] `src/cli/args.rs` `#[cfg(test)]` — assert `--deep` parses (`Args::parse_from(["bin","o/r","--deep"]).deep == true`).
- [ ] `src/error.rs` `#[cfg(test)]` — assert `RepoTooLarge.exit_code() == 6`, `UnsafeInput.exit_code() == 2`,
      and that `to_string()` contains both VI and EN lines + the MB value + `--deep`.
- [ ] Extend `src/cli/parse.rs` reject table (leading-dash, over-length) — file exists.
- [ ] Extend `src/github/client.rs` `decode()` fixtures with `"size"` — file exists.
- [ ] Framework install: none — Rust test harness is built in.

*Recommendation: extract a small pure decision function for the size guard so GUARD-01/03 are unit-
testable without network or a real clone. This is the single most valuable Wave 0 enabler.*

## Security Domain

> `security_enforcement: true`, ASVS level 1, block on `high`. This phase **is** the security phase —
> hardening intake against injection/abuse and DoS-by-oversize.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | `GITHUB_TOKEN` is optional bearer auth, unchanged this phase |
| V3 Session Management | no | No sessions (CLI) |
| V4 Access Control | no | Public-repo-only tool, no authz model |
| V5 Input Validation | **yes** | Allowlist charset + leading-dash reject + length caps in `parse_repo_ref` (hand-validated string parsing is appropriate here — input is a constrained `owner/repo`, not free-form; no parser library needed) |
| V6 Cryptography | no | TLS handled by `rustls`/libgit2; nothing hand-rolled |
| V12/V13 (DoS / resource) | **yes** | Pre-flight size guard caps clone resource consumption (GUARD-01) |

### Known Threat Patterns for {Rust CLI + git2 + reqwest intake}

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Argument injection via leading `-` (e.g. `--upload-pack`) | Tampering / EoP | Reject any segment starting with `-` at the parser (D-10); **and** `git2` never spawns a shell/`git` subprocess, so the classic flag-injection vector is structurally absent (D-15, VERIFIED by grep) |
| Path traversal (`..`) | Tampering | Existing `..` reject in `parse_repo_ref` + `is_valid_segment` (kept, D-12) |
| Backslash / control chars | Tampering | Existing `\` and `is_control()` rejects (kept, D-12) |
| Unsupported host / SSRF-ish redirect | Spoofing / Info disclosure | Host allowlist `github.com`/`www.github.com` only (existing); URL built from validated `owner/repo` only |
| Oversize-clone DoS (machine hang) | Denial of Service | Pre-flight `size > 500 MB` refuse-by-default (GUARD-01); `--deep` is an explicit, warned opt-in |
| Over-length segment (resource/edge abuse) | DoS / Tampering | owner ≤ 39, repo ≤ 100 caps (D-11) reject before network/git2 |

**Threat-model doc (`docs/THREAT-MODEL.md`, D-15) must explicitly state:** the intake attack surface
(leading-`-`, `..`, `\`, control chars, oversize DoS), the mitigations above, and the structural
fact that **`git2` is libgit2 via FFI — no shell, no `git` subprocess is spawned** (verified: zero
`std::process::Command` in `src/`), which is what makes the injection surface narrow by construction.

## Sources

### Primary (HIGH confidence)
- Codebase (read in full): `src/app/collect.rs`, `src/github/client.rs`, `src/cli/parse.rs`,
  `src/cli/args.rs`, `src/error.rs`, `src/i18n.rs`, `src/main.rs`, `src/app/session.rs`,
  `src/app/run.rs`, `src/snapshot.rs`, `src/repo/clone.rs`, `Cargo.toml`, `Cargo.lock` — all
  CONTEXT.md assumptions confirmed against actual source.
- `grep -rn 'std::process::Command' src/` → zero matches (only `std::process::exit` in main.rs and a
  doc-comment in clone.rs) — confirms the no-shell-spawn threat-model claim.
- github.com/dead-claudia/github-limits — owner max **39 chars**, repo name max **100 code points**.
- GitHub REST docs (`/rest/repos/repos`) — `size` is a **required integer** on the repo object.

### Secondary (MEDIUM confidence)
- WebSearch (multiple sources agree): repository `size` field is **in kilobytes**, sums files across
  full git history, **excludes Git LFS**, and can diverge from true disk usage (GraphQL `diskUsage`
  is more accurate but not needed here).
- libgit2.org + git2-rs README + copyninja.in writeup: `git2::Repository::clone` binds libgit2 via
  FFI and avoids fork/exec of `git` — corroborates the codebase grep.

### Tertiary (LOW confidence)
- None load-bearing. All critical facts cross-verified above.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new packages; all versions read from `Cargo.lock`.
- Architecture / integration points: HIGH — every file named in CONTEXT.md read and confirmed.
- External facts (KB unit, 39/100 limits, no-shell): HIGH — CITED/VERIFIED, cross-referenced.
- Pitfalls: HIGH — derived from the actual code (existing decode test, `is_valid_segment` bool, etc.).

**Research date:** 2026-06-04
**Valid until:** ~2026-07-04 (stable — GitHub limits and libgit2 architecture change rarely; codebase
facts valid until the listed files change).
