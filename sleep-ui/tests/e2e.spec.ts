/// <reference types="node" />

import { test, expect } from '@playwright/test';

// These must match your backend ADMIN_EMAIL / password.
// Provide via environment variables or skip the test if absent.
const EMAIL = process.env.PLAYWRIGHT_EMAIL;
const PASSWORD = process.env.PLAYWRIGHT_PASSWORD;

function formatDate(d: Date): string {
  const year = d.getFullYear();
  const month = String(d.getMonth() + 1).padStart(2, '0');
  const day = String(d.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

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

  await expect(page.getByTestId('dashboard-heading')).toBeVisible();
}

test('dashboard quick log -> edit -> delete, with duration warning', async ({ page }) => {
  await login(page);

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

test('multiple sessions per day display and edit', async ({ page }) => {
  const targetDate = formatDate(new Date(Date.now() - 24 * 60 * 60 * 1000));

  await login(page);

  const createSession = async (bed: string, wake: string, quality: string) => {
    await page.goto(`/sleep/new?date=${targetDate}`);
    await expect(page.getByRole('heading', { name: /new sleep entry/i })).toBeVisible();
    await page.getByLabel('Date').fill(targetDate);
    await page.getByLabel('Bed time').fill(bed);
    await page.getByLabel('Wake time').fill(wake);
    await page.getByLabel('Latency (min)').fill('5');
    await page.getByLabel('Awakenings').fill('0');
    await page.getByLabel('Quality (1-5)').fill(quality);
    await page.getByLabel('Exercise intensity').selectOption('light');

    await Promise.all([
      page.waitForURL('**/'),
      page.getByRole('button', { name: /^save$/i }).click(),
    ]);
    await expect(page.getByTestId('dashboard-heading')).toBeVisible();
  };

  await createSession('01:00', '04:00', '3');
  await createSession('05:00', '08:00', '4');

  await page.goto(`/day/${targetDate}`);
  await expect(page.getByRole('heading', { name: new RegExp(`day view:\\s*${targetDate}`, 'i') })).toBeVisible();

  const bedLabels = page.getByText(/Bed:\s*/);
  const bedCount = await bedLabels.count();
  expect(bedCount).toBeGreaterThanOrEqual(2);

  const targetCard = page
    .locator('div', { hasText: /Bed:\s*01:00/ })
    .filter({ hasText: /Wake:\s*04:00/ })
    .first();

  await targetCard.getByRole('button', { name: /^edit$/i }).click();
  await expect(page.getByRole('heading', { name: /edit sleep entry/i })).toBeVisible();
  await page.getByLabel('Quality (1-5)').fill('5');
  await Promise.all([
    page.waitForURL('**/'),
    page.getByRole('button', { name: /^update$/i }).click(),
  ]);

  await page.goto(`/day/${targetDate}`);
  await expect(
    page
      .locator('div', { hasText: /Bed:\s*01:00/ })
      .filter({ hasText: /Wake:\s*04:00/ })
      .getByText(/Quality:\s*5/)
  ).toBeVisible();

  // Cleanup: delete both sessions
  const deleteSession = async (bed: RegExp, wake: RegExp) => {
    await page.goto(`/day/${targetDate}`);
    const card = page.locator('div', { hasText: bed }).filter({ hasText: wake }).first();
    await card.getByRole('button', { name: /^edit$/i }).click();
    await expect(page.getByRole('heading', { name: /edit sleep entry/i })).toBeVisible();
    page.once('dialog', async (dialog) => {
      await dialog.accept();
    });
    await Promise.all([
      page.waitForURL('**/'),
      page.getByRole('button', { name: /^delete$/i }).click(),
    ]);
    await expect(page.getByTestId('dashboard-heading')).toBeVisible();
  };

  await deleteSession(/Bed:\s*01:00/, /Wake:\s*04:00/);
  await deleteSession(/Bed:\s*05:00/, /Wake:\s*08:00/);
});
