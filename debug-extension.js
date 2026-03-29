// Debug script - verify extension via toolbar and popup
const { chromium } = require('playwright');
const fs = require('fs');

const extensionPath = 'E:\\Github\\EditCookieMalaccamax\\dist\\chromium';
const userDataDir = 'C:\\temp\\chrome-debug-profile';

console.log('=== Edit Cookie Malaccamax Debug ===\n');
console.log('Manifest content:');
console.log(fs.readFileSync(extensionPath + '\\manifest.json', 'utf8'));
console.log('\n');

(async () => {
  try {
    const browser = await chromium.launchPersistentContext(userDataDir, {
      headless: false,
      channel: 'chrome',
      args: [
        `--load-extension=${extensionPath}`,
        '--enable-logging',
        '--v=2',
      ],
    });

    const page = browser.pages()[0];

    // Capture ALL console and log messages
    page.on('console', msg => {
      console.log(`[CONSOLE ${msg.type()}]`, msg.text());
    });
    page.on('pageerror', err => console.log('[PAGE ERROR]', err.message));
    page.on('requestfailed', req => {
      console.log('[REQUEST FAILED]', req.url(), req.failure()?.errorText);
    });

    // Go directly to test page
    console.log('Navigating to example.com...\n');
    await page.goto('https://example.com');
    await page.waitForTimeout(2000);

    // Check for extension in toolbar
    console.log('=== Checking Toolbar for Extension Icon ===');

    const toolbarData = await page.evaluate(() => {
      const results = {
        extensionsMenu: null,
        extensionIcons: [],
        popupOpen: false,
      };

      // Look for extensions menu (puzzle icon)
      const extensionsMenu = document.querySelector('[aria-label*="extensions"]') ||
                             document.querySelector('[title*="extensions"]') ||
                             document.querySelector('[data-tooltip*="extension"]');

      if (extensionsMenu) {
        results.extensionsMenu = extensionsMenu.getAttribute('aria-label') ||
                                 extensionsMenu.getAttribute('title') ||
                                 'found';
      }

      // Look for any extension icons in toolbar
      document.querySelectorAll('[role="button"], [aria-label], [title]').forEach(el => {
        const text = (el.getAttribute('aria-label') || el.getAttribute('title') || '').toLowerCase();
        if (text.includes('edit') || text.includes('cookie')) {
          results.extensionIcons.push({
            label: text,
            tag: el.tagName,
            id: el.id || 'no-id'
          });
        }
      });

      return results;
    });

    console.log('Extensions menu found:', !!toolbarData.extensionsMenu);
    if (toolbarData.extensionsMenu) {
      console.log('  Label:', toolbarData.extensionsMenu);
    }
    console.log('Extension icons in toolbar:', toolbarData.extensionIcons.length);
    toolbarData.extensionIcons.forEach(e => console.log('  -', e.label, e.tag, e.id));

    // Take screenshot of toolbar
    await page.screenshot({ path: 'toolbar-screenshot.png' });
    console.log('\nScreenshot: toolbar-screenshot.png');

    // Try to open extension popup via direct URL
    console.log('\n=== Attempting to Open Extension Popup ===');

    try {
      // Get extension ID from chrome.management API
      const extensionId = await page.evaluate(() => {
        return new Promise((resolve) => {
          if (chrome.management) {
            chrome.management.getAll((extensions) => {
              const editCookie = extensions.find(ext =>
                ext.name && ext.name.includes('Edit Cookie')
              );
              resolve(editCookie ? editCookie.id : null);
            });
          } else {
            resolve(null);
          }
        });
      });

      if (extensionId) {
        console.log('Extension ID found:', extensionId);

        // Open popup
        const popupUrl = `chrome-extension://${extensionId}/popup.html`;
        console.log('Opening popup at:', popupUrl);

        const popupPage = await browser.newPage();
        popupPage.on('console', msg => {
          console.log(`[POPUP CONSOLE ${msg.type()}]`, msg.text());
        });
        popupPage.on('pageerror', err => {
          console.log('[POPUP ERROR]', err.message);
        });

        await popupPage.goto(popupUrl);
        await popupPage.waitForTimeout(3000);

        // Check popup content
        const popupContent = await popupPage.evaluate(() => {
          return {
            title: document.title,
            bodyText: document.body.textContent?.substring(0, 500),
            hasApp: !!document.querySelector('#app'),
            hasError: !!document.body.querySelector('[class*="error"]') ||
                      document.body.textContent?.includes('Error')
          };
        });

        console.log('Popup content:', popupContent);

        await popupPage.screenshot({ path: 'popup-screenshot.png' });
        console.log('Popup screenshot: popup-screenshot.png');

        await popupPage.close();
      } else {
        console.log('Extension ID not found via chrome.management');
        console.log('Extension may not be loaded or visible to this context');
      }
    } catch (e) {
      console.log('Error opening popup:', e.message);
    }

    console.log('\n=== Debug Complete ===');
    console.log('Check the screenshots and console output above');
    console.log('Close browser when done');

    await new Promise(r => {
      browser.on('disconnected', r);
      setTimeout(() => browser.close(), 120000);
    });

  } catch (error) {
    console.error('Error:', error.message);
  }
})();
