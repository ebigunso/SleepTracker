# Hidden Needs Capture Plan

## Goal
Introduce an explainable, privacy-safe “possible hidden needs” capability that infers unmet needs from app behavior and user prompts, then presents actionable hypotheses in Trends.

## Definition of Done
- User can opt in/out of hidden-needs inference.
- System generates ranked need hypotheses from behavior signals and chat-intent signals.
- Each hypothesis includes transparent evidence, confidence, and one small next action.
- UI supports quick user feedback (“useful/not useful”) to improve future relevance.
- No medical diagnosis language; recommendations remain behavioral and non-clinical.
- Required backend/frontend validations pass; reviewer approves usability and clarity.

## Scope / Non-goals
**In scope**
- Event/signal capture limited to sleep app interactions and first-party assistant intents.
- Lightweight scoring/ranking for need hypotheses.
- Trends UI panel for “Possible needs this week” with explainability and feedback.

**Non-goals**
- Cross-product tracking.
- Black-box ML model deployment in first phase.
- Clinical or diagnostic claims.
- New standalone section/page outside current trends flow.

## Context (workspace)
- Trends and recommendation UX: `sleep-ui/src/routes/trends/+page.svelte`
- Frontend API client/types: `sleep-ui/src/lib/api.ts`
- Personalization/trends backend: `sleep-api/src/trends.rs`, `sleep-api/src/repository.rs`
- Friction/personalization models: `sleep-api/src/models/friction.rs`, `migrations/0005_personalization_friction.sql`
- Docs and action maps: `docs/personalization-agent-action-map.md`, `docs/personalization-metrics-shortlist.md`

## Tasks

### Task_1
- **title:** Define hidden-needs taxonomy and inference guardrails
- **type:** design
- **owns:**
  - `docs/personalization-agent-action-map.md`
  - `docs/personalization-metrics-shortlist.md`
- **depends_on:** []
- **acceptance:**
  - Define allowed need categories (e.g., consistency support, recovery support, wind-down support).
  - Define prohibited claim categories and language safety rules.
  - Define evidence schema (signals used, confidence rubric, action template).
- **validation:**
  - required: true | owner: worker | kind: review | detail: taxonomy aligns with existing personalization safety expectations

### Task_2
- **title:** Specify opt-in and data minimization contract
- **type:** design
- **owns:**
  - `openapi.yaml`
  - `docs/api_examples.md`
- **depends_on:** [Task_1]
- **acceptance:**
  - Define explicit opt-in/out controls and API-level flags.
  - Define retained signal fields and retention window.
  - Define endpoint contracts for needs hypotheses and feedback capture.
- **validation:**
  - required: true | owner: worker | kind: review | detail: contract is minimal and auditable

### Task_3
- **title:** Implement backend hidden-needs hypothesis endpoint
- **type:** impl
- **owns:**
  - `sleep-api/src/trends.rs`
  - `sleep-api/src/repository.rs`
  - `sleep-api/src/models/friction.rs`
  - `sleep-api/src/handlers.rs`
  - `openapi.yaml`
- **depends_on:** [Task_2]
- **acceptance:**
  - Add endpoint returning ranked hypotheses with evidence, confidence, and suggested action.
  - Enforce opt-in and return empty-state response when disabled/insufficient data.
  - Add conservative phrasing guardrails in response assembly.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api`
  - required: true | owner: worker | kind: command | detail: `cargo clippy -p sleep-api -- -D warnings`

### Task_4
- **title:** Implement feedback capture endpoint and persistence
- **type:** impl
- **owns:**
  - `sleep-api/src/trends.rs`
  - `sleep-api/src/repository.rs`
  - `sleep-api/src/models/friction.rs`
  - `migrations/`
  - `openapi.yaml`
- **depends_on:** [Task_2]
- **acceptance:**
  - Add endpoint to submit “useful/not useful” feedback per hypothesis.
  - Persist minimal fields required for future relevance tuning.
  - Keep schema changes additive and backward compatible.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api`
  - required: true | owner: reviewer | kind: review | detail: migration and persistence are minimal-risk and reversible

### Task_5
- **title:** Build trends hidden-needs panel UI
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
  - `sleep-ui/src/lib/api.ts`
- **depends_on:** [Task_3]
- **acceptance:**
  - Add “Possible needs this week” panel with need, evidence, confidence, and action.
  - Provide clear empty/disabled/insufficient-data states.
  - Keep style within existing design tokens/components.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run check`
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`

### Task_6
- **title:** Add in-panel feedback interactions
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
  - `sleep-ui/src/lib/api.ts`
- **depends_on:** [Task_4, Task_5]
- **acceptance:**
  - Users can mark each hypothesis useful/not useful.
  - Feedback submission status is clear and resilient to request failures.
  - Interaction does not disrupt existing trends workflows.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`
  - required: true | owner: reviewer | kind: e2e-visual | detail: verify feedback flow and states across desktop/mobile

### Task_7
- **title:** Add cross-signal intent integration rules
- **type:** impl
- **owns:**
  - `sleep-api/src/trends.rs`
  - `sleep-api/src/repository.rs`
  - `docs/personalization-agent-action-map.md`
- **depends_on:** [Task_3]
- **acceptance:**
  - Incorporate assistant-intent signal hints as optional inputs to hypothesis ranking.
  - Keep ranking deterministic and auditable in v1.
  - Preserve strict fallback behavior when intent data is absent.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api`
  - required: true | owner: reviewer | kind: review | detail: scoring logic remains explainable and bounded

## Task Waves
- **Wave 1 (sequential):** Task_1 → Task_2
- **Wave 2 (parallel):** Task_3, Task_4
- **Wave 3 (parallel):** Task_5, Task_7
- **Wave 4 (sequential):** Task_6

## E2E / Visual Validation Spec
- **artifact_root:** `.playwright-cli`
- **base_url:** `http://127.0.0.1:4173`
- **app_start_command:** `cd sleep-ui && npm run build && npm run preview -- --host 127.0.0.1 --port 4173`
- **flows:**
  - Flow A: opt-in enabled shows hypotheses with evidence/confidence/action.
  - Flow B: disabled/insufficient data states are clear and non-alarming.
  - Flow C: useful/not useful feedback interaction and success/error states.
- **viewports:**
  - `1366x768`
  - `390x844`
- **evidence requirements:**
  - Save screenshots under `.playwright-cli/`.
  - Record console/network errors.

## Risks / Mitigations
- Inference overreach risk → strict language guardrails and prohibited categories.
- Privacy sensitivity risk → opt-in default and minimized retained fields.
- Relevance drift risk → explicit user feedback loop and confidence labeling.
- UI complexity risk → cap visible hypotheses and keep one-action format.

## Open Questions
- Should hidden-needs inference default to off until explicit consent?
- Which assistant intents should be eligible in v1 (free text vs mapped categories only)?
- What maximum number of weekly hypotheses keeps the panel actionable?
