# Changelog

All notable changes to this project will be documented in this file.

This project adheres to Keep a Changelog and uses Semantic Versioning.

## [Unreleased]

### Added
- Comprehensive rustdoc across the sleep-api crate:
  - Crate-level docs with overview, example, and links to OpenAPI and API examples (C-EXAMPLE, C-LINK).
  - Module docs for app, config, db, domain, models, repository, time, trends, and views.
  - Item-level docs for public types and functions with “why-driven” examples that use the `?` operator (C-EXAMPLE, C-QUESTION-MARK).
  - Error sections on fallible functions, documenting expected error variants (C-FAILURE).
  - DST-aware behavior explained for time::compute_duration_min with example (C-EXAMPLE).
- Cargo metadata fields in sleep-api/Cargo.toml (authors, description, license, repository, keywords, categories) (C-METADATA).
- Link to Release Notes from crate-level documentation (C-RELNOTES).
- API: Added /api/session (GET) session probe and HEAD /health; OpenAPI updated accordingly.
- UI: Server-side auth guard in SvelteKit (+layout.server.ts) redirects unauthenticated requests to /login to prevent SSR of protected pages.

### Changed
- trends_page error handling to log template rendering errors and avoid unwraps in application code.
- Intra-doc links added between related items (e.g., models ↔ repository ↔ time) (C-LINK).
- Backend: Root "/" now returns 204 No Content (API-only; HTML removed). DELETE /sleep/{id} is idempotent and always returns 204 when authorized.
- UI: Dev Dockerfile now uses package-lock.json with npm ci for deterministic builds; vite proxy includes /login; removed unused deps (@vite-pwa/sveltekit, zod, @types/cookie).
- Tests: Added end-to-end checks for /api/session pre/post login and after logout; HEAD /health; and idempotent DELETE behavior.

### Hidden
- Marked impl From<DomainError> for ApiError as #[doc(hidden)] to avoid surfacing non-actionable internals in public docs (C-HIDDEN).

### Notes
- Doc examples that interact with the database are marked as `no_run` and use hidden setup lines to compile but avoid external effects. All examples prefer `?` over unwrap/try! (C-QUESTION-MARK).
- Ensure CI runs `cargo doc` and `cargo test --doc` with rustdoc link lints enabled for ongoing quality.

## [0.1.0] - 2025-08-11
### Added
- Initial SleepTracker API crate structure (Axum router, SQLx repository, models, time utilities).
- Basic routes and trends endpoints.
- Initial migrations and OpenAPI spec.
