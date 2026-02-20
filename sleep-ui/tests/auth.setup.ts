import { test, expect } from '@playwright/test';
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import dotenv from 'dotenv';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = path.resolve(dirname, '../..');
const AUTH_STATE_PATH = path.join(REPO_ROOT, '.playwright-cli/auth/storage-state.json');
const E2E_RUNTIME_STATE_PATH = path.join(REPO_ROOT, '.playwright-cli/e2e-runtime/runtime-state.json');
const DOTENV_PATH = path.resolve(dirname, '../.env');

type Credentials = {
  email: string;
  password: string;
};

dotenv.config({ path: DOTENV_PATH });

function isLocalHttpUrl(value: string): boolean {
  try {
    const url = new URL(value);
    const host = url.hostname;
    return (
      (url.protocol === 'http:' || url.protocol === 'https:') &&
      (host === 'localhost' || host === '127.0.0.1' || host === '::1')
    );
  } catch {
    return false;
  }
}

async function assertSafeE2EContext(): Promise<void> {
  const allowNonIsolated = process.env.ALLOW_NON_ISOLATED_E2E === '1';
  const runtimeRaw = await fs.readFile(E2E_RUNTIME_STATE_PATH, 'utf8').catch(() => null);
  const runtimeState = runtimeRaw ? JSON.parse(runtimeRaw) as { apiBaseUrl?: string; skipped?: boolean } : null;
  const apiBaseUrl = runtimeState?.apiBaseUrl ?? process.env.E2E_API_BASE_URL ?? 'http://127.0.0.1:18080';

  if (!allowNonIsolated && (!runtimeState || runtimeState.skipped)) {
    throw new Error('Unsafe E2E context: isolated runtime state missing. Run through Playwright global setup (npm run test:e2e).');
  }

  if (!allowNonIsolated && !isLocalHttpUrl(apiBaseUrl)) {
    throw new Error(
      `Unsafe E2E API target: ${apiBaseUrl}. Only localhost targets are allowed unless ALLOW_NON_ISOLATED_E2E=1 is set.`
    );
  }
}

async function resolveCredentials(): Promise<Credentials> {
  const email = process.env.PLAYWRIGHT_EMAIL;
  const password = process.env.PLAYWRIGHT_PASSWORD;

  if (!email || !password) {
    throw new Error(
      [
        'Missing Playwright auth credentials.',
        'Set PLAYWRIGHT_EMAIL and PLAYWRIGHT_PASSWORD in sleep-ui/.env or your shell environment.',
        'Then rerun: npm run e2e:auth:bootstrap'
      ].join(' ')
    );
  }

  return { email, password };
}

test('bootstrap authenticated storage state', async ({ page, context }) => {
  await assertSafeE2EContext();
  const { email, password } = await resolveCredentials();

  await fs.mkdir(path.dirname(AUTH_STATE_PATH), { recursive: true });

  await context.clearCookies();
  await page.goto('/login');
  await page.waitForLoadState('domcontentloaded');
  await expect(page.getByLabel('Email')).toBeVisible();

  await page.getByLabel('Email').fill(email);
  await page.getByLabel('Password', { exact: true }).fill(password);

  const [loginResponse] = await Promise.all([
    page.waitForResponse(
      (response) =>
        response.url().includes('/api/login') && response.request().method() === 'POST'
    ),
    page.getByRole('button', { name: 'Sign in' }).click()
  ]);

  if (loginResponse.status() >= 400) {
    throw new Error(
      [
        `Login failed during bootstrap (HTTP ${loginResponse.status()}).`,
        'Confirm API is running, credentials are correct, and COOKIE_SECURE=0 for local http:// development.',
        'Then rerun: npm run e2e:auth:bootstrap'
      ].join(' ')
    );
  }

  await expect(page.getByTestId('dashboard-heading')).toBeVisible();
  await context.storageState({ path: AUTH_STATE_PATH });
});
