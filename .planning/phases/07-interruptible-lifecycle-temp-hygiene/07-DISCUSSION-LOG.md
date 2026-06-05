# Phase 7: Interruptible Lifecycle & Temp Hygiene - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-05
**Phase:** 07-interruptible-lifecycle-temp-hygiene
**Areas discussed:** Startup sweep safety, Sweep notification, Interrupt UX, Panic behavior

---

## Startup sweep safety (CLEAN-02)

| Option | Description | Selected |
|--------|-------------|----------|
| Age-based | Remove `rust-to-you-clone-*` not modified in >60 min; no PID tagging; live instance has fresh mtime | ✓ |
| PID-tagged | Embed PID in dir name, remove only when owner PID is dead; most precise but cross-platform PID liveness awkward on macOS | |
| Wipe all by prefix | Sweep every `rust-to-you-clone-*`; simplest but can kill a concurrent instance's live temp | |

**User's choice:** Age-based.
**Notes:** Threshold locked to 60 min, hardcoded constant (D-07), mirroring Phase 6's `MAX_REPO_KB` minimal-surface stance. Sweep runs early in `main()` before clone (D-05), best-effort / non-blocking (D-06).

---

## Sweep notification (CLEAN-02)

| Option | Description | Selected |
|--------|-------------|----------|
| Announce only if swept | One bilingual line only when ≥1 dir removed; silent otherwise | ✓ |
| Always silent | Sweep quietly, say nothing | |
| Verbose | List each removed dir | |

**User's choice:** Announce only if swept.
**Notes:** Reinforces self-healing feel without noise; uses existing `i18n::two_line`/`bi`.

---

## Interrupt UX (CLEAN-01)

| Option | Description | Selected |
|--------|-------------|----------|
| Bilingual line + exit 130 | Ferris "cleaning up, bye" line + clean temp + exit 130 (128+SIGINT) | ✓ |
| Silent + exit 130 | Clean temp silently, exit 130, print nothing | |
| Message + exit 0 | Print message but exit 0 (not shell-convention for interrupt) | |

**User's choice:** Bilingual line + exit 130.
**Notes:** SIGTERM follows the same path; exit 130 still returned even if no workspace alive yet.

---

## Panic behavior (CLEAN-03)

| Option | Description | Selected |
|--------|-------------|----------|
| Panic hook: clean + friendly message | Hook cleans temp + prints Ferris apology, hides backtrace unless RUST_BACKTRACE set | ✓ |
| RAII guard, default panic | Guard cleans on unwind but Rust prints default scary backtrace | |
| Both: guard clean + hook message | Guard for cleanup robustness, hook for friendly message | |

**User's choice:** Panic hook (clean + friendly message).
**Notes:** Default panic profile is unwind (no `panic="abort"` in Cargo.toml); hook is robust either way. Whether to add an RAII guard on top is left to planner discretion (D-08 defense-in-depth).

---

## Claude's Discretion

- Signal crate choice (`ctrlc` vs `signal-hook` vs raw `libc`) — must be sync (no async runtime).
- Mechanism for the single source-of-truth live-temp path (global `OnceLock<Mutex<…>>` vs registry vs RAII).
- Whether to add an explicit RAII guard in addition to the global cleanup path.
- Exact bilingual wording of the three new lines (sweep / interrupt / panic).

## Deferred Ideas

- **GUARD-04** — time/commit budget bounding of `--deep` runs (deferred in REQUIREMENTS.md).
- **`--clean` manual subcommand** — user-invoked sweep; not requested, startup sweep self-heals. Note for a future phase.
