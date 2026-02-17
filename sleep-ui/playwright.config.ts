import { defineConfig, devices } from '@playwright/test';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const dirname = path.dirname(fileURLToPath(import.meta.url));
const authStatePath = path.resolve(dirname, '../.playwright-cli/auth/storage-state.json');

export default defineConfig({
  testDir: './tests',
  testIgnore: ['**/unit/**'],
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
    command: 'npm run dev',
    port: 5173,
    reuseExistingServer: true,
    timeout: 120_000
  }
});
