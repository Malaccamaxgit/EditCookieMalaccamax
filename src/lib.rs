//! Edit Cookie - A Rust/WebAssembly cookie editor extension
//!
//! This is a rewrite of the Edit-This-Cookie-V3 extension in Rust using Oxichrome.

use js_sys::{Object, Reflect};
use leptos::IntoView;
use wasm_bindgen::prelude::*;

mod background;
mod chrome_api;
mod core;
mod devtools;
mod options;
mod popup;
mod shared;

// ============================================================================
// Extension Definition
// ============================================================================

#[oxichrome::extension(
    name = "Edit Cookie",
    version = "0.2.0",
    description = "Edit, delete, protect, and export browser cookies. Manage values, expiration, and security flags with ease.",
    permissions = [
        "cookies",
        "contextMenus",
        "storage",
        "clipboardWrite",
        "downloads"
    ]
)]
struct Extension;

// ============================================================================
// Background Service Worker
// ============================================================================

#[oxichrome::background]
async fn start() {
    oxichrome::log!("Edit Cookie started!");

    // Initialize cookie monitoring
    background::init_cookie_monitor().await;

    let show_menu = crate::core::storage::PreferencesStorage::get()
        .await
        .show_context_menu;
    background::update_context_menus(show_menu).await;

    // DevTools panel is created on-demand
    background::init_devtools().await;
}

// ============================================================================
// Lifecycle Events
// ============================================================================

#[oxichrome::on(runtime::on_installed)]
async fn handle_install(details: JsValue) {
    let reason = Reflect::get(&details, &JsValue::from("reason"))
        .ok()
        .and_then(|v| v.as_string());

    if let Some(ref reason) = reason
        && reason == "install"
    {
        let create_props = Object::new();
        Reflect::set(
            &create_props,
            &JsValue::from("url"),
            &JsValue::from("https://editcookie.com/#start"),
        )
        .ok();
        let _ = chrome_api::create(create_props.into()).await.ok();
    }

    let reason_str = reason.as_deref().unwrap_or("unknown");
    oxichrome::log!("Extension installed: {}", reason_str);
}

// ============================================================================
// Popup
// ============================================================================

#[oxichrome::popup]
fn Popup() -> impl IntoView {
    popup::App()
}

// ============================================================================
// Options Page
// ============================================================================

#[oxichrome::options_page]
fn OptionsPage() -> impl IntoView {
    options::App()
}

