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

### Promotion Traceability

The following lessons were promoted into durable docs/skills and removed from active lesson entries:

- 2026-02-22 — Complete plan lifecycle before task close → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md` (plan lifecycle closeout gate)
- 2026-02-22 — Reviewer gate must include required UI evidence artifacts → `%USERPROFILE%/.agents/skills/playwright-e2e-evidence/SKILL.md`, `docs/coding-agent/references/ui-e2e.md`
- 2026-02-22 — Normalize cwd in persistent terminal validation runs → `%USERPROFILE%/.agents/skills/workspace-troubleshooting/SKILL.md`
- 2026-02-22 — Split mixed-abstraction doc planning and enforce harmonization pass → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md`, `%USERPROFILE%/.agents/skills/plan-format/SKILL.md`
- 2026-02-22 — Enforce task owns/objective alignment in plan quality → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md`, `%USERPROFILE%/.agents/skills/plan-format/SKILL.md`
- 2026-02-22 — Resolve validation precedence conflicts during doc harmonization → `docs/coding-agent/rules/common.md`, `docs/coding-agent/references/validation.md`
- 2026-02-22 — Expand skill plans with explicit language/tech depth on request → `%USERPROFILE%/.agents/skills/plan-format/SKILL.md`
- 2026-02-24 — Skill triggerability requires frontmatter-first precision → `%USERPROFILE%/.agents/skills/skills-maintenance/SKILL.md`
- 2026-02-25 — Favor high-signal lesson capture over routine iteration logging → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md`, `%USERPROFILE%/.agents/skills/improvement-loop/SKILL.md`, `docs/coding-agent/references/improvement-loop.md`
- 2026-02-25 — Ambiguity reduction requires taxonomy and evidence-field alignment → `%USERPROFILE%/.agents/skills/skills-maintenance/SKILL.md`
- 2026-02-25 — Treat third-party skills as read-only unless explicitly approved → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md`, `%USERPROFILE%/.agents/skills/skills-maintenance/SKILL.md`
- 2026-02-25 — Keep SKILL.md as routing, move procedural runbooks to references → `%USERPROFILE%/.agents/skills/skills-maintenance/SKILL.md`
- 2026-02-25 — Default commit strategy should enforce logical chunking → `%USERPROFILE%/.agents/skills/git-workflow/SKILL.md`
- 2026-02-25 — Persist new workflow defaults in lessons during the same turn → `%USERPROFILE%/.agents/skills/improvement-loop/SKILL.md`
- 2026-02-25 — Commit work on a branch, not directly on main → `%USERPROFILE%/.agents/skills/git-workflow/SKILL.md`

### Lesson Entries
<!-- Append new lessons below this line. Keep entries atomic. -->

## 2026-03-11 — Do not infer plan approval from follow-up requirements  [tags: workflow, approval-gate, assumptions]

Context:
- Plan: `docs/coding-agent/plans/active/login-password-visibility-icons-plan.md`
- Task/Wave: Pre-Wave 1 / Task_1 dispatch
- Roles involved: Orchestrator, User, Worker

Deviation:
- I treated a follow-up requirement clarification (theme-correct icon coloring) as implicit approval to execute a non-trivial plan.
- I dispatched Task_1 before the user gave an explicit approval signal.

Root cause:
- I collapsed “scope clarification” and “approval” into the same signal instead of treating approval as a separate gate.
- I did not require an explicit yes/approve-style acknowledgment before moving from planning to execution.

Fix applied:
- Paused further execution after the correction instead of continuing into reviewer work.
- Recorded the deviation and updated the active plan to reflect the pause and the need for explicit approval to continue beyond already-dispatched work.

Prevention:
- Primary promotion target: global-skill
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: Treat follow-up requirements, clarifications, and refinements as non-approval unless the user explicitly approves the plan or directly instructs execution.
- Optional guardrail:
  - Before dispatching any non-trivial Worker task after a plan, confirm the latest user message contains an explicit approval or direct execution instruction; otherwise stop and ask.

Evidence:
- User correction on 2026-03-11: "you should have not assumed I gave approval when that is not the obvious intention of the message."
