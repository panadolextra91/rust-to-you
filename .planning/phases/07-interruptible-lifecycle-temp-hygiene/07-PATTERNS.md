# Phase 7: Interruptible Lifecycle & Temp Hygiene - Pattern Map

**Mapped:** 2026-06-05
**Files analyzed:** 6 (2 NEW, 4 modified)
**Analogs found:** 5 / 6 (1 greenfield: `tests/interrupt.rs` — no `tests/` dir exists)

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| NEW `src/repo/hygiene.rs` | utility / lifecycle module | file-I/O + process-global state | `src/app/collect.rs` (pure `size_decision` + inline `#[cfg(test)]`) | role-match (pure-helper + tests template) |
| MOD `src/repo/clone.rs` | resource (RAII) | file-I/O | itself (existing `CloneWorkspace` + `clone_repo`) | exact (in-place edit) |
| MOD `src/main.rs` | entrypoint / wiring | request-response (single-shot) | itself (existing `main()` flow) | exact (in-place edit) |
| MOD `Cargo.toml` | config | — | existing `[dependencies]` block | exact |
| MOD `src/i18n.rs` | presentation helper (reuse, likely no edit) | transform | existing `bi()` / `two_line()` callers in `collect.rs` | exact (call pattern, not file edit) |
| NEW `tests/interrupt.rs` | test (integration) | event-driven (subprocess + signal) | **none** — no `tests/` dir; closest is inline `#[cfg(test)]` in `collect.rs` | no analog (greenfield) |

**Module registration (confirmed):** `src/lib.rs` uses flat `pub mod NAME;` declarations; `src/repo/mod.rs` currently declares `pub mod clone; pub mod history; pub mod branches;`. The new module is registered by adding **`pub mod hygiene;`** to `src/repo/mod.rs` (mirrors the three existing siblings). No `lib.rs` change needed if module lives under `repo/`.

---

## Pattern Assignments

### NEW `src/repo/hygiene.rs` (utility / lifecycle, file-I/O + process-global state)

**Primary analog:** `src/app/collect.rs` — the pure `size_decision()` helper (lines 13–35) plus its inline `#[cfg(test)]` module (lines 132–182). This is the exact template for "a pure, unit-testable decision/IO helper that lives next to its tests, with a hardcoded threshold `const`."

**Pure-helper + hardcoded-const pattern** — copy from `collect.rs:11-35`:
```rust
const MAX_REPO_KB: u64 = 500 * 1024;   // <- D-07 analog: ORPHAN_MAX_AGE is a const, no flag/env

#[derive(Debug, PartialEq, Eq)]
pub enum SizeDecision { Proceed, WarnDeep { size_mb: u64 }, WarnUnknown, TooLarge { size_mb: u64 } }

pub fn size_decision(meta_state: &RepoMetaState, deep: bool) -> SizeDecision {
    match meta_state { /* pure: no IO, fully testable */ }
}
```
→ New code mirrors this with `pub const ORPHAN_MAX_AGE: Duration = Duration::from_secs(60*60);` and the pure `sweep_orphans(dir, now, max_age) -> usize` (RESEARCH Pattern 3, lines 219–248). Keep the function `pub` and pure (inject `now` + `dir`) so it is unit-testable exactly like `size_decision`.

**Inline test-module pattern** — copy structure from `collect.rs:132-182`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // local fixture builder (analog: mock_metadata) → for hygiene: build a tempfile dir tree
    #[test]
    fn test_size_decision_boundary() { /* >= threshold semantics; round-up note */ }
}
```
→ New tests: `sweep_orphans` (stale removed, fresh kept, non-matching ignored), `sweep_orphans_empty` (returns 0 → silent), `cleanup_idempotent`, `panic_cleans_temp`. Use a `tempfile` fixture dir and pass a synthetic `now` (e.g. `SystemTime::now()` and dirs created "in the past" by shifting `now` forward) — **no `filetime` dev-dep needed** because `now` is injected into the pure fn (RESEARCH Validation Architecture, line 482).

**`>=` boundary + round-up precedent** (`collect.rs:23-31` and test `collect.rs:157-168`): the existing helper deliberately uses strict-greater + `div_ceil` and tests the exact boundary. Mirror this rigor for the age sweep: use `age >= max_age` (RESEARCH Pattern 3 line 240) and test the 60-min boundary.

**Process-global state + idempotent cleanup** (RESEARCH Pattern 4, lines 250–270) — no direct codebase analog (greenfield std pattern); copy verbatim from RESEARCH:
```rust
static LIVE_TEMP: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();
pub fn register_live_temp(p: PathBuf) { *slot().lock().unwrap() = Some(p); }
pub fn clear_live_temp()              { *slot().lock().unwrap() = None; }
pub fn cleanup_live_temp() {            // idempotent via .take()
    if let Some(p) = slot().lock().unwrap().take() { let _ = std::fs::remove_dir_all(&p); }
}
```

**Signal handler + panic hook** (RESEARCH Patterns 1 & 2, lines 171–217) — greenfield; the i18n call inside each follows the `collect.rs` bilingual pattern below.

---

### MOD `src/repo/clone.rs` (resource / RAII, file-I/O)

**Analog:** the file itself — current `CloneWorkspace` struct (lines 7–11) and `clone_repo` (lines 13–27).

**Existing struct + WARNING comment to replace** (lines 4–11):
```rust
/// WARNING: callers must not std::process::exit() while a CloneWorkspace is alive,
/// because exit skips the Drop trait and will leak the temp directory.
pub struct CloneWorkspace {
    #[allow(dead_code)]
    tmp: tempfile::TempDir,
    pub repo: git2::Repository,
}
```
→ D-08 edit: in `clone_repo` (after the `tempdir()` succeeds at line 17), call `crate::repo::hygiene::register_live_temp(tmp.path().to_path_buf());` **before** `git2::Repository::clone` (so a signal during the clone is covered). Add `impl Drop for CloneWorkspace` that calls `crate::repo::hygiene::clear_live_temp();` — `TempDir`'s own Drop still removes the files; clearing the slot prevents the handler racing it (RESEARCH lines 373–400). Update/remove the now-obsolete WARNING comment since the leak window is closed by the global path.

**Existing tempdir creation to hook into** (lines 14–18):
```rust
let tmp = tempfile::Builder::new()
    .prefix("rust-to-you-clone-")   // <- this prefix is the sweep's match key (PREFIX const)
    .tempdir()
    .map_err(|_| IntakeError::Network)?;
// INSERT: register_live_temp(tmp.path().to_path_buf());  <- D-08
```

---

### MOD `src/main.rs` (entrypoint / wiring, request-response)

**Analog:** the file itself — current `main()` (lines 6–23): `Args::parse()` → `parse_repo_ref` → `InvestigationSession::new` → `app::run` → `std::process::exit`.

**Existing flow to wire into** (lines 6–17):
```rust
fn main() {
    let args = Args::parse();
    match parse_repo_ref(&args.repo) {
        Ok(repo_ref) => {
            let session = InvestigationSession::new(repo_ref, args.deep);
            if let Err(e) = app::run(&session) { eprintln!("{}", e); std::process::exit(e.exit_code()); }
            std::process::exit(0);
        }
        Err(e) => { eprintln!("{}", e); std::process::exit(e.exit_code()); }
    }
}
```
→ D-04/D-03/D-05 edits: at the **very top of `main()`, before `Args::parse()`**, insert in order: `hygiene::install_panic_hook();`, `hygiene::install_signal_handler();`, then the startup sweep (RESEARCH "Wiring it all in main()", lines 346–371):
```rust
let removed = hygiene::sweep_orphans(&std::env::temp_dir(), SystemTime::now(), hygiene::ORPHAN_MAX_AGE);
if removed > 0 { /* print ONE bilingual line via i18n::two_line — D-02 silent if 0 */ }
```
**D-08 safety note for planner:** the three existing `std::process::exit()` calls (lines 14, 16, 21) all run *after* `app::run`/`collect()` has returned — the workspace is already Dropped — so they do NOT leak. The new global path exists only to cover the signal/panic-during-`collect()` window. Do not add any new `process::exit` while a workspace is alive without routing through `cleanup_live_temp()`.

---

### MOD `Cargo.toml` (config)

**Analog:** existing `[dependencies]` block (lines 18–29) and `[profile.dist]` (lines 50–52).

**Existing dependency style to match** (lines 19–24):
```toml
clap = { version = "4.4", features = ["derive"] }
git2 = { version = "0.21", features = ["https", "vendored-openssl"] }
tempfile = "3"
```
→ Add under `[dependencies]`, same `{ version, features }` table style:
```toml
ctrlc = { version = "3.5", features = ["termination"] }
```
→ Add a `[dev-dependencies]` section (does not yet exist) for the integration test's signal sender — `nix` (with `signal` feature) or `libc` (RESEARCH line 483). Prefer the minimal one the planner picks; `libc = "0.2"` is the lightest for `libc::kill(pid, SIGINT)`.

**`[profile.dist]` guard note** (lines 50–52): confirmed `inherits = "release"`, `lto = "thin"` — it does **NOT** set `panic = "abort"`, so default unwind is in effect and `Drop` runs. The panic hook is robust either way (RESEARCH Pitfall 4). Planner: add a comment that dist must not enable `panic = "abort"`, or accept the hook covers it.

---

### MOD `src/i18n.rs` — reuse only (likely NO file edit)

**Analog:** the bilingual call site in `collect.rs:58-72` — the canonical "build a Bilingual + emit two eprintln lines" pattern used for the Phase 6 `--deep` / unknown-size warnings:
```rust
let w = crate::i18n::two_line(&crate::i18n::bi(
    format!("🦀 Repo lớn ({} MB), Ferris vẫn đào vì --deep — sẽ lâu đó", size_mb),
    format!("Large repo ({} MB) — Ferris digs anyway (--deep); this will take a while", size_mb),
));
eprintln!("{}", w[0]);
eprintln!("{}", w[1]);
```
→ The 3 new lines (sweep notice, interrupt goodbye, panic apology) call `i18n::two_line(&i18n::bi(vi, en))` exactly like this — the sweep line uses `format!` for the `{removed}` count (analog uses `format!` for `size_mb`); the interrupt/panic lines can use string literals. **`bi()` accepts `impl Into<String>`** (`i18n.rs:16`) so both `&str` and `String` work — no signature change. Keep Ferris third-person (never "tôi/mình"). `i18n.rs` itself needs no edit.

---

## Shared Patterns

### Bilingual user notice (presentation)
**Source:** `src/i18n.rs:16-22` (`bi` / `two_line`) + call-site analog `src/app/collect.rs:58-65`.
**Apply to:** all 3 new user-facing lines (sweep notice in `main.rs`, interrupt line in `install_signal_handler`, panic line in `install_panic_hook`).
```rust
let w = crate::i18n::two_line(&crate::i18n::bi(vi, en));
eprintln!("{}", w[0]); eprintln!("{}", w[1]);
```

### Hardcoded threshold constant (config-as-code)
**Source:** `src/app/collect.rs:11` (`const MAX_REPO_KB: u64 = 500 * 1024;`).
**Apply to:** `hygiene.rs` — `pub const ORPHAN_MAX_AGE: Duration = Duration::from_secs(60*60);` and `const PREFIX: &str = "rust-to-you-clone-";`. No flag, no env (D-07 mirrors Phase 6 stance).

### Pure helper + inline `#[cfg(test)]` (testability)
**Source:** `src/app/collect.rs:21-35` (pure fn) + `:132-182` (inline tests with local fixture builder + boundary test).
**Apply to:** `sweep_orphans` (inject `dir` + `now`), `cleanup_live_temp` (idempotent). Tests use a `tempfile` fixture and synthetic `now` — no `filetime` dev-dep.

### Best-effort / never-block error handling (file-I/O)
**Source:** `src/app/collect.rs:47-50` — transient metadata error is logged and degraded, never aborts (`RepoMetaState::Unavailable`). Same "degrade, don't die" stance.
**Apply to:** `sweep_orphans` (D-06: `let Ok(entries) = read_dir(dir) else { return 0 };`, skip per-entry errors) and `cleanup_live_temp` (ignore `remove_dir_all` result). Sweep/cleanup failure never blocks the run.

### Module registration
**Source:** `src/repo/mod.rs` (`pub mod clone; pub mod history; pub mod branches;`).
**Apply to:** add `pub mod hygiene;` as a fourth sibling. (If planner instead puts it at `src/hygiene.rs`, add `pub mod hygiene;` to `src/lib.rs` alongside the existing flat decls.)

---

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| NEW `tests/interrupt.rs` | test (integration) | event-driven (subprocess + OS signal) | No `tests/` dir exists — confirmed (`ls tests/` → absent). This is the project's **first** integration test. No in-repo template for `std::process::Command` spawn + `libc::kill` + exit-status assertion. Closest in-repo reference is the inline `#[cfg(test)]` structure in `collect.rs:132-182` (use-`super::*`, `#[test]` fns, local fixture), but the subprocess/signal mechanics come from **RESEARCH Validation Architecture (lines 470–483)**, not the codebase. Planner: treat as greenfield; spawn `rust-to-you <repo>`, poll temp for `rust-to-you-clone-*`, send SIGINT/SIGTERM via dev-dep (`libc`/`nix`), assert no orphan + exit status 130. Mark `#[ignore]` if it needs network. |

The process-global `OnceLock<Mutex<Option<PathBuf>>>`, signal handler, and panic hook code also have no codebase analog — they are pure std/`ctrlc` patterns supplied verbatim by RESEARCH.md (Patterns 1, 2, 4). The planner should copy those from RESEARCH, not search for an analog.

---

## Metadata

**Analog search scope:** `src/app/`, `src/repo/`, `src/i18n.rs`, `src/main.rs`, `src/lib.rs`, `src/repo/mod.rs`, `Cargo.toml`, `tests/`
**Files scanned:** 7 (all confirmed against RESEARCH.md's cited analogs — no discrepancies)
**Pattern extraction date:** 2026-06-05
