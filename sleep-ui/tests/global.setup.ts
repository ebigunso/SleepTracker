import fs from 'node:fs/promises';
import { constants as fsConstants } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawn } from 'node:child_process';

type RuntimeState = {
  pid: number;
  apiBaseUrl: string;
  dbFilePath: string;
  runtimeDir: string;
  retainArtifacts: boolean;
  skipped: boolean;
};

const dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(dirname, '../..');
const runtimeRoot = path.resolve(repoRoot, '.playwright-cli/e2e-runtime');
const dbRoot = path.resolve(repoRoot, '.playwright-cli/e2e-db');

function currentRunId(): string {
  return process.env.PLAYWRIGHT_E2E_RUN_ID ?? 'default';
}

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

function toSqliteUrl(filePath: string): string {
  const normalized = filePath.replace(/\\/g, '/');
  return `sqlite:///${normalized}`;
}

async function waitForApiReady(apiBaseUrl: string, timeoutMs: number): Promise<void> {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const response = await fetch(`${apiBaseUrl}/api/session`);
      if (response.status === 200 || response.status === 401) return;
    } catch {
      // retry
    }
    await new Promise((resolve) => setTimeout(resolve, 300));
  }
  throw new Error(`Timed out waiting for API readiness at ${apiBaseUrl}`);
}

export default async function globalSetup(): Promise<void> {
  const allowNonIsolated = process.env.ALLOW_NON_ISOLATED_E2E === '1';
  const retainArtifacts = process.env.PLAYWRIGHT_E2E_RETAIN === '1';
  const apiBaseUrl = process.env.E2E_API_BASE_URL ?? 'http://127.0.0.1:18080';
  const apiUrl = new URL(apiBaseUrl);

  if (!apiUrl.port) {
    throw new Error(
      `E2E_API_BASE_URL must include an explicit port (current: ${apiBaseUrl}). Example: http://127.0.0.1:18080`
    );
  }

  if (!allowNonIsolated && !isLocalHttpUrl(apiBaseUrl)) {
    throw new Error(
      `Unsafe E2E API target: ${apiBaseUrl}. Only localhost targets are allowed unless ALLOW_NON_ISOLATED_E2E=1 is set.`
    );
  }

  await fs.mkdir(runtimeRoot, { recursive: true });
  await fs.mkdir(dbRoot, { recursive: true });

  const runtimeDir = path.join(runtimeRoot, `run-${currentRunId()}`);
  const runtimeStatePath = path.join(runtimeDir, 'runtime-state.json');
  const dbFilePath = path.join(dbRoot, `e2e-${Date.now()}.db`);

  await fs.mkdir(runtimeDir, { recursive: true });

  if (allowNonIsolated) {
    const skippedState: RuntimeState = {
      pid: -1,
      apiBaseUrl,
      dbFilePath: '',
      runtimeDir,
      retainArtifacts,
      skipped: true
    };
    await fs.writeFile(runtimeStatePath, JSON.stringify(skippedState, null, 2), 'utf8');
    console.warn('[e2e-safety] ALLOW_NON_ISOLATED_E2E=1 enabled. Isolation harness is bypassed.');
    return;
  }

  await fs.writeFile(dbFilePath, '', 'utf8');

  const bindHost = apiUrl.hostname === 'localhost' ? '127.0.0.1' : apiUrl.hostname;
  const bindPort = apiUrl.port || '18080';
  const apiBindAddr = `${bindHost}:${bindPort}`;

  const env = {
    ...process.env,
    DATABASE_URL: toSqliteUrl(dbFilePath),
    API_BIND_ADDR: apiBindAddr,
    COOKIE_SECURE: '0',
    APP_TZ: process.env.APP_TZ ?? 'Asia/Tokyo'
  };

  const child = spawn('cargo', ['run', '-p', 'sleep-api', '--bin', 'sleep-api'], {
    cwd: repoRoot,
    env,
    detached: true,
    stdio: 'ignore',
    windowsHide: true
  });

  if (!child.pid) {
    throw new Error('Failed to start isolated sleep-api process for E2E');
  }

  child.unref();
  await waitForApiReady(apiBaseUrl, 60_000);

  const state: RuntimeState = {
    pid: child.pid,
    apiBaseUrl,
    dbFilePath,
    runtimeDir,
    retainArtifacts,
    skipped: false
  };

  await fs.writeFile(runtimeStatePath, JSON.stringify(state, null, 2), 'utf8');

  await fs.access(dbFilePath, fsConstants.F_OK);
}
