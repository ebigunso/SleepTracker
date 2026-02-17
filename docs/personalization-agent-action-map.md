# Personalization agent action map

This is a companion to `docs/personalization-metrics-shortlist.md`.
It maps each high-priority metric to concrete autonomous actions, including trigger thresholds and guardrails.

## How to use this page

- Treat each row as a candidate automation rule.
- Run rules on a rolling window (for example: 28 days), then compare with the prior window.
- Apply only when trigger + confidence + guardrails are all satisfied.

## Backend endpoint mapping

Backend endpoints used by this map:

- `GET /api/trends/personalization`
- `POST /api/personalization/friction-telemetry`
- `GET /api/personalization/friction-backlog`

These endpoints are available as part of the API:

- `GET /api/trends/personalization`
- `POST /api/personalization/friction-telemetry`
- `GET /api/personalization/friction-backlog`

## Metric-to-action matrix

| Metric | Primary analysis question | Agent action candidates | Trigger to act | Guardrail before applying |
|---|---|---|---|---|
| Personal duration baseline (p10/p50/p90, IQR) | Is current sleep duration outside personal norm? | 1) Replace static unusual-duration warning with personalized range. 2) Add contextual warning text based on personal tails. | >= 60 sessions in baseline window and out-of-range incidence >= 5% in recent window | Do not apply if baseline window includes major schedule disruption period (travel/shift changes) |
| Timing baseline by day type (weekday/weekend medians) | Are start/end times predictably different by day type? | 1) Prefill form defaults using day-type median bed/wake time. 2) Offer one-click “Use your usual weekday/weekend times.” | >= 8 weekday and >= 4 weekend sessions in window, with stable medians across 2 windows | Do not auto-switch defaults if recent 14-day pattern diverges strongly from baseline |
| Social jetlag indicator (weekend-mid minus weekday-mid) | Is schedule phase shifting on weekends? | 1) Show schedule-shift insight card. 2) Suggest consistency-oriented trend view by default. | Absolute midpoint delta >= 30 min for 2 consecutive windows | Suppress if weekend sample is too small (< 4 sessions) |
| Schedule variability score (bed/wake dispersion) | Is irregular timing the main instability source? | 1) Prioritize regularity insight over duration-only insight. 2) Suggest adding quick rounding controls or consistent-time shortcuts in future UX backlog. | Variability >= 60 min and persists across 2 windows | Defer if data gaps are high (missing days > 30% in window) |
| Quality-aligned factor ranking | Which factors most align with higher quality nights? | 1) Rank top 2-3 actionable factors in dashboard insight text. 2) Shift default trend explanation toward quality-linked factors. | >= 40 sessions with quality and >= 3 distinct quality values; factor effect is stable across adjacent windows | Use directional language only (“associated with”), never causal language |
| Friction cost metrics (form time/errors/retries/immediate edits/partial follow-up failures) | Which workflow pain points waste the most time? | 1) Maintain auto-ranked UX backlog by estimated minutes saved/week. 2) Promote top item to implementation proposal when persistent. | >= 30 captured submit flows and at least one friction pattern persists for 2 windows | Require explicit evidence summary before proposing implementation changes |

## Backlog proposal policy (for autonomous suggestions)

When generating a feature/change proposal, include:

1. **Observed evidence** (counts/rates/deltas in current + prior window)
2. **Expected benefit** (estimated time saved per week or reduced rework)
3. **Confidence level** (high/medium/low based on sample and stability)
4. **Rollback condition** (what metric change invalidates the proposal)

Only auto-promote proposals when confidence is **medium or higher**.

## Suggested proposal templates

### Template A: Default tuning proposal

- **Problem:** Static defaults diverge from observed personal baseline.
- **Evidence:** Day-type medians stable across two windows.
- **Proposed change:** Update form defaults and warning thresholds to personal baseline.
- **Success metric:** Reduced immediate edit rate and reduced warning dismissals.

### Template B: Friction reduction proposal

- **Problem:** Repeated flow failures/retries indicate avoidable input friction.
- **Evidence:** Error cluster persists (same error kind or repair pattern).
- **Proposed change:** Add targeted UX affordance (quick-adjust, better boundary handling, retry guidance).
- **Success metric:** Reduced retries and reduced median time-in-form.

### Template C: Insight prioritization proposal

- **Problem:** Current insight emphasis does not match strongest personal drivers.
- **Evidence:** Quality-aligned factor ranking stable over adjacent windows.
- **Proposed change:** Reorder dashboard/trends insights to emphasize top factors.
- **Success metric:** Improved consistency in quality-linked outcomes over time.

## Non-goals

- No cohort comparisons or broad product analytics.
- No heavy event instrumentation beyond friction telemetry required for ranking.
- No automatic action when confidence is low or data quality guardrails fail.
