//! DevTools panel module.
//!
//! Oxichrome mounts `App` from `panel.html`, so this module is not referenced from `lib.rs` and appears unused to rustc.

#![allow(dead_code)]

use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn App() -> impl IntoView {
    use crate::core::cookie;
    use crate::shared::types::Cookie;

    let (cookies, set_cookies) = signal(Vec::<Cookie>::new());
    let (sort_field, set_sort_field) = signal(String::from("domain"));
    let (sort_desc, set_sort_desc) = signal(false);

    // Load cookies on mount
    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            match cookie::get_all().await {
                Ok(cookie_list) => set_cookies.set(cookie_list),
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to load cookies: {:?}", e).into())
                }
            }
        });
    });

    // Sort cookies
    let sorted_cookies = move || {
        let mut cookies = cookies.get();
        let field = sort_field.get();
        let desc = sort_desc.get();

        cookies.sort_by(|a, b| {
            let cmp = match field.as_str() {
                "domain" => a.domain.cmp(&b.domain),
                "name" => a.name.cmp(&b.name),
                "path" => a.path.cmp(&b.path),
                "expiration" => a
                    .expiration_date
                    .partial_cmp(&b.expiration_date)
                    .unwrap_or(std::cmp::Ordering::Equal),
                _ => std::cmp::Ordering::Equal,
            };
            if desc {
                cmp.reverse()
            } else {
                cmp
            }
        });

        cookies
    };

    view! {
        <div class="devtools-panel">
            <div class="panel-header">
                <h2>Cookie Editor</h2>
                <div class="sort-controls">
                    <label>Sort by:
                        <select
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(select) = target.dyn_ref::<web_sys::HtmlSelectElement>()
                                {
                                    set_sort_field.set(select.value());
                                }
                            }
                        >
                            <option value="domain">Domain</option>
                            <option value="name">Name</option>
                            <option value="path">Path</option>
                            <option value="expiration">Expiration</option>
                        </select>
                    </label>
                    <button
                        on:click=move |_| set_sort_desc.set(!sort_desc.get())
                        title="Toggle sort order"
                    >
                        <i class="fas" class:fa-sort-asc=move || !sort_desc.get() class:fa-sort-desc=move || sort_desc.get()></i>
                    </button>
                </div>
            </div>
            <div class="cookie-table-wrapper">
                <table class="cookie-table">
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Value</th>
                            <th>Domain</th>
                            <th>Path</th>
                            <th>Expires</th>
                            <th>Secure</th>
                            <th>HttpOnly</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=sorted_cookies
                            key=|cookie| format!("{}::{}", cookie.domain, cookie.name)
                            children=move |cookie| {
                                view! {
                                    <tr>
                                        <td>{cookie.name.clone()}</td>
                                        <td class="value-cell">{cookie.value.clone()}</td>
                                        <td>{cookie.domain.clone()}</td>
                                        <td>{cookie.path.clone().unwrap_or_default()}</td>
                                        <td>
                                            {cookie.expiration_date.map(format_timestamp).unwrap_or_else(|| "Session".to_string())}
                                        </td>
                                        <td>
                                            <i class="fas" class:fa-check=move || cookie.secure class:fa-times=move || !cookie.secure></i>
                                        </td>
                                        <td>
                                            <i class="fas" class:fa-check=move || cookie.http_only class:fa-times=move || !cookie.http_only></i>
                                        </td>
                                        <td>
                                            <button class="btn-icon" title="Edit">
                                                <i class="fas fa-edit"></i>
                                            </button>
                                            <button class="btn-icon" title="Delete">
                                                <i class="fas fa-trash"></i>
                                            </button>
                                        </td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>
        </div>
    }
}

fn format_timestamp(ts: f64) -> String {
    use js_sys::Date;
    let date = Date::new(&wasm_bindgen::JsValue::from_f64(ts * 1000.0));
    let iso = date.to_iso_string();
    iso.as_string().unwrap_or_default()
}

