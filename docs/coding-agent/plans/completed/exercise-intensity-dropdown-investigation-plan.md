# Exercise Intensity Dropdown Investigation Plan

- status: completed
- owner: Orchestrator
- created: 2026-02-25
- updated: 2026-02-25

## Objective
Identify why selecting `light` or `hard` exercise intensity fails in create/edit entry flows, then provide a robust fix proposal that addresses code-level, runtime, and data-contract causes.

## Scope
In scope:
- `sleep-ui` create/edit flow for exercise intensity select controls
- related API/domain serialization/deserialization for intensity values
- runtime/environment conditions that could make only specific options non-selectable

Out of scope:
- implementing behavior changes before plan approval
- unrelated UI redesign

## Task Waves
- Wave 1 (sequential): Task_1
- Wave 2 (sequential): Task_2
- Wave 3 (sequential): Task_3
- Wave 4 (sequential): Task_4

## Tasks

### Task_1
- type: research
- owns:
  - sleep-ui/src/**
  - sleep-api/src/**
  - sleep-ui/tests/**
  - docs/coding-agent/plans/active/exercise-intensity-dropdown-investigation-plan.md
- depends_on: []
- acceptance:
  - Reproduce or logically confirm failure path for selecting `light` and `hard`.
  - Enumerate plausible root causes across UI state handling, option binding, enum/string mapping, and API contract handling.
  - Verify whether create and edit flows share code or diverge in a way that explains selective option failure.
  - Capture evidence (code paths and, if needed, local runtime observations).
- validation:
  - required: true
    owner: orchestrator
    kind: review
    detail: Ensure investigation includes both code-path and runtime-condition analysis (not only static grep).

### Task_2
- type: review
- owns:
  - docs/coding-agent/plans/active/exercise-intensity-dropdown-investigation-plan.md
- depends_on:
  - Task_1
- acceptance:
  - Provide a prioritized fix proposal with root-cause confidence level.
  - Include at least one immediate fix and one hardening/guardrail recommendation (e.g., test coverage or schema validation).
  - State validation steps required to verify the fix after implementation.
- validation:
  - required: true
    owner: orchestrator
    kind: review
    detail: Proposal addresses create + edit flows and includes post-fix verification commands aligned with validation mapping.

### Task_3
- type: impl
- owns:
  - sleep-ui/src/lib/components/Input.svelte
  - sleep-ui/src/lib/components/SleepForm.svelte
  - sleep-ui/src/lib/utils/**
- depends_on:
  - Task_2
- acceptance:
  - Remove intensity selection dependence on wrapper-level `change` event delivery.
  - Ensure create/edit flows preserve user-selected intensity and still accept late `initialIntensity` updates when untouched.
  - Keep behavior scoped to intensity handling with minimal unrelated diff.
- validation:
  - required: true
    owner: worker
    kind: command
    detail: cd sleep-ui && npm run check

### Task_4
- type: test
- owns:
  - sleep-ui/tests/unit/**
- depends_on:
  - Task_3
- acceptance:
  - Add regression coverage for intensity sync/dirty-state behavior using non-E2E tests.
  - Validate wrapper event forwarding behavior where practical without introducing heavy new test infrastructure.
  - Keep E2E as optional/targeted only if a gap cannot be covered otherwise.
- validation:
  - required: true
    owner: worker
    kind: command
    detail: cd sleep-ui && npm run test:unit

## Validation Plan
- No code edits are made before user approval.
- Investigation evidence will include file-level references and (if needed) local reproduction checks.
- If UI behavior is runtime-sensitive, include explicit environment factors and mitigation.
- Implementation validations prioritize non-E2E coverage (`npm run check`, `npm run test:unit`).
- E2E is optional and only used for residual risk not effectively covered by unit-level tests.

## Progress Log
- 2026-02-25: Drafted plan for investigation/proposal workflow. Awaiting user approval.
- 2026-02-25: User approved proceeding directly to robust implementation; added implementation and unit-regression waves.
- 2026-02-25: Implemented robust fix in `Input` + `SleepForm` and added unit regression coverage for intensity synchronization state transitions.
- 2026-02-25: Validation executed: `cd sleep-ui && npm run check` (pass), `npm run test:unit` (pass after cwd-safe rerun).

## Decision Log
- 2026-02-25: Treated request as non-trivial due to non-obvious bug spanning UI + possible backend/runtime mapping concerns.
- 2026-02-25: Applied non-E2E-first validation strategy; deferred E2E because regression was fully covered by targeted unit tests and type checks.
