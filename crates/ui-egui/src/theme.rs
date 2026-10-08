//! Design tokens (Illustrator's four UI brightness levels), fonts and egui style.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brightness {
    Dark,
    #[default]
    MediumDark,
    MediumLight,
    Light,
}

impl Brightness {
    pub const ALL: [Brightness; 4] = [Brightness::Dark, Brightness::MediumDark, Brightness::MediumLight, Brightness::Light];
    pub fn label(self) -> &'static str {
        match self {
            Brightness::Dark => "Dark",
            Brightness::MediumDark => "Medium Dark",
            Brightness::MediumLight => "Medium Light",
            Brightness::Light => "Light",
        }
    }
    pub fn id(self) -> &'static str {
        match self {
            Brightness::Dark => "dark",
            Brightness::MediumDark => "mediumDark",
            Brightness::MediumLight => "mediumLight",
            Brightness::Light => "light",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.id().eq_ignore_ascii_case(s) || b.label().eq_ignore_ascii_case(s))
    }
}

/// All colours and metrics the UI uses. Never hard-code a colour in a widget.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub dark: bool,
    pub app_bar: Color32,
    pub panel: Color32,
    pub panel_darker: Color32,
    pub input: Color32,
    pub input_border: Color32,
    pub border: Color32,
    pub divider: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub tool_active: Color32,
    pub accent: Color32,
    pub accent_strong: Color32,
    pub row_selected: Color32,
    pub pasteboard: Color32,
    pub ruler: Color32,
    pub ruler_tick: Color32,
    pub smart_guide: Color32,
    pub guide: Color32,
    pub measure_bg: Color32,
    pub button: Color32,
    pub radius: u8,
    /// Field values, button labels, link labels.
    pub text_strong: Color32,
    /// Panel tab strips, doc-tab bar, dock header.
    pub tab_strip: Color32,
    /// Outline of secondary (outlined) buttons.
    pub button_border: Color32,
    /// Default Layer 1 selection colour and selected-anchor fill.
    pub selection: Color32,
    pub anchor_selected: Color32,
    /// Active item text in tool flyouts.
    pub flyout_active: Color32,
    /// Hint-bar / tab-bar heights etc. scale with this base font size.
    pub font_size: f32,
    /// Window Close button (custom title bar) when hovered, and its glyph there.
    pub caption_close: Color32,
    pub caption_close_text: Color32,
    /// What preview images mark (Flattener Preview highlights).
    pub highlight: Color32,
    /// The bleed outline around artboards (Document Setup → Bleed).
    pub bleed: Color32,
    /// Error marks (a missing linked file).
    pub error: Color32,
    /// Warning marks (a modified linked file).
    pub warning: Color32,
}

impl Tokens {
    pub fn for_brightness(b: Brightness) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        let base = Tokens {
            dark: true,
            app_bar: hex(0x0a0a0a),
            panel: hex(0x0a0a0a),
            panel_darker: hex(0x111111),
            input: hex(0x0d0d0f),
            input_border: hex(0x27272a),
            border: hex(0x27272a),
            divider: hex(0x27272a),
            text: hex(0xfafafa),
            text_dim: hex(0xa1a1aa),
            text_disabled: hex(0x52525b),
            icon: hex(0xcfcfcf),
            hover: hex(0x1f1f23),
            tool_active: hex(0x111111),
            accent: hex(0xff5100),
            accent_strong: hex(0xca4000),
            row_selected: hex(0x27272a),
            pasteboard: hex(0x18181b),
            ruler: hex(0x111111),
            ruler_tick: hex(0x52525b),
            smart_guide: hex(0xff3dfc),
            guide: hex(0x4affff),
            measure_bg: Color32::from_rgba_unmultiplied(39, 39, 42, 235),
            button: hex(0x18181b),
            radius: 4,
            text_strong: hex(0xffffff),
            tab_strip: hex(0x111111),
            button_border: hex(0x27272a),
            selection: hex(0xff5100),
            anchor_selected: hex(0xff5100),
            flyout_active: hex(0xff5100),
            font_size: 13.0,
            caption_close: hex(0xc42b1c),
            caption_close_text: Color32::WHITE,
            highlight: hex(0xff5100),
            bleed: hex(0xf03030),
            error: hex(0xe34850),
            warning: hex(0xf0a330),
        };
        match b {
            Brightness::Dark => base,
            // Measured from Illustrator 2026 (plan/illustrator/10-observed-ui.md §1).
            Brightness::MediumDark => Tokens {
                app_bar: hex(0x535353),
                panel: hex(0x535353),
                panel_darker: hex(0x424242),
                tab_strip: hex(0x424242),
                input: hex(0x454545),
                input_border: hex(0x5f5f5f),
                border: hex(0x383838),
                divider: hex(0x4b4b4b),
                text: hex(0xd1d1d1),
                text_strong: hex(0xffffff),
                text_dim: hex(0xb0b0b0),
                text_disabled: hex(0x7a7a7a),
                icon: hex(0xc2c2c2),
                hover: hex(0x606060),
                tool_active: hex(0x303030),
                row_selected: hex(0x52677c),
                pasteboard: hex(0x606060),
                ruler: hex(0x333333),
                ruler_tick: hex(0x8a8a8a),
                button: hex(0x535353),
                button_border: hex(0x747474),
                ..base
            },
            Brightness::MediumLight => Tokens {
                tab_strip: hex(0xa6a6a6),
                text_strong: hex(0x000000),
                button_border: hex(0x7a7a7a),
                dark: false,
                app_bar: hex(0xa9a9a9),
                panel: hex(0xb8b8b8),
                panel_darker: hex(0xaaaaaa),
                input: hex(0xd6d6d6),
                input_border: hex(0x8c8c8c),
                border: hex(0x969696),
                divider: hex(0x9e9e9e),
                text: hex(0x1f1f1f),
                text_dim: hex(0x3c3c3c),
                text_disabled: hex(0x7c7c7c),
                icon: hex(0x2a2a2a),
                hover: hex(0xc8c8c8),
                tool_active: hex(0x969696),
                row_selected: hex(0x9fb4d6),
                pasteboard: hex(0xa0a0a0),
                ruler: hex(0xc2c2c2),
                ruler_tick: hex(0x4a4a4a),
                button: hex(0xcacaca),
                ..base
            },
            Brightness::Light => Tokens {
                tab_strip: hex(0xf5f3f0),
                text_strong: hex(0x0f172a),
                button_border: hex(0xe8e5e1),
                dark: false,
                app_bar: hex(0xfdfcfb),
                panel: hex(0xfaf7f4),
                panel_darker: hex(0xf5f3f0),
                input: hex(0xfdfdfd),
                input_border: hex(0xe8e5e1),
                border: hex(0xe8e5e1),
                divider: hex(0xe8e5e1),
                text: hex(0x0f172a),
                text_dim: hex(0x64748b),
                text_disabled: hex(0x94a3b8),
                icon: hex(0x0f172a),
                hover: hex(0xefece8),
                tool_active: hex(0xefece8),
                row_selected: hex(0xefece8),
                pasteboard: hex(0xe5e2dc),
                ruler: hex(0xf5f3f0),
                ruler_tick: hex(0x64748b),
                button: hex(0xf5f3f0),
                ..base
            },
        }
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::NULL)).unwrap_or_else(|| Tokens::for_brightness(Brightness::MediumDark))
    }
    pub fn cr(&self) -> CornerRadius {
        CornerRadius::same(self.radius)
    }

    /// Dynamically update colors from SVGCode design tokens passed across the postMessage bridge
    pub fn update_from_svgcode_json(&mut self, json: &serde_json::Value) {
        let hex = |val: &str| {
            let s = val.trim_start_matches('#');
            if let Ok(num) = u32::from_str_radix(s, 16) {
                Color32::from_rgb((num >> 16) as u8, (num >> 8) as u8, num as u8)
            } else {
                Color32::WHITE
            }
        };

        if let Some(accent) = json.get("accent").and_then(|v| v.as_str()) {
            self.accent = hex(accent);
        }
        if let Some(accent_strong) = json.get("accentStrong").and_then(|v| v.as_str()) {
            self.accent_strong = hex(accent_strong);
        }
        if let Some(app_bar) = json.get("appBar").and_then(|v| v.as_str()) {
            self.app_bar = hex(app_bar);
        }
        if let Some(panel) = json.get("panel").and_then(|v| v.as_str()) {
            self.panel = hex(panel);
        }
        if let Some(panel_darker) = json.get("panelDarker").and_then(|v| v.as_str()) {
            self.panel_darker = hex(panel_darker);
        }
        if let Some(tab_strip) = json.get("tabStrip").and_then(|v| v.as_str()) {
            self.tab_strip = hex(tab_strip);
        }
        if let Some(input) = json.get("input").and_then(|v| v.as_str()) {
            self.input = hex(input);
        }
        if let Some(input_border) = json.get("inputBorder").and_then(|v| v.as_str()) {
            self.input_border = hex(input_border);
        }
        if let Some(border) = json.get("border").and_then(|v| v.as_str()) {
            self.border = hex(border);
        }
        if let Some(divider) = json.get("divider").and_then(|v| v.as_str()) {
            self.divider = hex(divider);
        }
        if let Some(text) = json.get("text").and_then(|v| v.as_str()) {
            self.text = hex(text);
        }
        if let Some(text_strong) = json.get("textStrong").and_then(|v| v.as_str()) {
            self.text_strong = hex(text_strong);
        }
        if let Some(text_dim) = json.get("textDim").and_then(|v| v.as_str()) {
            self.text_dim = hex(text_dim);
        }
        if let Some(hover) = json.get("hover").and_then(|v| v.as_str()) {
            self.hover = hex(hover);
        }
        if let Some(pasteboard) = json.get("pasteboard").and_then(|v| v.as_str()) {
            self.pasteboard = hex(pasteboard);
        }
    }
}

pub const FONT_UI: &str = "ui";
pub const FONT_UI_SEMIBOLD: &str = "ui-semibold";
pub const FONT_MONO: &str = "mono";

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_UI_SEMIBOLD.into()))
}
pub fn mono(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_MONO.into()))
}

pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let add = |f: &mut FontDefinitions, name: &str, bytes: &'static [u8]| {
        f.font_data.insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
    };
    add(&mut fonts, "Inter", include_bytes!("../../../assets/fonts/Inter-Regular.ttf"));
    add(&mut fonts, "Inter-Semibold", include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf"));
    add(&mut fonts, "SourceSans3", include_bytes!("../../../assets/fonts/SourceSans3-Regular.ttf"));
    add(&mut fonts, "SourceSans3-Semibold", include_bytes!("../../../assets/fonts/SourceSans3-Semibold.ttf"));
    add(&mut fonts, "JetBrainsMono", include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf"));
    let fallback: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let mut prop = vec!["Inter".to_string(), "SourceSans3".to_string()];
    prop.extend(fallback.clone());
    fonts.families.insert(FontFamily::Proportional, prop);
    let mut semi = vec!["Inter-Semibold".to_string(), "SourceSans3-Semibold".to_string()];
    semi.extend(fallback.clone());
    fonts.families.insert(FontFamily::Name(FONT_UI_SEMIBOLD.into()), semi);
    let mut ui = vec!["Inter".to_string(), "SourceSans3".to_string()];
    ui.extend(fallback.clone());
    fonts.families.insert(FontFamily::Name(FONT_UI.into()), ui);
    let mut mono = vec!["JetBrainsMono".to_string()];
    mono.extend(fallback);
    fonts.families.insert(FontFamily::Name(FONT_MONO.into()), mono);
    add_craft_fonts(&mut fonts);
    ctx.set_fonts(fonts);
}

/// The egui name of a craft-fonts face.
pub(crate) fn craft_font_name(f: &vectorcraft_text::CraftFont) -> String {
    format!("craft-fonts {} {}", f.family, f.style)
}

/// The Japanese craft-fonts faces (when built with `CRAFT_FONTS_DIR`) as fallbacks at the end of
/// every family, after the app's own fonts and before the installed fonts `ui_fonts` adds: BIZ
/// UDPGothic first (bold first in the semibold family), then the Mincho faces.
fn add_craft_fonts(fonts: &mut FontDefinitions) {
    for f in vectorcraft_text::CRAFT_FONTS.iter().filter(|f| f.is_japanese()) {
        fonts.font_data.insert(craft_font_name(f), Arc::new(FontData::from_static(f.bytes)));
    }
    for (family, stack) in fonts.families.iter_mut() {
        let bold = *family == FontFamily::Name(FONT_UI_SEMIBOLD.into());
        stack.extend(vectorcraft_text::craft_fonts::japanese_ui_fonts(bold).into_iter().map(craft_font_name));
    }
}

/// Apply tokens to egui's global style from Brightness enum.
pub fn apply(ctx: &egui::Context, b: Brightness) {
    apply_tokens(ctx, Tokens::for_brightness(b));
}

/// Apply tokens directly to egui's global style (used for SVGCode runtime token injection).
pub fn apply_tokens(ctx: &egui::Context, t: Tokens) {
    ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, t));
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.extreme_bg_color = t.input;
    v.faint_bg_color = t.panel_darker;
    v.window_stroke = Stroke::new(1.0, t.border);
    v.window_corner_radius = CornerRadius::same(6);
    v.menu_corner_radius = CornerRadius::same(6);
    v.selection.bg_fill = t.accent.gamma_multiply(0.55);
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.hyperlink_color = t.accent;
    v.override_text_color = Some(t.text);
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 14, spread: 0, color: Color32::from_black_alpha(if t.dark { 120 } else { 60 }) };
    v.window_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 28, spread: 0, color: Color32::from_black_alpha(if t.dark { 140 } else { 70 }) };
    let w = &mut v.widgets;
    for (wv, fill) in [
        (&mut w.noninteractive, t.panel),
        (&mut w.inactive, t.button),
        (&mut w.hovered, t.hover),
        (&mut w.active, t.tool_active),
        (&mut w.open, t.hover),
    ] {
        wv.bg_fill = fill;
        wv.weak_bg_fill = fill;
        wv.corner_radius = t.cr();
        wv.fg_stroke = Stroke::new(1.0, t.text);
    }
    w.noninteractive.bg_stroke = Stroke::new(1.0, t.divider);
    w.inactive.bg_stroke = Stroke::NONE;
    w.hovered.bg_stroke = Stroke::new(1.0, t.input_border);
    w.active.bg_stroke = Stroke::new(1.0, t.accent);
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 5.0);
        s.spacing.button_padding = egui::vec2(6.0, 2.0);
        s.spacing.interact_size = egui::vec2(24.0, 24.0);
        s.spacing.menu_margin = egui::Margin::same(6);
        s.spacing.slider_width = 120.0;
        s.text_styles = [
            (TextStyle::Small, FontId::proportional(11.0)),
            (TextStyle::Body, FontId::proportional(13.0)),
            (TextStyle::Button, FontId::proportional(13.0)),
            (TextStyle::Heading, FontId::new(15.0, FontFamily::Name(FONT_UI_SEMIBOLD.into()))),
            (TextStyle::Monospace, FontId::new(11.5, FontFamily::Name(FONT_MONO.into()))),
        ]
        .into();
        s.animation_time = 0.08;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_roundtrip() {
        for b in Brightness::ALL {
            assert_eq!(Brightness::parse(b.id()), Some(b));
            assert_eq!(Brightness::parse(b.label()), Some(b));
        }
        assert!(!Tokens::for_brightness(Brightness::Light).dark);
    }
}
