# Playwright script to debug the Edit Cookie Malaccamax extension
# Connects to existing Chrome instance at localhost:9222

$extensionPath = "E:\Github\EditCookieMalaccamax\dist\chromium"

# Create test script
$testScript = @"
const { chromium } = require('playwright');

(async () => {
  console.log('Connecting to Chrome at localhost:9222...');

  // Connect to existing Chrome instance
  const browser = await chromium.connectOverCDP('http://localhost:9222');
  console.log('Connected!');

  // Get first context or create new one
  const context = browser.contexts()[0] || await browser.newContext();
  let page = context.pages()[0];
  if (!page) {
    page = await context.newPage();
  }

  // Capture console messages
  page.on('console', msg => {
    console.log('[PAGE CONSOLE][' + msg.type() + ']', msg.text());
  });

  page.on('pageerror', err => {
    console.log('[PAGE ERROR]', err.message);
  });

  // Navigate to extensions page
  console.log('Navigating to extensions page...');
  await page.goto('chrome://extensions/');
  await page.waitForTimeout(2000);

  // Enable developer mode
  console.log('Enabling developer mode...');
  const devModeToggle = page.locator('#devMode input');
  if (await devModeToggle.count() > 0) {
    const isChecked = await devModeToggle.isChecked();
    if (!isChecked) {
      await devModeToggle.check();
      await page.waitForTimeout(500);
    }
    console.log('Developer mode enabled');
  }

  // Load unpacked extension
  console.log('Loading extension from:', '$extensionPath');
  const fileChooserPromise = page.waitForEvent('filechooser');
  await page.click('text=Load unpacked');
  const fileChooser = await fileChooserPromise;
  await fileChooser.setFiles('$extensionPath');
  await page.waitForTimeout(3000);

  // Check if extension loaded
  const extensions = await page.locator('.extension-list-item').all();
  console.log('Loaded extensions:', extensions.length);

  for (let ext of extensions) {
    const name = await ext.locator('.extension-name').textContent();
    console.log('  -', name);
    if (name && name.includes('Edit Cookie')) {
      console.log('Found Edit Cookie Malaccamax extension!');

      // Check for errors
      const errors = await ext.locator('.extension-error').all();
      if (errors.length > 0) {
        console.log('Extension errors:', errors.length);
        for (let err of errors) {
          console.log('  ERROR:', await err.textContent());
        }
      } else {
        console.log('No extension errors detected');
      }
    }
  }

  // Take screenshot
  await page.screenshot({ path: 'extensions-page.png' });
  console.log('Screenshot saved to extensions-page.png');

  // Open popup if extension is active
  try {
    const popupButton = page.locator('text=Edit Cookie').first();
    if (await popupButton.count() > 0) {
      console.log('Opening extension popup...');
      await popupButton.click();
      await page.waitForTimeout(2000);

      // Get popup console messages
      const popupPage = await context.waitForEvent('page');
      popupPage.on('console', msg => {
        console.log('[POPUP CONSOLE][' + msg.type() + ']', msg.text());
      });
    }
  } catch (e) {
    console.log('Could not open popup:', e.message);
  }

  console.log('Debug session complete');
  await browser.close();
})();
"@

# Run the script
npx playwright exec node -e "$testScript"
