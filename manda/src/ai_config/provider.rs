//! `manda provider` — select a MANDA provider preset or list the available ones.
//!
//! MANDA ships five OpenAI-compatible provider presets (nvidia, gemini,
//! openrouter, groq, cerebras). This command writes the chosen preset into
//! `assistant.toml` so it can be used by both the GUI chat (Cmd+L) and
//! `manda chat`/`manda ai`. Calling it with no argument lists the presets.

use std::io::Write;

use anyhow::Context;
use clap::Parser;

use manda_ai_utils::providers;

#[derive(Debug, Parser, Clone)]
pub struct ProviderCommand {
    /// Provider name or label to activate (e.g. "nvidia", "groq", "Cerebras").
    #[arg(value_name = "NAME")]
    pub name: Option<String>,
}

impl ProviderCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.name {
            Some(name) => self.activate(name),
            None => self.list(),
        }
    }

    fn activate(&self, name: &str) -> anyhow::Result<()> {
        let preset = providers::find_provider(name)
            .or_else(|| providers::find_provider_by_label(name))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "unknown provider '{}'. Run `manda provider` to list the available presets.",
                    name
                )
            })?;

        let path = crate::assistant_config::ensure_assistant_toml_exists()
            .context("ensure assistant.toml exists")?;
        super::tui::save_manda_assistant_field_to_path(&path, "Provider", preset.label)
            .context("apply provider preset")?;

        println!("Activated provider: {}", preset.label);
        println!("  base_url : {}", preset.base_url);
        println!("  model    : {}", preset.model);
        println!("  chat_model: {}", preset.chat_model);
        println!(
            "  api_key  : reads ${} (or set api_key in assistant.toml)",
            preset.api_key_env
        );
        Ok(())
    }

    fn list(&self) -> anyhow::Result<()> {
        let mut out = std::io::BufWriter::new(std::io::stdout());
        writeln!(
            out,
            "MANDA AI providers — select with `manda provider <name>`:"
        )?;
        for p in providers::PROVIDERS {
            writeln!(out, "  {} ({})", p.label, p.name)?;
            writeln!(out, "    base_url : {}", p.base_url)?;
            writeln!(out, "    model    : {}", p.model)?;
            writeln!(out, "    chat_model: {}", p.chat_model)?;
            writeln!(out, "    api key  : ${}", p.api_key_env)?;
        }
        out.flush().context("flush stdout")?;
        Ok(())
    }
}
