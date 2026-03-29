//! Search bar and attribute filter components

use leptos::prelude::*;

#[derive(Clone, Default, PartialEq)]
pub struct CookieFilters {
    pub host_only: bool,
    pub domain: bool,
    pub secure: bool,
    pub http_only: bool,
    pub session: bool,
    pub persistent: bool,
}

#[component]
pub fn SearchBar(
    search_term: ReadSignal<String>,
    set_search_term: WriteSignal<String>,
    filters: ReadSignal<CookieFilters>,
    set_filters: WriteSignal<CookieFilters>,
    cookie_count: Signal<usize>,
) -> impl IntoView {
    view! {
        <div id="searchBox">
            <input
                type="text"
                id="cookieSearchCondition"
                maxlength="200"
                placeholder="Search cookies..."
                prop:value=move || search_term.get()
                on:input=move |e| {
                    set_search_term.set(event_target_value(&e));
                }
            />
            <span class="cookie-count" title="Number of cookies shown">{move || cookie_count.get()}</span>
            <button id="searchBtn" title="Search">
                <i class="fas fa-search"></i>
            </button>
        </div>
        <div class="filter-bar">
            <button
                class="filter-btn filter-host"
                class:active=move || filters.get().host_only
                title="Host-only cookies"
                on:click=move |_| set_filters.update(|f| f.host_only = !f.host_only)
            >
                <i class="fas fa-server"></i>
            </button>
            <button
                class="filter-btn filter-domain"
                class:active=move || filters.get().domain
                title="Domain cookies"
                on:click=move |_| set_filters.update(|f| f.domain = !f.domain)
            >
                <i class="fas fa-globe"></i>
            </button>
            <button
                class="filter-btn filter-secure"
                class:active=move || filters.get().secure
                title="Secure cookies"
                on:click=move |_| set_filters.update(|f| f.secure = !f.secure)
            >
                <i class="fas fa-key"></i>
            </button>
            <button
                class="filter-btn filter-httponly"
                class:active=move || filters.get().http_only
                title="HttpOnly cookies"
                on:click=move |_| set_filters.update(|f| f.http_only = !f.http_only)
            >
                <i class="fas fa-eye-slash"></i>
            </button>
            <button
                class="filter-btn filter-session"
                class:active=move || filters.get().session
                title="Session cookies"
                on:click=move |_| set_filters.update(|f| f.session = !f.session)
            >
                <i class="fas fa-hourglass-end"></i>
            </button>
            <button
                class="filter-btn filter-persistent"
                class:active=move || filters.get().persistent
                title="Persistent cookies"
                on:click=move |_| set_filters.update(|f| f.persistent = !f.persistent)
            >
                <i class="fas fa-clock"></i>
            </button>
        </div>
    }
}
