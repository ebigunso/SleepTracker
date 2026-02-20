# Playwright Auth Bootstrap Plan

## Goal
Introduce a secure, repeatable Playwright authentication bootstrap path that uses environment secrets, generates authenticated storage state, and provides a single command contract for agents/reviewers when login is required.

## Definition of Done
- A dedicated auth bootstrap step logs in using configured env secrets and writes storage state under `.playwright-cli/auth/storage-state.json`.
- E2E specs stop duplicating inline login flows and consume shared authenticated state.
- One canonical command for authenticated E2E exists and fails fast with actionable errors when secrets or backend prerequisites are missing.
- Secret handling guidance and non-logging rules are documented.
- Reviewer-owned E2E/visual evidence for required flows and viewports is captured under `.playwright-cli/`.

## Scope / Non-goals
**In scope**
- Playwright config/setup changes in `sleep-ui`.
- E2E test auth flow refactor.
- Script contract and docs updates.
- Reviewer evidence design and gate.

**Non-goals**
- Reworking backend auth model.
- Adding CI E2E workflows in this phase.
- Broad test-suite refactors unrelated to authentication bootstrap.

## Assumptions
- Canonical local secret source is `sleep-ui/.env` (already gitignored) unless user chooses `sleep-ui/.env.local`.
- Backend API is available at runtime through current frontend proxy expectations.
- Reviewer evidence remains mandatory for UI validation gate.

## Tasks

### Task_1
- **title:** Define auth bootstrap command contract
- **type:** design
- **owns:**
  - `sleep-ui/package.json`
  - `README.md`
  - `sleep-ui/README.md` (create/update if needed)
- **depends_on:** []
- **acceptance:**
  - Define canonical scripts for auth bootstrap and authenticated E2E execution.
  - Define required env variable names and defaults.
  - Define fail-fast error messages for missing secrets and unreachable API.
- **validation:**
  - required: true | owner: worker | kind: review | detail: commands/docs are consistent and unambiguous

### Task_2
- **title:** Implement Playwright auth bootstrap setup
- **type:** impl
- **owns:**
  - `sleep-ui/playwright.config.ts`
  - `sleep-ui/tests/auth.setup.ts` (new)
  - `.gitignore` (only if `.env.local` convention is selected)
- **depends_on:** [Task_1]
- **acceptance:**
  - Setup authenticates once using env secrets and writes `.playwright-cli/auth/storage-state.json`.
  - Setup fails fast with actionable non-secret error messages when env prerequisites are missing.
  - Setup does not log secrets, cookies, or raw credential values.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run e2e:auth:bootstrap`

### Task_3
- **title:** Refactor E2E specs to consume shared authenticated state
- **type:** impl
- **owns:**
  - `sleep-ui/tests/auth.spec.ts`
  - `sleep-ui/tests/e2e.spec.ts`
- **depends_on:** [Task_2]
- **acceptance:**
  - Remove duplicated inline login helpers from E2E specs.
  - Keep existing scenario intent while relying on bootstrap-authenticated context.
  - Preserve unauthenticated coverage where explicitly needed.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:e2e:auth`

### Task_4
- **title:** Add agent-facing script surface
- **type:** impl
- **owns:**
  - `sleep-ui/package.json`
- **depends_on:** [Task_2]
- **acceptance:**
  - Add scripts for `e2e:auth:bootstrap` and `test:e2e:auth`.
  - Ensure non-zero exit code propagates on bootstrap failure.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:e2e:auth`

### Task_5
- **title:** Document secret-handling and evidence policy
- **type:** docs
- **owns:**
  - `README.md`
  - `sleep-ui/README.md` (create/update if needed)
  - `docs/feature-reference.md`
- **depends_on:** [Task_4]
- **acceptance:**
  - Document where secrets are configured and what must never be logged/committed.
  - Document auth state artifact path and lifecycle.
  - Document required reviewer evidence artifacts and capture scope.
- **validation:**
  - required: true | owner: worker | kind: review | detail: docs reflect commands, env vars, artifacts, and redaction rules

### Task_6
- **title:** Reviewer E2E/visual evidence gate
- **type:** review
- **owns:**
  - `.playwright-cli/**`
- **depends_on:** [Task_3, Task_5]
- **acceptance:**
  - Capture required flows with authenticated bootstrap:
    - Flow A: personalized new-entry defaults/warning fallback
    - Flow B: trends insight prioritization behavior
  - Capture both viewports: `1366x768` and `390x844`.
  - Record screenshots and console/network failure summary.
- **validation:**
  - required: true | owner: reviewer | kind: e2e-visual | detail: evidence bundle present under `.playwright-cli/`

## Task Waves
- **Wave 1:** Task_1
- **Wave 2:** Task_2
- **Wave 3 (parallel):** Task_3, Task_4
- **Wave 4:** Task_5
- **Wave 5:** Task_6

## Reviewer E2E Evidence Contract
- **artifact_root:** `.playwright-cli`
- **auth_state_path:** `.playwright-cli/auth/storage-state.json`
- **required screenshots:**
  - authenticated protected-page checkpoint (desktop/mobile)
  - Flow A checkpoint(s) (desktop/mobile)
  - Flow B checkpoint(s) (desktop/mobile)
- **required logs:**
  - `.playwright-cli/evidence/console.log`
  - `.playwright-cli/evidence/network-failures.log`
- **required metadata:**
  - executed commands
  - base URL used
  - viewport matrix

## Risks / Mitigations
- Secret leakage in logs → enforce redacted errors; forbid credential value prints.
- Stale storage state after credential rotation → bootstrap overwrites state on every run.
- Backend unavailable at auth time → bootstrap preflight check with actionable error.
- Cookie naming differences by environment → rely on browser storage state, not hardcoded cookie names.

## Open Questions
1. Keep secret file convention at `sleep-ui/.env`, or standardize on `sleep-ui/.env.local` and update `.gitignore`?
2. Should `test:e2e:auth` become the only supported agent command for authenticated E2E, with `test:e2e` retained only for local debugging?
