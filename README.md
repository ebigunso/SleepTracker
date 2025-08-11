# Sleep Tracker

SleepTracker is a small, single-user web API for tracking sleep sessions, built with Axum, SQLx, and SQLite. It includes first-class authentication, encrypted session cookies, CSRF protection via double-submit, and common security headers.

## Quick start

1) Copy env file and adjust values:
- Copy .env.example to .env
- Set ADMIN_EMAIL and ADMIN_PASSWORD_HASH
  - Generate a password hash:
    cargo run -p sleep-api --bin pw-hash
  - Paste the $argon2id$... string into ADMIN_PASSWORD_HASH
- Set SESSION_SECRET to a base64-encoded random value (32+ bytes recommended)
- Optional: Set ENABLE_HSTS=1 when serving over HTTPS
- Optional: See COOKIE_SECURE below for local HTTP development

2) Run database migrations and start the server:
- The server runs migrations on startup for the main binary
- Start the server:
  cargo run -p sleep-api

Server will listen on 0.0.0.0:8080.

## Authentication and sessions

- Single-user login based on ADMIN_EMAIL and ADMIN_PASSWORD_HASH.
- Endpoint: POST /login
  - Accepts both application/json and application/x-www-form-urlencoded
  - Payload schema:
    { "email": "...", "password": "..." }
  - On success, the server issues:
    - Encrypted session cookie (__Host-session by default)
    - CSRF cookie (__Host-csrf by default)
- Endpoint: POST /logout — clears session and CSRF cookies.

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

## OpenAPI

OpenAPI specification is in openapi.yaml and includes:
- /login and /logout endpoints
- Cookie-based session authentication scheme
- Double-submit CSRF requirement (X-CSRF-Token) on mutating endpoints

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
