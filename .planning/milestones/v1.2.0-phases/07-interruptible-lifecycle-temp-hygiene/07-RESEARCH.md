# Phase 7: Interruptible Lifecycle & Temp Hygiene - Research

**Researched:** 2026-06-05
**Domain:** Sync (non-async) Rust CLI — signal handling, panic hooks, RAII temp-dir lifecycle, filesystem sweep
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Startup sweep is **age-based** — remove `rust-to-you-clone-*` temp dirs whose mtime is older than the threshold. Chosen over PID-tagging and over unconditional prefix-wipe (would kill a concurrently-running instance). A live instance keeps its dir's mtime fresh (git writes continuously), so age-based is safe even for long `--deep` runs.
- **D-02:** Ferris prints a **single bilingual (VI+EN) line only when ≥1 dir was actually removed**. If nothing was swept, stay silent. Use `i18n::two_line`/`bi`.
- **D-03:** On SIGINT/SIGTERM mid-run: print **one bilingual Ferris line**, clean up the live temp dir, then exit with code **130**. SIGTERM follows the same path. If no workspace is alive yet, still exit 130 cleanly.
- **D-04:** Install a **panic hook** that (a) cleans up the live temp dir and (b) prints a friendly bilingual Ferris apology line, **suppressing the default backtrace unless `RUST_BACKTRACE` is set**.
- **D-05:** Sweep runs **early in `main()`, before any clone** (ideally before metadata fetch).
- **D-06:** Sweep is **best-effort** — a dir that fails to remove is skipped silently; sweep failure never blocks or aborts the run.
- **D-07:** Age threshold is **60 minutes**, a **hardcoded constant** (no env var, no flag).
- **D-08:** Establish a **single source of truth for the live temp path** — set when `CloneWorkspace` is created, cleared on its `Drop` — so the signal handler AND panic hook both clean the same path. `main.rs` must not `std::process::exit()` while a workspace is alive without going through cleanup.

### Claude's Discretion

- Signal crate choice: `ctrlc` vs `signal-hook` vs raw `libc` (must be sync — no async runtime).
- Mechanism for the shared live-temp-path source of truth: global `OnceLock<Mutex<…>>`, a registry, or an RAII guard that registers/deregisters.
- Whether to also add an explicit RAII guard in addition to the global path (defense in depth), as long as D-08's guarantee holds.
- Exact bilingual wording of the three new lines (sweep / interrupt / panic) — keep Ferris third-person voice; confirm via `i18n::two_line`.

### Deferred Ideas (OUT OF SCOPE)

- **GUARD-04** — time/commit budget bounding of `--deep` runs. Explicitly deferred.
- **`--clean` manual subcommand** — not requested; the automatic startup sweep already self-heals.
- Any change to the size guard or parser (Phase 6, done).
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CLEAN-01 | Interrupting a run (Ctrl-C / SIGINT / SIGTERM) never leaves an orphaned clone temp directory. | `ctrlc` crate with `termination` feature catches SIGINT+SIGTERM+SIGHUP on a **dedicated thread** (handler may safely call `fs::remove_dir_all` + `eprintln!`); cleans the shared live-temp path then `process::exit(130)`. See Standard Stack, Pattern 1. |
| CLEAN-02 | On startup, the tool sweeps away orphaned temp dirs from prior crashed/killed runs. | Pure `sweep_orphans(dir, now, max_age) -> usize` over `std::env::temp_dir()` (confirmed == tempfile's base) filtering `rust-to-you-clone-*` by `Metadata::modified()` mtime; best-effort. See Pattern 3 + Validation Architecture. |
| CLEAN-03 | No exit path (incl. panic/abort) leaves a clone workspace alive without cleanup. | `std::panic::set_hook` runs **before** unwind (so Drop still runs after on unwind → cleanup must be **idempotent**); plus the shared single-source-of-truth path (D-08) covers the `process::exit`-skips-Drop gap. See Pattern 2 + Pattern 4. |
</phase_requirements>

## Summary

This phase closes the temp-dir leak surface for a **synchronous** Rust CLI (tokio was deliberately dropped — confirmed in `PROJECT.md` §Key Decisions and `Cargo.toml`). The crux, confirmed in code: `CloneWorkspace` (`src/repo/clone.rs`) wraps a `tempfile::TempDir` with prefix `rust-to-you-clone-` and relies on `Drop` for cleanup. The `ws` local lives only inside `collect()` (`src/app/collect.rs`) — the `InvestigationSnapshot` is built from `ws.repo` facts but does **not** own `ws`, so the temp dir Drops the moment `collect()` returns. Therefore the leak window is precisely a **signal or panic that strikes during `collect()`**; normal completion is already covered by Drop, and all current `std::process::exit()` calls in `main.rs` run *after* `collect()` has returned (workspace already gone). The new global cleanup path exists to cover the signal/abort-during-`collect()` window that Drop cannot reach.

Three mechanisms, one shared source of truth: (1) a **startup sweep** removes stale orphans before any clone; (2) a **signal handler** (`ctrlc` with the `termination` feature) cleans the live path then exits 130; (3) a **panic hook** cleans the live path and prints a friendly bilingual line while suppressing the scary backtrace. All three read the same shared live-temp-path slot, which is set when `CloneWorkspace` is created and cleared on its `Drop`. Because the panic hook runs *before* unwind (so Drop also runs afterward) and the signal handler can race the TempDir's own Drop, the single non-negotiable invariant is that **cleanup must be idempotent** (`remove_dir_all` on an already-gone path must be a silent no-op).

**Primary recommendation:** Use `ctrlc = "3.5"` with `features = ["termination"]` (catches SIGINT+SIGTERM+SIGHUP, runs the callback on a **dedicated thread** so normal Rust code including `fs` and `eprintln!` is safe). Store the live temp path in a `static LIVE_TEMP: OnceLock<Mutex<Option<PathBuf>>>`. Register an idempotent `cleanup_live_temp()` helper from both the `ctrlc` handler and a `std::panic::set_hook`. Extract a pure `sweep_orphans(dir, now, max_age_secs) -> usize` for unit testing, and add one subprocess integration test that SIGINTs the binary mid-run and asserts no orphan remains.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Startup orphan sweep | Process/Filesystem (main early-init) | — | Runs once at `main()` entry over `env::temp_dir()`, before any session/network/clone work (D-05). |
| Signal capture (SIGINT/SIGTERM) | OS/Process (signal layer) | — | Inherently a process-lifecycle concern; lives at the `main()`/registration layer, not inside `collect()` or the TUI. |
| Live temp-path registry | Process-global state | RAII (`CloneWorkspace`) | One shared slot read by handler + hook; the RAII type owns set-on-create / clear-on-Drop. |
| Panic interception | Process/Runtime (panic hook) | RAII (Drop on unwind) | Hook runs before unwind; Drop is the redundant second pass — hence idempotency. |
| Temp-dir cleanup execution | Filesystem | — | `fs::remove_dir_all`, idempotent, best-effort. |
| Bilingual user notices | Presentation (`i18n`) | — | Reuse existing `i18n::two_line`/`bi`; Ferris third-person voice. |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `ctrlc` | `3.5` (latest 3.5.2, pub 2026-02-10, 99.9M total dls) | Cross-platform SIGINT/SIGTERM/SIGHUP handler that runs a callback on a **dedicated thread** | The de-facto crate for "run cleanup on Ctrl-C" in sync CLIs; far simpler than `signal-hook` for this exact need. `[VERIFIED: crates.io registry]` `[CITED: docs.rs/ctrlc]` |
| `std::panic` | std | `set_hook` for the panic path (D-04) | Standard library; no dependency. `[CITED: doc.rust-lang.org/std/panic]` |
| `std::sync::OnceLock` + `Mutex` | std (1.70+) | Shared single source of truth for the live temp path (D-08) | No external dep; safe interior mutability for a process-global. `[ASSUMED]` (std API, stable) |
| `tempfile` | `3` (locked 3.27.0) | Already in use by `CloneWorkspace`; `Builder::tempdir()` creates inside `env::temp_dir()` | Already a dependency. Confirms sweep root. `[VERIFIED: Cargo.lock]` `[CITED: docs.rs/tempfile]` |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `std::fs` / `std::path` | std | `read_dir`, `metadata().modified()`, `remove_dir_all` for sweep + cleanup | Always (sweep + idempotent cleanup). `[ASSUMED]` |
| `std::time::SystemTime` | std | Compare mtime vs `now - 60min` for the age sweep | Sweep filtering. `[ASSUMED]` |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `ctrlc` | `signal-hook` (0.4.4) | More flexible (iterator over signals, more signals), but you wire your own thread/loop or `signal-hook` iterator; heavier for a single "cleanup + exit" callback. Pick it only if you later need richer signal multiplexing. `[VERIFIED: crates.io registry]` |
| `ctrlc` | raw `libc::signal` / `sigaction` | Forces you into the **async-signal-safe** straitjacket: inside a raw handler you may NOT allocate, call `eprintln!` (locks), or call most libc/`fs` — you'd have to set an `AtomicBool` and poll it elsewhere, which doesn't fit a blocking `git2::clone`. Strongly discouraged here. `[CITED: POSIX signal-safety; man 7 signal-safety]` |
| Global `OnceLock<Mutex<Option<PathBuf>>>` | Registry / RAII-only | A registry is over-engineering for exactly one live workspace at a time. RAII-only cannot be reached from a signal handler/panic hook (which have no `&self`), so a process-global is required regardless; the RAII guard layers on top of it. |

**Installation:**
```bash
# Add to Cargo.toml [dependencies]:
ctrlc = { version = "3.5", features = ["termination"] }
```

**Version verification:**
- `ctrlc`: latest `3.5.2`, published 2026-02-10, ~99.9M total / 16.8M recent downloads — verified via crates.io API. `[VERIFIED: crates.io registry]`
- `signal-hook` (alternative): latest `0.4.4`, ~43M recent downloads — verified via crates.io API. `[VERIFIED: crates.io registry]`
- `tempfile`: locked at `3.27.0` in `Cargo.lock`. `[VERIFIED: Cargo.lock]`

> Note: `cargo search` reported `ctrlc = "3.5.2"`, `signal-hook = "0.4.4"`, `libc = "1.0.0-alpha.3"`. The `signal-hook 0.4.4` and `ctrlc 3.5.2` values were cross-confirmed against the crates.io API. `libc 1.0.0-alpha.3` is a pre-release line; not needed (we are not using raw libc).

## Package Legitimacy Audit

| Package | Registry | Age | Downloads | Source Repo | slopcheck | Disposition |
|---------|----------|-----|-----------|-------------|-----------|-------------|
| `ctrlc` | crates.io | published 2026-02-10 (mature crate, 99.9M total dls) | 16.8M recent | github.com/Detegr/rust-ctrlc | N/A (PyPI-only tool) | **Approved** |
| `signal-hook` | crates.io | mature (43M recent dls) | 43M recent | github.com/vorner/signal-hook | N/A (PyPI-only tool) | Approved (alternative, not selected) |

**slopcheck caveat (cross-ecosystem false positive — documented):** `slopcheck 0.6.1` only queries **PyPI**. Run against these Rust crate names it returned `[SLOP] signal-hook ("does not exist on pypi")` and `[SUS] ctrlc ("only 13 downloads on pypi")`. **Both verdicts are PyPI false positives and are disregarded** — these are crates.io packages, and the authoritative check is the crates.io registry API (both confirmed present with massive download counts and linked source repos above). This is exactly the cross-ecosystem confusion the legitimacy protocol warns about; the correct registry for this phase is **crates.io**, verified via `cargo search` + crates.io API.

**Packages removed due to slopcheck [SLOP] verdict:** none (the SLOP verdict was a cross-ecosystem PyPI false positive, not a crates.io result).
**Packages flagged as suspicious [SUS]:** none on the correct registry.

## Architecture Patterns

### System Architecture Diagram

```
                          ┌─────────────────────────────────────────┐
   process start ───────▶ │ main()                                   │
                          │  1. install_panic_hook()  (D-04)         │
                          │  2. ctrlc::set_handler(cleanup+exit130)  │  (D-03, registers
                          │  3. sweep_orphans(temp_dir, now, 60min)  │   handler on a
                          │     └─ if removed>0: print 1 VI+EN line  │   DEDICATED THREAD)
                          │  4. parse args → InvestigationSession    │
                          └───────────────────┬─────────────────────┘
                                              │ app::run(&session)
                                              ▼
                          ┌─────────────────────────────────────────┐
                          │ collect()  ◀── THE ONLY LIVE WINDOW      │
                          │   ws = clone_repo()                      │
                          │     └─ CloneWorkspace::new:              │
                          │         set LIVE_TEMP = Some(path) ◀─────┼──┐ shared
                          │   ...git2 history / scan over ws.repo... │  │ single source
                          │   return snapshot  (ws Drops here:       │  │ of truth
                          │     remove dir + clear LIVE_TEMP=None) ──┼──┘ OnceLock<Mutex<Option<PathBuf>>>
                          └───────────────────┬─────────────────────┘     ▲          ▲
                                              │ Ok(snapshot)              │          │
                                              ▼                  reads &  │          │ reads &
                          ┌──────────────────────────┐          clears    │          │ clears
                          │ tui::render → exit(0)     │                    │          │
                          └──────────────────────────┘          ┌─────────┴───┐  ┌───┴──────────┐
                                                                │ SIGINT/TERM │  │ panic hook   │
   ─ ─ ─ async events that can fire DURING collect(): ─ ─ ─ ─ ▶│ handler:    │  │ (before      │
                                                                │ cleanup()→  │  │ unwind):     │
                                                                │ exit(130)   │  │ cleanup()+   │
                                                                └─────────────┘  │ friendly msg │
                                                                                 └──────────────┘
              cleanup() = remove LIVE_TEMP path if Some  ── IDEMPOTENT (no-op if already gone)
```

### Component Responsibilities

| File | Responsibility (this phase) |
|------|------------------------------|
| `src/main.rs` | Install panic hook + signal handler, run startup sweep — all before `app::run`. No `process::exit` while a workspace is alive (D-08 guarantee holds because exits already happen post-`collect()`). |
| `src/repo/clone.rs` | `CloneWorkspace::new` registers the live temp path; its `Drop` runs idempotent cleanup AND clears the shared slot. Remove/replace the stale WARNING comment. |
| New module (e.g. `src/lifecycle.rs` or `src/repo/hygiene.rs`) | `LIVE_TEMP` static, `register_live_temp(path)`, `clear_live_temp()`, idempotent `cleanup_live_temp()`, pure `sweep_orphans(dir, now, max_age) -> usize`, `install_signal_handler()`, `install_panic_hook()`. Keep pure helpers `pub` for unit tests. |
| `src/i18n.rs` | Source the three new bilingual lines via `bi()` + `two_line()` (no signature change needed). |

### Recommended Project Structure
```
src/
├── main.rs                 # wires sweep + handler + hook at entry
├── repo/
│   ├── clone.rs            # CloneWorkspace: register on new, idempotent cleanup + clear on Drop
│   └── hygiene.rs          # NEW: LIVE_TEMP static, sweep_orphans, cleanup_live_temp,
│                           #      install_signal_handler, install_panic_hook  (pure parts tested)
tests/
└── interrupt.rs            # NEW: subprocess SIGINT integration test (asserts no orphan remains)
```

### Pattern 1: Signal handler that cleans then exits 130
**What:** Register one `ctrlc` handler (with `termination` feature) that runs the idempotent cleanup, prints the bilingual interrupt line, and exits 130.
**When to use:** Once, at `main()` start, before any work.
**Why safe:** `ctrlc` runs the callback on a **dedicated thread**, not in async-signal context — so `eprintln!`, allocation, and `fs::remove_dir_all` are all permitted.
```rust
// Source: docs.rs/ctrlc + CONTEXT D-03
// ctrlc = { version = "3.5", features = ["termination"] }
pub fn install_signal_handler() {
    // termination feature => fires for SIGINT, SIGTERM, SIGHUP
    let _ = ctrlc::set_handler(|| {
        // runs on a dedicated thread: normal Rust is safe here
        cleanup_live_temp();                 // idempotent
        let w = crate::i18n::two_line(&crate::i18n::bi(
            "🦀 Ferris dọn dẹp rồi rút nha",
            "Ferris cleaned up — bye",
        ));
        eprintln!("{}", w[0]);
        eprintln!("{}", w[1]);
        std::process::exit(130);             // 128 + SIGINT(2)
    });
}
```
> **Exit-code note (D-03 vs convention):** D-03 says exit **130 for both** SIGINT and SIGTERM. The shell convention is 130 for SIGINT (128+2) and **143** for SIGTERM (128+15). `ctrlc`'s single callback does not tell you which signal fired, so 130-for-both is the natural implementation and matches D-03. Planner: keep D-03 (130 for both) unless the user wants per-signal codes (which would require dropping to `signal-hook`).

### Pattern 2: Panic hook with cleanup + suppressed backtrace
**What:** A `std::panic::set_hook` that cleans the live temp, prints a friendly bilingual line, and only shows the backtrace if `RUST_BACKTRACE` is set.
**When to use:** Once, at `main()` start (install **before** the signal handler is fine; order between them does not matter).
**Why idempotent matters:** The hook runs **before** unwinding begins, so on an unwinding panic the `CloneWorkspace::Drop` will ALSO run afterward and try to clean the same path → double cleanup. `cleanup_live_temp()` must treat "already gone" as success.
```rust
// Source: doc.rust-lang.org/std/panic/fn.set_hook + CONTEXT D-04
pub fn install_panic_hook() {
    let default = std::panic::take_hook();   // capture default for backtrace passthrough
    std::panic::set_hook(Box::new(move |info| {
        cleanup_live_temp();                 // idempotent; Drop may also run on unwind
        let w = crate::i18n::two_line(&crate::i18n::bi(
            "🦀 Ơ Ferris vấp ngã rồi, nhưng đã dọn temp xong xuôi",
            "Ferris tripped over a bug, but cleaned up the temp dir",
        ));
        eprintln!("{}", w[0]);
        eprintln!("{}", w[1]);
        // Only show the scary trace if the user asked for it.
        if std::env::var_os("RUST_BACKTRACE").is_some() {
            default(info);
        }
    }));
}
```

### Pattern 3: Pure age-based orphan sweep
**What:** A pure function over a directory + clock that returns how many orphans it removed; the `main()` wrapper calls it with `env::temp_dir()` and `SystemTime::now()`.
**When to use:** Early in `main()` (D-05), before metadata/clone.
```rust
// Source: docs.rs/tempfile (base = env::temp_dir) + CONTEXT D-01/D-06/D-07
pub const ORPHAN_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(60 * 60);
const PREFIX: &str = "rust-to-you-clone-";

/// Pure: enumerate `dir`, remove `rust-to-you-clone-*` dirs older than `max_age`. Best-effort.
pub fn sweep_orphans(dir: &std::path::Path, now: std::time::SystemTime,
                     max_age: std::time::Duration) -> usize {
    let mut removed = 0;
    let Ok(entries) = std::fs::read_dir(dir) else { return 0 };   // D-06: never block
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if !name.starts_with(PREFIX) { continue; }
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_dir() { continue; }
        let Ok(mtime) = meta.modified() else { continue };
        match now.duration_since(mtime) {
            Ok(age) if age >= max_age => {
                if std::fs::remove_dir_all(entry.path()).is_ok() { removed += 1; } // skip on err (D-06)
            }
            _ => {} // fresh (live instance keeps mtime warm — D-01) or clock skew: leave it
        }
    }
    removed
}
```

### Pattern 4: Idempotent cleanup + shared source of truth (D-08)
```rust
// Source: CONTEXT D-08 + std::sync docs
use std::sync::{Mutex, OnceLock};
use std::path::PathBuf;

static LIVE_TEMP: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();
fn slot() -> &'static Mutex<Option<PathBuf>> { LIVE_TEMP.get_or_init(|| Mutex::new(None)) }

pub fn register_live_temp(p: PathBuf) { *slot().lock().unwrap() = Some(p); }
pub fn clear_live_temp()              { *slot().lock().unwrap() = None; }

/// Idempotent: safe to call from signal handler, panic hook, AND Drop. No-op if already gone.
pub fn cleanup_live_temp() {
    // take() so a racing second caller sees None and does nothing
    let path = slot().lock().unwrap().take();
    if let Some(p) = path {
        let _ = std::fs::remove_dir_all(&p); // "already gone" => Ok or harmless Err, ignored
    }
}
```
> `Mutex::lock().unwrap()` inside a panic hook can theoretically poison if a previous panic happened while the lock was held; the window is tiny (set/clear/take are non-panicking). If the planner wants belt-and-suspenders, use `lock().unwrap_or_else(|e| e.into_inner())` to recover a poisoned guard.

### Anti-Patterns to Avoid
- **Raw `libc::signal` + work inside the handler:** violates async-signal-safety (no alloc, no `eprintln!`/stdio locks, no `fs`). `ctrlc`'s dedicated thread sidesteps this entirely.
- **Letting the signal handler and `TempDir::Drop` both `remove_dir_all` non-idempotently:** double-free-style spurious error logs. Make cleanup idempotent (use `.take()`).
- **Printing the sweep line unconditionally:** D-02 requires silence when 0 dirs removed.
- **Adding an env var / flag for the 60-min threshold:** D-07 says hardcoded constant only.
- **Re-introducing `process::exit` while a workspace is alive without routing through cleanup:** the whole point of D-08.
- **Spawning `git` as a subprocess for cleanup:** the project's security posture (THREAT-MODEL.md) is "no shell, no git subprocess" — cleanup is pure `std::fs`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Catch SIGINT/SIGTERM cross-platform | Raw `sigaction` + self-pipe + poll loop | `ctrlc` (termination feature) | Dedicated-thread callback removes the async-signal-safety minefield; macOS+Linux+Windows handled. |
| Temp-dir creation/teardown | Manual `mkdtemp` + recursive rm | `tempfile::TempDir` (already used) | RAII Drop already correct for the normal path. |
| Recursive directory removal | Hand-rolled walk + unlink | `std::fs::remove_dir_all` | Handles nested git objects; idempotent enough when wrapped. |
| File mtime cross-platform | Platform `stat` FFI | `std::fs::Metadata::modified()` | Stable std API; works on macOS + Linux. |
| Backtrace suppression | Parse/strip default output | Custom `set_hook` that simply doesn't print the trace | A custom hook fully replaces default output — you choose what to print. |

**Key insight:** Every piece of this phase is std + one tiny, ubiquitous crate. The risk is not "missing library" — it is **ordering and idempotency** (hook-before-unwind double cleanup, handler-vs-Drop race). Get those right and there is nothing custom to build.

## Runtime State Inventory

> Rename/refactor lens applied because this phase touches lifecycle plumbing. Most categories are N/A (no rename), but the orphan-state question is the heart of the phase.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | **Orphaned temp dirs on disk:** `${TMPDIR}/rust-to-you-clone-*` from prior killed runs (Phase 6 UAT left a real 5 MB orphan). Verified base dir == `std::env::temp_dir()` (tempfile docs). At research time the dev machine had **0** orphans present (checked `${TMPDIR}/rust-to-you-clone-*`). | Startup sweep (D-01) removes those older than 60 min. |
| Live service config | None — no external services, daemons, or registries. Verified: project is a single-shot local CLI, no async runtime, one GitHub API call. | None. |
| OS-registered state | None — no launchd/systemd/Task Scheduler entries; no saved process manager state. Verified by source tree (no service install code). | None. |
| Secrets/env vars | `RUST_BACKTRACE` is **read** by the new panic hook (presence gate) but not written. `GITHUB_TOKEN` (existing, unrelated) untouched. No secret renames. | None (read-only use of `RUST_BACKTRACE`). |
| Build artifacts | Adding `ctrlc` updates `Cargo.lock`. No egg-info/binary-name churn. | `cargo build` regenerates lock; commit it. |

**The canonical question — "after every file is updated, what runtime systems still hold the old state?":** Only on-disk orphan temp dirs, which the startup sweep is explicitly designed to reclaim. Nothing else persists.

## Common Pitfalls

### Pitfall 1: Double cleanup (panic hook before unwind, then Drop)
**What goes wrong:** Panic hook cleans the temp dir, then unwinding runs `CloneWorkspace::Drop` (and `TempDir::Drop`) which try to clean the *same* path again → spurious error or log noise.
**Why it happens:** `set_hook` runs **before** the panic runtime/unwinding (confirmed in std docs). On the default `unwind` profile, Drop still executes.
**How to avoid:** Make `cleanup_live_temp()` idempotent via `.take()` (second caller gets `None`); ignore `remove_dir_all` errors.
**Warning signs:** "No such file or directory" errors printed on panic; flaky cleanup tests.

### Pitfall 2: Async-signal-safety violation (if tempted by raw libc)
**What goes wrong:** Doing `eprintln!`/alloc/`fs` inside a raw POSIX handler can deadlock (stdio/heap lock held by interrupted code).
**Why it happens:** Raw handlers run in the interrupted thread's signal context; only async-signal-safe calls are legal.
**How to avoid:** Use `ctrlc` — its callback runs on a **separate dedicated thread**, so normal Rust is safe.
**Warning signs:** Intermittent hangs on Ctrl-C under load.

### Pitfall 3: Sweep deletes a concurrent instance's live dir
**What goes wrong:** A second `rust-to-you` running a long `--deep` clone gets its temp dir wiped by another instance's startup sweep.
**Why it happens:** Unconditional prefix-wipe ignores liveness.
**How to avoid:** Age-based sweep (D-01) — a live clone keeps mtime fresh (git writes continuously), so it is never older than 60 min while active. Use `>= max_age` and skip on clock-skew (`duration_since` Err).
**Warning signs:** Concurrent runs fail mid-clone with "directory removed."

### Pitfall 4: `[profile.dist]` silently enabling `panic = "abort"`
**What goes wrong:** If a future dist profile sets `panic = "abort"`, unwinding stops — but the **panic hook still runs** (it runs before the abort), so cleanup is preserved. Drop would NOT run under abort, making the hook the sole cleanup path.
**Why it happens:** cargo-dist profiles can override panic strategy.
**How to avoid:** Confirmed: current `[profile.dist]` (`inherits = "release"`, `lto = "thin"`) does **NOT** set `panic = "abort"` — so default unwind is in effect (Drop runs). The panic-hook design is robust either way. Planner: add a guard note that dist must not set abort, or accept that the hook covers it.
**Warning signs:** Temp leaks only in release/dist binaries, not debug.

### Pitfall 5: TempDir::Drop racing the signal handler
**What goes wrong:** Signal arrives exactly as `collect()` returns; both the handler thread and the main thread's Drop try to remove the dir.
**Why it happens:** `ctrlc` callback runs on its own thread concurrently with main.
**How to avoid:** Idempotent `cleanup_live_temp()` with `.take()` under the mutex serializes the two; whoever wins removes, the loser no-ops.
**Warning signs:** Rare double-remove errors under stress.

### Pitfall 6: Windows file-handle lock (note only — primary target is macOS/Linux)
**What goes wrong:** On Windows, `git2::Repository` may hold an open handle inside the temp dir, blocking `remove_dir_all`.
**Why it happens:** Windows mandatory file locking.
**How to avoid:** Best-effort cleanup (D-06) already swallows the error; the next startup sweep reclaims it once handles are released. Primary targets are `aarch64/x86_64-apple-darwin` + `x86_64-unknown-linux-gnu` (Cargo.toml `targets`), where this is a non-issue. Document, don't engineer around it.

## Code Examples

### Wiring it all in main() (D-05 ordering)
```rust
// Source: composed from CONTEXT D-03/D-04/D-05 + existing src/main.rs flow
fn main() {
    rust_to_you::repo::hygiene::install_panic_hook();      // D-04
    rust_to_you::repo::hygiene::install_signal_handler();  // D-03

    // D-05: sweep before any clone (ideally before metadata too)
    let removed = rust_to_you::repo::hygiene::sweep_orphans(
        &std::env::temp_dir(),
        std::time::SystemTime::now(),
        rust_to_you::repo::hygiene::ORPHAN_MAX_AGE,
    );
    if removed > 0 {                                       // D-02: silent if 0
        let w = rust_to_you::i18n::two_line(&rust_to_you::i18n::bi(
            format!("🦀 Ferris dọn {removed} temp cũ từ lần trước nha"),
            format!("Ferris swept {removed} stale temp dir(s) from a previous run"),
        ));
        eprintln!("{}", w[0]);
        eprintln!("{}", w[1]);
    }

    let args = Args::parse();
    // ... existing parse_repo_ref → InvestigationSession → app::run → exit ...
}
```

### CloneWorkspace registers/clears the shared path (D-08)
```rust
// Source: edit of src/repo/clone.rs per CONTEXT D-08
pub struct CloneWorkspace {
    tmp: tempfile::TempDir,
    pub repo: git2::Repository,
}

pub fn clone_repo(repo_ref: &RepoRef) -> Result<CloneWorkspace, IntakeError> {
    let url = format!("https://github.com/{}/{}.git", repo_ref.owner, repo_ref.repo);
    let tmp = tempfile::Builder::new()
        .prefix("rust-to-you-clone-")
        .tempdir()
        .map_err(|_| IntakeError::Network)?;
    crate::repo::hygiene::register_live_temp(tmp.path().to_path_buf()); // D-08: set on create
    let repo = git2::Repository::clone(&url, tmp.path()).map_err(|_| IntakeError::Network)?;
    Ok(CloneWorkspace { tmp, repo })
}

impl Drop for CloneWorkspace {
    fn drop(&mut self) {
        // TempDir's own Drop removes the dir; we just clear the shared slot so the
        // handler/hook don't try to re-remove a path that's already being torn down.
        crate::repo::hygiene::clear_live_temp(); // idempotent w/ cleanup_live_temp's take()
    }
}
```
> Note: `TempDir` removes on its own Drop; clearing the slot first (or letting `cleanup`'s `.take()` win) prevents the handler from racing it. Planner decides whether `Drop` calls `clear_live_temp()` (slot bookkeeping) or `cleanup_live_temp()` (also removes) — clearing is sufficient since `TempDir::Drop` removes the files.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Raw `libc::signal` handlers | `ctrlc` dedicated-thread callback | mature (ctrlc 3.x) | Removes async-signal-safety constraints for cleanup work. |
| `lazy_static!`/`once_cell` for globals | `std::sync::OnceLock` (std since 1.70) | Rust 1.70 (2023) | No external dep for the shared path slot. |
| Manual backtrace stripping | Custom `panic::set_hook` (replaces default output) | stable std | Choose exactly what to print; gate on `RUST_BACKTRACE`. |

**Deprecated/outdated:** Nothing in the recommended stack is deprecated. `once_cell` is superseded by std `OnceLock` for this use; no need to add it.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `OnceLock` + `Mutex` available on the project's Rust toolchain (≥1.70) | Standard Stack | Low — if toolchain is older, use `once_cell`; trivially swappable. Edition 2021 + recent crates imply ≥1.70. |
| A2 | Exact bilingual wording of the three lines is illustrative, not final | Patterns 1–3, main() | None — D (discretion) explicitly leaves wording to the planner; must keep Ferris third-person + `two_line`. |
| A3 | Exit-130-for-both (SIGINT and SIGTERM) is acceptable per D-03 | Pattern 1 | Low — D-03 states it explicitly; flagged the 143 convention for the user to confirm. |
| A4 | `std::fs` calls (`read_dir`, `modified`, `remove_dir_all`) behave consistently on macOS+Linux | Patterns 3–4 | Low — stable std; Windows handle-lock noted as best-effort (Pitfall 6). |

## Open Questions

1. **Exit code for SIGTERM: 130 (per D-03) or 143 (shell convention)?**
   - What we know: D-03 says 130 for both; `ctrlc`'s single callback cannot distinguish signals.
   - What's unclear: whether the user prefers convention-correct 143 for SIGTERM.
   - Recommendation: keep D-03 (130 for both). Only revisit (→ `signal-hook`) if per-signal codes are wanted.

2. **Should `CloneWorkspace::Drop` clear-only or also remove?**
   - What we know: `TempDir::Drop` already removes the files; the slot just needs clearing.
   - What's unclear: defense-in-depth preference.
   - Recommendation: `Drop` calls `clear_live_temp()`; rely on `cleanup_live_temp()`'s `.take()` to serialize against the handler. Planner's call (D discretion).

3. **Where does the new module live — `src/repo/hygiene.rs` or `src/lifecycle.rs`?**
   - Recommendation: `src/repo/hygiene.rs` (co-located with `clone.rs`, which owns the temp dir). Cosmetic; planner decides.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain (`cargo`) | build + tests | ✓ | (project builds: `cargo test --no-run` succeeded) | — |
| `ctrlc` crate | signal handling | ✓ (crates.io) | 3.5.2 | `signal-hook` 0.4.4 |
| `tempfile` crate | temp dirs | ✓ (locked) | 3.27.0 | — |
| `std::env::temp_dir()` writable | sweep target | ✓ | — | — |

**Missing dependencies with no fallback:** none.
**Missing dependencies with fallback:** signal crate (`ctrlc` preferred; `signal-hook` is the documented fallback).

## Validation Architecture

> nyquist_validation is enabled (config.json `workflow.nyquist_validation: true`).

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Built-in Rust test harness (`cargo test`) — `#[test]` unit tests live inline in modules; integration tests go in `tests/`. |
| Config file | none (Cargo built-in); no `tests/` dir exists yet → Wave 0 creates `tests/interrupt.rs`. |
| Quick run command | `cargo test --lib hygiene` (or `cargo test sweep`) — sub-second pure-function tests |
| Full suite command | `cargo test` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CLEAN-02 | Sweep removes `rust-to-you-clone-*` dirs older than 60 min; leaves fresh + non-matching | unit (pure `sweep_orphans` over `tempfile` fixture) | `cargo test --lib sweep_orphans` | ❌ Wave 0 |
| CLEAN-02 | Sweep returns 0 (silent) when nothing stale | unit | `cargo test --lib sweep_orphans_empty` | ❌ Wave 0 |
| CLEAN-03 | `cleanup_live_temp()` is idempotent (2nd call no-ops, no error) | unit | `cargo test --lib cleanup_idempotent` | ❌ Wave 0 |
| CLEAN-03 | Panic hook cleans the live temp (hook→cleanup path) | unit (`catch_unwind` + assert dir gone + slot None) | `cargo test --lib panic_cleans_temp` | ❌ Wave 0 |
| CLEAN-01 | SIGINT mid-run leaves no orphan + exits 130 | integration (subprocess) | `cargo test --test interrupt sigint_cleans` | ❌ Wave 0 |
| CLEAN-01 | SIGTERM mid-run leaves no orphan | integration (subprocess) | `cargo test --test interrupt sigterm_cleans` | ❌ Wave 0 |

### What can ONLY be integration-tested
- **CLEAN-01 (real signal delivery + `process::exit`):** must spawn the actual binary, send the signal, and assert. A unit test cannot exercise the OS signal path nor `process::exit(130)` (which would kill the test runner). Use `std::process::Command` to launch `rust-to-you <some-repo>`, sleep until a `rust-to-you-clone-*` dir appears in temp (poll, bounded), send SIGINT via `libc::kill`/`nix::sys::signal` or `Command` + `.id()`, wait, then assert no orphan remains and exit status == 130. Point it at a small/slow-enough clone or a controllable fixture; mark `#[ignore]` if it needs network, and run in CI explicitly.
- **Everything else is pure/unit-testable** by extracting `sweep_orphans` (pure over dir+clock) and `cleanup_live_temp` (idempotent), tested with `tempfile` fixtures and a hand-set mtime (`filetime` crate or `set` via a fresh dir + `now - 61min`).

### Sampling Rate
- **Per task commit:** `cargo test --lib` (fast pure tests)
- **Per wave merge:** `cargo test` (incl. integration; allow `--include-ignored` if the signal test is `#[ignore]`)
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps
- [ ] `tests/interrupt.rs` — subprocess SIGINT/SIGTERM integration test (covers CLEAN-01)
- [ ] Inline `#[cfg(test)]` in the new hygiene module — `sweep_orphans`, `cleanup_live_temp`, panic-hook cleanup (covers CLEAN-02, CLEAN-03)
- [ ] Decide mtime-setting approach for fixtures: add `filetime` dev-dependency, or create dir then assert with `now` shifted (pass a synthetic `now` into the pure `sweep_orphans` — preferred, no dev-dep needed)
- [ ] For sending the signal in the integration test: `libc` or `nix` as a dev-dependency (`libc::kill(pid, SIGINT)`), or shell out to `kill`

## Security Domain

> `security_enforcement: true`, `security_asvs_level: 1` in config.json.

### Applicable ASVS Categories
| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No auth surface in this phase. |
| V3 Session Management | no | No sessions. |
| V4 Access Control | no | Local CLI, no multi-user. |
| V5 Input Validation | partial | Sweep only acts on entries matching the fixed `rust-to-you-clone-` prefix; never deletes outside that pattern; never follows user-controlled paths. |
| V6 Cryptography | no | No crypto. |
| V12 Files & Resources | **yes** | This is the phase's security core: bounded deletion (prefix + age + dir-only), best-effort (no privilege escalation), no symlink-following surprises (`remove_dir_all` on a matched dir in `env::temp_dir()`). |

### Known Threat Patterns for {sync Rust CLI / filesystem cleanup}
| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Sweep deletes unintended dirs (over-broad match) | Tampering / DoS (user data loss) | Strict `starts_with("rust-to-you-clone-")` + `is_dir()` + age gate; only inside `env::temp_dir()`. |
| Concurrent instance's live dir deleted | DoS | Age-based liveness (D-01): live dirs stay mtime-fresh. |
| Symlink/TOCTOU in temp dir | Tampering | Operate only on prefix-matched entries in the system temp dir; `remove_dir_all` on a same-name dir; do not resolve user-supplied paths. (Low risk: names are tool-generated by `tempfile`.) |
| Cleanup spawning a shell/git | Injection | None — cleanup is pure `std::fs`; honors THREAT-MODEL.md "no shell, no git subprocess." |
| Signal handler doing unsafe work | DoS (deadlock) | `ctrlc` dedicated thread, not async-signal context. |

## Sources

### Primary (HIGH confidence)
- `docs.rs/ctrlc` — confirmed: dedicated signal-handling thread runs the callback; `termination` feature adds SIGTERM+SIGHUP; macOS/Linux/Windows support. `[CITED]`
- `doc.rust-lang.org/std/panic/fn.set_hook` — confirmed: hook runs **before** the panic runtime/unwinding; custom hook replaces default backtrace output; `take_hook` to capture default. `[CITED]`
- `docs.rs/tempfile` (`Builder`) — confirmed: `tempdir()` creates inside `env::temp_dir()` (sweep root). `[CITED]`
- crates.io API — `ctrlc` 3.5.2 (2026-02-10, 99.9M total dls), `signal-hook` 0.4.4 (43M recent). `[VERIFIED]`
- Codebase reads — `src/repo/clone.rs`, `src/app/collect.rs`, `src/app/run.rs`, `src/main.rs`, `src/i18n.rs`, `Cargo.toml`, `Cargo.lock`, `src/error.rs`, `docs/THREAT-MODEL.md`, `.planning/config.json`. All CONTEXT.md assumptions **confirmed**. `[VERIFIED]`

### Secondary (MEDIUM confidence)
- POSIX async-signal-safety constraint (`man 7 signal-safety`) — basis for preferring `ctrlc` over raw libc. `[CITED]`

### Tertiary (LOW confidence)
- None outstanding.

## Project Constraints (from PROJECT.md / THREAT-MODEL.md)

- **No async runtime** (tokio dropped) → signal handling MUST be sync. `ctrlc`'s thread-based model satisfies this without an executor.
- **No shell, no git subprocess** (THREAT-MODEL.md) → cleanup is pure `std::fs`, no `git clean`/`rm` shell-out.
- **Ferris third-person, bilingual VI+EN two-line** → all three new lines via `i18n::two_line`/`bi`; never "tôi/mình".
- **Hardcoded-constant, minimal-surface stance** (Phase 6 precedent, D-07) → 60-min threshold is a `const`, no flag/env.
- No `CLAUDE.md` and no `.claude/skills/` present in the repo — no additional project directives to honor.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — `ctrlc` behavior (dedicated thread, termination feature) and `tempfile` base dir verified against official docs + crates.io registry.
- Architecture: HIGH — all CONTEXT assumptions confirmed by direct source reads; leak window (during `collect()`) verified in `collect.rs` ownership.
- Pitfalls: HIGH — double-cleanup (hook-before-unwind) and idempotency confirmed via std docs; async-signal-safety is well-established.

**Research date:** 2026-06-05
**Valid until:** 2026-07-05 (stable std + mature crate; 30-day horizon)
