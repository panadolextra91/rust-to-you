# Phase 2: Collection Layer - Research

**Researched:** 2026-06-02
**Domain:** Rust sync collection pipeline — git2 0.21 (clone + history walk), reqwest 0.13 blocking (one GitHub REST call), tokei 14 (language counts), tempfile (RAII workspace)
**Confidence:** HIGH (all crate versions + core APIs verified against crates.io and docs.rs this session)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Full clone into a **unique temp dir** with **RAII auto-clean** (deleted on normal exit AND on error/panic; recommend `tempfile`). **No caching in V1.** Read-only guarantee preserved: never write the remote; workspace owned and always cleaned.
- **D-02:** GitHub API metadata call (`GET /repos/{owner}/{repo}`) runs **FIRST, before the clone**. This is where Phase 1's reserved `IntakeError` variants get populated:
  - HTTP 404 → `RepoNotFoundOrPrivate` → **ABORT** (exit 3).
  - network error → `Network` (exit 4); HTTP 403/rate-limit → `RateLimited` (exit 4).
- **D-03:** Failure policy is **abort-on-404, degrade-on-transient**: 404 aborts; a transient `Network`/`RateLimited` failure **on the metadata call** does NOT kill the run — proceed to clone and mark stars/forks/description **unavailable/"unknown"** in the snapshot. A failure of the **clone itself** is a hard error.
- **D-04:** Use `GITHUB_TOKEN` env var **if present** (higher limit); otherwise unauthenticated.
- **D-05:** Expensive passes — **most-modified-file** and **time-of-day signals** (`night_pct`, `weekend_pct`, `business_hours_pct`) — bounded to the **most recent 1000 commits**. If ≤1000 commits, use all. When capped, result is **labeled "based on last N commits."**
- **D-06:** Cheap full-history aggregations are **NOT** bounded (never diff): total commits, contributor counts, bus factor, repo age, branch enumeration + branch last-commit dates.
- **D-07:** Walking skeleton (02-01) = full-clone input repo → compute **repo age** (from first commit date) → print **one plain-text line** via the existing Phase 1 `run(session)` seam. Throwaway output, NOT ratatui. Explicitly allowed to be replaced by real collectors in 02-03/02-04.

### Claude's Discretion
- **Infra detection method:** path/glob **presence** checks (no content parsing in V1). Signal LIST locked by REQUIREMENTS (Docker, Terraform, GitHub Actions, GitLab CI, CircleCI, Jenkins, Dependabot, Renovate); typical paths: `Dockerfile`/`docker-compose.y*ml`, `*.tf`, `.github/workflows/`, `.gitlab-ci.yml`, `.circleci/config.yml`, `Jenkinsfile`, `.github/dependabot.yml`, `renovate.json`/`.github/renovate.json`.
- **Language Soup:** use the **`tokei` crate** (library, line-count based) — not a custom counter.
- **`InvestigationSnapshot` internal shape** (sub-structs like RepoMetadata / HistoryFacts / BranchFacts / FilesystemFacts) — planner's call, per research/ARCHITECTURE.md.
- **Default branch** comes from the clone's HEAD (git), not the API.

### Carried Forward (locked earlier — do NOT re-litigate)
- Full clone (not shallow) · git2 for ALL local data · API only for stars/forks/description · `reqwest` **blocking**, NO `tokio` · no rename tracking · **bot + identity filtering shared** across contributor_count/top_author_share/bus_factor · normalize into one `InvestigationSnapshot` · builds on Phase 1's `run()` seam, `RepoRef`, `IntakeError`.

### Deferred Ideas (OUT OF SCOPE)
- Caching / offline mode (`--cache`, `--offline`) → v2.
- `--deep` unbounded history → v2 (lift the 1000-commit cap).
- Distinguishing private vs not-found → needs auth; v2 (EXPD-03).
- Content-parsing infra configs (vs presence detection) → not needed for V1.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| COLL-01 | Overview metadata: age, default branch, stars, forks, contributors, last activity | Age = first-commit date via `Sort::TIME \| Sort::REVERSE` revwalk (§3). Default branch = clone HEAD `repo.head()?.shorthand()` (§4, D-07 says git not API). Stars/forks/description = the single `GET /repos/{owner}/{repo}` call (§7), degradable to "unknown" (D-03). Contributors = full-history author→count with shared filtering (§6). Last activity = max branch-tip commit time and/or `pushed_at` (§4). |
| COLL-02 | Git/branch/history evidence + local snapshot | Total commits = full revwalk count (§2). Branch enumeration + tip dates = `repo.branches(...)` (§4 — note remote-tracking refs after clone). Bounded last-1000-commit window for most-modified-file + time-of-day (§2, §5). All normalized into `InvestigationSnapshot` (§9). |
| COLL-03 | Language + infra signals (no remote mutation) | tokei 14 library `Languages::get_statistics` over the clone path → percentages (§8). Infra = path/glob presence scan over the clone tree (§ Infra). Read-only guaranteed: all reads happen on the local temp clone; remote is only ever cloned (read), never pushed. |
</phase_requirements>

## Summary

Phase 2 is a **fully synchronous** collection pipeline that runs in this strict order (per D-02): (1) one blocking GitHub REST call for stars/forks/description, with transient failure degrading rather than aborting; (2) a full clone into a `tempfile::TempDir` (RAII auto-clean); (3) git2 history/branch passes — cheap full-history counts plus bounded last-1000-commit diff passes; (4) a tokei language pass and a path-presence infra scan over the cloned tree; (5) normalization into one `InvestigationSnapshot`. The walking skeleton (02-01) is a vertical slice of just clone→repo-age→print-one-line through the existing `run(&session)` seam.

All crate versions are verified current on crates.io as of 2026-06-02: `git2` 0.21.0, `reqwest` 0.13.4, `tokei` 14.0.0, `tempfile` 3.27.0, `chrono` 0.4.44, `serde_json` 1.0.150, plus `serde` (derive). All core git2/reqwest/tokei API signatures were verified against docs.rs this session.

**Two non-obvious gotchas dominate this phase and MUST be in the plan:**
1. **git2 0.21 `default` features are EMPTY** — HTTPS clone of `https://github.com/...` requires enabling the `https` feature (which links OpenSSL). Without it, `Repository::clone` fails on the transport. Recommend `vendored-openssl` for build portability on macOS/CI.
2. **GitHub rejects any request with no `User-Agent` header with 403** (verified in GitHub docs). The blocking reqwest client MUST set a `user_agent(...)` on the builder, or every metadata call 403s and gets misclassified as a rate-limit.

**Primary recommendation:** Add `git2 = { version = "0.21", features = ["vendored-openssl"] }`, `reqwest = { version = "0.13", default-features = false, features = ["blocking", "json", "rustls-tls"] }`, `tokei = { version = "14", default-features = false }`, `tempfile = "3"`, `chrono = "0.4"`, `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"`. Build a `repo/` module (clone + git2 helpers), a `github/` module (one REST call + serde model), a `scan/` module (tokei + infra presence), and a `snapshot` model that represents degraded metadata explicitly via `enum`/`Option`.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Stars / forks / description | GitHub REST API (`github/`) | — | These are GitHub-platform facts git cannot know. One call only (Pitfall 2). |
| Default branch | Local git (`repo/`, clone HEAD) | — | D-07 + Discretion: comes from clone HEAD, NOT the API (clone already checked out the default). |
| Repo age (first commit) | Local git (`repo/` revwalk) | — | Full history is local after clone; no API. |
| Total commits / contributors / bus factor | Local git full-history walk (`repo/`) | — | Cheap counting pass, D-06 (never diffs). |
| Branch enumeration + tip dates | Local git (`repo/` branches) | — | D-06; remote-tracking refs after clone (§4). |
| Most-modified-file / time-of-day | Local git bounded walk (`repo/`) | — | D-05 expensive diff pass, capped at 1000. |
| Language percentages | tokei lib over clone path (`scan/`) | — | Discretion: tokei crate, not custom. |
| Infra footprints | Filesystem presence scan over clone (`scan/`) | — | Discretion: path/glob presence, no content parse. |
| Last activity | Local git (max branch-tip time) | API `pushed_at` (degradable) | Git tip time is always available; `pushed_at` is a nice cross-check but degrades with the API. |
| Snapshot normalization | Data model (`snapshot`/`app`) | — | ARCHITECTURE Pattern 2; analyzers (Phase 3) consume only this. |
| Workspace lifecycle (clone dir) | `repo/` (owns `TempDir`) | `app/run` | RAII cleanup must outlive collection but be dropped before process exit (D-01). |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `git2` | 0.21.0 | Full clone + ALL local git facts (commits, branches, contributors, file history, default branch) | The libgit2 binding; avoids shelling out to `git`. Locked in STACK.md. `[VERIFIED: crates.io]` |
| `reqwest` | 0.13.4 | Single blocking GitHub REST call (stars/forks/description) | Blocking feature needs no async runtime — matches the no-tokio decision. `[VERIFIED: crates.io]` |
| `tokei` | 14.0.0 | Language line-count percentages (Language Soup) | De-facto Rust line-counter; library API is stable. `[VERIFIED: crates.io]` |
| `tempfile` | 3.27.0 | RAII temp clone workspace (auto-clean on drop/panic) | Standard scratch-dir crate; `TempDir` deletes on drop. `[VERIFIED: crates.io]` (D-01 recommends it explicitly) |
| `chrono` | 0.4.44 | Repo age, last-activity, night/weekend/business-hours bucketing | Already in STACK.md; converts git2 epoch+offset to local-author datetimes. `[VERIFIED: crates.io]` |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` (derive) | 1.x | Derive the small GitHub response struct | For the `#[derive(Deserialize)]` model of the `/repos` payload. STACK.md lists it. `[VERIFIED: crates.io]` |
| `serde_json` | 1.0.150 | Decode the REST JSON | reqwest `.json::<T>()` uses serde under the hood; explicit dep only if you parse manually. `[VERIFIED: crates.io]` |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `git2` | `std::process::Command("git")` | Shelling out avoids the OpenSSL link, but adds a runtime `git` binary dependency and string-parsing fragility. Locked to git2 in STACK.md — do NOT switch. |
| `reqwest` blocking | `ureq` / `minreq` | Lighter (no OpenSSL/rustls choice), but STACK.md locks `reqwest` blocking. Keep reqwest. |
| `reqwest` JSON | `serde_json` manual parse | `.json::<T>()` is cleaner for one struct; only manual-parse if you must inspect rate-limit headers on error. |
| `vendored-openssl` | system OpenSSL (`https` feature) | Vendored avoids "OpenSSL not found" on macOS/CI at the cost of a longer first build. Either works; vendored is the portability default. |

**Installation (planner: this is the cargo add command set):**
```bash
cargo add git2 --features vendored-openssl
cargo add reqwest --no-default-features --features blocking,json,rustls-tls
cargo add tokei --no-default-features
cargo add tempfile chrono
cargo add serde --features derive
cargo add serde_json
```

**Feature-flag notes (CRITICAL — verified via crates.io feature inspection this session):**
- `git2` 0.21 `default = []` (EMPTY). The `https` feature = `["libgit2-sys/https", "openssl-sys", "openssl-probe", "cred"]`. `vendored-openssl` = `["openssl-sys/vendored", "libgit2-sys/vendored-openssl"]`. **Cloning over HTTPS needs OpenSSL transport.** Use `vendored-openssl` (statically links, no system OpenSSL needed) — strongly recommended for macOS (the dev box) and CI. `[VERIFIED: crates.io git2/0.21.0 features]`
- `reqwest` blocking + `rustls-tls` avoids a *second* OpenSSL dependency for the HTTP side (git2 uses OpenSSL; reqwest uses rustls — fine to mix). `--no-default-features` drops the async default client. `[CITED: docs.rs/reqwest/0.13.4 blocking]`
- `tokei` 14 `default = ["cli"]` pulls clap/colored/env_logger/num-format — **library users MUST set `default-features = false`** to avoid those. `[VERIFIED: crates.io tokei/14.0.0 features]`

## Package Legitimacy Audit

slopcheck was available and run this session (`slopcheck install <pkgs>`). Note: slopcheck's `install` verb has a side effect of running `cargo add` — this researcher reverted the resulting `Cargo.toml`/`Cargo.lock` changes immediately (working tree confirmed clean). The planner's install task is the authoritative one.

| Package | Registry | Age | Downloads (recent) | Source Repo | slopcheck | Disposition |
|---------|----------|-----|--------------------|-------------|-----------|-------------|
| git2 | crates.io | mature | ~14.1M | github.com/rust-lang/git2-rs | [OK] | Approved |
| reqwest | crates.io | mature | ~119.9M | github.com/seanmonstar/reqwest | [OK] | Approved |
| tempfile | crates.io | mature | ~129.5M | github.com/Stebalien/tempfile | [OK] | Approved |
| chrono | crates.io | mature | ~117.9M | github.com/chronotope/chrono | [OK] | Approved |
| serde_json | crates.io | mature | ~188.7M | github.com/serde-rs/json | [OK] | Approved |
| tokei | crates.io | mature | ~160K | github.com/XAMPPRocky/tokei | [SUS] | **Approved despite [SUS]** — false positive (typosquat-similarity to `tokio` is a string-distance artifact). tokei is the explicitly-locked language counter in STACK.md, ~160K recent downloads, well-known project. No checkpoint needed; this disposition is the research verdict. |

**Packages removed due to slopcheck [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** tokei — confirmed false positive (name resembles `tokio`). Use as planned.

## Architecture Patterns

### System Architecture Diagram

```text
RepoRef {owner, repo}  (from Phase 1, validated)
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│ github/  ── GET /repos/{owner}/{repo}  (reqwest blocking)  │
│   200 → RepoMetadata{stars,forks,description,topics,...}   │
│   404 → IntakeError::RepoNotFoundOrPrivate  ── ABORT (3)   │
│   403/rate → RateLimited ─┐  network err → Network ─┐      │
│                           └──── DEGRADE (mark unknown)─┘    │  (D-03)
└──────────────────────────────────────────────────────────┘
        │ (continue unless 404)
        ▼
┌──────────────────────────────────────────────────────────┐
│ repo/  ── tempfile::TempDir (RAII)                         │
│   Repository::clone(https url, tmp)  ── clone fail = HARD  │
│        │                                                   │
│        ├─ FULL-HISTORY walk (cheap, D-06):                 │
│        │    total commits · contributors+bus factor ·      │
│        │    repo age(first commit) · branch enum+tip dates │
│        │                                                   │
│        └─ BOUNDED last-1000 walk (expensive, D-05):        │
│             most-modified-file · night/weekend/biz-hours % │
│             ── labeled "based on last N commits"           │
└──────────────────────────────────────────────────────────┘
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│ scan/  ── over the cloned working tree (read-only)         │
│   tokei Languages::get_statistics → lang→pct               │
│   infra path/glob presence → 8 boolean footprints          │
└──────────────────────────────────────────────────────────┘
        │
        ▼
   InvestigationSnapshot { repo_meta(degradable), history,
                           branches, filesystem }   ── one normalized struct
        │
        ▼
   run(&session, snapshot)  (Phase 1 seam; 02-01 prints repo age line)
```

### Recommended Project Structure
```text
src/
├── app/
│   ├── run.rs        # extend: run(&session) → orchestrates collect, prints (02-01) / passes snapshot
│   └── collect.rs    # NEW (opt): top-level collection orchestrator (api → clone → walk → scan → snapshot)
├── github/           # NEW: REST client + serde model + error mapping
│   ├── mod.rs
│   └── client.rs     # blocking reqwest, GET /repos, 404/403/network classification
├── repo/             # NEW: clone workspace + git2 history/branch helpers
│   ├── mod.rs
│   ├── clone.rs      # TempDir + Repository::clone + clone-error mapping
│   └── history.rs    # revwalk: counts, repo age, contributors/bus inputs, bounded passes
├── scan/             # NEW: filesystem signals
│   ├── mod.rs
│   ├── lang.rs       # tokei wrapper → percentages
│   └── infra.rs      # path/glob presence → footprint flags
├── snapshot.rs       # NEW: InvestigationSnapshot + sub-structs (degradable metadata)
└── error.rs          # EXTEND: populate RepoNotFoundOrPrivate/Network/RateLimited (already defined)
```

### Pattern 1: API-first, degrade-on-transient (D-02/D-03)
**What:** Run the single REST call first; a 404 aborts, but `Network`/`RateLimited` are caught and converted to "metadata unavailable," then the clone proceeds.
**When to use:** The metadata step only. The clone step has the opposite policy — any clone failure is fatal.
**Example:**
```rust
// Source: derived from D-02/D-03 + reqwest blocking API (docs.rs/reqwest/0.13.4)
let repo_meta = match github::fetch_metadata(&repo_ref) {
    Ok(meta) => RepoMetaState::Available(meta),
    Err(GithubError::NotFound) => {
        // 404 → abort (private OR missing — never claim to distinguish, per Phase 1 D-04)
        return Err(IntakeError::RepoNotFoundOrPrivate {
            owner: repo_ref.owner.clone(), repo: repo_ref.repo.clone(),
        });
    }
    Err(GithubError::RateLimited) | Err(GithubError::Network) => {
        eprintln!("🦀 Metadata không lấy được — vẫn điều tra bằng git nha");
        RepoMetaState::Unavailable   // explicit "unknown", NOT zeroed (D-03 / specifics)
    }
};
```

### Pattern 2: RAII clone workspace (D-01)
**What:** Own a `tempfile::TempDir`; clone into it; keep it alive for the whole collection; let `Drop` delete it (also fires on panic via unwinding).
**Example:**
```rust
// Source: tempfile docs + git2 Repository::clone (docs.rs/git2/0.21.0)
use tempfile::TempDir;
use git2::Repository;

let workspace = TempDir::new()?;                 // unique dir, auto-cleaned on drop
let url = format!("https://github.com/{}/{}.git", repo_ref.owner, repo_ref.repo);
let repo = Repository::clone(&url, workspace.path())
    .map_err(map_clone_error)?;                  // clone failure = HARD error
// ... do all git2 + scan work while `workspace` is in scope ...
// workspace dropped here → directory removed
```
**Gotcha:** Do NOT `std::process::exit()` while the `TempDir` is still in scope — `exit()` skips destructors and leaks the temp dir. Let `main` return / drop normally, OR keep the `exit(0)` only after the collection scope has ended.

### Pattern 3: Normalized snapshot with explicit degradation (§9)
**What:** One `InvestigationSnapshot` where missing metadata is an explicit variant, and bounded metrics carry a caveat flag.
(See §9 for the suggested struct shape.)

### Anti-Patterns to Avoid
- **Diffing full history for most-modified-file** → violates D-05 + Pitfall 1. Bound to 1000 commits, label the result.
- **Issuing >1 GitHub request** → Pitfall 2. One `GET /repos` only; everything else is local git2.
- **Zeroing degraded metadata** → must be an explicit "unknown" enum/Option (D-03 / specifics), not `0`.
- **Calling `std::process::exit()` before the `TempDir` drops** → leaks workspace.
- **Enabling tokei default features** → drags in CLI deps; use `default-features = false`.
- **Reading default branch from the API** → D-07/Discretion says use clone HEAD.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Temp dir + cleanup on panic | Manual `std::fs::create_dir` + cleanup | `tempfile::TempDir` | RAII handles drop AND unwinding panic; correct unique-name + permissions. |
| Git clone / history | Shell out to `git`, parse stdout | `git2::Repository` | Locked; no binary dep, typed objects, no string parsing. |
| Language line counting | Walk files, count newlines by extension | `tokei` lib | Handles comments, blanks, hundreds of languages, gitignore. |
| Epoch+offset → local datetime | Manual modular arithmetic | `chrono` `FixedOffset` + `DateTime` | git2 `Time` gives `(secs, offset_minutes)`; chrono assembles author-local time correctly for night/weekend buckets. |
| Author identity merging | Custom email-dedup | git2 `Mailmap::resolve_signature` (then lowercased-email fallback) | git2 reads `.mailmap` natively (verified). |

**Key insight:** Every "facts" capability already has a battle-tested crate. The only bespoke logic in this phase is the *filtering/aggregation rules* (bot exclusion, merge exclusion, bus-factor accumulation per METRICS.md) — those are pure functions over git2 output and belong in `repo/history.rs`.

## git2 0.21 HOW-TO (verified signatures this session)

> All signatures below were confirmed against docs.rs/git2/0.21.0 on 2026-06-02. `[VERIFIED: docs.rs/git2/0.21.0]`

### §1 Clone (full, to a path) + error mapping
```rust
// Repository::clone<P: AsRef<Path>>(url: &str, into: P) -> Result<Repository, git2::Error>
let repo = Repository::clone(&url, workspace.path())?;
```
- **Full clone is the default** — `Repository::clone` does NOT shallow-clone (libgit2 has no robust shallow support; this matches D-01 "full clone"). No depth option needed.
- **Error mapping:** inspect `git2::Error`. Use `err.class()` (`git2::ErrorClass`) and `err.code()` (`git2::ErrorCode`). With the API-first 404 check (D-02), the repo is known to exist+be public, so clone failures are almost always transport/network → map to `IntakeError::Network`. An auth/`ErrorClass::Http`/`ErrorCode::Auth` failure (rare for public) can also map to `Network` (generic "couldn't reach"). **Clone failure is a HARD error** (D-03) — do not degrade.
- For HTTPS clone to work at all you MUST have built git2 with the `https` or `vendored-openssl` feature (see Installation). Without it `clone` errors with an "unsupported URL protocol"-class transport error.

### §2 History walk — count + iterate most-recent N
```rust
// Revwalk: type Item = Result<Oid, git2::Error>  (verified)
// set_sorting(&mut self, sort: git2::Sort) -> Result<(), Error>
// push_head(&mut self) -> Result<(), Error>
use git2::Sort;

// TOTAL COMMITS (cheap, full history — D-06):
let mut walk = repo.revwalk()?;
walk.push_head()?;                       // start from HEAD (default branch tip)
let total_commits = walk.count();        // consumes the walk; counts all reachable

// MOST-RECENT N (default branch), newest-first — for bounded passes (D-05):
let mut walk = repo.revwalk()?;
walk.set_sorting(Sort::TIME)?;           // commit-time order, newest first
walk.push_head()?;
for oid in walk.take(1000) {             // cap at 1000 (D-05); .take handles ≤1000 repos
    let oid = oid?;
    let commit = repo.find_commit(oid)?;
    let sig = commit.author();           // Signature
    let name = sig.name().unwrap_or("");     // name() -> Result<&str, Error>
    let email = sig.email().unwrap_or("");   // email() -> Result<&str, Error>
    let when = sig.when();               // git2::Time
    let secs = when.seconds();           // i64 epoch seconds
    let offset_min = when.offset_minutes(); // i32 author-local TZ offset
    let parents = commit.parent_count(); // usize — >1 means a merge commit (drop it)
    // ...
}
```
- **Author time + timezone (for night/weekend/business-hours in author-local time, D-05):** `commit.author().when()` returns `git2::Time` where `seconds()` is epoch seconds and `offset_minutes()` is the committer's preferred-TZ offset. Build author-local time with chrono:
  ```rust
  use chrono::{DateTime, FixedOffset, Utc, TimeZone, Timelike, Datelike};
  let utc = Utc.timestamp_opt(secs, 0).single().unwrap();
  let tz = FixedOffset::east_opt(offset_min * 60).unwrap();
  let local: DateTime<FixedOffset> = utc.with_timezone(&tz);
  let hour = local.hour();                    // for night/business buckets
  let weekday = local.weekday();              // for weekend bucket
  ```
  Use **author** time (not committer) for "when the human worked." `[VERIFIED: docs.rs/git2/0.21.0 Commit/Signature/Time]`
- **Merge filtering:** `commit.parent_count() > 1` → merge commit → exclude from author counting (METRICS.md). For the bounded most-modified-file pass, also skip merges (diffing a merge against one parent is misleading).

### §3 First-commit date / repo age (cheapest correct way — 02-01 needs exactly this)
```rust
// Walk oldest-first, take the first item:
use git2::Sort;
let mut walk = repo.revwalk()?;
walk.set_sorting(Sort::TIME | Sort::REVERSE)?;   // REVERSE = oldest first
walk.push_head()?;
let first_oid = walk.next().ok_or(/* empty repo */)??;   // Option<Result<Oid,_>>
let first_commit = repo.find_commit(first_oid)?;
let first_secs = first_commit.author().when().seconds();
// repo age = now - first_secs, render with chrono
```
- `Sort::TIME | Sort::REVERSE` gives oldest-first; `.next()` is O(reachable) to seed the heap but returns the first immediately — cheap enough for the skeleton. (Bitflags OR works because `Sort` is a bitflags type in git2 0.21.) `[VERIFIED: docs.rs/git2/0.21.0 Revwalk::set_sorting]`
- **Walking-skeleton (02-01) scope:** clone → this snippet → `now - first_secs` → print one line. Nothing else.

### §4 Branch enumeration + default branch + tip dates (D-06)
```rust
// branches(&self, Option<BranchType>) -> Result<Branches, Error>
// Branches: type Item = Result<(Branch, BranchType), Error>  (verified)
use git2::BranchType;

// DEFAULT BRANCH from clone HEAD (Discretion: git, not API):
let head = repo.head()?;                      // Reference
let default_branch = head.shorthand().unwrap_or("HEAD").to_string();  // e.g. "main"

// ENUMERATE ALL BRANCHES of a fresh clone:
for b in repo.branches(Some(BranchType::Remote))? {
    let (branch, _bt) = b?;
    let name = branch.name()?.unwrap_or("");          // name() -> Result<Option<&str>, Error>
    let tip = branch.get().peel_to_commit()?;         // Reference::peel_to_commit -> Commit
    let tip_time = tip.author().when().seconds();     // for last-activity / stale candidates
    // skip "origin/HEAD" symbolic ref
}
```
- **CRITICAL — fresh-clone branch topology:** A `Repository::clone` checks out ONLY the default branch as a **local** branch. Every OTHER branch exists only as a **remote-tracking** ref under `refs/remotes/origin/*`. Therefore to enumerate ALL branches you MUST iterate `BranchType::Remote` (these are `origin/main`, `origin/dev`, ...), NOT `BranchType::Local` (which would show just the one checked-out branch). The default branch appears in both; de-dupe by stripping the `origin/` prefix and ignore the `origin/HEAD` symbolic pointer. This directly affects COLL-02 "total branches" — using `Local` only would report `1` for every repo. `[VERIFIED: git2 clone semantics + docs.rs/git2/0.21.0 branches]`
- **Last activity:** the max `tip_time` across remote branches is the most reliable git-only "last activity"; `pushed_at` from the API (§7) is a cross-check that degrades with the API.

### §5 Most-modified-file (bounded, last 1000 — D-05)
```rust
// diff_tree_to_tree(&self, a: Option<&Tree>, b: Option<&Tree>, opts: Option<&mut DiffOptions>)
//   -> Result<Diff, Error>
use std::collections::HashMap;
let mut counts: HashMap<String, u32> = HashMap::new();

// iterate the same last-1000 newest-first walk from §2:
for oid in walk.take(1000) {
    let commit = repo.find_commit(oid?)?;
    if commit.parent_count() != 1 { continue; }       // skip merges & root commit
    let this_tree = commit.tree()?;
    let parent_tree = commit.parent(0)?.tree()?;
    let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&this_tree), None)?;
    for delta in diff.deltas() {                       // name-only style
        if let Some(p) = delta.new_file().path() {
            *counts.entry(p.to_string_lossy().into_owned()).or_insert(0) += 1;
        }
    }
}
// most-modified = max-by value; LABEL result "based on last N commits"
```
- **Rename tracking is OFF** (locked) — do NOT call `Diff::find_similar`. A renamed file counts as add+delete; that's the accepted V1 behavior.
- **Perf:** this is the expensive pass (Pitfall 1). Each `diff_tree_to_tree` is roughly O(changed entries); capping at 1000 commits bounds it. On a 10k-commit repo (axum/tokio) this stays well under the slow-investigation threshold. Pass `None` for `DiffOptions` (defaults are name-level deltas — no content/hunk computation, which would be far slower).
- `delta.new_file().path()` is `Option<&Path>`; for deletes the new path is absent — use `old_file().path()` as fallback if you want to count deletions, but counting `new_file` paths matches "modified" semantics.

### §6 Contributors + bus-factor inputs (full history, shared filtering — METRICS.md)
```rust
use std::collections::HashMap;
let mailmap = repo.mailmap().ok();        // mailmap() -> Result<Mailmap, Error>; may be absent

let mut by_author: HashMap<String, u32> = HashMap::new();
let mut walk = repo.revwalk()?;
walk.push_head()?;                         // full history — cheap, no diffs (D-06)
for oid in walk {
    let commit = repo.find_commit(oid?)?;
    if commit.parent_count() > 1 { continue; }          // drop merge commits
    let raw_sig = commit.author();
    // identity normalization via mailmap if present, else lowercased email:
    let resolved = mailmap.as_ref()
        .and_then(|m| m.resolve_signature(&raw_sig).ok());
    let sig = resolved.as_ref().unwrap_or(&raw_sig);
    let name = sig.name().unwrap_or("");
    let email = sig.email().unwrap_or("").to_lowercase();
    if name.ends_with("[bot]") || is_bot_email(&email) { continue; }  // drop bots
    *by_author.entry(email).or_insert(0) += 1;          // key = normalized email
}
// contributor_count = by_author.len()
// bus_factor: sort DESC by count then name ASC, accumulate to >=50% (METRICS.md algorithm)
// FALLBACK: if filtering removed ALL authors, recompute without the bot filter (METRICS.md)
```
- **Mailmap IS exposed:** `Repository::mailmap()` returns `Result<Mailmap, Error>` and `Mailmap::resolve_signature(&Signature) -> Result<Signature<'static>, Error>` resolves to the real name+email. Verified this session. Group by lowercased email of the *resolved* signature; lowercased-email is the fallback when no `.mailmap`. `[VERIFIED: docs.rs/git2/0.21.0 Mailmap::resolve_signature]`
- **Bot list:** reuse the Infra bot convention — `*[bot]` suffix on name, plus known emails (`dependabot[bot]`, `renovate[bot]`, `github-actions[bot]`). METRICS.md mandates this filter be SHARED across contributor_count / top_author_share / bus_factor — implement it ONCE as a `fn is_authored_commit(commit) -> bool` + `fn normalized_identity(sig) -> String`.
- Bus factor uses **full** history (D-06) — counting never diffs, so it's cheap even on huge repos.

## github/ HOW-TO — single blocking REST call (§7)

```rust
// Source: docs.rs/reqwest/0.13.4 blocking::Client (verified) + GitHub REST docs
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RepoMetadata {
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub description: Option<String>,     // null in JSON → None
    #[serde(default)]
    pub topics: Vec<String>,
    pub default_branch: String,          // we IGNORE this in favor of git HEAD (Discretion)
    pub pushed_at: Option<String>,       // ISO-8601; cross-check for last activity
    pub created_at: Option<String>,
}

pub fn fetch_metadata(r: &RepoRef) -> Result<RepoMetadata, GithubError> {
    let client = Client::builder()
        .user_agent("rust-to-you/0.1")   // REQUIRED — no UA → GitHub returns 403 (verified)
        .build().map_err(|_| GithubError::Network)?;

    let url = format!("https://api.github.com/repos/{}/{}", r.owner, r.repo);
    let mut req = client.get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28");
    if let Ok(tok) = std::env::var("GITHUB_TOKEN") {       // D-04: use if present
        if !tok.is_empty() { req = req.bearer_auth(tok); }
    }

    let resp = req.send().map_err(|_| GithubError::Network)?;  // network/DNS/TLS → Network
    match resp.status() {
        StatusCode::OK => resp.json::<RepoMetadata>().map_err(|_| GithubError::Network),
        StatusCode::NOT_FOUND => Err(GithubError::NotFound),  // 404 → abort (D-02)
        StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => Err(GithubError::RateLimited),
        _ => Err(GithubError::Network),                       // anything else → transient
    }
}
```
- **User-Agent is mandatory:** GitHub docs (verified this session): *"Requests with no `User-Agent` header will be rejected. If you provide an invalid `User-Agent` header, you will receive a `403 Forbidden` response."* Set it on the builder so EVERY request carries it. `[CITED: docs.github.com/.../getting-started-with-the-rest-api]`
- **Distinguishing 404 vs 403-rate-limit vs network:**
  - `send()` returning `Err` = transport/network/DNS/TLS → `Network`.
  - `status() == 404` → `NotFound` → `RepoNotFoundOrPrivate` (Phase 1 D-04: never claim to tell private from missing).
  - `status() == 403` or `429` → `RateLimited`. (Pure rate-limit is 403 with `x-ratelimit-remaining: 0` or 429; you can read the header for a nicer message but it's optional for V1 — one call rarely hits the limit per D-04.)
  - any other non-2xx → treat as transient `Network`.
- **API host is `api.github.com`** (NOT `github.com`) — the REST base differs from the clone host. The clone URL is `https://github.com/{owner}/{repo}.git`.
- The blocking `Response::json::<T>()` exists in reqwest blocking and uses serde. `[CITED: docs.rs/reqwest/0.13.4]`

## scan/ HOW-TO

### §8 tokei (library) → language percentages
```rust
// Source: docs.rs/tokei/14.0.0 (verified)
use tokei::{Config, Languages, LanguageType};

let mut langs = Languages::new();
let paths = [workspace.path()];               // the cloned tree
let ignored = &[".git", "target"];            // gitignore-syntax excludes
langs.get_statistics(&paths, ignored, &Config::default());

// Languages derefs to BTreeMap<LanguageType, Language>; `.code` = code lines
let total: usize = langs.values().map(|l| l.code).sum();
let mut pct: Vec<(LanguageType, f64)> = langs.iter()
    .filter(|(_, l)| l.code > 0)
    .map(|(ty, l)| (*ty, 100.0 * l.code as f64 / total.max(1) as f64))
    .collect();
pct.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());   // descending
```
- `get_statistics(&mut self, paths: &[A: AsRef<Path>], ignored: &[&str], config: &Config)` — verified signature. `Config::default()` is the standard init. `[VERIFIED: docs.rs/tokei/14.0.0 Languages]`
- Use `l.code` (code lines) for percentages, not comments/blanks. Percentages should sum ~100 (Pitfall checklist: "percentages sum correctly"). Guard `total == 0` (empty repo) → empty language list, not a divide-by-zero.
- `default-features = false` to drop tokei's CLI deps (clap/colored/etc.).

### Infra footprints — path/glob presence (Discretion, no content parse)
8 signals (COLL-03 / ANLY-05). Detect by presence at the clone root (and known subpaths):

| Signal | Path/glob to test (relative to clone root) |
|--------|-------------------------------------------|
| Docker | `Dockerfile` OR `docker-compose.yml` OR `docker-compose.yaml` |
| Terraform | any `*.tf` (root or shallow scan) |
| GitHub Actions | `.github/workflows/` dir exists AND contains ≥1 `*.yml`/`*.yaml` |
| GitLab CI | `.gitlab-ci.yml` |
| CircleCI | `.circleci/config.yml` |
| Jenkins | `Jenkinsfile` |
| Dependabot | `.github/dependabot.yml` OR `.github/dependabot.yaml` |
| Renovate | `renovate.json` OR `.github/renovate.json` OR `.renovaterc` OR `.renovaterc.json` |

- Implement as `std::path::Path::exists()` checks + a small glob for `*.tf` / workflow files. Keep each detector a named function so the list is extensible (Pitfall: "hard-coding inline detectors" is acceptable only for the first few — wrap them).
- **Read-only:** all checks are `exists()` / read on the local clone. Never touch the remote (COLL-03 explicit).

## §9 InvestigationSnapshot — suggested shape (degradation explicit)

> Final field names are the planner's call (Discretion). This shape satisfies D-03 (explicit "unknown") and the "based on last N commits" caveat (specifics).

```rust
pub struct InvestigationSnapshot {
    pub repo: RepoRef,                       // from Phase 1
    pub metadata: RepoMetaState,             // degradable (D-03)
    pub history: HistoryFacts,               // always present (git2)
    pub branches: BranchFacts,               // always present (git2)
    pub filesystem: FilesystemFacts,         // always present (tokei + infra)
}

/// Explicit "unknown" — NOT zeroed (D-03 / specifics).
pub enum RepoMetaState {
    Available(RepoMetadata),                 // stars/forks/description/topics
    Unavailable,                             // transient API failure; report shows "unknown"
}

pub struct HistoryFacts {
    pub total_commits: usize,                // full history (D-06)
    pub repo_age_days: i64,                  // from first commit (§3)
    pub contributor_count: usize,            // filtered, full history
    pub bus_factor: usize,                   // METRICS.md
    pub top_author_share_pct: f64,
    pub window: CommitWindow,                // caveat carrier (below)
    pub most_modified_file: Option<String>,  // bounded pass (D-05)
    pub night_pct: f64,
    pub weekend_pct: f64,
    pub business_hours_pct: f64,
}

/// Travels with bounded metrics so Phase 3/4 can surface "based on last N commits".
pub struct CommitWindow {
    pub scanned: usize,                      // N actually walked (≤1000)
    pub capped: bool,                        // true if total_commits > 1000
}

pub struct BranchFacts {
    pub default_branch: String,              // clone HEAD (Discretion)
    pub branches: Vec<BranchInfo>,           // all remote-tracking, de-duped
    pub last_activity_secs: i64,             // max branch-tip time
}
pub struct BranchInfo { pub name: String, pub tip_time_secs: i64 }

pub struct FilesystemFacts {
    pub languages: Vec<(String, f64)>,       // name, pct — descending
    pub infra: InfraFootprints,
}
pub struct InfraFootprints {
    pub docker: bool, pub terraform: bool, pub github_actions: bool,
    pub gitlab_ci: bool, pub circleci: bool, pub jenkins: bool,
    pub dependabot: bool, pub renovate: bool,
}
```
- **`RepoMetaState::Unavailable`** is the literal mechanism for "degraded metadata representable as explicit unknown" (D-03). Phase 4 renders stars/forks as "unknown" when this variant is hit.
- **`CommitWindow { capped, scanned }`** is the "based on last N commits" caveat carrier — attach it to any metric derived from the bounded pass.

## Runtime State Inventory

Greenfield-with-prior-phase. This phase ADDS collection modules and a runtime artifact (the temp clone). No rename/migration of existing state.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None — V1 has no datastore (no cache, D-01). Each run clones fresh. | None |
| Live service config | None — no external service config owned by this tool. | None |
| OS-registered state | None. | None |
| Secrets/env vars | `GITHUB_TOKEN` is READ if present (D-04), never written/persisted. Phase 1 introduced no secrets. | Read-only env access; do NOT log the token (PITFALLS security: never log auth material). |
| Build artifacts | New crates added to `Cargo.toml`/`Cargo.lock` (git2 pulls libgit2-sys + OpenSSL via vendored). First build is slower. No stale artifact from Phase 1 to clean. | `cargo build` after dep add; expect longer first compile (vendored OpenSSL). |

**Runtime artifact created by this phase:** the temp clone dir under the OS temp location, owned by a `TempDir` and removed on drop. The only "state" to be careful about is NOT leaking it via `process::exit()` before the `TempDir` drops (Pattern 2 gotcha).

## Common Pitfalls

### Pitfall 1: git2 HTTPS clone fails because `https`/`vendored-openssl` feature is missing
**What goes wrong:** `Repository::clone` returns a transport error ("unsupported URL protocol" / TLS) on every `https://` clone.
**Why it happens:** git2 0.21 `default = []` — HTTPS is NOT compiled in by default (verified via crates.io feature inspection).
**How to avoid:** `git2 = { version = "0.21", features = ["vendored-openssl"] }`. Verify with a real clone of a tiny public repo in an early test.
**Warning signs:** Clone fails for ALL repos identically, even known-good ones.

### Pitfall 2: GitHub 403 because no User-Agent header
**What goes wrong:** Every metadata call returns 403; gets misclassified as `RateLimited`.
**Why it happens:** GitHub rejects UA-less requests (verified in GitHub docs).
**How to avoid:** `Client::builder().user_agent("rust-to-you/0.1")`. Set it on the client so all requests inherit it.
**Warning signs:** 403 even with a valid `GITHUB_TOKEN` and zero prior requests.

### Pitfall 3: Branch count is always 1 (used `BranchType::Local`)
**What goes wrong:** "Total branches" reports 1 for every repo.
**Why it happens:** A fresh clone checks out only the default branch locally; all others are remote-tracking refs.
**How to avoid:** Enumerate `BranchType::Remote`, strip `origin/` prefix, skip `origin/HEAD`. (See §4.)
**Warning signs:** Branch Jungle shows one branch on a known multi-branch repo.

### Pitfall 4: Expensive most-modified-file pass walks full history
**What goes wrong:** Slow investigations on large repos (Pitfall 1 in PITFALLS.md).
**Why it happens:** Diffing every commit, unbounded.
**How to avoid:** Cap at 1000 commits (D-05), `walk.take(1000)`, pass `None` DiffOptions (name-level deltas only), skip merges. Label result with `CommitWindow.capped`.
**Warning signs:** Most-modified-file pass dominates runtime; multi-second runs on tokio/axum-size repos.

### Pitfall 5: Temp clone leaks because of `process::exit()`
**What goes wrong:** Temp dirs accumulate across runs.
**Why it happens:** `std::process::exit()` skips destructors, so `TempDir::drop` never runs.
**How to avoid:** Keep the collection (and `TempDir`) inside a scope that returns a `Result` to `main`; only `exit(code)` AFTER the workspace has dropped. Or restructure `main` to map the result to an exit code at the very end.
**Warning signs:** `$TMPDIR` fills with `*.tmp` clone dirs.

### Pitfall 6: Bot-only repo zeroes out all author metrics
**What goes wrong:** A repo whose only committers are bots yields `contributor_count = 0`, `bus_factor` undefined.
**Why it happens:** Bot filter removes every author.
**How to avoid:** METRICS.md fallback — if filtering removes ALL authors, recompute the affected metric WITHOUT the bot filter.
**Warning signs:** Division by zero / empty author map on dependabot-heavy mirror repos.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `tui-rs` | `ratatui` (Phase 4) | — | Not this phase, but noted in STACK.md. |
| Shelling to `git` | git2/libgit2 binding | — | Locked; typed objects. |
| Async reqwest + tokio | reqwest `blocking` | This project | No runtime needed for one sequential call. |
| Manual extension line-count | tokei lib | — | Standard counter. |

**Deprecated/outdated:** none relevant to this phase. git2 0.21, reqwest 0.13, tokei 14 are all current as of 2026-06-02.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `git2::Diff::deltas()` + `delta.new_file().path()` gives the changed-path list usable for the most-modified tally | §5 | LOW — this is the documented diff iteration API; if the exact accessor differs, the executor adapts via docs.rs. Pattern is correct. |
| A2 | reqwest blocking `Response::json::<T>()` and `RequestBuilder::bearer_auth` exist with these signatures | §7 | LOW — standard reqwest API; verified `Client::builder().user_agent` + `get` + `send` directly, `json`/`bearer_auth` are long-stable. |
| A3 | `git2::Time` has `seconds()` (i64) and `offset_minutes()` (i32) accessors | §2 | LOW — docs confirm Time = (epoch secs, offset minutes); accessor names are the standard git2 ones. Executor verifies against docs.rs if a name differs. |
| A4 | 403 vs 429 both mean rate-limit for our purposes; reading `x-ratelimit-remaining` is optional in V1 | §7 | LOW — D-04 says one call rarely hits the limit; conflating them only affects the error message, not control flow. |
| A5 | `vendored-openssl` builds cleanly on the macOS dev box | Installation | MEDIUM — vendored OpenSSL needs a C compiler/perl at build time. If it fails, fall back to the `https` feature + system OpenSSL (`brew install openssl`). Planner should keep this fallback in the install task. |

## Open Questions

1. **Stale-branch threshold (days)** — still OPEN per STATE.md, but it's a **Phase 3** decision (Branch Jungle analysis), not Phase 2. Phase 2 only needs to COLLECT each branch's tip time; the cutoff is applied downstream. Recommendation: collect `tip_time_secs` per branch; defer the threshold to Phase 3.
2. **`pushed_at` vs git tip time for "last activity"** — both are available. Recommendation: prefer the git max-tip-time (always present, degrades gracefully); store `pushed_at` from the API as a secondary cross-check when metadata is Available.
3. **vendored vs system OpenSSL** — see A5. Recommendation: try `vendored-openssl` first; the planner keeps a one-line fallback note (`https` feature + brew openssl) in the dependency task.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain | build | ✓ | rustc 1.93.0 / cargo 1.93.0 | — |
| Network (HTTPS to api.github.com + github.com) | metadata call + clone | runtime-dependent | — | Metadata: degrade (D-03). Clone: hard fail (no repo = no report). |
| C compiler + perl (for `vendored-openssl` build) | git2 build | likely (Xcode CLT on macOS) | — | Use `https` feature + system OpenSSL (`brew install openssl@3`) |
| `GITHUB_TOKEN` | higher rate limit (optional) | optional | — | Run unauthenticated (D-04) |

**Missing dependencies with no fallback:** none at build time on this box (rustc 1.93 present). Runtime network for the CLONE has no fallback by design (D-03 — clone failure is fatal).
**Missing dependencies with fallback:** vendored OpenSSL build toolchain → system OpenSSL feature path.

## Validation Architecture

> nyquist_validation is enabled (config.json `workflow.nyquist_validation: true`). This section is REQUIRED.

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` / `cargo test` (no external test crate; matches Phase 1) |
| Config file | none — Cargo convention; unit tests live in `#[cfg(test)] mod tests` per file |
| Quick run command | `cargo test --lib` |
| Full suite command | `cargo test` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| COLL-02 | Repo age from first commit (02-01 skeleton metric) | unit (fixture git repo built in test) | `cargo test --lib repo::history::tests::repo_age -x` | ❌ Wave 0 |
| COLL-02 | Total-commit count over full history | unit (fixture repo) | `cargo test --lib repo::history::tests::total_commits` | ❌ Wave 0 |
| COLL-02 | Branch enumeration counts ALL branches (remote refs) | unit (fixture repo with 2+ branches, then clone) | `cargo test --lib repo::history::tests::branch_count` | ❌ Wave 0 |
| COLL-01 | contributor_count / bus_factor with bot+merge+identity filtering | unit (fixture repo with bot + dup-email commits) | `cargo test --lib repo::history::tests::bus_factor` | ❌ Wave 0 |
| COLL-02 | most-modified-file bounded + capped flag | unit (fixture repo >N commits or N param injected) | `cargo test --lib repo::history::tests::most_modified` | ❌ Wave 0 |
| COLL-03 | tokei language percentages sum ~100 | unit (temp dir with known files) | `cargo test --lib scan::lang::tests::percentages` | ❌ Wave 0 |
| COLL-03 | infra footprint detection per signal | unit (temp dir with marker files) | `cargo test --lib scan::infra::tests::detect` | ❌ Wave 0 |
| COLL-01 | GitHub status → IntakeError mapping (404→NotFound, 403→RateLimited, net→Network) | unit (pure mapping fn over StatusCode, NO live network) | `cargo test --lib github::tests::status_mapping` | ❌ Wave 0 |
| COLL-01 | serde decode of a sample `/repos` JSON payload | unit (static JSON fixture string) | `cargo test --lib github::tests::decode` | ❌ Wave 0 |

### What is deterministically testable vs needs network
- **Deterministic (no network) — the bulk of the phase:**
  - **git2 passes:** build a *fixture git repo in a `TempDir` inside the test* (`git2::Repository::init`, create commits with controlled author name/email/time via `Repository::commit` + crafted `Signature` with explicit `Time`, add merge commits, add a `[bot]` author, add duplicate-email authors, add a `.mailmap`). Then run the history/branch/bus-factor functions against it. For the "fresh clone topology" branch test, `Repository::clone` from a local `file://` path to a second TempDir — no network, exercises the remote-ref behavior.
  - **tokei:** point `get_statistics` at a `TempDir` containing a couple of known `.rs`/`.py` files; assert percentages.
  - **infra:** create marker files in a `TempDir`; assert each detector flag.
  - **GitHub error mapping + JSON decode:** make the status→error mapping a PURE function over `reqwest::StatusCode` (no I/O) and unit-test it; test serde decode against a static JSON string fixture.
- **Needs live network (do NOT put in the default suite):**
  - The actual `fetch_metadata` HTTP round-trip and the actual remote clone. **Recommendation:** keep network out of `cargo test` default. Either (a) make `fetch_metadata` take an injectable "transport" so a fake response drives the mapping in tests, or (b) gate any real-network test behind `#[ignore]` (run via `cargo test -- --ignored` manually). The walking-skeleton's real clone is verified manually (run the binary against a tiny public repo), not in CI.

### Sampling Rate
- **Per task commit:** `cargo test --lib` (+ `cargo clippy` per STACK.md dev tools)
- **Per wave merge:** `cargo test` (full)
- **Phase gate:** full suite green + `cargo clippy` clean + a manual end-to-end run of the 02-01 skeleton against a small public repo (e.g. `octocat/Hello-World`) before `/gsd-verify-work`.

### Wave 0 Gaps
- [ ] `tests`/in-module test scaffolding for `repo/history.rs` — a `fn make_fixture_repo() -> TempDir` helper that builds commits with controlled signatures/times (shared fixture for age/count/bus-factor/most-modified tests)
- [ ] `scan/lang.rs` + `scan/infra.rs` test modules with TempDir fixtures
- [ ] `github/` test module: a static `/repos` JSON sample string + a pure `classify(status) -> Result<_, GithubError>` fn to test without network
- [ ] No framework install needed — Rust built-in test harness (already used in Phase 1)

## Security Domain

> `security_enforcement: true`, `security_asvs_level: 1`, `security_block_on: high`. This phase makes network calls + clones untrusted remote content + reads env secrets, so security applies.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | partial | `GITHUB_TOKEN` read from env if present (D-04); never logged, never persisted. No auth UI. |
| V3 Session Management | no | No sessions; single CLI invocation. |
| V4 Access Control | no | Read-only public-repo tool; no privilege boundaries. |
| V5 Input Validation | yes | `owner`/`repo` already validated by Phase 1 `parse_repo_ref` (charset-restricted, rejects `..`/`\`). The clone URL is built from those validated segments — no path-traversal or URL-injection into the clone target. |
| V6 Cryptography | yes (transport) | HTTPS for both API (`api.github.com`) and clone (`github.com`) via rustls (reqwest) / OpenSSL (git2). No hand-rolled crypto. The FNV case_id hash (Phase 1) is non-security. |
| V7 Error/Logging | yes | Do NOT log `GITHUB_TOKEN` or full auth headers (PITFALLS security: "Logging full URLs or auth material"). Error messages are crab-voiced and must not echo the token. |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Token leakage via logs | Information Disclosure | Never `println!`/`eprintln!` the token or the `Authorization` header. Read env, use, drop. |
| Malicious repo content executed during clone/scan | Tampering / Elevation | git2 clone + tokei + `Path::exists()` are all READ-only; we never execute repo files, run hooks, or parse content as code. (Infra detection is presence-only — Discretion.) |
| Path traversal via crafted owner/repo | Tampering | Phase 1 validation already blocks `..`/`\`/control chars; clone target is a fresh `TempDir` we own. |
| Symlink escape from cloned tree during scan | Tampering | tokei/infra scan stays within `workspace.path()`; do not follow symlinks out of the clone (tokei respects this; infra `exists()` checks use joined paths under the clone root). |
| MITM on API/clone | Information Disclosure / Tampering | Enforce HTTPS (no `http://`); rustls/OpenSSL validate certs by default — do NOT disable cert verification. |
| TempDir info disclosure (clone contents readable by other users) | Information Disclosure | `tempfile` creates dirs with user-only permissions by default; acceptable for V1. |

## Sources

### Primary (HIGH confidence)
- crates.io API (with User-Agent) — verified versions + features this session: git2 0.21.0 (`default=[]`, `https`, `vendored-openssl` features), reqwest 0.13.4, tokei 14.0.0 (`default=["cli"]`, `has_lib=true`), tempfile 3.27.0, chrono 0.4.44, serde_json 1.0.150
- docs.rs/git2/0.21.0 — Repository (clone/revwalk/head/branches/mailmap/find_commit/diff_tree_to_tree), Revwalk (push_head/set_sorting/Item=Result<Oid>), Commit (author/time/parent_count/tree), Signature (name/email/when → Result/Time), Time ((secs, offset_min)), Mailmap (resolve_signature), Branches (Item=Result<(Branch,BranchType)>)
- docs.rs/reqwest/0.13.4 — blocking::Client builder/user_agent/get/send, RequestBuilder
- docs.rs/tokei/14.0.0 — Languages::new/get_statistics, Config::default, BTreeMap deref
- docs.github.com REST `/repos/{owner}/{repo}` — field names (stargazers_count, forks_count, description, topics, default_branch, created_at, pushed_at), 404 for missing/private
- docs.github.com getting-started — User-Agent REQUIRED, UA-less request rejected, invalid UA → 403 (quoted)
- `cargo search git2` → 0.21.0 (registry confirm)

### Secondary (MEDIUM confidence)
- slopcheck `install` run — 5/6 [OK], tokei [SUS] (typosquat-similarity false positive vs tokio)

### Tertiary (LOW confidence)
- Diff delta iteration accessor exact names (A1) — pattern verified, exact method to be confirmed by executor against docs.rs if needed

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all versions + critical features verified on crates.io this session.
- git2/reqwest/tokei HOW-TO: HIGH — core signatures verified on docs.rs; a couple of accessor names (Time, Diff delta) flagged in Assumptions Log as LOW-risk.
- Architecture/snapshot shape: HIGH (constrained by locked decisions + ARCHITECTURE.md).
- Pitfalls: HIGH — the two dominant ones (git2 https feature, GitHub UA) are verified facts, not folklore.

**Research date:** 2026-06-02
**Valid until:** 2026-07-02 (stable ecosystem; git2/reqwest/tokei major versions are current)
