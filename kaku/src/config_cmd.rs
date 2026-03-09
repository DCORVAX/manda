use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;

use crate::config_tui;

#[derive(Debug, Parser, Clone, Default)]
pub struct ConfigCommand {
    /// Ensure an editable Kaku config file exists, but do not open it.
    #[arg(long, hide = true)]
    ensure_only: bool,
}

impl ConfigCommand {
    pub fn run(&self, config_path: Option<PathBuf>) -> anyhow::Result<()> {
        let config_path = config_tui::ensure_editable_config_exists(config_path.as_deref())?;
        if self.ensure_only {
            println!("Ensured config: {}", config_path.display());
            return Ok(());
        }

        // Launch TUI
        config_tui::run(config_path).context("config tui")
    }
}
