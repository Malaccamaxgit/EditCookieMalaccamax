# Changelog

> **Notable changes** — Version history following [Keep a Changelog](https://keepachangelog.com/).

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
Versioning: [Semantic Versioning](https://semver.org/)

## [0.1.6] — 2026-03-29

### Added

- **Save As dialog** — File export now uses `chrome.downloads` API with `saveAs: true` for native file picker instead of auto-downloading
- **Cookie attribute filter toggles** — Row of toggle buttons below search bar to filter by host-only, domain, secure, HTTP-only, session, or persistent
- **Cookie count badge** — Live count of visible cookies displayed in the search bar, reflecting active search and filters
- **Host/domain scope badge** — Cookies now show a server or globe icon indicating host-only vs domain scope
- **Filename sanitization** — Export filenames are cleaned of invalid characters before download

### Changed

- **Permissions reduced** — Removed `tabs` (redundant with `<all_urls>` host permissions); added `downloads` for native Save As
- **Description updated** — Chrome Web Store-ready description: "Edit, delete, protect, and export browser cookies. Manage values, expiration, and security flags with ease."
- **Badge readability** — Increased badge font sizes and spacing for better readability
- **Field and storage limits** — Added practical limits to input fields and storage operations
- **Save button UX** — Improved feedback and reactivity on save operations
- **Auto-scroll** — Editor scrolls to the active cookie on selection
- **Sticky action bar** — Action bar remains visible while scrolling
- **Settings enhancements** — Domain before name option, command labels, confirmation alerts, skip cache refresh

### Removed

- **`web_accessible_resources`** — Build script now strips this manifest entry to prevent extension fingerprinting

### Security

- Filename sanitization prevents path traversal in cookie exports
- Data URLs used instead of Blob URLs for download stability

---

## [0.1.0] — 2026-03-28

### Added

- **Initial release** — Cookie editor built with Rust/WebAssembly
- **Cookie CRUD operations** — Create, read, update, delete cookies with full field control
- **Protected cookies** — Mark cookies as read-only with auto-restoration
- **Copy to clipboard** — Three export formats (JSON, Netscape, Semicolon pairs)
- **Dark/Light theme** — System-aware theming with manual override
- **Settings modal** — In-popup settings with Save/Cancel/Close buttons
- **Search functionality** — Filter cookies by keyword
- **Sort options** — By domain, name, or expiration date
- **Context menu** — Right-click to edit cookies for current site
- **Cookie age limit** — Auto-expire cookies after configurable time
- **Version display** — Discreet version number in UI

### Changed

- Renamed extension display to "Edit Cookies"
- Settings converted from full page to modal popup
- Improved expiration date formatting for datetime-local input
- Fixed signal reactivity issues in cookie editor

### Security

- Memory-safe cookie handling via Rust
- No external data transmission
- Local storage only
- Content Security Policy enforced

---

## [Unreleased]

### Planned

- Import cookies from file
- Bulk edit operations
- Cookie expiration warnings
- Domain-based cookie grouping
- Export all cookies to file
