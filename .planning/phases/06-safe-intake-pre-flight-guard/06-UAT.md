---
status: complete
phase: 06-safe-intake-pre-flight-guard
source: [06-01-SUMMARY.md, 06-02-SUMMARY.md, 06-03-SUMMARY.md]
started: 2026-06-05T06:24:19Z
updated: 2026-06-05T06:24:19Z
---

## Current Test

[testing complete]

## Tests

### 1. Leading-dash input rejected at parser (SEC-01, D-10/D-14)
expected: `rust-to-you https://github.com/-foo/bar` (and `foo/-bar`) is refused with the distinct bilingual "input không an toàn / looks unsafe" message and exit code 2, before any network or git call.
result: pass
note: Verified live. Both `-foo/bar` and `foo/-bar` → "🦀 Input này kỳ lạ / không an toàn… / This input looks unsafe" → exit 2. Leading-dash routed to UnsafeInput (distinct from MalformedRepoPath), confirming the guard was hoisted to parse_repo_ref.

### 2. Over-length owner/repo rejected at parser (SEC-01, D-11)
expected: owner > 39 chars or repo > 100 chars is refused with the unsafe-input message and exit 2, before any network/git call.
result: pass
note: Verified live. 40-char owner and 101-char repo both → UnsafeInput → exit 2. (39/100 at-limit pass the parser by design.)

### 3. Existing parser guards preserved (D-12)
expected: `..` path traversal, non-GitHub host, and empty input are still rejected with their original messages/exit 2 (not regressed by the new guards).
result: pass
note: Verified live. `foo/../bar` → MalformedRepoPath; `gitlab.com/foo/bar` → UnsupportedHost; empty → EmptyInput. All exit 2, original distinct messages intact.

### 4. Oversized repo refused before clone (GUARD-01, GUARD-02)
expected: pointing the tool at a repo larger than 500 MB (without `--deep`) stops before the clone with a bilingual Ferris message that shows the actual size and names `--deep`; exit code 6; machine never starts the oversized clone.
result: pass
note: Verified live against torvalds/linux (GitHub API size 6158 MB). Output: "🦀 Repo này to quá (6158 MB, vượt ngưỡng 500 MB)… --deep… / This repo is too large (6158 MB, over the 500 MB limit)…". Exit 6. Returned in ~1s with no clone started (refused after metadata fetch, before clone_repo).

### 5. `--deep` opts into analysing a large repo (GUARD-03, D-07/D-08)
expected: passing `--deep` on a large repo prints a one-shot bilingual "đào anyway / digs anyway" warning and then proceeds to clone (does not refuse, does not exit 6).
result: pass
note: Verified live (torvalds/linux --deep, hard-killed at 4s). Output: "🦀 Repo lớn (6158 MB), Ferris vẫn đào vì --deep — sẽ lâu đó / Large repo (6158 MB) — Ferris digs anyway (--deep)…" then proceeded past the guard into clone (no exit 6). Confirms --deep bypasses ONLY the size guard. (The interrupted clone left a 5 MB orphaned temp dir — expected; this is exactly what Phase 7 / CLEAN-* will fix.)

### 6. Unknown-size fail-open (GUARD-01 edge, D-09)
expected: when metadata is Unavailable (rate-limit / no token / transient), the tool warns ("không biết kích thước repo, Ferris cứ đào nha") and proceeds rather than blocking.
result: skipped
reason: Covered by green unit test `app::collect::tests::test_size_decision_unavailable` (WarnUnknown branch). Live reproduction requires forcing a metadata failure (network outage / exhausted rate limit), which is not safely/deterministically reproducible in this session.

### 7. Intake threat model documented + test-backed (SEC-02, D-15/D-16)
expected: `docs/THREAT-MODEL.md` documents the intake attack surface (leading-dash arg injection, `..`/`\`/control chars, oversize-clone DoS) and states git2 is libgit2 FFI with no shell spawn; mitigations reference the backing tests.
result: pass
note: Verified via content checks — file exists; contains STRIDE register, libgit2, no-shell-spawn rationale, 500 MB guard, leading-dash, path traversal, `parse_repo_ref`, and references to `test_parse_repo_ref_reject` / `size_decision` tests.

## Summary

total: 7
passed: 6
issues: 0
pending: 0
skipped: 1
blocked: 0

## Gaps

[none — all phase success criteria verified; no code issues found]

## Verification Method Notes

- Build: `cargo build` (working tree, uncommitted Phase 6 changes incl. 4 review cleanups) → binary OK.
- Unit suite: `cargo test` → 65/65 pass. `cargo clippy --all-targets` → clean.
- CLI behaviors 1–5 exercised end-to-end against the built binary; behavior 4 against a real >500 MB public repo (refused before clone); behavior 7 via doc content assertions.
- Orphaned temp dir from the interrupted `--deep` test was manually removed; permanent cleanup is Phase 7 scope.
