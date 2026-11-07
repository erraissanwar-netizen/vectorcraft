use super::{Sense, Stroke, Tokens, Ui, VectorcraftApp, icons, json, theme, vec2, widgets, zoom_label};

/// A document tab's title: "Name* @ 66.67 % (RGB/Preview)", or while an opacity mask is edited
/// "Name* @ 66.67 % (<Opacity Mask>/Opacity Mask)".
pub(super) fn tab_title(d: &vectorcraft_engine::DocState, zoom: f64, outline: bool, display_print_size: bool) -> String {
    let mode = if d.doc.mask_edit.is_some() {
        format!("<{0}>/{0}", tl!("Opacity Mask"))
    } else {
        let color = if d.doc.color_mode == vectorcraft_doc::ColorMode::Cmyk { "CMYK" } else { "RGB" };
        format!("{color}/{}", if outline { tl!("Outline") } else { tl!("Preview") })
    };
    format!("{}{} @ {} ({mode})", d.title(), if d.is_dirty() { "*" } else { "" }, zoom_label(zoom, display_print_size).replace('%', " %"))
}

/// Whether a document tab's × comes before its title (macOS) rather than after it (Windows,
/// Linux and the web).
pub(super) const CLOSE_BEFORE_TITLE: bool = cfg!(target_os = "macos");

/// The first of tabs `widths` wide that the strip shows in `room`: the first one, or a later one so
/// that the `active` tab fits.
pub(super) fn first_tab_shown(widths: &[f32], active: Option<usize>, room: f32) -> usize {
    let Some(a) = active else { return 0 };
    let mut first = 0;
    while first < a && widths.get(first..=a).map_or(0.0, |w| w.iter().sum::<f32>()) > room {
        first += 1;
    }
    first
}

/// Document tab strip: "Name* @ 66.67% (RGB/Preview)". User Interface › Large Tabs makes the tabs
/// taller, with larger titles. Tabs that don't fit are reached from a » button at the strip's
/// right end, which lists every open document, and the active tab is always in view (#746).
pub fn doc_tabs(app: &mut VectorcraftApp, ui: &mut Ui) {
    /// The width of the » button.
    const MORE: f32 = 30.0;
    let t = Tokens::get(ui.ctx());
    let (height, title_size) = if app.session.prefs.large_tabs { (44.0, 14.0) } else { (35.0, 12.5) };
    let (strip, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(strip, 0.0, t.tab_strip);
    ui.painter().line_segment([strip.left_bottom(), strip.right_bottom()], Stroke::new(1.0, t.border));
    // On the Home screen no tab is the current one.
    let active = app.session.active_index().filter(|_| app.ui.home.is_none());
    let tabs: Vec<(String, std::sync::Arc<egui::Galley>)> = app
        .session
        .documents()
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let zoom = app.views.get(i).map(|v| v.zoom).unwrap_or(1.0);
            let title = tab_title(d, zoom, app.ui.view.outline, app.session.prefs.display_print_size);
            let color = if Some(i) == active { t.text_strong } else { t.text_dim };
            (title.clone(), ui.painter().layout_no_wrap(title, theme::semibold(title_size), color))
        })
        .collect();
    let widths: Vec<f32> = tabs.iter().map(|(_, g)| g.size().x + 50.0).collect();
    let reserved_right = 195.0; // Reserve space for "Edit in Code Editor" button at extreme right
    let tab_area_width = (strip.width() - reserved_right).max(80.0);
    let overflow = widths.iter().sum::<f32>() > tab_area_width;
    let room = tab_area_width - if overflow { MORE } else { 0.0 };
    let first = if overflow { first_tab_shown(&widths, active, room) } else { 0 };
    let tab_gap = 6.0;
    let mut x = strip.left() + 6.0;
    let mut activate = None;
    let mut close = None;
    let mut hovering_art = false;
    for (i, ((title, _), w)) in tabs.iter().zip(&widths).enumerate().skip(first) {
        if x + w > strip.left() + room && Some(i) != active {
            break;
        }
        let is_active = Some(i) == active;
        let tab_box = egui::Rect::from_min_size(egui::pos2(x, strip.top() + 4.0), vec2(*w, strip.height() - 8.0));
        let resp = ui.interact(tab_box, ui.scope_id().with(("tab", i)), Sense::click());

        // Homepage tabs rising animation (0.35s ease-out)
        let anim = ui.ctx().animate_bool_with_time(ui.scope_id().with(("doc_tab_anim", i)), is_active, 0.35);

        // Inactive tab pill box styling: subtle border, rounded-md (6px), slight hover background
        if anim < 0.99 {
            let bg = if resp.hovered() { t.hover.gamma_multiply(0.4) } else { egui::Color32::TRANSPARENT };
            let border = if resp.hovered() { t.input_border } else { t.border };
            ui.painter().rect_filled(tab_box, egui::CornerRadius::same(6), bg);
            ui.painter().rect_stroke(tab_box, egui::CornerRadius::same(6), Stroke::new(1.0, border), egui::StrokeKind::Inside);
        }

        // Active tab pill box styling: rising accent gradient/fill (#ff5100) from bottom (y: 100% -> 0%)
        if anim > 0.005 {
            let fill_height = tab_box.height() * anim;
            let rising_rect = egui::Rect::from_min_max(egui::pos2(tab_box.left(), tab_box.bottom() - fill_height), tab_box.right_bottom());
            let clip_painter = ui.painter().with_clip_rect(tab_box);
            clip_painter.rect_filled(rising_rect, egui::CornerRadius::same(6), t.accent);
            if anim > 0.85 {
                clip_painter.rect_stroke(tab_box, egui::CornerRadius::same(6), Stroke::new(1.0, t.accent), egui::StrokeKind::Inside);
            }
        }

        // The × where each platform puts a tab's: before the title on macOS, after it elsewhere
        let (x_center, title_left) =
            if CLOSE_BEFORE_TITLE { (tab_box.left() + 14.0, tab_box.left() + 28.0) } else { (tab_box.right() - 14.0, tab_box.left() + 12.0) };
        let xr = egui::Rect::from_center_size(egui::pos2(x_center, tab_box.center().y), vec2(12.0, 12.0));
        let xresp = ui.interact(xr.expand(2.0), ui.scope_id().with(("tabx", i)), Sense::click());

        let x_color = if anim >= 0.5 {
            if xresp.hovered() { egui::Color32::from_white_alpha(200) } else { egui::Color32::WHITE }
        } else {
            if xresp.hovered() { t.text_strong } else { t.text_dim }
        };
        icons::paint(ui, "x", xr, x_color);

        // Tab title text (white when active, text_dim when inactive)
        let text_color = if anim >= 0.5 { egui::Color32::WHITE } else { t.text_dim };
        let title_galley = ui.painter().layout_no_wrap(title.clone(), theme::semibold(title_size), text_color);
        let text_max_right = if CLOSE_BEFORE_TITLE { tab_box.right() - 8.0 } else { xr.left() - 4.0 };
        let text_clip = egui::Rect::from_min_max(egui::pos2(title_left, tab_box.top()), egui::pos2(text_max_right.max(title_left), tab_box.bottom()));
        let text_painter = ui.painter().with_clip_rect(text_clip);
        text_painter.galley(egui::pos2(title_left, tab_box.center().y - title_galley.size().y / 2.0), title_galley, text_color);

        hovering_art |= crate::document_drag::hover_tab(app, &resp, i, xr.expand(2.0));
        if xresp.clicked() {
            close = Some(i);
        } else if resp.clicked() {
            activate = Some(i);
        }
        x += w + tab_gap;
    }
    crate::document_drag::finish_tabs(ui.ctx(), hovering_art);
    // The » button: every open document, the active one checked. The list scrolls once it
    // reaches the bottom of the window, so every document of a large session stays reachable
    // (#4017).
    if overflow {
        let more = egui::Rect::from_min_max(
            egui::pos2(strip.left() + tab_area_width - MORE, strip.top()),
            egui::pos2(strip.left() + tab_area_width, strip.bottom() - 1.0),
        );
        let resp = ui.interact(more, ui.scope_id().with("tabs-more"), Sense::click());
        ui.painter().rect_filled(more, 0.0, if resp.hovered() { t.hover.gamma_multiply(0.4) } else { t.tab_strip });
        icons::paint(ui, "chevrons-right", egui::Rect::from_center_size(more.center(), vec2(14.0, 14.0)), t.text);
        let resp = resp.on_hover_text(tl!("Show all open documents"));
        egui::Popup::menu(&resp).show(|ui| {
            widgets::menu_scroll(ui, |ui| {
                for (i, (title, _)) in tabs.iter().enumerate() {
                    if widgets::menu_item_name(ui, title, true, Some(i) == active) {
                        activate = Some(i);
                        ui.close();
                    }
                }
            });
        });
    }
    // "Edit in Code Editor" button at extreme right of document bar
    let edit_btn_w = 176.0;
    let edit_btn_h = 27.0;
    let edit_btn_rect = egui::Rect::from_min_size(
        egui::pos2(strip.right() - edit_btn_w - 8.0, strip.top() + (strip.height() - edit_btn_h) * 0.5),
        vec2(edit_btn_w, edit_btn_h),
    );
    let edit_btn_resp = ui.interact(edit_btn_rect, ui.scope_id().with("edit_in_code_btn"), Sense::click());
    if edit_btn_resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let bg_color =
        if edit_btn_resp.hovered() { t.hover } else { if t.dark { egui::Color32::from_rgb(0x18, 0x1b, 0x22) } else { egui::Color32::WHITE } };
    let border_color = if edit_btn_resp.hovered() { t.accent } else { t.border };
    ui.painter().rect_filled(edit_btn_rect, CornerRadius::same(6), bg_color);
    ui.painter().rect_stroke(edit_btn_rect, CornerRadius::same(6), Stroke::new(1.0, border_color), egui::StrokeKind::Inside);

    icons::paint(
        ui,
        "external-link",
        egui::Rect::from_center_size(edit_btn_rect.left_center() + vec2(14.0, 0.0), vec2(13.0, 13.0)),
        if edit_btn_resp.hovered() { t.accent } else { t.icon },
    );

    let edit_label = match crate::i18n::current().code() {
        "zh" | "zh-CN" | "zh-hant" | "cn" => "在代码编辑器中编辑",
        "es" => "Editar en el editor de código",
        "pt" | "pt-br" => "Editar no editor de código",
        "it" => "Modifica nell'editor di codice",
        "de" => "Im Code-Editor bearbeiten",
        "fr" => "Modifier dans l'éditeur de code",
        "nl" => "Bewerken in code-editor",
        "ru" => "Редактировать в редакторе кода",
        "ja" | "jp" => "コードエディタで編集",
        _ => "Edit in Code Editor",
    };
    let text_color = if edit_btn_resp.hovered() { t.accent } else { t.text };
    let text_galley = ui.painter().layout_no_wrap(edit_label.into(), theme::semibold(12.0), text_color);
    ui.painter().galley(egui::pos2(edit_btn_rect.left() + 26.0, edit_btn_rect.center().y - text_galley.size().y / 2.0), text_galley, text_color);

    if edit_btn_resp.clicked() {
        ui.ctx().open_url(egui::OpenUrl::same_tab("#navigate-code"));
    }

    if let Some(i) = close {
        if let Err(e) = crate::unsaved::close(app, i) {
            app.status(e);
        }
    } else if let Some(i) = activate {
        app.ui.home = None;
        let changed = app.session.active_index() != Some(i);
        if app.session.set_active(i)
            && changed
            && app.session.tool_id() == "artboard"
            && let Some(n) = app.session.active().map(|d| d.doc.artboards.len())
        {
            // A document switch recreates the tool; restore the displayed board as its target.
            // Clicking the current tab keeps its existing multiselection and tool options.
            let selected = crate::panels::artboards::selected(app, n);
            crate::panels::artboards::select(app, selected);
        }
    }
}

/// The isolation mode breadcrumb bar. It is drawn over the top of `area` (the canvas), after the
/// canvas, so entering isolation mode does not change the canvas's size (#3126).
pub fn isolation_bar(app: &mut VectorcraftApp, ui: &mut Ui, area: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    if let Some(st) = app.session.active()
        && let Some(iso) = st.isolation
    {
        let ancestry = st.doc.ancestry(iso).unwrap_or_default();
        struct Crumb {
            id: Option<vectorcraft_doc::NodeId>,
            is_layer: bool,
            name: String,
        }
        let mut crumbs: Vec<Crumb> = ancestry
            .iter()
            .filter_map(|id| st.doc.node(*id).map(|n| Crumb { id: Some(*id), is_layer: n.is_layer(), name: n.display_name() }))
            .collect();
        // Pattern editing mode: the pattern's name, and Save a Copy / Done / Cancel at the right.
        let pattern_edit = st.doc.pattern_edit.as_ref().map(|e| e.pattern.clone());
        if let Some(pn) = &pattern_edit {
            crumbs.push(Crumb { id: None, is_layer: false, name: pn.clone() });
        }
        let bar = egui::Rect::from_min_size(area.min, vec2(area.width(), 24.0));
        ui.interact(bar, ui.scope_id().with("iso-bar"), Sense::click_and_drag());
        ui.painter().rect_filled(bar, 0.0, t.panel);
        let back = egui::Rect::from_min_size(bar.min + vec2(6.0, 3.0), vec2(18.0, 18.0));
        let bresp = ui.interact(back, ui.scope_id().with("iso-back"), Sense::click());
        icons::paint(ui, "chevron-left", back, if bresp.hovered() { t.text } else { t.icon });

        let mut x = bar.left() + 30.0;
        let y_center = bar.center().y;
        let font_id = egui::FontId::proportional(12.0);
        let mut nav_cmd = None;
        let total = crumbs.len();
        for (i, crumb) in crumbs.iter().enumerate() {
            let is_leaf = i + 1 == total;
            let galley = ui.painter().layout_no_wrap(crumb.name.clone(), font_id.clone(), t.text);
            let w = galley.size().x;
            let text_pos = egui::pos2(x, y_center - galley.size().y * 0.5);
            if !is_leaf {
                let crumb_rect = egui::Rect::from_min_max(egui::pos2(x - 3.0, bar.top() + 3.0), egui::pos2(x + w + 3.0, bar.bottom() - 3.0));
                let resp = ui.interact(crumb_rect, ui.scope_id().with(("iso-crumb", i)), Sense::click());
                if resp.hovered() {
                    ui.painter().rect_filled(crumb_rect, 3.0, t.hover);
                }
                if resp.clicked() {
                    if crumb.is_layer {
                        nav_cmd = Some(("object.exitIsolation", json!({})));
                    } else if let Some(target) = crumb.id {
                        nav_cmd = Some(("object.isolate", json!({ "id": target.0 })));
                    }
                }
                ui.painter().galley(text_pos, galley, if resp.hovered() { t.text } else { t.text_dim });
                x += w + 6.0;
                let sep = ui.painter().layout_no_wrap("›".to_string(), font_id.clone(), t.text_dim);
                let sw = sep.size().x;
                ui.painter().galley(egui::pos2(x, y_center - sep.size().y * 0.5), sep, t.text_dim);
                x += sw + 6.0;
            } else {
                ui.painter().galley(text_pos, galley, t.text);
                x += w;
            }
        }
        let mut pattern_cmd = None;
        if pattern_edit.is_some() {
            let mut x = bar.right() - 6.0;
            for (label, cmd) in [("Cancel", "object.pattern.cancel"), ("Done", "object.pattern.done"), ("Save a Copy", "object.pattern.saveCopy")] {
                let label = tl!(label);
                let w = 12.0 + 7.0 * label.len() as f32;
                let r = egui::Rect::from_min_max(egui::pos2(x - w, bar.top() + 3.0), egui::pos2(x, bar.bottom() - 3.0));
                let resp = ui.interact(r, ui.scope_id().with(("pat-bar", cmd)), Sense::click());
                ui.painter().rect_filled(r, 3.0, if resp.hovered() { t.hover } else { t.panel });
                ui.painter().rect_stroke(r, 3.0, Stroke::new(1.0, t.button_border), egui::StrokeKind::Inside);
                ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(12.0), t.text);
                if resp.clicked() {
                    pattern_cmd = Some(cmd);
                }
                x -= w + 6.0;
            }
        }
        if bresp.clicked() {
            pattern_cmd = pattern_cmd.or(pattern_edit.as_ref().map(|_| "object.pattern.done"));
            if pattern_cmd.is_none() {
                // Up one level: the isolated object's parent, or out of isolation mode from a layer.
                nav_cmd = Some(match ancestry.iter().rev().nth(1).filter(|p| st.doc.node(**p).is_some_and(|n| !n.is_layer())) {
                    Some(parent) => ("object.isolate", json!({ "id": parent.0 })),
                    None => ("object.exitIsolation", json!({})),
                });
            }
        }
        if let Some((cmd, params)) = nav_cmd {
            app.run(cmd, params).ok();
        }
        if let Some(cmd) = pattern_cmd {
            app.run(cmd, json!({})).ok();
        }
    }
}

/// Draws the artboard navigator's field, which shows the active artboard (`cur`) by number and
/// name. The field's list shows every artboard by number and name, as the Artboards panel lists
/// them, and opens with the active artboard's row in view. The list lays out only the rows in view
/// and at most one more. Returns the artboard chosen in the list.
pub(super) fn artboard_list(app: &VectorcraftApp, ui: &mut Ui, cur: usize) -> Option<usize> {
    /// The width of the field.
    const FIELD_W: f32 = 140.0;
    /// The least width of the list.
    const LIST_W: f32 = 220.0;
    /// The height of a row, as in the Artboards panel.
    const ROW_H: f32 = 24.0;
    /// The rows the list shows at once.
    const ROWS: f32 = 10.0;
    let t = Tokens::get(ui.ctx());
    let boards = app.session.active().map(|d| d.doc.artboards.as_slice()).unwrap_or_default();
    let shown = boards.get(cur).map_or_else(|| "–".to_string(), |a| format!("{}  {}", cur + 1, a.name));
    // Under `drawn`, egui's temp data stores the pass the list was last drawn in.
    let drawn = ui.scope_id().with("artboard-list");
    let height = ROW_H * ROWS;
    ui.add_enabled_ui(!boards.is_empty(), |ui| {
        ui.set_max_width(FIELD_W);
        ui.name_inputs(tl!("Artboard"), |ui| {
            craft_ui::ComboBox::from_id_salt("artboard-list")
                .selected_text(egui::RichText::new(shown).size(11.5))
                .width(FIELD_W)
                .wrap_mode(egui::TextWrapMode::Truncate)
                .height(height)
                .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                .show_ui(ui, |ui| {
                    let pass = ui.ctx().cumulative_pass_nr();
                    let last = ui.data(|d| d.get_temp::<u64>(drawn)).unwrap_or_default();
                    ui.data_mut(|d| d.insert_temp(drawn, pass));
                    let mut area = egui::ScrollArea::vertical().max_height(height);
                    // As the list opens, it scrolls the active row as close to the middle as the list allows.
                    if last + 1 < pass || last == 0 {
                        area = area.vertical_scroll_offset((cur as f32 * ROW_H - (height - ROW_H) * 0.5).max(0.0));
                    }
                    // Every number ends where the widest one ends.
                    let numbers = ui.painter().layout_no_wrap(boards.len().to_string(), egui::FontId::proportional(12.0), t.text_dim).size().x;
                    let width = ui.available_width().max(LIST_W);
                    let mut chosen = None;
                    ui.spacing_mut().item_spacing.y = 0.0;
                    area.show_rows(ui, ROW_H, boards.len(), |ui, range| {
                        for i in range {
                            let Some(board) = boards.get(i) else { continue };
                            let (r, resp) = ui.allocate_exact_size(vec2(width, ROW_H), Sense::click());
                            if i == cur {
                                ui.painter().rect_filled(r, 0.0, t.row_selected);
                            } else if resp.hovered() {
                                ui.painter().rect_filled(r, 0.0, t.hover);
                            }
                            let number_end = r.left() + 8.0 + numbers;
                            ui.painter().text(
                                egui::pos2(number_end, r.center().y),
                                egui::Align2::RIGHT_CENTER,
                                (i + 1).to_string(),
                                egui::FontId::proportional(12.0),
                                t.text_dim,
                            );
                            let name_start = number_end + 10.0;
                            let name = egui::WidgetText::from(egui::RichText::new(&board.name).size(12.5)).into_galley(
                                ui,
                                Some(egui::TextWrapMode::Truncate),
                                r.right() - 8.0 - name_start,
                                egui::TextStyle::Body,
                            );
                            ui.painter().galley(egui::pos2(name_start, r.center().y - name.size().y / 2.0), name, t.text);
                            resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, i == cur, format!("{}  {}", i + 1, board.name)));
                            if resp.clicked() {
                                chosen = Some(i);
                            }
                        }
                    });
                    if chosen.is_some() {
                        ui.close();
                    }
                    chosen
                })
                .inner
                .flatten()
        })
    })
    .inner
}
