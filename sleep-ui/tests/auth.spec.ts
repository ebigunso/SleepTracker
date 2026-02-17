import { test, expect } from '@playwright/test';
test('auth smoke: bootstrap state reaches home and persists', async ({ page }) => {
  await page.goto('/');

  await expect(page.getByRole('heading', { name: /SleepTracker/i })).toBeVisible();
  await expect(page.getByRole('button', { name: /profile/i })).toBeVisible();

  await page.reload();
  await expect(page.getByRole('button', { name: /profile/i })).toBeVisible();
});
