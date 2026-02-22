# Coding Agent Knowledge Store (System of Record)

This directory is the **system of record** for agent operation in this repository.
It is structured for progressive disclosure: short indexes point to deeper docs.

If something important is not documented here (or linked from here), assume agents will not reliably do it.

---

## Navigation

### 1) Rules (role + repo constraints)
- `rules/index.md` — what rule files exist and who should read what
- `rules/common.md` — shared repo constraints, validation policy, boundaries
- `rules/orchestrator.md` — orchestration details and repo conventions
- `rules/worker.md` — worker scope discipline and reporting expectations

### 2) Plans (execution plans, not just initial decompositions)
- `plans/README.md` — plan lifecycle and directory structure
- `plans/active/` — active execution plans
- `plans/completed/` — completed execution plans
- `plans/tech-debt-tracker.md` — known debt items and follow-up candidates

### 3) Design (architecture + “taste”)
- `design/core-beliefs.md` — stable principles that guide changes and reviews
- `design/architecture.md` — repository architecture overview (update as needed)
- `design/taste.md` — conventions, maintainability expectations, review “smell tests”

### 4) References (how to run, validate, verify)
- `references/how-to-run.md` — local dev, ports, startup, readiness checks
- `references/validation.md` — path → required validations mapping (repo-specific)
- `references/ui-e2e.md` — UI/E2E evidence using `playwright-cli` and artifact rules
- `references/improvement-loop.md` — deviation handling checklist (pause → lesson → prevention)

### 5) Quality & troubleshooting
- `quality/index.md` — quality baseline entrypoint (principles + gates + rubric layering)
- `quality/QUALITY_SCORE.md` — quality rubric (for self-review and gating)
- `troubleshooting/index.md` — repo-specific troubleshooting notes

### 6) Runtime accumulation (optional files)
These may exist depending on how the repo has been used:
- `lessons.md` — repo-local lessons log (runtime captured)
- `skill-candidates.md` — staged skill candidates for later migration
- `skill-drafts/` — draft skill packages (repo-local staging)

---

## How to keep this knowledge store healthy

- Prefer adding *small, high-signal* docs over growing a single huge file.
- Keep indexes current (this file + `rules/index.md` + `plans/README.md`).
- If an agent repeatedly misses a step, treat it as a harness defect:
  - add a doc, rule, or checklist
  - consider making it mechanically enforceable later (scripts + CI)

---

## Common “missing doc” triggers

Add or update docs under `references/` when:
- agents repeatedly miss required validation commands
- local startup steps are unclear (ports, env, prerequisites)
- UI flows need repeatable evidence collection instructions
- repository boundaries or conventions are implied but not written down
