# Developer Handbook — Edit Cookie Malaccamax

> **Purpose** — Build, test, verify, and navigate the codebase for **Edit Cookie Malaccamax** (v0.2.0).

This handbook describes how to set up the environment, produce a loadable Chromium extension, run tests, and follow project conventions.

**Project facts**

| Item | Value |
|------|--------|
| Version | 0.1.6 |
| Rust edition | 2024 |
| Extension output | `dist/chromium/` (load this folder in Chrome) |
| Framework | Oxichrome 0.2, Leptos 0.8 (CSR), wasm-bindgen 0.2 |
| Font Awesome | 6.4.0, bundled locally (no CDN) |
| Theme modes | Light and Dark only (no System / follow-OS option) |

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Development Environment](#development-environment)
3. [Build Process](#build-process)
4. [Testing](#testing)
5. [Verification Checklist](#verification-checklist)
6. [Code Organization](#code-organization)
7. [Signal/Reactivity Patterns](#signalreactivity-patterns)
8. [Chrome APIs Used](#chrome-apis-used)
9. [Troubleshooting](#troubleshooting)
10. [Common Tasks](#common-tasks)

---

## Quick Start

```powershell
# From the repository root (example path)
cd E:\Github\EditCookieMalaccamax

# Recommended: full automated verification (release build + checks)
.\scripts\Verify-Build.ps1 -Release

# Standard release build (wraps Oxichrome and copies assets — do not use plain cargo --release alone)
.\scripts\Build-Extension.ps1 -Release

# Unit tests
cargo test

# Lint (warnings denied)
cargo clippy --target wasm32-unknown-unknown -- -D warnings

# E2E (Playwright, 14 tests in tests/extension.spec.js)
npx playwright test
```

**Load in Chrome**

1. Open `chrome://extensions/`
2. Enable **Developer mode**
3. **Load unpacked** → select **`dist/chromium/`** (not `dist/` alone)

---

## Development Environment

### Required Tools

| Tool | Purpose | Notes |
|------|---------|--------|
| Rust (2024 edition) | Compile the extension | Use `rustup`; match project toolchain if pinned |
| `wasm32-unknown-unknown` | WASM target | `rustup target add wasm32-unknown-unknown` |
| Oxichrome CLI | Extension build | `cargo install oxichrome` (invoked as `cargo oxichrome …`) |
| Node.js | Playwright E2E | For `npx playwright test` |
| Git | Version control | — |

### Verify Installation

```powershell
rustc --version
rustup target list --installed
cargo oxichrome --version
```

### Dependencies (Rust)

Core crates used in the extension: **oxichrome**, **leptos**, **wasm-bindgen**, **wasm-bindgen-futures**, **serde**, **serde_json**, **serde-wasm-bindgen**, **web-sys**, **js-sys**. There is **no `chrono`** dependency.

### Permissions (manifest)

The extension uses: **tabs**, **cookies**, **contextMenus**, **storage**, **clipboardWrite**. It does **not** use `unlimitedStorage`.

---

## Build Process

### Standard Build

Use the project script so Oxichrome output is post-processed (Font Awesome, webfonts, DevTools HTML/JS):

```powershell
# Release (typical for manual testing and packing)
.\scripts\Build-Extension.ps1 -Release
```

This runs **`cargo oxichrome build --release`**, then copies from `public/` into `dist/chromium/`:

- Font Awesome CSS (`css/fontawesome.min.css`)
- Font Awesome **webfonts/**
- DevTools **panel.html** and **js/devtools-page.js** (refreshed so stale copies are not left in `dist`)

Do **not** rely on `cargo build --release` alone for a complete, loadable extension; the script is the supported build entry point.

### Build Output (`dist/chromium/`)

```
dist/chromium/
├── manifest.json
├── background.js
├── popup.html + popup.js
├── options.html + options.js
├── css/
│   ├── popup.css
│   ├── options.css
│   ├── devtools.css
│   └── fontawesome.min.css
├── webfonts/               # Font Awesome files
├── icons/
├── devtools/               # DevTools HTML (e.g. panel)
├── js/                     # DevTools page loader
└── wasm/
    ├── edit_cookie_malaccamax_bg.wasm
    └── edit_cookie_malaccamax.js
```

### Automated Verification

```powershell
.\scripts\Verify-Build.ps1 -Release      # full verification
.\scripts\Verify-Build.ps1 -SkipTests    # build checks, skip tests
.\scripts\Verify-Build.ps1 -Verbose
```

The verify script checks toolchain, formatting, tests, build success, expected artifacts under `dist/chromium/`, and WASM sizes (see script for details).

### Common Build Errors

| Symptom | What to do |
|---------|------------|
| `cargo` / `oxichrome` not found | Ensure `~\.cargo\bin` is on `PATH`; install Oxichrome if needed |
| Missing WASM target | `rustup target add wasm32-unknown-unknown` |
| Extension loads but assets missing | Run `.\scripts\Build-Extension.ps1 -Release` (not raw `cargo build`) |
| Locked files on Windows | Close Chrome tabs using the extension; remove `dist/chromium` if needed and rebuild |

### Security Notes (Build & Supply Chain)

- Font Awesome is **bundled locally**; the UI does not load icon fonts from a CDN.
- Same-origin / cookie safety details are enforced in code (see [Chrome APIs Used](#chrome-apis-used) and `cookie_monitor.rs`).

---

## Testing

### Unit Tests

```powershell
cargo test
cargo test test_name -- --nocapture
```

Relevant areas: `src/core/cookie.rs`, `src/core/storage.rs`, `src/core/rules.rs`, `src/shared/helpers.rs`, and other modules as tests are added.

### End-to-End Tests (Playwright)

```powershell
npx playwright test
```

- Spec file: **`tests/extension.spec.js`**
- **14** tests covering extension behavior in a real browser profile

### Manual Smoke Checks

After loading `dist/chromium/`:

1. Popup opens and lists cookies for the active site
2. CRUD: edit, add, delete cookies
3. Search / filter and sort (domain, name, expiration)
4. Theme: **Light** / **Dark** only
5. Settings modal and preferences persistence
6. Copy to clipboard (JSON, Netscape, semicolon)
7. Protected (read-only) cookies and background behavior
8. Context menu entry
9. DevTools panel

---

## Verification Checklist

### Pre-Commit

- [ ] `.\scripts\Build-Extension.ps1 -Release` succeeds
- [ ] `cargo test` passes
- [ ] `cargo fmt` applied
- [ ] `cargo clippy --target wasm32-unknown-unknown -- -D warnings` passes
- [ ] Docs updated when behavior or paths change

### Pre-Release / Manual QA

- [ ] Load **`dist/chromium/`** — no extension errors
- [ ] Popup, options, DevTools: no unexpected console errors
- [ ] CRUD, search, sort, themes, settings, copy, protected cookies, context menu
- [ ] `npx playwright test` passes

### Security-Related Checks

- [ ] No new external script or font CDNs; FA remains under `public/` → `dist/chromium/`
- [ ] Cookie set paths still validate **SameSite** before `chrome.cookies.set`
- [ ] Description length remains capped (**500** chars); duplicate display name bounded (**255** chars)
- [ ] Event handlers avoid `.unwrap()` on DOM targets; use safe `if let` / pattern matching

**Known Issues:** None currently.

---

## Code Organization

### Source Tree (`src/`)

```
src/
├── lib.rs                  # Entry point, #[oxichrome::extension] macro
├── chrome_api.rs           # wasm-bindgen bindings: cookies, tabs, storage, menus
├── background/
│   ├── mod.rs              # Service worker init, re-exports
│   ├── cookie_monitor.rs   # onChanged listener, RAII SuppressGuard, auto-restore
│   ├── context_menus.rs    # Right-click menu
│   └── devtools.rs         # DevTools registration
├── popup/
│   ├── mod.rs              # Popup App + CookieListWithSearch
│   ├── cookie_list.rs      # CookieRow, CookieDetails, actions, description editor
│   ├── cookie_editor.rs    # Standalone editor form (reserved, not currently mounted)
│   └── search.rs           # SearchBar component
├── options/
│   ├── mod.rs              # Options page App
│   └── preferences.rs      # PreferencesForm
├── core/
│   ├── mod.rs              # Module exports
│   ├── cookie.rs           # Cookie CRUD: get_all, get_all_for_url, set, remove
│   ├── storage.rs          # PreferencesStorage, DataStorage, CustomDescriptions
│   └── rules.rs            # Filter matching, is_read_only, maybe_shorten
├── shared/
│   ├── mod.rs              # Module exports
│   ├── types.rs            # Cookie, Preferences, Filter, ReadOnlyCookie, ExtensionStats
│   ├── helpers.rs          # get_active_tab, get_current_tab_url, apply_theme, ensure_stylesheets
│   └── known_cookies.rs    # Built-in cookie description dictionary (~100 entries)
└── devtools/
    └── mod.rs              # DevTools panel UI
```

### Features (v0.2.0)

- Cookie CRUD
- Protected cookies (read-only with auto-restore in the background worker)
- Copy to clipboard: JSON, Netscape, semicolon-separated
- Search / filter; sort by domain, name, expiration
- Light / Dark theme; settings modal
- Cookie age limit enforcement
- Context menu integration
- DevTools panel
- Built-in cookie descriptions (**100+**) plus user-editable descriptions (stored in `chrome.storage.local`)
- Visual badges (Secure, HttpOnly, expiration)
- Duplicate cookie handling
- Playwright E2E suite (**14** tests)

### Scripts

| Script | Role |
|--------|------|
| `scripts/Build-Extension.ps1` | Oxichrome build + post-build asset copy to `dist/chromium/` |
| `scripts/Verify-Build.ps1` | Automated build verification |

---

## Signal/Reactivity Patterns

### `get_untracked()` in async and long-lived closures

When a signal’s value is read inside a closure that is later run asynchronously (or where you must not create a stale reactive dependency), use **`get_untracked()`** instead of **`get()`**.

```rust
// Avoid: .get() can tie the closure to reactive updates incorrectly for async work
// Prefer:
let value = signal.get_untracked();
```

### Rust 2024: collapsed `let` chains

Edition **2024** expects **collapsed `let`-chains** — avoid nested `if let` where the edition style guide prefers a single chained form. Match surrounding code and compiler hints.

### Stylesheets at runtime

Oxichrome emits minimal HTML without linked project CSS. **`ensure_stylesheets()`** (in `shared/helpers.rs`) injects the correct stylesheets at runtime so popup, options, and DevTools match the bundled `dist/chromium/css/` layout.

### Host-only cookies and `chrome.cookies.set`

For **host-only** cookies, **omit the `domain` field** in `chrome.cookies.set` when appropriate so Chrome does not create an extra domain-scoped cookie alongside the host-only one.

---

## Chrome APIs Used

Bindings live in **`src/chrome_api.rs`** (wasm-bindgen to `chrome.*`).

### `cookies`

- `getAll`, `set`, `remove`, `onChanged`
- **SameSite** is validated before calling `set`
- Protected cookies: background **`cookie_monitor.rs`** uses a **RAII `SuppressGuard`** to avoid feedback loops while applying auto-restore

### `storage`

- `storage.local` for preferences and custom descriptions

### `tabs`

- Query active / current tab for URL-scoped cookie operations

### `contextMenus`

- Create items and handle clicks (see `background/context_menus.rs`)

### Clipboard

- Copy formats use **`clipboardWrite`** as declared in the manifest

---

## Troubleshooting

| Problem | What to check |
|---------|----------------|
| Blank popup or missing styles | Console errors; confirm `ensure_stylesheets` runs; rebuild with `Build-Extension.ps1` |
| Wrong or empty `dist/` layout | Load **`dist/chromium/`**; run the build script, not only `cargo build` |
| Clippy / WASM | Use `--target wasm32-unknown-unknown` as in the lint command above |
| Stale UI values in handlers | Replace `.get()` with `.get_untracked()` where the closure captures state for async work |
| Cookies doubling after set | Host-only vs domain cookies: omit `domain` when setting host-only cookies |
| Playwright failures | Extension path must point at built `dist/chromium/`; see Playwright config |

---

## Common Tasks

### Add a preference

1. Extend **`Preferences`** in `src/shared/types.rs` (and `Default`).
2. Wire UI in **`src/options/preferences.rs`** and persistence via **`src/core/storage.rs`** as needed.

### Add a popup action

1. UI in **`src/popup/cookie_list.rs`** (or related popup modules).
2. Call into **`src/core/cookie.rs`** or **`chrome_api.rs`** as appropriate.
3. Add or extend **unit tests** and, if user-visible, **Playwright** coverage.

### Adjust popup layout / theme

Edit source styles under **`public/css/`** (e.g. `popup.css`); rebuild with **`Build-Extension.ps1`** so `dist/chromium/css/` updates.

### Add another clipboard format

1. Implement in the copy path in **`cookie_list.rs`**.
2. Extend any **`CopyFormat`** (or equivalent) in **`src/shared/types.rs`** and expose in preferences if required.

### Debug WASM artifact

```powershell
# After release build
Get-Item .\dist\chromium\wasm\edit_cookie_malaccamax_bg.wasm
# Optional: wasm-decompile or similar tools on that path
```

### Inspect stored data

In the popup or options devtools console:

```javascript
chrome.storage.local.get(null, console.log);
```

### Reload the unpacked extension

`chrome://extensions/` → your extension → **Reload** → reopen popup / DevTools.

---

## Contact

- **Email:** [benjamin.alloul@gmail.com](mailto:benjamin.alloul@gmail.com)
- **Issues:** [GitHub Issues](https://github.com/Malaccamaxgit/EditCookieMalaccamax/issues)

**Last Updated:** March 2026
