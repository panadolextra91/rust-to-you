# Phase 6: Safe Intake & Pre-flight Guard - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-04
**Phase:** 06-safe-intake-pre-flight-guard
**Areas discussed:** Size threshold, --deep behavior, Unknown-size behavior, Threat model & error UX

---

## Size threshold

| Option | Description | Selected |
|--------|-------------|----------|
| 500 MB | Balanced: most normal repos <100MB pass; huge monorepos blocked | ✓ |
| 1 GB | Looser — only blocks truly huge repos | |
| 250 MB | Safer, blocks earlier — may false-positive medium-large repos | |

**User's choice:** 500 MB
**Notes:** GitHub API `size` is in KB → compare `size_kb > 500*1024`.

## Threshold configurability

| Option | Description | Selected |
|--------|-------------|----------|
| Hardcoded const | Single constant in code, simple + testable; `--deep` is the escape hatch | ✓ |
| Const + env override | Default hardcoded but overridable via env var | |

**User's choice:** Hardcoded constant
**Notes:** Aligns with "tighten + document, not a new subsystem".

## --deep behavior

| Option | Description | Selected |
|--------|-------------|----------|
| Warn then proceed | Print one bilingual line ("repo lớn, Ferris vẫn đào vì --deep, sẽ lâu") then clone | ✓ |
| Silent bypass | Skip check entirely, print nothing | |

**User's choice:** Warn then proceed
**Notes:** --deep scope this phase = size-guard bypass only; no other behavior unlocked.

## Unknown-size behavior

| Option | Description | Selected |
|--------|-------------|----------|
| Warn then clone | Fail-open with notice; matches existing degrade-on-transient path | ✓ |
| Fail-closed (refuse) | Block when size unknown, require --deep | |
| Fail-open silent | Clone with no message | |

**User's choice:** Warn then clone (fail-open with notice)
**Notes:** Preserves core value when API is flaky (rate-limit / no token).

## Threat model doc location

| Option | Description | Selected |
|--------|-------------|----------|
| docs/THREAT-MODEL.md | Standalone, discoverable, version-controlled | ✓ |
| SECURITY.md (root) | GitHub convention, but usually disclosure policy not threat model | |
| README section | Co-located, but bilingual README already long | |

**User's choice:** docs/THREAT-MODEL.md

## Error variants

| Option | Description | Selected |
|--------|-------------|----------|
| 2 new variants | RepoTooLarge {size,threshold} (exit 6) + distinct unsafe-input variant | ✓ |
| 1 new variant | Only RepoTooLarge; reuse MalformedRepoPath for dash/over-length | |
| No new variant | Print RepoTooLarge inline; everything else MalformedRepoPath | |

**User's choice:** 2 new variants
**Notes:** Clearest Ferris messaging; easier to test each case independently.

## Segment length caps

| Option | Description | Selected |
|--------|-------------|----------|
| GitHub limits | owner ≤ 39, repo ≤ 100; reject leading `-` | ✓ |
| Simple shared cap | Single limit for both | |

**User's choice:** GitHub limits

---

## Claude's Discretion

- Exact plumbing of `--deep` from `Args` to `collect` (session field vs function param).
- Exact name of the unsafe-input error variant; whether over-length reuses it or `MalformedRepoPath`.
- Routing new notice/warning lines through `i18n::two_line` (preferred for consistency).

## Deferred Ideas

- GUARD-04 — time/commit budget bounding of `--deep` runs (deferred in REQUIREMENTS.md).
- CLEAN-01/02/03 — SIGINT/SIGTERM cleanup + orphaned-temp sweep (Phase 7).
