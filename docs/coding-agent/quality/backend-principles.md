# Backend Refactor Quality Baseline

## Purpose

This baseline defines stable, high-level principles for backend refactors in SleepTracker.
Use this document to protect security invariants, domain correctness, and contract stability while changing internals.

## Principles

- Preserve security invariants first.
  - Authentication, CSRF protections, and security header behavior are non-negotiable system guarantees; refactors must keep these guarantees intact even if implementation structure changes.

- Keep domain rules explicit and centralized.
  - Date/time semantics, DST behavior, valid ranges, and overlap constraints represent business truth; refactors should clarify these rules instead of distributing them across unrelated layers.

- Protect contract stability at system boundaries.
  - API shapes, status behavior, and error contracts should remain predictable for existing clients unless a deliberate versioned change is approved.

- Prefer behavior-preserving internal change over coupled rewrites.
  - Repository, handler, and domain internals may evolve, but refactors should minimize cross-layer coupling and avoid changing multiple boundaries without clear need.

- Improve observability of critical paths.
  - Refactors should make failures and invariants easier to diagnose (for example, clearer error intent and boundary ownership) without exposing sensitive details.

- Make correctness easy to validate.
  - Structure changes so security, time/domain correctness, and contract behavior can be validated directly and repeatedly, not inferred indirectly.

## Anti-Patterns To Avoid

- Security drift: changing auth/CSRF/header behavior as a side effect of unrelated cleanup.
- Domain rule duplication: re-implementing DST/range/overlap logic in multiple layers.
- Hidden contract changes: altering request/response or error semantics without explicit boundary review.
- Cross-layer entanglement: refactors that mix domain, transport, and persistence concerns without clear ownership.
- “Big bang” backend rewrites: replacing multiple boundaries at once without staged, behavior-preserving transitions.

## How To Use In Planning and Review

- During planning: map the refactor to these principles and identify which invariant category (security, domain correctness, contract stability) is most at risk.
- During implementation: use the principles as guardrails to keep scope behavior-preserving and boundary-aware.
- During review: reject changes that violate these principles even if code style or local structure appears improved.
- For detailed checks: pair this baseline with architecture/language-specific gate documents when they are in scope.
