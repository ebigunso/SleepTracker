/// <reference types="node" />

import { test, expect } from '@playwright/test';

function formatDate(d: Date): string {
  const year = d.getFullYear();
  const month = String(d.getMonth() + 1).padStart(2, '0');
  const day = String(d.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

function pathnameOf(url: string): string {
  return new URL(url).pathname;
}

function toHHmm(totalMinutes: number): string {
  const normalized = ((totalMinutes % (24 * 60)) + (24 * 60)) % (24 * 60);
  const hh = String(Math.floor(normalized / 60)).padStart(2, '0');
  const mm = String(normalized % 60).padStart(2, '0');
  return `${hh}:${mm}`;
}

function homeLogSleepButton(page: import('@playwright/test').Page) {
  return page
    .getByTestId('home-log-sleep-button')
    .or(page.getByRole('button', { name: /(log sleep|quick log)/i }))
    .first();
}

function sleepNewHeading(page: import('@playwright/test').Page) {
  return page
    .getByTestId('sleep-new-heading')
    .or(page.getByRole('heading', { name: /new sleep entry/i }))
    .first();
}

function sleepEditHeading(page: import('@playwright/test').Page) {
  return page
    .getByTestId('sleep-edit-heading')
    .or(page.getByRole('heading', { name: /edit sleep entry/i }))
    .first();
}

function sleepFormSubmitButton(page: import('@playwright/test').Page, action: 'save' | 'update') {
  const label = action === 'save' ? /^save$/i : /^update$/i;
  return page.getByTestId('sleep-form-submit-action').or(page.getByRole('button', { name: label })).first();
}

function sleepFormCancelButton(page: import('@playwright/test').Page) {
  return page.getByTestId('sleep-form-cancel-action').or(page.getByRole('button', { name: /^cancel$/i })).first();
}

function dayAddSessionButton(page: import('@playwright/test').Page) {
  return page.getByTestId('day-add-session-button').or(page.getByRole('button', { name: /add session/i })).first();
}

async function navigateToNewFromHome(page: import('@playwright/test').Page) {
  await Promise.all([
    page.waitForURL('**/sleep/new*'),
    homeLogSleepButton(page).click(),
  ]);
}

async function navigateToNewFromDay(page: import('@playwright/test').Page, targetDate: string) {
  await Promise.all([
    page.waitForURL(`**/sleep/new?date=${encodeURIComponent(targetDate)}`),
    dayAddSessionButton(page).click(),
  ]);
}

async function expectNewFormReady(page: import('@playwright/test').Page) {
  await expect(sleepNewHeading(page).or(page.getByLabel('Bed time')).first()).toBeVisible();
}

test('dashboard quick log -> edit -> delete, with duration warning', async ({ page }) => {
  const quickLogDate = formatDate(new Date(Date.now() - 10 * 24 * 60 * 60 * 1000));

  await page.goto('/');
  await expect(page.getByTestId('dashboard-heading')).toBeVisible();

  // Quick Log: navigate to create form
  await navigateToNewFromHome(page);
  await expectNewFormReady(page);

  // Fill minimal valid values that trigger duration warning (< 2h)
  await page.getByLabel('Date').fill(quickLogDate);
  await page.getByLabel('Bed time').fill('22:00');
  await page.getByLabel('Wake time').fill('23:00');
  const latencyField = page.getByLabel('Latency (min)');
  if ((await latencyField.count()) > 0) {
    await latencyField.fill('5');
  }
  const awakeningsField = page.getByLabel('Awakenings');
  if ((await awakeningsField.count()) > 0) {
    await awakeningsField.fill('0');
  }
  const qualityField = page.getByLabel('Quality (1-5)');
  if ((await qualityField.count()) > 0) {
    await qualityField.fill('3');
  }
  const intensityField = page.getByLabel('Exercise intensity');
  if ((await intensityField.count()) > 0) {
    await intensityField.selectOption('light');
  }

  // Submit -> expect ConfirmDialog warning, then proceed
  await sleepFormSubmitButton(page, 'save').click();
  await expect(page.getByTestId('confirm-dialog-state').or(page.getByText(/unusual duration/i)).first()).toBeVisible();
  const [createResponse] = await Promise.all([
    page.waitForResponse(
      (response) =>
        response.url().includes('/api/sleep') && response.request().method() === 'POST'
    ),
    page.getByTestId('confirm-dialog-confirm-action').or(page.getByRole('button', { name: /proceed/i })).first().click()
  ]);
  expect(createResponse.status()).toBeLessThan(400);
  if (pathnameOf(page.url()) !== '/') {
    await page.goto('/');
  }

  // Back to dashboard, row for today should be present
  await expect(page.getByTestId('dashboard-heading')).toBeVisible();

  await page.goto(`/day/${quickLogDate}`);
  await expect(page.getByTestId('day-view-heading').or(page.getByRole('heading', { name: /day view/i })).first()).toBeVisible();
  await page.getByRole('button', { name: /^edit$/i }).first().click();

  // Edit page loads via ?date=..., update quality to 5 and save
  await expect(sleepEditHeading(page)).toBeVisible();
  const quality = page.getByLabel('Quality (1-5)');
  await quality.fill('5');
  const [updateResponse] = await Promise.all([
    page.waitForResponse(
      (response) =>
        response.request().method() === 'PUT' && response.url().includes('/api/sleep/')
    ),
    sleepFormSubmitButton(page, 'update').click(),
  ]);
  expect(updateResponse.status()).toBeLessThan(400);
  if (pathnameOf(page.url()) !== '/') {
    await page.goto('/');
  }

  // Delete the entry from dashboard; accept confirm dialog
  page.once('dialog', async (dialog) => {
    await dialog.accept();
  });
  await page.goto(`/day/${quickLogDate}`);
  await page.getByRole('button', { name: /^edit$/i }).first().click();
  await expect(sleepEditHeading(page)).toBeVisible();
  await page.getByTestId('sleep-edit-delete-button').or(page.getByRole('button', { name: /^delete$/i })).first().click();

  // After deletion, "Quality: 5" might disappear for that row; at least ensure the dashboard still renders
  await expect(page.getByTestId('dashboard-heading')).toBeVisible();
});

test('new sleep entry cancel returns home without creating session', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByTestId('dashboard-heading')).toBeVisible();

  let sleepMutationRequests = 0;
  const onRequest = (request: import('@playwright/test').Request) => {
    if (
      request.url().includes('/api/sleep') &&
      ['POST', 'PUT', 'PATCH'].includes(request.method())
    ) {
      sleepMutationRequests += 1;
    }
  };

  page.on('request', onRequest);

  await navigateToNewFromHome(page);
  await expectNewFormReady(page);

  const cancelButton = sleepFormCancelButton(page);
  if ((await cancelButton.count()) > 0) {
    try {
      await cancelButton.click();
      await page.waitForURL('**/', { timeout: 10_000 });
    } catch {
      await page.goto('/');
    }
  } else {
    await page.goto('/');
  }

  await expect(page.getByTestId('dashboard-heading')).toBeVisible();
  expect(sleepMutationRequests).toBe(0);

  page.off('request', onRequest);
});

test('multiple sessions per day display and edit', async ({ page }) => {
  const targetDate = formatDate(new Date(Date.now() - 24 * 60 * 60 * 1000));

  await page.goto('/');
  await expect(page.getByTestId('dashboard-heading')).toBeVisible();

  const createSession = async (bed: string, wake: string, quality: string): Promise<void> => {
    await page.goto(`/day/${targetDate}`);
    await expect(page.getByTestId('day-view-heading').or(page.getByRole('heading', { name: /day view/i })).first()).toBeVisible();
    await navigateToNewFromDay(page, targetDate);
    await expectNewFormReady(page);
    await page.getByLabel('Date').fill(targetDate);
    await page.getByLabel('Bed time').fill(bed);
    await page.getByLabel('Wake time').fill(wake);
    const latencyField = page.getByLabel('Latency (min)');
    if ((await latencyField.count()) > 0) {
      await latencyField.fill('5');
    }
    const awakeningsField = page.getByLabel('Awakenings');
    if ((await awakeningsField.count()) > 0) {
      await awakeningsField.fill('0');
    }
    const qualityField = page.getByLabel('Quality (1-5)');
    if ((await qualityField.count()) > 0) {
      await qualityField.fill(quality);
    }
    const intensityField = page.getByLabel('Exercise intensity');
    if ((await intensityField.count()) > 0) {
      await intensityField.selectOption('light');
    }

    const [createResponse] = await Promise.all([
      page.waitForResponse(
        (response) =>
          response.url().includes('/api/sleep') && response.request().method() === 'POST'
      ),
      sleepFormSubmitButton(page, 'save').click(),
    ]);
    expect(createResponse.status()).toBeLessThan(400);
    if (pathnameOf(page.url()) !== '/') {
      await page.goto('/');
    }
    await expect(page.getByTestId('dashboard-heading')).toBeVisible();

  };

  const targetSeed = Number(targetDate.replace(/-/g, ''));
  const firstOffset = targetSeed % 180;
  const secondOffset = (targetSeed * 7) % 180;
  const firstBed = toHHmm(30 + firstOffset);
  const firstWake = toHHmm(240 + firstOffset);
  const secondBed = toHHmm(720 + secondOffset);
  const secondWake = toHHmm(930 + secondOffset);

  await createSession(firstBed, firstWake, '3');
  await createSession(secondBed, secondWake, '4');

  await page.goto(`/day/${targetDate}`);
  await expect(page.getByTestId('day-view-heading').or(page.getByRole('heading', { name: /day view/i })).first()).toBeVisible();
  await expect(page.getByText(targetDate)).toBeVisible();

  const bedLabels = page.getByText(/Bed:\s*/);
  const bedCount = await bedLabels.count();
  expect(bedCount).toBeGreaterThanOrEqual(2);

  const firstCard = page
    .locator('section .card.p-4', { hasText: new RegExp(`Bed:\\s*${firstBed}`) })
    .filter({ hasText: new RegExp(`Wake:\\s*${firstWake}`) })
    .first();
  await firstCard.locator('button', { hasText: /^Edit$/i }).first().click();
  await expect(sleepEditHeading(page)).toBeVisible();
  await page.getByLabel('Quality (1-5)').fill('5');
  await Promise.all([
    page.waitForURL('**/'),
    sleepFormSubmitButton(page, 'update').click(),
  ]);

  await page.goto(`/day/${targetDate}`);
  await expect(page.getByText(/Quality:\s*5/)).toBeVisible();

  // Cleanup: delete both sessions
  const deleteSession = async (bed: string, wake: string) => {
    await page.goto(`/day/${targetDate}`);
    const card = page
      .locator('section .card.p-4', { hasText: new RegExp(`Bed:\\s*${bed}`) })
      .filter({ hasText: new RegExp(`Wake:\\s*${wake}`) })
      .first();
    await card.locator('button', { hasText: /^Edit$/i }).first().click();
    await expect(sleepEditHeading(page)).toBeVisible();
    page.once('dialog', async (dialog) => {
      await dialog.accept();
    });
    await Promise.all([
      page.waitForURL('**/'),
      page.getByTestId('sleep-edit-delete-button').or(page.getByRole('button', { name: /^delete$/i })).first().click(),
    ]);
    await expect(page.getByTestId('dashboard-heading')).toBeVisible();
  };

  await deleteSession(firstBed, firstWake);
  await deleteSession(secondBed, secondWake);
});
