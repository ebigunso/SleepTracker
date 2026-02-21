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
