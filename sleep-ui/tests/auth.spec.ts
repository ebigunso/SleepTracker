import { test, expect } from '@playwright/test';
test('auth smoke: bootstrap state reaches home and persists', async ({ page }) => {
  await page.goto('/');

  await expect(page.getByTestId('dashboard-heading')).toBeVisible();
  await expect(page.getByTestId('global-nav-brand-action').or(page.getByRole('heading', { name: /SleepTracker/i })).first()).toBeVisible();
  await expect(page.getByTestId('profile-menu-trigger-action').or(page.getByRole('button', { name: /profile/i })).first()).toBeVisible();

  await page.reload();
  await expect(page.getByTestId('profile-menu-trigger-action').or(page.getByRole('button', { name: /profile/i })).first()).toBeVisible();
});
