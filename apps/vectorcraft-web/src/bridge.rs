//! Web bridge connecting the host SVGCode Next.js application to VectorcraftApp via window.postMessage.

use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde_json::{Value, json};

/// Queue of incoming commands from SVGCode host waiting to be consumed by the app frame
#[derive(Default, Clone)]
pub struct BridgeInbox {
    pub commands: Arc<Mutex<Vec<Value>>>,
}

pub fn post_to_parent(msg: &Value) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(parent)) = window.parent() {
            let js_str = msg.to_string();
            let _ = parent.post_message(&JsValue::from_str(&js_str), "*");
        }
    }
}

pub fn notify_ready() {
    post_to_parent(&json!({
        "type": "vectorcraft:ready",
        "payload": {
            "version": "0.7.0",
            "capabilities": ["svg", "theme_injection", "undo_redo", "live_export"]
        }
    }));
}

/// Sets up the window.addEventListener("message", ...) bridge listener
pub fn start_listener(inbox: BridgeInbox, ctx: egui::Context) {
    let Some(window) = web_sys::window() else { return };

    let on_message = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |e: web_sys::MessageEvent| {
        let Ok(txt) = e.data().as_string().ok_or(()).or_else(|_| js_sys::JSON::stringify(&e.data()).map(|s| s.into())) else {
            return;
        };

        let Ok(parsed): Result<Value, _> = serde_json::from_str(&txt) else {
            return;
        };

        let msg_type = parsed.get("type").and_then(|v| v.as_str()).unwrap_or_default();

        match msg_type {
            "svgcode:set_theme" | "svgcode:init" => {
                if let Some(tokens_val) = parsed.pointer("/payload/tokens") {
                    let mut tokens = vectorcraft_ui_egui::theme::Tokens::get(&ctx);
                    tokens.update_from_svgcode_json(tokens_val);
                    vectorcraft_ui_egui::theme::apply_tokens(&ctx, tokens);
                    ctx.request_repaint();
                }
                if msg_type == "svgcode:init" {
                    if let Some(initial_svg) = parsed.pointer("/payload/initialSvg").and_then(|v| v.as_str()) {
                        if !initial_svg.is_empty() {
                            let mut queue = inbox.commands.lock().unwrap();
                            queue.push(json!({
                                "action": "load_svg",
                                "svg": initial_svg
                            }));
                            ctx.request_repaint();
                        }
                    }
                }
            }
            "svgcode:load_svg" => {
                if let Some(svg) = parsed.pointer("/payload/svg").and_then(|v| v.as_str()) {
                    let mut queue = inbox.commands.lock().unwrap();
                    queue.push(json!({
                        "action": "load_svg",
                        "svg": svg
                    }));
                    ctx.request_repaint();
                }
            }
            "svgcode:request_export" => {
                let req_id = parsed.pointer("/payload/requestId").and_then(|v| v.as_str()).unwrap_or("req");
                let fmt = parsed.pointer("/payload/format").and_then(|v| v.as_str()).unwrap_or("svg");
                let mut queue = inbox.commands.lock().unwrap();
                queue.push(json!({
                    "action": "export",
                    "requestId": req_id,
                    "format": fmt
                }));
                ctx.request_repaint();
            }
            "svgcode:undo" => {
                let mut queue = inbox.commands.lock().unwrap();
                queue.push(json!({ "action": "undo" }));
                ctx.request_repaint();
            }
            "svgcode:redo" => {
                let mut queue = inbox.commands.lock().unwrap();
                queue.push(json!({ "action": "redo" }));
                ctx.request_repaint();
            }
            "svgcode:zoom_fit" => {
                let mut queue = inbox.commands.lock().unwrap();
                queue.push(json!({ "action": "zoom_fit" }));
                ctx.request_repaint();
            }
            _ => {}
        }
    });

    if let Err(e) = window.add_event_listener_with_callback("message", on_message.as_ref().unchecked_ref()) {
        log::error!("vectorcraft-web: failed to install message listener: {e:?}");
    }

    on_message.forget();
}
