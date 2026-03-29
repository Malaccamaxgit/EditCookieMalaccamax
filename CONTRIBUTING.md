# Contributing to Edit Cookie Malaccamax

> **Community contributions welcome** — Help build the cookie editor.

Thank you for considering contributing to Edit Cookie Malaccamax! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a Contributor Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to benjamin.alloul@gmail.com.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps to reproduce the problem**
* **Provide specific examples to demonstrate the steps**
* **Describe the behavior you observed and what behavior you expected**
* **Include screenshots if possible**
* **Include browser version and extension version**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a detailed description of the suggested enhancement**
* **Explain why this enhancement would be useful**
* **List some examples of how this enhancement would be used**

### Pull Requests

* Fill in the required template
* Follow the Rust style guide (rustfmt)
* Include comments in your code where necessary
* Update documentation as needed

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Chrome or Chromium browser
- Git

### Setting Up

```bash
# Clone the repository
git clone https://github.com/Malaccamaxgit/EditCookieMalaccamax.git
cd EditCookieMalaccamax

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Oxichrome
cargo install oxichrome

# Build
cargo build --release

# Load in Chrome
# 1. Navigate to chrome://extensions/
# 2. Enable Developer mode
# 3. Click "Load unpacked"
# 4. Select the dist/ folder
```

## Coding Guidelines

### Rust Code Style

This project uses `rustfmt` for consistent code formatting:

```bash
# Format your code before committing
cargo fmt
```

### Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line

### Code Review Process

The core team looks at Pull Requests on a regular basis. After feedback is addressed, PRs will be merged by a maintainer.

## Architecture Overview

```
src/
├── background/         # Service worker logic
├── popup/              # Main UI component
├── options/            # Settings UI
├── core/               # Business logic (cookie, storage, rules)
├── shared/             # Shared types and utilities
└── chrome_api/         # Chrome API bindings
```

### Key Concepts

* **Leptos Signals** — Reactive state management using `ReadSignal` and `WriteSignal`
* **Chrome Cookies API** — All cookie operations go through `chrome.cookies.*`
* **Storage Layer** — Typed wrapper around `chrome.storage.local`
* **Background Worker** — Enforces protected cookies and filter rules

## Questions?

Feel free to open an issue for any questions about contributing.

---

**Thanks for contributing to Edit Cookie Malaccamax!**
