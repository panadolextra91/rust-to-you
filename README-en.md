<div align="center">

# 🦀 rust-to-you

[🇻🇳 Tiếng Việt](README.md) · **🇬🇧 English**

*Ferris rushes to a repository so you don't have to.*

<img src="https://res.cloudinary.com/duy8dombh/image/upload/v1780487432/ferris_r0xhkh.png" alt="Ferris 🦀" width="360" />

</div>

---

Hey there 👋 I'm **Ferris** — the Rust crab mascot 🦀

Ever had to open a dozen GitHub tabs and click around endlessly just to understand an unfamiliar repo? Exhausting, right? Let Ferris handle it! Give me **one command**, I'll scuttle over to that repo, dig through its git history, snoop around its structure, CI/CD and infra clues, then tell you all about it as a **cute investigation report** — bilingual Vietnamese–English.

This is **not** a dry analytics dashboard. This is **repository archaeology & gossip** 🤣

```sh
rust-to-you tokio-rs/axum
```

## 🔍 What does Ferris dish out?

One scrollable report, 9 sections, reads like a case file:

1. 🌱 **First Impressions** — age, default branch, stars, forks, contributors, last activity
2. ☠️ **Commit Crimes** — total commits, commits this month, top contributor, bus factor
3. 🔥 **Branch Jungle** — total / active / stale / oldest branches
4. 🏺 **Ancient Relics** — oldest file, most-modified file, oldest contributor, longest-living branch
5. 🌿 **Language Soup** — language breakdown with ASCII bars
6. ⚙️ **Infrastructure Footprints** — Docker, Terraform, GitHub Actions, GitLab CI, CircleCI, Jenkins, Dependabot, Renovate
7. 🔮 **Repository Vibes** — Ferris assigns a "personality" (Solo Wizard 🧙, Ancient Temple 🏛️, …) backed by evidence
8. 🔎 **Interesting Findings** — the gossip-worthy bits
9. 🦀 **Crab Verdict** — strengths, risks, and Ferris's final ruling

## 🚀 Quick install

**🍺 Homebrew (easiest on macOS):**
```sh
brew install panadolextra91/tap/rust-to-you
```

**🐚 Shell — no Rust required (macOS/Linux):**
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/panadolextra91/rust-to-you/releases/latest/download/rust-to-you-installer.sh | sh
```

**📦 If you have Rust:**
```sh
cargo install --git https://github.com/panadolextra91/rust-to-you
```

## 🎮 Usage

```sh
rust-to-you owner/repo                      # shorthand
rust-to-you https://github.com/owner/repo   # or a full URL
rust-to-you owner/repo | less              # plain text (when piped / non-TTY)
```

In the TUI: scroll with the **trackpad / mouse wheel**, or `↓ ↑ j k PgUp PgDn g G`; quit with `q`.

> 💡 Set `GITHUB_TOKEN` to raise the API rate limit (optional — only stars/forks use the API; everything else comes from a local clone).

## 🧠 How does Ferris work?

Curious about how Ferris runs the investigation (architecture, data flow, diagrams)? Read 👉 **[ARCHITECTURE.md](ARCHITECTURE.md)** — the detailed technical docs.

## 🦀 A few promises from Ferris
- **Read-only** — Ferris never modifies your repo.
- **Public GitHub repos only** (V1).
- **Private** — runs on your machine, clones into a temp dir, then cleans up after itself.

## 📄 License

At your option: **MIT** ([LICENSE-MIT](LICENSE-MIT)) or **Apache-2.0** ([LICENSE-APACHE](LICENSE-APACHE)).

<div align="center">

*Made with 🦀 and a lot of gossip — by Ferris.*

</div>
