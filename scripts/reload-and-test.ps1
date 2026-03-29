# Reload extension and test
$env:Path = "C:\Users\benja\.cargo\bin;" + $env:Path

# Connect to Chrome and reload extension
$testScript = @"
const { chromium } = require('playwright');

(async () => {
  console.log('Connecting to Chrome...');
  const browser = await chromium.connectOverCDP('http://127.0.0.1:9222');
  console.log('Connected!');

  const context = browser.contexts()[0];
  let page = context.pages()[0];
  if (!page) {
    page = await context.newPage();
  }

  // Navigate to example.com
  console.log('Navigating to example.com...');
  await page.goto('https://example.com');
  await page.waitForTimeout(1000);

  // Set test cookies
  console.log('Setting test cookies...');
  await page.context().addCookies([
    { name: 'session_id', value: 'xyz789', domain: 'example.com', path: '/' },
    { name: 'user_pref', value: 'theme=dark', domain: 'example.com', path: '/' }
  ]);
  console.log('Cookies set!');

  await page.reload();
  await page.waitForTimeout(500);

  console.log('');
  console.log('=== MANUAL CHECK ===');
  console.log('1. Open the Edit Cookie Malaccamax extension popup');
  console.log('2. Verify you see: session_id, user_pref cookies');
  console.log('');
  console.log('Waiting 30 seconds...');
  await new Promise(r => setTimeout(r, 30000));
  await browser.close();
  console.log('Done.');
})();
"@

npx playwright exec node -e $testScript
