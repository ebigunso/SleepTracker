# Frontend Refactor Quality Baseline

## Purpose

This baseline defines stable, high-level principles for frontend refactors in SleepTracker.
Use this document to preserve auth/session boundaries, consistent API interaction behavior, semantic theming, and resilient user flows while improving internals.

## Principles

- Preserve SSR and auth boundary intent.
  - Refactors must keep server/client responsibilities clear so session and authorization behavior remains correct and predictable across routes.

- Keep API interaction behavior centralized.
  - Request/response handling, CSRF token usage, and shared error handling should stay coordinated through the common client path rather than being reintroduced ad hoc in feature code.

- Maintain semantic theming contracts.
  - UI changes should continue to rely on repository semantic theme tokens/components so dark/light behavior and visual consistency are preserved through refactors.

- Protect end-to-end flow resilience.
  - Core user journeys (for example login and daily logging/editing flows) must remain reliable across loading, validation, and failure states during structural changes.

- Prefer incremental, behavior-preserving UI refactors.
  - Improve internal composition and state boundaries without silently changing user-visible outcomes unless explicitly intended.

- Keep state and side effects predictable.
  - Refactors should reduce accidental divergence between route-level data, client state, and mutation outcomes.

## Anti-Patterns To Avoid

- Boundary erosion: moving auth/session-sensitive behavior to inappropriate client-only paths.
- API sprawl: bypassing the shared client/CSRF pathway with one-off fetch logic.
- Theme regressions: introducing hard-coded visual values that bypass semantic tokens.
- Fragile happy-path UX: refactors that work only when requests succeed instantly.
- Hidden behavior shifts: changing route or interaction semantics under the label of cleanup.

## How To Use In Planning and Review

- During planning: identify which principle is most exposed (auth boundary, API centralization, theming, or flow resilience).
- During implementation: use these principles as constraints when reorganizing routes, stores, and components.
- During review: prioritize user-flow integrity and boundary correctness over purely local code simplification.
- For detailed checks: pair this baseline with architecture/language-specific gate documents when they are in scope.
