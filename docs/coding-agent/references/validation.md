# Validation Mapping (Repo-Specific)

This doc maps *what changed* → *what validations are required*.

The goal is to remove ambiguity so agents do not treat validation as optional.

---

## Canonical local commands

Fill in the canonical commands used in this repo.

- Unit tests:
- Lint:
- Typecheck:
- Build:
- Formatting:
- E2E (if any):

---

## CI workflow pointers

List the CI workflow files that matter for validation.

- `.github/workflows/<name>.yml` — purpose:
- `.github/workflows/<name>.yml` — purpose:

If the repo uses path filters in CI, document the mapping here.

---

## Path → required validations mapping

Fill in a table like this. Keep it simple and actionable.

| Changed paths (glob) | Required validations (commands/manual) | Notes |
|---|---|---|
| `src/**` | `<unit command>`, `<lint command>`, `<typecheck>` | |
| `docs/**` | reviewer check for broken links / clarity | |
| `ui/**` | reviewer E2E via `playwright-cli` + screenshots | |

---

## Validation evidence expectations

- If a validation is required and cannot be run:
  - do not mark done
  - record why and what remains
  - request explicit waiver if needed

- If UI flows are impacted:
  - collect E2E/visual evidence under `.playwright-cli/`
  - reference artifacts in the review output
