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
### Promotion Traceability (Task_7: 2026-02-25)

The following lessons were promoted into durable docs/skills and removed from active lesson entries:

- 2026-02-22 — Complete plan lifecycle before task close → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md` (plan lifecycle closeout gate)
- 2026-02-22 — Reviewer gate must include required UI evidence artifacts → `%USERPROFILE%/.agents/skills/playwright-e2e-evidence/SKILL.md`, `docs/coding-agent/references/ui-e2e.md`
- 2026-02-22 — Normalize cwd in persistent terminal validation runs → `%USERPROFILE%/.agents/skills/workspace-troubleshooting/SKILL.md`
- 2026-02-22 — Split mixed-abstraction doc planning and enforce harmonization pass → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md`, `%USERPROFILE%/.agents/skills/plan-format/SKILL.md`
- 2026-02-22 — Enforce task owns/objective alignment in plan quality → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md`, `%USERPROFILE%/.agents/skills/plan-format/SKILL.md`
- 2026-02-22 — Resolve validation precedence conflicts during doc harmonization → `docs/coding-agent/rules/common.md`, `docs/coding-agent/references/validation.md`
- 2026-02-22 — Expand skill plans with explicit language/tech depth on request → `%USERPROFILE%/.agents/skills/plan-format/SKILL.md`
- 2026-02-25 — Favor high-signal lesson capture over routine iteration logging → `%APPDATA%/Code/User/prompts/Orchestrator.agent.md`, `%USERPROFILE%/.agents/skills/improvement-loop/SKILL.md`, `docs/coding-agent/references/improvement-loop.md`

## 2026-02-24 — Skill triggerability requires frontmatter-first precision  [tags: skills, planning, quality]

Context:
- Plan: docs/coding-agent/plans/active/engineering-quality-baselines-review-remediation-plan.md
- Task/Wave: pre-execution review remediation planning
- Roles involved: Orchestrator

Deviation:
- Skill trigger criteria were written mainly in body sections ("When to Use") instead of being concentrated in the frontmatter `description`, reducing trigger quality.
- Root skill guidance remained partially descriptive instead of fully operational (quick-start procedure, stop condition, and structured output template).

Root cause:
- Initial skill drafting prioritized conceptual clarity over trigger mechanics and operational execution ergonomics.

Fix applied:
- Added remediation tasks to rewrite SKILL frontmatter/body for triggerability and operational workflow.
- Added consistency tasks for progressive-disclosure enforcement, template conformance, and policy precedence harmonization.

Prevention:
- Primary promotion target: rules/orchestrator
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: When authoring skills, encode trigger criteria in frontmatter `description` first and keep body optimized for execution flow (quick start, routing, stop conditions, output template).
- Optional guardrail:
  - Pre-dispatch checklist: "Can trigger logic be inferred from frontmatter alone?"

Evidence:
- User provided explicit review comments requiring trigger criteria relocation and operational SKILL.md restructuring.

## 2026-02-25 — Ambiguity reduction requires taxonomy and evidence-field alignment  [tags: skills, quality, consistency]

Context:
- Plan: docs/coding-agent/plans/active/engineering-quality-baselines-ambiguity-remediation-plan.md
- Task/Wave: pre-execution assessment and planning
- Roles involved: Orchestrator

Deviation:
- Follow-up review identified ambiguity from mixed terminology (routing levels vs validation levels), underspecified evidence fields, and broad routing lists that can cause unnecessary doc loading.

Root cause:
- Initial remediation improved structure but left small cross-doc terminology and output-template mismatches that reduce operational precision.

Fix applied:
- Planned targeted updates to tighten trigger description, add load-cues, align taxonomy terms, strengthen required-check evidence fields, clarify stop conditions, and add missing-mapping fallback guidance.

Prevention:
- Primary promotion target: rules/orchestrator
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: For skill reviews, run a final ambiguity pass focused on trigger precision, taxonomy alignment, and evidence-template enforceability before declaring done.
- Optional guardrail:
  - Include a checklist item: "Can this template be completed without hand-waving required validation outcomes?"

Evidence:
- User provided explicit recommendation set targeting triggerability, routing efficiency, evidence rigor, and taxonomy consistency.

## 2026-02-25 — Treat third-party skills as read-only unless explicitly approved  [tags: scope, skills, governance]

Context:
- Plan: docs/coding-agent/plans/active/global-promotion-of-lessons-plan.md
- Task/Wave: planning delta before execution approval
- Roles involved: Orchestrator

Deviation:
- Planned updates included `skill-creator`, but user clarified it is sourced from credible third parties and should not be freely modified.

Root cause:
- Promotion planning optimized for consistency across skill files and did not apply a provenance-based editability check before scoping targets.

Fix applied:
- Removed `skill-creator` from planned `owns` and acceptance criteria in the active plan.
- Marked `skill-creator` as read-only reference input for this effort.

Prevention:
- Primary promotion target: rules/orchestrator
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: Before planning edits to global skills, verify provenance/editability constraints and exclude third-party-managed assets unless explicitly approved.
- Optional guardrail:
  - Add a pre-approval checklist item: "Are all targeted files editable under current governance constraints?"

Evidence:
- User explicitly directed: skip planned modifications to `skill-creator` because it is third-party sourced.

## 2026-02-25 — Keep SKILL.md as routing, move procedural runbooks to references  [tags: skills, structure, quality]

Context:
- Plan: follow-up refinement after global-promotion-of-lessons execution
- Task/Wave: post-implementation correction handling
- Roles involved: Orchestrator

Deviation:
- Shell troubleshooting procedures were written directly in `workspace-troubleshooting/SKILL.md` instead of being placed in a reference file with SKILL-level routing cues.

Root cause:
- Previous update optimized for content completeness and missed progressive-disclosure placement discipline from skill-authoring guidance.

Fix applied:
- Refactored `workspace-troubleshooting/SKILL.md` to keep high-level routing guidance only.
- Added `workspace-troubleshooting/references/persistent-shell-cwd-normalization.md` for detailed shell/cwd runbook steps.

Prevention:
- Primary promotion target: rules/orchestrator
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: For skill updates, place command-level troubleshooting procedures in `references/*` and keep SKILL.md focused on trigger/routing guidance.
- Optional guardrail:
  - Before finalizing skill edits, check: "Does SKILL.md contain only core rules + when-to-read pointers, with detailed runbooks moved to references?"

Evidence:
- User explicitly requested relocating shell troubleshooting steps from SKILL.md into reference docs and using skill-creator best practices.

## 2026-02-25 — Default commit strategy should enforce logical chunking  [tags: process, git, workflow]

Context:
- Plan: follow-up preference capture after documentation updates
- Task/Wave: post-commit user correction
- Roles involved: Orchestrator

Deviation:
- Commit chunking was performed in this run, but it was not yet established as an explicit default workflow behavior.

Root cause:
- Commit execution behavior relied on local task judgment instead of a persisted default preference.

Fix applied:
- Adopted chunked, logically scoped commits as the default behavior for subsequent tasks.

Prevention:
- Primary promotion target: rules/orchestrator
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: Create commits in cohesive, reviewable chunks that align to logical units of work unless the user requests otherwise.
- Optional guardrail:
  - Before committing, check: "Does this commit represent one coherent intent and avoid unrelated changes?"

Evidence:
- User explicitly requested making reasonable-chunk commits the default going forward.

## 2026-02-25 — Persist new workflow defaults in lessons during the same turn  [tags: process, lessons, workflow]

Context:
- Plan: follow-up preference capture after documentation updates
- Task/Wave: post-commit user correction
- Roles involved: Orchestrator

Deviation:
- A new default workflow preference was applied, but lesson capture for promotion readiness was not recorded in the same turn.

Root cause:
- Preference adoption and lesson persistence were handled as separate steps instead of one atomic correction flow.

Fix applied:
- Added explicit lesson capture for the new default workflow preference.

Prevention:
- Primary promotion target: references/improvement-loop
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: When a user sets a persistent default, append a lesson entry in the same turn so promotion tracking is never deferred.
- Optional guardrail:
  - Before ending a turn with default-behavior changes, check: "Is the new default persisted in lessons with a promotion target?"

Evidence:
- User explicitly requested that default preferences be captured in lessons for later promotion.
