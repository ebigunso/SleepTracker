# Sleep UI E2E DB Isolation Guardrails Plan

- status: in_progress
- owner: orchestrator
- work_type: mixed
- created: 2026-02-21

## Goal
Guarantee Playwright E2E runs only against an isolated disposable database and cannot accidentally write to live/shared data.

## Definition of Done
- E2E harness provisions and uses a disposable DB path unique to each run.
- E2E fails fast when API target is non-local or isolation prerequisites are invalid.
- Home/day flows continue to pass under isolated harness.
- Docs clearly describe safe E2E execution and override risk.
- Validation evidence includes positive isolation proof and negative safety checks.

## Scope
- E2E harness orchestration and guardrails in `sleep-ui`.
- Minimal backend runtime support for configurable API bind where needed.
- Safety documentation updates for local workflow.

## Non-goals
- No application feature changes.
- No auth model redesign.
- No production deployment changes.

## Task Waves
- Wave 1 (sequential foundation): `Task_1`
- Wave 2 (parallel implementation): `Task_2`, `Task_3`
- Wave 3 (sequential stabilization): `Task_4`
- Wave 4 (sequential validation/review): `Task_5`

## Tasks

### Task_1
- type: design
- title: Define isolation contract and unsafe-target policy
- owns:
  - `sleep-ui/playwright.config.ts`
  - `sleep-ui/tests/**`
  - `sleep-ui/package.json`
- depends_on: []
- acceptance:
  - Isolation root path and lifecycle policy are defined (`.playwright-cli/e2e-db/**`).
  - Unsafe API target policy is explicit (block non-local by default).
  - Escape hatch policy is explicit and loud (`ALLOW_NON_ISOLATED_E2E=1`).
- validation:
  - required: true
    owner: orchestrator
    kind: review
    detail: Contract reviewed before code changes.

### Task_2
- type: impl
- title: Add harness-managed isolated API startup/teardown
- owns:
  - `sleep-ui/playwright.config.ts`
  - `sleep-ui/tests/global.setup.ts`
  - `sleep-ui/tests/global.teardown.ts`
  - `sleep-ui/package.json`
  - `sleep-api/src/main.rs`
- depends_on: [Task_1]
- acceptance:
  - Setup spawns API with run-unique disposable DB path.
  - Setup forces local API target for UI proxy during E2E.
  - Teardown stops API and cleans disposable DB artifacts (unless retain flag set).
- validation:
  - required: true
    owner: worker
    kind: command
    detail: Targeted E2E smoke passes using isolated harness.

### Task_3
- type: impl
- title: Add fail-fast guardrails for unsafe targets and DB paths
- owns:
  - `sleep-ui/playwright.config.ts`
  - `sleep-ui/tests/auth.setup.ts`
  - `sleep-ui/tests/global.setup.ts`
- depends_on: [Task_1]
- acceptance:
  - Non-local API/proxy targets hard fail by default before any mutation.
  - DB path outside allowed isolation root hard fails.
  - Override mode requires explicit env and prints warning.
- validation:
  - required: true
    owner: worker
    kind: negative-check
    detail: Unsafe target/path scenarios fail before login/mutations.

### Task_4
- type: docs
- title: Document safe E2E workflow and risk model
- owns:
  - `README.md`
  - `sleep-ui/README.md`
  - `docs/coding-agent/references/how-to-run.md`
  - `docs/coding-agent/references/validation.md`
- depends_on: [Task_2, Task_3]
- acceptance:
  - Commands explain isolated E2E defaults.
  - Guard behavior and override risk are documented.
  - Validation mapping includes E2E isolation checks.
- validation:
  - required: true
    owner: worker
    kind: docs-review
    detail: Commands/paths are consistent and runnable.

### Task_5
- type: review
- title: Execute validations and reviewer safety gate
- owns:
  - `sleep-ui/**`
  - `sleep-api/src/main.rs`
  - `README.md`
  - `docs/coding-agent/references/**`
- depends_on: [Task_4]
- acceptance:
  - `npm run check` and `npm run test:unit` pass.
  - Targeted E2E smoke passes on isolated harness.
  - Negative guard checks pass.
  - Reviewer status is APPROVED (or APPROVED_WITH_RISK with explicit mitigation).
- validation:
  - required: true
    owner: worker
    kind: command
    detail: `npm run check`, `npm run test:unit`, targeted e2e command.
  - required: true
    owner: reviewer
    kind: review
    detail: Confirm no mutation path to live/shared DB in default E2E flow.

## Progress Log
- 2026-02-21: Drafted in response to live-data mutation incident risk.
- 2026-02-21: Implemented harness-managed isolated API startup/teardown with disposable DB path and local-target guardrails.
- 2026-02-21: Validation pass: `npm run check`, `npm run test:unit`, `cargo test -p sleep-api`, negative unsafe-target guard check.
- 2026-02-21: Validation pending: authenticated Playwright smoke currently times out on `/api/login` response in this environment and requires follow-up.

## Decision Log
- 2026-02-21: Chose harness-managed disposable DB + hard fail-fast guardrails over guardrails-only approach due to safety criticality.
