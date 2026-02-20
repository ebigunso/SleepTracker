# Plans (Execution Plans)

Plans in this repository are **execution plans**, not just initial decompositions.
They are expected to be updated during execution with progress and decision logs.

---

## Directory structure

- `active/` — plans currently in progress
- `completed/` — finished plans (validated and closed)
- `tech-debt-tracker.md` — recurring debt items / follow-up candidates

---

## Lifecycle

1) Create plan under `active/`:
- `docs/coding-agent/plans/active/<kebab>-plan.md`

2) Review and approve:
- status: `draft` → `approved`

3) Execute in waves:
- status: `in_progress`
- update Progress Log after each wave
- update Decision Log when re-planning is required

4) Close:
- status: `done`
- move the plan to `completed/`

---

## Conventions

- Task IDs are `Task_X` (Task_1, Task_2, …).
- Plans must include:
  - explicit validation ownership per task
  - explicit Task Waves with parallel dispatch intent
  - progress log and decision log sections
