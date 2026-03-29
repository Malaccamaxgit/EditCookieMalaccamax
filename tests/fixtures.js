// Shared Playwright fixtures for testing a Chrome extension.
//
// Uses Playwright's BUNDLED Chromium (not installed Chrome) because
// --load-extension only works with the bundled browser.

const { test: base, chromium } = require('@playwright/test');
const path = require('path');
const os = require('os');
const fs = require('fs');

const EXTENSION_PATH = path.resolve(__dirname, '..', 'dist', 'chromium');

const test = base.extend({
  // Worker-scoped persistent browser context with the extension side-loaded.
  extensionContext: [async ({}, use) => {
    const userDataDir = fs.mkdtempSync(path.join(os.tmpdir(), 'ecm-test-'));

    const context = await chromium.launchPersistentContext(userDataDir, {
      headless: false,
      args: [
        `--disable-extensions-except=${EXTENSION_PATH}`,
        `--load-extension=${EXTENSION_PATH}`,
      ],
    });

    await use(context);
    await context.close();

    try { fs.rmSync(userDataDir, { recursive: true, force: true }); } catch {}
  }, { scope: 'worker' }],

  // Worker-scoped extension ID derived from the service worker URL.
  extensionId: [async ({ extensionContext }, use) => {
    let [worker] = extensionContext.serviceWorkers();
    if (!worker) {
      worker = await extensionContext.waitForEvent('serviceworker');
    }
    const id = worker.url().split('/')[2];
    await use(id);
  }, { scope: 'worker' }],

  // Per-test: opens the popup in a new page (no URL override).
  popupPage: async ({ extensionContext, extensionId }, use) => {
    const page = await extensionContext.newPage();
    await page.goto(`chrome-extension://${extensionId}/popup.html`);
    await page.waitForSelector('.app-container', { timeout: 15_000 });
    await use(page);
    await page.close();
  },
});

/**
 * Open the popup in the context of a specific site URL.
 *
 * Because the popup queries chrome.tabs.query({active:true}) to find
 * the current tab, and a standalone page always sees itself, we inject
 * a script that overrides chrome.tabs.query to return the target URL.
 */
async function openPopupForSite(context, extensionId, targetUrl) {
  const page = await context.newPage();

  // Intercept chrome.tabs.query so the popup thinks targetUrl is active
  await page.addInitScript((url) => {
    const origQuery = chrome.tabs.query.bind(chrome.tabs);
    chrome.tabs.query = (opts) => {
      if (opts && opts.active && opts.currentWindow) {
        return Promise.resolve([{ url, id: 9999 }]);
      }
      return origQuery(opts);
    };
  }, targetUrl);

  await page.goto(`chrome-extension://${extensionId}/popup.html`);
  await page.waitForSelector('.app-container', { timeout: 15_000 });
  return page;
}

const expect = test.expect;

module.exports = { test, expect, openPopupForSite };
