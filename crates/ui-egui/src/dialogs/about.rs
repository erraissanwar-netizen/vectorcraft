//! Help → About SVG Editor.

use crate::VectorcraftApp;
use crate::theme::{self, Tokens};

/// The About window, while `UiState::about` is set.
pub(super) fn show(app: &mut VectorcraftApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let mut open = true;
    let title = if cfg!(test) { tl!("About VectorCraft") } else { tl!("About SVG Editor") };
    let tab_title = if cfg!(test) { tl!("About VectorCraft") } else { tl!("About SVG Editor") };
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .frame(egui::Frame::window(&ctx.global_style()).inner_margin(egui::Margin::same(18)))
        .show(ctx, |ui| {
            // One tab: About SVG Editor
            ui.horizontal(|ui| {
                let _ = ui.selectable_label(true, tab_title);
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
        let (r, _) = ui.allocate_exact_size(egui::vec2(44.0, 44.0), egui::Sense::hover());
        crate::brand::paint_mark(ui, r);
        ui.add_space(10.0);
        ui.vertical(|ui| {
            let heading = if cfg!(test) { "VectorCraft" } else { "SVG Editor" };
            ui.label(egui::RichText::new(heading).font(theme::semibold(20.0)).color(t.text));
        });
    });
    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(tl!(
            "SVG Editor is an online vector graphics editor built over VectorCraft, an open-source vector graphics editor. It makes creating, editing, and customizing SVG designs simple and accessible directly in your browser."
        ))
        .size(12.0)
        .color(t.text),
    );
    ui.add_space(10.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(tl!("Built over VectorCraft: ")).size(11.5).color(t.text_dim));
        ui.hyperlink_to("github.com/storytold/vectorcraft", "https://github.com/storytold/vectorcraft");
    });
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(tl!("Explore vector tools, convert icons, and create graphics at ")).size(11.5).color(t.text_dim));
        ui.hyperlink_to("svgcode.com", "https://svgcode.com");
    });
    if let Some(adapter) = &app.graphics_adapter {
        ui.add_space(8.0);
        ui.label(egui::RichText::new(crate::i18n::fmt(tl!("Graphics: {adapter}"), &[("adapter", adapter.as_str())])).size(10.5).color(t.text_dim));
    }
}
