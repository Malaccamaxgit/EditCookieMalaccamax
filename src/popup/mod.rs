//! Popup UI module

use std::collections::HashMap;

use leptos::ev;
use leptos::prelude::*;

mod cookie_editor;
mod cookie_list;
mod search;

use cookie_list::CookieRow;
use search::{CookieFilters, SearchBar};

use crate::core::storage::{CustomDescriptions, PreferencesStorage};
use crate::options::preferences::PreferencesForm;
use crate::shared::helpers;
use crate::shared::types::{Cookie, Preferences};

fn scroll_list_to_top() {
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("cookiesList"))
    {
        el.scroll_into_view();
    }
}

fn request_animation_frame(f: fn()) {
    use wasm_bindgen::prelude::*;
    let cb = Closure::once_into_js(f);
    if let Some(w) = web_sys::window() {
        let _ = w.request_animation_frame(cb.unchecked_ref());
    }
}

#[component]
pub fn App() -> impl IntoView {
    helpers::ensure_stylesheets("css/popup.css");

    let (preferences, set_preferences) = signal(Preferences::default());
    let (show_settings, set_show_settings) = signal(false);

    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let prefs = PreferencesStorage::get().await;
            set_preferences.set(prefs.clone());
            helpers::apply_theme(&prefs.theme);
        });
    });

    let cycle_theme = move |_: ev::MouseEvent| {
        let current_theme = preferences.get().theme;
        let new_theme = match current_theme.as_str() {
            "light" => "dark",
            _ => "light",
        };

        let mut new_prefs = preferences.get();
        new_prefs.theme = new_theme.to_string();
        let prefs_to_save = new_prefs.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let _ = PreferencesStorage::set(&prefs_to_save).await;
        });
        set_preferences.set(new_prefs);
        helpers::apply_theme(new_theme);
    };

    let theme_label = Signal::derive(move || {
        match preferences.get().theme.as_str() {
            "dark" => "Dark",
            _ => "Light",
        }
        .to_string()
    });

    let theme_icon = Signal::derive(move || {
        match preferences.get().theme.as_str() {
            "dark" => "fa-moon",
            _ => "fa-sun",
        }
    });

    let open_settings = move |_| {
        // Re-read from storage to get the latest state when opening the modal
        wasm_bindgen_futures::spawn_local(async move {
            let prefs = PreferencesStorage::get().await;
            set_preferences.set(prefs);
        });
        set_show_settings.set(true);
    };

    let close_settings = move |_| set_show_settings.set(false);

    // Save writes the in-memory preferences object to storage as one atomic write
    let save_settings = move |_| {
        let prefs = preferences.get();
        let theme = prefs.theme.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = PreferencesStorage::set(&prefs).await;
        });
        helpers::apply_theme(&theme);
        set_show_settings.set(false);
    };

    view! {
        <div class="app-container">
            <div class="popup-header">
                <h1 class="popup-title">
                    <img src="icons/icon-32.png" alt="" class="popup-title-icon" />
                    <span>"Edit Cookies"</span>
                </h1>
                <div class="header-actions">
                    <button class="theme-toggle" on:click=cycle_theme title="Toggle theme">
                        <i class=move || format!("fas {}", theme_icon.get())></i>
                        <span class="theme-label">{move || theme_label.get()}</span>
                    </button>
                    <button class="settings-btn" on:click=open_settings title="Settings">
                        <i class="fas fa-cog"></i>
                    </button>
                </div>
            </div>
            <CookieListWithSearch preferences=preferences />
            <div class="version-number">{format!("v{}", env!("CARGO_PKG_VERSION"))}</div>

            // Settings Modal
            <Show when=move || show_settings.get()>
                <div class="modal-overlay" on:click=close_settings>
                    <div class="modal-content" on:click=move |e: ev::MouseEvent| e.stop_propagation()>
                        <div class="modal-header">
                            <h2>"Settings"</h2>
                            <button class="modal-close" on:click=close_settings>
                                <i class="fas fa-times"></i>
                            </button>
                        </div>
                        <div class="modal-body">
                            <PreferencesForm
                                preferences=preferences
                                set_preferences=set_preferences
                            />
                        </div>
                        <div class="modal-footer">
                            <button class="btn btn-secondary" on:click=close_settings>"Cancel"</button>
                            <button class="btn btn-primary" on:click=save_settings>"Save"</button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}

/// Cookie list with search bar and add-cookie button.
/// `preferences` is threaded through so display prefs (show_domain, etc.) are reactive.
#[component]
fn CookieListWithSearch(preferences: ReadSignal<Preferences>) -> impl IntoView {
    let (cookies, set_cookies) = signal(Vec::<Cookie>::new());
    let (loading, set_loading) = signal(true);
    let (current_url, set_current_url) = signal(String::new());
    let (search_term, set_search_term) = signal(String::new());
    let (filters, set_filters) = signal(CookieFilters::default());
    let (custom_descs, set_custom_descs) = signal(HashMap::<String, String>::new());
    let (expand_key, set_expand_key) = signal(Option::<String>::None);

    let reload_cookies = Callback::new(move |_| {
        set_expand_key.set(None);
        set_loading.set(true);
        let url = current_url.get_untracked();
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(cookies) = crate::core::cookie::get_all_for_url(&url).await {
                set_cookies.set(cookies);
            }
            set_loading.set(false);
        });
    });

    // Load cookies + custom descriptions on mount
    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            set_loading.set(true);

            let descs = CustomDescriptions::get_all().await;
            set_custom_descs.set(descs);

            if let Some(url) = helpers::get_current_tab_url().await {
                set_current_url.set(url.clone());
                match crate::core::cookie::get_all_for_url(&url).await {
                    Ok(c) => set_cookies.set(c),
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!("Failed to load cookies: {:?}", e).into(),
                        );
                    }
                }
            }
            set_loading.set(false);
        });
    });

    // Auto-scroll to the pinned cookie once loading finishes
    Effect::new(move |_| {
        let is_loading = loading.get();
        let has_pinned = expand_key.get().is_some();
        if !is_loading && has_pinned {
            request_animation_frame(scroll_list_to_top);
        }
    });

    let on_desc_changed = Callback::new(move |(name, desc): (String, Option<String>)| {
        let mut descs = custom_descs.get();
        match desc {
            Some(d) => { descs.insert(name, d); }
            None => { descs.remove(&name); }
        }
        set_custom_descs.set(descs.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let _ = CustomDescriptions::save(&descs).await;
        });
    });

    // Derive a filtered + sorted cookie list from the raw list + search term + preferences.
    // Newly created/duplicated cookies are pinned to the top.
    let visible_cookies = Signal::derive(move || {
        let term = search_term.get().to_lowercase();
        let f = filters.get();
        let prefs = preferences.get();
        let pinned = expand_key.get();
        let mut list: Vec<Cookie> = cookies
            .get()
            .into_iter()
            .filter(|c| {
                if term.is_empty() {
                    return true;
                }
                c.name.to_lowercase().contains(&term)
                    || c.domain.to_lowercase().contains(&term)
                    || c.value.to_lowercase().contains(&term)
            })
            .filter(|c| {
                // Scope pair: host-only vs domain
                match (f.host_only, f.domain) {
                    (true, false) => { if c.domain.starts_with('.') { return false; } }
                    (false, true) => { if !c.domain.starts_with('.') { return false; } }
                    _ => {}
                }
                if f.secure && !c.secure { return false; }
                if f.http_only && !c.http_only { return false; }
                // Expiry pair: session vs persistent
                match (f.session, f.persistent) {
                    (true, false) => { if c.expiration_date.is_some() { return false; } }
                    (false, true) => { if c.expiration_date.is_none() { return false; } }
                    _ => {}
                }
                true
            })
            .collect();

        match prefs.sort_cookies_type.as_str() {
            "name_alpha" => list.sort_by(|a, b| a.name.cmp(&b.name)),
            "expiration" => list.sort_by(|a, b| {
                a.expiration_date
                    .partial_cmp(&b.expiration_date)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            _ => list.sort_by(|a, b| a.domain.cmp(&b.domain).then(a.name.cmp(&b.name))),
        }

        if let Some(ref key) = pinned {
            list.sort_by(|a, b| {
                let a_key = format!("{}::{}", a.domain, a.name);
                let b_key = format!("{}::{}", b.domain, b.name);
                let a_pinned = a_key == *key;
                let b_pinned = b_key == *key;
                b_pinned.cmp(&a_pinned)
            });
        }

        list
    });

    let show_domain = Signal::derive(move || preferences.get().show_domain);
    let show_domain_before_name = Signal::derive(move || preferences.get().show_domain_before_name);
    let show_alerts = Signal::derive(move || preferences.get().show_alerts);
    let show_command_labels = Signal::derive(move || preferences.get().show_commands_labels);

    let is_http_page = Signal::derive(move || {
        let url = current_url.get();
        url.starts_with("http://") || url.starts_with("https://")
    });

    view! {
        <>
            <SearchBar
                search_term=search_term
                set_search_term=set_search_term
                filters=filters
                set_filters=set_filters
                cookie_count=Signal::derive(move || visible_cookies.get().len())
            />
            <div id="cookiesList">
                <Show when=move || loading.get()>
                    <div class="loader-container">
                        <i class="fas fa-spinner fa-spin"></i>
                    </div>
                </Show>
                <Show when=move || !loading.get() && visible_cookies.get().is_empty()>
                    <div id="noCookies">
                        <img src="icons/icon-48.png" alt="" class="no-cookies-icon" />
                        <span>"No cookies found"</span>
                    </div>
                </Show>
                <For
                    each=move || visible_cookies.get()
                    key=|cookie| format!("{}::{}", cookie.domain, cookie.name)
                    children=move |cookie| {
                        let key = format!("{}::{}", cookie.domain, cookie.name);
                        let should_expand = expand_key.get_untracked().as_deref() == Some(key.as_str());
                        view! {
                            <CookieRow
                                cookie=cookie.clone()
                                current_url=current_url.get()
                                on_cookie_changed=reload_cookies
                                show_domain=show_domain
                                show_domain_before_name=show_domain_before_name
                                show_alerts=show_alerts
                                show_command_labels=show_command_labels
                                custom_descriptions=custom_descs
                                on_desc_changed=on_desc_changed
                                initially_expanded=should_expand
                            />
                        }
                    }
                />
            </div>
            <div class="popup-footer">
                <button
                    class="add-cookie-btn"
                    disabled=move || !is_http_page.get()
                    title=move || if is_http_page.get() {
                        "Add a new cookie for this site"
                    } else {
                        "Cookies can only be created on HTTP/HTTPS pages"
                    }
                    on:click=move |_| {
                        let reload = reload_cookies;
                        wasm_bindgen_futures::spawn_local(async move {
                            let Some(url) = helpers::get_current_tab_url().await else {
                                return;
                            };
                            if !url.starts_with("http://") && !url.starts_with("https://") {
                                return;
                            }
                            let host = url.split("://").nth(1)
                                .and_then(|rest| rest.split('/').next())
                                .unwrap_or("");
                            if host.is_empty() {
                                return;
                            }
                            let domain = host.strip_prefix("www.").unwrap_or(host);
                            let secure = url.starts_with("https");
                            let same_site = if secure { "no_restriction" } else { "lax" };
                            let cookie_domain = format!(".{}", domain);
                            let new_cookie = Cookie {
                                name: "new_cookie".to_string(),
                                value: String::new(),
                                domain: cookie_domain.clone(),
                                path: Some("/".to_string()),
                                secure,
                                http_only: false,
                                expiration_date: None,
                                same_site: Some(same_site.to_string()),
                                store_id: None,
                            };
                            match crate::core::cookie::set(&new_cookie).await {
                                Ok(_) => {
                                    reload.run(());
                                    set_expand_key.set(Some(format!("{}::new_cookie", cookie_domain)));
                                }
                                Err(e) => {
                                    web_sys::console::error_1(
                                        &format!("Failed to create cookie: {:?}", e).into(),
                                    );
                                }
                            }
                        });
                    }
                >
                    <i class="fas fa-plus"></i>
                    <Show when=move || is_http_page.get()>
                        <span>"Add Cookie"</span>
                    </Show>
                    <Show when=move || !is_http_page.get()>
                        <span>"Cookies unavailable on this page"</span>
                    </Show>
                </button>
            </div>
        </>
    }
}
