# Plan: Sleep Edit Dark-Mode Parity and DRY Assessment

- status: draft
- generated: 2026-02-22
- last_updated: 2026-02-22
- work_type: mixed

## Goal
- Align the sleep-log edit screen visuals with the create screen dark-mode token usage, and remove unnecessary duplication only if it can be done with low regression risk.

## Definition of Done
- Edit screen header/subtext/container/delete action render correctly in dark mode using existing theme tokens/components.
- Create and edit behaviors remain unchanged (create, update, delete, cancel, personalization/intensity handling).
- Existing E2E selectors remain stable.
- Reviewer provides dark-mode parity evidence for create and edit screens at desktop and mobile viewports.

## Scope / Non-goals
- Scope:
  - `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte`
  - `sleep-ui/src/routes/sleep/new/+page.svelte` (only if needed for unification)
  - optional tiny presentational component under `sleep-ui/src/lib/components/` if unification is clearly safe
- Non-goals:
  - No `SleepForm` business logic redesign
  - No backend/API contract changes
  - No new theming system; only use existing tokens/components

## Context (workspace)
- Related files/areas:
  - `sleep-ui/src/routes/sleep/new/+page.svelte`
  - `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte`
  - `sleep-ui/src/lib/components/SleepForm.svelte`
  - `sleep-ui/src/lib/components/Button.svelte`
  - `sleep-ui/src/app.css`
- Existing patterns or references:
  - create page uses `page-title`, `text-muted`, `surface-card`
  - edit page currently uses hardcoded slate/white/rose Tailwind palette classes
- Repo reference docs consulted:
  - `docs/coding-agent/references/validation.md`
  - `docs/coding-agent/references/ui-e2e.md`

## Open Questions (max 3)
- Q1: For this task, should delete action shape be preserved as current pill style, as long as tokenized dark-mode-safe styling is used?

## Assumptions
- A1: Minimal-risk implementation is preferred over broad structural refactor.
- A2: DRY unification is acceptable only if it remains presentation-only and keeps route-specific data/handlers local.

## Tasks

### Task_1: Implement edit-page dark-mode parity visuals
- type: impl
- owns:
  - `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte`
- depends_on: []
- description: |
  Replace hardcoded light-mode classes with existing tokenized classes/components used by the create screen and app design system.
  Keep current behavior and test ids unchanged.
- acceptance:
  - Edit title/subtitle/container match tokenized create-page visual language in light and dark themes.
  - Delete action uses design-system-safe styling (tokenized/button variant) without behavior changes.
  - No regressions to update/delete/cancel interactions.
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

### Task_2: DRY evaluation and safe unification decision
- type: design
- owns:
  - `sleep-ui/src/routes/sleep/new/+page.svelte`
  - `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte`
  - `sleep-ui/src/lib/components/*`
- depends_on: [Task_1]
- description: |
  Evaluate whether create/edit wrapper UI should remain separate or be unified.
  If safe and beneficial, implement a minimal presentational unification; otherwise document why separation is retained.
- acceptance:
  - Decision is explicit with concise rationale and risk assessment.
  - If unified, abstraction is presentation-only and does not move route-specific loader/action logic.
  - If not unified, duplication is minimized and rationale recorded in worker report.
- validation:
  - kind: review
    required: true
    owner: reviewer
    detail: "Confirm DRY decision is low-risk and preserves behavior boundaries"

### Task_3: Reviewer UI/E2E dark-mode parity evidence
- type: review
- owns: []
- depends_on: [Task_1, Task_2]
- description: |
  Validate create/edit parity using browser evidence in dark mode and ensure no user-flow regressions.
- acceptance:
  - Reviewer status is APPROVED.
  - Evidence captured under `.playwright-cli/`.
  - Create and edit screens show consistent tokenized visuals in dark mode at required viewports.
- validation:
  - kind: command
    required: true
    owner: reviewer
    detail: "cd sleep-ui && npm run test:e2e"
  - kind: e2e
    required: true
    owner: reviewer
    detail: "Capture screenshots + console/network scan per ui-e2e checklist"

## Task Waves (explicit parallel dispatch sets)

- Wave 1 (parallel): [Task_1]
- Wave 2 (parallel): [Task_2]
- Wave 3 (parallel): [Task_3]

## E2E / Visual Validation Spec (required; UI impacted)

- artifact_root: ".playwright-cli"
- base_url: "http://127.0.0.1:4173"
- app_start_command: "cd sleep-ui && npm run test:e2e"
- readiness_check: "Playwright global setup health check and app URL readiness"
- flows:
  - "Create sleep log screen in dark mode: verify title/subtitle/card surface"
  - "Edit sleep log screen in dark mode: verify title/subtitle/card surface/delete action visual parity"
  - "Basic create->edit flow remains functional"
- viewports:
  - "desktop (1280x720)"
  - "mobile (390x844)"
- evidence_requirements:
  - "Screenshots for create/edit dark-mode key states under .playwright-cli/"
  - "Console error scan"
  - "Network failed request scan"
- known_flakiness:
  - "None expected; use existing isolated harness defaults"

## Rollback / Safety
- Revert route-level UI markup changes in `sleep-ui/src/routes/sleep/[id]/edit/+page.svelte` (and any optional presentational component) without touching API/form logic.

## Progress Log (append-only)

- 2026-02-22 00:00 Wave 0 planning: drafted plan from Researcher findings.
  - Summary: Root cause identified (hardcoded edit classes bypass theme tokens); implementation intentionally scoped to minimal-risk parity + DRY assessment.
  - Validation evidence: N/A (planning phase)
  - Notes: Awaiting user approval.

## Decision Log (append-only; re-plans and major discoveries)

- 2026-02-22 00:00 Decision:
  - Trigger / new insight: Research confirmed `SleepForm` already unifies core form UI; duplication is mostly route shell markup and delete action styling.
  - Plan delta (what changed): Created focused 3-task plan emphasizing parity first, DRY decision second, reviewer evidence gate third.
  - Tradeoffs considered: Full shell extraction now vs minimal parity fix first.
  - User approval: no (pending)

## Notes
- Risks:
  - Over-unifying route shells could couple distinct route concerns and create test-id drift.
- Edge cases:
  - Preserve delete interaction prominence and accessibility while tokenizing style.
