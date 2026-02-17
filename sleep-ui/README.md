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
   - `npm run test:e2e:full`

## Notes

- Bootstrap logs in through the existing `/login` page and stores auth state at `.playwright-cli/auth/storage-state.json`.
- Missing credentials fail fast with an actionable setup error.
- `test:e2e:auth` is an auth-smoke check; use `test:e2e:full` for the full authenticated scenario set.
- Never log or commit credential values, cookies, or `.playwright-cli/` auth artifacts.
- `sleep-ui/.env` is local-only and gitignored.
