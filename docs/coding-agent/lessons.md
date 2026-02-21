# Lessons Log (Append-Only)

This file captures durable lessons discovered during work in this repository.
It is optimized for:
- quick triage
- searchability
- promotion into rules / docs / global skills later

---

## Rules for writing lessons (mandatory)

1) Atomic entries:
- Write **one lesson per distinct failure category**.
- If an entry has more than one independent root cause, split it.

2) Promotion target:
- Each entry must name exactly one primary promotion target:
  - repo rules (common/worker/orchestrator),
  - repo reference docs (how-to-run/validation/ui-e2e/improvement-loop),
  - troubleshooting entry,
  - or a future global skill.

3) Capture deviations (not just user corrections):
Write a lesson entry whenever there is a deviation, including:
- user course correction
- re-plan / plan delta due to new insight
- subagent blocked/failed
- reviewer needs revision/failed
- required validation skip/waiver or unexpected validation failure
- environment/tooling recovery that should become repeatable knowledge

4) Subagent-inclusive:
- Subagents should surface “Lesson Candidates” when they encounter deviations.
- Orchestrator is the single writer of this file and should persist the relevant candidates.

---

## Deviation signal patterns (examples)

These phrases often imply “this should change going forward”:
- “This should be accounted for going forward”
- “Prefer Y as a default”
- “Don’t do X next time”
- “You missed validation / evidence”
- “This is too cautious / too risky”
- “You should have … first”

When in doubt, capture a lesson. Atomic entries keep cost low.

---

## Lesson template (copy/paste)

## YYYY-MM-DD — <short title>  [tags: <comma-separated>]

Context:
- Plan:
- Task/Wave:
- Roles involved:

Deviation:
- <what went wrong or what required course correction, 1–3 bullets>

Root cause:
- <why it happened, 1–3 bullets>

Fix applied:
- <what was done to resolve it>

Prevention:
- Primary promotion target: <one of: rules/* | references/* | troubleshooting/* | global-skill>
- Candidate prevention rule (optional):
  - audience: common | worker | orchestrator
  - proposed rule: <one sentence>
- Optional guardrail:
  - <micro-checklist or dispatch/plan guardrail>

Evidence:
- <what confirmed this is a real recurring pattern>

---

## Entries

<!-- Append new lessons below this line. Keep entries atomic. -->

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
