pub mod tui;

use anyhow::Context;
use clap::Parser;
use wezterm_term::color::SrgbaTuple;

const OPENCODE_THEME_DARK_JSON: &str = r##"{
  "$schema": "https://opencode.ai/theme.json",
  "defs": {
    "bg": "#15141B",
    "panel": "#15141B",
    "element": "#1F1D28",
    "text": "#EDECEE",
    "muted": "#6D6D6D",
    "primary": "#A277FF",
    "secondary": "#61FFCA",
    "accent": "#FFCA85",
    "error": "#FF6767",
    "warning": "#FFCA85",
    "success": "#61FFCA",
    "info": "#5FA8FF",
    "border": "#15141B",
    "border_active": "#29263C",
    "border_subtle": "#15141B"
  },
  "theme": {
    "primary": { "dark": "primary", "light": "primary" },
    "secondary": { "dark": "secondary", "light": "secondary" },
    "accent": { "dark": "accent", "light": "accent" },
    "error": { "dark": "error", "light": "error" },
    "warning": { "dark": "warning", "light": "warning" },
    "success": { "dark": "success", "light": "success" },
    "info": { "dark": "info", "light": "info" },
    "text": { "dark": "text", "light": "text" },
    "textMuted": { "dark": "muted", "light": "muted" },
    "background": { "dark": "bg", "light": "bg" },
    "backgroundPanel": { "dark": "panel", "light": "panel" },
    "backgroundElement": { "dark": "element", "light": "element" },
    "border": { "dark": "border", "light": "border" },
    "borderActive": { "dark": "border_active", "light": "border_active" },
    "borderSubtle": { "dark": "border_subtle", "light": "border_subtle" },
    "diffAdded": { "dark": "success", "light": "success" },
    "diffRemoved": { "dark": "error", "light": "error" },
    "diffContext": { "dark": "muted", "light": "muted" },
    "diffHunkHeader": { "dark": "primary", "light": "primary" },
    "diffHighlightAdded": { "dark": "success", "light": "success" },
    "diffHighlightRemoved": { "dark": "error", "light": "error" },
    "diffAddedBg": { "dark": "#1B2A24", "light": "#1B2A24" },
    "diffRemovedBg": { "dark": "#2A1B20", "light": "#2A1B20" },
    "diffContextBg": { "dark": "bg", "light": "bg" },
    "diffLineNumber": { "dark": "muted", "light": "muted" },
    "diffAddedLineNumberBg": { "dark": "#1B2A24", "light": "#1B2A24" },
    "diffRemovedLineNumberBg": { "dark": "#2A1B20", "light": "#2A1B20" },
    "markdownText": { "dark": "text", "light": "text" },
    "markdownHeading": { "dark": "primary", "light": "primary" },
    "markdownLink": { "dark": "info", "light": "info" },
    "markdownLinkText": { "dark": "primary", "light": "primary" },
    "markdownCode": { "dark": "accent", "light": "accent" },
    "markdownBlockQuote": { "dark": "muted", "light": "muted" },
    "markdownEmph": { "dark": "accent", "light": "accent" },
    "markdownStrong": { "dark": "secondary", "light": "secondary" },
    "markdownHorizontalRule": { "dark": "muted", "light": "muted" },
    "markdownListItem": { "dark": "primary", "light": "primary" },
    "markdownListEnumeration": { "dark": "accent", "light": "accent" },
    "markdownImage": { "dark": "info", "light": "info" },
    "markdownImageText": { "dark": "primary", "light": "primary" },
    "markdownCodeBlock": { "dark": "text", "light": "text" },
    "syntaxComment": { "dark": "muted", "light": "muted" },
    "syntaxKeyword": { "dark": "primary", "light": "primary" },
    "syntaxFunction": { "dark": "secondary", "light": "secondary" },
    "syntaxVariable": { "dark": "text", "light": "text" },
    "syntaxString": { "dark": "success", "light": "success" },
    "syntaxNumber": { "dark": "accent", "light": "accent" },
    "syntaxType": { "dark": "info", "light": "info" },
    "syntaxOperator": { "dark": "primary", "light": "primary" },
    "syntaxPunctuation": { "dark": "text", "light": "text" }
  }
}"##;

const OPENCODE_THEME_LIGHT_JSON: &str = r##"{
  "$schema": "https://opencode.ai/theme.json",
  "defs": {
    "bg": "#FFFCF0",
    "panel": "#FAF7EA",
    "element": "#F3EEDA",
    "text": "#403E3C",
    "muted": "#7A7872",
    "primary": "#5E3DB3",
    "secondary": "#24837B",
    "accent": "#9A7400",
    "error": "#AF3029",
    "warning": "#9A7400",
    "success": "#24837B",
    "info": "#205EA6",
    "border": "#DDD8C8",
    "border_active": "#C7C1AE",
    "border_subtle": "#ECE7D7"
  },
  "theme": {
    "primary": { "dark": "primary", "light": "primary" },
    "secondary": { "dark": "secondary", "light": "secondary" },
    "accent": { "dark": "accent", "light": "accent" },
    "error": { "dark": "error", "light": "error" },
    "warning": { "dark": "warning", "light": "warning" },
    "success": { "dark": "success", "light": "success" },
    "info": { "dark": "info", "light": "info" },
    "text": { "dark": "text", "light": "text" },
    "textMuted": { "dark": "muted", "light": "muted" },
    "background": { "dark": "bg", "light": "bg" },
    "backgroundPanel": { "dark": "panel", "light": "panel" },
    "backgroundElement": { "dark": "element", "light": "element" },
    "border": { "dark": "border", "light": "border" },
    "borderActive": { "dark": "border_active", "light": "border_active" },
    "borderSubtle": { "dark": "border_subtle", "light": "border_subtle" },
    "diffAdded": { "dark": "success", "light": "success" },
    "diffRemoved": { "dark": "error", "light": "error" },
    "diffContext": { "dark": "muted", "light": "muted" },
    "diffHunkHeader": { "dark": "primary", "light": "primary" },
    "diffHighlightAdded": { "dark": "success", "light": "success" },
    "diffHighlightRemoved": { "dark": "error", "light": "error" },
    "diffAddedBg": { "dark": "#EAF4EC", "light": "#EAF4EC" },
    "diffRemovedBg": { "dark": "#F8EBEA", "light": "#F8EBEA" },
    "diffContextBg": { "dark": "bg", "light": "bg" },
    "diffLineNumber": { "dark": "muted", "light": "muted" },
    "diffAddedLineNumberBg": { "dark": "#EAF4EC", "light": "#EAF4EC" },
    "diffRemovedLineNumberBg": { "dark": "#F8EBEA", "light": "#F8EBEA" },
    "markdownText": { "dark": "text", "light": "text" },
    "markdownHeading": { "dark": "primary", "light": "primary" },
    "markdownLink": { "dark": "info", "light": "info" },
    "markdownLinkText": { "dark": "primary", "light": "primary" },
    "markdownCode": { "dark": "accent", "light": "accent" },
    "markdownBlockQuote": { "dark": "muted", "light": "muted" },
    "markdownEmph": { "dark": "accent", "light": "accent" },
    "markdownStrong": { "dark": "secondary", "light": "secondary" },
    "markdownHorizontalRule": { "dark": "muted", "light": "muted" },
    "markdownListItem": { "dark": "primary", "light": "primary" },
    "markdownListEnumeration": { "dark": "accent", "light": "accent" },
    "markdownImage": { "dark": "info", "light": "info" },
    "markdownImageText": { "dark": "primary", "light": "primary" },
    "markdownCodeBlock": { "dark": "text", "light": "text" },
    "syntaxComment": { "dark": "muted", "light": "muted" },
    "syntaxKeyword": { "dark": "primary", "light": "primary" },
    "syntaxFunction": { "dark": "secondary", "light": "secondary" },
    "syntaxVariable": { "dark": "text", "light": "text" },
    "syntaxString": { "dark": "success", "light": "success" },
    "syntaxNumber": { "dark": "accent", "light": "accent" },
    "syntaxType": { "dark": "info", "light": "info" },
    "syntaxOperator": { "dark": "primary", "light": "primary" },
    "syntaxPunctuation": { "dark": "text", "light": "text" }
  }
}"##;

fn opaque(color: SrgbaTuple) -> SrgbaTuple {
    SrgbaTuple(color.0, color.1, color.2, 1.0)
}

fn blend(base: SrgbaTuple, overlay: SrgbaTuple, amount: f32) -> SrgbaTuple {
    let amount = amount.clamp(0.0, 1.0);
    SrgbaTuple(
        base.0 + (overlay.0 - base.0) * amount,
        base.1 + (overlay.1 - base.1) * amount,
        base.2 + (overlay.2 - base.2) * amount,
        1.0,
    )
}

fn to_hex(color: SrgbaTuple) -> String {
    let (r, g, b, _) = opaque(color).to_srgb_u8();
    format!("#{r:02X}{g:02X}{b:02X}")
}

/// Returns OpenCode theme JSON derived from the user's current Kaku palette.
pub fn opencode_theme_json() -> String {
    let palette = crate::kaku_theme::current_theme_palette();
    if palette.is_light
        && to_hex(palette.bg) == "#FFFCF0"
        && to_hex(palette.primary) == "#5E3DB3"
        && to_hex(palette.panel) == "#FAF7EA"
    {
        return OPENCODE_THEME_LIGHT_JSON.to_string();
    }
    if !palette.is_light
        && to_hex(palette.bg) == "#15141B"
        && to_hex(palette.primary) == "#A277FF"
        && to_hex(palette.panel) == "#1F1D28"
    {
        return OPENCODE_THEME_DARK_JSON.to_string();
    }

    let warning = palette.accent;
    let success = palette.secondary;
    let info = palette.info;
    let border = blend(
        palette.bg,
        palette.text,
        if palette.is_light { 0.14 } else { 0.18 },
    );
    let border_active = blend(
        palette.bg,
        palette.primary,
        if palette.is_light { 0.28 } else { 0.32 },
    );
    let border_subtle = blend(
        palette.bg,
        palette.text,
        if palette.is_light { 0.08 } else { 0.1 },
    );
    let element = blend(
        palette.bg,
        palette.text,
        if palette.is_light { 0.09 } else { 0.13 },
    );
    let diff_added_bg = blend(
        palette.bg,
        success,
        if palette.is_light { 0.14 } else { 0.18 },
    );
    let diff_removed_bg = blend(
        palette.bg,
        palette.error,
        if palette.is_light { 0.14 } else { 0.18 },
    );

    format!(
        r#"{{
  "$schema": "https://opencode.ai/theme.json",
  "defs": {{
    "bg": "{bg}",
    "panel": "{panel}",
    "element": "{element}",
    "text": "{text}",
    "muted": "{muted}",
    "primary": "{primary}",
    "secondary": "{secondary}",
    "accent": "{accent}",
    "error": "{error}",
    "warning": "{warning}",
    "success": "{success}",
    "info": "{info}",
    "border": "{border}",
    "border_active": "{border_active}",
    "border_subtle": "{border_subtle}",
    "diff_added_bg": "{diff_added_bg}",
    "diff_removed_bg": "{diff_removed_bg}"
  }},
  "theme": {{
    "primary": {{ "dark": "primary", "light": "primary" }},
    "secondary": {{ "dark": "secondary", "light": "secondary" }},
    "accent": {{ "dark": "accent", "light": "accent" }},
    "error": {{ "dark": "error", "light": "error" }},
    "warning": {{ "dark": "warning", "light": "warning" }},
    "success": {{ "dark": "success", "light": "success" }},
    "info": {{ "dark": "info", "light": "info" }},
    "text": {{ "dark": "text", "light": "text" }},
    "textMuted": {{ "dark": "muted", "light": "muted" }},
    "background": {{ "dark": "bg", "light": "bg" }},
    "backgroundPanel": {{ "dark": "panel", "light": "panel" }},
    "backgroundElement": {{ "dark": "element", "light": "element" }},
    "border": {{ "dark": "border", "light": "border" }},
    "borderActive": {{ "dark": "border_active", "light": "border_active" }},
    "borderSubtle": {{ "dark": "border_subtle", "light": "border_subtle" }},
    "diffAdded": {{ "dark": "success", "light": "success" }},
    "diffRemoved": {{ "dark": "error", "light": "error" }},
    "diffContext": {{ "dark": "muted", "light": "muted" }},
    "diffHunkHeader": {{ "dark": "primary", "light": "primary" }},
    "diffHighlightAdded": {{ "dark": "success", "light": "success" }},
    "diffHighlightRemoved": {{ "dark": "error", "light": "error" }},
    "diffAddedBg": {{ "dark": "diff_added_bg", "light": "diff_added_bg" }},
    "diffRemovedBg": {{ "dark": "diff_removed_bg", "light": "diff_removed_bg" }},
    "diffContextBg": {{ "dark": "bg", "light": "bg" }},
    "diffLineNumber": {{ "dark": "muted", "light": "muted" }},
    "diffAddedLineNumberBg": {{ "dark": "diff_added_bg", "light": "diff_added_bg" }},
    "diffRemovedLineNumberBg": {{ "dark": "diff_removed_bg", "light": "diff_removed_bg" }},
    "markdownText": {{ "dark": "text", "light": "text" }},
    "markdownHeading": {{ "dark": "primary", "light": "primary" }},
    "markdownLink": {{ "dark": "info", "light": "info" }},
    "markdownLinkText": {{ "dark": "primary", "light": "primary" }},
    "markdownCode": {{ "dark": "accent", "light": "accent" }},
    "markdownBlockQuote": {{ "dark": "muted", "light": "muted" }},
    "markdownEmph": {{ "dark": "accent", "light": "accent" }},
    "markdownStrong": {{ "dark": "secondary", "light": "secondary" }},
    "markdownHorizontalRule": {{ "dark": "muted", "light": "muted" }},
    "markdownListItem": {{ "dark": "primary", "light": "primary" }},
    "markdownListEnumeration": {{ "dark": "accent", "light": "accent" }},
    "markdownImage": {{ "dark": "info", "light": "info" }},
    "markdownImageText": {{ "dark": "primary", "light": "primary" }},
    "markdownCodeBlock": {{ "dark": "text", "light": "text" }},
    "syntaxComment": {{ "dark": "muted", "light": "muted" }},
    "syntaxKeyword": {{ "dark": "primary", "light": "primary" }},
    "syntaxFunction": {{ "dark": "secondary", "light": "secondary" }},
    "syntaxVariable": {{ "dark": "text", "light": "text" }},
    "syntaxString": {{ "dark": "success", "light": "success" }},
    "syntaxNumber": {{ "dark": "accent", "light": "accent" }},
    "syntaxType": {{ "dark": "info", "light": "info" }},
    "syntaxOperator": {{ "dark": "primary", "light": "primary" }},
    "syntaxPunctuation": {{ "dark": "text", "light": "text" }}
  }}
}}"#,
        bg = to_hex(palette.bg),
        panel = to_hex(palette.panel),
        element = to_hex(element),
        text = to_hex(palette.text),
        muted = to_hex(palette.muted),
        primary = to_hex(palette.primary),
        secondary = to_hex(palette.secondary),
        accent = to_hex(palette.accent),
        error = to_hex(palette.error),
        warning = to_hex(warning),
        success = to_hex(success),
        info = to_hex(info),
        border = to_hex(border),
        border_active = to_hex(border_active),
        border_subtle = to_hex(border_subtle),
        diff_added_bg = to_hex(diff_added_bg),
        diff_removed_bg = to_hex(diff_removed_bg),
    )
}

#[derive(Debug, Parser, Clone, Default)]
pub struct AiConfigCommand {}

impl AiConfigCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        tui::run().context("ai config tui")
    }
}

#[cfg(test)]
mod tests {
    use super::opencode_theme_json;

    #[test]
    fn opencode_theme_json_is_valid_json() {
        let json = opencode_theme_json();
        let parsed: serde_json::Value =
            serde_json::from_str(&json).expect("opencode theme json should parse");

        assert_eq!(parsed["$schema"], "https://opencode.ai/theme.json");
        assert!(parsed["defs"]["bg"].is_string());
        assert!(parsed["theme"]["background"]["dark"].is_string());
    }
}
