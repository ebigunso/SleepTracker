# Plan: Address PR #62 Review Comments and Path Leak

- status: done
- generated: 2026-03-12
- last_updated: 2026-03-12
- work_type: docs

## Goal
- Remove the username-bearing local path leak from PR #62 and resolve the related documentation inconsistencies flagged in review, without expanding into unrelated UI polish unless explicitly requested.

## Definition of Done
- No committed PR artifacts include the leaked local absolute path or username-bearing validation command.
- The completed plan and lessons references are internally consistent after remediation.
- Review-facing follow-up can clearly distinguish which comments were addressed in code versus which were intentionally left for later.
- Targeted validation confirms the touched files no longer contain newly introduced local absolute path leaks.

## Scope / Non-goals
- Scope:
  - `docs/coding-agent/plans/completed/login-password-visibility-icons-plan.md`
  - `docs/coding-agent/lessons.md`
  - PR review triage for #62
- Non-goals:
  - Reworking historical promotion traceability entries outside the immediate PR follow-up
  - Additional UI restyling unless explicitly requested
  - Rewriting unrelated PR review comments into larger documentation cleanups

## Context (workspace)
- Related files/areas:
  - `docs/coding-agent/plans/completed/login-password-visibility-icons-plan.md`
  - `docs/coding-agent/lessons.md`
- Existing review findings:
  - Must address: absolute `/c/Users/...` validation command leak in the completed plan
  - Should address: dead active-plan reference in lessons, approval-state contradiction in the completed plan
  - Optional: auth toggle rounded-corner UI polish, modifier naming clarity reply
- Repo reference docs consulted:
  - `docs/coding-agent/rules/common.md`
  - `docs/coding-agent/rules/orchestrator.md`
  - `docs/coding-agent/lessons.md`

## Open Questions (max 3)
- Q1: None; keep remediation scoped to the privacy leak and the directly related documentation inconsistencies.

## Assumptions
- A1: Repo-relative or environment-agnostic validation commands are acceptable replacements for absolute local paths in plan evidence.
- A2: The rounded-corner UI review comment can be deferred unless the user wants to address non-blocking polish in this PR.

## Tasks

### Task_1: Sanitize the leaked local path and reconcile review-blocking doc inconsistencies
- type: impl
- owns:
  - `docs/coding-agent/plans/completed/login-password-visibility-icons-plan.md`
  - `docs/coding-agent/lessons.md`
- depends_on: []
- description: |
  Replace the committed absolute local validation path with a repo-relative or environment-agnostic command.
  Fix the stale plan-path reference in lessons and reconcile the approval-state inconsistency flagged in the completed plan.
- acceptance:
  - No username-bearing or machine-specific absolute validation path remains in the touched plan file.
  - The lessons entry references the completed plan path accurately.
  - The completed plan's progress and decision log no longer contradict the approval status history.
- validation:
  - kind: command
    required: true
    owner: worker
    detail: "Run a targeted search over the touched files for `C:/Users`, `/c/Users`, `%USERPROFILE%`, `%APPDATA%`, `AppData`, and `GitLocal` to confirm the new leak is removed."

### Task_2: Re-triage PR review comments after remediation
- type: review
- owns: []
- depends_on: [Task_1]
- description: |
  Re-check the PR review findings against the updated branch and identify which comments can be resolved, replied to, or intentionally deferred.
- acceptance:
  - The must-fix leak comment can be resolved after remediation.
  - The related documentation comments have a clear addressed/deferred status.
  - Any deferred comments are documented with rationale for user review.
- validation:
  - kind: review
    required: true
    owner: orchestrator
    detail: "Summarize PR #62 review comments after remediation and confirm which comments remain actionable."

## Task Waves (explicit parallel dispatch sets)

- Wave 1 (parallel): [Task_1]
- Wave 2 (parallel): [Task_2]

## Rollback / Safety
- Limit edits to the review-flagged documentation files.
- Preserve the factual execution history while sanitizing local path details and fixing broken references.

## Progress Log (append-only)

- 2026-03-12 00:00 Draft created.
  - Summary: Planned PR review remediation focused on the leaked local path and directly related doc inconsistencies.
  - Validation evidence: N/A (planning stage)
  - Notes: Awaiting user approval before execution.

- 2026-03-12 00:05 Execution started.
  - Summary: User approved remediation; proceeding with documentation fixes for the path leak and related review comments.
  - Validation evidence: N/A (execution in progress)
  - Notes: Dispatching Task_1 with targeted leak-search validation.

- 2026-03-12 00:12 Wave 1 completed: [Task_1]
  - Summary: Sanitized the completed plan's leaked absolute validation path, fixed the stale lessons reference, and clarified the approval-history note.
  - Validation evidence:
    - Targeted leak-marker search returned no matches in `docs/coding-agent/plans/completed/login-password-visibility-icons-plan.md`
    - `docs/coding-agent/lessons.md` still contains pre-existing `%USERPROFILE%` and `%APPDATA%` placeholders in historical promotion traceability entries, outside this plan's minimal remediation scope
  - Notes: The new privacy lesson was also scrubbed so it does not repeat the leaked command form.

- 2026-03-12 00:15 Wave 2 completed: [Task_2]
  - Summary: Re-triaged PR #62 review comments after remediation.
  - Validation evidence:
    - Must-fix leak comment: addressed by sanitizing the completed plan command
    - Related doc comments: addressed by fixing the stale lessons reference and clarifying the approval-history note
    - Remaining comments: rounded-corner UI polish is optional; modifier naming feedback can be answered without code changes
  - Notes: No additional code changes were required for the optional UI and naming comments.

## Decision Log (append-only; re-plans and major discoveries)

- 2026-03-12 00:00 Decision:
  - Trigger / new insight: PR research confirmed one username-bearing leak introduced by the branch plus two doc-consistency comments worth fixing in the same files.
  - Plan delta (what changed): Scoped remediation to the privacy leak and doc consistency items; left optional UI polish out of scope.
  - Tradeoffs considered: Keeping scope narrow reduces risk of mixing blocking privacy cleanup with unrelated UI restyling.
  - User approval: yes

- 2026-03-12 00:15 Decision:
  - Trigger / new insight: After remediation, the only remaining review items are a non-blocking UI polish suggestion and a naming-clarity comment.
  - Plan delta (what changed): No scope expansion; close the plan after pushing the documentation fixes to the PR branch.
  - Tradeoffs considered: Avoid mixing privacy/doc remediation with optional UI restyling in the same review-response pass.
  - User approval: yes

## Notes
- Risks:
  - Historical traceability entries still contain environment-specific placeholders outside this PR's minimal remediation scope.
  - Over-editing the completed plan could accidentally rewrite history instead of just clarifying it.
