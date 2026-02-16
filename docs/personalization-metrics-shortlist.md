# Personalization metrics shortlist (data-backed)

This document captures a **focused, implement-first** set of personalization metrics, based on observed data in the live backend DB snapshot on 2026-02-17.

## Snapshot basis

Observed in `/data/sleep.db` (copied read-only for analysis):

- Sleep sessions: **152** (wake-date range: 2025-08-22 → 2026-02-15)
- Exercise daily intensity rows: **151**
- Notes rows: **1**
- Duration distribution (min): p25 **322.5**, p50 **360**, p75 **480**, p90 **600**
- Fixed unusual-duration rule hit rate (`<2h || >14h`): **1.3%**
- Weekday vs weekend duration median delta: **+60 min** (weekend longer)
- Social jetlag (mid-sleep median delta, weekend - weekday): **+45 min**
- Timing variability: bed/wake standard deviation about **140-147 min**
- Exercise distribution: mostly `none` (147 `none`, 4 `light`, 0 `hard`)
- Notes tags: no hashtags in current dataset

---

## Implement-first 6 metrics (with go/no-go)

| Metric | Why this is high value now | Personalization enabled | Go threshold | No-go / defer condition |
|---|---|---|---|---|
| Personal duration baseline (rolling p10/p50/p90 + IQR) | Duration has strong spread and enough samples to define a stable personal range | Adaptive unusual-duration warnings and baseline-aware trend annotations | >= 60 sessions in lookback and IQR >= 60 min | Fewer than 30 sessions in lookback or highly sparse logging |
| Sleep timing baseline (bed/wake/mid-sleep medians by weekday vs weekend) | Clear weekday/weekend separation and broad timing variance in current data | Day-type smart defaults and schedule-shift insights | >= 8 weekday and >= 4 weekend sessions per lookback window | Work/travel period dominates window or recent behavior reset |
| Social jetlag indicator (weekend-mid - weekday-mid) | Effect size is already meaningful (+45 min) | Trigger schedule regularity nudges and timeline callouts | Absolute delta >= 30 min for 2 consecutive windows | Delta unstable across windows or weekend sample too small |
| Schedule variability score (std dev or robust MAD of bed/wake) | High variability exists today and is a direct personalization target | Prioritize consistency-focused insights over duration-only messaging | Variability >= 60 min and persists across 2 windows | Window too small (< 21 days) or logging gaps dominate |
| Quality-aligned factor ranking (quality vs timing/duration features) | Quality has usable spread (1-5 with concentration at 4/5, but not degenerate) | Prioritize changes likely to improve your own high-quality nights | >= 40 sessions with non-missing quality and at least 3 distinct quality levels | Quality scoring behavior changes abruptly (scale drift) |
| Friction cost metrics (time-in-form, error_kind, immediate edit rate, partial follow-up failure) | Highest expected ROI metric family for autonomous roadmap decisions; currently missing and should be added first | Rank UX improvements by expected minutes saved/week | >= 30 submit flows captured and at least one recurrent friction cluster | Too few events, or events cannot be tied to save outcomes |

---

## Defer for now (insufficient signal)

| Metric family | Current status | Revisit when |
|---|---|---|
| Exercise-conditioned personalization | Very low class diversity (`hard` absent, `light` rare) | At least 20+ non-`none` intensity days and >= 2 intensity levels with usable counts |
| Notes/tag correlation mining | Notes nearly absent (1 row, no hashtags) | >= 20 noted days and recurring tags (>= 5 occurrences/tag for top tags) |
| Nap-specific personalization | Nap-like sessions are rare (2.6%), multi-session days rare (0.7%) | >= 10% nap-like sessions or >= 10 multi-session days in lookback |
| Latency/awakenings personalization | Metrics effectively near-zero in this dataset | Non-trivial variance appears (e.g., p90 > 0 in sustained windows) |

---

## Practical rollout order

1. Implement personal duration + timing baselines.
2. Add social jetlag + variability indicators.
3. Add quality-aligned ranking logic.
4. Instrument friction cost telemetry (the only new collection required for high-ROI UX automation).
5. Re-evaluate deferred families monthly using the revisit thresholds above.
