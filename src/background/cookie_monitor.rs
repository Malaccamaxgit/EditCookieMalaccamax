//! Cookie change monitoring and rule enforcement

use crate::chrome_api;
use crate::core::{cookie, rules};
use js_sys::Function;
use std::cell::Cell;
use wasm_bindgen::prelude::*;

thread_local! {
    /// Re-entrancy guard: set while we are restoring/removing a cookie
    /// so the resulting onChanged event is ignored.
    static SUPPRESSED: Cell<bool> = const { Cell::new(false) };
}

/// RAII guard that sets SUPPRESSED to `true` on creation and back to `false`
/// on drop — even if a panic unwinds through an await point.
struct SuppressGuard;

impl SuppressGuard {
    fn new() -> Self {
        SUPPRESSED.with(|s| s.set(true));
        SuppressGuard
    }
}

impl Drop for SuppressGuard {
    fn drop(&mut self) {
        SUPPRESSED.with(|s| s.set(false));
    }
}

/// Initialize the cookie change listener
pub async fn init_cookie_monitor() {
    let listener = Closure::wrap(Box::new(move |change_info: JsValue| {
        if SUPPRESSED.with(|s| s.get()) {
            return;
        }
        wasm_bindgen_futures::spawn_local(async move {
            handle_cookie_change(change_info).await;
        });
    }) as Box<dyn FnMut(JsValue)>);

    let on_changed = chrome_api::on_cookie_changed();
    let js_func = listener.as_ref().clone().unchecked_into::<Function>();
    on_changed.call1(&JsValue::NULL, &js_func).ok();

    listener.forget();

    oxichrome::log!("Cookie monitor initialized");
}

/// Handle cookie change events
async fn handle_cookie_change(change_info: JsValue) {
    use js_sys::Reflect;

    let cookie_obj = match Reflect::get(&change_info, &JsValue::from("cookie")) {
        Ok(val) => val,
        Err(_) => return,
    };

    let removed = Reflect::get(&change_info, &JsValue::from("removed"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let Some(cookie) = chrome_api::js_value_to_cookie(cookie_obj) else {
        return;
    };

    if removed {
        if rules::is_read_only(&cookie).await {
            oxichrome::log!("Restoring read-only cookie: {} on {}", cookie.name, cookie.domain);
            let _guard = SuppressGuard::new();
            if let Err(e) = cookie::set(&cookie).await {
                oxichrome::log!("Failed to restore cookie: {:?}", e);
            }
        }
    } else {
        if rules::matches_filter(&cookie).await {
            oxichrome::log!("Blocking filtered cookie: {} on {}", cookie.name, cookie.domain);
            let _guard = SuppressGuard::new();
            if let Err(e) = cookie::remove(&cookie).await {
                oxichrome::log!("Failed to remove filtered cookie: {:?}", e);
            }
        }

        if let Some(shortened) = rules::maybe_shorten(&cookie).await
            && shortened != cookie
        {
            let _guard = SuppressGuard::new();
            let _ = cookie::set(&shortened).await;
        }
    }
}
