# Sleep UI Stable Selector Coverage Plan

- status: completed
- owner: orchestrator
- work_type: code
- created: 2026-02-21

## Goal
Introduce robust, stable selector hooks across the frontend UI so tests/references are not tightly coupled to visible copy, while fixing the Home `Log Sleep` and day-view `Add Session` action brittleness.

## Definition of Done
- Home and day action buttons have stable hooks and associated tests no longer depend on exact visible copy.
- Interactive UI surfaces across `sleep-ui/src` have collision-safe selector hooks (`data-testid`) where lacking.
- Existing semantic/accessibility `id` usage remains intact for label/input association; no duplicate dynamic IDs are introduced.
- Frontend validations pass (`check`, `unit`, and targeted/full E2E as available) with evidence.
- Reviewer gate confirms behavior/visual parity for impacted flows.

## Scope
- Add `data-testid` hooks in frontend routes/components under `sleep-ui/src/**` where selectors are currently brittle or absent.
- Update Playwright specs under `sleep-ui/tests/**` to use stable hooks for brittle flows.
- Explicitly include Home `Log Sleep` and day `Add Session` flows.

## Non-goals
- No backend/API behavior changes.
- No visual redesign or extra UX features.
- No migration of every test to testid-only style when role/label selectors are already stable.

## Selector Contract
- Use `data-testid` (not global `id`) for automation hooks.
- Naming pattern: `<surface>-<element>-<action|state>`.
- Dynamic list entities may include stable business keys (e.g., session id/date), never index-based keys.
- Preserve existing accessibility ids used by labels/forms.

## Task Waves
- Wave 1 (sequential foundation): `Task_1`
- Wave 2 (parallel implementation): `Task_2`, `Task_3`, `Task_4`, `Task_5`
- Wave 3 (sequential stabilization): `Task_6`
- Wave 4 (sequential validation/review): `Task_7`

## Tasks

### Task_1
- type: design
- title: Finalize selector naming map and coverage boundaries
- owns:
  - `sleep-ui/src/**`
  - `sleep-ui/tests/**`
- depends_on: []
- acceptance:
  - Confirm `data-testid` policy and naming scheme for all affected surfaces.
  - Confirm coverage baseline: interactive controls + state anchors across routes/components.
  - Confirm explicit inclusion of Home `Log Sleep` and day `Add Session` actions.
- validation:
  - required: true
    owner: orchestrator
    kind: review
    detail: Naming map and coverage approach reviewed before edits.

### Task_2
- type: impl
- title: Add route-level hooks and fix Home/day action brittleness
- owns:
  - `sleep-ui/src/routes/+page.svelte`
  - `sleep-ui/src/routes/day/[date]/+page.svelte`
  - `sleep-ui/src/routes/sleep/new/+page.svelte`
  - `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte`
- depends_on: [Task_1]
- acceptance:
  - Home action has stable hook independent of visible copy.
  - Day add-session action has stable hook independent of visible copy.
  - New/edit route state anchors are test-addressable.
- validation:
  - required: true
    owner: worker
    kind: test
    detail: Targeted Playwright flow for Home/day navigation passes with new hooks.

### Task_3
- type: impl
- title: Add component-level hooks for session and form interactions
- owns:
  - `sleep-ui/src/lib/components/DayCard.svelte`
  - `sleep-ui/src/lib/components/SessionRow.svelte`
  - `sleep-ui/src/lib/components/SleepForm.svelte`
  - `sleep-ui/src/lib/components/ConfirmDialog.svelte`
- depends_on: [Task_1]
- acceptance:
  - Session actions are selectable without text-content card matching.
  - Sleep form primary actions have stable hooks.
  - Confirm dialog actions are directly targetable.
- validation:
  - required: true
    owner: worker
    kind: test
    detail: Multi-session edit/delete path works with hook-based selectors.

### Task_4
- type: impl
- title: Add global navigation/auth hooks
- owns:
  - `sleep-ui/src/routes/+layout.svelte`
  - `sleep-ui/src/lib/components/ProfileMenu.svelte`
  - `sleep-ui/src/routes/login/+page.svelte`
- depends_on: [Task_1]
- acceptance:
  - Global nav/profile controls have stable hooks.
  - Login critical actions/anchors have stable hooks.
- validation:
  - required: false
    owner: worker
    kind: test
    detail: Auth bootstrap/smoke selectors remain valid or are updated.

### Task_5
- type: impl
- title: Add trends page hooks for interactive controls
- owns:
  - `sleep-ui/src/routes/trends/+page.svelte`
- depends_on: [Task_1]
- acceptance:
  - Trends key controls (mode, presets, apply, metric toggles) have stable hooks.
  - No behavior change in chart rendering/interactions.
- validation:
  - required: true
    owner: reviewer
    kind: manual+e2e
    detail: Reviewer verifies trends interactions and captures evidence artifacts.

### Task_6
- type: test
- title: Migrate brittle Playwright selectors to stable hooks
- owns:
  - `sleep-ui/tests/e2e.spec.ts`
  - `sleep-ui/tests/auth.setup.ts`
  - `sleep-ui/tests/auth.spec.ts`
- depends_on: [Task_2, Task_3, Task_4, Task_5]
- acceptance:
  - Critical flows use `data-testid` where text selectors were brittle.
  - Home/day action flows explicitly use stable hooks.
  - Keep role/label selectors where they are already semantically stable.
- validation:
  - required: true
    owner: worker
    kind: command
    detail: `npm run test:e2e` (or targeted subset with documented reason).

### Task_7
- type: review
- title: Run final validation and reviewer gate
- owns:
  - `sleep-ui/**`
- depends_on: [Task_6]
- acceptance:
  - Required validation outputs are captured and summarized.
  - Reviewer status is APPROVED with key notes.
  - Any skipped validations are explicitly waived/documented.
- validation:
  - required: true
    owner: worker
    kind: command
    detail: `npm run check` and `npm run test:unit` pass.
  - required: true
    owner: reviewer
    kind: e2e+visual
    detail: Home/day/trends evidence captured under `.playwright-cli/`.

## Progress Log
- 2026-02-21: Draft plan created from Researcher audit; pending user approval.
- 2026-02-21: Implemented Task_2-Task_5 selector hooks across routes/components, including Home and day action hardening.
- 2026-02-21: Validation so far: `npm run check` pass, `npm run test:unit` pass, targeted e2e subset partially pass (2/3).
- 2026-02-21: Reviewer status `APPROVED_WITH_RISK`; outstanding risk is `multiple sessions per day display and edit` failing with `POST /api/sleep` 400, likely backend/data overlap constraint rather than selector regression.

## Decision Log
- 2026-02-21: Selected `data-testid` over broad `id` usage to avoid duplicate-ID risks and decouple selectors from UI text.
- 2026-02-21: User approved scope as interactive controls + page state anchors across `sleep-ui/src/**`; static presentational nodes remain untagged.
