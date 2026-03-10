# Global Promotion of Lessons Plan

- status: completed
- owner: Orchestrator
- created: 2026-02-25
- updated: 2026-02-25
- related_request: Plan lesson-promotion changes first (global prompts + global skills + repo-local alignment)

## Objective

Translate vetted lesson-promotion guidance into a sequenced implementation plan that:
- updates global orchestration/review behavior where universally applicable,
- keeps repo-specific canonical-source/path choices local,
- avoids overlap between `playwright-cli` and evidence-spec skills,
- and preserves strict gate semantics (required items must have evidence to close).

## Scope

In scope:
- Global prompt files under `%APPDATA%/Code/User/prompts/`
- Global skill files under `%USERPROFILE%/.agents/skills/` (except third-party `skill-creator`, treated as read-only)
- Repo-local alignment updates under `docs/coding-agent/rules/` and/or `docs/coding-agent/references/` if needed for precedence/path conventions.

Out of scope:
- Product code changes in `sleep-api/` or `sleep-ui/`
- Non-related rule/skill refactors.

## Task Waves

- Wave 1 (sequential): Task_1
- Wave 2 (parallel): Task_2, Task_3, Task_4
- Wave 3 (sequential): Task_5
- Wave 4 (sequential): Task_6
- Wave 5 (sequential): Task_7

## Task_1

- type: docs
- owns:
  - docs/coding-agent/plans/active/global-promotion-of-lessons-plan.md
- depends_on: []
- acceptance:
  - Plan captures promotion targets for all vetted lessons.
  - Plan explicitly separates globally generalizable rules from repo-local overrides.
  - Plan defines required validation ownership for every execution task.
- validation:
  - required: true
    owner: orchestrator
    kind: plan-review
    detail: Verify tasks/waves/owns/dependencies are coherent and aligned with requested promotion strategy.

## Task_2

- type: docs
- owns:
  - ../../AppData/Roaming/Code/User/prompts/Orchestrator.agent.md
- depends_on:
  - Task_1
- acceptance:
  - Add/strengthen explicit plan lifecycle closeout gate before final completion state.
  - Add generalized required-evidence completeness gate (not UI-only) with fail-fast completion blocking.
  - Add pre-dispatch task integrity check ensuring acceptance criteria are satisfiable within `owns` and validation ownership is explicit.
  - Add mixed-abstraction planning heuristic requiring harmonization pass before review when abstraction levels are split.
  - Keep worker-first delegation default language clear for implementation/cleanup follow-ups.
  - Refine lesson-capture threshold language to favor high-signal capture while preserving mandatory capture for hard gate misses.
- validation:
  - required: true
    owner: worker
    kind: doc-review
    detail: Confirm all new gates are normative and non-conflicting with existing Plan Gate / Validation Gate semantics.

## Task_3

- type: docs
- owns:
  - ../../.agents/skills/plan-format/SKILL.md
- depends_on:
  - Task_1
- acceptance:
  - Add plan-integrity checklist guidance (owns vs acceptance, validation owner/required semantics).
  - Add mixed-abstraction decomposition + harmonization-pass planning guidance.
  - Preserve existing wave semantics and parallel-by-default behavior.
  - Include language/tech depth decomposition trigger when user explicitly requests that depth.
- validation:
  - required: true
    owner: worker
    kind: doc-review
    detail: Confirm plan-format remains concise while adding enforceable planning checks.

## Task_4

- type: docs
- owns:
  - ../../.agents/skills/playwright-e2e-evidence/SKILL.md
  - ../../.agents/skills/workspace-troubleshooting/SKILL.md
  - ../../.agents/skills/improvement-loop/SKILL.md
- depends_on:
  - Task_1
- acceptance:
  - `playwright-e2e-evidence` explicitly requires reviewer artifact-path existence checks and fail status when required artifacts are missing.
  - `playwright-e2e-evidence` remains focused on evidence/spec shape and does not duplicate `playwright-cli` operational command guidance.
  - `workspace-troubleshooting` includes persistent-shell cwd normalization and recovery checklist patterns.
  - `improvement-loop` wording aligns with high-signal lesson-capture threshold while preserving mandatory capture for hard-gate deviations.
- validation:
  - required: true
    owner: worker
    kind: doc-review
    detail: Confirm role separation across skills and no duplicated operational guidance with `playwright-cli`.

## Task_5

- type: docs
- owns:
  - docs/coding-agent/rules/common.md
  - docs/coding-agent/rules/orchestrator.md
  - docs/coding-agent/references/validation.md
  - docs/coding-agent/references/ui-e2e.md
  - docs/coding-agent/references/improvement-loop.md
- depends_on:
  - Task_2
  - Task_3
  - Task_4
- acceptance:
  - Repo-local docs preserve canonical precedence details that are path-specific to this repository.
  - Globalized language is mirrored locally without contradiction.
  - Any path-specific artifact-root/canonical-doc requirements remain explicitly documented as local overrides.
  - `Global Migration Candidates` sections are updated with finalized migration bullets where appropriate.
- validation:
  - required: true
    owner: worker
    kind: consistency-check
    detail: Cross-check local rules/references against updated global prompt/skills for precedence and terminology parity.

## Task_6

- type: review
- owns:
  - ../../AppData/Roaming/Code/User/prompts/Orchestrator.agent.md
  - ../../.agents/skills/plan-format/SKILL.md
  - ../../.agents/skills/playwright-e2e-evidence/SKILL.md
  - ../../.agents/skills/workspace-troubleshooting/SKILL.md
  - ../../.agents/skills/improvement-loop/SKILL.md
  - docs/coding-agent/rules/common.md
  - docs/coding-agent/rules/orchestrator.md
  - docs/coding-agent/references/validation.md
  - docs/coding-agent/references/ui-e2e.md
  - docs/coding-agent/references/improvement-loop.md
- depends_on:
  - Task_5
- acceptance:
  - Reviewer confirms all required promotion decisions are implemented with correct global-vs-local boundaries.
  - Reviewer confirms no critical conflicts among plan gates, validation gates, and improvement-loop behavior.
  - Reviewer confirms evidence/gate vocabulary is consistently enforced (required/optional, owner, evidence, gate).
  - Reviewer returns APPROVED or provides concrete NEEDS_REVISION findings.
- validation:
  - required: true
    owner: reviewer
    kind: review
    detail: Structured review with explicit approval status and issue severities.

## Task_7

- type: docs
- owns:
  - docs/coding-agent/lessons.md
- depends_on:
  - Task_6
- acceptance:
  - Remove lesson entries only when the promoted behavior has been implemented and verified in target global/repo docs.
  - Keep non-migrated, partially migrated, or still-actionable lessons intact.
  - For removed entries, preserve traceability by adding a brief note in `lessons.md` indicating promotion completion and destination.
  - Lesson-log cleanup remains append-safe and does not remove unrelated historical entries.
- validation:
  - required: true
    owner: worker
    kind: consistency-check
    detail: Verify each removed lesson has a matching implemented promotion target and no unresolved lesson evidence requirement.

## Progress Log

- 2026-02-25: Plan drafted from user request and current global/repo cross-reference; awaiting user approval before execution.
- 2026-02-25: User approved plan; execution started.
- 2026-02-25: Task_2 completed (global Orchestrator gate updates).
- 2026-02-25: Task_3 completed (plan-format integrity/decomposition guidance updates).
- 2026-02-25: Task_4 completed (playwright-e2e-evidence, workspace-troubleshooting, and improvement-loop updates).
- 2026-02-25: Task_5 completed (repo-local rules/reference alignment and migration bullets).
- 2026-02-25: Task_6 Reviewer gate returned APPROVED with no issues.
- 2026-02-25: Task_7 completed (removed migrated redundant lessons and added promotion traceability block).
- 2026-02-25: All required validation/review gates satisfied; plan marked completed.

## Decision Log

- 2026-02-25: Chose to separate global prompt updates (Task_2), plan-structure skill updates (Task_3), and evidence/troubleshooting/authoring skill updates (Task_4) for parallel execution.
- 2026-02-25: Kept canonical validation-source paths and artifact-root specifics as repo-local alignment concerns (Task_5) while globalizing only the precedence/evidence patterns.
- 2026-02-25: Added explicit reviewer gate (Task_6) to enforce completion-quality evidence before closeout.
- 2026-02-25: User constrained third-party `skill-creator` to read-only handling; removed planned modifications and kept its existing guidance as a reference input only.
- 2026-02-25: Added final cleanup step (Task_7) to remove only migrated-and-redundant lesson entries after review approval.
- 2026-02-25: Evaluated Task_5 lesson candidate on include-pattern usage; treated as low-signal non-gate deviation and not persisted as a new lesson entry.
