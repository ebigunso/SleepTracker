# Trends Purpose-First Improvements Plan

## Goal
Improve the Trends experience so each visualization answers a clear user question: what changed, whether it is favorable, and what action is most useful next.

## Definition of Done
- Selected metric shows a clear purpose statement and period-over-period delta.
- Users can optionally compare current period against previous period in the same chart.
- Each metric has lightweight interpretation cues that reduce ambiguity.
- Insight recommendations are visually tied to relevant chart evidence (dates/segments).
- Existing controls (range, custom dates, chart/schedule toggle) remain intact.
- Frontend checks pass and reviewer approves desktop/mobile visual clarity.

## Scope / Non-goals
**In scope**
- Trends page UX and chart behavior changes in existing frontend stack.
- Reuse existing personalization/trends APIs and summary endpoint where useful.
- Minimal explanatory microcopy to improve scanability and actionability.

**Non-goals**
- Replacing Chart.js.
- Broad personalization model rewrite.
- New pages/modals or major IA changes.
- Backend schema migration for this phase.

## Context (workspace)
- Main UI: `sleep-ui/src/routes/trends/+page.svelte`
- API client: `sleep-ui/src/lib/api.ts`
- Time helpers: `sleep-ui/src/lib/utils/sleep.ts`
- Trends APIs: `sleep-api/src/trends.rs`, `openapi.yaml`

## Tasks

### Task_1
- **title:** Define metric purpose matrix and comparison rules
- **type:** design
- **owns:**
  - `docs/feature-reference.md`
  - `docs/personalization-agent-action-map.md`
- **depends_on:** []
- **acceptance:**
  - Map each metric to a primary user question and action intent.
  - Define period comparison policy (alignment, missing data handling, default on/off).
  - Define interpretation cues per metric with low-clutter constraints.
- **validation:**
  - required: true | owner: worker | kind: review | detail: spec is compatible with current trends payloads and UI controls

### Task_2
- **title:** Add selected-metric purpose and delta row
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
- **depends_on:** [Task_1]
- **acceptance:**
  - Render concise “question + delta vs prior period” row above chart.
  - Support all metrics with graceful fallback when prior data is unavailable.
  - Preserve current card layout and design tokens.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run check`

### Task_3
- **title:** Add optional prior-period comparator overlay
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
  - `sleep-ui/src/lib/utils/sleep.ts`
- **depends_on:** [Task_1]
- **acceptance:**
  - Add muted comparator series for selected metric and prior period.
  - Keep wrapped-time behavior correct for bedtime/waketime.
  - Tooltip clearly distinguishes current vs prior values.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run test:unit`
  - required: true | owner: reviewer | kind: e2e-visual | detail: verify readability and interpretation on duration + bedtime views

### Task_4
- **title:** Add metric-specific interpretation cues
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
  - `sleep-ui/src/lib/api.ts`
- **depends_on:** [Task_2, Task_3]
- **acceptance:**
  - Duration includes personal reference band/cue when data exists.
  - Quality includes clear midpoint/context cue.
  - Bed/wake include variability cue tied to existing variability metrics.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run check`
  - required: true | owner: reviewer | kind: review | detail: cues improve interpretation without clutter

### Task_5
- **title:** Connect recommendation cards to chart evidence
- **type:** impl
- **owns:**
  - `sleep-ui/src/routes/trends/+page.svelte`
- **depends_on:** [Task_4]
- **acceptance:**
  - Recommendation cards can highlight or focus relevant chart period/points.
  - Evidence and recommendation are co-located and understandable at a glance.
  - No extra pages/modals introduced.
- **validation:**
  - required: true | owner: reviewer | kind: e2e-visual | detail: confirm recommendation-to-data traceability on desktop/mobile

### Task_6
- **title:** Add optional weekly rollup lens from summary endpoint
- **type:** impl
- **owns:**
  - `sleep-ui/src/lib/api.ts`
  - `sleep-ui/src/routes/trends/+page.svelte`
- **depends_on:** [Task_2]
- **acceptance:**
  - Add optional smoothed weekly lens backed by `/api/trends/summary`.
  - Keep daily view as default and highest-priority flow.
  - Do not increase cognitive load in default state.
- **validation:**
  - required: true | owner: worker | kind: command | detail: `cd sleep-ui && npm run build`
  - required: true | owner: reviewer | kind: review | detail: weekly lens is optional and clearly distinct from daily data

## Task Waves
- **Wave 1 (sequential):** Task_1
- **Wave 2 (parallel):** Task_2, Task_3
- **Wave 3 (sequential):** Task_4
- **Wave 4 (sequential):** Task_5
- **Wave 5 (sequential):** Task_6

## E2E / Visual Validation Spec
- **artifact_root:** `.playwright-cli`
- **base_url:** `http://127.0.0.1:4173`
- **app_start_command:** `cd sleep-ui && npm run build && npm run preview -- --host 127.0.0.1 --port 4173`
- **flows:**
  - Flow A: verify purpose+delta scanability for each metric.
  - Flow B: verify prior-period overlay readability for duration and bedtime.
  - Flow C: verify interpretation cues are visible but not dominant.
  - Flow D: verify recommendation card highlights corresponding chart evidence.
- **viewports:**
  - `1366x768`
  - `390x844`
- **evidence requirements:**
  - Save screenshots per flow/viewport under `.playwright-cli/`.
  - Record console errors and failed requests.

## Risks / Mitigations
- Comparator overlay can confuse period alignment → explicit labeling and legend text in tooltip.
- Cue overload can reduce clarity → enforce one primary cue per metric.
- Sparse data can produce noisy deltas → include confidence/insufficient-data fallback copy.

## Open Questions
- Should prior-period comparator default to on, or remain opt-in?
- Should weekly rollup be available for all metrics or duration/quality only?
