//! Cookie row / detail components used by the popup cookie list.

use std::collections::HashMap;

use crate::shared::types::Cookie;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn CookieRow(
    cookie: Cookie,
    current_url: String,
    on_cookie_changed: Callback<()>,
    show_domain: Signal<bool>,
    show_domain_before_name: Signal<bool>,
    show_alerts: Signal<bool>,
    show_command_labels: Signal<bool>,
    custom_descriptions: ReadSignal<HashMap<String, String>>,
    on_desc_changed: Callback<(String, Option<String>)>,
    #[prop(default = false)] initially_expanded: bool,
) -> impl IntoView {
    let (expanded, set_expanded) = signal(initially_expanded);
    let (is_protected, set_is_protected) = signal(false);
    let name = cookie.name.clone();
    let domain = cookie.domain.clone();
    let is_secure = cookie.secure;
    let is_http_only = cookie.http_only;
    let is_host_only = !cookie.domain.starts_with('.');
    let expiry = cookie.expiration_date;

    {
        let cookie_name = cookie.name.clone();
        let cookie_domain = cookie.domain.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let prefs = crate::core::storage::PreferencesStorage::get().await;
            let protected = prefs
                .protected_cookies
                .iter()
                .any(|c| c.name == cookie_name && c.domain == cookie_domain);
            set_is_protected.set(protected);
        });
    }

    let desc_name = cookie.name.clone();
    let description = Signal::derive(move || {
        let custom = custom_descriptions.get();
        if let Some(d) = custom.get(&desc_name) {
            return Some(d.clone());
        }
        crate::shared::known_cookies::lookup(&desc_name).map(|s| s.to_string())
    });

    let expiry_label = format_relative_expiry(expiry);
    let expiry_title: &'static str = if expiry.is_some() { "Expiration" } else { "Session cookie" };

    view! {
        <div class="cookie-row" class:cookie-row-protected=is_protected>
            <div class="cookie-header" on:click=move |_| set_expanded.set(!expanded.get())>
                {
                    let domain_before = domain.clone();
                    view! {
                        <Show when=move || show_domain.get() && show_domain_before_name.get()>
                            <span class="cookie-domain cookie-domain-before">{domain_before.clone()}</span>
                        </Show>
                    }
                }
                <div class="cookie-name-group">
                    <span class="cookie-name">{name.clone()}</span>
                    <Show when=move || description.get().is_some()>
                        <span class="cookie-description">{move || description.get().unwrap_or_default()}</span>
                    </Show>
                </div>
                <span class="cookie-badges">
                    <Show when=move || is_protected.get()>
                        <span class="badge badge-protected" title="Protected — this cookie is auto-restored if changed">
                            <i class="fas fa-lock"></i>
                        </span>
                    </Show>
                    {is_secure.then(|| view! {
                        <span class="badge badge-secure" title="Secure (HTTPS only)"><i class="fas fa-key"></i></span>
                    })}
                    {is_http_only.then(|| view! {
                        <span class="badge badge-httponly" title="HttpOnly (no JavaScript access)"><i class="fas fa-eye-slash"></i></span>
                    })}
                    {if is_host_only {
                        view! {
                            <span class="badge badge-scope" title="Host-only — accessible only to this exact host">
                                <i class="fas fa-server"></i>
                            </span>
                        }
                    } else {
                        view! {
                            <span class="badge badge-scope" title="Domain cookie — accessible to this domain and all subdomains">
                                <i class="fas fa-globe"></i>
                            </span>
                        }
                    }}
                    <span class="badge badge-expiry" title=expiry_title>
                        <i class="fas fa-clock"></i>
                        {expiry_label}
                    </span>
                </span>
                {
                    let domain_after = domain.clone();
                    view! {
                        <Show when=move || show_domain.get() && !show_domain_before_name.get()>
                            <span class="cookie-domain">{domain_after.clone()}</span>
                        </Show>
                    }
                }
                <i class="fas" class:fa-chevron-down=move || expanded.get() class:fa-chevron-right=move || !expanded.get()></i>
            </div>
            <Show when=move || expanded.get()>
                <CookieDetails
                    cookie=cookie.clone()
                    current_url=current_url.clone()
                    on_cookie_changed=on_cookie_changed
                    show_alerts=show_alerts
                    show_command_labels=show_command_labels
                    custom_descriptions=custom_descriptions
                    on_desc_changed=on_desc_changed
                    is_protected=is_protected
                    set_is_protected=set_is_protected
                />
            </Show>
        </div>
    }
}

fn format_relative_expiry(expiry: Option<f64>) -> String {
    let Some(ts) = expiry else {
        return "Session".to_string();
    };
    let now = js_sys::Date::now() / 1000.0;
    let diff = ts - now;
    if diff <= 0.0 {
        return "Expired".to_string();
    }
    let minutes = diff / 60.0;
    let hours = minutes / 60.0;
    let days = hours / 24.0;
    if days >= 365.0 {
        format!("{:.0}y", days / 365.0)
    } else if days >= 30.0 {
        format!("{:.0}mo", days / 30.0)
    } else if days >= 1.0 {
        format!("{:.0}d", days)
    } else if hours >= 1.0 {
        format!("{:.0}h", hours)
    } else {
        format!("{:.0}m", minutes.max(1.0))
    }
}

#[component]
fn CookieDetails(
    cookie: Cookie,
    current_url: String,
    on_cookie_changed: Callback<()>,
    show_alerts: Signal<bool>,
    show_command_labels: Signal<bool>,
    custom_descriptions: ReadSignal<HashMap<String, String>>,
    on_desc_changed: Callback<(String, Option<String>)>,
    is_protected: ReadSignal<bool>,
    set_is_protected: WriteSignal<bool>,
) -> impl IntoView {
    let (value, set_value) = signal(cookie.value.clone());
    let (domain, set_domain) = signal(cookie.domain.clone());
    let (path, set_path) = signal(cookie.path.clone().unwrap_or_default());
    let (secure, set_secure) = signal(cookie.secure);
    let (http_only, set_http_only) = signal(cookie.http_only);
    let (expiration_date, set_expiration_date) = signal(cookie.expiration_date);

    // --- Description editor state ---
    // Store cookie name in a signal so closures inside <Show> only capture Copy types
    let (desc_name_sig, _) = signal(cookie.name.clone());
    let (editing_desc, set_editing_desc) = signal(false);
    let (desc_input, set_desc_input) = signal(String::new());

    let desc_text = Signal::derive(move || {
        let name = desc_name_sig.get();
        let custom = custom_descriptions.get();
        if let Some(d) = custom.get(&name) {
            return d.clone();
        }
        crate::shared::known_cookies::lookup(&name)
            .map(|s| s.to_string())
            .unwrap_or_default()
    });

    let is_custom_desc = Signal::derive(move || {
        custom_descriptions.get().contains_key(&desc_name_sig.get())
    });

    let has_builtin = crate::shared::known_cookies::lookup(&cookie.name).is_some();

    let cookie_name = cookie.name.clone();
    let cookie_same_site = cookie.same_site.clone();
    let cookie_store_id = cookie.store_id.clone();

    view! {
        <div class="cookie-details" class:protected=is_protected>
            // Description editor row
            <div class="detail-row desc-row">
                <label>"Description:"</label>
                <Show when=move || !editing_desc.get()>
                    <Show when=move || !desc_text.get().is_empty()>
                        <span class="cookie-desc-text" class:custom=is_custom_desc>
                            {move || desc_text.get()}
                        </span>
                    </Show>
                    <button class="desc-edit-btn" title="Edit description" on:click=move |_| {
                        set_desc_input.set(desc_text.get_untracked());
                        set_editing_desc.set(true);
                    }>
                        <i class="fas fa-pencil-alt"></i>
                    </button>
                    <Show when=move || is_custom_desc.get() && has_builtin>
                        <button class="desc-reset-btn" title="Reset to default" on:click=move |_| {
                            on_desc_changed.run((desc_name_sig.get(), None));
                        }>
                            <i class="fas fa-undo"></i>
                        </button>
                    </Show>
                </Show>
                <Show when=move || editing_desc.get()>
                    <input
                        type="text"
                        class="desc-input"
                        maxlength="500"
                        placeholder="Enter cookie description..."
                        prop:value=desc_input
                        on:input=move |e| set_desc_input.set(event_target_value(&e))
                    />
                    <button class="desc-save-btn" title="Save" on:click=move |_| {
                        let val = desc_input.get_untracked();
                        let trimmed = val.trim();
                        if !trimmed.is_empty() {
                            let bounded: String = trimmed.chars().take(500).collect();
                            on_desc_changed.run((desc_name_sig.get(), Some(bounded)));
                        }
                        set_editing_desc.set(false);
                    }>
                        <i class="fas fa-check"></i>
                    </button>
                    <button class="desc-cancel-btn" title="Cancel" on:click=move |_| {
                        set_editing_desc.set(false);
                    }>
                        <i class="fas fa-times"></i>
                    </button>
                </Show>
            </div>

            <div class="detail-row">
                <label>"Value:"</label>
                <textarea
                    class="value"
                    maxlength="4096"
                    prop:value=value
                    on:input=move |e| set_value.set(event_target_value(&e))
                ></textarea>
            </div>
            <div class="detail-row">
                <label>"Domain:"</label>
                <input
                    type="text"
                    maxlength="253"
                    prop:value=domain
                    on:input=move |e| set_domain.set(event_target_value(&e))
                />
            </div>
            <div class="detail-row">
                <label>"Path:"</label>
                <input
                    type="text"
                    maxlength="1024"
                    prop:value=path
                    on:input=move |e| set_path.set(event_target_value(&e))
                />
            </div>
            <div class="detail-row">
                <label>"Expiration:"</label>
                <input
                    type="datetime-local"
                    prop:value=move || expiration_date.get().map(format_timestamp).unwrap_or_default()
                    on:change=move |e| {
                        let val = event_target_value(&e);
                        if !val.is_empty() {
                            let date = js_sys::Date::new(&JsValue::from_str(&val));
                            set_expiration_date.set(Some(date.get_time() / 1000.0));
                        } else {
                            set_expiration_date.set(None);
                        }
                    }
                />
            </div>
            <div class="detail-row">
                <label>
                    <input
                        type="checkbox"
                        prop:checked=secure
                        on:change=move |e| set_secure.set(event_target_checked(&e))
                    />
                    "Secure"
                </label>
                <label>
                    <input
                        type="checkbox"
                        prop:checked=http_only
                        on:change=move |e| set_http_only.set(event_target_checked(&e))
                    />
                    "HttpOnly"
                </label>
            </div>
            <div class="cookie-actions">
                // Save
                {
                    let (save_state, set_save_state) = signal("idle"); // "idle" | "ok" | "err"
                    view! {
                        <button
                            class="action-btn"
                            class:save-ok=move || save_state.get() == "ok"
                            class:save-err=move || save_state.get() == "err"
                            title="Apply changes"
                            on:click={
                                let cookie_name = cookie_name.clone();
                                let cookie_same_site = cookie_same_site.clone();
                                let cookie_store_id = cookie_store_id.clone();
                                move |_| {
                                    if show_alerts.get_untracked()
                                        && !confirm("Apply changes to this cookie?")
                                    {
                                        return;
                                    }
                                    let cookie_to_save = Cookie {
                                        name: cookie_name.clone(),
                                        value: value.get_untracked(),
                                        domain: domain.get_untracked(),
                                        path: Some(path.get_untracked()),
                                        secure: secure.get_untracked(),
                                        http_only: http_only.get_untracked(),
                                        expiration_date: expiration_date.get_untracked(),
                                        same_site: cookie_same_site.clone(),
                                        store_id: cookie_store_id.clone(),
                                    };
                                    wasm_bindgen_futures::spawn_local(async move {
                                        match crate::core::cookie::set(&cookie_to_save).await {
                                            Ok(_) => {
                                                set_save_state.set("ok");
                                                on_cookie_changed.run(());

                                                let prefs = crate::core::storage::PreferencesStorage::get().await;
                                                if prefs.refresh_after_submit {
                                                    reload_active_tab().await;
                                                }
                                            }
                                            Err(e) => {
                                                set_save_state.set("err");
                                                web_sys::console::error_1(
                                                    &format!("Failed to save cookie: {:?}", e).into(),
                                                );
                                            }
                                        }
                                        flash_reset(set_save_state);
                                    });
                                }
                            }
                        >
                            <Show when=move || save_state.get() == "idle">
                                <i class="fas fa-pen"></i>
                            </Show>
                            <Show when=move || save_state.get() == "ok">
                                <i class="fas fa-check"></i>
                            </Show>
                            <Show when=move || save_state.get() == "err">
                                <i class="fas fa-exclamation-triangle"></i>
                            </Show>
                            <Show when=move || show_command_labels.get()>
                                <span class="cmd-label">"Apply"</span>
                            </Show>
                        </button>
                    }
                }
                // Delete
                <button class="action-btn" title="Delete" on:click={
                    let cookie_name = cookie_name.clone();
                    move |_| {
                        if show_alerts.get_untracked() {
                            let msg = format!("Delete cookie \"{}\"?", cookie_name);
                            if !confirm(&msg) {
                                return;
                            }
                        }
                        let url = current_url.clone();
                        let name = cookie_name.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            match crate::core::cookie::remove_by_name(&url, &name).await {
                                Ok(_) => {
                                    web_sys::console::log_1(&"Cookie deleted".into());
                                    on_cookie_changed.run(());
                                }
                                Err(e) => {
                                    web_sys::console::error_1(
                                        &format!("Failed to delete cookie: {:?}", e).into(),
                                    );
                                }
                            }
                        });
                    }
                }>
                    <i class="fas fa-trash"></i>
                    <Show when=move || show_command_labels.get()>
                        <span class="cmd-label">"Delete"</span>
                    </Show>
                </button>
                // Toggle read-only protection
                <button
                    class="action-btn action-btn-lock"
                    class:active=is_protected
                    title="Protect — auto-restores this cookie if it is changed or deleted"
                    on:click={
                        let cookie_name = cookie_name.clone();
                        let cookie_domain = cookie.domain.clone();
                        move |_| {
                            let name = cookie_name.clone();
                            let domain = cookie_domain.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                use crate::core::storage::PreferencesStorage;
                                use crate::shared::types::ReadOnlyCookie;

                                let mut prefs = PreferencesStorage::get().await;
                                let already = prefs
                                    .protected_cookies
                                    .iter()
                                    .any(|c| c.name == name && c.domain == domain);

                                if already {
                                    prefs
                                        .protected_cookies
                                        .retain(|c| !(c.name == name && c.domain == domain));
                                } else if prefs.protected_cookies.len() >= 200 {
                                    web_sys::console::warn_1(
                                        &"Protected cookies limit reached (200). Remove some before adding more.".into(),
                                    );
                                    return;
                                } else {
                                    prefs.protected_cookies.push(ReadOnlyCookie {
                                        name: name.clone(),
                                        domain: domain.clone(),
                                    });
                                }

                                if let Err(e) = PreferencesStorage::set(&prefs).await {
                                    web_sys::console::error_1(
                                        &format!("Failed to save protected cookies: {:?}", e).into(),
                                    );
                                }
                            });
                            set_is_protected.set(!is_protected.get_untracked());
                        }
                    }
                >
                    <i class="fas" class:fa-lock=is_protected class:fa-lock-open=move || !is_protected.get()></i>
                    <Show when=move || show_command_labels.get()>
                        <span class="cmd-label">"Protect"</span>
                    </Show>
                </button>
                // Copy
                <button class="action-btn" title="Copy to clipboard" on:click={
                    let cookie = cookie.clone();
                    move |_| {
                        let cookie_to_copy = cookie.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let prefs = crate::core::storage::PreferencesStorage::get().await;
                            let cookie_text = format_cookie_for_copy(&cookie_to_copy, &prefs.copy_cookies_type);

                            if let Some(window) = web_sys::window() {
                                let clipboard = window.navigator().clipboard();
                                if let Err(e) = clipboard.write_text(&cookie_text).await {
                                    web_sys::console::error_1(
                                        &format!("Failed to copy to clipboard: {:?}", e).into(),
                                    );
                                }
                            }
                        });
                    }
                }>
                    <i class="fas fa-clipboard"></i>
                    <Show when=move || show_command_labels.get()>
                        <span class="cmd-label">"Copy"</span>
                    </Show>
                </button>
                // Download as file
                <button class="action-btn" title="Download cookie as file" on:click={
                    let cookie = cookie.clone();
                    move |_| {
                        let cookie_to_dl = cookie.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let prefs = crate::core::storage::PreferencesStorage::get().await;
                            let (text, ext) = match prefs.copy_cookies_type.as_str() {
                                "json" => (format_cookie_for_copy(&cookie_to_dl, "json"), "json"),
                                "netscape" => (format_cookie_for_copy(&cookie_to_dl, "netscape"), "txt"),
                                _ => (format_cookie_for_copy(&cookie_to_dl, "semicolon"), "txt"),
                            };
                            let filename = sanitize_filename(&cookie_to_dl.name, ext);
                            save_file_with_dialog(&filename, &text).await;
                        });
                    }
                }>
                    <i class="fas fa-download"></i>
                    <Show when=move || show_command_labels.get()>
                        <span class="cmd-label">"Save"</span>
                    </Show>
                </button>
                // Duplicate
                <button class="action-btn" title="Duplicate" on:click={
                    let cookie_name = cookie_name.clone();
                    let cookie = cookie.clone();
                    move |_| {
                        let mut new_cookie = cookie.clone();
                        let base = cookie_name.trim_end_matches("_copy");
                        let mut candidate = format!("{}_copy", base);
                        if candidate.len() > 255 {
                            candidate.truncate(255);
                        }
                        new_cookie.name = candidate;
                        wasm_bindgen_futures::spawn_local(async move {
                            match crate::core::cookie::set(&new_cookie).await {
                                Ok(_) => {
                                    web_sys::console::log_1(&"Cookie duplicated".into());
                                    on_cookie_changed.run(());
                                }
                                Err(e) => {
                                    web_sys::console::error_1(
                                        &format!("Failed to duplicate cookie: {:?}", e).into(),
                                    );
                                }
                            }
                        });
                    }
                }>
                    <i class="fas fa-copy"></i>
                    <Show when=move || show_command_labels.get()>
                        <span class="cmd-label">"Duplicate"</span>
                    </Show>
                </button>
            </div>
        </div>
    }
}

/// Show a browser confirm() dialog; returns true if the user clicked OK.
fn confirm(message: &str) -> bool {
    web_sys::window()
        .map(|w| w.confirm_with_message(message).unwrap_or(false))
        .unwrap_or(false)
}

/// Reset a flash state signal back to "idle" after a short delay.
fn flash_reset(set_state: WriteSignal<&'static str>) {
    wasm_bindgen_futures::spawn_local(async move {
        let promise = js_sys::Promise::new(&mut |resolve, _| {
            let _ = web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 1500);
        });
        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
        set_state.set("idle");
    });
}

fn format_timestamp(ts: f64) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ts * 1000.0));
    let year = date.get_full_year();
    let month = date.get_month() + 1;
    let day = date.get_date();
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}",
        year, month, day, hours, minutes
    )
}

/// Format a cookie for clipboard in the user's preferred format.
fn format_cookie_for_copy(cookie: &Cookie, format: &str) -> String {
    match format {
        "json" => serde_json::to_string_pretty(cookie).unwrap_or_default(),
        "netscape" => {
            let host_only = !cookie.domain.starts_with('.');
            let subdomains_flag = if host_only { "FALSE" } else { "TRUE" };
            let secure_flag = if cookie.secure { "TRUE" } else { "FALSE" };
            let exp = cookie
                .expiration_date
                .map(|e| e.to_string())
                .unwrap_or_else(|| "0".to_string());
            format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                cookie.domain,
                subdomains_flag,
                cookie.path.as_deref().unwrap_or("/"),
                secure_flag,
                exp,
                cookie.name,
                cookie.value
            )
        }
        _ => {
            format!("{}={}", cookie.name, cookie.value)
        }
    }
}

/// Sanitize a cookie name for use as a download filename.
fn sanitize_filename(name: &str, ext: &str) -> String {
    let clean: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            _ => c,
        })
        .collect();
    let trimmed = clean.trim_start_matches('.');
    let base = if trimmed.is_empty() { "cookie" } else { trimmed };
    format!("{}.{}", base, ext)
}

/// Save text content via chrome.downloads with a native Save As dialog.
/// Uses a data URL to avoid Blob lifecycle issues if the popup closes
/// while the dialog is open.
async fn save_file_with_dialog(filename: &str, content: &str) {
    use js_sys::{Object, Reflect};

    let encoded_js = js_sys::encode_uri_component(content);
    let encoded: JsValue = encoded_js.into();
    let encoded_str = encoded.as_string().unwrap_or_default();
    let data_url = format!("data:text/plain;charset=utf-8,{}", encoded_str);

    let opts = Object::new();
    Reflect::set(&opts, &JsValue::from("url"), &JsValue::from(&data_url)).ok();
    Reflect::set(&opts, &JsValue::from("filename"), &JsValue::from(filename)).ok();
    Reflect::set(&opts, &JsValue::from("saveAs"), &JsValue::from(true)).ok();

    if let Err(e) = crate::chrome_api::download(opts.into()).await {
        let msg = format!("{:?}", e);
        if !msg.contains("cancel") {
            web_sys::console::error_1(&format!("Download failed: {}", msg).into());
        }
    }
}

/// Reload the active tab (used when `refresh_after_submit` is enabled).
/// Reads `skip_cache_refresh` to decide whether to bypass the browser cache.
async fn reload_active_tab() {
    use js_sys::{Object, Reflect};

    let prefs = crate::core::storage::PreferencesStorage::get().await;
    let bypass = !prefs.skip_cache_refresh;

    let Some(tab) = crate::shared::helpers::get_active_tab().await else {
        return;
    };
    let Ok(tab_id) = Reflect::get(&tab, &JsValue::from("id")) else {
        return;
    };
    let chrome = js_sys::Reflect::get(&js_sys::global(), &JsValue::from("chrome"))
        .unwrap_or(JsValue::UNDEFINED);
    let tabs = Reflect::get(&chrome, &JsValue::from("tabs")).unwrap_or(JsValue::UNDEFINED);
    if let Ok(reload_fn) = Reflect::get(&tabs, &JsValue::from("reload"))
        && let Ok(func) = reload_fn.dyn_into::<js_sys::Function>()
    {
        let opts = Object::new();
        Reflect::set(&opts, &JsValue::from("bypassCache"), &JsValue::from(bypass)).ok();
        let _ = func.call2(&tabs, &tab_id, &opts);
    }
}
