//! DevTools panel handling.
//!
//! `chrome.devtools.panels.create` must be called from a devtools page, not the service worker.
//! The actual panel creation happens in `public/devtools/devtools-page.html`.
//! This module just logs readiness.

/// Initialize DevTools support (called from the background service worker).
pub async fn init_devtools() {
    oxichrome::log!("DevTools support enabled");
}
