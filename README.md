# Sleep Tracker

SleepTracker is a small, single-user web API for tracking sleep sessions, built with Axum, SQLx, and SQLite. It includes first-class authentication, encrypted session cookies, CSRF protection via double-submit, and common security headers.

## Quick start

1) Copy env file and adjust values:
- Copy .env.example to .env
- Set ADMIN_EMAIL and ADMIN_PASSWORD_HASH
  - Generate a password hash:
    cargo run -p sleep-api --bin pw-hash
  - Paste the $argon2id$... string into ADMIN_PASSWORD_HASH (IMPORTANT: use single quotes in .env/.env.docker to prevent $-expansion by dotenv)
    Example:
      ADMIN_PASSWORD_HASH='$argon2id$v=19$m=19456,t=2,p=1$...$...'
- Set SESSION_SECRET to a base64-encoded random value (32+ bytes recommended)
- Optional: Set ENABLE_HSTS=1 when serving over HTTPS
- Optional: See COOKIE_SECURE below for local HTTP development

2) Run database migrations and start the server:
- The server runs migrations on startup for the main binary
- Start the server:
  cargo run -p sleep-api

Server will listen on 0.0.0.0:8080.

## Run with Docker

Use Docker Compose to build and run the app.

- Docker Compose (recommended)
  - Copy .env.docker.example to .env.docker and fill values:
    - ADMIN_EMAIL, ADMIN_PASSWORD_HASH (quote the $argon2id$... string with single quotes)
    - SESSION_SECRET: base64-encoded random value (32+ bytes)
    - For local HTTP development, set COOKIE_SECURE=0. For HTTPS/prod, use COOKIE_SECURE=1.
  - Build and start:
    docker compose up --build
    - Add -d to run in the background.
  - Access the API at http://localhost:8080
  - Follow logs:
    docker compose logs -f api
  - Stop:
    docker compose down
  - Stop and delete the persistent data volume (DESTROYS DB):
    docker compose down -v
  - Notes:
    - Data is stored at /data inside the container and persists in a named volume across restarts.
    - Migrations run automatically on startup.


## Authentication and sessions

- Single-user login based on ADMIN_EMAIL and ADMIN_PASSWORD_HASH.
- Endpoint: POST /api/login
  - Accepts both application/json and application/x-www-form-urlencoded
  - Payload schema:
    { "email": "...", "password": "..." }
  - On success, the server issues:
    - Encrypted session cookie (__Host-session by default)
    - CSRF cookie (__Host-csrf by default)
- Endpoint: POST /api/logout — clears session and CSRF cookies.

Session cookie properties:
- Encrypted/signed via axum-extra PrivateCookieJar using a key derived from SESSION_SECRET
- HttpOnly, SameSite=Lax, Path=/
- Secure when COOKIE_SECURE is true (default)

## CSRF protection (double-submit)

Mutating routes (POST, PUT, DELETE) require:
- A CSRF cookie (default name: __Host-csrf), and
- A header X-CSRF-Token whose value equals the CSRF cookie value
  - The header value is percent-decoded before comparison to tolerate encodings like %2F
- If the Sec-Fetch-Site header is present, it must be same-origin or same-site

This approach is the classic double-submit pattern. Tokens are random per-login and are not derived from a separate CSRF secret.

## Local development over HTTP and cookie behavior

The __Host- cookie prefix enforces Secure + Path=/ and additional constraints in browsers; cookies with __Host- are ignored over http:// schemes.

Options for development:
- Recommended: run behind TLS (e.g., mkcert or a reverse proxy) so __Host- cookies work as-is.
- Dev mode switch (implemented): set COOKIE_SECURE=false in .env when serving over plain HTTP during local development.
  - In this mode:
    - Cookie names change to "session" (instead of __Host-session) and "csrf" (instead of __Host-csrf)
    - Cookies are sent without the Secure attribute
  - Do not use this setting in production.

## Security headers

The API applies the following headers to all responses:
- X-Content-Type-Options: nosniff
- X-Frame-Options: DENY
- Referrer-Policy: strict-origin-when-cross-origin
- Content-Security-Policy: default-src 'self'; script-src 'self' 'unsafe-inline'
  - TODO: Move to nonces/hashes and remove 'unsafe-inline' when templates are adjusted
- Strict-Transport-Security (HSTS) when ENABLE_HSTS=1/true

## SvelteKit UI (frontend)

For local UI development:
- cd sleep-ui
- npm ci
- npm run dev

The dev server runs at http://localhost:5173 and proxies API calls to http://localhost:8080 via vite.config.ts. Authentication is cookie-based with CSRF double-submit.

Server-side route protection:
- +layout.server.ts fetches /api/session during SSR and redirects unauthenticated requests to /login. This prevents rendering protected pages on the server and avoids client-side flashes.

Local HTTP note:
- For local HTTP development, set COOKIE_SECURE=0 in the API environment so non-__Host- cookies are accepted over http. Do not use this setting in production.

## Playwright authenticated E2E bootstrap

Use these commands from `sleep-ui/` when E2E requires an authenticated session:

- `npm run e2e:auth:bootstrap`
  - Logs in through the real `/login` UI flow.
  - Reads `PLAYWRIGHT_EMAIL` and `PLAYWRIGHT_PASSWORD` from `sleep-ui/.env` (or process env).
  - Writes browser storage state to `.playwright-cli/auth/storage-state.json`.
- `npm run test:e2e:auth`
  - Runs the authenticated bootstrap smoke check using the generated storage state.
- `npm run test:e2e`
  - Runs the full authenticated Playwright suite using the generated storage state.

Secret handling rules:

- Do not print, paste, or commit secret values (`PLAYWRIGHT_EMAIL`, `PLAYWRIGHT_PASSWORD`).
- Do not commit `.playwright-cli/` artifacts or auth state.
- Keep credentials in local-only env files (for this repo: `sleep-ui/.env`, already ignored by git).

## OpenAPI

OpenAPI specification is in openapi.yaml and includes:
- /api/login and /api/logout endpoints
- Cookie-based session authentication scheme
- Double-submit CSRF requirement (X-CSRF-Token) on mutating endpoints
- /api/session endpoint for session probe (GET)
- HEAD /api/health endpoint

## Personalization rollout flags (safe defaults)

Personalization endpoints are rollout-gated and default to off, preserving legacy behavior.

- `ENABLE_PERSONALIZATION_TRENDS=1` enables `GET /api/trends/personalization`
- `ENABLE_PERSONALIZATION_FRICTION_TELEMETRY=1` enables `POST /api/personalization/friction-telemetry`
- `ENABLE_PERSONALIZATION_FRICTION_BACKLOG=1` enables `GET /api/personalization/friction-backlog`

When a flag is off, its endpoint is not registered and returns `404` (same behavior as pre-rollout builds).

Guardrail/confidence policy for personalization actions:
- Apply actions only when trigger + guardrails are satisfied.
- Auto-promote only when confidence is `medium` or `high`.
- Roll back or suppress a proposal when persistence no longer holds for two windows or confidence drops below `medium`.

See `docs/personalization-agent-action-map.md` for the metric-level trigger thresholds and guardrails.

## Sleep sessions, wake dates, and timezones

- Sleep entries use wake-date semantics: the `date` field is the wake date (local).
  - If `bed_time` is later than `wake_time`, the bed datetime is treated as the previous calendar day.
- Multiple sessions per wake date are supported. `GET /api/sleep/date/{date}` returns an array (possibly empty).
- `GET /api/sleep/range` returns per-session rows ordered by date ascending, then `wake_time` ascending.
- Overlap is rejected: any overlap, including end == start, returns 400 with an error message.
- Duration calculations are timezone-aware (DST-aware). The API uses the saved user timezone or falls back to `APP_TZ` (default `Asia/Tokyo`).
  - Set the timezone via `POST /api/settings/timezone` with `{ "timezone": "Asia/Tokyo" }` (IANA name).

## Building, formatting, linting, testing

- Format:
  cargo fmt --all

- Lint (treat warnings as errors):
  cargo clippy --workspace -- -D warnings

- Test:
  cargo test

## Notes

- The cookie encryption Key is derived from SESSION_SECRET if present; otherwise a random key is generated (sessions will break on restart in that case).
- Default database is sqlite::memory: for ephemeral dev/testing. For a persistent DB use DATABASE_URL=sqlite://./data/sleep.db and create the directory.

## Environments

- Local (cargo run):
  - Use .env for local settings (e.g., COOKIE_SECURE=0 for http://).
  - Start: `cargo run -p sleep-api`.

- Docker Compose:
  - Copy `.env.docker.example` to `.env.docker` and fill values (ADMIN_EMAIL, ADMIN_PASSWORD_HASH, SESSION_SECRET; optionally COOKIE_SECURE=1).
  - Compose injects only `.env.docker` into the container; your local `.env` is not used inside the container.
  - Start: `docker compose up --build`.

- Paths in Docker:
  - DATABASE_URL should point to the named volume path: `sqlite:///data/sleep.db`.
