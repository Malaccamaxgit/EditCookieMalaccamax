# Pre-Flight Checklist

> **Release readiness** — Pre-release verification checklist for EditCookieMalaccamax.

## Security

### Secrets
- [ ] No hardcoded API keys, tokens, or passwords
- [ ] No `.env` files committed (patterns in `.gitignore`)
- [ ] No absolute file paths leaked
- [ ] Test fixtures use placeholder data only

### Git History
- [ ] No sensitive files ever committed
- [ ] Clean history verified

### Dependencies
- [ ] `cargo audit`: 0 vulnerabilities
- [ ] `npm audit`: 0 vulnerabilities
- [ ] All packages actively maintained
- [ ] No deprecated dependencies

## Documentation

| File | Status |
|------|--------|
| README.md | Installation, features, development setup |
| SECURITY.md | Vulnerability reporting instructions |
| PRIVACY.md | Data handling policy |
| LICENSE | GPL-3.0 full text included |
| CODE_OF_CONDUCT.md | Community standards |
| CHANGELOG.md | Version history |

## Repository Health

- [ ] GitHub issue templates: Bug report + feature request
- [ ] GitHub Actions: Security audit workflow active
- [ ] Pre-commit hooks: `scripts/secret-scanner.js` (Husky)
- [ ] Tests: All Playwright tests passing
- [ ] Build: `scripts/Build-Extension.ps1` completes successfully
- [ ] `cargo clippy`: No warnings

## Build Verification

```powershell
# Build the extension
.\scripts\Build-Extension.ps1

# Run tests
npx playwright test

# Audit dependencies
cargo audit
npm audit
```

## Summary

**Status:** Ready for public release

All security checks passed:
- 0 secrets or CVEs
- Comprehensive `.gitignore`
- GPL-3.0 legally compliant
- Documentation complete
- CI/CD automation active

## Next Steps

```bash
git add .
git commit -m "chore: pre-flight verification complete"
git push origin main
```

Post-push:
- Verify GitHub Actions pass
- Enable GitHub Security Advisories
- Add repository topics: `chrome-extension`, `rust`, `webassembly`, `cookie-editor`, `manifest-v3`
