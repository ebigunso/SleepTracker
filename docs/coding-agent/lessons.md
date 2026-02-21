# Lessons Log (Coding Agent)

Purpose:
- Capture recurring mistakes and their prevention mechanisms.
- Reduce repeated failures by turning “corrections” into durable guardrails.

## How to use
- Append a new entry after any user correction or significant miss (validation skipped, review miss, scope drift, tooling failure).
- Keep entries short and actionable.
- Promote repeated/high-severity lessons into repo rules or skill drafts.

## Tags (recommended)
- planning
- validation
- delegation
- review
- ui-e2e
- tooling
- ci
- scope-owns

## Entries
(append newest at the top)

## 2026-02-21 — Keep lesson entries atomic for promotion  [tags: review, planning]

Context:
- Plan: docs/coding-agent/plans/active/sleep-ui-e2e-db-isolation-guardrails-plan.md
- Task/Wave: post-correction lesson capture
- Roles involved: Orchestrator

Symptom:
- A single large lesson bundled multiple unrelated misses, making triage and future promotion harder.

Root cause:
- I optimized for speed and wrote one combined narrative instead of atomic entries.

Fix applied:
- Split the combined lesson into focused entries by failure type.

Prevention:
- Repo rule candidate:
	- audience: orchestrator
	- proposed rule: Write one lesson per distinct failure category (e.g., planning, delegation, environment handling) with a single primary promotion target.
- Dispatch/plan guardrail (optional):
	- Before ending a correction turn, run a quick “entry atomization check”: if an entry has more than one independent root cause, split it.

Evidence:
- User correction explicitly requested triage-friendly lesson structure for global guideline promotion.

## 2026-02-21 — Capture E2E blocker learnings immediately  [tags: validation, ui-e2e, tooling]

Context:
- Plan: docs/coding-agent/plans/active/sleep-ui-e2e-db-isolation-guardrails-plan.md
- Task/Wave: E2E stabilization and environment troubleshooting
- Roles involved: Orchestrator

Symptom:
- I did not record persistent learnings immediately after user-provided E2E corrections (container restart guidance and data-handling issue).

Root cause:
- I treated runtime recovery as transient troubleshooting instead of a durable lesson capture event.

Fix applied:
- Recorded this as an explicit reusable lesson category.

Prevention:
- Repo rule candidate:
	- audience: orchestrator
	- proposed rule: Any user correction that resolves an E2E/runtime blocker must produce a lessons entry in the same turn, including the trigger and verified recovery action.
- Dispatch/plan guardrail (optional):
	- Add a turn-closing check item: “Were new troubleshooting insights converted to lessons/rules candidates?”

Evidence:
- User correction highlighted missed E2E lesson capture after Docker restart and data handling guidance.

## 2026-02-21 — Enforce subagent-first + plan-first for non-trivial review work  [tags: planning, delegation, review]

Context:
- Plan: docs/coding-agent/plans/active/sleep-ui-e2e-db-isolation-guardrails-plan.md
- Task/Wave: PR53 review-comment response
- Roles involved: Orchestrator

Symptom:
- I moved directly into PR comment research/fixes without dispatching subagents and without formalizing the plan first.

Root cause:
- I misclassified the PR comment response as a quick local edit instead of non-trivial multi-step work.

Fix applied:
- Re-established explicit requirement to plan first and dispatch subagents before implementing non-trivial PR comment responses.

Prevention:
- Repo rule candidate:
	- audience: orchestrator
	- proposed rule: For PR threads with multiple unresolved comments, perform subagent triage and write/refresh an execution plan before any code edits.
- Dispatch/plan guardrail (optional):
	- Minimum sequence: (1) collect comments, (2) classify must/should, (3) update plan, (4) dispatch subagent(s), (5) implement.

Evidence:
- User correction explicitly called out missing subagent dispatch and missing upfront plan formalization.
