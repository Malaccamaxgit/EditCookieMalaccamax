# Edit Cookie Malaccamax

Cookie editor extension for Chrome. Built with Rust and WebAssembly.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/github/package-json/v/Malaccamaxgit/EditCookieMalaccamax?color=blue)](https://github.com/Malaccamaxgit/EditCookieMalaccamax)
[![Target: MV3](https://img.shields.io/badge/Target-Manifest%20V3-blue)](https://developer.chrome.com/docs/extensions/mv3/intro/)

---

## Features

- Create, read, update, and delete cookies
- Protect cookies from modification (read-only with auto-restore)
- Built-in descriptions for 100+ well-known cookies (Google, Meta, Cloudflare, etc.)
- User-editable cookie descriptions that persist across sessions
- Visual badges for Secure, HttpOnly, and expiration status
- Export cookies as JSON, Netscape format, or semicolon pairs
- Filter cookies by domain/name patterns
- Dark and light theme support
- Search and sort cookies
- Context menu integration
- Duplicate cookies with one click
- DevTools panel for full cookie table view
- Playwright end-to-end test suite

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
- **Badges** — Secure (lock icon), HttpOnly (shield icon), Expiration (clock + relative time)
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
├── chrome_api.rs           # Chrome API bindings (cookies, tabs, storage, etc.)
├── background/
│   ├── mod.rs              # Service worker setup
│   ├── cookie_monitor.rs   # onChanged listener, auto-restore, RAII guards
│   ├── context_menus.rs    # Right-click menu
│   └── devtools.rs         # DevTools panel registration
├── popup/
│   ├── mod.rs              # Main popup UI, theme, settings modal
│   ├── cookie_list.rs      # Cookie list, detail view, actions, descriptions
│   ├── cookie_editor.rs    # Standalone editor form (reserved)
│   └── search.rs           # Search bar component
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
├── css/
│   ├── popup.css           # Popup styles (light + dark theme)
│   ├── options.css         # Options styles
│   ├── devtools.css        # DevTools panel styles
│   └── fontawesome.min.css # Font Awesome 6.4.0 (bundled locally)
├── webfonts/               # Font Awesome webfont files (woff2 + ttf)
├── devtools/               # DevTools HTML pages
├── js/                     # DevTools page loader
└── icons/                  # Extension icons (16–128px)

scripts/
├── Build-Extension.ps1     # Build + post-build asset copy
└── Verify-Build.ps1        # Automated build verification

tests/
├── extension.spec.js       # Playwright E2E test suite (14 tests)
└── fixtures.js             # Playwright shared fixtures
```

---

## Testing

### Playwright E2E Tests

```powershell
npx playwright test
```

The test suite covers popup rendering, cookie CRUD, search/filter, theme toggle, settings modal, all action buttons (save, delete, lock, copy, duplicate), description editor, and badges.

### Unit Tests

```powershell
cargo test
```

### Linting

```powershell
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

---

## Documentation

### Users

- [README.md](./README.md) — Features, installation, usage
- [PRIVACY.md](./PRIVACY.md) — Privacy policy
- [CHANGELOG.md](./CHANGELOG.md) — Version history

### Developers

- [docs/guides/DEVELOPER.md](./docs/guides/DEVELOPER.md) — Build, test, and verification procedures
- [docs/guides/CLAUDE.md](./docs/guides/CLAUDE.md) — Quick reference for AI assistants
- [docs/architecture/OVERVIEW.md](./docs/architecture/OVERVIEW.md) — Architecture overview
- [SECURITY.md](./SECURITY.md) — Security policy and vulnerability reporting
- [CONTRIBUTING.md](./CONTRIBUTING.md) — Contribution guidelines

### Scripts

- [scripts/Build-Extension.ps1](./scripts/Build-Extension.ps1) — Build + post-build asset copy
- [scripts/Verify-Build.ps1](./scripts/Verify-Build.ps1) — Automated build verification

---

## License

GNU General Public License v3.0 — see [LICENSE](./LICENSE) for details.

---

**Issues**: [GitHub Issues](https://github.com/Malaccamaxgit/EditCookieMalaccamax/issues)
**Discussions**: [GitHub Discussions](https://github.com/Malaccamaxgit/EditCookieMalaccamax/discussions)
