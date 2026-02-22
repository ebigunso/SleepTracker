# Rust Refactor Quality Gates

## Purpose

Use these gates for refactors that touch Rust backend code (`sleep-api/**`, workspace `Cargo.toml`, `migrations/**`).
These are language/runtime-specific checks and backend-invariant checks for this repository.

## Required vs Recommended Validation Semantics

Run from repository root unless noted.

Canonical required validations are defined by path in `docs/coding-agent/references/validation.md`.
For backend source changes, apply the canonical scope exactly as mapped there:

- `sleep-api/src/**` when E2E harness/runtime is touched (for example API startup/config changes) → `cargo test -p sleep-api`

Outside canonical-required scope, `cargo test -p sleep-api` remains strongly recommended for Rust refactors.

For formatting command parity with canonical command mapping, use:

- `cargo fmt --all` (or `cargo fmt --all --check` for CI-style verification)

Recommended Rust refactor quality gates (strongly recommended unless task scope is explicitly narrow):

- Lint gate (warnings fail gate): `cargo clippy -- -D warnings`
- Build gate for backend binaries/libraries: `cargo build -p sleep-api`

When a refactor touches API runtime/container-related backend paths (as mapped in worker rules), also run:

- `docker build -t sleep-api:ci .`

## Invariant-Focused Rust Quality Checks

In addition to command gates, review code changes against these repository invariants.

### 1) Auth and CSRF Enforcement Invariants

- Auth extraction/layer behavior stays enforced on protected routes.
- CSRF double-submit flow remains intact for mutating requests (CSRF cookie + `X-CSRF-Token` header semantics).
- Refactors in `sleep-api/src/auth.rs`, `sleep-api/src/middleware/`, or `sleep-api/src/security/` do not weaken default protection paths.

Evidence to seek:

- Existing auth/CSRF tests continue to pass (for example in `sleep-api/tests/auth_csrf.rs`).
- No new bypass path introduced by handler/middleware signature or routing changes.

### 2) Time and DST Correctness Invariants

- Date/time conversions preserve current domain semantics (including local date interpretation and timezone handling behavior).
- DST transition behavior remains correct and deterministic.
- Refactors do not duplicate time logic across handlers/repository when shared utilities already define behavior.

Evidence to seek:

- Time-focused tests pass (for example `sleep-api/tests/time_dst.rs`, `sleep-api/tests/settings_timezone.rs`).
- No ad-hoc timezone math added in handlers when central time/domain modules exist.

### 3) Range and Overlap Domain Invariants

- Sleep session overlap prevention remains enforced.
- Duration and date-range validations remain consistent with domain rules.
- Refactors keep validation ownership clear (domain/repository boundary), not split inconsistently across layers.

Evidence to seek:

- Sleep API behavior tests pass (for example `sleep-api/tests/api_sleep.rs`, `sleep-api/tests/api_sleep_list.rs`).
- Overlap/range logic changes have explicit tests or existing tests proving unchanged behavior.

### 4) Repository Transaction and Data Correctness Invariants

- Repository operations remain atomic where required; partial writes are not introduced.
- SQLx query and mapping changes preserve data correctness and expected error handling.
- Refactors keep business rules from leaking into ad-hoc SQL scattered across unrelated modules.

Evidence to seek:

- Repository-focused tests pass (for example `sleep-api/tests/friction_repository.rs`).
- Any migration interaction preserves compatibility with existing migration history (do not edit past migrations).

## Rust Refactor Anti-Patterns

- Replacing strongly typed domain/time models with loosely typed primitives in handlers.
- Moving security checks out of shared middleware/auth pathways into per-handler ad-hoc logic.
- Refactoring to “clean up” by merging handler, domain, and repository responsibilities.
- Introducing silent fallback behavior for parse/validation failures that used to be explicit errors.
- Changing transaction boundaries as an incidental side effect of function extraction.

## How To Use In Planning and Review

- Planning:
  - Identify which invariant set is touched (auth/csrf, time/DST, range/overlap, repository/data).
  - Define required command gates and any targeted regression tests before coding.
- Implementation:
  - Keep refactor commits behavior-preserving and boundary-local where possible.
  - Reuse existing modules for auth/security/time/repository logic rather than duplicating rules.
- Review:
  - Require canonical required evidence first (per `references/validation.md`), then recommended gate evidence (`fmt`, `clippy`, `build`, and `docker build` when applicable).
  - Reject refactors that pass style checks but weaken invariants or blur ownership boundaries.
