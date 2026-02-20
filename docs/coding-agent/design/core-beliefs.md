# Core Beliefs (Golden Principles)

These principles guide decisions across code, docs, and tooling in this repository.
They are intentionally stable and high-level.

---

## 1) Optimize for agent legibility

- Prefer explicit structure over cleverness.
- Encode key knowledge in `docs/coding-agent/` so it is discoverable and consistent.
- If a workflow step is important, document it and link it from the index.

## 2) Prefer mechanical enforcement over “remembering”

- If something must not be skipped (validation, boundaries), aim to make it enforceable:
  - scripts
  - CI checks
  - structural tests
- Use docs + checklists as the first step; promote to enforcement when it matters.

## 3) Small diffs, shipped in waves

- Prefer small, reviewable increments with clear validation.
- Break down work into Task_X and waves with explicit parallel boundaries (`owns`).

## 4) Evidence before “done”

- “Done” requires evidence:
  - required commands run (or explicitly waived)
  - required review gate passed
  - required UI/E2E evidence captured when UI correctness matters

## 5) Taste matters (but must be explainable)

- Prefer changes that reduce future cognitive load.
- If a change increases complexity, justify it and consider alternatives.

## 6) Improve the harness when the harness fails

- When a miss occurs (validation skipped, wrong assumption, unclear run steps):
  - capture a lesson (repo-local)
  - add a rule/doc to prevent recurrence
  - stage a migration candidate if it is cross-repo
