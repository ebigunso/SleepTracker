# Validation Mapping (Repo-Specific)

This doc maps *what changed* → *what validations are required*.

The goal is to remove ambiguity so agents do not treat validation as optional.

---

## Canonical local commands

- Unit tests: `cd sleep-ui && npm run test:unit`
- Lint: (no dedicated frontend lint script currently)
- Typecheck: `cd sleep-ui && npm run check`
- Build: `cd sleep-ui && npm run build`
- Formatting: `cargo fmt --all`
- E2E (safe default): `cd sleep-ui && npm run test:e2e`

---

## CI workflow pointers

- `.github/workflows/ci-frontend.yml` — frontend check/unit/build validation for UI changes

If the repo uses path filters in CI, document the mapping here.

---

## Path → required validations mapping

Fill in a table like this. Keep it simple and actionable.

| Changed paths (glob) | Required validations (commands/manual) | Notes |
|---|---|---|
| `sleep-ui/src/**` | `cd sleep-ui && npm run check`, `cd sleep-ui && npm run test:unit` | Required for UI code changes |
| `sleep-ui/tests/**` | `cd sleep-ui && npm run test:e2e` (or targeted equivalent + rationale) | Must use isolated E2E harness defaults |
| `sleep-api/src/**` (E2E harness/runtime touched) | `cargo test -p sleep-api` | Required when API startup/config is changed |
| `docs/**` | Reviewer clarity check + command consistency | Ensure docs match runnable commands |

---

## Validation evidence expectations

- If a validation is required and cannot be run:
  - do not mark done
  - record why and what remains
  - request explicit waiver if needed

- If UI flows are impacted:
  - collect E2E/visual evidence under `.playwright-cli/`
  - reference artifacts in the review output

- If E2E harness/config is impacted:
  - verify non-local target guardrails fail fast by default
  - verify isolated DB path is used for default `npm run test:e2e`
