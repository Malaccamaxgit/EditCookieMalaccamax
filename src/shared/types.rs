//! Shared types for the cookie extension

use serde::{Deserialize, Serialize};

/// Cookie representation matching chrome.cookies.Cookie
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: Option<String>,
    pub expiration_date: Option<f64>,
    pub http_only: bool,
    pub secure: bool,
    pub same_site: Option<String>,
    pub store_id: Option<String>,
}

impl Cookie {
    /// Whether the cookie is past its expiration (session cookies are never expired here).
    #[allow(dead_code)]
    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expiration_date {
            let now = js_sys::Date::now() / 1000.0;
            exp < now
        } else {
            false // Session cookies are not expired
        }
    }
}

/// Filter/rule for blocking cookies
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub id: u64,
    pub domain_pattern: String,
    pub name_pattern: Option<String>,
    pub enabled: bool,
}

impl Filter {
    pub fn matches(&self, cookie: &Cookie) -> bool {
        let domain_matches = if let Some(rest) = self.domain_pattern.strip_prefix('*') {
            cookie.domain.ends_with(rest)
        } else if let Some(prefix) = self.domain_pattern.strip_suffix('*') {
            cookie.domain.starts_with(prefix)
        } else {
            cookie.domain == self.domain_pattern
        };

        if !domain_matches {
            return false;
        }

        if let Some(ref name_pattern) = self.name_pattern {
            if let Some(rest) = name_pattern.strip_prefix('*') {
                return cookie.name.ends_with(rest);
            } else if let Some(prefix) = name_pattern.strip_suffix('*') {
                return cookie.name.starts_with(prefix);
            }
            return cookie.name == *name_pattern;
        }

        true
    }
}

/// Read-only protected cookie
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadOnlyCookie {
    pub name: String,
    pub domain: String,
}

/// Extension preferences
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    #[serde(default = "default_false")]
    pub show_alerts: bool,
    #[serde(default = "default_false")]
    pub show_commands_labels: bool,
    #[serde(default = "default_true")]
    pub show_domain: bool,
    #[serde(default = "default_true")]
    pub show_domain_before_name: bool,
    #[serde(default = "default_false")]
    pub show_flag_and_delete_all: bool,
    #[serde(default = "default_true")]
    pub show_context_menu: bool,
    #[serde(default = "default_false")]
    pub refresh_after_submit: bool,
    #[serde(default = "default_true")]
    pub skip_cache_refresh: bool,
    #[serde(default = "default_false")]
    pub use_max_cookie_age: bool,
    #[serde(default)]
    pub max_cookie_age_type: i32,
    #[serde(default = "default_one")]
    pub max_cookie_age: u64,
    #[serde(default = "default_false")]
    pub use_custom_locale: bool,
    #[serde(default = "default_en")]
    pub custom_locale: String,
    #[serde(default = "default_json")]
    pub copy_cookies_type: String,
    #[serde(default = "default_domain_alpha")]
    pub sort_cookies_type: String,
    #[serde(default = "default_system")]
    pub theme: String,
    #[serde(default = "default_protected_cookies")]
    pub protected_cookies: Vec<ReadOnlyCookie>,
}

fn default_false() -> bool {
    false
}
fn default_true() -> bool {
    true
}
fn default_one() -> u64 {
    1
}
fn default_en() -> String {
    "en".to_string()
}
fn default_json() -> String {
    "json".to_string()
}
fn default_domain_alpha() -> String {
    "domain_alpha".to_string()
}
fn default_system() -> String {
    "light".to_string()
}
fn default_protected_cookies() -> Vec<ReadOnlyCookie> {
    Vec::new()
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            show_alerts: false,
            show_commands_labels: false,
            show_domain: true,
            show_domain_before_name: true,
            show_flag_and_delete_all: false,
            show_context_menu: true,
            refresh_after_submit: false,
            skip_cache_refresh: true,
            use_max_cookie_age: false,
            max_cookie_age_type: 0,
            max_cookie_age: 1,
            use_custom_locale: false,
            custom_locale: "en".to_string(),
            copy_cookies_type: "json".to_string(),
            sort_cookies_type: "domain_alpha".to_string(),
            theme: "light".to_string(),
            protected_cookies: default_protected_cookies(),
        }
    }
}

/// Extension statistics (for counters once `DataStorage` stats helpers are wired into the UI).
#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionStats {
    #[serde(default)]
    pub n_cookies_created: u64,
    #[serde(default)]
    pub n_cookies_changed: u64,
    #[serde(default)]
    pub n_cookies_deleted: u64,
    #[serde(default)]
    pub n_cookies_protected: u64,
    #[serde(default)]
    pub n_cookies_flagged: u64,
    #[serde(default)]
    pub n_cookies_shortened: u64,
    #[serde(default)]
    pub n_popup_clicked: u64,
    #[serde(default)]
    pub n_panel_clicked: u64,
    #[serde(default)]
    pub n_cookies_imported: u64,
    #[serde(default)]
    pub n_cookies_exported: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_stats_default_roundtrip() {
        let s = ExtensionStats::default();
        assert_eq!(s.n_cookies_created, 0);
    }
}
