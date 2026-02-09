import { test, expect } from '@playwright/test';

// These must match your backend ADMIN_EMAIL / password.
// Provide via environment variables or skip the test if absent.
const EMAIL = process.env.PLAYWRIGHT_EMAIL;
const PASSWORD = process.env.PLAYWRIGHT_PASSWORD;

// Helper to skip when creds are not provided
test.skip(!EMAIL || !PASSWORD, 'PLAYWRIGHT_EMAIL and PLAYWRIGHT_PASSWORD are required for this test');

async function login(page: import('@playwright/test').Page) {
  await page.context().clearCookies();
  await page.goto('/login');
  await page.waitForLoadState('domcontentloaded');
  await page.waitForTimeout(500);
  await expect(page.getByLabel('Email')).toBeVisible();

  await page.getByLabel('Email').fill(EMAIL!);
  await page.getByLabel('Password', { exact: true }).fill(PASSWORD!);

  const [loginRequest] = await Promise.all([
    page.waitForRequest('**/api/login'),
    page.getByRole('button', { name: 'Sign in' }).click()
  ]);

  if (loginRequest.method() !== 'POST') {
    throw new Error(`Login request used unexpected method: ${loginRequest.method()}`);
  }
}

test('login smoke: visit /login, authenticate, and reach home', async ({ page }) => {
  await login(page);

  // Expect home/dashboard to be visible (header, Logout button etc.)
  await expect(page.getByRole('heading', { name: /SleepTracker/i })).toBeVisible();
  await expect(page.getByRole('button', { name: /logout/i })).toBeVisible();

  // Refresh: session should persist
  await page.reload();
  await expect(page.getByRole('button', { name: /logout/i })).toBeVisible();
});
