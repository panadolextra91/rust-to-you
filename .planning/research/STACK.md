# Stack Research

**Domain:** Rust CLI/TUI for GitHub repository investigation
**Researched:** 2026-06-02
**Confidence:** HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| `clap` | 4.6.1 | CLI parsing and help output | Mature, ergonomic command parsing for a single-command UX with clean validation and built-in help text. |
| `git2` | 0.21.0 | Local git inspection and shallow clone workflow | Keeps repository archaeology inside Rust without shelling out to `git` for every operation. |
| `reqwest` | 0.13.4 | GitHub HTTP client | Handles GitHub REST calls for repo metadata and branch enumeration while staying compatible with async Rust. |
| `ratatui` | 0.30.0 | Terminal report rendering | The current maintained Rust TUI standard and a better fit than older `tui-rs` patterns. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `crossterm` | 0.29.0 | Terminal events and screen control | Use for keyboard handling, alternate screen, and vertical scrolling in the report. |
| `tokio` | 1.52.3 | Async runtime | Use when fetching GitHub metadata and branch pages concurrently. |
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
cargo add clap git2 reqwest ratatui

# Supporting
cargo add crossterm tokio --features full
cargo add serde --features derive
cargo add serde_json chrono
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| `reqwest` | `octocrab` | Use `octocrab` only if typed GitHub API ergonomics matter more than keeping HTTP behavior explicit. |
| `git2` | `std::process::Command("git")` | Use shelling out only if libgit2 edge cases become a platform problem and the binary dependency is acceptable. |
| `ratatui` | Plain ANSI output | Use plain ANSI only if the project deliberately drops scrolling and structured sections later. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Scraping GitHub HTML | Fragile against UI changes and slower than structured endpoints | GitHub REST plus local git inspection |
| Deep cloning every repo in V1 | Wastes time and bandwidth for a read-mostly report | Shallow clone plus targeted metadata fetches |
| `tui-rs` era examples copied blindly | Many are stale and predate current Ratatui APIs | Ratatui 0.30 patterns and current docs |

## Stack Patterns by Variant

**If metadata fits inside public API limits:**
- Use GitHub REST for stars, forks, default branch, branches, contributors, and remote metadata.
- Because it avoids over-fetching full git history just to show overview numbers.

**If a metric depends on local history or file evolution:**
- Use `git2` against the cloned repo snapshot.
- Because archaeology-style sections need real commit and path history, not just API summaries.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| `ratatui@0.30.0` | `crossterm@0.29.0` | Current common pairing for event handling plus rendering. |
| `reqwest@0.13.4` | `tokio@1.52.3` | Async HTTP stack should be built together from the start. |
| `serde@1.0.228` | `serde_json@1.0.150` | Standard serialization pair for API models and future exports. |

## Sources

- https://docs.rs/crate/clap/latest — current crate version and API docs
- https://docs.rs/crate/git2/latest — current crate version and libgit2 notes
- https://docs.rs/crate/reqwest/latest — current crate version and async HTTP guidance
- https://docs.rs/crate/ratatui/latest — current crate version and maintained TUI ecosystem
- https://docs.rs/crate/crossterm/latest — terminal event handling support
- https://docs.rs/crate/serde/latest — serialization support
- https://docs.rs/crate/serde_json/latest/source/ — current JSON crate release
- https://docs.rs/crate/tokio — current async runtime release
- https://docs.rs/crate/chrono/latest — current date/time crate release

---
*Stack research for: Rust CLI/TUI for GitHub repository investigation*
*Researched: 2026-06-02*
