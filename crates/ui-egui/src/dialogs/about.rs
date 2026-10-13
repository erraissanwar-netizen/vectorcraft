//! Help → About SVGCode Editor.

use crate::VectorcraftApp;
use crate::theme::{self, Tokens};

/// The About window, while `UiState::about` is set.
pub(super) fn show(app: &mut VectorcraftApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let mut open = true;
    egui::Window::new(tl!("About SVGCode Editor"))
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .frame(egui::Frame::window(&ctx.global_style()).inner_margin(egui::Margin::same(18)))
        .show(ctx, |ui| {
            // One tab: About SVGCode Editor
            ui.horizontal(|ui| {
                ui.selectable_label(true, tl!("About SVGCode Editor"));
            });
            ui.separator();
            about_tab(app, ui);
        });
    app.ui.about = open;
}

/// The About tab: mark, description, and link to VectorCraft.
fn about_tab(app: &mut VectorcraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.set_width(420.0);
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(egui::vec2(64.0, 32.0), egui::Sense::hover());
        crate::brand::paint_mark(ui, r);
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("SVGCode Editor").font(theme::semibold(20.0)).color(t.text));
        });
    });
    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(tl!(
            "SVGCode Editor is a free online vector graphics editor that makes creating, editing, and customizing SVG designs simple and accessible. From drawing shapes and editing paths to designing icons, logos, and illustrations, our goal is to provide a powerful SVG editing experience directly in your browser."
        ))
        .size(12.0)
        .color(t.text),
    );
    ui.add_space(10.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new(tl!("SVGCode Editor is built on the open-source "))
                .size(11.5)
                .color(t.text_dim),
        );
        ui.hyperlink_to("VectorCraft", "https://github.com/storytold/vectorcraft");
        ui.label(
            egui::RichText::new(tl!(" project, customized and enhanced for the SVGCode platform."))
                .size(11.5)
                .color(t.text_dim),
        );
    });
    if let Some(adapter) = &app.graphics_adapter {
        ui.add_space(8.0);
        ui.label(egui::RichText::new(crate::i18n::fmt(tl!("Graphics: {adapter}"), &[("adapter", adapter.as_str())])).size(10.5).color(t.text_dim));
    }
}
