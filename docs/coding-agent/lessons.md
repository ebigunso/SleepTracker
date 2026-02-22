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

## 2026-02-22 — Reviewer gate must include required UI evidence artifacts  [tags: validation, review]

Context:
- Plan: docs/coding-agent/plans/active/sleep-edit-theme-shell-unification-plan.md
- Task/Wave: Task_2 / Wave 2
- Roles involved: Orchestrator, Reviewer

Deviation:
- Reviewer returned NEEDS_REVISION because required dark-mode screenshots under `.playwright-cli/` were missing.
- Initial review evidence did not conclusively satisfy the planned create/edit/delete smoke-flow proof.

Root cause:
- Reviewer dispatch prompt did not enforce artifact path verification as a hard completion condition before returning status.
- Validation gate was interpreted as best-effort test execution instead of strict evidence checklist completion.

Fix applied:
- Paused execution and re-routed through improvement loop before continuing.
- Added a plan decision-log delta to re-run reviewer gate with explicit artifact capture and evidence checklist checks.

Prevention:
- Primary promotion target: references/ui-e2e.md
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: Reviewer prompts for UI-impact tasks must include explicit required artifact filenames/locations and fail-fast if missing.
- Optional guardrail:
  - Before accepting Reviewer APPROVED, verify each required artifact path exists.

Evidence:
- Reviewer report status NEEDS_REVISION with missing `.playwright-cli` screenshot artifacts.

## 2026-02-22 — Normalize cwd in persistent terminal validation runs  [tags: environment, validation]

Context:
- Plan: docs/coding-agent/plans/active/sleep-edit-theme-shell-unification-plan.md
- Task/Wave: Task_1 / Wave 1
- Roles involved: Worker

Deviation:
- Required command `cd sleep-ui && npm run test:unit` failed on first attempt because the persistent shell was already in `sleep-ui`.

Root cause:
- Validation commands assumed repository-root cwd for each invocation despite persistent terminal state.

Fix applied:
- Worker recovered by checking cwd and running `npm run test:unit` from the correct directory.

Prevention:
- Primary promotion target: troubleshooting/index.md
- Candidate prevention rule (optional):
  - audience: worker
  - proposed rule: In persistent terminal sessions, verify cwd (`pwd`) before repeated `cd <dir> && ...` sequences.
- Optional guardrail:
  - Prefer absolute-path command forms for required validation scripts when run serially.

Evidence:
- Worker report captured initial cwd-related failure followed by successful recovered run.

## 2026-02-22 — Split mixed-abstraction doc planning and enforce harmonization pass  [tags: planning, scope, docs]

Context:
- Plan: docs/coding-agent/plans/active/refactor-quality-baselines-plan.md
- Task/Wave: planning phase before execution
- Roles involved: Orchestrator

Deviation:
- Initial plan grouped high-level principles and lower-level architecture/language gates too coarsely, creating risk of worker context overload.
- Cross-document optimization/consistency safeguards were not explicit enough in the first decomposition.

Root cause:
- Plan decomposition prioritized deliverable grouping over abstraction-level separation.
- Missing default planning guardrail to force a final harmonization pass when documents are intentionally split by abstraction.

Fix applied:
- Replanned into finer tasks separating high-level principles from architecture and language-specific gates.
- Added explicit whole-spectrum harmonization task before reviewer gate.

Prevention:
- Primary promotion target: rules/orchestrator
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: When documentation spans multiple abstraction levels, decompose tasks by abstraction boundary and require a pre-review harmonization pass for consistency.
- Optional guardrail:
  - During plan drafting, run a quick check: “Are high-level principles, architecture gates, and language gates authored in separate tasks, and is there a final consistency sweep?”

Evidence:
- User explicitly requested finer task breakdown by abstraction level and explicit final consistency/optimization pass.

## 2026-02-22 — Enforce task owns/objective alignment in plan quality  [tags: planning, scope]

Context:
- Plan: docs/coding-agent/plans/active/refactor-quality-baselines-plan.md
- Task/Wave: plan refinement before Worker dispatch
- Roles involved: Orchestrator

Deviation:
- `Task_1` objective required backend and frontend high-level principles, but `owns` did not include the frontend principles file.
- This mismatch risked preventing Workers from fully satisfying acceptance criteria within allowed edit scope.

Root cause:
- Plan edit introduced scope/objective drift without a final integrity check for each task block (`owns`, `acceptance`, `depends_on`, `validation`).

Fix applied:
- Updated `Task_1` `owns` to include both backend and frontend principles files.
- Repaired the full plan structure to restore consistent task definitions and dependency links.

Prevention:
- Primary promotion target: rules/orchestrator
- Candidate prevention rule (optional):
  - audience: orchestrator
  - proposed rule: Before requesting approval or dispatching Workers, validate each Task_X for one-to-one alignment between objective language and declared `owns` paths.
- Optional guardrail:
  - Add a pre-dispatch checklist item: “Can every acceptance bullet be delivered by files listed in `owns`?”

Evidence:
- User flagged direct inconsistency between `Task_1` narrative and `owns` scope.

## 2026-02-22 — Resolve validation precedence conflicts during doc harmonization  [tags: validation, docs, review]

Context:
- Plan: docs/coding-agent/plans/active/refactor-quality-baselines-plan.md
- Task/Wave: Task_6 reviewer gate after Task_5 harmonization
- Roles involved: Worker, Reviewer, Orchestrator

Deviation:
- Reviewer returned NEEDS_REVISION because new quality gate docs contained required-validation semantics that conflicted with existing validation mapping references.
- Conflicts included Rust command/requirement mismatches and frontend E2E scope mismatch for `sleep-ui/src/**` vs `sleep-ui/tests/**` path mapping.

Root cause:
- Harmonization pass linked docs but did not enforce command/path parity against the canonical validation mapping as a hard checklist step.
- No explicit precedence rule was documented for conflict resolution between quality gate docs and validation mapping table.

Fix applied:
- Paused execution before plan close and captured reviewer findings as a deviation event.
- Initiated plan-delta path to add a targeted reconciliation task before rerunning reviewer gate.

Prevention:
- Primary promotion target: rules/common
- Candidate prevention rule (optional):
  - audience: common
  - proposed rule: When multiple docs define validation expectations, `docs/coding-agent/references/validation.md` is canonical for required checks unless explicitly superseded in the same change set with synchronized updates.
- Optional guardrail:
  - Harmonization checklist must include command/path parity verification across quality gates, validation mapping, and role rules.

Evidence:
- Reviewer Task_6 result NEEDS_REVISION with concrete conflict findings and file references.
