# Svelte/TypeScript Refactor Quality Gates

## Purpose

Use these gates for frontend refactors in `sleep-ui` to keep SvelteKit behavior, TypeScript safety, and test reliability aligned with repository standards.

These gates are intentionally language/framework-specific. Cross-layer architecture concerns belong in architecture-level gate docs.

## Required vs Recommended Validation Semantics

Run from repository root unless noted:

Canonical required validations are defined by path in `docs/coding-agent/references/validation.md`:

- For `sleep-ui/src/**`: `cd sleep-ui && npm run check`, `cd sleep-ui && npm run test:unit`
- For `sleep-ui/tests/**`: `cd sleep-ui && npm run test:e2e` (or targeted equivalent + rationale)

Recommended additional gates for frontend refactors:

- Build confidence gate: `cd sleep-ui && npm run build`
- Flow-impact gate (route/form/auth/navigation behavior touched): `cd sleep-ui && npm run test:e2e` with `.playwright-cli/` evidence per `docs/coding-agent/references/ui-e2e.md`

If running targeted E2E instead of full suite, include rationale and ensure equivalent coverage for changed flow.

## Gates

### Gate 1 — SSR Auth Boundary Is Preserved

- Keep route auth guard behavior server-side (`+layout.server.ts` load path), including:
  - authenticated users are redirected away from `/login`
  - unauthenticated users are redirected to `/login` for protected routes
  - session check remains based on `/api/session` and SSR load semantics
- Refactors must not shift auth gating into client-only lifecycle code.

Evidence to seek:
- `npm run check` passes
- `npm run test:e2e` (or targeted auth flow equivalent) confirms redirects/session behavior

### Gate 2 — Shared API Wrapper Is The Single Fetch Path

- Use `sleep-ui/src/lib/api.ts` wrappers (`apiGet`, `apiPost`, `apiPut`, `apiDelete`, `apiFetch`) for API calls.
- Do not introduce ad hoc `fetch` usage in feature code for API mutations.
- Mutating requests must keep credentials + CSRF header behavior inherited from wrapper helpers.

Evidence to seek:
- Code review confirms changed API calls flow through wrapper helpers
- `npm run test:unit` and relevant E2E flow pass

### Gate 3 — CSRF/Credential Behavior Is Not Reimplemented

- Do not duplicate cookie parsing, CSRF token read logic, or request header assembly outside shared API utility.
- Preserve double-submit pattern assumptions (cookie + `X-CSRF-Token`) for mutating methods.

Evidence to seek:
- No new duplicated CSRF helper logic in feature modules
- Mutating flow tests continue passing (unit and/or E2E)

### Gate 4 — TypeScript/Svelte Safety Is Maintained

- Refactors must compile under `svelte-check` (`npm run check`) without relaxing types to bypass errors.
- Avoid introducing broad `any`/unsafe casts to force compatibility.
- Maintain strongly typed API payload and response usage where touched.

Evidence to seek:
- `npm run check` passes
- `npm run build` passes

### Gate 5 — Semantic Theme Tokens And Component Classes Are Preserved

- UI styling changes must use existing semantic tokens/classes from `sleep-ui/src/app.css` (for example: `page-title`, `section-title`, `card`, semantic color variables).
- Do not hard-code new one-off colors/shadows that bypass tokenized dark/light behavior.
- Preserve theme cookie + SSR theme handoff behavior when touching layout/theme paths.

Evidence to seek:
- Visual review shows unchanged dark/light semantics for affected views
- Changed classes/tokens map to existing `app.css` semantics

### Gate 6 — E2E Flow Stability And Selector Discipline

- Keep selectors resilient:
  - prefer stable `getByTestId(...)`
  - allow semantic fallback via `getByRole(...)` / `getByLabel(...)` where appropriate
  - avoid brittle structural selectors unless unavoidable
- Preserve navigation/assertion synchronization patterns (`waitForURL`, response waits) for async flows.
- For UI behavior changes requiring evidence, collect artifacts under `.playwright-cli/` per UI/E2E reference.

Evidence to seek:
- Relevant E2E flow passes (`npm run test:e2e` or targeted equivalent)
- Required screenshots/evidence exist under `.playwright-cli/` when UI evidence is in scope

## Anti-Patterns

- Moving auth redirect/session gating from SSR load to client-only route logic
- Calling `fetch` directly for API mutations instead of shared API wrappers
- Re-implementing CSRF cookie/header plumbing in components/routes
- Silencing type problems with broad casts instead of fixing the contract
- Introducing hard-coded visual values that bypass semantic tokens
- Writing fragile E2E selectors tied to incidental DOM structure

## How To Use In Planning and Review

- During planning:
  - mark which gates are impacted by each refactor task
  - pre-select canonical required commands from `references/validation.md`, then add recommended gates (`build`, plus `test:e2e` when flow/auth/routes are touched)
- During implementation:
  - keep API and auth boundary changes small and traceable to these gates
  - update selectors/tests in the same change when UI semantics move
- During review:
  - verify canonical required evidence is present, plus flow-impact E2E artifacts when that guidance is in scope
  - reject changes that pass locally only by bypassing wrapper, SSR guard, or semantic token conventions
