## Skill routing

When the user's request matches an available skill, ALWAYS invoke it using the Skill
tool as your FIRST action. Do NOT answer directly, do NOT use other tools first.
The skill has specialized workflows that produce better results than ad-hoc answers.

Key routing rules:
- Product ideas, "is this worth building", brainstorming → invoke office-hours
- Bugs, errors, "why is this broken", 500 errors → invoke investigate
- Ship, deploy, push, create PR → invoke ship
- QA, test the site, find bugs → invoke qa
- Code review, check my diff → invoke review
- Update docs after shipping → invoke document-release
- Weekly retro → invoke retro
- Design system, brand → invoke design-consultation
- Visual audit, design polish → invoke design-review
- Architecture review → invoke plan-eng-review

## spec Agent Workflow

Use this workflow when editing `.unit.spec` files or responding to validation and test feedback.

- Touch source specs, not generated output or passports. Edit `.unit.spec` files, then let `spec` regenerate `.rs` files and `.spec.passport.json` artifacts.
- Follow the 5-step loop: `spec status .` to find invalid, stale, or missing-evidence units, `spec validate [path] --format json` to read machine-parsable failures, edit the `.unit.spec`, run `spec build [path]`, then run `spec test [path]` and repeat until everything is green.
- Treat `spec validate --format json` as the primary feedback channel. Read `status`, `errors`, and `warnings` from stdout; each error object includes a stable `SPEC_*` machine code, the unit path, and any relevant structured fields such as `dep`, `field`, or `value`.
- A passport is the co-located `.spec.passport.json` record for a unit. It is "done" only when the unit validates, builds, tests, and has fresh passport evidence from `spec test`.
- A stale unit is marked with `~` in `spec status` when the passport's stored contract hash no longer matches the current spec contract. Treat stale as work to redo, not as success.

## spec status health states (schema_version 2)

`spec status --format json` emits `schema_version: 2`. Each unit has a `status` field:

| status     | symbol | meaning                                               |
|------------|--------|-------------------------------------------------------|
| invalid    | ✗      | Validation errors; see `errors[]`                     |
| failing    | ✗      | Build failed or a test result is `fail`               |
| stale      | ~      | Contract changed since last `spec test` run           |
| incomplete | ?      | Evidence exists but ≥1 test result is `unknown`       |
| untested   | —      | No passport or no evidence field                      |
| valid      | ✓      | All checks pass                                       |

`reason` is present for non-valid, non-invalid units. Exit code 1 for any non-valid unit.

Breaking changes from schema_version 1: `stale: bool` field removed; `reason: Option<String>`
added; new state values `failing`, `incomplete`, `untested` added (old values remain valid).
