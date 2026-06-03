<div align="center">

# 🏗️ rust-to-you — Architecture

Technical documentation · README: [🇻🇳 Tiếng Việt](README.md) · [🇬🇧 English](README-en.md)

</div>

---

> The READMEs are Ferris talking to *you*. This document is the engineering side: how the tool is
> built, how data flows, and why the key decisions were made.

## 1. Overview

`rust-to-you` is a **single, synchronous pipeline** (no async runtime). One command in, one
report out:

```
URL  →  intake/validate  →  collect (API + clone)  →  normalize (snapshot)
     →  analyze (pure fns)  →  view models  →  render (TUI or plain text)
```

Design principles:
- **Read-only**, public GitHub repos only.
- **Git-first**: a full local clone (via `git2`) is the source of truth; the GitHub API is used
  for exactly one call (stars / forks / description).
- **Pure analyzers**: every metric is a pure function over a normalized `InvestigationSnapshot`,
  so it is deterministic and unit-testable with synthetic fixtures.
- **Snapshot is the carrier**: the cloned workspace is dropped after collection; everything
  downstream reads the snapshot only.
- **Bilingual by construction**: all user-facing text is VI + EN, narrated by Ferris.

## 2. Component architecture

```mermaid
flowchart TB
    subgraph INTAKE["Intake — src/cli, src/app"]
        A[clap Args] --> B[parse_repo_ref → RepoRef]
        B --> C[InvestigationSession + case_id]
    end
    subgraph COLLECT["Collection"]
        D[github::fetch_metadata<br/>reqwest blocking, 1 call]
        E[repo::clone / history / branches<br/>git2 full clone]
        F[scan::lang / infra<br/>tokei + path detection]
    end
    G[(InvestigationSnapshot<br/>normalized data)]
    subgraph ANALYZE["Analysis — src/analyze (pure)"]
        H[commit · branch · relics<br/>language · infra]
        I[vibes · findings · verdict]
    end
    J[FactualSections + VibeResult<br/>src/report — view models]
    subgraph RENDER["Render — src/tui"]
        K[ratatui TUI<br/>scrollable, interactive]
        L[plain text<br/>non-TTY fallback]
    end

    C --> D & E & F
    D & E & F --> G
    G --> H & I
    H & I --> J
    J --> K & L
```

| Layer | Modules | Responsibility |
|-------|---------|----------------|
| Intake | `cli/`, `app/` | Parse + validate the URL, build the session, choose TTY vs plain |
| Collection | `github/`, `repo/`, `scan/` | One API call + full clone + git/file facts |
| Data model | `snapshot.rs` | Normalize everything into `InvestigationSnapshot` |
| Analysis | `analyze/` | Pure functions → section metrics, vibes, findings, verdict |
| View models | `report/` | `FactualSections` (sections 1–6) + vibe/finding/verdict results |
| Render | `tui/` | ratatui report (or plain text); shared `i18n` + format helpers |
| Errors | `error.rs` | `IntakeError` taxonomy + tiered exit codes |

## 3. Investigation flow

```mermaid
flowchart LR
    U([repo URL]) --> P{parse_repo_ref}
    P -->|invalid / unsupported| ERR[IntakeError → stderr<br/>exit 2]
    P -->|RepoRef| API[GET /repos/owner/repo]
    API -->|404| NF[RepoNotFoundOrPrivate<br/>exit 3]
    API -->|network / 403| DEG[degrade:<br/>stars/forks = unknown]
    API -->|ok| META[metadata]
    DEG --> CLONE
    META --> CLONE[full clone → temp dir]
    CLONE --> SNAP[(InvestigationSnapshot)]
    SNAP --> AN[analyzers: factual + vibes/findings/verdict]
    AN --> VM[view models]
    VM --> T{stdout is a TTY?}
    T -->|yes| TUI[ratatui scrollable report]
    T -->|no| PLAIN[plain text]
```

**Bounded passes:** cheap aggregations (total commits, contributors, bus factor, repo age, branch
enumeration) walk the *full* history; expensive passes (most-modified file, time-of-day buckets)
are bounded to the **last 1000 commits** and labelled accordingly.

## 4. Runtime sequence

```mermaid
sequenceDiagram
    actor User
    participant CLI as rust-to-you
    participant GH as GitHub API
    participant Git as git2 (clone)
    participant An as Analyzers
    participant R as Renderer

    User->>CLI: rust-to-you owner/repo
    CLI->>CLI: parse → RepoRef, build session (case_id)
    CLI->>GH: GET /repos/{owner}/{repo}
    alt 404
        GH-->>CLI: not found → abort (exit 3)
    else network / rate-limit
        GH-->>CLI: error → degrade (stars/forks unknown)
    else ok
        GH-->>CLI: stars / forks / description
    end
    CLI->>Git: clone into temp dir (RAII)
    Git-->>CLI: local repo
    CLI->>An: build InvestigationSnapshot
    An->>An: factual metrics + vibes (VIBES ruleset) + findings + verdict
    An-->>CLI: FactualSections + VibeResult
    CLI->>R: render(session, sections, now)
    R-->>User: scrollable bilingual report (TUI) / plain text
    Note over CLI,Git: temp workspace dropped + cleaned up on exit
```

## 5. Repository Vibes — the classifier

Section 7 is a **weighted-scoring classifier** (`analyze/vibes.rs`):
- Each of the 7 vibes accumulates points from satisfied conditions over snapshot signals.
- Highest score wins; ties broken by specificity order; below `MIN_SCORE = 4` falls back to
  **Chaotic Good**.
- The satisfied conditions become the **evidence bullets** (grounded comedy — every label is
  justified).
- The runner-up vibe flows into Section 8 (Interesting Findings).

## 6. Module map

```text
src/
├── main.rs            # entrypoint: parse → session → run; maps errors to stderr + exit code
├── lib.rs             # module wiring (lib + bin crate)
├── error.rs           # IntakeError taxonomy + exit codes
├── i18n.rs            # Bilingual{vi,en} + two_line / inline_label
├── cli/               # clap Args + URL parsing → RepoRef
├── app/               # session, collect orchestration, run() seam
├── github/            # reqwest blocking client + RepoMetadata + classify(StatusCode)
├── repo/              # clone (tempfile RAII), history, branches (git2)
├── scan/              # tokei language breakdown + infra footprint detection
├── snapshot.rs        # InvestigationSnapshot (normalized data carrier)
├── analyze/           # pure analyzers: commit, branch, relics, language, infra, vibes, findings, verdict
├── report/            # FactualSections view models
└── tui/               # ratatui report, plain renderer, format helpers, scroll/keys (app.rs)
```

## 7. Key decisions

| Decision | Why |
|----------|-----|
| Full clone (not shallow) | Archaeology metrics need complete history; expensive passes are bounded instead |
| API limited to one call | Git supplies everything else → rate limits are a non-issue |
| No `tokio` (sync) | A single sequential call needs no async runtime |
| `reqwest` blocking + `rustls-tls` | Avoids an async stack; portable TLS |
| `git2` with `vendored-openssl` | Self-contained HTTPS clone without relying on system OpenSSL |
| `tokei` for languages | De-facto standard line counter |
| Pure analyzers over a snapshot | Deterministic, fixture-testable; enables a future `--json` |
| Bilingual `i18n` helper shared by TUI + plain | One source of truth for VI+EN text |
| Bus factor = fewest authors making ≥50% of commits | Cheap (commit-count), honest, bot/merge-filtered, identity-normalized |

## 8. Testing

- **Unit / fixture tests** (`cargo test`): URL parser table, git analyzers against in-memory
  fixture repos, GitHub status→error mapping, format helpers, vibe/finding/verdict rules, and a
  ratatui `TestBackend` smoke test for the report.
- **Manual**: the interactive TUI (scroll/keys, resize) and live clones against real repos.

---

<div align="center">

Back to: [🇻🇳 README (Tiếng Việt)](README.md) · [🇬🇧 README (English)](README-en.md)

</div>
