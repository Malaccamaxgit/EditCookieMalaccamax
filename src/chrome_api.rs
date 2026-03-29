//! Raw Chrome API bindings via wasm-bindgen

use js_sys::{Array, Function, Object};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ============================================================================
// Chrome Cookies API
// ============================================================================

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["chrome", "cookies"])]
    pub async fn get(details: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "cookies"])]
    pub async fn getAll(details: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "cookies"])]
    pub async fn set(details: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "cookies"])]
    pub async fn remove(details: JsValue) -> Result<JsValue, JsValue>;

    // Note: onChanged is an event, accessed via js_name attribute
    #[wasm_bindgen(thread_local_v2, js_namespace = ["chrome", "cookies"], js_name = onChanged)]
    pub static ON_CHANGED: JsValue;

    #[wasm_bindgen(js_namespace = ["chrome", "contextMenus"], js_name = create)]
    pub fn create_menu_item(createProperties: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["chrome", "contextMenus"])]
    pub fn remove_menu_item(menuItemId: JsValue);

    #[wasm_bindgen(thread_local_v2, js_namespace = ["chrome", "contextMenus"], js_name = onClicked)]
    pub static ON_CLICKED: JsValue;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "tabs"])]
    pub async fn query(query_info: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "tabs"])]
    pub async fn create(create_properties: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "tabs"])]
    pub async fn update(tab_id: JsValue, update_properties: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "action"])]
    pub async fn setIcon(details: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "downloads"])]
    pub async fn download(options: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "devtools", "panels"])]
    pub async fn create_panel(title: &str, icon_path: &str, page_path: &str) -> Result<JsValue, JsValue>;
}

/// Access the chrome.cookies.onChanged event
pub fn on_cookie_changed() -> Function {
    ON_CHANGED.with(|v| v.clone().unchecked_into())
}

/// Access the chrome.contextMenus.onClicked event
pub fn on_context_menu_clicked() -> Function {
    ON_CLICKED.with(|v| v.clone().unchecked_into())
}

// ============================================================================
// Cookie Details Builders
// ============================================================================

#[derive(Default)]
pub struct CookieDetails {
    pub url: Option<String>,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: Option<bool>,
    pub session: Option<bool>,
    pub store_id: Option<String>,
}

impl CookieDetails {
    pub fn to_js(&self) -> JsValue {
        let obj = Object::new();
        if let Some(ref url) = self.url {
            js_sys::Reflect::set(&obj, &JsValue::from("url"), &JsValue::from(url)).ok();
        }
        if let Some(ref name) = self.name {
            js_sys::Reflect::set(&obj, &JsValue::from("name"), &JsValue::from(name)).ok();
        }
        if let Some(ref domain) = self.domain {
            js_sys::Reflect::set(&obj, &JsValue::from("domain"), &JsValue::from(domain)).ok();
        }
        if let Some(ref path) = self.path {
            js_sys::Reflect::set(&obj, &JsValue::from("path"), &JsValue::from(path)).ok();
        }
        if let Some(secure) = self.secure {
            js_sys::Reflect::set(&obj, &JsValue::from("secure"), &JsValue::from(secure)).ok();
        }
        if let Some(session) = self.session {
            js_sys::Reflect::set(&obj, &JsValue::from("session"), &JsValue::from(session)).ok();
        }
        if let Some(ref store_id) = self.store_id {
            js_sys::Reflect::set(&obj, &JsValue::from("storeId"), &JsValue::from(store_id)).ok();
        }
        obj.into()
    }
}

pub struct SetCookieDetails {
    pub url: String,
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: Option<bool>,
    pub http_only: Option<bool>,
    pub expiration_date: Option<f64>,
    pub same_site: Option<String>,
    pub store_id: Option<String>,
}

impl SetCookieDetails {
    pub fn to_js(&self) -> JsValue {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from("url"), &JsValue::from(&self.url)).ok();
        js_sys::Reflect::set(&obj, &JsValue::from("name"), &JsValue::from(&self.name)).ok();
        js_sys::Reflect::set(&obj, &JsValue::from("value"), &JsValue::from(&self.value)).ok();
        if let Some(ref domain) = self.domain {
            // Only pass domain for domain-cookies (dot prefix). For host-only
            // cookies, omit the field so Chrome infers it from the URL —
            // passing a bare hostname makes Chrome create a *new* domain cookie.
            if domain.starts_with('.') {
                js_sys::Reflect::set(&obj, &JsValue::from("domain"), &JsValue::from(domain))
                    .ok();
            }
        }
        if let Some(ref path) = self.path {
            js_sys::Reflect::set(&obj, &JsValue::from("path"), &JsValue::from(path)).ok();
        }
        if let Some(secure) = self.secure {
            js_sys::Reflect::set(&obj, &JsValue::from("secure"), &JsValue::from(secure)).ok();
        }
        if let Some(http_only) = self.http_only {
            js_sys::Reflect::set(&obj, &JsValue::from("httpOnly"), &JsValue::from(http_only)).ok();
        }
        if let Some(exp) = self.expiration_date {
            js_sys::Reflect::set(
                &obj,
                &JsValue::from("expirationDate"),
                &JsValue::from_f64(exp),
            )
            .ok();
        }
        if let Some(ref same_site) = self.same_site {
            let valid = matches!(same_site.as_str(), "no_restriction" | "lax" | "strict" | "unspecified");
            if valid {
                js_sys::Reflect::set(&obj, &JsValue::from("sameSite"), &JsValue::from(same_site)).ok();
            }
        }
        if let Some(ref store_id) = self.store_id {
            js_sys::Reflect::set(&obj, &JsValue::from("storeId"), &JsValue::from(store_id)).ok();
        }
        obj.into()
    }
}

pub struct RemoveCookieDetails {
    pub url: String,
    pub name: String,
    pub store_id: Option<String>,
}

impl RemoveCookieDetails {
    pub fn to_js(&self) -> JsValue {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from("url"), &JsValue::from(&self.url)).ok();
        js_sys::Reflect::set(&obj, &JsValue::from("name"), &JsValue::from(&self.name)).ok();
        if let Some(ref store_id) = self.store_id {
            js_sys::Reflect::set(&obj, &JsValue::from("storeId"), &JsValue::from(store_id)).ok();
        }
        obj.into()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

pub fn array_to_vec(value: JsValue) -> Vec<JsValue> {
    if let Ok(array) = value.dyn_into::<Array>() {
        array.iter().collect()
    } else {
        vec![]
    }
}

pub fn js_value_to_cookie(value: JsValue) -> Option<crate::shared::types::Cookie> {
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RawCookie {
        name: String,
        value: String,
        domain: String,
        path: Option<String>,
        expiration_date: Option<f64>,
        http_only: Option<bool>,
        secure: Option<bool>,
        same_site: Option<String>,
        store_id: Option<String>,
    }

    serde_wasm_bindgen::from_value::<RawCookie>(value)
        .ok()
        .map(|c| crate::shared::types::Cookie {
            name: c.name,
            value: c.value,
            domain: c.domain,
            path: c.path,
            expiration_date: c.expiration_date,
            http_only: c.http_only.unwrap_or(false),
            secure: c.secure.unwrap_or(false),
            same_site: c.same_site,
            store_id: c.store_id,
        })
}

pub fn js_value_to_cookies(value: JsValue) -> Vec<crate::shared::types::Cookie> {
    array_to_vec(value)
        .into_iter()
        .filter_map(js_value_to_cookie)
        .collect()
}
