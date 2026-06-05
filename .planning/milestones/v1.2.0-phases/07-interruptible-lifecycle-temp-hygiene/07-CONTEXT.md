# Phase 7: Interruptible Lifecycle & Temp Hygiene - Context

**Gathered:** 2026-06-05
**Status:** Ready for planning

<domain>
## Phase Boundary

No run — interrupted, crashed, or completed — ever leaves an orphaned clone temp
directory on the user's machine. Three guarantees: (1) Ctrl-C / SIGINT / SIGTERM
mid-run cleans up the live clone temp dir before exit; (2) on startup, the tool
sweeps away orphaned temp dirs left by previously crashed/killed runs (self-heal);
(3) no process exit path — including panic — leaves a clone workspace alive without
cleaning it up first.

**In scope:** signal handling (SIGINT/SIGTERM) with temp cleanup + bilingual notice,
startup sweep of orphaned `rust-to-you-clone-*` temp dirs (age-based), panic hook
with cleanup + friendly bilingual message, and closing the `std::process::exit()`
-skips-`Drop` gap so no exit path leaks the workspace.

**Out of scope (deferred / other):** GUARD-04 (time/commit budget bounding of `--deep`
runs) — explicitly deferred; any change to the size guard or parser (Phase 6, done);
a manual `--clean` subcommand (not requested — startup sweep covers self-healing).

</domain>

<decisions>
## Implementation Decisions

### Startup temp sweep (CLEAN-02)
- **D-01:** Sweep is **age-based** — at startup, remove `rust-to-you-clone-*` temp dirs
  whose mtime is older than the threshold. Chosen over PID-tagging (cross-platform PID
  liveness is awkward on macOS) and over unconditional prefix-wipe (would kill a
  concurrently-running instance's live temp dir). A live instance keeps its dir's mtime
  fresh (git writes continuously during clone), so age-based is safe even for long
  `--deep` runs.
- **D-07:** The age threshold is **60 minutes**, a **hardcoded constant** (no env var,
  no flag) — same "tighten + document, minimal surface" stance as Phase 6's `MAX_REPO_KB`.
- **D-05:** The sweep runs **early in `main()`, before any clone** (and ideally before
  the metadata fetch) so prior leaks are reclaimed at the start of every run.
- **D-06:** The sweep is **best-effort** — a dir that fails to remove (permission, busy)
  is skipped silently; sweep failure **never blocks or aborts** the run.

### Sweep notification (CLEAN-02)
- **D-02:** Ferris prints a **single bilingual (VI+EN) line only when ≥1 dir was actually
  removed** (e.g. "🦀 Ferris dọn N temp cũ từ lần trước nha / Ferris swept N stale temp
  dir(s) from a previous run"). If nothing was swept, **stay silent**. Reinforces the
  self-healing feel without noise. Use the existing `i18n::two_line`/`bi` helpers.

### Interrupt UX (CLEAN-01)
- **D-03:** On SIGINT/SIGTERM mid-run: print **one bilingual Ferris line** ("🦀 Ferris
  dọn dẹp rồi rút nha / cleaning up, bye"), **clean up the live temp dir**, then exit with
  code **130** (the conventional 128 + SIGINT). SIGTERM follows the same path. If no
  workspace is alive yet (e.g. interrupt during metadata fetch), still exit 130 cleanly.

### Panic behavior (CLEAN-03)
- **D-04:** Install a **panic hook** that (a) cleans up the live temp dir and (b) prints a
  friendly bilingual Ferris apology line, **suppressing the default scary backtrace unless
  `RUST_BACKTRACE` is set**. Keeps the cute vibe even on a crash while still honoring the
  cleanup guarantee.

### Exit-path safety (CLEAN-03)
- **D-08:** Establish a **single source of truth for the live temp path** — set when the
  `CloneWorkspace` is created, cleared on its `Drop` — so the signal handler AND the panic
  hook both clean the same path. `main.rs` must not `std::process::exit()` while a
  workspace is alive without going through cleanup. (Note: today the workspace is alive
  only inside `collect()` and Drops on normal return — see code_context — so the current
  `process::exit()` calls in `main.rs` run *after* the workspace is gone; the new global
  cleanup path covers the signal/abort-during-`collect()` window that Drop cannot.)

### Claude's Discretion
- Signal crate choice: `ctrlc` vs `signal-hook` vs raw `libc` (planner/researcher decide;
  must be sync — no async runtime per project decision to drop tokio).
- Mechanism for the shared live-temp-path source of truth: global `OnceLock<Mutex<…>>`,
  a small registry, or an RAII guard that registers/deregisters — planner's call.
- Whether to also add an explicit RAII guard in addition to the global path (defense in
  depth) — discretion, as long as D-08's guarantee holds.
- Exact bilingual wording of the three new lines (sweep / interrupt / panic) — keep Ferris
  third-person voice; confirm via `i18n::two_line`.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope & requirements
- `.planning/ROADMAP.md` — Phase 7 section (goal + 3 success criteria)
- `.planning/REQUIREMENTS.md` §CLEAN — CLEAN-01/02/03 text
- `.planning/PROJECT.md` §Key Decisions — locked: drop tokio / reqwest blocking (no async
  runtime → signal handling must be sync); bilingual VI+EN two-line; Ferris third-person
  narrator

### Phase 6 (precedent for style/constants)
- `.planning/phases/06-safe-intake-pre-flight-guard/06-CONTEXT.md` — hardcoded-constant +
  "tighten + document, minimal surface" precedent (mirrored by D-07)
- `docs/THREAT-MODEL.md` — the oversize-clone / temp-leak surface this phase closes

No external specs/ADRs beyond the planning docs — decisions fully captured above.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/repo/clone.rs` — `CloneWorkspace` already wraps `tempfile::TempDir` (prefix
  `rust-to-you-clone-`); cleans on `Drop`. Carries an explicit WARNING comment that
  `std::process::exit()` skips `Drop` and leaks the temp dir — this phase resolves that.
- `src/i18n.rs` — `two_line()`/`bi()` bilingual helpers for the three new VI+EN lines.
- `tempfile = "3"` already a dependency. `std::env::temp_dir()` gives the sweep root
  (same base `tempfile` uses).

### Established Patterns
- Sync, single-shot CLI: `main.rs` parses args → `parse_repo_ref` → `InvestigationSession`
  → `app::run` → TUI → `std::process::exit(code)`. No async runtime (tokio dropped).
- All exit codes flow through `IntakeError::exit_code()`; interrupt (130) and panic are
  new exit paths outside that enum — handle at the signal-handler / panic-hook layer.

### Integration Points
- **Live window is `collect()` only:** `clone_repo()` returns a `CloneWorkspace` local in
  `src/app/collect.rs`; the `InvestigationSnapshot` is built from `ws.repo` facts but does
  NOT own `ws`, so the temp dir `Drop`s when `collect()` returns. The leak window is
  therefore precisely a signal/abort **during `collect()`** — Drop already covers the
  normal-completion case. The sweep (D-01) + signal handler (D-03) + panic hook (D-04)
  cover the kill/crash cases Drop cannot.
- **Panic profile:** `Cargo.toml` has no `panic = "abort"` (default unwind) — but a panic
  hook (D-04) cleans regardless of unwind vs abort, so it is robust if `[profile.dist]`
  ever sets abort. Planner should confirm `[profile.dist]` does not silently enable abort.
- New code wires into `main()` (sweep at startup + handler/hook registration) and into the
  `CloneWorkspace` lifecycle (register/clear the live temp path).

</code_context>

<specifics>
## Specific Ideas

- The three new user-facing lines must stay in Ferris's third-person voice (never
  "tôi/mình"), bilingual VI+EN via `i18n::two_line`, consistent with Phase 5/6 messaging.
- Exit code 130 for interrupt is a deliberate shell-convention choice (128 + SIGINT=2).
- Real-world motivation: Phase 6 UAT (`--deep` killed at 4s) left a 5 MB orphaned
  `rust-to-you-clone-*` dir — the exact failure this phase eliminates.

</specifics>

<deferred>
## Deferred Ideas

- **GUARD-04** — time/commit budget bounding of `--deep` runs. Explicitly deferred in
  REQUIREMENTS.md; refuse-by-default already bounds the common case.
- **`--clean` manual subcommand** — a user-invoked "sweep now" command. Not requested;
  the automatic startup sweep (D-01) already self-heals. Note for a future phase if users
  ever want manual control.

</deferred>

---

*Phase: 07-interruptible-lifecycle-temp-hygiene*
*Context gathered: 2026-06-05*
