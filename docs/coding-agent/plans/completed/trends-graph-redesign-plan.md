# Trends Graph Redesign Plan

## Goal
Redesign the Trends chart so each metric uses a visualization that matches its data semantics, improving readability and decision-making while preserving existing date controls and schedule view.

## Re-evaluation Summary (Data → Visual)
- **Duration (minutes, continuous over time):** line + points (trend clarity).
- **Quality (1–5, discrete/ordinal):** point chart with integer ticks (distribution clarity, avoids misleading bar area).
- **Bedtime (clock/circular time):** wrapped-time line + points (fix midnight discontinuity while keeping continuity).
- **Wake time (clock/circular time):** wrapped-time line + points (same rationale as bedtime).

## Definition of Done
- Trends chart uses metric-specific visualization mapping above instead of one shared bar type.
- Bedtime/wake time are readable around midnight without false large jumps.
- Existing controls remain intact: range presets, custom date range, chart/schedule toggle.
- Tooltips and axis labels remain human-readable (`h:mm`, `HH:MM`, quality score).
- Required frontend checks and reviewer-owned visual evidence pass.

## Scope / Non-goals
**In scope**
- Frontend Trends chart redesign in current route/component path.
- Tests/docs updates for changed chart behavior.

**Non-goals**
- Replacing Chart.js or introducing a new charting library.
- Redesigning the schedule timeline (`SleepBar`) view.
- Changing personalization recommendation logic.
- Broad backend metric expansion.
- Adding a second summary/statistics row in chart view.

## Context (workspace)
- Primary UI: `sleep-ui/src/routes/trends/+page.svelte`
- API helpers/types: `sleep-ui/src/lib/api.ts`
- Time formatting helpers: `sleep-ui/src/lib/utils/sleep.ts`
- Backend summary endpoint: `sleep-api/src/trends.rs` (`GET /api/trends/summary`)
- Docs/contract: `docs/feature-reference.md`, `openapi.yaml`

## Tasks

### Task_1
- **title:** Define chart-spec contract per metric
- **type:** design
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
- **depends_on:** []
- **acceptance:**
  - Specify per-metric chart type, axis behavior, tooltip format, and missing-value behavior.
  - Keep existing controls/flows unchanged.
  - Confirm wrapped-time strategy for bedtime/wake time (e.g., anchor day boundary at midday).
- **validation:**
  - required: true | owner: worker | kind: review | detail: ensure spec aligns with existing `MetricKey` set and current route controls

### Task_2
- **title:** Refactor chart rendering to metric-driven configuration
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
- **depends_on:** [Task_1]
- **acceptance:**
  - Replace one-size-fits-all chart config with per-metric config builder.
  - Preserve theme token usage (`--color-chart-*`, `--color-*`) and current lifecycle safety (`chart.destroy()`).
  - Keep chart/schedule toggle behavior unchanged.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run check`

### Task_3
- **title:** Implement duration and quality visual redesign
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
- **depends_on:** [Task_2]
- **acceptance:**
  - Duration renders as line + points.
  - Quality renders as points with integer y-axis ticks (`1..5`) and clear handling of missing values.
  - Tooltip labels remain consistent with existing text formatting.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`

### Task_4
- **title:** Implement bedtime/wake-time wrapped-time charts
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
  - `sleep-ui/src/lib/utils/sleep.ts`
- **depends_on:** [Task_2]
- **acceptance:**
  - Bedtime/wake time render as connected line + points on wrapped/circular-friendly axis.
  - Axis ticks and tooltip values display familiar local clock times.
  - Near-midnight points are visually adjacent rather than split far apart.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`
  - required: true | owner: reviewer | kind: e2e-visual | detail: screenshot evidence for bedtime/waketime around midnight cases

### Task_5
- **title:** Update docs and complete review gate
- **type:** docs
- **owns:**
  - `docs/feature-reference.md`
  - `openapi.yaml` (only if contract usage/documentation changes)
  - `sleep-ui/tests/e2e.spec.ts` (if coverage expansion needed)
  - `sleep-ui/tests/unit/` (targeted trend-related tests)
- **depends_on:** [Task_3, Task_4]
- **acceptance:**
  - Document metric-specific chart mapping and any summary-context behavior.
  - Add/adjust tests for new transform/format behavior where appropriate.
  - Reviewer approves visual correctness across desktop/mobile viewports.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run build`
  - required: true | owner: reviewer | kind: command | detail: `cd sleep-ui && npm run test:e2e`

## Task Waves
- **Wave 1 (sequential):** Task_1 → Task_2
- **Wave 2 (parallel):** Task_3, Task_4
- **Wave 3 (sequential):** Task_5

## E2E / Visual Validation Spec
- **artifact_root:** `.playwright-cli`
- **base_url:** `http://127.0.0.1:4173`
- **app_start_command:** `cd sleep-ui && npm run build && npm run preview -- --host 127.0.0.1 --port 4173`
- **readiness_check:** GET `/trends` returns page containing heading `Trends`
- **flows:**
  - Flow A: Duration and quality metric switching
    - Open `/trends`
    - Switch between Duration and Quality metrics
    - Verify chart type and y-axis semantics match spec
  - Flow B: Bedtime and wake-time midnight readability
    - Seed/use data with near-midnight values
    - Verify points near 23:xx and 00:xx appear adjacent in wrapped-time view
  - Flow C: Schedule fallback parity
    - Toggle to Schedule view and confirm unchanged timeline behavior
- **viewports:**
  - `1366x768`
  - `390x844`
- **evidence requirements:**
  - Save screenshots per flow/viewport under `.playwright-cli/`
  - Record console errors and failed network requests

## Risks / Mitigations
- Wrapped-time transform can confuse users if labeling is unclear → keep all labels/tooltips in familiar clock time.
- Mixed chart types can introduce chart lifecycle bugs → centralize config and keep explicit destroy/recreate path.
- Optional summary row can crowd mobile layout → ship only compact one-row stats; defer if cluttered.

## Open Questions
None.
