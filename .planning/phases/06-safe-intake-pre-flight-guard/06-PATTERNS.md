# Phase 6: Safe Intake & Pre-flight Guard - Pattern Map

**Mapped:** 2026-06-04
**Files analyzed:** 7 (6 code/modified + 1 greenfield doc)
**Analogs found:** 6 / 7 (THREAT-MODEL.md is greenfield, no code analog)

> All analogs below are **in-file** — every modified file is its own best analog
> (existing variants / fields / tests / branches to copy from). This is a "tighten +
> document" phase, so the closest pattern is almost always one line above the change.
> Line numbers verified against current source on 2026-06-04.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `src/github/client.rs` | model (serde DTO) | transform (deserialize) | existing `RepoMetadata` fields + `decode()` test (same file) | exact |
| `src/app/collect.rs` | service (orchestration) | request-response | existing meta-state match / abort-404 / degrade-transient branch (same file) | exact |
| `src/cli/parse.rs` | utility (validation) | transform (string→`RepoRef`) | existing `is_valid_segment` + reject test table (same file) | exact |
| `src/cli/args.rs` | config (CLI flags) | request-response | existing single-field clap `Args` struct (same file) | exact |
| `src/error.rs` | model (error enum) | event-driven (error propagation) | existing `MalformedRepoPath` variant + `exit_code()` arm (same file) | exact |
| `src/app/session.rs` | model (session state) | request-response | existing `InvestigationSession` struct + `new()` (same file) | exact (plumbing — D-06 discretion) |
| `docs/THREAT-MODEL.md` | doc | — | none — `docs/` does not exist yet | **no analog (greenfield)** |

**Plumbing note:** `--deep` threading (D-06) is planner's discretion. Recommended path
(RESEARCH Pitfall 4): add `pub deep: bool` to `InvestigationSession`, set it in
`main.rs` where the session is already built (`main.rs:11`), so `collect(session)` reads
`session.deep` with no new function params. `session.rs` is therefore touched too.

## Pattern Assignments

### `src/github/client.rs` (model, transform)

**Analog:** existing `RepoMetadata` struct + `decode()` test, same file.

**Add field** — follow the existing `pub <name>: <type>` field convention (`client.rs:6-15`).
`size` is a **required** integer KB per GitHub REST (no `#[serde(default)]` — unlike `topics`):
```rust
#[derive(Deserialize, Debug)]
pub struct RepoMetadata {
    pub stargazers_count: u64,
    pub forks_count: u64,
    // ...existing fields...
    pub created_at: Option<String>,
    pub size: u64, // NEW — kilobytes, required field per GitHub REST
}
```

**Update decode test** — both JSON fixtures in `decode()` (`client.rs:75-83` and `94-101`)
omit `size`; adding a non-default `u64` breaks them. Add `"size": <n>` to **both** fixtures
and assert, matching the existing assert style (`client.rs:86-92`):
```rust
// first fixture (client.rs:75-83): add  "size": 1024
assert_eq!(meta.size, 1024);
// second fixture (client.rs:94-101): add "size": 0  (or any value) so decode still succeeds
```
**Pitfall (RESEARCH P1):** `cargo test --lib github::client::tests::decode` fails if either
fixture is missed.

---

### `src/error.rs` (model, event-driven)

**Analog:** existing `MalformedRepoPath` variant (`error.rs:14-15`) + `exit_code()` match (`error.rs:30-41`).

**Variant pattern** — bilingual two-line `#[error("VI\nEN")]` with `🦀` prefix, named struct
fields (copy the exact shape of `RepoNotFoundOrPrivate { owner, repo }` at `error.rs:17-18`):
```rust
#[error("🦀 Repo này to quá ({size_mb} MB), Ferris không đào tự động đâu — chạy lại với --deep nếu bạn chấp nhận chờ lâu\nThis repo is too large ({size_mb} MB). Ferris won't auto-dig — re-run with --deep if you accept the longer wait")]
RepoTooLarge { size_mb: u64, threshold_mb: u64 },   // D-13 — message MUST show MB + name --deep

#[error("🦀 Input này kỳ lạ / không an toàn, Ferris không nhận\nThis input looks unsafe — Ferris won't take it")]
UnsafeInput { input: String },                        // D-14 — distinct from MalformedRepoPath
```

**exit_code() pattern** — add to the existing match (`error.rs:32-40`). `UnsafeInput` joins
the input class (2); `RepoTooLarge` gets the **new code 6**:
```rust
Self::EmptyInput
  | Self::NotAUrl { .. }
  | Self::UnsupportedHost { .. }
  | Self::MalformedRepoPath { .. }
  | Self::UnsafeInput { .. } => 2,        // D-14 — input class
Self::RepoNotFoundOrPrivate { .. } => 3,
Self::Network | Self::RateLimited => 4,
Self::CollectionFailed { .. } => 5,
Self::RepoTooLarge { .. } => 6,           // D-13 — NEW exit code 6
```

**Test (Wave 0, new):** `error.rs` has no `#[cfg(test)]` today. Add one asserting
`RepoTooLarge{..}.exit_code() == 6`, `UnsafeInput{..}.exit_code() == 2`, and that
`to_string()` contains both VI+EN lines, the MB value, and `--deep` (GUARD-02).
Enum derives `Clone, PartialEq, Eq` (`error.rs:3`) — constructible in tests.

---

### `src/cli/parse.rs` (utility, transform)

**Analog:** existing `is_valid_segment` (`parse.rs:89-94`) + existing `..`/`\` reject loop
(`parse.rs:39-43`) + reject test table (`parse.rs:128-155`).

**Leading-dash guard (D-10) — hoist to `parse_repo_ref`, NOT inside `is_valid_segment`.**
`is_valid_segment` returns `bool` and loses the "why", so it cannot emit the distinct
`UnsafeInput` message D-14 requires. Decide it where `owner`/`repo_str` are known
(after `parse.rs:79`, alongside the existing charset check):
```rust
// in parse_repo_ref, after owner/repo_str identified (near parse.rs:78-81):
if owner.starts_with('-') || repo_str.starts_with('-') {
    return Err(IntakeError::UnsafeInput { input: input.to_string() }); // D-10 distinct message
}
```

**Length caps (D-11)** — owner ≤ 39, repo ≤ 100, before the network/git2 call:
```rust
if owner.len() > 39 || repo_str.len() > 100 {
    return Err(IntakeError::UnsafeInput { input: input.to_string() }); // reuse UnsafeInput (RESEARCH OQ1 rec)
}
```

**Keep existing guards (D-12):** the `..`/`\` loop (`parse.rs:40-42`), control-char + 2048
cap (`parse.rs:15-17`), host allowlist (`parse.rs:51,67`), charset allowlist
(`is_valid_segment`, `parse.rs:93`) — all unchanged.

**Reject-table pattern (D-16)** — extend the existing `cases` vec in `test_parse_repo_ref_reject`
(`parse.rs:130-143`), same `(input, expected_err)` tuple shape:
```rust
("-foo/bar", IntakeError::UnsafeInput { input: "-foo/bar".to_string() }),   // leading dash, owner
("foo/-bar", IntakeError::UnsafeInput { input: "foo/-bar".to_string() }),   // leading dash, repo
// over-length owner (>39) and repo (>100) using format!("{}", "a".repeat(40)) etc.
```
SEC-02: keep the existing `..`/`\`/`git@`/`gitlab` rows (`parse.rs:135-142`) — they prove
rejection happens at the parser before any `fetch_metadata`/`clone_repo`.
**Pitfall (RESEARCH P2):** if the dash check lands inside `is_valid_segment`, `-foo/bar`
yields `MalformedRepoPath` and the test (and D-14) fails.

---

### `src/cli/args.rs` (config, request-response)

**Analog:** existing single-field `Args` struct (`args.rs:8-14`), `#[derive(Parser)]` + `#[arg(...)]`.

**Add flag** — first non-positional flag on the struct; boolean flags need no `required`:
```rust
pub struct Args {
    #[arg(help = "...", required = true)]
    pub repo: String,

    #[arg(long, help = "Đào sâu kể cả repo lớn (bỏ qua giới hạn kích thước) / Dig even large repos (skip the size guard)")]
    pub deep: bool, // D-06 — clap derives --deep from the field name
}
```

**Test (Wave 0, new):** `args.rs` has no tests today. Add
`Args::parse_from(["bin","o/r","--deep"]).deep == true` and the default-false case.

---

### `src/app/collect.rs` (service, request-response)

**Analog:** existing meta-state match + abort-404 / degrade-transient branch (`collect.rs:12-25`),
and the clone call (`collect.rs:28`). Notice the existing degrade notice uses **`eprintln!`**
(`collect.rs:22`) — new warnings/notices stay on **stderr** to match.

**Size-guard branch — insert between the meta_state match (ends `collect.rs:25`) and
`clone_repo` (`collect.rs:28`).** Extract a **pure decision helper** so GUARD-01/03 are
unit-testable without a live clone (RESEARCH Wave-0 recommendation — the single most valuable
enabler):
```rust
const MAX_REPO_KB: u64 = 500 * 1024; // 500 MB — D-01/D-02 hardcoded, no env var

enum SizeDecision { Proceed, WarnDeep { size_mb: u64 }, WarnUnknown, TooLarge { size_mb: u64 } }

// pure, no I/O — directly unit-testable with a RepoMetadata fixture
fn size_decision(meta_state: &RepoMetaState, deep: bool) -> SizeDecision {
    match meta_state {
        RepoMetaState::Available(m) if m.size > MAX_REPO_KB => {
            let size_mb = m.size / 1024;
            if deep { SizeDecision::WarnDeep { size_mb } } else { SizeDecision::TooLarge { size_mb } }
        }
        RepoMetaState::Unavailable => SizeDecision::WarnUnknown, // D-09 fail-open
        _ => SizeDecision::Proceed,
    }
}
```
Then act on it in `collect`, returning **before** clone for the refuse case (RESEARCH
anti-pattern: never `process::exit` while a `CloneWorkspace` is alive — returning `Err`
up to `main.rs:14` is the safe path; the guard returns before clone so no workspace exists):
```rust
match size_decision(&meta_state, session.deep) {
    SizeDecision::TooLarge { size_mb } => {
        return Err(IntakeError::RepoTooLarge { size_mb, threshold_mb: 500 }); // exit 6
    }
    SizeDecision::WarnDeep { size_mb } => { /* two_line warning, eprintln, fall through */ }
    SizeDecision::WarnUnknown => { /* two_line notice, eprintln, fall through */ }
    SizeDecision::Proceed => {}
}
let ws = clone_repo(&session.repo).map_err(|_| IntakeError::Network)?; // existing collect.rs:28
```

**Bilingual notice pattern (D-07/D-09)** — use `i18n::two_line(&i18n::bi(vi, en))` (see Shared
Patterns), `eprintln!` both lines to match the existing degrade line at `collect.rs:22`.

**Pitfall (RESEARCH P3/P5):** use the binary `500 * 1024` constant and `size_mb = size / 1024`;
ensure `--deep` + large → warn + **fall through** (do not return), `!deep` + large → return Err.

---

### `src/app/session.rs` (model, request-response) — `--deep` plumbing (D-06, discretion)

**Analog:** existing `InvestigationSession` struct (`session.rs:5-9`) + `new()` (`session.rs:12-20`).

Recommended (RESEARCH Pitfall 4): add `pub deep: bool` to the struct and a param to `new`
(or a `with_deep` builder), set from `args.deep` at `main.rs:11`:
```rust
pub struct InvestigationSession {
    pub repo: RepoRef,
    pub case_id: String,
    pub started_at: SystemTime,
    pub deep: bool, // NEW
}
// main.rs:11 — InvestigationSession::new(repo_ref, args.deep)
```
This keeps the "everything hangs off `session`" pattern; `collect(session)` reads `session.deep`
with no new function params. (Planner may instead pass `deep` as a `collect` param — D-06.)

---

### `docs/THREAT-MODEL.md` (doc) — GREENFIELD, no analog

`docs/` does not exist yet (verified). No in-repo doc to copy structure from — planner should
use RESEARCH §Security Domain (the STRIDE threat table at RESEARCH.md:457-471) as the content
spec. Must cover (D-15): leading-`-` arg-injection, `..` traversal, `\`, control chars,
oversize-clone DoS, and the structural fact that **`git2` is libgit2 via FFI — no shell / no
`git` subprocess** (verified: zero `std::process::Command` in `src/`). Back with the injection
tests in `parse.rs` (D-16).

## Shared Patterns

### Bilingual user-facing text
**Source:** `src/i18n.rs` — `bi(vi, en)` (`i18n.rs:16`) → `Bilingual`; `two_line(&b) -> [String; 2]` (`i18n.rs:20`).
**Apply to:** every new notice (D-07 `--deep` warning, D-09 unknown-size) in `collect.rs`.
**Apply to (error variants):** `error.rs` variants embed the bilingual string inline in
`#[error("VI\nEN")]` (they do NOT call `two_line`) — match the existing variant style.
```rust
let w = crate::i18n::two_line(&crate::i18n::bi(
    format!("🦀 Repo lớn ({} MB), Ferris vẫn đào vì --deep — sẽ lâu đó", size_mb),
    format!("Large repo ({} MB) — Ferris digs anyway (--deep); this will take a while", size_mb),
));
eprintln!("{}", w[0]); eprintln!("{}", w[1]);
```

### Ferris voice
**Source:** every `#[error(...)]` in `error.rs` and notices in `collect.rs:22`.
**Apply to:** all new strings. Third-person "Ferris", never "tôi/mình"; lead with `🦀`.

### Error → exit-code propagation
**Source:** `error.rs::exit_code()` (`error.rs:30-41`) consumed at `main.rs:13-14,20-21`.
**Apply to:** new `RepoTooLarge`/`UnsafeInput`. No scattered `process::exit(N)` — return the
`IntakeError`, let `main.rs` map it. `RepoTooLarge` returns **before** clone (no live workspace).

### serde DTO + decode test
**Source:** `RepoMetadata` (`client.rs:5-15`) + `decode()` (`client.rs:73-106`).
**Apply to:** the new `size` field — add to struct AND to both decode fixtures.

### Reject test table
**Source:** `test_parse_repo_ref_reject` `cases` vec (`parse.rs:130-143`), `(input, IntakeError)` tuples.
**Apply to:** new leading-dash / over-length injection rows (D-16, SEC-02).

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `docs/THREAT-MODEL.md` | doc | — | `docs/` directory does not exist yet; no version-controlled doc to mirror. Use RESEARCH §Security Domain (RESEARCH.md:441-471) as the content spec. |

## Metadata

**Analog search scope:** `src/error.rs`, `src/cli/parse.rs`, `src/cli/args.rs`,
`src/github/client.rs`, `src/app/collect.rs`, `src/app/session.rs`, `src/main.rs`,
`src/i18n.rs`, `docs/` (absent), `.claude/skills` + `.agents/skills` (absent).
**Files scanned:** 8 source files (all read in full; each ≤ 160 lines, single Read each).
**Pattern extraction date:** 2026-06-04
