use crate::quad::TripleLayerQuadAllocator;
use crate::termwindow::box_model::{
    BorderColor, BoxDimension, Corners, DisplayType, Element, ElementColors, ElementContent,
    LayoutContext, SizedPoly,
};
use crate::termwindow::render::corners::{
    BOTTOM_LEFT_ROUNDED_CORNER, BOTTOM_RIGHT_ROUNDED_CORNER, TOP_LEFT_ROUNDED_CORNER,
    TOP_RIGHT_ROUNDED_CORNER,
};
use crate::termwindow::render::{forces_opaque_kaku_tui_window_background, RenderScreenLineParams};
use crate::termwindow::UIItemType;
use crate::utilsprites::RenderMetrics;
use config::{ConfigHandle, Dimension, TabBarColors};
use mux::renderable::RenderableDimensions;
use mux::Mux;
use termwiz::cell::unicode_column_width;
use termwiz_funcs::truncate_right;
use wezterm_term::color::ColorAttribute;
use window::color::LinearRgba;

fn tab_pane_menu_origin(
    anchor: &crate::termwindow::UIItem,
    menu_width: f32,
    menu_height: f32,
    window_width: f32,
    window_height: f32,
    tab_bar_at_bottom: bool,
) -> (f32, f32) {
    let x = (anchor.x as f32).clamp(0.0, (window_width - menu_width).max(0.0));
    let y = if tab_bar_at_bottom {
        anchor.y as f32 - menu_height
    } else {
        anchor.y as f32 + anchor.height as f32
    }
    .clamp(0.0, (window_height - menu_height).max(0.0));
    (x, y)
}

impl crate::TermWindow {
    pub fn paint_tab_bar(&mut self, layers: &mut TripleLayerQuadAllocator) -> anyhow::Result<()> {
        let border = self.get_os_border();
        let tab_bar_height = self.tab_bar_pixel_height()?;
        let tab_bar_y = if self.config.tab_bar_at_bottom {
            ((self.dimensions.pixel_height as f32) - tab_bar_height - border.bottom.get() as f32)
                .max(0.)
        } else {
            // Offset below the OS top inset so cells aren't clipped by the
            // macOS rounded corner / integrated buttons window mask.
            border.top.get() as f32
        };
        let panes = self.get_panes_to_render();
        let force_opaque_tab_bar_background = forces_opaque_kaku_tui_window_background(&panes);

        if self.config.use_fancy_tab_bar {
            if self.fancy_tab_bar.is_none() {
                let palette = self.palette().clone();
                let tab_bar = self.build_fancy_tab_bar(&palette)?;
                self.fancy_tab_bar.replace(tab_bar);
            }

            // In transparent mode, fill the tab bar area with a transparent
            // background so it blends consistently with the window.
            let window_is_transparent =
                !self.window_background.is_empty() || self.config.window_background_opacity != 1.0;
            if window_is_transparent && !force_opaque_tab_bar_background {
                let tab_bar_bg = if let Some(active) = self.get_active_pane_or_overlay() {
                    active
                        .palette()
                        .background
                        .to_linear()
                        .mul_alpha(self.config.window_background_opacity)
                } else {
                    self.palette()
                        .background
                        .to_linear()
                        .mul_alpha(self.config.window_background_opacity)
                };
                self.filled_rectangle(
                    layers,
                    0,
                    euclid::rect(
                        0.0,
                        tab_bar_y,
                        self.dimensions.pixel_width as f32,
                        tab_bar_height,
                    ),
                    tab_bar_bg,
                )?;
            }

            let mut fancy_ui_items = self.paint_fancy_tab_bar()?;
            self.ui_items.append(&mut fancy_ui_items);
            return Ok(());
        }

        let palette = self.palette().clone();

        let tab_metrics = if self.config.tab_bar_at_bottom {
            // Bottom tabs have no rounded titlebar mask above them, so keep the
            // compact natural height used by earlier releases.
            RenderMetrics::with_font_metrics(&self.fonts.default_font()?.metrics())
        } else {
            // Top tabs sit under the macOS titlebar mask; honor line_height so
            // tall fonts don't clip against the top edge.
            self.render_metrics
        };

        self.ui_items.append(&mut self.tab_bar.compute_ui_items(
            tab_bar_y as usize,
            tab_metrics.cell_size.height as usize,
            tab_metrics.cell_size.width as usize,
        ));

        let window_is_transparent =
            !self.window_background.is_empty() || self.config.window_background_opacity != 1.0;
        let effective_window_is_transparent =
            window_is_transparent && !force_opaque_tab_bar_background;
        let gl_state = self.render_state.as_ref().unwrap();
        let white_space = gl_state.util_sprites.white_space.texture_coords();
        let filled_box = gl_state.util_sprites.filled_box.texture_coords();
        let default_bg = palette
            .resolve_bg(ColorAttribute::Default)
            .to_linear()
            .mul_alpha(if effective_window_is_transparent {
                0.
            } else {
                self.config.text_background_opacity
            });

        if effective_window_is_transparent {
            let tab_bar_bg = if let Some(active) = self.get_active_pane_or_overlay() {
                active
                    .palette()
                    .background
                    .to_linear()
                    .mul_alpha(self.config.window_background_opacity)
            } else {
                palette
                    .background
                    .to_linear()
                    .mul_alpha(self.config.window_background_opacity)
            };
            self.filled_rectangle(
                layers,
                0,
                euclid::rect(
                    0.0,
                    tab_bar_y,
                    self.dimensions.pixel_width as f32,
                    tab_bar_height,
                ),
                tab_bar_bg,
            )?;
        }

        self.render_screen_line(
            RenderScreenLineParams {
                top_pixel_y: tab_bar_y,
                left_pixel_x: 0.,
                pixel_width: self.dimensions.pixel_width as f32,
                stable_line_idx: None,
                line: self.tab_bar.line(),
                selection: 0..0,
                cursor: &Default::default(),
                palette: &palette,
                dims: &RenderableDimensions {
                    cols: self.dimensions.pixel_width / tab_metrics.cell_size.width as usize,
                    physical_top: 0,
                    scrollback_rows: 0,
                    scrollback_top: 0,
                    viewport_rows: 1,
                    dpi: self.terminal_size.dpi,
                    pixel_height: tab_metrics.cell_size.height as usize,
                    pixel_width: self.terminal_size.pixel_width,
                    reverse_video: false,
                },
                config: &self.config,
                cursor_border_color: LinearRgba::default(),
                foreground: palette.foreground.to_linear(),
                pane: None,
                is_active: true,
                selection_fg: LinearRgba::default(),
                selection_bg: LinearRgba::default(),
                cursor_fg: LinearRgba::default(),
                cursor_bg: LinearRgba::default(),
                cursor_is_default_color: true,
                white_space,
                filled_box,
                window_is_transparent: effective_window_is_transparent,
                default_bg,
                style: None,
                font: None,
                use_pixel_positioning: self.config.experimental_pixel_positioning,
                render_metrics: tab_metrics,
                shape_key: None,
                password_input: false,
            },
            layers,
        )?;

        Ok(())
    }

    pub fn paint_tab_pane_menu(&mut self) -> anyhow::Result<()> {
        let Some(menu_state) = self.tab_pane_menu else {
            return Ok(());
        };
        if self.config.use_fancy_tab_bar || !self.show_tab_bar {
            self.tab_pane_menu = None;
            return Ok(());
        }

        let mux = Mux::get();
        let Some(window) = mux.get_window(self.mux_window_id) else {
            self.tab_pane_menu = None;
            return Ok(());
        };
        let Some((tab_idx, tab)) = window
            .iter()
            .enumerate()
            .find(|(_, tab)| tab.tab_id() == menu_state.tab_id)
        else {
            self.tab_pane_menu = None;
            return Ok(());
        };
        let panes = tab.iter_panes();
        if panes.len() <= 1 {
            self.tab_pane_menu = None;
            return Ok(());
        }

        let Some(anchor) = self.ui_items.iter().find(|item| {
            matches!(
                item.item_type,
                UIItemType::TabBar(crate::tabbar::TabBarItem::Tab {
                    tab_idx: item_tab_idx,
                    ..
                }) if item_tab_idx == tab_idx
            )
        }) else {
            return Ok(());
        };
        let anchor = anchor.clone();

        let include_process = self.config.tab_title_show_foreground_process;
        let mut entries = panes
            .into_iter()
            .map(|pane| {
                let pane_info = Self::pos_pane_to_pane_info(&pane);
                let title = crate::tabbar::compute_pane_plain_title(&pane_info, include_process);
                (pane.index, pane.is_active, title)
            })
            .collect::<Vec<_>>();
        entries.sort_by_key(|(pane_idx, _, _)| *pane_idx);

        let font = self.fonts.default_font()?;
        let metrics = RenderMetrics::with_font_metrics(&font.metrics());
        let cell_width = metrics.cell_size.width as f32;
        let cell_height = metrics.cell_size.height as f32;
        let anchor_columns = ((anchor.width as f32) / cell_width).ceil() as usize;
        let content_columns = entries
            .iter()
            .map(|(_, _, title)| unicode_column_width(title, None) + 4)
            .max()
            .unwrap_or(8);
        let menu_columns = anchor_columns.max(content_columns).clamp(8, 40);
        let menu_width = menu_columns as f32 * cell_width;
        let row_height = cell_height * 1.35;
        let menu_height = row_height * entries.len() as f32 + 2.0;
        let window_width = self.dimensions.pixel_width as f32;
        let window_height = self.dimensions.pixel_height as f32;
        let (x, y) = tab_pane_menu_origin(
            &anchor,
            menu_width,
            menu_height,
            window_width,
            window_height,
            self.config.tab_bar_at_bottom,
        );

        let tab_colors = self
            .config
            .resolved_palette
            .tab_bar
            .as_ref()
            .cloned()
            .unwrap_or_else(TabBarColors::default);
        let active = tab_colors.active_tab();
        let inactive = tab_colors.inactive_tab();
        let hover = tab_colors.inactive_tab_hover();
        let background = active.bg_color.to_linear();
        let radius = Dimension::Pixels(6.0);
        let max_title_columns = menu_columns.saturating_sub(4).max(1);

        let rows = entries
            .into_iter()
            .map(|(pane_idx, is_active, title)| {
                let title = truncate_right(&title, max_title_columns);
                let label = format!("{} {title}", if is_active { "✓" } else { " " });
                let row_color = if is_active {
                    active.fg_color.to_linear()
                } else {
                    inactive.fg_color.to_linear()
                };
                Element::new(&font, ElementContent::Text(label))
                    .colors(ElementColors {
                        border: BorderColor::default(),
                        bg: background.into(),
                        text: row_color.into(),
                    })
                    .hover_colors(Some(ElementColors {
                        border: BorderColor::default(),
                        bg: hover.bg_color.to_linear().into(),
                        text: hover.fg_color.to_linear().into(),
                    }))
                    .padding(BoxDimension {
                        left: Dimension::Cells(0.5),
                        right: Dimension::Cells(0.5),
                        top: Dimension::Cells(0.15),
                        bottom: Dimension::Cells(0.15),
                    })
                    .min_width(Some(Dimension::Pixels(menu_width - 2.0)))
                    .min_height(Some(Dimension::Pixels(row_height)))
                    .item_type(UIItemType::TabPaneMenu {
                        tab_id: menu_state.tab_id,
                        pane_idx,
                    })
                    .display(DisplayType::Block)
            })
            .collect::<Vec<_>>();

        let menu = Element::new(&font, ElementContent::Children(rows))
            .colors(ElementColors {
                border: BorderColor::new(background.into()),
                bg: background.into(),
                text: active.fg_color.to_linear().into(),
            })
            .border(BoxDimension::new(Dimension::Pixels(1.0)))
            .border_corners(Some(Corners {
                top_left: SizedPoly {
                    width: radius,
                    height: radius,
                    poly: TOP_LEFT_ROUNDED_CORNER,
                },
                top_right: SizedPoly {
                    width: radius,
                    height: radius,
                    poly: TOP_RIGHT_ROUNDED_CORNER,
                },
                bottom_left: SizedPoly {
                    width: radius,
                    height: radius,
                    poly: BOTTOM_LEFT_ROUNDED_CORNER,
                },
                bottom_right: SizedPoly {
                    width: radius,
                    height: radius,
                    poly: BOTTOM_RIGHT_ROUNDED_CORNER,
                },
            }))
            .min_width(Some(Dimension::Pixels(menu_width)))
            .max_width(Some(Dimension::Pixels(menu_width)))
            .display(DisplayType::Block)
            .zindex(120);

        let computed = self.compute_element(
            &LayoutContext {
                height: config::DimensionContext {
                    dpi: self.dimensions.dpi as f32,
                    pixel_max: window_height,
                    pixel_cell: cell_height,
                },
                width: config::DimensionContext {
                    dpi: self.dimensions.dpi as f32,
                    pixel_max: window_width,
                    pixel_cell: cell_width,
                },
                bounds: euclid::rect(x, y, menu_width, menu_height),
                metrics: &metrics,
                gl_state: self.render_state.as_ref().unwrap(),
                zindex: 120,
            },
            &menu,
        )?;
        let mut ui_items = computed.ui_items();
        let gl_state = self.render_state.as_ref().unwrap();
        self.render_element(&computed, gl_state, None)?;
        self.ui_items.append(&mut ui_items);

        Ok(())
    }

    pub fn tab_bar_pixel_height_impl(
        config: &ConfigHandle,
        fontconfig: &wezterm_font::FontConfiguration,
        render_metrics: &RenderMetrics,
    ) -> anyhow::Result<f32> {
        if config.use_fancy_tab_bar {
            let font = fontconfig.title_font()?;
            Ok((font.metrics().cell_height.get() as f32 * 1.75).ceil())
        } else if config.tab_bar_at_bottom {
            Ok(render_metrics.natural_cell_height as f32)
        } else {
            Ok(render_metrics.cell_size.height as f32)
        }
    }

    /// Cheap approximation of tab bar height that avoids the ~485ms cost of
    /// resolving the title font on macOS cold start (CoreText substitution
    /// lookup + HarfBuzz shaper init). Used only to compute initial window
    /// dimensions; the real height is computed lazily on first render via
    /// `tab_bar_pixel_height()`.
    pub fn estimated_tab_bar_pixel_height(
        config: &ConfigHandle,
        render_metrics: &RenderMetrics,
    ) -> f32 {
        if config.use_fancy_tab_bar {
            // Mirror tab_bar_pixel_height_impl's fancy-path formula, but use
            // the terminal cell height as a stand-in for the title font cell
            // height. The two differ by ~1-2 pixels in typical configs.
            (render_metrics.cell_size.height as f32 * 1.75).ceil()
        } else if config.tab_bar_at_bottom {
            render_metrics.natural_cell_height as f32
        } else {
            render_metrics.cell_size.height as f32
        }
    }

    pub fn tab_bar_pixel_height(&self) -> anyhow::Result<f32> {
        Self::tab_bar_pixel_height_impl(&self.config, &self.fonts, &self.render_metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tabbar::TabBarItem;
    use crate::termwindow::{UIItem, UIItemType};

    fn anchor(x: usize, y: usize) -> UIItem {
        UIItem {
            x,
            y,
            width: 120,
            height: 20,
            item_type: UIItemType::TabBar(TabBarItem::Tab {
                tab_idx: 0,
                active: true,
            }),
        }
    }

    #[test]
    fn pane_menu_opens_away_from_the_tab_bar() {
        assert_eq!(
            tab_pane_menu_origin(&anchor(40, 10), 160.0, 80.0, 800.0, 600.0, false),
            (40.0, 30.0)
        );
        assert_eq!(
            tab_pane_menu_origin(&anchor(40, 560), 160.0, 80.0, 800.0, 600.0, true),
            (40.0, 480.0)
        );
    }

    #[test]
    fn pane_menu_stays_inside_the_window() {
        assert_eq!(
            tab_pane_menu_origin(&anchor(760, 10), 160.0, 80.0, 800.0, 600.0, false),
            (640.0, 30.0)
        );
        assert_eq!(
            tab_pane_menu_origin(&anchor(10, 30), 160.0, 80.0, 800.0, 600.0, true),
            (10.0, 0.0)
        );
    }
}
