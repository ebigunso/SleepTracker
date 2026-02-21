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

## 2026-02-22 — Complete plan lifecycle before task close  [tags: planning, review]

Context:
- Plan: docs/coding-agent/plans/completed/pr53-unresolved-comments-and-main-conflict-resolution-plan.md
- Task/Wave: post-merge housekeeping
- Roles involved: Orchestrator

Deviation:
- Completed execution work but did not immediately set plan status to completed and move plan files from active to completed.

Root cause:
- Turn closure focused on code/PR outcomes and skipped final plan-lifecycle checkpoint.

Fix applied:
- Updated relevant plan files to `status: completed`.
- Moved completed plans from `docs/coding-agent/plans/active/` to `docs/coding-agent/plans/completed/`.

Prevention:
- Primary promotion target: rules/orchestrator
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: Before final response on non-trivial tasks, run a mandatory “plan lifecycle closeout” check (status + archive move).
- Optional guardrail:
  - Add a turn-closing checklist item: “Are all executed active plans marked completed and archived?”

Evidence:
- User explicitly flagged missed plan completion/archival after task execution.
