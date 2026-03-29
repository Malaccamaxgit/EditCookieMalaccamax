//! Cookie editor component (not yet mounted in the popup; kept for reuse / Oxichrome exports).
#![allow(dead_code)]

use crate::shared::types::Cookie;
use leptos::prelude::*;

#[component]
pub fn CookieEditor(#[prop(optional)] cookie: Option<Cookie>) -> impl IntoView {
    let is_edit = cookie.is_some();
    let default_cookie = Cookie {
        name: String::new(),
        value: String::new(),
        domain: String::new(),
        path: Some("/".to_string()),
        expiration_date: None,
        http_only: false,
        secure: false,
        same_site: None,
        store_id: None,
    };

    let cookie = cookie.unwrap_or(default_cookie);

    let (_name, set_name) = signal(cookie.name);
    let (_value, set_value) = signal(cookie.value);
    let (_domain, set_domain) = signal(cookie.domain);
    let (_path, set_path) = signal(cookie.path.unwrap_or_else(|| "/".to_string()));
    let (secure, set_secure) = signal(cookie.secure);
    let (http_only, set_http_only) = signal(cookie.http_only);

    view! {
        <div id="newCookie" class="cookie-editor">
            <h3>{if is_edit { "Edit Cookie" } else { "New Cookie" }}</h3>
            <div class="form-row">
                <label>Name:</label>
                <input
                    type="text"
                    id="cookieName"
                    on:input=move |e| set_name.set(event_target_value(&e))
                />
            </div>
            <div class="form-row">
                <label>Value:</label>
                <textarea
                    id="cookieValue"
                    on:input=move |e| set_value.set(event_target_value(&e))
                />
            </div>
            <div class="form-row">
                <label>Domain:</label>
                <input
                    type="text"
                    id="cookieDomain"
                    on:input=move |e| set_domain.set(event_target_value(&e))
                />
            </div>
            <div class="form-row">
                <label>Path:</label>
                <input
                    type="text"
                    id="cookiePath"
                    on:input=move |e| set_path.set(event_target_value(&e))
                />
            </div>
            <div class="form-row">
                <label>
                    <input
                        type="checkbox"
                        id="cookieSecure"
                        checked=secure.get()
                        on:change=move |e| set_secure.set(event_target_checked(&e))
                    />
                    Secure
                </label>
                <label>
                    <input
                        type="checkbox"
                        id="cookieHttpOnly"
                        checked=http_only.get()
                        on:change=move |e| set_http_only.set(event_target_checked(&e))
                    />
                    HttpOnly
                </label>
            </div>
            <div class="form-actions">
                <button class="btn btn-primary" id="submitCookieBtn">
                    <i class="fas fa-save"></i>
                    {if is_edit { "Update" } else { "Add"}}
                </button>
                <button class="btn btn-secondary" id="cancelCookieBtn">
                    <i class="fas fa-times"></i>
                    Cancel
                </button>
            </div>
        </div>
    }
}
