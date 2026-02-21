# How to Run (Local Development)

This file documents the minimum steps to run and validate the repository locally.
Update it whenever local dev steps change.

---

## Prerequisites

- Runtime(s): Rust (for `sleep-api`), Node.js 20+ (for `sleep-ui`)
- Package manager(s): npm, cargo
- OS assumptions: Windows/macOS/Linux (commands below are shell-compatible)
- Required env vars: `PLAYWRIGHT_EMAIL`, `PLAYWRIGHT_PASSWORD` for authenticated Playwright flows

---

## Install

From repository root:

  cd sleep-ui
  npm ci

Backend dependencies are handled by Cargo during build/run.

---

## Run (dev)

- API:
  - Command: `cargo run -p sleep-api`
  - Default host/port: `0.0.0.0:8080`
  - Override bind: set `API_BIND_ADDR`, e.g. `API_BIND_ADDR=127.0.0.1:18080`
- UI:
  - Command: `cd sleep-ui && npm run dev`
  - Default host/port: `127.0.0.1:5173`
  - API proxy target: `PROXY_TARGET` (defaults to `http://localhost:8080`)

---

## Readiness checks

- API readiness URL: `http://127.0.0.1:8080/api/health` (or your configured bind)
- UI readiness URL: `http://127.0.0.1:5173/`
- Expected signal:
  - API `HEAD /api/health` returns success
  - UI loads login/dashboard route successfully

---

## Common workflows

- Safe authenticated E2E:
  - `cd sleep-ui && npm run test:e2e`
  - Starts isolated API + disposable DB harness, then runs Playwright authenticated suite.
- Auth bootstrap only:
  - `cd sleep-ui && npm run e2e:auth:bootstrap`
- Auth smoke only:
  - `cd sleep-ui && npm run test:e2e:auth`

---

## Shutdown / cleanup

- Stop API/UI dev servers with `Ctrl+C` in each terminal.
- Playwright isolated E2E DB/runtime artifacts are cleaned by global teardown unless `PLAYWRIGHT_E2E_RETAIN=1` is set.
