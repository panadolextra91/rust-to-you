# rust-to-you Intake Threat Model

This document outlines the security posture, trust boundaries, and threat mitigation strategies for the intake surface of `rust-to-you`.

## Security Architecture & Design

By design, `rust-to-you` is a read-only visualizer for public GitHub repositories.
A key architectural security property of `rust-to-you` is that **it does not spawn any shell or invoke any `git` subprocess**.
Instead, it interacts with Git repositories through **`git2` (libgit2 FFI bindings)**. This structural choice narrows down the flag injection attack surface by construction, as there is no shell command shell-escape parsing involved during git operations.

## Trust Boundaries

- **User CLI inputs -> `parse_repo_ref`**: The boundary where untrusted input strings enter the application. Inputs must be fully validated before any network or Git operations occur.
- **GitHub API JSON -> `RepoMetadata`**: Remote-sourced JSON metadata is parsed, including the `size` field used to enforce clone safety.

## STRIDE Threat Register

| Threat ID | Threat Category | Threat Description | Mitigation Strategy | Backing Guard & Test |
|-----------|-----------------|--------------------|---------------------|----------------------|
| T-06-01 | Tampering / Elevation of Privilege | Argument injection via leading `-` (e.g., `-upload-pack` or flags mimicking options). | Reject any owner or repo segment starting with `-` at `parse_repo_ref` before Git/network operations, throwing an `IntakeError::UnsafeInput` error. | Guard: `starts_with('-')` check in `parse_repo_ref`. Test: `test_parse_repo_ref_reject` ("-foo/bar", "foo/-bar"). |
| T-06-02 | Tampering | Path traversal (e.g., `..` or `\`) | Reject segments containing `..` or `\` in the parser. | Guard: segment checks in `parse_repo_ref`. Test: `test_parse_repo_ref_reject` ("tokio-rs/axum/../path"). |
| T-06-03 | Tampering / Denial of Service | Over-length input or control characters. | Cap overall input length to 2048 and segment lengths (owner ≤ 39, repo ≤ 100), rejecting any control characters. | Guard: length checks and `is_control` in `parse_repo_ref`. Test: `test_parse_repo_ref_reject` (over-length owner/repo). |
| T-06-04 | Spoofing / Information Disclosure | Unsupported host or SSRF-like redirect. | Strict host allowlist limited only to `github.com` and `www.github.com`. | Guard: host allowlist check in `parse_repo_ref`. Test: `test_parse_repo_ref_reject` ("gitlab.com/foo/bar"). |
| T-06-05 | Denial of Service | Oversize-clone resource exhaustion (disk/CPU exhaustion). | Pre-flight size guard checked using GitHub API metadata before clone begins. Repos > **500 MB** are refused by default unless `--deep` is explicitly specified. | Guard: `size_decision` check in `collect.rs`. Test: `test_size_decision_boundary` in `collect.rs`. |

## Backing Test Reference

Mitigations are verified automatically by the test suite. Specifically:
- Parser injection and length rejections are tested in [src/cli/parse.rs](file:///Users/huynhngocanhthu/rust-to-you/src/cli/parse.rs) via `test_parse_repo_ref_reject`.
- Size decisions (and the `--deep` bypass) are tested in [src/app/collect.rs](file:///Users/huynhngocanhthu/rust-to-you/src/app/collect.rs) via `size_decision` unit tests.
