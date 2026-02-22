# Architecture Refactor Quality Gates

## Purpose

This document defines architecture-level quality gates for SleepTracker refactors.
Use these gates to evaluate cross-layer integrity (API, security, domain, persistence, and validation evidence) without duplicating Rust- or Svelte/TypeScript-specific checks.

## Scope

- Applies to non-trivial refactors that cross module/layer boundaries, change integration behavior, or alter validation strategy.
- Complements (does not replace) language/framework gate documents.
- Uses repository references for evidence expectations:
  - `docs/coding-agent/references/validation.md`
  - `docs/coding-agent/references/ui-e2e.md`

## Gate Definitions

### Gate A1: API Contract Coherence

- Definition:
  - Public API behavior remains coherent across transport layer, handlers, and client-facing contract expectations.
- What to verify:
  - Request/response shapes and error semantics are intentionally preserved or explicitly versioned/migrated.
  - Contract changes are reflected consistently across API surface documentation and integration touchpoints.
  - Refactor does not introduce silent behavior drift (status semantics, required fields, auth-related response behavior).
- Acceptable evidence:
  - Explicit contract-impact statement in PR/planning notes (no change vs intentional change).
  - Updated contract artifacts when applicable (for example `openapi.yaml` and related API examples/docs).
  - Validation evidence showing unchanged intended flows or intentional migration behavior.

### Gate A2: Security Boundary Integrity

- Definition:
  - Refactor preserves security boundary guarantees across authentication, CSRF, and request-protection controls.
- What to verify:
  - Security-critical logic is not bypassed, duplicated in weaker form, or moved to inappropriate boundaries.
  - Auth/session assumptions remain consistent between server-rendered and API-mediated flows.
  - Security-related failure paths remain explicit and safe by default.
- Acceptable evidence:
  - Clear boundary mapping in review notes for where security controls are enforced after refactor.
  - Targeted validation evidence for protected and rejected flows (authorized, unauthorized, invalid token/state).
  - No uncontrolled widening of trusted paths or bypass routes introduced by structural cleanup.

### Gate A3: Domain Invariant Preservation

- Definition:
  - Core business invariants remain centralized, enforceable, and unchanged unless intentionally re-specified.
- What to verify:
  - Invariants (for example overlap constraints, date/time semantics, and range validity) remain owned by the correct domain boundary.
  - Refactor does not duplicate domain rules across handlers/UI/persistence adapters.
  - Error behavior for invariant violations remains deterministic for consumers.
- Acceptable evidence:
  - Invariant impact checklist in PR/planning notes (which invariants touched, why, and expected behavior).
  - Validation evidence covering both valid and invalid cases that exercise unchanged/updated invariants.
  - Demonstrated single-source ownership of invariant logic after refactor.

### Gate A4: Data & Persistence Boundary Discipline

- Definition:
  - Persistence concerns remain behind explicit repository/data boundaries; refactors do not leak storage coupling upward.
- What to verify:
  - Domain and transport layers avoid direct persistence leakage (schema-specific behavior embedded outside data boundary).
  - Data shape transformations remain intentional at boundary seams, not ad hoc across layers.
  - Migration/data-model evolution is coordinated with boundary expectations and backward behavior.
- Acceptable evidence:
  - Boundary ownership notes in PR describing where persistence assumptions are now enforced.
  - If schema/data behavior changes, corresponding migration/design notes and compatibility expectations are documented.
  - Validation evidence that critical read/write flows still satisfy boundary contracts.

### Gate A5: Validation Evidence Completeness

- Definition:
  - Completion claims are supported by evidence proportional to architectural risk and mapped required validations.
- What to verify:
  - Required validation items are explicitly mapped from changed paths and plan ownership.
  - Missing validations are treated as blockers unless explicit waiver and risk statement are documented.
  - Evidence includes both automated checks and manual flow evidence where architecture/user-flow boundaries changed.
- Acceptable evidence:
  - Validation matrix in task/PR notes aligned with `docs/coding-agent/references/validation.md`.
  - Pass/fail outcomes recorded with clear scope (what was run, against what change surface).
  - Explicit unresolved-risk list when any required validation is deferred by waiver.

### Gate A6: E2E Isolation & Safety Expectations

- Definition:
  - End-to-end validation remains isolated, safe by default, and representative of real cross-layer behavior.
- What to verify:
  - E2E runs use isolated/local-safe defaults and do not target non-local environments without explicit override.
  - Evidence collection follows repository artifact/reporting conventions for reproducibility.
  - Refactors touching cross-layer flows preserve reliable startup/readiness and teardown assumptions in test harness behavior.
- Acceptable evidence:
  - E2E evidence artifacts and reporting aligned with `docs/coding-agent/references/ui-e2e.md`.
  - Confirmation that safety guardrails/isolation expectations remain enforced for default runs.
  - Flow-level evidence for impacted journeys (for example auth + dashboard log/edit/delete paths) with reproducible run context.

## Architecture Anti-Patterns

- Cross-layer “cleanup” that changes contract semantics without declaring contract impact.
- Security logic drift caused by moving checks across boundaries without equivalent guarantees.
- Domain-rule scattering (same invariant reimplemented in API handlers, UI, and persistence layer).
- Repository/data boundary bypasses introduced for short-term convenience.
- Declaring refactor complete with partial or non-reproducible validation evidence.
- Running E2E in ways that can hit shared/non-local targets by default.

## How To Use In Planning and Review

- In planning:
  - Mark which gates are in scope for the change and where risk is highest.
  - Define expected evidence for each in-scope gate before implementation starts.
- In implementation/review:
  - Require explicit pass/fail disposition for each in-scope gate.
  - Reject “done” claims when required gate evidence is missing or non-reproducible.
- In handoff:
  - Record unresolved risks, waivers, and follow-up ownership when any gate is partially satisfied.

## Out of Scope

This document intentionally excludes language/framework-specific commands and code-style checks.
Use language/framework gate documents for Rust- or Svelte/TypeScript-specific verification details.
