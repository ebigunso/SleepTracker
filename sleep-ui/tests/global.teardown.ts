import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

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
const runtimeStatePath = path.join(runtimeRoot, 'runtime-state.json');

export default async function globalTeardown(): Promise<void> {
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
    try {
      process.kill(state.pid, 'SIGTERM');
    } catch {
      // ignore process-already-exited
    }
  }

  if (!state.retainArtifacts && !state.skipped) {
    try {
      if (state.dbFilePath) await fs.rm(state.dbFilePath, { force: true });
    } catch {
      // ignore cleanup errors
    }
  }

  try {
    await fs.rm(runtimeStatePath, { force: true });
  } catch {
    // ignore
  }
}
