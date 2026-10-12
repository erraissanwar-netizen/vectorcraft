//! Help → About SVGCode.

use crate::VectorcraftApp;
use crate::theme::{self, Tokens};

/// The About window, while `UiState::about` is set.
pub(super) fn show(app: &mut VectorcraftApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let mut open = true;
    egui::Window::new(tl!("About SVGCode"))
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .frame(egui::Frame::window(&ctx.global_style()).inner_margin(egui::Margin::same(18)))
        .show(ctx, |ui| {
            // Tabs: About · Contributors · Models (the credits are compiled in, see `credits`).
            let tab_id = egui::Id::new("about_tab");
            let mut tab = ui.data_mut(|d| d.get_temp::<u8>(tab_id)).unwrap_or(0);
            ui.horizontal(|ui| {
                for (i, label) in [tl!("About"), tl!("Contributors"), tl!("Models")].into_iter().enumerate() {
                    let i = i as u8;
                    if ui.selectable_label(tab == i, label).clicked() {
                        tab = i;
                    }
                }
            });
            ui.data_mut(|d| d.insert_temp(tab_id, tab));
            ui.separator();
            match tab {
                1 => {
                    ui.set_width(660.0);
                    crate::credits::contributors_ui(ui);
                }
                2 => {
                    ui.set_width(660.0);
                    crate::credits::models_ui(ui);
                }
                _ => about_tab(app, ui),
            }
        });
    app.ui.about = open;
}

/// The About tab: mark, version, owner credits, licences and the GPU in use.
fn about_tab(app: &mut VectorcraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.set_width(380.0);
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(egui::vec2(64.0, 32.0), egui::Sense::hover());
        crate::brand::paint_mark(ui, r);
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("SVGCode").font(theme::semibold(22.0)).color(t.text));
            ui.label(
                egui::RichText::new(crate::i18n::fmt(
                    tl!("Version {version} — Professional SVG Vector Editor"),
                    &[("version", env!("CARGO_PKG_VERSION"))],
                ))
                .size(12.0)
                .color(t.text_dim),
            );
        });
    });
    ui.add_space(14.0);
    ui.label(
        egui::RichText::new(tl!("Created and maintained by SVGCode (https://svgcode.com)"))
            .font(theme::semibold(13.0))
            .color(t.text),
    );
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(tl!(
            "An open, browser-native vector design and SVG code workspace created to make vector editing fast, private, and accessible to everyone 100% in your browser."
        ))
        .size(11.5)
        .color(t.text_dim),
    );
    ui.add_space(10.0);
    ui.label(
        egui::RichText::new(tl!(
            "Engine: Powered by WebAssembly & Rust · MIT / Apache-2.0 · Special thanks to the ArtCraft open-source community."
        ))
        .size(10.5)
        .color(t.text_dim),
    );
    // The GPU the window renders with, for bug reports (Preferences › Performance picks it).
    if let Some(adapter) = &app.graphics_adapter {
        ui.add_space(8.0);
        ui.label(egui::RichText::new(crate::i18n::fmt(tl!("Graphics: {adapter}"), &[("adapter", adapter.as_str())])).size(11.0).color(t.text_dim));
    }
}
