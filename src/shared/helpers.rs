//! Shared helper functions used across popup, options, background, etc.

use js_sys::{Object, Reflect};
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

thread_local! {
    static STYLESHEETS_INJECTED: Cell<bool> = const { Cell::new(false) };
}

/// Resolve a relative path to an absolute `chrome-extension://` URL via
/// `chrome.runtime.getURL()`.  Falls back to the original path if the
/// Chrome API is unavailable (e.g. unit-test / HTTP context).
fn resolve_extension_url(path: &str) -> String {
    let Ok(chrome) = Reflect::get(&js_sys::global(), &JsValue::from("chrome")) else {
        return path.to_string();
    };
    let Ok(runtime) = Reflect::get(&chrome, &JsValue::from("runtime")) else {
        return path.to_string();
    };
    let Ok(get_url) = Reflect::get(&runtime, &JsValue::from("getURL")) else {
        return path.to_string();
    };
    let Ok(func) = get_url.dyn_into::<js_sys::Function>() else {
        return path.to_string();
    };
    func.call1(&JsValue::UNDEFINED, &JsValue::from(path))
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| path.to_string())
}

/// Inject CSS `<link>` elements into `<head>` for pages where Oxichrome's
/// generated HTML doesn't include them (popup, options).
///
/// Each page CSS already has `@import "fontawesome.min.css"` so the FA
/// stylesheet loads automatically.  We also inject the FA link as a
/// fallback in case the @import is stripped or delayed.
pub fn ensure_stylesheets(css_path: &str) {
    if STYLESHEETS_INJECTED.with(|s| s.get()) {
        return;
    }
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let Some(head) = document.head() else { return };

    for relative in &[css_path, "css/fontawesome.min.css"] {
        let href = resolve_extension_url(relative);
        if let Ok(link) = document.create_element("link") {
            link.set_attribute("rel", "stylesheet").ok();
            link.set_attribute("href", &href).ok();
            head.append_child(&link).ok();
        }
    }

    STYLESHEETS_INJECTED.with(|s| s.set(true));
}

/// Query the active tab in the current window and return the raw JsValue tab object.
pub async fn get_active_tab() -> Option<JsValue> {
    let query_info = Object::new();
    Reflect::set(&query_info, &JsValue::from("active"), &JsValue::from(true)).ok();
    Reflect::set(
        &query_info,
        &JsValue::from("currentWindow"),
        &JsValue::from(true),
    )
    .ok();

    let tabs = crate::chrome_api::query(query_info.into()).await.ok()?;
    let array: js_sys::Array = tabs.dyn_into().ok()?;
    let tab = array.get(0);
    if tab.is_undefined() || tab.is_null() {
        return None;
    }
    Some(tab)
}

/// Get the URL of the currently active tab.
pub async fn get_current_tab_url() -> Option<String> {
    let tab = get_active_tab().await?;
    Reflect::get(&tab, &JsValue::from("url"))
        .ok()
        .and_then(|v| v.as_string())
}

/// Apply a theme class to the document body.
pub fn apply_theme(theme: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(body) = document.body() else { return };
    let class_list = body.class_list();

    match theme {
        "dark" => {
            class_list.add_1("dark-theme").ok();
            class_list.remove_1("light-theme").ok();
        }
        _ => {
            class_list.add_1("light-theme").ok();
            class_list.remove_1("dark-theme").ok();
        }
    }
}
