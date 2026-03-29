//! Typed storage wrapper for chrome.storage.local

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::types::{ExtensionStats, Filter, Preferences, ReadOnlyCookie};

const DATA_PREFIX: &str = "data_";

/// Get a value from storage with a default
pub async fn get<T: for<'de> Deserialize<'de>>(key: &str, default: T) -> T {
    match oxichrome::storage::get::<T>(key).await {
        Ok(Some(val)) => val,
        _ => default,
    }
}

/// Set a value in storage
pub async fn set<T: Serialize>(key: &str, value: &T) -> Result<(), String> {
    oxichrome::storage::set(key, value)
        .await
        .map_err(|e| format!("{:?}", e))
}

/// Remove a value from storage
#[allow(dead_code)]
pub async fn remove(key: &str) -> Result<(), String> {
    oxichrome::storage::remove(key)
        .await
        .map_err(|e| format!("{:?}", e))
}

/// Preferences storage — always read/write the full object to avoid split-brain.
pub struct PreferencesStorage;

impl PreferencesStorage {
    pub async fn get() -> Preferences {
        get("preferences", Preferences::default()).await
    }

    pub async fn set(prefs: &Preferences) -> Result<(), String> {
        let mut sanitized = prefs.clone();

        if !matches!(sanitized.theme.as_str(), "light" | "dark") {
            sanitized.theme = "light".to_string();
        }
        if !matches!(
            sanitized.copy_cookies_type.as_str(),
            "json" | "netscape" | "semicolon"
        ) {
            sanitized.copy_cookies_type = "json".to_string();
        }
        if !matches!(
            sanitized.sort_cookies_type.as_str(),
            "domain_alpha" | "name_alpha" | "expiration"
        ) {
            sanitized.sort_cookies_type = "domain_alpha".to_string();
        }

        if sanitized.custom_locale.len() > 10 {
            sanitized.custom_locale.truncate(10);
        }

        if sanitized.max_cookie_age > 36500 {
            sanitized.max_cookie_age = 36500;
        }

        set("preferences", &sanitized).await
    }

    /// Read-modify-write a single boolean field by name (camelCase key matches `Preferences` fields).
    pub async fn update_bool(field: &str, value: bool) -> Result<(), String> {
        let mut prefs = Self::get().await;
        match field {
            "showAlerts" => prefs.show_alerts = value,
            "showCommandsLabels" => prefs.show_commands_labels = value,
            "showDomain" => prefs.show_domain = value,
            "showDomainBeforeName" => prefs.show_domain_before_name = value,
            "showFlagAndDeleteAll" => prefs.show_flag_and_delete_all = value,
            "showContextMenu" => prefs.show_context_menu = value,
            "refreshAfterSubmit" => prefs.refresh_after_submit = value,
            "skipCacheRefresh" => prefs.skip_cache_refresh = value,
            "useMaxCookieAge" => prefs.use_max_cookie_age = value,
            "useCustomLocale" => prefs.use_custom_locale = value,
            _ => {}
        }
        Self::set(&prefs).await
    }
}

/// Data storage for filters, read-only cookies, and stats
pub struct DataStorage;

impl DataStorage {
    pub async fn get_filters() -> Vec<Filter> {
        get(&format!("{}filters", DATA_PREFIX), Vec::new()).await
    }

    #[allow(dead_code)]
    pub async fn set_filters(filters: &[Filter]) -> Result<(), String> {
        set(&format!("{}filters", DATA_PREFIX), &filters.to_vec()).await
    }

    pub async fn get_read_only() -> Vec<ReadOnlyCookie> {
        get(&format!("{}readOnly", DATA_PREFIX), Vec::new()).await
    }

    #[allow(dead_code)]
    pub async fn set_read_only(cookies: &[ReadOnlyCookie]) -> Result<(), String> {
        set(&format!("{}readOnly", DATA_PREFIX), &cookies.to_vec()).await
    }

    #[allow(dead_code)]
    pub async fn get_stats() -> ExtensionStats {
        get("stats", ExtensionStats::default()).await
    }

    #[allow(dead_code)]
    pub async fn set_stats(stats: &ExtensionStats) -> Result<(), String> {
        set("stats", stats).await
    }

    #[allow(dead_code)]
    pub async fn increment_counter(counter: &str) -> Result<(), String> {
        let mut stats = Self::get_stats().await;
        match counter {
            "created" => stats.n_cookies_created += 1,
            "changed" => stats.n_cookies_changed += 1,
            "deleted" => stats.n_cookies_deleted += 1,
            "protected" => stats.n_cookies_protected += 1,
            "flagged" => stats.n_cookies_flagged += 1,
            "shortened" => stats.n_cookies_shortened += 1,
            "popup_clicked" => stats.n_popup_clicked += 1,
            "panel_clicked" => stats.n_panel_clicked += 1,
            "imported" => stats.n_cookies_imported += 1,
            "exported" => stats.n_cookies_exported += 1,
            _ => {}
        }
        Self::set_stats(&stats).await
    }
}

const CUSTOM_DESCS_KEY: &str = "custom_cookie_descriptions";

/// User-defined cookie descriptions, persisted in chrome.storage.local.
pub struct CustomDescriptions;

const MAX_CUSTOM_DESCRIPTIONS: usize = 500;

impl CustomDescriptions {
    pub async fn get_all() -> HashMap<String, String> {
        get(CUSTOM_DESCS_KEY, HashMap::new()).await
    }

    pub async fn save(descs: &HashMap<String, String>) -> Result<(), String> {
        if descs.len() > MAX_CUSTOM_DESCRIPTIONS {
            return Err(format!(
                "Custom descriptions limit reached ({}, max {})",
                descs.len(),
                MAX_CUSTOM_DESCRIPTIONS
            ));
        }
        set(CUSTOM_DESCS_KEY, descs).await
    }
}
