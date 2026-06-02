# Stack Research

**Domain:** Rust CLI/TUI for GitHub repository investigation
**Researched:** 2026-06-02
**Confidence:** HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| `clap` | 4.6.1 | CLI parsing and help output | Mature, ergonomic command parsing for a single-command UX with clean validation and built-in help text. |
| `git2` | 0.21.0 | Local git inspection and full clone workflow | Keeps repository archaeology inside Rust without shelling out to `git` for every operation. Full clone gives complete history for archaeology. |
| `reqwest` | 0.13.4 | GitHub HTTP client (blocking) | Handles the single GitHub REST call for stars/forks/description. Use the `blocking` feature — no async runtime needed for one sequential request. |
| `ratatui` | 0.30.0 | Terminal report rendering | The current maintained Rust TUI standard and a better fit than older `tui-rs` patterns. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `crossterm` | 0.29.0 | Terminal events and screen control | Use for keyboard handling, alternate screen, and vertical scrolling in the report. |
| `serde` | 1.0.228 | Serialization/deserialization | Use for GitHub API payload models and config-safe internal structs. |
| `serde_json` | 1.0.150 | JSON support | Use for decoding GitHub responses and future `--json` export work. |
| `chrono` | 0.4.44 | Date/time math | Use for repo age, "last activity," stale-branch logic, and month-based commit statistics. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `cargo fmt` | Code formatting | Keep report and analyzer code predictable as modules multiply. |
| `cargo clippy` | Linting | Especially useful for analysis code where integer math and iterator pipelines can drift into subtle bugs. |
| `cargo test` | Regression safety | Critical for heuristics, section formatting, and sample-repo fixtures. |

## Installation

```bash
# Core
cargo add clap git2 ratatui
cargo add reqwest --no-default-features --features blocking,json,rustls-tls

# Supporting
cargo add crossterm
cargo add serde --features derive
cargo add serde_json chrono
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| `reqwest` | `octocrab` | Use `octocrab` only if typed GitHub API ergonomics matter more than keeping HTTP behavior explicit. |
| `git2` | `std::process::Command("git")` | Use shelling out only if libgit2 edge cases become a platform problem and the binary dependency is acceptable. |
| `ratatui` | Plain ANSI output | Use plain ANSI only if the project deliberately drops scrolling and structured sections later. (Note: the Phase 2 walking skeleton intentionally uses plain `println!` before TUI exists.) |
| hand-rolled line counter | `tokei` | For Language Soup, prefer the `tokei` crate over a custom extension/line counter — it is the de-facto standard for per-language line counts and avoids reinventing detection. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Scraping GitHub HTML | Fragile against UI changes and slower than structured endpoints | GitHub REST plus local git inspection |
| Unbounded full-history traversal on huge repos | A full clone has complete history, but diffing every commit is slow | Full clone, but bound expensive passes (e.g. cap commits scanned for most-modified-file) |
| `tokio` / async for V1 | Adds runtime + `.await` complexity for a single sequential API call | `reqwest` blocking feature |
| `tui-rs` era examples copied blindly | Many are stale and predate current Ratatui APIs | Ratatui 0.30 patterns and current docs |

## Stack Patterns by Variant

**For facts git cannot know locally (stars, forks, description/topics):**
- Use a single GitHub REST call (`GET /repos/{owner}/{repo}`).
- Because these are GitHub-platform facts, not git facts — the only thing the API is needed for.

**For everything else (commits, branches, contributors, file ages, languages, infra):**
- Use `git2` against the full local clone.
- Because archaeology-style sections need real commit and path history, and the full clone already has it — no extra API calls or rate-limit exposure.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| `ratatui@0.30.0` | `crossterm@0.29.0` | Current common pairing for event handling plus rendering. |
| `reqwest@0.13.4` | blocking feature | Use `reqwest`'s `blocking` client — no `tokio` runtime required for V1. |
| `serde@1.0.228` | `serde_json@1.0.150` | Standard serialization pair for API models and future exports. |

## Sources

- https://docs.rs/crate/clap/latest — current crate version and API docs
- https://docs.rs/crate/git2/latest — current crate version and libgit2 notes
- https://docs.rs/crate/reqwest/latest — current crate version and async HTTP guidance
- https://docs.rs/crate/ratatui/latest — current crate version and maintained TUI ecosystem
- https://docs.rs/crate/crossterm/latest — terminal event handling support
- https://docs.rs/crate/serde/latest — serialization support
- https://docs.rs/crate/serde_json/latest/source/ — current JSON crate release
- https://docs.rs/crate/chrono/latest — current date/time crate release
- https://docs.rs/crate/tokei/latest — language line-count crate for Language Soup

---
*Stack research for: Rust CLI/TUI for GitHub repository investigation*
*Researched: 2026-06-02*
