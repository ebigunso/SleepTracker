import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

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

function currentRunId(): string {
  return process.env.PLAYWRIGHT_E2E_RUN_ID ?? 'default';
}

async function sleep(ms: number): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, ms));
}

function processAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

async function terminateProcess(pid: number): Promise<void> {
  if (pid <= 0) return;

  if (process.platform === 'win32') {
    spawnSync('taskkill', ['/PID', String(pid), '/T', '/F'], { stdio: 'ignore', windowsHide: true });
    return;
  }

  const processGroupId = -Math.abs(pid);

  try {
    process.kill(processGroupId, 'SIGTERM');
  } catch {
    try {
      process.kill(pid, 'SIGTERM');
    } catch {
      return;
    }
  }

  for (let index = 0; index < 20; index += 1) {
    if (!processAlive(pid)) return;
    await sleep(100);
  }

  try {
    process.kill(processGroupId, 'SIGKILL');
  } catch {
    try {
      process.kill(pid, 'SIGKILL');
    } catch {
      // ignore
    }
  }
}

async function removeDbWithRetry(dbFilePath: string): Promise<void> {
  if (!dbFilePath) return;
  let lastError: unknown = null;
  for (let attempt = 0; attempt < 10; attempt += 1) {
    try {
      await fs.rm(dbFilePath, { force: true });
      return;
    } catch (error) {
      lastError = error;
      await sleep(100 * (2 ** attempt));
    }
  }

  console.warn(
    `[e2e-safety] Failed to remove DB artifact after retries: ${dbFilePath}`,
    lastError
  );
}

export default async function globalTeardown(): Promise<void> {
  const runtimeStatePath = path.join(runtimeRoot, `run-${currentRunId()}`, 'runtime-state.json');
  let raw: string;
  try {
    raw = await fs.readFile(runtimeStatePath, 'utf8');
  } catch {
    return;
  }

  let state: RuntimeState;
  try {
    state = JSON.parse(raw) as RuntimeState;
  } catch {
    return;
  }

  if (!state.skipped && state.pid > 0) {
    await terminateProcess(state.pid);
  }

  if (!state.retainArtifacts && !state.skipped) {
    await removeDbWithRetry(state.dbFilePath);
  }

  try {
    await fs.rm(runtimeStatePath, { force: true });
  } catch {
    // ignore
  }
}
