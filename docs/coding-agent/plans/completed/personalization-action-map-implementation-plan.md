# Personalization Action Map Implementation Plan

## Goal
Implement the action items in `docs/personalization-agent-action-map.md` as production-ready backend + frontend behavior with explicit guardrails, confidence gating, and phased rollout safety.

## Definition of Done
- Personalization metrics and action recommendations are computed from rolling windows and exposed through authenticated API endpoints.
- Friction telemetry is captured (auth+CSRF protected), queryable for ranked backlog proposals, and policy-compliant (evidence + confidence + rollback condition).
- Sleep form applies personalized defaults/warnings only when eligibility guardrails pass; otherwise current behavior remains unchanged.
- Trends page renders schedule-shift/variability/quality-prioritized insights from personalization output with graceful fallback.
- Contract, docs, and tests are updated; required validation and reviewer gate are satisfied.

## Scope / Non-goals
**In scope**
- Backend metrics engine, telemetry ingest/read APIs, OpenAPI updates, frontend integration, tests, rollout flags/docs.

**Non-goals**
- Cohort analytics or multi-user segmentation.
- Heavy instrumentation beyond friction telemetry defined in the action map.
- Automatic action application when confidence is low or guardrails fail.

## Context (workspace)
- Backend: `sleep-api/src/trends.rs`, `sleep-api/src/handlers.rs`, `sleep-api/src/repository.rs`, `sleep-api/src/config.rs`, `sleep-api/src/app.rs`.
- Frontend: `sleep-ui/src/routes/sleep/new/+page.svelte`, `sleep-ui/src/lib/components/SleepForm.svelte`, `sleep-ui/src/routes/trends/+page.svelte`, `sleep-ui/src/lib/api.ts`.
- Contract/docs: `openapi.yaml`, `README.md`, `docs/personalization-agent-action-map.md`, `docs/feature-reference.md`.
- DB migrations: add new file under `migrations/` (`0005_*`), no edits to existing migrations.

## Assumptions
- MVP uses a single consolidated personalization read endpoint plus one telemetry write endpoint.
- Proposal evidence is computed-on-read in MVP (not separately persisted as proposal history).
- Personalized defaults apply to new-entry flow only in phase 1.

## Ownership Declaration
- **Researcher owns:** codebase discovery and plan-fill inputs (completed).
- **Worker owns:** implementation per Task_X `owns` boundaries and worker-owned validation.
- **Reviewer owns:** independent review gate plus UI E2E/visual evidence collection using Playwright.

## Tasks

### Task_1
- **title:** Implement personalization metrics and recommendation engine endpoint
- **type:** impl
- **owns:**
  - `sleep-api/src/trends.rs`
  - `sleep-api/src/app.rs`
  - `sleep-api/src/lib.rs`
- **depends_on:** []
- **acceptance:**
  - Add authenticated read endpoint returning rolling-window metrics for duration baseline, day-type timing baseline, social jetlag, schedule variability, and quality-factor ranking eligibility.
  - Evaluate triggers/guardrails from the action map and include action recommendations with confidence and suppression reasons.
  - Use existing wake-date/daily aggregation semantics (no duplicate date logic).
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api --test trends_bars`
  - required: true | owner: worker | kind: command | detail: `cargo fmt -- --check`
  - required: true | owner: worker | kind: command | detail: `cargo clippy -- -D warnings`

### Task_2
- **title:** Add friction telemetry migration and repository primitives
- **type:** impl
- **owns:**
  - `migrations/0005_personalization_friction.sql` (new)
  - `sleep-api/src/repository.rs`
  - `sleep-api/src/models/` (new/updated telemetry model files)
- **depends_on:** []
- **acceptance:**
  - Create append-only friction telemetry table(s) with columns supporting form time, error kind, retries, immediate edit, and follow-up failure.
  - Add indexes for rolling window query performance.
  - Implement repository insert/query primitives for telemetry and windowed aggregations.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api`
  - required: true | owner: worker | kind: command | detail: verify migrations apply on fresh DB during test run

### Task_3
- **title:** Add telemetry ingest endpoint and friction backlog evidence output
- **type:** impl
- **owns:**
  - `sleep-api/src/handlers.rs`
  - `sleep-api/src/app.rs`
  - `sleep-api/src/repository.rs`
  - `sleep-api/src/error.rs`
- **depends_on:** [Task_2]
- **acceptance:**
  - Add mutating telemetry endpoint secured with existing auth+CSRF pattern.
  - Extend personalization response (or companion endpoint) with friction backlog proposals including: observed evidence, expected benefit, confidence, rollback condition.
  - Enforce auto-promotion rule only for confidence >= medium and persistence across two windows.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api --test auth_csrf`
  - required: true | owner: worker | kind: command | detail: targeted tests for telemetry auth/csrf and aggregation output

### Task_4
- **title:** Update API contract and typed frontend wrappers
- **type:** impl
- **owns:**
  - `openapi.yaml`
  - `sleep-ui/src/lib/api.ts`
- **depends_on:** [Task_1, Task_3]
- **acceptance:**
  - Document new personalization/telemetry endpoint schemas and security requirements.
  - Add typed API wrapper functions used by routes/components.
  - Keep request/response conventions aligned with existing API style.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run check`

### Task_5
- **title:** Implement personalized sleep-form defaults and contextual warning text
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/sleep/new/+page.svelte`
  - `sleep-ui/src/lib/components/SleepForm.svelte`
  - `sleep-ui/src/routes/sleep/new/+page.server.ts` (if needed)
- **depends_on:** [Task_4]
- **acceptance:**
  - Apply day-type (weekday/weekend) baseline defaults only when API marks action eligible.
  - Replace static unusual-duration warning with personalized range text when eligible, with fallback to current static rule.
  - Add one-click “use usual weekday/weekend times” action only when confidence/guardrails pass.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`

### Task_6
- **title:** Add trends insight cards and explanation prioritization
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
  - `sleep-ui/src/lib/stores/` (new helper store if needed)
  - `sleep-ui/src/lib/utils/` (new helper utilities if needed)
- **depends_on:** [Task_4]
- **acceptance:**
  - Show schedule-shift and variability insight cards from personalization recommendations.
  - Prioritize regularity/quality-linked explanation text when triggers pass.
  - Preserve existing trends behavior when personalization is unavailable/suppressed.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`
  - required: true | owner: reviewer | kind: e2e-visual | detail: run E2E spec and collect screenshot evidence

### Task_7
- **title:** Add/extend backend and frontend tests for personalization behavior
- **type:** test
- **owns:**
  - `sleep-api/tests/` (new personalization and telemetry tests)
  - `sleep-ui/tests/unit/`
  - `sleep-ui/tests/e2e.spec.ts`
- **depends_on:** [Task_5, Task_6]
- **acceptance:**
  - Backend tests cover threshold triggers, guardrail suppressions, and confidence transitions.
  - Backend tests verify telemetry auth+csrf enforcement and evidence payload shape.
  - Frontend tests cover default selection and warning-mode switching.
  - E2E includes at least one personalized form case and one trends prioritization case.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api`
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`
  - required: true | owner: reviewer | kind: command | detail: `cd sleep-ui && npm run test:e2e`

### Task_8
- **title:** Add rollout flags and documentation updates
- **type:** docs
- **owns:**
  - `sleep-api/src/config.rs`
  - `README.md`
  - `docs/personalization-agent-action-map.md`
  - `docs/feature-reference.md`
- **depends_on:** [Task_7]
- **acceptance:**
  - Add feature flags for personalization read, UI action application, and telemetry ingest.
  - Document phased rollout and rollback conditions aligned with action-map proposal policy.
  - Confirm default behavior is unchanged when all flags are off.
- **validation:**
  - required: true | owner: worker | kind: review | detail: documentation consistency pass across endpoint names/flags/guardrails

## Task Waves
- **Wave 1 (parallel):** Task_1, Task_2
- **Wave 2 (sequential):** Task_3, Task_4
- **Wave 3 (parallel):** Task_5, Task_6
- **Wave 4 (sequential):** Task_7, Task_8

## E2E / Visual Validation Spec
- **artifact_root:** `.playwright-cli`
- **base_url:** `http://127.0.0.1:4173`
- **app_start_command:** `cd sleep-ui && npm run build && npm run preview -- --host 127.0.0.1 --port 4173`
- **readiness_check:** GET `/login` returns page containing login form heading.
- **flows:**
  - Flow A: Personalized new-entry defaults
    - Log in.
    - Open new sleep entry page.
    - Verify personalized defaults appear only when recommendation eligibility is true.
    - Trigger fallback scenario and verify static defaults/warning behavior.
  - Flow B: Trends insight prioritization
    - Open trends page.
    - Verify schedule-shift/variability card visibility under eligible dataset.
    - Verify explanation priority changes with recommendation ordering.
- **viewports:**
  - `1366x768`
  - `390x844`
- **evidence requirements:**
  - Save screenshots for each flow + viewport under `.playwright-cli/`.
  - Record console errors and failed network requests; any blocking issue fails review.
- **known_flakiness / assumptions:**
  - Test data setup must guarantee eligibility and suppression scenarios; otherwise assertions can be non-deterministic.

## Rollback / Safety
- Keep all personalization application behavior gated by backend eligibility + feature flags.
- Roll back by toggling flags off (no schema rollback needed).
- Telemetry write failures must fail-open for user save flow (do not block sleep entry submission).

## Notes
- High-risk ambiguity deferred via assumptions: consolidated endpoint shape, computed-on-read proposal evidence, new-entry-only defaulting.
- If assumptions change, update this plan before execution dispatch.
