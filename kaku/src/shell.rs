use clap::ValueEnum;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ManagedShell {
    Zsh,
    Fish,
}

impl ManagedShell {
    pub fn name(self) -> &'static str {
        match self {
            Self::Zsh => "zsh",
            Self::Fish => "fish",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellKind {
    Zsh,
    Fish,
    Unsupported(String),
    Unknown,
}

impl ShellKind {
    pub fn is_managed(&self) -> bool {
        matches!(self, ShellKind::Zsh | ShellKind::Fish)
    }

    pub fn name(&self) -> &str {
        match self {
            ShellKind::Zsh => "zsh",
            ShellKind::Fish => "fish",
            ShellKind::Unsupported(s) => s.as_str(),
            ShellKind::Unknown => "unknown",
        }
    }

    pub fn managed(&self) -> Option<ManagedShell> {
        match self {
            Self::Zsh => Some(ManagedShell::Zsh),
            Self::Fish => Some(ManagedShell::Fish),
            _ => None,
        }
    }
}

pub fn detect_shell_kind() -> ShellKind {
    match std::env::var("SHELL") {
        Err(_) => ShellKind::Unknown,
        Ok(s) => shell_kind_from_path(&s),
    }
}

pub fn resolve_shell_kind(shell: Option<ManagedShell>) -> ShellKind {
    match shell {
        Some(ManagedShell::Zsh) => ShellKind::Zsh,
        Some(ManagedShell::Fish) => ShellKind::Fish,
        None => detect_shell_kind(),
    }
}

pub fn preferred_managed_shell() -> ManagedShell {
    detect_shell_kind().managed().unwrap_or(ManagedShell::Zsh)
}

pub fn find_shell_executable(shell: ManagedShell) -> Option<PathBuf> {
    if let Some(current) = std::env::var_os("SHELL").map(PathBuf::from) {
        if current.file_name().and_then(OsStr::to_str) == Some(shell.name())
            && config::is_executable_file(&current)
        {
            return Some(current);
        }
    }

    if let Some(path) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(shell.name());
            if config::is_executable_file(&candidate) {
                return Some(candidate);
            }
        }
    }

    let candidates: &[&str] = match shell {
        ManagedShell::Zsh => &["/bin/zsh", "/usr/bin/zsh"],
        ManagedShell::Fish => &[
            "/opt/homebrew/bin/fish",
            "/usr/local/bin/fish",
            "/usr/bin/fish",
        ],
    };
    candidates
        .iter()
        .map(PathBuf::from)
        .find(|candidate| config::is_executable_file(candidate))
}

fn shell_kind_from_path(shell: &str) -> ShellKind {
    match Path::new(shell)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("")
    {
        "zsh" => ShellKind::Zsh,
        "fish" => ShellKind::Fish,
        "" => ShellKind::Unknown,
        other => ShellKind::Unsupported(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_kind_uses_executable_name() {
        assert_eq!(shell_kind_from_path("/bin/zsh"), ShellKind::Zsh);
        assert_eq!(
            shell_kind_from_path("/opt/homebrew/bin/fish"),
            ShellKind::Fish
        );
        assert_eq!(
            shell_kind_from_path("/bin/bash"),
            ShellKind::Unsupported("bash".to_string())
        );
    }

    #[test]
    fn managed_shell_names_match_cli_values() {
        assert_eq!(ManagedShell::Zsh.name(), "zsh");
        assert_eq!(ManagedShell::Fish.name(), "fish");
    }
}
