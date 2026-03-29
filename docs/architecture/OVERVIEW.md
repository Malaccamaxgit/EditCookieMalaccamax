# Architecture Overview

> **Purpose** — High-level architecture documentation for **Edit Cookie Malaccamax** v0.2.0.

| Topic | Detail |
|--------|--------|
| Language / UI | Rust (edition **2024**), **Leptos 0.8** CSR |
| Extension tooling | **Oxichrome 0.2** |
| Release build | `.\scripts\Build-Extension.ps1 -Release` → **`dist/chromium/`** |
| Icons / styling | **Font Awesome 6.4.0** bundled locally (no CDN) |
| E2E | **Playwright** — 14 tests in `tests/extension.spec.js` |

---

## Extension Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Chrome Browser                        │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │   Popup     │  │   Options   │  │  DevTools   │     │
│  │  (Leptos)   │  │  (Leptos)   │  │   Panel     │     │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘     │
│         └────────────────┴────────────────┘             │
│                          │                              │
│                  ┌───────▼───────┐                      │
│                  │   Background  │                      │
│                  │  Service Worker                      │
│                  └───────┬───────┘                      │
│                          │                              │
│         ┌────────────────┼────────────────┐             │
│         │                │                │             │
│  ┌──────▼──────┐  ┌──────▼──────┐  ┌──────▼──────┐     │
│  │  Cookies    │  │   Storage   │  │   Context   │     │
│  │    API      │  │    API      │  │   Menus     │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
└─────────────────────────────────────────────────────────┘
```

---

## Component Overview

### Background service worker (`src/background/`)

| Module | Role |
|--------|------|
| `mod.rs` | Initialization, re-exports |
| `cookie_monitor.rs` | `onChanged` listener with RAII `SuppressGuard`, auto-restore for protected cookies, filter enforcement, age clamping |
| `context_menus.rs` | Right-click menu creation and click handling |
| `devtools.rs` | DevTools panel registration (minimal) |

### Popup UI (`src/popup/`)

| Module | Role |
|--------|------|
| `mod.rs` | Root `App`, `CookieListWithSearch`, theme toggle, settings modal, custom descriptions state |
| `cookie_list.rs` | `CookieRow` (description + badges), `CookieDetails` (save / delete / lock / copy / duplicate), description editor |
| `cookie_editor.rs` | Standalone editor form (reserved for future use) |
| `search.rs` | `SearchBar` component |

### Options page (`src/options/`)

| Module | Role |
|--------|------|
| `mod.rs` | Options `App` with theme application |
| `preferences.rs` | `PreferencesForm` with let-chain event handlers |

### Core (`src/core/`)

| Module | Role |
|--------|------|
| `cookie.rs` | Cookie CRUD: `get_all`, `get_all_for_url`, `set`, `remove`, `remove_by_name` |
| `storage.rs` | `PreferencesStorage`, `DataStorage` (filters, read-only, stats), `CustomDescriptions` |
| `rules.rs` | `matches_filter`, `is_read_only`, `maybe_shorten`, add/remove filter and read-only rules |

### Shared (`src/shared/`)

| Module | Role |
|--------|------|
| `types.rs` | `Cookie`, `Filter`, `ReadOnlyCookie`, `Preferences`, `ExtensionStats` (all `serde`) |
| `helpers.rs` | `get_active_tab`, `get_current_tab_url`, `apply_theme`, `ensure_stylesheets` |
| `known_cookies.rs` | Built-in dictionary (~100 cookies: Google, Meta, Microsoft, Cloudflare, HubSpot, LinkedIn, TikTok, Workday, ServiceNow, SAP, Salesforce, Atlassian, Zendesk, AWS, Stripe, Segment, etc.) |

### Chrome API (`src/chrome_api.rs`)

Single module with `wasm-bindgen` extern bindings:

- `CookieDetails`, `SetCookieDetails` (including host-only domain handling), `RemoveCookieDetails` builders  
- `js_value_to_cookie` / `js_value_to_cookies` conversion helpers  
- SameSite validation: `no_restriction`, `lax`, `strict`, `unspecified`

### DevTools (`src/devtools/mod.rs`)

Sortable cookie table panel.

---

## Data flow

### Cookie loading

Popup → `helpers::get_current_tab_url` → `core::cookie::get_all_for_url` → `chrome.cookies.getAll` → Leptos signals drive the list.

### Cookie editing

User edits fields → Save → `core::cookie::set` → `chrome.cookies.set` → list reload.

### Protected cookie enforcement

Background `onChanged` → `rules::is_read_only` → `SuppressGuard` → `cookie::set` restores the original value when a protected cookie is altered externally.

### Cookie descriptions

Resolve text from `known_cookies` (built-in), then apply overrides from `CustomDescriptions` in `chrome.storage.local`. Users can edit or reset via the inline editor in the popup.

---

## Storage schema

### Key `preferences`

```rust
struct Preferences {
    show_alerts: bool,
    show_commands_labels: bool,
    show_domain: bool,
    show_domain_before_name: bool,
    show_flag_and_delete_all: bool,
    show_context_menu: bool,
    refresh_after_submit: bool,
    skip_cache_refresh: bool,
    use_max_cookie_age: bool,
    max_cookie_age_type: i32,     // 0 = days, 1 = hours
    max_cookie_age: u64,
    use_custom_locale: bool,
    custom_locale: String,
    copy_cookies_type: String,    // "json" | "netscape" | "semicolon"
    sort_cookies_type: String,    // "domain_alpha" | "name_alpha" | "expiration"
    theme: String,                // "light" | "dark"
    protected_cookies: Vec<ReadOnlyCookie>,
}
```

`ReadOnlyCookie` is a name/domain pair used for protection; see `shared::types`.

### Key `custom_cookie_descriptions`

```rust
HashMap<String, String>  // cookie_name → user-defined description
```

### Other keys

| Key | Contents |
|-----|----------|
| `data_filters` | Filter rules |
| `data_readOnly` | Read-only list |
| `stats` | Usage counters |

---

## Build output (`dist/chromium/`)

```
dist/chromium/
├── manifest.json           # Generated by Oxichrome
├── background.js           # Generated service worker loader
├── popup.html + popup.js   # Generated (minimal HTML; JS loads WASM)
├── options.html + options.js
├── css/                    # Copied from public/css/
│   ├── popup.css, options.css, devtools.css
│   └── fontawesome.min.css # Copied by Build-Extension.ps1
├── webfonts/               # Copied by Build-Extension.ps1
├── icons/                  # Copied by Oxichrome
├── devtools/               # Copied from public/devtools/
├── js/                     # Copied by Build-Extension.ps1
└── wasm/                   # Generated by wasm-bindgen
    ├── edit_cookie_malaccamax_bg.wasm
    └── edit_cookie_malaccamax.js
```

---

## Permissions

`tabs`, `cookies`, `contextMenus`, `storage`, `clipboardWrite`

---

## Security

| Area | Approach |
|------|----------|
| Memory safety | Rust ownership prevents use-after-free and double-free |
| Script execution | No `eval()`; CSP: `script-src 'self' 'wasm-unsafe-eval'; object-src 'self'` |
| Supply chain / privacy | Font Awesome bundled locally — no external font or icon CDN requests |
| Monitor robustness | RAII `SuppressGuard` avoids leaving the monitor permanently disabled on panic |
| Chrome API | SameSite values validated before calls into `chrome.cookies` |
| Input limits | Description input capped at 500 characters; duplicate cookie names bounded to 255 characters |
| DOM / WASM | No `.unwrap()` on event targets (reduces WASM panics from invalid targets) |
| Logging | Cookie values are never logged; names and domains only |

---

## Testing

| Layer | Command / location |
|-------|---------------------|
| Unit tests | `cargo test` (Rust host target) |
| E2E | `npx playwright test` — 14 tests: popup, CRUD, search, theme, settings, action buttons, description editor, badges |
| Lint (WASM) | `cargo clippy --target wasm32-unknown-unknown -- -D warnings` |

---

## Design decisions

1. **Rust + WebAssembly** — Memory and type safety; one primary language across the extension.  
2. **Leptos 0.8** — Fine-grained reactivity with signals for popup/options UI.  
3. **Oxichrome** — Proc macros for entry points and automatic manifest generation.  
4. **Settings as a modal** — Faster access without leaving the popup context.  
5. **Protected cookies as an array** — Simple name/domain pairs stored in preferences.  
6. **Local Font Awesome** — Removes CDN supply-chain risk and supports offline use.  
7. **RAII guards** — Panic-safe re-entrancy control in the cookie monitor.  
8. **Dynamic stylesheet injection** — Compensates for Oxichrome’s minimal generated HTML so themes and FA apply correctly.

---

**Last updated:** March 2026
