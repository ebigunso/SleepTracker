# Quality Score Rubric

Use this rubric for review gates and self-checks. The goal is consistent quality across human and agent work.

This is intentionally lightweight. Use it to identify weak points and decide whether to:
- add validation
- tighten plan tasks
- improve docs/harness

Quality baseline entrypoint: `docs/coding-agent/quality/index.md`.

---

## Scoring

Score each dimension as 0–2:

- 0 = unacceptable / missing
- 1 = acceptable but has gaps
- 2 = strong / complete

### Dimensions

1) Correctness
- Does the change meet acceptance criteria?
- Are edge cases and errors handled appropriately?

2) Validation evidence
- Were required commands run (or explicitly waived)?
- If UI impacted: is there concrete UI evidence?

3) Maintainability
- Is the solution understandable and aligned with existing patterns?
- No dead code/config, no unnecessary complexity?

4) Consistency / taste
- Reuses existing types/interfaces where appropriate?
- Avoids redundant logic and avoids style drift?

5) Documentation & legibility
- Are docs updated if behavior or workflows changed?
- Is the repo more legible after the change?

6) Harness contribution (optional but encouraged)
- Did we add/update a doc/checklist/rule that prevents recurrence of mistakes?

---

## Interpreting totals

- 10–12: Excellent
- 7–9: Acceptable, but capture at least one improvement item
- ≤6: Do not mark done; rework plan/validation/review

---

## Notes

Use `docs/coding-agent/plans/tech-debt-tracker.md` to capture known debt items discovered during review.

For baseline-guided scoring:
- Principles context (broad intent): `docs/coding-agent/quality/backend-principles.md`, `docs/coding-agent/quality/frontend-principles.md`
- Verification gates (specific checks): `docs/coding-agent/quality/gates-architecture.md`, `docs/coding-agent/quality/gates-rust.md`, `docs/coding-agent/quality/gates-svelte-typescript.md`
- Required command/evidence mapping: `docs/coding-agent/references/validation.md`

Scoring is a quality summary, not a substitute for required validations.
