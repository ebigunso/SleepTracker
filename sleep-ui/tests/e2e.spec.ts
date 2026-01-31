/// <reference types="node" />

import { test, expect } from '@playwright/test';

// These must match your backend ADMIN_EMAIL / password.
// Provide via environment variables or skip the test if absent.
const EMAIL = process.env.PLAYWRIGHT_EMAIL;
const PASSWORD = process.env.PLAYWRIGHT_PASSWORD;

// Helper to skip when creds are not provided
test.skip(!EMAIL || !PASSWORD, 'PLAYWRIGHT_EMAIL and PLAYWRIGHT_PASSWORD are required for this test');

test('dashboard quick log -> edit -> delete, with duration warning', async ({ page }) => {
  // Login
  await page.goto('/login');
  await page.getByLabel('Email').fill(EMAIL!);
  await page.getByLabel('Password').fill(PASSWORD!);
  await Promise.all([page.waitForURL('**/'), page.getByRole('button', { name: /sign in/i }).click()]);
  await expect(page.getByTestId('dashboard-heading')).toBeVisible();

  // Quick Log: navigate to create form
  await page.getByRole('button', { name: /quick log/i }).click();
  await expect(page.getByRole('heading', { name: /new sleep entry/i })).toBeVisible();

  // Fill minimal valid values that trigger duration warning (< 2h)
  // Date defaults to today; leave it.
  await page.getByLabel('Bed time').fill('22:00');
  await page.getByLabel('Wake time').fill('23:00');
  await page.getByLabel('Latency (min)').fill('5');
  await page.getByLabel('Awakenings').fill('0');
  await page.getByLabel('Quality (1-5)').fill('3');
  await page.getByLabel('Exercise intensity').selectOption('light');

  // Submit -> expect ConfirmDialog warning, then proceed
  await page.getByRole('button', { name: /^save$/i }).click();
  await expect(page.getByText(/unusual duration/i)).toBeVisible();
  await page.getByRole('button', { name: /proceed/i }).click();

  // Back to dashboard, row for today should be present
  await expect(page.getByTestId('dashboard-heading')).toBeVisible();

  // Edit the first row (assumes newest first)
  await page.getByRole('button', { name: /^edit$/i }).first().click();

  // Edit page loads via ?date=..., update quality to 5 and save
  await expect(page.getByRole('heading', { name: /edit sleep entry/i })).toBeVisible();
  const quality = page.getByLabel('Quality (1-5)');
  await quality.fill('5');
  await page.getByRole('button', { name: /^update$/i }).click();

  // Back to dashboard; verify that some "Quality: 5" text exists in the first row area
  await expect(page.getByText(/Quality:\s*5/i)).toBeVisible();

  // Delete the entry from dashboard; accept confirm dialog
  page.once('dialog', async (dialog) => {
    await dialog.accept();
  });
  await page.getByRole('button', { name: /^delete$/i }).first().click();

  // After deletion, "Quality: 5" might disappear for that row; at least ensure the dashboard still renders
  await expect(page.getByTestId('dashboard-heading')).toBeVisible();
});
