# rust-to-you 🦀

> Ferris rushes to a repository so you don't have to.

A playful Rust **CLI/TUI** that investigates a public GitHub repository from one command and
renders a cute, scrollable, **bilingual (Tiếng Việt + English)** "case file" report — narrated by
**Ferris**, the Rust crab mascot.

Not repository analytics. Not a git dashboard. This is **repository archaeology & gossip.** 🤣

```
rust-to-you tokio-rs/axum
```

## What it tells you

A single scrollable report with 9 sections:

1. 🌱 **First Impressions** — age, default branch, stars, forks, contributors, last activity
2. ☠️ **Commit Crimes** — total commits, commits this month, top contributor, bus factor
3. 🔥 **Branch Jungle** — total / active / stale / oldest branches
4. 🏺 **Ancient Relics** — oldest file, most-modified file, oldest contributor, longest-living branch
5. 🌿 **Language Soup** — language breakdown with ASCII bars
6. ⚙️ **Infrastructure Footprints** — Docker, Terraform, GitHub Actions, GitLab CI, CircleCI, Jenkins, Dependabot, Renovate
7. 🔮 **Repository Vibes** — a personality (Solo Wizard 🧙, Ancient Temple 🏛️, …) backed by evidence
8. 🔎 **Interesting Findings** — gossip-worthy observations
9. 🦀 **Crab Verdict** — strengths, risks, and Ferris's overall rating

## Install

**Prebuilt binary (no Rust needed):**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/panadolextra91/rust-to-you/releases/latest/download/rust-to-you-installer.sh | sh
```

**With Cargo (compiles locally):**

```sh
cargo install --git https://github.com/panadolextra91/rust-to-you
```

## Usage

```sh
rust-to-you owner/repo                  # shorthand
rust-to-you https://github.com/owner/repo
rust-to-you owner/repo | less           # plain-text (piped / non-TTY)
```

In the TUI: scroll with the **trackpad / mouse wheel**, or `↓ ↑ j k PgUp PgDn g G`; quit with `q`.

Set `GITHUB_TOKEN` to raise the API rate limit (optional — only stars/forks use the API; everything
else comes from a local clone).

## How it works

`URL → minimal GitHub API metadata + full clone → git2/tokei analysis → InvestigationSnapshot →
pure section analyzers → ratatui TUI (or plain text when piped)`. Read-only, public repos only.

## License

Licensed under either of **MIT** ([LICENSE-MIT](LICENSE-MIT)) or **Apache-2.0**
([LICENSE-APACHE](LICENSE-APACHE)) at your option.
