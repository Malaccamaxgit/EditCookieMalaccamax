# Security Policy

> **Security measures** — Memory-safe Rust, minimal permissions, and local storage only.

## Reporting a Vulnerability

If you discover a security vulnerability in this extension, please report it responsibly:

1. **GitHub Security Advisories** (preferred): Go to the "Security" tab → "Advisories" → "Report a vulnerability"
2. **Email**: Send details to **benjamin.alloul@gmail.com** with subject `SECURITY: EditCookieMalaccamax`

**Please do not open public GitHub issues for security vulnerabilities.**

### What to Include

| Item | Description |
|------|-------------|
| Description | Vulnerability description |
| Reproduction | Steps to reproduce |
| Affected version | Check `manifest.json` |
| Environment | Chrome/OS version (if relevant) |
| Evidence | Proof of concept code or screenshots |

### Response Expectations

This is a solo-maintained open source project. Security reports are reviewed on a **best-effort basis** as time permits. There are no guaranteed response timelines or SLAs.

## Security Features

### Memory-Safe Cookie Handling

| Measure | Implementation |
|---------|---------------|
| Language | Rust with compile-time memory safety |
| No manual memory management | Ownership system prevents use-after-free, double-free |
| Type safety | Strong typing prevents injection attacks |
| No garbage collector | Predictable memory behavior |

### Data Storage

| Measure | Implementation |
|---------|---------------|
| Storage location | `chrome.storage.local` only |
| No external transmission | All data stays on device |
| No encryption | Data stored as plaintext in browser storage |
| Protected cookies | Stored as name/domain pairs only (values not persisted) |

**Threat model:** Extension storage is accessible to anyone with access to the browser profile. This provides **casual inspection** protection but not protection against determined attackers with browser access.

### Permissions

| Permission | Scope | Justification |
|------------|-------|---------------|
| `cookies` | All origins | Read, modify, delete cookies (core functionality) |
| `storage` | Local only | Save preferences and protected cookies |
| `contextMenus` | All pages | Add right-click menu option |
| `clipboardWrite` | User-initiated | Copy cookies to clipboard |
| `downloads` | User-initiated | Export cookies to file with Save As dialog |

### Other Measures

| Measure | Description |
|---------|-------------|
| **Content Security Policy** | Strict CSP in manifest prevents inline script execution |
| **No eval()** | No dynamic code execution |
| **No external resources** | All code bundled locally |
| **Minimal permissions** | Only permissions required for core functionality |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x | ✅ Yes |
| < 0.1.0 | ❌ No |

## Security Testing

| Test Type | Method |
|-----------|--------|
| Memory safety | Rust compiler guarantees |
| Type safety | Rust type system |
| Manual testing | Cookie CRUD operations |

Run tests: `cargo test`

## Dependencies

Dependencies are checked for known vulnerabilities:

| Method | Frequency |
|--------|-----------|
| Cargo audit | Manual run during development |
| GitHub Dependabot | Automated alerts (if enabled) |

**Current status:** 0 known vulnerabilities

## Limitations

| Limitation | Description |
|------------|-------------|
| Chrome Sandbox | Subject to Chrome extension sandbox limitations |
| Storage limits | Chrome local storage limits apply |
| No server-side validation | All validation is client-side |

## Contact

| Issue Type | Contact |
|------------|---------|
| Security Issues | benjamin.alloul@gmail.com |
| General Issues | [GitHub Issues](https://github.com/Malaccamaxgit/EditCookieMalaccamax/issues) |

---

**Last Updated:** March 2026
