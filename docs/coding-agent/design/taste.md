# Taste & Review Expectations

This doc encodes “taste” as explicit review heuristics so quality is consistent.

---

## Maintainability heuristics

- Prefer reusing existing types/interfaces over creating near-duplicates.
- Avoid unused config fields and “dead switches”; if a field is introduced, show where it is read.
- Simplify redundant branches and duplicated logic.
- Keep error handling consistent with existing patterns (error types, messages, logs).
- Avoid broad refactors unless they are the task’s explicit goal.

---

## Change discipline

- Keep diffs minimal and focused (avoid formatting-only churn unless explicitly required).
- Preserve existing style and conventions unless there is a stated migration.

---

## UI/UX heuristics (when applicable)

- Changes that affect user flows require evidence:
  - screenshots at key states
  - console error scan
  - network failure scan
- Favor stable selectors and explicit test ids if E2E will rely on them.

---

## Documentation heuristics (when applicable)

- Prefer short docs with links over large single docs.
- Keep terminology consistent.
- Ensure “how to run / validate / verify” docs are updated when workflows change.

---

## “Smell tests” (quick checks)

If any of these are true, consider re-planning or adding validation:

- You cannot clearly state how to validate the change.
- The change touches multiple unrelated areas without a strong reason.
- You introduced a new type that duplicates an existing one.
- You introduced a config field but did not wire it into behavior.
- You changed UI behavior but do not have concrete visual evidence.
