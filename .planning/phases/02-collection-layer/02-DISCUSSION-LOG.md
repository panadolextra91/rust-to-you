# Phase 2: Collection Layer - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-02
**Phase:** 2-Collection Layer
**Areas discussed:** Clone workspace lifecycle, API ordering + failure, Bounded history window, Walking skeleton

---

## Clone workspace lifecycle

| Option | Description | Selected |
|--------|-------------|----------|
| Temp + auto-clean | Unique temp dir, RAII Drop deletes on exit/error; no cache in V1 | ✓ |
| Cache dir, keep | ~/.cache reuse across runs (pulls v2 --cache forward) | |
| Temp, no cleanup | Temp but left behind | |

**User's choice:** Temp + auto-clean (tempfile crate)

---

## API ordering + failure handling

| Option | Description | Selected |
|--------|-------------|----------|
| API first; 404→abort, transient→degrade | GET /repos first → 404 = RepoNotFoundOrPrivate (abort); network/rate-limit → continue git-only, stars/forks "unknown" | ✓ |
| API first; any failure → abort | Strict; transient hiccup kills the run | |
| Clone first; API best-effort | Clone drives; private repo → ugly git error | |

**User's choice:** API first; 404→abort, network/rate-limit→degrade

---

## Bounded history window N

| Option | Description | Selected |
|--------|-------------|----------|
| Last 1000 commits | Cap expensive diff/time passes at 1000; cheap counts use full history; label when capped | ✓ |
| Last 2 years | Time-based window | |
| All, no cap | Accurate but slow on 12k+ commit repos | |

**User's choice:** Last 1000 commits

---

## Walking skeleton (02-01)

| Option | Description | Selected |
|--------|-------------|----------|
| clone→repo age→println via run() | Full clone → repo age → one plain line through Phase 1 run() seam; throwaway | ✓ |
| clone→"cloned OK" | Thinner; no metric | |
| Drop skeleton | Straight to real collectors | |

**User's choice:** clone→repo age→println via run() seam

---

## Claude's Discretion

- Infra detection = path/glob presence (signal list locked by REQUIREMENTS; no content parsing V1).
- Language Soup via the `tokei` crate.
- `InvestigationSnapshot` internal sub-struct shape — planner's call.
- `GITHUB_TOKEN` used if present, else unauthenticated.
- Default branch from clone HEAD (git), not API.

## Deferred Ideas

- Caching/offline (`--cache`/`--offline`) → v2.
- `--deep` unbounded history (lift 1000 cap) → v2.
- Distinguishing private vs not-found → needs auth (v2).
- Content-parsing infra configs → not needed V1.
