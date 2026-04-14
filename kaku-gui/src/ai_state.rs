//! Persistent state for the AI chat overlay.
//!
//! Stores UI state like the last selected model in `~/.config/kaku/ai_chat_state.json`.
//! Load once at overlay start; save when the user switches models.

use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize)]
struct StateFile {
    version: u32,
    /// Last model selected by the user via Shift+Tab.
    last_model: Option<String>,
}

/// Load the last selected model from disk. Returns None on any error (non-fatal).
pub fn load_last_model() -> Option<String> {
    match try_load() {
        Ok(model) => model,
        Err(e) => {
            log::warn!("Could not load AI chat state: {e}");
            None
        }
    }
}

fn try_load() -> Result<Option<String>> {
    let path = state_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let raw = std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let file: StateFile =
        serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
    Ok(file.last_model)
}

/// Save the last selected model to disk atomically.
pub fn save_last_model(model: &str) -> Result<()> {
    let path = state_path()?;
    let file = StateFile {
        version: 1,
        last_model: Some(model.to_string()),
    };
    let json = serde_json::to_string_pretty(&file).context("serialize state")?;

    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, &json).with_context(|| format!("write {}", tmp.display()))?;
    std::fs::rename(&tmp, &path)
        .with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}

fn state_path() -> Result<PathBuf> {
    let user_config_path = config::user_config_path();
    let config_dir = user_config_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("invalid user config path"))?;
    Ok(config_dir.join("ai_chat_state.json"))
}
