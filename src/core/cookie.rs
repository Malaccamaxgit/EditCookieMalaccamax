//! Cookie operations wrapper using raw Chrome API

use crate::chrome_api;
use crate::shared::types::Cookie;
use js_sys::Object;
use wasm_bindgen::prelude::*;

/// Build a URL from a cookie's domain and path.
/// Uses `https://` for secure cookies, `http://` otherwise.
fn build_cookie_url(cookie: &Cookie) -> String {
    let clean_domain = cookie.domain.strip_prefix('.').unwrap_or(&cookie.domain);
    let path = cookie.path.as_deref().unwrap_or("/");
    let scheme = if cookie.secure { "https" } else { "http" };
    format!("{}://{}{}", scheme, clean_domain, path)
}

/// Get all cookies (no filter). Used by the DevTools panel.
#[allow(dead_code)]
pub async fn get_all() -> Result<Vec<Cookie>, JsValue> {
    let details = Object::new();
    let result = chrome_api::getAll(details.into()).await?;
    Ok(chrome_api::js_value_to_cookies(result))
}

/// Get all cookies for a specific URL
pub async fn get_all_for_url(url: &str) -> Result<Vec<Cookie>, JsValue> {
    let details = chrome_api::CookieDetails {
        url: Some(url.to_string()),
        ..Default::default()
    };
    let result = chrome_api::getAll(details.to_js()).await?;
    Ok(chrome_api::js_value_to_cookies(result))
}

const MAX_COOKIE_NAME_LEN: usize = 4096;
const MAX_COOKIE_VALUE_LEN: usize = 4096;
const MAX_COOKIE_DOMAIN_LEN: usize = 253;
const MAX_COOKIE_PATH_LEN: usize = 1024;

fn validate_cookie_fields(cookie: &Cookie) -> Result<(), JsValue> {
    if cookie.name.len() > MAX_COOKIE_NAME_LEN {
        return Err(JsValue::from_str(&format!(
            "Cookie name too long ({} bytes, max {})",
            cookie.name.len(),
            MAX_COOKIE_NAME_LEN
        )));
    }
    if cookie.value.len() > MAX_COOKIE_VALUE_LEN {
        return Err(JsValue::from_str(&format!(
            "Cookie value too long ({} bytes, max {})",
            cookie.value.len(),
            MAX_COOKIE_VALUE_LEN
        )));
    }
    if cookie.domain.len() > MAX_COOKIE_DOMAIN_LEN {
        return Err(JsValue::from_str(&format!(
            "Cookie domain too long ({} chars, max {})",
            cookie.domain.len(),
            MAX_COOKIE_DOMAIN_LEN
        )));
    }
    if let Some(ref path) = cookie.path
        && path.len() > MAX_COOKIE_PATH_LEN
    {
        return Err(JsValue::from_str(&format!(
            "Cookie path too long ({} chars, max {})",
            path.len(),
            MAX_COOKIE_PATH_LEN
        )));
    }
    Ok(())
}

/// Set a cookie
pub async fn set(cookie: &Cookie) -> Result<Cookie, JsValue> {
    validate_cookie_fields(cookie)?;
    let url = build_cookie_url(cookie);

    let details = chrome_api::SetCookieDetails {
        url,
        name: cookie.name.clone(),
        value: cookie.value.clone(),
        domain: Some(cookie.domain.clone()),
        path: cookie.path.clone(),
        secure: Some(cookie.secure),
        http_only: Some(cookie.http_only),
        expiration_date: cookie.expiration_date,
        same_site: cookie.same_site.clone(),
        store_id: cookie.store_id.clone(),
    };

    let result = chrome_api::set(details.to_js()).await?;
    chrome_api::js_value_to_cookie(result)
        .ok_or_else(|| JsValue::from_str("Failed to set cookie"))
}

/// Remove a cookie
pub async fn remove(cookie: &Cookie) -> Result<(), JsValue> {
    let url = build_cookie_url(cookie);

    let details = chrome_api::RemoveCookieDetails {
        url,
        name: cookie.name.clone(),
        store_id: cookie.store_id.clone(),
    };

    let result = chrome_api::remove(details.to_js()).await?;
    if result.is_null() || result.is_undefined() {
        return Err(JsValue::from_str("chrome.cookies.remove returned null — cookie not found or permission denied"));
    }
    Ok(())
}

/// Remove a cookie by name and URL
pub async fn remove_by_name(url: &str, name: &str) -> Result<(), JsValue> {
    let details = chrome_api::RemoveCookieDetails {
        url: url.to_string(),
        name: name.to_string(),
        store_id: None,
    };

    let result = chrome_api::remove(details.to_js()).await?;
    if result.is_null() || result.is_undefined() {
        return Err(JsValue::from_str("chrome.cookies.remove returned null — cookie not found or permission denied"));
    }
    Ok(())
}
