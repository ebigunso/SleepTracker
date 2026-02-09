import { test, expect } from '@playwright/test';

// These must match your backend ADMIN_EMAIL / password.
// Provide via environment variables or skip the test if absent.
const EMAIL = process.env.PLAYWRIGHT_EMAIL;
const PASSWORD = process.env.PLAYWRIGHT_PASSWORD;

// Helper to skip when creds are not provided
test.skip(!EMAIL || !PASSWORD, 'PLAYWRIGHT_EMAIL and PLAYWRIGHT_PASSWORD are required for this test');

test('login smoke: visit /login, authenticate, and reach home', async ({ page }) => {
  // Visit login page
  await page.goto('/login');

  // Fill credentials and submit
  await page.getByLabel('Email').fill(EMAIL!);
  await page.getByLabel('Password', { exact: true }).fill(PASSWORD!);
  await Promise.all([
    page.waitForURL('http://localhost:5173/'),
    page.getByRole('button', { name: /sign in/i }).click()
  ]);

  // Expect home/dashboard to be visible (header, Logout button etc.)
  await expect(page.getByRole('heading', { name: /SleepTracker/i })).toBeVisible();
  await expect(page.getByRole('button', { name: /logout/i })).toBeVisible();

  // Refresh: session should persist
  await page.reload();
  await expect(page.getByRole('button', { name: /logout/i })).toBeVisible();
});
