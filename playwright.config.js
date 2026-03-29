// @ts-check
const { defineConfig } = require('@playwright/test');
const path = require('path');

module.exports = defineConfig({
  testDir: './tests',
  timeout: 60_000,
  expect: { timeout: 10_000 },
  retries: 0,
  workers: 1, // extensions require serial execution (single browser context)
  reporter: 'html',
  use: {
    headless: false, // extensions don't work headless
    viewport: { width: 800, height: 600 },
    actionTimeout: 10_000,
  },
});
