# PR53 Unresolved Comments and Main Conflict Resolution Plan

- status: completed
- owner: orchestrator
- work_type: mixed
- created: 2026-02-22

## Goal
Resolve new unresolved PR #53 review comments (must + should as requested) and cleanly merge `origin/main` into `feat/e2e-db-isolation-guardrails` without regressions.

## Definition of Done
- New unresolved **must** comments are fixed:
  - bypass runtime-state write path creates parent runtime dir.
  - teardown termination is reliable for Unix detached/process-group behavior.
- New unresolved **should** comment is addressed:
  - readiness predicate in setup is tightened from broad `<500` behavior.
- Branch is rebased/merged with `origin/main` and has no unresolved conflicts.
- Required validations pass and PR #53 is updated with response notes.

## Scope
- `sleep-ui/tests/global.setup.ts`
- `sleep-ui/tests/global.teardown.ts`
- `sleep-ui/src/routes/login/+page.svelte` (merge conflict)
- PR #53 thread responses and validation evidence

## Non-goals
- No broad E2E harness redesign.
- No unrelated UI/test refactors.
- No backend behavior changes beyond what is needed for comment/conflict resolution.

## Task Waves
- Wave 1 (sequential triage): `Task_1`
- Wave 2 (parallel implementation): `Task_2`, `Task_3`, `Task_4`
- Wave 3 (sequential verification): `Task_5`

## Tasks

### Task_1
- type: research
- title: Confirm unresolved comments and merge conflict targets
- owns:
  - PR #53 review thread metadata
  - merge-base status with `origin/main`
- depends_on: []
- acceptance:
  - unresolved items are classified into must/should.
  - exact conflicted file list is confirmed.
- validation:
  - required: true
    owner: orchestrator
    kind: review
    detail: Classification accepted before edits.

### Task_2
- type: impl
- title: Fix runtime-state bypass path and readiness strictness
- owns:
  - `sleep-ui/tests/global.setup.ts`
- depends_on: [Task_1]
- acceptance:
  - `ALLOW_NON_ISOLATED_E2E=1` path cannot fail from missing runtime dir.
  - readiness probe checks expected successful/known states (not generic `<500`).
- validation:
  - required: true
    owner: worker
    kind: command
    detail: targeted auth smoke passes and unsafe/bypass paths behave as expected.

### Task_3
- type: impl
- title: Harden teardown for detached process cleanup on Unix
- owns:
  - `sleep-ui/tests/global.teardown.ts`
- depends_on: [Task_1]
- acceptance:
  - teardown targets detached process group on Unix where needed.
  - DB cleanup runs with retry only after termination attempts.
- validation:
  - required: true
    owner: worker
    kind: command
    detail: `npm run test:e2e:auth` finishes without leaked API process symptoms.

### Task_4
- type: impl
- title: Resolve merge conflict with origin/main in login page
- owns:
  - `sleep-ui/src/routes/login/+page.svelte`
- depends_on: [Task_1]
- acceptance:
  - preserve both `method="post" action="/api/login"` and incoming `data-testid` form hook.
  - no conflict markers remain.
- validation:
  - required: true
    owner: worker
    kind: command
    detail: `git diff --name-only --diff-filter=U` is empty after merge.

### Task_5
- type: validation
- title: Run required validations and post PR53 updates
- owns:
  - `sleep-ui/**`
- depends_on: [Task_2, Task_3, Task_4]
- acceptance:
  - `npm run check` and `npm run test:unit` pass.
  - `npm run test:e2e:auth` pass.
  - PR #53 comment with resolution mapping is posted.
- validation:
  - required: true
    owner: worker
    kind: command
    detail: `cd sleep-ui && npm run check && npm run test:unit && npm run test:e2e:auth`

## Progress Log
- 2026-02-22: Drafted from Researcher triage of newest unresolved threads and merge conflict.

## Decision Log
- 2026-02-22: Treat two teardown/setup threads as must-fix and readiness strictness as should-fix per user request to address both categories.
