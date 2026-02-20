# Personalization Rollout Flags Removal Plan

## Goal
Remove rollout-flag complexity for personalization endpoints in this single-user app while keeping endpoint behavior, security, and tests intact.

## Definition of Done
- Personalization routes are always registered (no rollout gating in router wiring).
- Personalization env flag readers are removed from config.
- Tests no longer depend on setting personalization rollout env flags.
- OpenAPI and docs no longer describe rollout-gated/404-off behavior.
- Backend/frontend validation commands pass.

## Scope / Non-goals
**In scope**
- Remove `ENABLE_PERSONALIZATION_*` gating code and docs references.
- Keep endpoint contracts and authorization/CSRF behavior unchanged.

**Non-goals**
- Changing personalization metrics/logic itself.
- Reworking auth model or UI feature behavior.

## Tasks

### Task_1
- **title:** Remove backend rollout gating and flag readers
- **type:** impl
- **owns:**
  - `sleep-api/src/app.rs`
  - `sleep-api/src/config.rs`
  - `sleep-api/tests/auth_csrf.rs`
  - `sleep-api/tests/trends_bars.rs`
- **depends_on:** []
- **acceptance:**
  - Personalization routes are registered unconditionally in router.
  - `config.rs` no longer exposes `personalization_*_enabled` readers.
  - Backend tests no longer set personalization env flags.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cargo fmt -- --check`
  - required: true | owner: worker | kind: command | detail: `cargo clippy -- -D warnings`
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api --test trends_bars`
  - required: true | owner: worker | kind: command | detail: `cargo test -p sleep-api --test auth_csrf`

### Task_2
- **title:** Remove rollout-gated wording from API/docs
- **type:** docs
- **owns:**
  - `openapi.yaml`
  - `README.md`
  - `docs/personalization-agent-action-map.md`
  - `docs/feature-reference.md`
- **depends_on:** [Task_1]
- **acceptance:**
  - No remaining references to `ENABLE_PERSONALIZATION_*` flags.
  - No docs claiming routes are absent/404 when flags are off.
  - Endpoint documentation remains accurate for auth/csrf requirements.
- **validation:**
  - required: true | owner: worker | kind: review | detail: grep for removed flag names returns no matches in docs/OpenAPI.

### Task_3
- **title:** Final regression validation and PR metadata refresh
- **type:** chore
- **owns:**
  - PR branch `feat/personalization-action-map`
  - PR #48 metadata
- **depends_on:** [Task_2]
- **acceptance:**
  - Frontend checks still pass (`check`, `test:unit`, `build`).
  - Changes are committed in reasonable chunks on the PR branch.
  - PR #48 description reflects no rollout flags.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run check`
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run build`

## Task Waves
- **Wave 1:** Task_1
- **Wave 2:** Task_2
- **Wave 3:** Task_3
