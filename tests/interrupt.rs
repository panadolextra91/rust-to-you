//! Integration test for CLEAN-01: a signal mid-clone must clean the live temp
//! dir and exit 130, leaving no orphan.
//!
//! These tests are `#[ignore]` because they clone over the network. Run with:
//!   cargo test --test interrupt -- --ignored
//!
//! We deliberately point at a LARGE repo (`torvalds/linux`) with `--deep` so the
//! clone is still in progress when the signal lands — that is the actual leak
//! window. A small/fast repo would finish cloning before the signal arrives, so
//! the cleanup would be done by `CloneWorkspace::Drop` on normal return rather
//! than by the signal handler, and the test would pass for the wrong reason (or
//! skip silently). We send the signal the instant the temp dir appears, so only
//! a few MB are ever downloaded before we kill it.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

/// A repo large enough that the clone cannot finish before we interrupt it.
/// `--deep` is required so the pre-flight size guard lets the large clone start.
const BIG_REPO: &str = "https://github.com/torvalds/linux";

fn get_clone_dirs() -> HashSet<PathBuf> {
    let mut set = HashSet::new();
    if let Ok(entries) = fs::read_dir(std::env::temp_dir()) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("rust-to-you-clone-") && entry.path().is_dir() {
                    set.insert(entry.path());
                }
            }
        }
    }
    set
}

fn test_interrupt(signal: libc::c_int) {
    // Snapshot pre-existing clone dirs so we never touch another run's temp dir.
    let pre_existing = get_clone_dirs();

    let mut child = Command::new(env!("CARGO_BIN_EXE_rust-to-you"))
        .arg(BIG_REPO)
        .arg("--deep")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn rust-to-you");

    let pid = child.id() as libc::pid_t;

    // Poll until THIS child's clone temp dir appears (the leak window is open).
    // Bounded to ~30s to tolerate slow networks.
    let mut child_temp = None;
    for _ in 0..600 {
        std::thread::sleep(Duration::from_millis(50));
        if let Some(dir) = get_clone_dirs().difference(&pre_existing).next() {
            child_temp = Some(dir.clone());
            break;
        }
        // If the child exited before any temp dir appeared, the clone never
        // started (network/auth failure) — we cannot exercise the leak window.
        if let Ok(Some(_)) = child.try_wait() {
            break;
        }
    }

    let Some(temp_path) = child_temp else {
        // Could not open the leak window (almost always a network failure on a
        // CI box). Skip LOUDLY rather than passing silently — a green here must
        // never be mistaken for "CLEAN-01 verified".
        let _ = child.kill();
        let _ = child.wait();
        eprintln!(
            "SKIP: clone temp dir never appeared for {BIG_REPO} — network unavailable? \
             CLEAN-01 was NOT exercised."
        );
        return;
    };

    // Tiny cushion so register_live_temp() (called right after tempdir()) has
    // definitely run and git2 is mid-transfer. A 6 GB clone is nowhere near done.
    std::thread::sleep(Duration::from_millis(150));

    // SAFETY: kill(2) with a valid child pid and a constant signal number.
    unsafe {
        libc::kill(pid, signal);
    }

    let status = child.wait().expect("failed to wait on child");

    // Handler calls std::process::exit(130) for both SIGINT and SIGTERM (D-03),
    // so the child exits normally with code 130 rather than being signal-killed.
    assert_eq!(
        status.code(),
        Some(130),
        "expected exit 130 from the signal handler, got {status:?}"
    );

    // The live temp dir must have been cleaned by the handler.
    assert!(
        !temp_path.exists(),
        "orphaned temp dir survived the interrupt: {}",
        temp_path.display()
    );
}

#[test]
#[ignore = "clones torvalds/linux over the network; run with --ignored"]
fn sigint_cleans() {
    test_interrupt(libc::SIGINT);
}

#[test]
#[ignore = "clones torvalds/linux over the network; run with --ignored"]
fn sigterm_cleans() {
    test_interrupt(libc::SIGTERM);
}
