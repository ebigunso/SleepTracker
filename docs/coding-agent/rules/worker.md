# Repository Rules (Worker)

- last_updated: 2026-02-03

## Repo-Specific Worker Notes
- Common pitfalls in this repo: local HTTP needs COOKIE_SECURE=0 or __Host- cookies will not be set; mutating API calls require CSRF double-submit (cookie + X-CSRF-Token); sleep `date` uses wake-date semantics and overlapping sessions are rejected.
- Preferred patterns / libraries: Rust Axum + SQLx in sleep-api; SvelteKit + Tailwind in sleep-ui; use sleep-ui/src/lib/api.ts for API calls; use existing models in sleep-api/src/models.
- Style rules that are not auto-enforced: keep SvelteKit route structure (+page/+layout) intact; keep API request/response shapes aligned with openapi.yaml.
- Any directories that require extra caution: migrations/ (do not edit existing migrations), sleep-api/src/security/, sleep-api/src/auth.rs, sleep-api/src/middleware/.

## Repo CI / Checks Mapping
- If you touch sleep-api/**, Cargo.toml, Cargo.lock, migrations/**, run cargo fmt -- --check; cargo clippy -- -D warnings; cargo test / expect CI Backend.
- If you touch sleep-ui/** or sleep-ui/Dockerfile, run npm ci; npm run check; npm run test:unit; npm run build / expect CI Frontend.
- If you touch Dockerfile, docker-entrypoint.sh, compose.yaml, .cargo/**, sleep-api/**, Cargo.toml, Cargo.lock, or migrations/**, run docker build -t sleep-api:ci . / expect CI Docker.

## Mandatory checks policy (repo-enforced)

If your changes touch a path that has mapped checks in this file:
- those checks are REQUIRED before reporting status: done
- skipping requires explicit user waiver, otherwise return status: blocked

## Global Migration Candidates

<!--
Add cross-repo / cross-task rules discovered at runtime here.
These will be migrated into global agent/skills definitions later.
-->
