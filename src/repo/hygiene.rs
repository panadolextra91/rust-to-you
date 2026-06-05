use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime};

pub const ORPHAN_MAX_AGE: Duration = Duration::from_secs(60 * 60);
const PREFIX: &str = "rust-to-you-clone-";

static LIVE_TEMP: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

fn slot() -> &'static Mutex<Option<PathBuf>> {
    LIVE_TEMP.get_or_init(|| Mutex::new(None))
}

pub fn register_live_temp(p: PathBuf) {
    *slot().lock().unwrap_or_else(|e| e.into_inner()) = Some(p);
}

pub fn clear_live_temp() {
    *slot().lock().unwrap_or_else(|e| e.into_inner()) = None;
}

pub fn cleanup_live_temp() {
    let path = slot().lock().unwrap_or_else(|e| e.into_inner()).take();
    if let Some(p) = path {
        let _ = std::fs::remove_dir_all(&p);
    }
}

pub fn sweep_orphans(dir: &Path, now: SystemTime, max_age: Duration) -> usize {
    let mut removed = 0;
    let Ok(entries) = std::fs::read_dir(dir) else { return 0 };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name_str) = name.to_str() else { continue };
        if !name_str.starts_with(PREFIX) { continue; }
        
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_dir() { continue; }
        
        let Ok(mtime) = meta.modified() else { continue };
        match now.duration_since(mtime) {
            Ok(age) if age >= max_age => {
                if std::fs::remove_dir_all(entry.path()).is_ok() {
                    removed += 1;
                }
            }
            _ => {}
        }
    }
    removed
}

pub fn install_signal_handler() {
    let _ = ctrlc::set_handler(|| {
        cleanup_live_temp();
        let w = crate::i18n::two_line(&crate::i18n::bi(
            "🦀 Ferris dọn dẹp rồi rút nha",
            "Ferris cleaned up — bye",
        ));
        eprintln!("{}", w[0]);
        eprintln!("{}", w[1]);
        std::process::exit(130);
    });
}

pub fn install_panic_hook() {
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        cleanup_live_temp();
        let w = crate::i18n::two_line(&crate::i18n::bi(
            "🦀 Ơ Ferris vấp ngã rồi, nhưng đã dọn temp xong xuôi",
            "Ferris tripped over a bug, but cleaned up the temp dir",
        ));
        eprintln!("{}", w[0]);
        eprintln!("{}", w[1]);
        if std::env::var_os("RUST_BACKTRACE").is_some() {
            default(info);
        }
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    // `cleanup_idempotent` and `panic_cleans_temp` both mutate the process-global
    // LIVE_TEMP slot. The default test runner runs them in parallel threads, so
    // without serialization one test's `.take()` could steal the other's path and
    // make an assertion flake. Hold this guard for the duration of those tests.
    static GLOBAL_SLOT_GUARD: Mutex<()> = Mutex::new(());

    #[test]
    fn sweep_orphans() {
        let base = Builder::new().tempdir().unwrap();
        let real_now = SystemTime::now();

        // 1. Non-matching dir (should never be removed)
        let other_dir = base.path().join("other-dir");
        std::fs::create_dir(&other_dir).unwrap();

        // 2. Matching dir, but fresh (should not be removed)
        let fresh_dir = base.path().join("rust-to-you-clone-fresh");
        std::fs::create_dir(&fresh_dir).unwrap();

        let removed = super::sweep_orphans(base.path(), real_now, ORPHAN_MAX_AGE);
        assert_eq!(removed, 0);
        assert!(other_dir.exists());
        assert!(fresh_dir.exists());

        // 3. Simulated future 'now' where fresh_dir becomes stale
        let future_now = real_now + ORPHAN_MAX_AGE + Duration::from_secs(1);
        let removed = super::sweep_orphans(base.path(), future_now, ORPHAN_MAX_AGE);
        assert_eq!(removed, 1);
        assert!(other_dir.exists()); // Still exists because it doesn't match prefix
        assert!(!fresh_dir.exists()); // Removed because it matched and is old enough
    }

    #[test]
    fn sweep_orphans_empty() {
        let base = Builder::new().tempdir().unwrap();
        let removed = super::sweep_orphans(base.path(), SystemTime::now(), ORPHAN_MAX_AGE);
        assert_eq!(removed, 0);
    }

    #[test]
    fn sweep_best_effort() {
        // Unreadable path
        let removed = super::sweep_orphans(Path::new("/does/not/exist/surely"), SystemTime::now(), ORPHAN_MAX_AGE);
        assert_eq!(removed, 0);
    }

    #[test]
    fn cleanup_idempotent() {
        let _guard = GLOBAL_SLOT_GUARD.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = Builder::new().prefix("rust-to-you-clone-idempotent-").tempdir().unwrap();
        let path = tmp.path().to_path_buf();
        register_live_temp(path.clone());
        
        assert!(path.exists());
        
        cleanup_live_temp();
        assert!(!path.exists());
        assert!(slot().lock().unwrap_or_else(|e| e.into_inner()).is_none());
        
        // Second call should no-op
        cleanup_live_temp();
    }

    #[test]
    fn panic_cleans_temp() {
        let _guard = GLOBAL_SLOT_GUARD.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = Builder::new().prefix("rust-to-you-clone-panic-").tempdir().unwrap();
        let path = tmp.path().to_path_buf();
        register_live_temp(path.clone());
        
        install_panic_hook();
        
        let result = std::panic::catch_unwind(|| {
            panic!("Test panic");
        });
        
        assert!(result.is_err());
        assert!(!path.exists());
        assert!(slot().lock().unwrap_or_else(|e| e.into_inner()).is_none());
    }
}
