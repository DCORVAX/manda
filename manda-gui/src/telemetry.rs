//! Opt-in anonymous telemetry for MANDA.
//!
//! Disabled by default. Enable with `MANDA_TELEMETRY=1` (env var) or
//! `telemetry = true` in `assistant.toml`.
//!
//! Events are written as one JSON line per event to
//! `<config_dir>/telemetry.jsonl` where config_dir honors
//! `$XDG_CONFIG_HOME/manda` and falls back to `~/.config/manda`. No personal
//! data is collected: only coarse usage signals (event name, provider, model,
//! message counts) plus a random per-install id so repeated events are
//! de-duplicable without identifying the user. Nothing is sent over the
//! network by this module; a future "opt-in upload" step would be a separate,
//! explicitly documented feature.

use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const ENV_FLAG: &str = "MANDA_TELEMETRY";

fn env_value_enables(value: Option<&str>) -> bool {
    value.is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

fn enabled_by_env() -> bool {
    env_value_enables(std::env::var(ENV_FLAG).ok().as_deref())
}

/// Config dir honoring XDG: `$XDG_CONFIG_HOME/manda` or `~/.config/manda`.
/// Mirrors the resolution used by `crate::soul`.
fn manda_config_dir() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return Some(PathBuf::from(xdg).join("manda"));
        }
    }
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config/manda"))
}

fn assistant_toml_path() -> Option<PathBuf> {
    manda_config_dir().map(|dir| dir.join("assistant.toml"))
}

fn enabled_by_assistant_toml() -> bool {
    let Some(path) = assistant_toml_path() else {
        return false;
    };
    let Ok(text) = std::fs::read_to_string(path) else {
        return false;
    };
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == "telemetry" {
                let v = value.trim().trim_matches('"').trim_matches('\'');
                return v == "true" || v == "1";
            }
        }
    }
    false
}

fn install_id() -> String {
    let Some(dir) = manda_config_dir() else {
        return String::new();
    };
    let path = dir.join("telemetry_id");
    if let Ok(existing) = std::fs::read_to_string(&path) {
        let existing = existing.trim();
        if !existing.is_empty() {
            return existing.to_string();
        }
    }
    // Random per-install id derived from time + pid. Not derived from any
    // user data; just enough to de-duplicate.
    let seed = format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default() as u64
            ^ (std::process::id() as u64) << 32
    );
    let id = format!("manda-{}", &seed[..seed.len().min(12)]);
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(&path, &id);
    id
}

// The config file is tiny, but chat submits are frequent enough that we avoid
// a disk read on every submit: cache the enabled state for a short TTL.
static ENABLED_CACHE: Mutex<Option<(Instant, bool)>> = Mutex::new(None);
const ENABLED_CACHE_TTL: Duration = Duration::from_secs(30);

/// Returns true when telemetry is enabled by env or config (cached briefly).
pub fn enabled() -> bool {
    if enabled_by_env() {
        return true;
    }
    let mut cache = match ENABLED_CACHE.lock() {
        Ok(c) => c,
        Err(_) => return enabled_by_assistant_toml(), // poisoned: read directly
    };
    if let Some((at, value)) = *cache {
        if at.elapsed() < ENABLED_CACHE_TTL {
            return value;
        }
    }
    let value = enabled_by_assistant_toml();
    *cache = Some((Instant::now(), value));
    value
}

/// Record an anonymous usage event. No-op when telemetry is disabled.
/// Serialization failures are swallowed: telemetry must never break the app.
pub fn record(event: &str, provider: &str, model: &str, payload: Option<&str>) {
    if !enabled() {
        return;
    }
    let Some(dir) = manda_config_dir() else {
        return;
    };
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let line = serde_json::json!({
        "event": event,
        "provider": provider,
        "model": model,
        "payload": payload,
        "install_id": install_id(),
        "ts": chrono::Utc::now().timestamp(),
        "version": env!("CARGO_PKG_VERSION"),
    });
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("telemetry.jsonl"))
    {
        Ok(f) => f,
        Err(_) => return,
    };
    let _ = writeln!(file, "{}", line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_flag_parsing_is_pure() {
        // Pure helper: no global env mutation, so tests never race.
        assert!(!env_value_enables(None));
        assert!(!env_value_enables(Some("0")));
        assert!(!env_value_enables(Some("false")));
        assert!(env_value_enables(Some("1")));
        assert!(env_value_enables(Some("true")));
        assert!(env_value_enables(Some("TRUE")));
    }

    #[test]
    fn config_dir_honors_xdg() {
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", "/tmp/xdg-test");
        }
        let dir = manda_config_dir();
        assert_eq!(dir, Some(PathBuf::from("/tmp/xdg-test/manda")));
        unsafe {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
    }

    #[test]
    fn telemetry_disabled_by_default() {
        // Without the env var and without a real assistant.toml, enabled()
        // should be false in the test environment (HOME may not exist).
        assert!(!enabled_by_env());
    }

    #[test]
    fn record_is_noop_when_disabled() {
        // Calling record() with telemetry disabled must not panic or write.
        record("chat_submit", "nvidia", "test-model", None);
    }
}
