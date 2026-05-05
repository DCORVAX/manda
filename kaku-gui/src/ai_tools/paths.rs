//! Path-resolution and sensitive-path guards used by every fs / search tool.
//!
//! Lives in its own submodule because it is pure, has no AI / LLM coupling,
//! and is the natural first slice of the long-term `ai_tools/` split (see
//! `kaku-gui/AGENTS.md`). Keeping it isolated also makes the security check
//! easy to audit independently of the dispatcher in `mod.rs`.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Refuse reads of well-known credential / system-secret locations, even when
/// the caller passes an absolute or `~/`-prefixed path (both of which bypass
/// the cwd sandbox). Best-effort canonicalization: on ENOENT we compare the
/// raw path so a file about to be created in a blocked directory is still
/// caught.
pub(crate) fn reject_if_sensitive(path: &Path) -> Result<()> {
    let canon = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let home = std::env::var("HOME").unwrap_or_default();
    let mut blocked: Vec<PathBuf> = vec![
        PathBuf::from("/etc/shadow"),
        PathBuf::from("/etc/sudoers"),
        PathBuf::from("/etc/sudoers.d"),
        PathBuf::from("/private/etc/shadow"),
        PathBuf::from("/private/etc/sudoers"),
        PathBuf::from("/private/etc/sudoers.d"),
    ];
    if !home.is_empty() {
        for rel in [".ssh", ".aws/credentials", ".gnupg", ".config/kaku/secrets"] {
            blocked.push(PathBuf::from(&home).join(rel));
        }
    }
    for b in &blocked {
        let b_canon = std::fs::canonicalize(b).unwrap_or_else(|_| b.clone());
        if canon == b_canon || canon.starts_with(&b_canon) {
            anyhow::bail!(
                "refused: '{}' is a protected secret location",
                path.display()
            );
        }
    }
    Ok(())
}

/// Handles `~/…` expansion and relative paths (resolved against `cwd`).
pub(crate) fn resolve(path: &str, cwd: &str) -> Result<PathBuf> {
    let p = if path.starts_with("~/") || path == "~" {
        let home = std::env::var("HOME").context("HOME not set")?;
        if path == "~" {
            PathBuf::from(home)
        } else {
            PathBuf::from(home).join(&path[2..])
        }
    } else if path.starts_with('/') {
        PathBuf::from(path)
    } else {
        PathBuf::from(cwd).join(path)
    };
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_expands_tilde() {
        let home = std::env::var("HOME").expect("HOME not set");
        assert_eq!(
            resolve("~/foo", "/tmp").unwrap(),
            PathBuf::from(&home).join("foo")
        );
        assert_eq!(resolve("~", "/tmp").unwrap(), PathBuf::from(&home));
    }

    #[test]
    fn resolve_absolute_unchanged() {
        assert_eq!(
            resolve("/etc/passwd", "/tmp").unwrap(),
            PathBuf::from("/etc/passwd")
        );
    }

    #[test]
    fn resolve_relative_to_cwd() {
        assert_eq!(
            resolve("src/main.rs", "/project").unwrap(),
            PathBuf::from("/project/src/main.rs")
        );
    }

    #[test]
    fn reject_if_sensitive_blocks_ssh() {
        let home = std::env::var("HOME").expect("HOME not set");
        let ssh = PathBuf::from(&home).join(".ssh");
        let err = reject_if_sensitive(&ssh).expect_err("must reject ~/.ssh");
        assert!(err.to_string().contains("protected secret location"));
    }

    #[test]
    fn reject_if_sensitive_allows_normal_paths() {
        // /tmp is not in the blocked list; resolve_if_sensitive must Ok it.
        assert!(reject_if_sensitive(&PathBuf::from("/tmp")).is_ok());
    }
}
