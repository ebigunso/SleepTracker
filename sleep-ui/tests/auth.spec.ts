import { test, expect } from '@playwright/test';

// These must match your backend ADMIN_EMAIL / password.
// Provide via environment variables or skip the test if absent.
const EMAIL = process.env.PLAYWRIGHT_EMAIL;
const PASSWORD = process.env.PLAYWRIGHT_PASSWORD;

// Helper to skip when creds are not provided
test.skip(!EMAIL || !PASSWORD, 'PLAYWRIGHT_EMAIL and PLAYWRIGHT_PASSWORD are required for this test');

async function login(page: import('@playwright/test').Page) {
  await page.goto('/login');

  await page.getByLabel('Email').fill(EMAIL!);
  await page.getByLabel('Password', { exact: true }).fill(PASSWORD!);

  const [loginResponse] = await Promise.all([
    page.waitForResponse((response) => response.url().includes('/api/login')),
    page.getByRole('button', { name: /sign in/i }).click()
  ]);

  const status = loginResponse.status();
  if (status >= 400) {
    throw new Error(`Login failed with status ${status}. Check backend auth config (ADMIN_EMAIL/password, COOKIE_SECURE=0 for http).`);
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
