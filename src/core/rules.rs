//! Filter and rule matching logic.
//!
//! Filter/read-only CRUD is reserved for a future options UI; the background worker only uses matching helpers today.

#![allow(dead_code)]

use crate::core::storage::{DataStorage, PreferencesStorage};
use crate::shared::types::{Cookie, Filter, ReadOnlyCookie};
use wasm_bindgen::JsValue;

/// Check if a cookie matches any filter rule
pub async fn matches_filter(cookie: &Cookie) -> bool {
    let filters = DataStorage::get_filters().await;
    filters.iter().any(|f| f.enabled && f.matches(cookie))
}

/// Check if a cookie is read-only protected
pub async fn is_read_only(cookie: &Cookie) -> bool {
    let read_only = DataStorage::get_read_only().await;
    if read_only
        .iter()
        .any(|r| r.domain == cookie.domain && r.name == cookie.name)
    {
        return true;
    }

    let prefs = PreferencesStorage::get().await;
    prefs
        .protected_cookies
        .iter()
        .any(|r| r.domain == cookie.domain && r.name == cookie.name)
}

/// If cookie-age limiting is enabled and the cookie's expiration exceeds the max,
/// return a new Cookie with a clamped expiration.  Returns `None` if no shortening
/// is needed or the feature is disabled.
pub async fn maybe_shorten(cookie: &Cookie) -> Option<Cookie> {
    let prefs = PreferencesStorage::get().await;
    if !prefs.use_max_cookie_age {
        return None;
    }
    let exp = cookie.expiration_date?; // session cookies — nothing to shorten

    let multiplier: f64 = match prefs.max_cookie_age_type {
        1 => 3600.0,        // hours
        _ => 86400.0,       // days (default)
    };
    let now = js_sys::Date::now() / 1000.0;
    let max_exp = now + (prefs.max_cookie_age as f64 * multiplier);

    if exp > max_exp {
        let mut shortened = cookie.clone();
        shortened.expiration_date = Some(max_exp);
        Some(shortened)
    } else {
        None
    }
}

const MAX_FILTERS: usize = 100;
const MAX_PATTERN_LEN: usize = 253;

/// Add a new filter rule
pub async fn add_filter(
    domain_pattern: String,
    name_pattern: Option<String>,
) -> Result<(), JsValue> {
    if domain_pattern.len() > MAX_PATTERN_LEN {
        return Err(JsValue::from_str(&format!(
            "Domain pattern too long ({} chars, max {})",
            domain_pattern.len(),
            MAX_PATTERN_LEN
        )));
    }
    if let Some(ref np) = name_pattern
        && np.len() > MAX_PATTERN_LEN
    {
        return Err(JsValue::from_str(&format!(
            "Name pattern too long ({} chars, max {})",
            np.len(),
            MAX_PATTERN_LEN
        )));
    }

    let mut filters = DataStorage::get_filters().await;

    if filters.len() >= MAX_FILTERS {
        return Err(JsValue::from_str(&format!(
            "Filter limit reached ({}, max {}). Remove some before adding more.",
            filters.len(),
            MAX_FILTERS
        )));
    }

    let new_id = filters.iter().map(|f| f.id).max().unwrap_or(0) + 1;

    filters.push(Filter {
        id: new_id,
        domain_pattern,
        name_pattern,
        enabled: true,
    });

    DataStorage::set_filters(&filters)
        .await
        .map_err(|e| JsValue::from_str(&e))
}

/// Remove a filter rule
pub async fn remove_filter(filter_id: u64) -> Result<(), JsValue> {
    let mut filters = DataStorage::get_filters().await;
    filters.retain(|f| f.id != filter_id);
    DataStorage::set_filters(&filters)
        .await
        .map_err(|e| JsValue::from_str(&e))
}

/// Toggle filter enabled state
pub async fn toggle_filter(filter_id: u64) -> Result<(), JsValue> {
    let mut filters = DataStorage::get_filters().await;
    if let Some(filter) = filters.iter_mut().find(|f| f.id == filter_id) {
        filter.enabled = !filter.enabled;
    }
    DataStorage::set_filters(&filters)
        .await
        .map_err(|e| JsValue::from_str(&e))
}

/// Add a read-only cookie rule
pub async fn add_read_only(name: String, domain: String) -> Result<(), JsValue> {
    let mut read_only = DataStorage::get_read_only().await;

    if !read_only
        .iter()
        .any(|r| r.domain == domain && r.name == name)
    {
        read_only.push(ReadOnlyCookie { name, domain });
        DataStorage::set_read_only(&read_only).await?;
        DataStorage::increment_counter("protected").await?;
    }

    Ok(())
}

/// Remove a read-only rule
pub async fn remove_read_only(name: &str, domain: &str) -> Result<(), JsValue> {
    let mut read_only = DataStorage::get_read_only().await;
    read_only.retain(|r| !(r.domain == domain && r.name == name));
    DataStorage::set_read_only(&read_only)
        .await
        .map_err(|e| JsValue::from_str(&e))
}
