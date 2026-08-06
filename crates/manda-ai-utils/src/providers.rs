//! Predefined OpenAI-compatible provider presets for MANDA.
//!
//! Ported from the original Python MANDA engine (`manda/providers.py`) so the
//! GUI client and the CLI/TUI share one source of truth. These are the
//! "bring your own AI" profiles selectable from `manda ai` / `manda provider`.

/// A single selectable provider profile.
///
/// All five built-in providers speak the OpenAI Chat Completions protocol, so
/// a preset is pure configuration data: base URL, default simple/deep models,
/// the curated deep-model choices, and the environment variable that holds the
/// API key (matching the original MANDA env-first key resolution).
pub struct ProviderPreset {
    /// Stable machine name, e.g. "nvidia". Also used by `manda provider <name>`.
    pub name: &'static str,
    /// Human-facing label shown in the TUI, e.g. "NVIDIA NIM".
    pub label: &'static str,
    /// OpenAI-compatible API root URL.
    pub base_url: &'static str,
    /// Environment variable that holds the API key.
    pub api_key_env: &'static str,
    /// Simple model used for quick command generation and lightweight chat.
    pub model: &'static str,
    /// Deep model used for the chat overlay (Cmd+L) and tool-using chat.
    pub chat_model: &'static str,
    /// Curated deep-model choices for the chat overlay model picker.
    pub chat_model_choices: &'static [&'static str],
}

/// The five provider presets accepted by MANDA.
pub const PROVIDERS: &[ProviderPreset] = &[
    ProviderPreset {
        name: "nvidia",
        label: "NVIDIA NIM",
        base_url: "https://integrate.api.nvidia.com/v1",
        api_key_env: "NVIDIA_API_KEY",
        model: "openai/gpt-oss-20b",
        chat_model: "nvidia/nemotron-3-ultra-550b-a55b",
        chat_model_choices: &[
            "nvidia/nemotron-3-ultra-550b-a55b",
            "nvidia/nemotron-3-super-120b-a12b",
            "nvidia/nemotron-3-nano-30b-a3b",
            "openai/gpt-oss-120b",
            "openai/gpt-oss-20b",
        ],
    },
    ProviderPreset {
        name: "gemini",
        label: "Google Gemini",
        base_url: "https://generativelanguage.googleapis.com/v1beta/openai/",
        api_key_env: "GEMINI_API_KEY",
        model: "gemini-2.5-flash",
        chat_model: "gemini-2.5-pro",
        chat_model_choices: &["gemini-2.5-pro", "gemini-2.5-flash"],
    },
    ProviderPreset {
        name: "openrouter",
        label: "OpenRouter",
        base_url: "https://openrouter.ai/api/v1",
        api_key_env: "OPENROUTER_API_KEY",
        model: "google/gemini-2.5-flash:free",
        chat_model: "deepseek/deepseek-chat-v3-0324:free",
        chat_model_choices: &[
            "deepseek/deepseek-chat-v3-0324:free",
            "google/gemini-2.5-flash:free",
            "meta-llama/llama-3.3-70b-instruct:free",
        ],
    },
    ProviderPreset {
        name: "groq",
        label: "Groq",
        base_url: "https://api.groq.com/openai/v1",
        api_key_env: "GROQ_API_KEY",
        model: "llama-3.3-70b-versatile",
        chat_model: "llama-3.3-70b-versatile",
        chat_model_choices: &["llama-3.3-70b-versatile", "llama-3.1-8b-instant"],
    },
    ProviderPreset {
        name: "cerebras",
        label: "Cerebras",
        base_url: "https://api.cerebras.ai/v1",
        api_key_env: "CEREBRAS_API_KEY",
        model: "llama-3.3-70b",
        chat_model: "llama-3.3-70b",
        chat_model_choices: &["llama-3.3-70b"],
    },
];

/// Looks up a preset by its stable machine name.
pub fn find_provider(name: &str) -> Option<&'static ProviderPreset> {
    PROVIDERS
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(name.trim()))
}

/// Detects which built-in preset a base URL belongs to, if any.
///
/// Matching ignores trailing slashes, case, and scheme. Used by the GUI client
/// to surface a friendly provider name instead of "Custom".
pub fn detect_provider_for_base_url(base_url: &str) -> Option<&'static ProviderPreset> {
    let normalized = base_url.trim().trim_end_matches('/').to_ascii_lowercase();
    PROVIDERS.iter().find(|p| {
        p.base_url
            .trim()
            .trim_end_matches('/')
            .to_ascii_lowercase()
            == normalized
    })
}

/// The human-facing label for the provider a base URL maps to, or "Custom".
///
/// "Custom" means an OpenAI-compatible endpoint that is not one of MANDA's five
/// built-in presets, so the TUI lets the user keep editing Base URL / models.
pub fn provider_label_for_base_url(base_url: &str) -> &'static str {
    detect_provider_for_base_url(base_url)
        .map(|p| p.label)
        .unwrap_or("Custom")
}

/// `Vec<String>` of selectable provider labels, ending with "Custom".
pub fn provider_options() -> Vec<String> {
    let mut options: Vec<String> =
        PROVIDERS.iter().map(|p| p.label.to_string()).collect();
    if !options.iter().any(|o| o == "Custom") {
        options.push("Custom".to_string());
    }
    options
}

/// Finds a preset by its human-facing label (as selected in the TUI).
pub fn find_provider_by_label(label: &str) -> Option<&'static ProviderPreset> {
    PROVIDERS.iter().find(|p| p.label == label)
}
