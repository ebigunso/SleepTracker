# sleep-ui

## Authenticated Playwright workflow

Use the frontend auth bootstrap commands when running authenticated E2E:

1. Add local credentials to `sleep-ui/.env`:
   - `PLAYWRIGHT_EMAIL=...`
   - `PLAYWRIGHT_PASSWORD=...`
2. Run bootstrap:
   - `npm run e2e:auth:bootstrap`
3. Run authenticated E2E:
   - `npm run test:e2e:auth`
4. (Optional) Run full authenticated suite:
   - `npm run test:e2e`

Safety defaults:

- `npm run test:e2e` is safe-by-default and uses a harness-managed local API with an isolated disposable DB.
- Runs fail fast when the API target is non-local or isolation preconditions are unsafe.
- Bypass is explicit only via `ALLOW_NON_ISOLATED_E2E=1 npm run test:e2e`.
- Do not use unsafe mode against live/shared data.
- E2E starts its own UI dev server on port `5173` with strict port binding; stop any existing process already using `5173` before running Playwright.

## Notes

- Bootstrap logs in through the existing `/login` page and stores auth state at `.playwright-cli/auth/storage-state.json`.
- Missing credentials fail fast with an actionable setup error.
- `test:e2e:auth` is an auth-smoke check; use `test:e2e` for the full authenticated scenario set.
- Never log or commit credential values, cookies, or `.playwright-cli/` auth artifacts.
- `sleep-ui/.env` is local-only and gitignored.
