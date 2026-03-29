# Edit Cookie Malaccamax

> **Cookie editor for Chrome** — View, create, edit, delete, protect, and export browser cookies with a fast Rust/WebAssembly-powered interface.

[![License: GPL v3+](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/github/package-json/v/Malaccamaxgit/EditCookieMalaccamax?color=blue)](https://github.com/Malaccamaxgit/EditCookieMalaccamax)
[![Target: MV3](https://img.shields.io/badge/Target-Manifest%20V3-blue)](https://developer.chrome.com/docs/extensions/mv3/intro/)
[![Tests](https://img.shields.io/badge/Tests-29%20passing-brightgreen)](https://github.com/Malaccamaxgit/EditCookieMalaccamax)

---

## Features

- **Cookie CRUD** — Create, read, update, and delete cookies with full field control
- **Cookie Protection** — Mark cookies as read-only with automatic restoration on modification
- **Search & Filter** — Real-time search by name/domain, plus toggle filters for host-only, domain, Secure, HttpOnly, session, and persistent cookies
- **Cookie Count** — Live count of visible cookies, reflecting active search and filters
- **Scope Badges** — Visual indicators showing whether a cookie is host-only or domain-scoped
- **Security Badges** — Secure (lock), HttpOnly (shield), and expiration status at a glance
- **Export to File** — Save cookies via native Save As dialog (JSON, Netscape, or semicolon pairs)
- **Copy to Clipboard** — One-click copy in your preferred format
- **Built-in Descriptions** — 100+ well-known cookies identified (Google, Meta, Cloudflare, HubSpot, Stripe, AWS, etc.)
- **Custom Descriptions** — Add your own notes to any cookie, persisted across sessions
- **Duplicate Cookies** — Clone a cookie with one click
- **Dark & Light Themes** — System-aware theming with manual override
- **Context Menu** — Right-click to edit cookies for the current site
- **DevTools Panel** — Full cookie table view in Chrome DevTools
- **Open Source** — Built with Rust and WebAssembly; inspect every line of code

---

## Installation

### Users

1. Download the latest release from [GitHub Releases](https://github.com/Malaccamaxgit/EditCookieMalaccamax/releases)
2. Open Chrome and navigate to `chrome://extensions/`
3. Enable **Developer mode** (toggle in top-right)
4. Click **Load unpacked**
5. Select the `dist/chromium/` folder

### Development

```powershell
# Clone repository
git clone https://github.com/Malaccamaxgit/EditCookieMalaccamax.git
cd EditCookieMalaccamax

# Install npm dependencies (for Playwright tests)
npm install

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Oxichrome
cargo install oxichrome

# Build (includes post-build asset copy)
.\scripts\Build-Extension.ps1 -Release
```

---

## Usage

Click the extension icon to view and edit cookies for the current site.

### Cookie List

Each cookie row displays:
- **Name** and **description** (from built-in dictionary or user-defined)
- **Badges** — Scope (host/domain), Secure (lock), HttpOnly (shield), Expiration (clock + relative time)
- **Domain** (toggleable in settings)

### Cookie Actions

| Action | Icon | Description |
|--------|------|-------------|
| **Edit** | — | Click cookie row to expand and modify fields |
| **Save** | Floppy disk | Commit changes to Chrome |
| **Delete** | Trash | Remove cookie |
| **Protect** | Lock | Toggle read-only (auto-restores if modified) |
| **Copy** | Clipboard | Copy to clipboard in configured format |
| **Duplicate** | Copy | Create copy with `_copy` suffix |

### Cookie Descriptions

- **Built-in dictionary** covers 100+ common cookies (Google Analytics, Facebook Pixel, Cloudflare, HubSpot, LinkedIn, Workday, ServiceNow, SAP, Salesforce, Atlassian, AWS, Stripe, and more)
- **User-defined descriptions** can be added or edited via the pencil icon in the detail view
- Custom descriptions persist across sessions via `chrome.storage.local`
- A "Reset" button appears when a custom description overrides a built-in one

### Copy Formats

Configure in Settings → Appearance → Copy format:

| Format | Example |
|--------|---------|
| **JSON** | `{"name":"session","value":"abc123",...}` |
| **Netscape** | `.example.com	TRUE	/	TRUE	1735689600	session	abc123` |
| **Semicolon pairs** | `session=abc123` |

---

## Settings

Click the gear icon to open settings.

### Display Options

| Option | Description |
|--------|-------------|
| Show cookie domain | Display domain next to cookie name |
| Show domain before name | Place domain before name |
| Show command labels | Show text labels next to icons |

### Behavior

| Option | Description |
|--------|-------------|
| Show context menu | Add right-click menu option |
| Refresh after submit | Reload page after saving |
| Show confirmation alerts | Confirm before destructive actions |
| Skip cache refresh | Skip browser cache refresh on save |

### Cookie Age Limit

| Option | Description |
|--------|-------------|
| Limit cookie expiration | Auto-expire cookies after max age |
| Max age | Time limit (days or hours) |

### Appearance

| Option | Description |
|--------|-------------|
| Theme | Light / Dark |
| Sort cookies by | Domain, Name, or Expiration |
| Copy format | JSON / Netscape / Semicolon pairs |

---

## Permissions

| Permission | Purpose |
|------------|---------|
| `cookies` | Read, modify, delete cookies |
| `storage` | Save preferences, protected cookies, and custom descriptions |
| `contextMenus` | Add right-click menu option |
| `clipboardWrite` | Copy cookies to clipboard |
| `downloads` | Export cookies to file with Save As dialog |

---

## Project Structure

```
src/
├── lib.rs                  # Entry point, extension definition
├── chrome_api.rs           # Chrome API bindings (cookies, tabs, storage, downloads)
├── background/
│   ├── mod.rs              # Service worker setup
│   ├── cookie_monitor.rs   # onChanged listener, auto-restore, RAII guards
│   ├── context_menus.rs    # Right-click menu
│   └── devtools.rs         # DevTools panel registration
├── popup/
│   ├── mod.rs              # Main popup UI, theme, settings modal, filter logic
│   ├── cookie_list.rs      # Cookie list, detail view, actions, descriptions
│   ├── cookie_editor.rs    # Standalone editor form (reserved)
│   └── search.rs           # Search bar, filter toggles, cookie count
├── options/
│   ├── mod.rs              # Options page
│   └── preferences.rs      # Settings form
├── core/
│   ├── cookie.rs           # Cookie CRUD operations
│   ├── storage.rs          # Typed chrome.storage.local wrapper
│   └── rules.rs            # Filter matching, read-only checks, age clamping
├── shared/
│   ├── types.rs            # Cookie, Preferences, Filter, etc.
│   ├── helpers.rs          # Tab queries, theme, stylesheet injection
│   └── known_cookies.rs    # Built-in cookie description dictionary
└── devtools/
    └── mod.rs              # DevTools panel UI

public/
├── popup.html              # Popup shell
├── options.html            # Options shell
├── css/                    # Stylesheets (popup, options, devtools, Font Awesome)
├── webfonts/               # Font Awesome webfont files (woff2 + ttf)
├── devtools/               # DevTools HTML pages
├── js/                     # DevTools page loader
└── icons/                  # Extension icons (16–128px)

scripts/
├── Build-Extension.ps1     # Build + post-build asset copy
├── Verify-Build.ps1        # Automated build verification
└── secret-scanner.js       # Pre-commit secret detection

tests/
├── extension.spec.js       # Playwright E2E test suite (29 tests)
└── fixtures.js             # Playwright shared fixtures
```

---

## Testing

### Playwright E2E Tests

```powershell
npx playwright test
```

The test suite (29 tests) covers popup rendering, cookie CRUD, search/filter, theme toggle, settings modal, all action buttons (save, delete, lock, copy, duplicate), description editor, and badges.

### Linting

```powershell
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

---

## Documentation

- [PRIVACY.md](./PRIVACY.md) — Privacy policy (no data collection)
- [CHANGELOG.md](./CHANGELOG.md) — Version history
- [SECURITY.md](./SECURITY.md) — Security policy and vulnerability reporting
- [CONTRIBUTING.md](./CONTRIBUTING.md) — Contribution guidelines
- [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) — Community standards
- [docs/guides/DEVELOPER.md](./docs/guides/DEVELOPER.md) — Build, test, and verification procedures
- [docs/architecture/OVERVIEW.md](./docs/architecture/OVERVIEW.md) — Architecture overview

---

## Tech Stack

| Category | Technology |
|----------|-----------|
| **Language** | Rust (2024 edition) |
| **WASM Toolchain** | wasm-bindgen, wasm-bindgen-futures |
| **UI Framework** | Leptos 0.8 (CSR) |
| **Browser APIs** | web-sys, js-sys |
| **Build Tool** | Oxichrome |
| **Test Runner** | Playwright (29 tests) |
| **Target** | Chrome Manifest V3 |

---

## License

GNU General Public License v3.0 or later — see [LICENSE](./LICENSE) for details.

---

**Issues**: [GitHub Issues](https://github.com/Malaccamaxgit/EditCookieMalaccamax/issues)
**Discussions**: [GitHub Discussions](https://github.com/Malaccamaxgit/EditCookieMalaccamax/discussions)
