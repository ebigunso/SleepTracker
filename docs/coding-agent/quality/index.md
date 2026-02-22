# Quality Baselines Index

This index is the entry point for refactor quality baselines in this repository.
Use it to select the right level of guidance before planning, implementation, or review.

---

## Baseline Layers

1) High-level principles (broad, stable)
- `backend-principles.md`
- `frontend-principles.md`

2) Architecture gates (cross-layer, evidence-oriented)
- `gates-architecture.md`

3) Language/framework gates (implementation-specific)
- `gates-rust.md`
- `gates-svelte-typescript.md`

4) Review scoring rubric (decision aid)
- `QUALITY_SCORE.md`

---

## How To Use Without Duplication

- Start with principles to determine what must stay behavior-safe.
- Apply architecture gates when boundaries/contracts/security/domain ownership may change.
- Apply language/framework gates for stack-specific command and implementation checks.
- Use `QUALITY_SCORE.md` to summarize review quality; do not replace required validations with scoring.

Keep this separation explicit:
- Principles answer **what must remain true**.
- Gates answer **what must be verified**.
- The rubric answers **how strong the overall change quality is**.

---

## Validation Alignment

- Required validation commands and path-based requirements come from:
  - `docs/coding-agent/references/validation.md`
- UI/E2E evidence requirements come from:
  - `docs/coding-agent/references/ui-e2e.md`

This index complements those references and does not override them.

### Precedence / Interpretation Rule

If wording in any quality gate document appears stricter or looser than required validation mapping:

1) Treat `docs/coding-agent/references/validation.md` as canonical for what is strictly required.
2) Treat gate docs (`gates-*.md`) as additional recommended/conditional quality guidance unless they explicitly restate canonical required mapping.
3) Keep command names and path scope consistent with canonical mapping to avoid ambiguity.
