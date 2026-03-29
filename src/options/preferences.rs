//! Preferences form component

use crate::core::storage;
use crate::shared::types::Preferences;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn PreferencesForm(
    preferences: ReadSignal<Preferences>,
    set_preferences: WriteSignal<Preferences>,
) -> impl IntoView {
    let prefs = preferences.get();

    let toggle_pref = move |field: &'static str, value: bool| {
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = storage::PreferencesStorage::update_bool(field, value).await {
                web_sys::console::error_1(&format!("Failed to save preference: {:?}", e).into());
            }
        });
    };

    let set_string_pref = move |field: &'static str, value: String| {
        wasm_bindgen_futures::spawn_local(async move {
            let mut new_prefs = storage::PreferencesStorage::get().await;
            match field {
                "customLocale" => new_prefs.custom_locale = value,
                "copyCookiesType" => new_prefs.copy_cookies_type = value,
                "sortCookiesType" => new_prefs.sort_cookies_type = value,
                "theme" => new_prefs.theme = value,
                _ => {}
            }
            if let Err(e) = storage::PreferencesStorage::set(&new_prefs).await {
                web_sys::console::error_1(&format!("Failed to save preference: {:?}", e).into());
            }
        });
    };

    view! {
        <div class="preferences-form">
            <section class="pref-section">
                <h2>"Display Options"</h2>
                <div class="pref-row">
                    <label title="Show the cookie domain next to the cookie name">
                        <input
                            type="checkbox"
                            checked=prefs.show_domain
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                {
                                    let val = input.checked();
                                    set_preferences.update(|p| p.show_domain = val);
                                    toggle_pref("showDomain", val);
                                }
                            }
                        />
                        <span>"Show cookie domain"</span>
                    </label>
                </div>
                <div class="pref-row">
                    <label title="Display domain before the cookie name instead of after">
                        <input
                            type="checkbox"
                            checked=prefs.show_domain_before_name
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                {
                                    let val = input.checked();
                                    set_preferences.update(|p| p.show_domain_before_name = val);
                                    toggle_pref("showDomainBeforeName", val);
                                }
                            }
                        />
                        <span>"Show domain before name"</span>
                    </label>
                </div>
                <div class="pref-row">
                    <label title="Show text labels next to command icons for easier identification">
                        <input
                            type="checkbox"
                            checked=prefs.show_commands_labels
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                {
                                    let val = input.checked();
                                    set_preferences.update(|p| p.show_commands_labels = val);
                                    toggle_pref("showCommandsLabels", val);
                                }
                            }
                        />
                        <span>"Show command labels"</span>
                    </label>
                </div>
            </section>

            <section class="pref-section">
                <h2>"Behavior"</h2>
                <div class="pref-row">
                    <label title="Add right-click context menu option to edit cookies for the current site">
                        <input
                            type="checkbox"
                            checked=prefs.show_context_menu
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                {
                                    let val = input.checked();
                                    set_preferences.update(|p| p.show_context_menu = val);
                                    toggle_pref("showContextMenu", val);
                                }
                            }
                        />
                        <span>"Show context menu"</span>
                    </label>
                </div>
                <div class="pref-row">
                    <label title="Automatically refresh the page after saving cookie changes">
                        <input
                            type="checkbox"
                            checked=prefs.refresh_after_submit
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                {
                                    let val = input.checked();
                                    set_preferences.update(|p| p.refresh_after_submit = val);
                                    toggle_pref("refreshAfterSubmit", val);
                                }
                            }
                        />
                        <span>"Refresh after submit"</span>
                    </label>
                </div>
                <div class="pref-row">
                    <label title="Show confirmation dialogs before deleting or modifying cookies">
                        <input
                            type="checkbox"
                            checked=prefs.show_alerts
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                {
                                    let val = input.checked();
                                    set_preferences.update(|p| p.show_alerts = val);
                                    toggle_pref("showAlerts", val);
                                }
                            }
                        />
                        <span>"Show confirmation alerts"</span>
                    </label>
                </div>
                <div class="pref-row">
                    <label title="Skip refreshing the browser cache when saving cookie changes">
                        <input
                            type="checkbox"
                            checked=prefs.skip_cache_refresh
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                {
                                    let val = input.checked();
                                    set_preferences.update(|p| p.skip_cache_refresh = val);
                                    toggle_pref("skipCacheRefresh", val);
                                }
                            }
                        />
                        <span>"Skip cache refresh"</span>
                    </label>
                </div>
            </section>

            <section class="pref-section">
                <h2>"Cookie Age Limit"</h2>
                <div class="pref-row">
                    <label title="Automatically expire cookies older than the specified time limit">
                        <input
                            type="checkbox"
                            checked=prefs.use_max_cookie_age
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                {
                                    let val = input.checked();
                                    set_preferences.update(|p| p.use_max_cookie_age = val);
                                    toggle_pref("useMaxCookieAge", val);
                                }
                            }
                        />
                        <span>"Limit cookie expiration"</span>
                    </label>
                </div>
                <Show when=move || preferences.get().use_max_cookie_age>
                    <div class="pref-row nested">
                        <label title="Maximum age for cookies before they expire">
                            "Max age:"
                            <input
                                type="number"
                                min="1"
                                title="Enter the maximum age value"
                                value=prefs.max_cookie_age.to_string()
                                on:input=move |e| {
                                    if let Some(target) = e.target()
                                        && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                                    {
                                        let val = input.value();
                                        if let Ok(age) = val.parse::<u64>() {
                                            set_preferences.update(|p| p.max_cookie_age = age);
                                        }
                                    }
                                }
                            />
                            <select
                                on:change=move |e| {
                                    if let Some(target) = e.target()
                                        && let Some(select) = target.dyn_ref::<web_sys::HtmlSelectElement>()
                                    {
                                        let val = select.value();
                                        set_preferences.update(|p| p.max_cookie_age_type = if val == "days" { 0 } else { 1 });
                                    }
                                }
                                title="Select time unit (days or hours)"
                            >
                                <option value="days" selected=prefs.max_cookie_age_type == 0>"Days"</option>
                                <option value="hours" selected=prefs.max_cookie_age_type == 1>"Hours"</option>
                            </select>
                        </label>
                    </div>
                </Show>
            </section>

            <section class="pref-section">
                <h2>"Appearance"</h2>
                <div class="pref-row">
                    <label title="Select theme: System (follows OS), Light, or Dark">
                        "Theme:"
                        <select
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(select) = target.dyn_ref::<web_sys::HtmlSelectElement>()
                                {
                                    let val = select.value();
                                    set_preferences.update(|p| p.theme = val.clone());
                                    set_string_pref("theme", val);
                                }
                            }
                        >
                            <option value="light" selected=prefs.theme == "light">"Light"</option>
                            <option value="dark" selected=prefs.theme == "dark">"Dark"</option>
                        </select>
                    </label>
                </div>
                <div class="pref-row">
                    <label title="Choose how cookies are sorted in the list">
                        "Sort cookies by:"
                        <select
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(select) = target.dyn_ref::<web_sys::HtmlSelectElement>()
                                {
                                    let val = select.value();
                                    set_preferences.update(|p| p.sort_cookies_type = val.clone());
                                    set_string_pref("sortCookiesType", val);
                                }
                            }
                        >
                            <option value="domain_alpha" selected=prefs.sort_cookies_type == "domain_alpha">"Domain (A-Z)"</option>
                            <option value="name_alpha" selected=prefs.sort_cookies_type == "name_alpha">"Name (A-Z)"</option>
                            <option value="expiration" selected=prefs.sort_cookies_type == "expiration">"Expiration"</option>
                        </select>
                    </label>
                </div>
                <div class="pref-row">
                    <label title="Format used when copying cookies to clipboard">
                        "Copy format:"
                        <select
                            on:change=move |e| {
                                if let Some(target) = e.target()
                                    && let Some(select) = target.dyn_ref::<web_sys::HtmlSelectElement>()
                                {
                                    let val = select.value();
                                    set_preferences.update(|p| p.copy_cookies_type = val.clone());
                                    set_string_pref("copyCookiesType", val);
                                }
                            }
                        >
                            <option value="json" selected=prefs.copy_cookies_type == "json">"JSON"</option>
                            <option value="netscape" selected=prefs.copy_cookies_type == "netscape">"Netscape"</option>
                            <option value="semicolon" selected=prefs.copy_cookies_type == "semicolon">"Semicolon pairs"</option>
                        </select>
                    </label>
                </div>
            </section>

        </div>
    }
}
