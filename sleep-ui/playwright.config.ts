import { defineConfig, devices } from '@playwright/test';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const authStatePath = path.resolve(dirname, '../.playwright-cli/auth/storage-state.json');
const e2eApiBaseUrl = process.env.E2E_API_BASE_URL ?? 'http://127.0.0.1:18080';

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

const allowNonIsolated = process.env.ALLOW_NON_ISOLATED_E2E === '1';
if (!allowNonIsolated && !isLocalHttpUrl(e2eApiBaseUrl)) {
  throw new Error(
    `Unsafe E2E API target: ${e2eApiBaseUrl}. Set E2E_API_BASE_URL to localhost/127.0.0.1 or explicitly opt in with ALLOW_NON_ISOLATED_E2E=1.`
  );
}

export default defineConfig({
  testDir: './tests',
  testIgnore: ['**/unit/**'],
  globalSetup: './tests/global.setup.ts',
  globalTeardown: './tests/global.teardown.ts',
  timeout: 30_000,
  expect: {
    timeout: 5_000
  },
  fullyParallel: true,
  retries: 0,
  reporter: [['list']],
  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry'
  },
  projects: [
    {
      name: 'auth-setup',
      testMatch: ['**/auth.setup.ts'],
      testIgnore: ['**/unit/**'],
      use: {
        ...devices['Desktop Chrome'],
        storageState: undefined
      }
    },
    {
      name: 'e2e-auth',
      testMatch: ['**/auth.spec.ts', '**/e2e.spec.ts'],
      testIgnore: ['**/unit/**', '**/auth.setup.ts'],
      dependencies: ['auth-setup'],
      use: {
        ...devices['Desktop Chrome'],
        storageState: authStatePath
      }
    }
  ],
  webServer: {
    command: 'npm run dev -- --host 127.0.0.1 --port 5173 --strictPort',
    port: 5173,
    reuseExistingServer: false,
    timeout: 120_000,
    env: {
      PROXY_TARGET: e2eApiBaseUrl,
      PLAYWRIGHT_E2E_ISOLATED: allowNonIsolated ? '0' : '1',
      E2E_API_BASE_URL: e2eApiBaseUrl,
      ALLOW_NON_ISOLATED_E2E: process.env.ALLOW_NON_ISOLATED_E2E ?? '0'
    }
  }
});
