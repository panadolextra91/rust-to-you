---
phase: 6
slug: safe-intake-pre-flight-guard
status: verified
threats_open: 0
asvs_level: 1
created: 2026-06-05
---

# Phase 6 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

This phase **is** the intake-hardening security phase. The register below was authored
at plan time (all 3 PLAN.md files carried `<threat_model>` blocks) and verified against
the implementation — most mitigations were exercised end-to-end during UAT (see `06-UAT.md`).

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| user CLI arg → `parse_repo_ref` | Untrusted owner/repo string; must be fully validated before any network or git2 call | untrusted string (high) |
| GitHub API JSON → `RepoMetadata` | Untrusted remote JSON deserialized; `size` (KB) feeds the pre-flight guard | remote numeric field (low) |
| `RepoMetadata.size` → size guard | Remote-reported size drives refuse/proceed; a hostile under-report could understate clone cost | remote heuristic (medium) |
| `collect()` → `clone_repo` (git2 FFI) | Guard sits on the path to the only resource-heavy op; refusal must happen before clone starts | local resource (high) |

**Structural property:** `git2` is libgit2 FFI — no shell and no `git` subprocess is ever
spawned (verified: zero `std::process::Command` in `src/`). The classic argument/flag-injection
vector is therefore absent by construction, not merely filtered.

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-06-01 | Tampering / EoP | `parse_repo_ref` — leading `-` (e.g. `--upload-pack=…`) | mitigate | Reject any owner/repo segment starting with `-` → `UnsafeInput`, exit 2 (D-10). Verified live (UAT-1). | closed |
| T-06-02 | Tampering | `parse_repo_ref` — `..` traversal, `\` backslash | mitigate | Existing reject loop kept unchanged (`parse.rs:40-42`, D-12). Verified live (UAT-3). | closed |
| T-06-03 | Tampering | `parse_repo_ref` — control chars, >2048 length | mitigate | Existing control-char + 2048-cap guard kept (`parse.rs:15`, D-12). Verified live (tab char + 2100-char both rejected before any network call). | closed |
| T-06-04 | Spoofing / Info disclosure | host allowlist | mitigate | `github.com`/`www.github.com` allowlist kept (`parse.rs:51,67`); URL built only from validated owner/repo. Verified live (UAT-3, gitlab.com rejected). | closed |
| T-06-05 | DoS / resource abuse | over-length owner/repo segment | mitigate | owner ≤ 39, repo ≤ 100 caps → `UnsafeInput`, exit 2 (D-11), rejected before network/git2. Verified live (UAT-2). | closed |
| T-06-06 | Denial of Service | oversize-clone machine hang | mitigate | Pre-flight `size > 500 MB` refuse-by-default in `collect.rs` BEFORE `clone_repo` (GUARD-01, D-04); guard returns `Err` to main.rs so no clone starts. Verified live (UAT-4: torvalds/linux 6158 MB → exit 6, no clone). | closed |
| T-06-07 | DoS (false-negative) | remote `size` under-reports (LFS-excluded) | accept | `size` excludes Git LFS; the guard is a heuristic safety net, not a hard quota. Low residual risk — documented (RESEARCH Assumptions A2). | closed |
| T-06-08 | Availability regression | metadata Unavailable (rate-limit / no token / flaky net) | accept | Fail-open by design (D-09): warn + proceed when size unknown, never block on missing data. Unit-covered (`size_decision` WarnUnknown branch). | closed |
| T-06-09 | Resource leak / EoP | process exit while `CloneWorkspace` alive | mitigate | `RepoTooLarge` returns `Err` BEFORE `clone_repo`, so no workspace is alive at refusal; no new `std::process::exit` in `collect.rs`. (Full interrupt/temp hygiene is Phase 7 / CLEAN-*.) | closed |
| T-06-10 | Repudiation / audit gap | undocumented intake surface | mitigate | `docs/THREAT-MODEL.md` makes the intake threat model explicit and auditable (SEC-02, D-15), each threat tied to its guard + test. Verified live (UAT-7). | closed |
| T-06-SC | Tampering (supply chain) | npm/pip/cargo installs | accept | Zero new dependencies this phase (verified against `Cargo.lock`) — no install surface to slopcheck. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-06-01 | T-06-07 | GitHub API `size` excludes Git LFS and may under-report; pre-flight guard is a heuristic safety net, not a hard quota. A repo just over threshold via LFS may slip past — bounded by `--deep` opt-in and the user's own runtime. Time/commit budget bounding is deferred (GUARD-04). | panadolextra91 | 2026-06-05 |
| AR-06-02 | T-06-08 | Fail-open on unknown size (metadata Unavailable) is intentional — preserves the tool's core value on a flaky/rate-limited API rather than blocking on missing data. | panadolextra91 | 2026-06-05 |
| AR-06-03 | T-06-SC | No packages installed this phase (zero new deps); nothing to audit. | panadolextra91 | 2026-06-05 |

*Accepted risks do not resurface in future audit runs.*

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-06-05 | 11 | 11 | 0 | /gsd-secure-phase (State B, plan-time register, short-circuit; mitigations cross-verified against 06-UAT.md live results) |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-06-05
