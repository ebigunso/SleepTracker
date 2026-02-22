# Rules (Common)

- last_updated: 2026-02-22

These are repo-specific common rules that apply across roles (Orchestrator / Worker / Researcher / Reviewer).
Keep them short and concrete. Put deeper guidance in `docs/coding-agent/` references.

---

## 0. Principles

- Prefer minimal diffs: avoid unrelated formatting/renames unless explicitly required.
- Follow existing patterns and conventions unless the task is a migration.
- Non-trivial work requires an execution plan under `docs/coding-agent/plans/active/`.
- Do not claim “done” without required validation evidence (or explicit waiver).

---

## 1. Repository entrypoints

Primary entrypoint: `AGENTS.md` (repo root)

Knowledge store index: `docs/coding-agent/index.md`

---

## 2. Repository Reference Documents (must-read pointers)

This section exists because repo-local rules are editable at runtime.
Keep it updated with the docs that should be read before specific kinds of work.

| Document path | When to read | Why |
|---|---|---|
| `docs/coding-agent/design/core-beliefs.md` | before non-trivial changes | stable principles; avoids avoidable mistakes |
| `docs/coding-agent/design/architecture.md` | before cross-module or architectural changes | boundaries and dependency direction |
| `docs/coding-agent/design/taste.md` | before refactors or broad edits | maintainability heuristics |
| `docs/coding-agent/references/how-to-run.md` | before running local dev / UI checks | ports, readiness, startup |
| `docs/coding-agent/references/validation.md` | before marking tasks done | path→required validations mapping |
| `docs/coding-agent/references/ui-e2e.md` | before UI/E2E evidence collection | artifact rules + evidence checklist |
| `docs/coding-agent/references/improvement-loop.md` | after deviations, corrections, or missed gates | mandatory pause→lesson→prevention loop |
| `docs/coding-agent/quality/index.md` | before refactor/review quality checks | baseline layering (principles vs gates vs rubric) |
| `docs/coding-agent/quality/QUALITY_SCORE.md` | during review gates | consistent quality rubric |

---

## 3. Workflow (repo conventions)

- Preferred flow: Plan → Branch → PR (or equivalent) for non-trivial work.
- Keep work in small waves that preserve a runnable state.
- If you are on `main` at task start, create a new branch before edits.

Plans:
- Create plans under: `docs/coding-agent/plans/active/`
- Move completed plans to: `docs/coding-agent/plans/completed/`

---

## 4. Validation (repo-specific)

- Required validations are defined by:
  - the plan’s validation items, and
  - `docs/coding-agent/references/validation.md` mapping.

Rules:
- If a validation item is required and owned by a system role (worker/reviewer/orchestrator), it must be executed and evidenced before claiming done.
- Skips require explicit user waiver and must record risk.

UI evidence:
- Use `playwright-cli` for E2E/visual evidence.
- Store artifacts under `.playwright-cli/` and reference paths in reports.

---

## 5. Safety / boundaries

- Avoid breaking changes unless explicitly approved.
- Avoid touching unrelated modules outside the task scope.
- Prefer conservative edits when repository constraints are unclear; document assumptions.

---

## 6. Naming / structure (repo-specific)

- (Fill in repo naming conventions and directory rules here.)

---

## Global Migration Candidates

<!--
Add cross-repo / cross-task rules discovered at runtime here.
These will be migrated into global agent/skills definitions later.
-->

- (none yet)
