# Refactor Quality Baselines Plan

- status: completed
- owner: Orchestrator
- created: 2026-02-22
- updated: 2026-02-22
- related_request: Establish repository refactor principles and quality gates baselines (backend/frontend + architecture/language-specific)

## Objective

Create durable, repo-grounded baseline documents that define refactor principles and quality gates for:
- broad categories: backend and frontend
- specific categories: architecture-level and language/stack-level (Rust, Svelte/TypeScript)

These baselines must be easy to reference during planning, implementation, and review.

## Task Waves

- Wave 1 (sequential): Task_1
- Wave 2 (parallel): Task_2, Task_3, Task_4
- Wave 3 (sequential): Task_5
- Wave 4 (sequential): Task_6
- Wave 5 (sequential): Task_7
- Wave 6 (sequential): Task_8

## Task_1

- type: docs
- owns:
  - docs/coding-agent/quality/backend-principles.md
  - docs/coding-agent/quality/frontend-principles.md
- depends_on: []
- acceptance:
  - High-level baseline principles are defined separately for backend and frontend.
  - Principles remain broad and stable (non-language-specific), with cross-links to lower-level gate documents deferred to later tasks.
  - Content is repository-grounded and avoids generic-only guidance.
- validation:
  - required: true
    owner: worker
    kind: doc-review
    detail: Verify principles are clear, non-overlapping, and remain at high abstraction level.

## Task_2

- type: docs
- owns:
  - docs/coding-agent/quality/gates-architecture.md
- depends_on:
  - Task_1
- acceptance:
  - Architecture-level quality gates are documented as a dedicated baseline.
  - Gates cover cross-layer concerns (contracts, security, validation evidence, integration boundaries).
  - Architecture gate guidance references relevant existing repository references where applicable.
- validation:
  - required: true
    owner: worker
    kind: doc-review
    detail: Confirm architecture gates are actionable, evidence-oriented, and not duplicated from language-specific docs.

## Task_3

- type: docs
- owns:
  - docs/coding-agent/quality/gates-rust.md
- depends_on:
  - Task_1
- acceptance:
  - Rust-specific quality gates are documented with repository-valid commands and code-quality checks.
  - Guidance addresses backend invariants relevant to this repository (auth/csrf, time correctness, repository boundaries).
  - Guidance is language-focused and does not duplicate architecture-level gates.
- validation:
  - required: true
    owner: worker
    kind: doc-review
    detail: Confirm Rust gates are specific, runnable, and aligned with repository validation references.

## Task_4

- type: docs
- owns:
  - docs/coding-agent/quality/gates-svelte-typescript.md
- depends_on:
  - Task_1
- acceptance:
  - Svelte/TypeScript-specific quality gates are documented with repository-valid commands and frontend flow guardrails.
  - Guidance addresses SSR/auth boundary, shared API wrapper usage, and E2E evidence expectations.
  - Guidance is language/framework-focused and does not duplicate architecture-level gates.
- validation:
  - required: true
    owner: worker
    kind: doc-review
    detail: Confirm Svelte/TypeScript gates are specific, runnable, and aligned with repository validation references.

## Task_5

- type: docs
- owns:
  - docs/coding-agent/quality/index.md
  - docs/coding-agent/quality/QUALITY_SCORE.md
  - docs/coding-agent/index.md
  - docs/coding-agent/rules/common.md
- depends_on:
  - Task_2
  - Task_3
  - Task_4
- acceptance:
  - A final whole-spectrum harmonization pass is completed across all written baseline documents.
  - Inconsistencies between high-level principles and lower-level architecture/language gates are resolved.
  - Discoverability is updated in quality/index, coding-agent/index, and common rules references.
  - Quality rubric references baseline docs without changing rubric intent.
- validation:
  - required: true
    owner: worker
    kind: consistency-check
    detail: Cross-document pass verifies terminology alignment, non-duplication, and compatible guidance across all baseline docs.

## Task_6

- type: review
- owns:
  - docs/coding-agent/quality/**
  - docs/coding-agent/index.md
  - docs/coding-agent/rules/common.md
- depends_on:
  - Task_5
- acceptance:
  - Reviewer confirms baseline docs are complete for requested categories.
  - Reviewer confirms high-level principles and lower-level gates remain cleanly separated.
  - Reviewer confirms final harmonization pass resolved whole-spectrum consistency issues.
  - Reviewer confirms no conflicts with existing repo rules/references.
  - Reviewer confirms final artifacts are suitable as refactor quality assessment baselines.
- validation:
  - required: true
    owner: reviewer
    kind: review
    detail: Structured review with APPROVED/NEEDS_REVISION decision and concrete findings.

## Task_7

- type: docs
- owns:
  - docs/coding-agent/quality/gates-rust.md
  - docs/coding-agent/quality/gates-svelte-typescript.md
  - docs/coding-agent/quality/index.md
- depends_on:
  - Task_6
- acceptance:
  - Validation semantics in quality gate docs are reconciled with `docs/coding-agent/references/validation.md`.
  - Rust gate command and requirement wording no longer conflicts with canonical/required mapping.
  - Frontend E2E requirement scope no longer conflicts with path-based required mapping.
  - Quality baseline index includes a precedence/interpretation note to prevent future conflicts.
- validation:
  - required: true
    owner: worker
    kind: consistency-check
    detail: Verify command/path parity and required-vs-recommended semantics align with canonical validation mapping.

## Task_8

- type: review
- owns:
  - docs/coding-agent/quality/**
  - docs/coding-agent/index.md
  - docs/coding-agent/rules/common.md
- depends_on:
  - Task_7
- acceptance:
  - Reviewer confirms previous Task_6 major conflicts are resolved.
  - Reviewer confirms required validation semantics are internally consistent across quality and reference docs.
  - Reviewer returns APPROVED for final baseline artifact set.
- validation:
  - required: true
    owner: reviewer
    kind: review
    detail: Structured rerun focused on conflict resolution and final completion readiness.

## Progress Log

- 2026-02-22: Plan drafted from user request + researcher context; awaiting user approval.
- 2026-02-22: Replanned per user correction to further decompose by abstraction level and add explicit whole-document harmonization pass before review.
- 2026-02-22: Fixed Task_1 ownership inconsistency to include both backend and frontend principle documents and repaired plan structure integrity.
- 2026-02-22: User approved plan; execution started.
- 2026-02-22: Task_6 reviewer returned NEEDS_REVISION for validation-semantics conflicts; execution paused and plan delta approved for targeted reconciliation + review rerun.
- 2026-02-22: Task_7 completed with focused reconciliation across quality gate docs and baseline index precedence guidance.
- 2026-02-22: Task_8 reviewer rerun returned APPROVED; all plan acceptance and validation gates satisfied.

## Decision Log

- 2026-02-22: Chose dedicated quality baseline docs under docs/coding-agent/quality/ to keep principles modular and referenceable.
- 2026-02-22: Increased task granularity to reduce worker context overload by separating high-level principles from architecture and language-specific gate authoring.
- 2026-02-22: Added Task_5 harmonization pass to handle cross-spectrum consistency/optimization limits that may arise from split task execution.
- 2026-02-22: Enforced strict owns/acceptance consistency so each task objective is fully representable within its declared edit scope.
- 2026-02-22: Added Task_7/Task_8 to resolve reviewer-identified conflicts between quality gates and canonical validation mapping before plan closeout.
