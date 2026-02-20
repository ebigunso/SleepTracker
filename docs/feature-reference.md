# SleepTracker feature reference (domain inventory)

This document inventories implemented capabilities by domain, with active user-visible features separated from legacy and non-UI-surfaced items.

## Active domains (in-use UI capabilities)

### 1) Auth / session

**Behavior**
- Login page authenticates with email/password and establishes session + CSRF cookies.
- App shell enforces auth gating (`/login` redirect rules) and exposes logout from profile menu.
- Session probe determines authenticated/unauthenticated state for route protection.

**Endpoints / dependencies**
- `POST /api/login`
- `GET /api/session`
- `POST /api/logout`
- SvelteKit gate in `sleep-ui/src/routes/+layout.server.ts`; login/logout UI in `sleep-ui/src/routes/login/+page.svelte` and `sleep-ui/src/routes/+layout.svelte`.

**Key constraints**
- Mutating logout requires CSRF token header matching CSRF cookie.
- Cookie names/flags depend on secure-cookie configuration.

**Source evidence**
- `sleep-api/src/app.rs` (route wiring + auth handler docs)
- `openapi.yaml` (`/api/login`, `/api/session`, `/api/logout`)
- `sleep-ui/src/routes/+layout.server.ts`, `sleep-ui/src/routes/+layout.svelte`, `sleep-ui/src/routes/login/+page.svelte`

### 2) Sleep tracking

**Behavior**
- Users create, edit, delete, and view sleep sessions (dashboard range, day detail, edit detail).
- Dashboard and day pages rely on wake-date semantics for grouping.
- Duration warning is shown for unusual sessions (<2h or >14h) before save.

**Endpoints / dependencies**
- `POST /api/sleep`
- `GET /api/sleep/date/{date}`
- `GET /api/sleep/{id}`
- `PUT /api/sleep/{id}`
- `DELETE /api/sleep/{id}`
- `GET /api/sleep/range`
- UI routes: `/`, `/day/[date]`, `/sleep/new`, `/sleep/[id]/edit`.

**Key constraints**
- Overlapping sessions are rejected on create/update.
- Range query enforces `from <= to` and max 62-day span.
- Auth required for reads; auth + CSRF required for mutating calls.

**Source evidence**
- `sleep-api/src/app.rs` (sleep route registration and validation)
- `openapi.yaml` (`/api/sleep*`)
- `sleep-ui/src/routes/+page.server.ts`, `sleep-ui/src/routes/day/[date]/+page.server.ts`, `sleep-ui/src/routes/sleep/[id]/edit/+page.server.ts`
- `sleep-ui/src/lib/components/SleepForm.svelte`

### 3) Exercise intensity

**Behavior**
- Intensity can be set during sleep create/edit flow and displayed as date badges in dashboard context.

**Endpoints / dependencies**
- `POST /api/exercise`
- `GET /api/exercise/intensity`
- UI dependency: `SleepForm` and dashboard/day data composition.

**Key constraints**
- Intensity range lookups share range validation (`from <= to`, max 62 days).
- Exercise upsert in form flow is best-effort (sleep save proceeds even if exercise write fails).

**Source evidence**
- `sleep-api/src/app.rs` (`create_exercise`, `get_exercise_intensity`)
- `openapi.yaml` (`/api/exercise`, `/api/exercise/intensity`)
- `sleep-ui/src/lib/components/SleepForm.svelte`, `sleep-ui/src/routes/+page.server.ts`, `sleep-ui/src/lib/api.ts`

### 4) Trends

**Behavior**
- Trends page renders chart and schedule views over selectable ranges (presets + custom dates).
- Chart view uses metric-specific visuals: duration as line + points, quality as points on integer 1..5 scale, bedtime/wake time as wrapped-time line + points to keep near-midnight values visually adjacent.

**Endpoints / dependencies**
- `GET /api/trends/sleep-bars`
- UI route: `/trends` (`sleep-ui/src/routes/trends/+page.svelte`), with chart rendering and `SleepBar` timeline support.

**Key constraints**
- Requires authenticated session.
- Uses wake-date semantics in returned bar records.

**Source evidence**
- `sleep-api/src/app.rs` (trends route wiring)
- `openapi.yaml` (`/api/trends/sleep-bars`)
- `sleep-ui/src/routes/trends/+page.svelte`

### 5) Settings / theme / timezone

**Behavior**
- Theme toggle is user-visible in the profile menu and persisted client-side.
- App automatically detects browser timezone and attempts to persist it to backend for DST-aware calculations.

**Endpoints / dependencies**
- `GET /api/settings/timezone`
- `POST /api/settings/timezone`
- Theme state store: `sleep-ui/src/lib/stores/theme.ts`
- Timezone sync via `setUserTimezoneIfSupported` called from `sleep-ui/src/routes/+layout.svelte`.

**Key constraints**
- Timezone set requires auth + CSRF and valid IANA timezone.
- Theme persistence uses `sleeptracker.theme` cookie with `document.documentElement.dataset.theme` application.

**Source evidence**
- `sleep-api/src/app.rs` (`get_settings_timezone`, `post_settings_timezone`)
- `openapi.yaml` (`/api/settings/timezone`)
- `sleep-ui/src/routes/+layout.server.ts`, `sleep-ui/src/routes/+layout.svelte`, `sleep-ui/src/lib/api.ts`, `sleep-ui/src/lib/stores/theme.ts`

### 6) Security

**Behavior**
- Session-cookie auth is enforced for protected JSON APIs.
- CSRF double-submit protection is enforced on mutating endpoints.
- Security headers are applied to the API router.

**Endpoints / dependencies**
- Cross-cutting across auth/logout, settings writes, sleep writes, exercise writes, and note writes.
- Middleware/guards: `RequireSessionJson`, `CsrfGuard`, header application.

**Key constraints**
- Missing/invalid session yields `401`; CSRF mismatch yields `403` on protected mutating endpoints.

**Source evidence**
- `sleep-api/src/app.rs`
- `sleep-api/src/middleware/auth_layer.rs`
- `sleep-api/src/security/csrf.rs`, `sleep-api/src/security/headers.rs`
- `openapi.yaml` security scheme + per-endpoint security requirements

### 7) Notes

**Behavior**
- Notes can be attached optionally during sleep create/edit submission.
- Current UI provides note capture input but no note listing/editing view.

**Endpoints / dependencies**
- `POST /api/note`
- UI dependency: `sleep-ui/src/lib/components/SleepForm.svelte`.

**Key constraints**
- UI sends notes only when non-empty and length <= 280.
- Note creation is best-effort in form flow.

**Source evidence**
- `sleep-api/src/app.rs` (`create_note`)
- `openapi.yaml` (`/api/note`)
- `sleep-ui/src/lib/components/SleepForm.svelte`

### 8) Personalization

**Behavior**
- Provides rolling-window personalization metrics and recommendation outputs.
- Captures friction telemetry events and produces ranked friction backlog proposals.

**Endpoints / dependencies**
- `GET /api/trends/personalization`
- `POST /api/personalization/friction-telemetry`
- `GET /api/personalization/friction-backlog`

**Guardrail/confidence/rollback policy**
- Actions are considered only when trigger conditions and guardrails are satisfied.
- Auto-promotion requires at least `medium` confidence.
- Backlog proposals include rollback conditions and are downgraded when persistence or confidence weakens.

**Source evidence**
- `sleep-api/src/app.rs` (personalization route wiring)
- `sleep-api/src/trends.rs` (personalization metrics and recommendations)
- `sleep-api/src/handlers.rs` (friction telemetry ingestion and backlog policy fields)
- `docs/personalization-agent-action-map.md` (trigger/guardrail policy)

---

## Constraints, assumptions, and known gaps (for product analysis)

### Implemented constraints (source-backed)

#### A) Date-range and day-window limits
- `GET /api/sleep/recent` accepts `days` only in `1..=31`; missing `days` defaults to `7`; out-of-range returns `400`.
- `GET /api/sleep/range` and `GET /api/exercise/intensity` require `from <= to` and inclusive span `<= 62` days; violations return `400`.
- `GET /api/trends/sleep-bars` and `GET /api/trends/summary` validate date parse/order (`to >= from`) but do not apply a 62-day cap.

**Source/test pointers**
- Source: `sleep-api/src/app.rs` (`get_sleep_recent`, `get_sleep_range`, `get_exercise_intensity`), `sleep-api/src/trends.rs` (`parse_and_validate_date_range`)
- Contract: `openapi.yaml` (`/api/sleep/recent`, `/api/sleep/range`, `/api/exercise/intensity`, `/api/trends/sleep-bars`, `/api/trends/summary`)
- Tests: `sleep-api/tests/api_sleep_list.rs` (`test_sleep_list_invalid_params`)

#### B) Overlap rejection rules (sleep create/update)
- Sleep create/update rejects overlaps before write and also maps DB overlap violations to `400`.
- Overlap is inclusive: boundary-touching windows (`end == start`) are treated as overlap and rejected.

**Source/test pointers**
- Source: `sleep-api/src/handlers.rs` (`create_sleep`, `update_sleep`, overlap error mapping), `sleep-api/src/repository.rs` (`has_sleep_overlap` docs + SQL)
- Contract: `openapi.yaml` (`/api/sleep` POST and `/api/sleep/{id}` PUT include overlap rejection in `400` behavior)
- Tests: `sleep-api/tests/api_sleep.rs` (`test_sleep_overlap_rejection_inclusive`)

#### C) Timezone + DST behavior
- Duration computation uses stored user timezone; if unavailable/invalid in settings storage, backend falls back to `APP_TZ`.
- Timezone writes require auth + CSRF and valid IANA timezone parsing; invalid timezone returns `400`.
- DST behavior in duration math:
	- ambiguous local times choose earliest instant for bed and latest instant for wake,
	- spring-forward gaps scan forward minute-by-minute (up to 3 hours) to find a valid local time,
	- if still unresolved, code falls back to UTC projection with warning (best-effort).

**Source/test pointers**
- Source: `sleep-api/src/repository.rs` (`get_user_timezone`, `set_user_timezone`), `sleep-api/src/handlers.rs` (`set_user_timezone`), `sleep-api/src/time.rs` (`resolve_local`, `compute_duration_min`)
- Contract: `openapi.yaml` (`/api/settings/timezone`)
- Tests: `sleep-api/tests/settings_timezone.rs` (`test_get_and_set_timezone`), `sleep-api/tests/time_dst.rs` (`fall_back_same_local_times_yield_positive_duration`)

#### D) Partial-write caveat in UI submit flow
- Sleep save is primary; exercise upsert and note create are follow-up best-effort calls.
- If sleep save succeeds but exercise/note call fails, the sleep record remains saved; the combined payload is not atomic across all three domains.

**Source/test pointers**
- Source: `sleep-ui/src/lib/components/SleepForm.svelte` (`submitInner`: `createSleep`/`updateSleep` first, then best-effort `upsertExercise` and `apiPost('/api/note', ...)`)
- Contract: separate endpoints in `openapi.yaml` (`/api/sleep`, `/api/exercise`, `/api/note`)
- Tests: `sleep-api/tests/api_sleep.rs` (`test_sleep_flow`, `test_exercise_and_note`) validate each endpoint path independently (no cross-endpoint atomic transaction test)

### Explicit assumptions/inference (not implementation guarantees)

- **Inference:** Because exercise and note are best-effort follow-ups, analytics that assume “sleep + intensity + note always arrive together” may overcount completeness unless they account for partial writes.
- **Inference:** The app stores one timezone key (`user_timezone`) in `app_settings`; current behavior aligns with single-user/admin operation, not per-user timezone partitioning.

### Known gaps / mismatches to track

- **Intra-code doc mismatch:** `get_sleep_recent` doc comment says “days clamped to [1, 31]”, but implementation rejects out-of-range values with `400`.
- **Constraint inconsistency across domains:** Sleep/exercise range APIs enforce a 62-day cap, while trends APIs currently do not.
- **Cross-domain atomicity gap:** The UI form flow can persist sleep without corresponding exercise/note due to best-effort sequencing.

---

## Deprecated / legacy items

### `POST /api/login.json` (deprecated)
- JSON login endpoint remains wired for compatibility/testing but is marked deprecated in API contract.
- Browser login flow uses `POST /api/login` form endpoint.

**Source evidence**
- `openapi.yaml` (`/api/login.json` has `deprecated: true`)
- `sleep-api/src/app.rs` (route still registered)
- `sleep-ui/src/routes/login/+page.svelte` (uses form login endpoint)

---

## Implemented but not currently surfaced in UI

### `GET /api/sleep/recent`
- Implemented and documented; not used by current in-use routes (dashboard uses `GET /api/sleep/range`).
- Available through frontend API helper (`getRecent`) but not invoked by active route loads.

### `GET /api/trends/summary`
- Implemented and documented aggregate endpoint; current trends page only calls `/api/trends/sleep-bars`.

### `GET|HEAD /api/health`
- Operational health probe endpoint for infrastructure/readiness, not a user-facing UI capability.

**Source evidence**
- `sleep-api/src/app.rs` (routes wired)
- `openapi.yaml` (endpoints listed)
- `sleep-ui/src/routes/+page.server.ts`, `sleep-ui/src/routes/trends/+page.svelte`, `sleep-ui/src/lib/api.ts` (active UI calls)
