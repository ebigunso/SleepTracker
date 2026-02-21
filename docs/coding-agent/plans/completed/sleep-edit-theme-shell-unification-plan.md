# Plan: Align Sleep Edit UI Theme and DRY Wrapper Structure

- status: done
- generated: 2026-02-22
- last_updated: 2026-02-22
- work_type: code

## Goal
- Make the sleep-log edit page match the dark-mode-ready visuals used by the create page, and remove avoidable wrapper duplication while preserving existing behavior and selectors.

## Definition of Done
- Edit page wrapper uses semantic theme classes/tokens (no hard-coded light-only wrapper chrome).
- Create/edit wrapper chrome is unified where safe (presentational reuse only).
- Existing create/edit behavior remains intact (save/cancel/delete/prefill flow).
- Existing E2E selectors remain stable unless explicitly justified.
- Required validation and reviewer UI evidence are recorded.

## Scope / Non-goals
- Scope:
  - `sleep-ui/src/routes/sleep/new/+page.svelte`
  - `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte`
  - `sleep-ui/src/lib/components/**` (only if needed for shared presentational shell)
  - `sleep-ui/tests/**` only if selector or UI validation updates are required
- Non-goals:
  - API/backend/model changes
  - Broad redesign of sleep flows
  - Rewriting `SleepForm` business logic

## Context (workspace)
- Related files/areas:
  - `sleep-ui/src/routes/sleep/new/+page.svelte`
  - `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte`
  - `sleep-ui/src/lib/components/SleepForm.svelte`
  - `sleep-ui/src/app.css`
- Existing patterns or references:
  - Semantic theme classes (`page-title`, `text-muted`, `surface-card`, `btn-danger`) should be preferred over hard-coded color utilities.
  - `SleepForm.svelte` already centralizes form logic for create/edit modes.
- Repo reference docs consulted:
  - `docs/coding-agent/rules/common.md`
  - `docs/coding-agent/rules/orchestrator.md`
  - `docs/coding-agent/references/validation.md`
  - `docs/coding-agent/references/ui-e2e.md`

## Open Questions (max 3)
- Q1: None; proceed with minimal presentational unification and preserve existing test ids.

## Assumptions
- A1: Dark-mode mismatch is limited to route-level wrapper chrome on edit page.
- A2: A shared presentational shell can be introduced without altering route load/action contracts.

## Tasks

### Task_1: Implement theme parity and DRY wrapper reuse for create/edit pages
- type: impl
- owns:
  - `sleep-ui/src/routes/sleep/new/+page.svelte`
  - `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte`
  - `sleep-ui/src/lib/components/**`
- depends_on: []
- description: |
  Replace edit wrapper light-only classes with semantic theme classes matching create behavior.
  If duplication remains, extract shared presentational shell component/structure while keeping route-specific logic in route files.
  Preserve existing ids used by tests.
- acceptance:
  - Edit wrapper uses semantic theme classes equivalent to create shell visuals.
  - Shared wrapper structure is centralized when safe (presentational only).
  - No behavior regression in create/edit submission, cancel, and delete flows.
  - Existing test ids are preserved or intentionally updated with corresponding test adjustments.
- validation:
  - kind: command
    required: true
    owner: worker
    detail: "cd sleep-ui && npm run check"
  - kind: command
    required: true
    owner: worker
    detail: "cd sleep-ui && npm run test:unit"
  - kind: command
    required: true
    owner: worker
    detail: "cd sleep-ui && npm run build"

### Task_2: Reviewer UI/E2E evidence gate for create/edit dark-mode parity
- type: review
- owns: []
- depends_on: [Task_1]
- description: |
  Run browser-based checks for create/edit wrappers in dark mode and key create->edit flow stability.
  Collect evidence artifacts under `.playwright-cli/`.
- acceptance:
  - Reviewer confirms create and edit wrappers render with consistent dark-mode visuals.
  - Reviewer confirms critical flow is intact (create/edit/delete path smoke coverage).
  - Reviewer status is APPROVED or explicit issues are returned.
- validation:
  - kind: e2e
    required: true
    owner: reviewer
    detail: "Run the E2E spec in this plan using playwright-cli and capture evidence under .playwright-cli/"

## Task Waves (explicit parallel dispatch sets)

- Wave 1 (parallel): [Task_1]
- Wave 2 (parallel): [Task_2]

## E2E / Visual Validation Spec (required: UI impacted)

- artifact_root: ".playwright-cli"
- base_url: "http://127.0.0.1:5173"
- app_start_command: "cd sleep-ui && npm run test:e2e"
- readiness_check: "Playwright global setup completes and app is reachable on strict local host"
- flows:
  - "Open create sleep page and verify heading/card/subtext theme visuals in dark mode"
  - "Create/edit entry path smoke check and verify edit heading/card/delete chrome in dark mode"
- viewports:
  - "desktop default from playwright config"
- evidence_requirements:
  - "At least one screenshot for create page dark mode wrapper"
  - "At least one screenshot for edit page dark mode wrapper"
  - "Reviewer note for flow pass/fail and console/network anomalies"
- known_flakiness:
  - "Use existing E2E harness isolation and localhost guardrails"

## Rollback / Safety
- Revert changes in route wrappers and shared shell component; keep `SleepForm` API unchanged.
- If shared shell introduces risk, fall back to direct class parity update in edit page only.

## Progress Log (append-only)

- 2026-02-22 00:00 Draft created.
  - Summary: Planned dark-mode parity fix + DRY wrapper unification + reviewer UI evidence gate.
  - Validation evidence: N/A (planning stage)
  - Notes: Awaiting user approval before execution.

- 2026-02-22 00:20 Wave 1 completed: [Task_1]
  - Summary: Added shared `SleepEntryShell` presentational component and switched create/edit wrappers to semantic theme-tokenized shell.
  - Validation evidence:
    - `cd sleep-ui && npm run check` (pass)
    - `cd sleep-ui && npm run test:unit` (pass after cwd normalization retry)
    - `cd sleep-ui && npm run build` (pass)
  - Notes: Existing key test ids were preserved.

- 2026-02-22 00:30 Wave 2 attempted: [Task_2]
  - Summary: Reviewer returned NEEDS_REVISION; required dark-mode screenshots under `.playwright-cli/` were missing and flow evidence was inconclusive due E2E failures.
  - Validation evidence: Reviewer report status `NEEDS_REVISION`.
  - Notes: Execution paused for mandatory deviation improvement loop before re-running review gate.

- 2026-02-22 00:42 Wave 2 completed: [Task_2]
  - Summary: Reviewer re-gate APPROVED with required dark-mode create/edit wrapper screenshots and create→edit→delete smoke-flow confirmation.
  - Validation evidence:
    - `sleep-ui/.playwright-cli/task2-create-dark-wrapper.png`
    - `sleep-ui/.playwright-cli/task2-edit-dark-wrapper.png`
    - Reviewer flow snapshots and summary evidence under `sleep-ui/.playwright-cli/`.
  - Notes: No additional implementation changes required after review re-gate.

## Decision Log (append-only; re-plans and major discoveries)

- 2026-02-22 00:00 Decision:
  - Trigger / new insight: Research showed form logic is already shared; wrapper chrome drift is in edit route.
  - Plan delta (what changed): Prioritized presentational wrapper unification over form-level refactor.
  - Tradeoffs considered: Monolithic create/edit page merge rejected to avoid coupling route-specific side effects.
  - User approval: yes

- 2026-02-22 00:32 Decision:
  - Trigger / new insight: Reviewer gate lacked required screenshot artifacts and did not conclusively establish smoke-flow stability.
  - Plan delta (what changed): Re-run Task_2 with explicit artifact capture requirements and focused create/edit/delete smoke verification.
  - Tradeoffs considered: Avoid code changes until reviewer evidence confirms whether observed failures are regressions versus harness flakiness.
  - User approval: not required (no scope expansion)

## Notes
- Risks:
  - Selector drift could break E2E if ids change.
  - Dark-mode contrast on destructive action needs verification.
- Edge cases:
  - Edit route async prefill behavior must remain unchanged.
