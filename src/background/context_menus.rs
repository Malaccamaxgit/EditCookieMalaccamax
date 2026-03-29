//! Context menu handling

use crate::chrome_api;
use crate::shared::helpers;
use js_sys::{Array, Function, Object, Reflect};
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

thread_local! {
    static INITIALIZED: Cell<bool> = const { Cell::new(false) };
}

/// Register the context menu item and its click listener.
/// Safe to call multiple times — the listener is only attached once.
async fn init_context_menus() {
    let create_props = Object::new();
    Reflect::set(
        &create_props,
        &JsValue::from("id"),
        &JsValue::from("edit_cookie_open_popup"),
    )
    .ok();
    Reflect::set(
        &create_props,
        &JsValue::from("title"),
        &JsValue::from("Edit cookies for this site"),
    )
    .ok();

    let contexts = Array::new();
    contexts.push(&JsValue::from("all"));
    Reflect::set(&create_props, &JsValue::from("contexts"), &contexts).ok();

    let _ = chrome_api::create_menu_item(create_props.into());

    if !INITIALIZED.with(|i| i.get()) {
        let listener = Closure::wrap(Box::new(move |info: JsValue| {
            wasm_bindgen_futures::spawn_local(async move {
                handle_menu_click(info).await;
            });
        }) as Box<dyn FnMut(JsValue)>);

        let on_clicked = chrome_api::on_context_menu_clicked();
        let js_func = listener.as_ref().clone().unchecked_into::<Function>();
        on_clicked.call1(&JsValue::NULL, &js_func).ok();

        listener.forget();
        INITIALIZED.with(|i| i.set(true));
    }

    oxichrome::log!("Context menus initialized");
}

/// Handle context menu clicks
async fn handle_menu_click(info: JsValue) {
    let menu_item_id = Reflect::get(&info, &JsValue::from("menuItemId"))
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_default();

    if menu_item_id != "edit_cookie_open_popup" {
        return;
    }

    let Some(tab_js) = helpers::get_active_tab().await else {
        return;
    };

    let Ok(id) = Reflect::get(&tab_js, &JsValue::from("id")) else {
        return;
    };
    if id.as_f64().is_none() {
        return;
    }

    let update_props = Object::new();
    Reflect::set(
        &update_props,
        &JsValue::from("highlighted"),
        &JsValue::from(true),
    )
    .ok();
    let _ = chrome_api::update(id, update_props.into()).await.ok();

    // Try chrome.action.openPopup()
    let action = js_sys::Reflect::get(&js_sys::global(), &JsValue::from("chrome"))
        .ok()
        .and_then(|chrome| Reflect::get(&chrome, &JsValue::from("action")).ok())
        .unwrap_or(JsValue::UNDEFINED);

    if let Some(open_popup_fn) = Reflect::get(&action, &JsValue::from("openPopup"))
        .ok()
        .and_then(|v| v.dyn_into::<Function>().ok())
    {
        let _ = open_popup_fn.call0(&action);
    } else {
        web_sys::console::log_1(
            &"Please click the extension icon to open the cookie editor".into(),
        );
    }
}

/// Update context menus based on preferences
pub async fn update_context_menus(show: bool) {
    if show {
        init_context_menus().await;
    } else {
        chrome_api::remove_menu_item(JsValue::from("edit_cookie_open_popup"));
    }
}
