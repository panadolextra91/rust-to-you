# Phase 1: Intake & Guardrails - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-02
**Phase:** 1-Intake & Guardrails
**Areas discussed:** URL forms, Private vs not-found, Failure UX, Success seam

---

## URL forms accepted

| Option | Description | Selected |
|--------|-------------|----------|
| Generous | https full (+.git/trailing), scheme-less, deep links (extract owner/repo), shorthand owner/repo; SSH deferred to v2 | ✓ |
| Standard | https full + scheme-less + .git/trailing only; no deep link, no shorthand | |
| Strict canonical | only `https://github.com/owner/repo` | |

**User's choice:** Generous
**Notes:** Matches the "paste any GitHub link and it just runs" UX goal. SSH `git@` form explicitly deferred to v2.

---

## Private vs not-found

| Option | Description | Selected |
|--------|-------------|----------|
| Combined + enum now | Define `RepoNotFoundOrPrivate` variant in Phase 1 (no network); honest "private OR doesn't exist" message; Phase 2 fills it from 404 | ✓ |
| Try to distinguish | Separate private vs 404 in Phase 2 (noted as near-impossible unauthenticated) | |
| Generic error | Just "can't access repo" with no reason | |

**User's choice:** Combined + enum now
**Notes:** Confirms Phase 1 stays syntactic-only / offline; the network variant is defined now but wired in Phase 2.

---

## Failure UX — exit codes

| Option | Description | Selected |
|--------|-------------|----------|
| Tiered 0/2/3/4 | 0 ok · 2 input · 3 not-found/private · 4 network/rate-limit | ✓ |
| Simple 0/1 | success / any failure | |
| clap default | 0 ok, 2 usage, 1 runtime | |

**User's choice:** Tiered 0/2/3/4

## Failure UX — error tone

| Option | Description | Selected |
|--------|-------------|----------|
| Crab-voiced + actionable | 🦀 friendly, includes a fix example, to stderr | ✓ |
| Plain & terse | minimal, no emoji, to stderr | |
| Crab-voiced, no emoji | playful wording, no emoji | |

**User's choice:** Crab-voiced + actionable

---

## Success seam

| Option | Description | Selected |
|--------|-------------|----------|
| Stub line + session | Build `InvestigationSession` + print "🦀 Investigation opened: …" + exit 0; becomes Phase 2 seam | ✓ |
| Session silently | Build session, no output | |
| Just exit 0 | Validate only, no session | |

**User's choice:** Stub line + session

## Success seam — case ID

| Option | Description | Selected |
|--------|-------------|----------|
| Hash deterministic | `{REPO_UPPER~4}-{4 hex of hash("owner/repo")}` — testable, stable | ✓ |
| Random each run | different each run; not testable | |
| Drop case ID in V1 | skip the feature | |

**User's choice:** Hash deterministic

---

## Claude's Discretion

- Ergonomic crate choices (recommended `thiserror`, `url`) — minimal & sync; hand-rolling allowed.
- Exact non-crypto hash function for `case_id`.
- Internal file split within `src/cli/` and `src/app/` (e.g. `error.rs` placement).

## Deferred Ideas

- SSH `git@github.com:owner/repo.git` URL form → v2.
- Distinguishing private vs not-found → needs auth; revisit in v2 (EXPD-03).
- Actual network/existence validation → Phase 2 (Collection Layer).
