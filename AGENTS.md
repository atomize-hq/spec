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

- Touch source specs, not generated output or observed artifacts. Edit `.unit.spec` or `.test.spec` files, then let `spec` regenerate `.rs` files, `.spec.passport.json`, and `.test.evidence.json` artifacts.
- Follow the 5-step loop: `spec status .` to find invalid, stale, or missing-evidence units, `spec validate [path] --format json` to read machine-parsable failures, edit the `.unit.spec`, run `spec build [path]`, then run `spec test [path]` and repeat until everything is green.
- Treat `spec validate --format json` as the primary feedback channel. Read `status`, `errors`, and `warnings` from stdout; this includes pre-validation workspace-config failures such as broken `[libraries]` entries. Each error object includes a stable `SPEC_*` machine code, the unit path when applicable, and any relevant structured fields such as `dep`, `field`, or `value`.
- A passport is the co-located `.spec.passport.json` record for a unit. It is "done" only when the unit validates, builds, tests, and has fresh passport evidence from `spec test`.
- A stale unit is marked with `~` in `spec status` when the passport's stored contract hash no longer matches the current spec contract. Treat stale as work to redo, not as success.
- For molecule tests, run `spec test path/to/file.test.spec` to execute only that interaction test and refresh only its co-located `.test.evidence.json` artifact.
- For `kind: data`, keep shared seam semantics in `data.fields`, `constructors`, and `methods`. Do not author top-level `contract`, `deps`, `imports`, or `body.rust`.
- Canonical M12 wedge loop:
  `cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/checkout_quote.unit.spec --format json`
  `cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated`
  `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/checkout_quote.unit.spec --output examples/ecommerce/src/generated`
  `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/checkout_flow.test.spec --output examples/ecommerce/src/generated`
  `cargo run -p spec-cli -- status examples/ecommerce --format json`

## Plan Artifact Workflow

Use this workflow when authoring or reviewing `.plan.spec` files.

- `.plan.spec` is authored intent plus acceptance targets. `computed_impact` is derived output only and is never authored in the source file.
- Run `spec plan validate <file> --format json` as the machine-readable source of truth for plan artifacts.
- `spec plan validate/export` accepts exactly one `.plan.spec` file, not a directory.
- Plan scope is local-library only in M10. `changes[].unit` and `acceptance.validate[]` must use local unit ids like `pricing/apply_tax`, not `shared::pricing/apply_tax`.
- `modify` and `remove` require an existing current-graph unit and derive current local-library impact.
- `add` requires a syntactically valid but currently missing local unit id and yields `computed_impact.unresolved[]` instead of fabricated impact.
- `computed_impact.status` is `complete` when every change has truthful current-graph impact and `partial` when any `add` remains unresolved.
- `spec plan export <file>` emits a dedicated plan bundle and does not change the `spec export` unit bundle contract.

## spec status health states (schema_version 3)

`spec status --format json` emits `schema_version: 3` with root-grouped output:

- `roots[]` is the authoritative result set.
- Each root contains `units[]` and `molecule_tests[]` as separate health planes.
- Top-level `units[]` remains as a flattened compatibility view.
- `loader_errors[]` remains top-level when discovery or library loading fails before rows can be computed.
- Zero discovered roots is non-green in both text and JSON mode.

Each unit or molecule test row has a `status` field:

| status     | symbol | meaning                                               |
|------------|--------|-------------------------------------------------------|
| invalid    | ✗      | Validation errors; see `errors[]`                     |
| failing    | ✗      | Build failed or a test result is `fail`               |
| stale      | ~      | Contract changed since last `spec test` run           |
| incomplete | ?      | Evidence exists but ≥1 test result is `unknown`       |
| untested   | —      | No passport or no evidence field                      |
| valid      | ✓      | All checks pass                                       |

`reason` is present for non-valid, non-invalid rows. Exit code 1 for any non-valid unit or molecule test.

Breaking changes from schema_version 1: `stale: bool` field removed; `reason: Option<String>`
added; new state values `failing`, `incomplete`, `untested` added (old values remain valid).

## Atom and Molecule Tests

**Atom tests** (`local_tests` in `.unit.spec`) test one unit's behavior in isolation. They are
generated inside the unit's `#[cfg(test)]` module. Each test has a single `expect` expression
that must evaluate to `true`.

**Molecule tests** (`.test.spec` files) test interactions between multiple units. They are
generated as `#[test]` functions in `molecule_tests.rs` per namespace. Each test declares which
units it `covers`, provides a full Rust block body, and records observed results in a co-located
`*.test.evidence.json` artifact after `spec test`.

**Boundary rule:** if a test needs to import more than one unit, it belongs in a `.test.spec`
file. If it tests only a single unit's behavior, it belongs in `local_tests`.

**body.rust is compiled Rust code.** Treat it with the same trust as any source file in your
project. The validator blocks `unsafe` blocks; all other Rust constructs are the author's
responsibility. Writing `include!`, `std::process::Command`, or `std::fs` calls in a molecule
test body is permitted but has the same implications as writing them in any other test file.

## Molecule Test Status Propagation

Molecule test failure does **not** propagate to unit status. A failing molecule test affects only
the molecule test's own status row. Unit status is determined solely by:

- Unit validation (schema + semantic)
- `spec test` evidence for that unit's local tests
- `contract_hash` staleness

This boundary prevents the "five units fail because one molecule test failed" ambiguity. A unit
can be `valid` while a molecule test that covers it is failing — these are independent signals.
