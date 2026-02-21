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

## 5) Generalize rules; avoid special-cases

- Prefer rules that apply across tasks and repos.
- If something looks like a one-off rule, rewrite it as:
  - a general principle, plus
  - a few examples / heuristics.
- Complexity-driven behavior beats situation-driven behavior:
  - plan-first decisions are based on complexity, not “PR vs not PR”.

## 6) Improve the harness when the harness fails

- Deviations are harness update events:
  - pause execution
  - capture **atomic** lessons (one per failure category)
  - apply the smallest prevention update (rules/docs/troubleshooting)
- A deviation includes:
  - user course correction,
  - blocked/failed subagent outcomes,
  - reviewer needs revision,
  - validation skips/waivers,
  - environment/tooling recoveries.
