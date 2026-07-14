use super::TermWindow;
use crate::overlay::start_overlay_pane;
use mux::pane::{CachePolicy, Pane};
use std::ops::Range;
use std::sync::Arc;
use termwiz::surface::Line;
use wezterm_term::color::{ColorPalette, SrgbaTuple};
use wezterm_term::StableRowIndex;

const VISIBLE_CONTEXT_ROWS: StableRowIndex = 20;
const TAB_SNAPSHOT_ROWS: StableRowIndex = 120;
const TAB_SNAPSHOT_BYTE_LIMIT: usize = 12 * 1024;
const FAILED_COMMAND_OUTPUT_ROWS: StableRowIndex = 51;

pub(super) fn toggle_overlay(term: &mut TermWindow, pane: &Arc<dyn Pane>) {
    let pane_id = pane.pane_id();
    if term.ai_chat_overlay_panes.contains(&pane_id) {
        term.cancel_overlay_for_pane(pane_id);
        return;
    }

    let context = build_terminal_context(term, pane);
    let (overlay, future) = start_overlay_pane(term, pane, move |pane_id, term| {
        crate::overlay::ai_chat::ai_chat_overlay(pane_id, term, context)
    });
    term.assign_overlay_for_pane(pane_id, overlay);
    term.ai_chat_overlay_panes.insert(pane_id);

    // The AI chat overlay uses tighter bottom padding. Re-run layout so the
    // overlay pane gets the extra row(s) immediately.
    if let Some(window) = term.window.clone() {
        let dims = term.dimensions;
        term.apply_dimensions(&dims, None, &window, false);
    }

    promise::spawn::spawn(async move {
        if let Err(e) = future.await {
            log::error!("AI chat overlay error for pane {pane_id}: {e:#}");
        }
    })
    .detach();
}

fn build_terminal_context(
    term: &mut TermWindow,
    pane: &Arc<dyn Pane>,
) -> crate::overlay::ai_chat::TerminalContext {
    let dims = pane.get_dimensions();
    let bottom = dims.physical_top + dims.viewport_rows as StableRowIndex;

    let visible_top = bottom.saturating_sub(VISIBLE_CONTEXT_ROWS);
    let (_, visible_lines) = pane.get_lines(visible_top..bottom);
    let visible_lines = line_texts(&visible_lines);

    let tab_top = bottom.saturating_sub(TAB_SNAPSHOT_ROWS);
    let (_, tab_lines) = pane.get_lines(tab_top..bottom);
    let tab_snapshot = tab_snapshot_text(&tab_lines);

    let cwd = pane
        .get_current_working_dir(CachePolicy::AllowStale)
        .map(|u| u.path().to_string())
        .unwrap_or_default();
    let selected_text = term.selection_text(pane);

    let last_exit_code = pane.get_last_command_status();
    let last_command_output =
        failed_command_output_range(last_exit_code, pane.get_last_command_output_start(), bottom)
            .map(|range| {
                let (_, output_lines) = pane.get_lines(range);
                line_texts(&output_lines)
            });

    crate::overlay::ai_chat::TerminalContext {
        cwd,
        visible_lines,
        tab_snapshot,
        selected_text,
        colors: chat_palette(term.palette()),
        panel_cols: dims.cols,
        panel_rows: dims.viewport_rows,
        last_exit_code,
        last_command_output,
    }
}

fn line_texts(lines: &[Line]) -> Vec<String> {
    lines.iter().map(|line| line.as_str().to_string()).collect()
}

fn tab_snapshot_text(lines: &[Line]) -> String {
    let mut snapshot = String::new();
    for line in lines {
        let text = line.as_str();
        let next_len = snapshot.len() + text.len() + 1;
        if next_len > TAB_SNAPSHOT_BYTE_LIMIT {
            break;
        }
        if !snapshot.is_empty() {
            snapshot.push('\n');
        }
        snapshot.push_str(&text);
    }
    snapshot
}

fn failed_command_output_range(
    last_exit_code: Option<i32>,
    output_start: Option<StableRowIndex>,
    bottom: StableRowIndex,
) -> Option<Range<StableRowIndex>> {
    let code = last_exit_code?;
    if code == 0 {
        return None;
    }
    let output_start = output_start?;
    let output_end = bottom.min(output_start + FAILED_COMMAND_OUTPUT_ROWS);
    (output_end > output_start).then_some(output_start..output_end)
}

fn chat_palette(pal: &ColorPalette) -> crate::overlay::ai_chat::ChatPalette {
    fn srgb(t: SrgbaTuple) -> SrgbaTuple {
        t
    }

    // colors.0 layout: 0-7 = ANSI, 8-15 = bright ANSI.
    // bright cyan (14) for accent, bright black (8) for border,
    // bright yellow (11) for user header.
    crate::overlay::ai_chat::ChatPalette {
        bg: srgb(pal.background),
        fg: srgb(pal.foreground),
        accent: srgb(pal.colors.0[14]),
        border: srgb(pal.colors.0[8]),
        user_header: srgb(pal.colors.0[11]),
        user_text: srgb(pal.foreground),
        ai_text: srgb(pal.foreground),
        selection_fg: srgb(pal.selection_fg),
        selection_bg: srgb(pal.selection_bg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use termwiz::cell::CellAttributes;
    use termwiz::surface::SEQ_ZERO;

    fn line(text: &str) -> Line {
        Line::from_text(text, &CellAttributes::default(), SEQ_ZERO, None)
    }

    #[test]
    fn tab_snapshot_stops_before_byte_limit() {
        let lines = vec![line(&"a".repeat(TAB_SNAPSHOT_BYTE_LIMIT)), line("later")];

        assert_eq!(tab_snapshot_text(&lines), "");
    }

    #[test]
    fn tab_snapshot_joins_lines_until_limit() {
        let lines = vec![line("one"), line("two")];

        assert_eq!(tab_snapshot_text(&lines), "one\ntwo");
    }

    #[test]
    fn failed_command_output_requires_nonzero_status() {
        assert_eq!(failed_command_output_range(Some(0), Some(10), 40), None);
        assert_eq!(failed_command_output_range(None, Some(10), 40), None);
        assert_eq!(failed_command_output_range(Some(1), None, 40), None);
    }

    #[test]
    fn failed_command_output_is_capped_at_existing_window() {
        assert_eq!(
            failed_command_output_range(Some(1), Some(10), 100),
            Some(10..61)
        );
        assert_eq!(
            failed_command_output_range(Some(1), Some(10), 20),
            Some(10..20)
        );
    }

    #[test]
    fn failed_command_output_keeps_empty_ranges_out() {
        assert_eq!(failed_command_output_range(Some(1), Some(20), 20), None);
    }
}
