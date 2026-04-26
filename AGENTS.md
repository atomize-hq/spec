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
- Follow the 5-step loop: `spec status .` to find invalid, stale, or missing-evidence units, `spec validate <unit-or-root> --format json` to read machine-parsable failures, edit the source `.unit.spec` or `.test.spec`, run `spec build <units-dir>` when you need to regenerate the shared output tree, then run `spec test <unit.unit.spec>` for exact-unit proof or `spec test <file.test.spec>` for a single molecule test and repeat until everything is green.
- Treat `spec validate --format json` as the primary feedback channel. Read `status`, `errors`, and `warnings` from stdout; this includes pre-validation workspace-config failures such as broken `[libraries]` entries. Each error object includes a stable `SPEC_*` machine code, the unit path when applicable, and any relevant structured fields such as `dep`, `field`, or `value`.
- A passport is the co-located `.spec.passport.json` record for a unit. It is "done" only when the unit validates, builds, tests, and has fresh passport evidence from `spec test`.
- A stale unit is marked with `~` in `spec status` when the current freshness projection says authored truth or backend execution changed since the last proof anchor. Treat stale as work to redo, not as success.
- For molecule tests, run `spec test path/to/file.test.spec` to execute only that interaction test and refresh only its co-located `.test.evidence.json` artifact.
- Semantic review for `kind:function` is bounded to the shipped family vocabulary, not arbitrary function understanding. The current supported family keys are `function.arithmetic_leaf.monotone_down_nonnegative.v1`, `function.arithmetic_leaf.monotone_up.v1`, and `function.wrapper.pipeline.v1`.
- In the canonical ecommerce example, `pricing/apply_discount` refreshes to `function.arithmetic_leaf.monotone_down_nonnegative.v1`, `pricing/apply_tax` refreshes to `function.arithmetic_leaf.monotone_up.v1`, and truthful `pricing/calculate_total` refreshes to `function.wrapper.pipeline.v1`. Unsupported near-miss function shapes remain additive-only and non-demoting under `unsupported.function.v1`.
- Only `spec test` refreshes semantic review truth. `spec build`, `spec generate`, `spec status`, and `spec export` only project stored truth and do not mint new supported-function semantic review.
- For seam kinds, keep shared semantics inside the seam container and nested behaviors: `kind: data` uses `data.fields`, `constructors`, and `methods`; `kind: sum` uses `sum.variants` and `methods`. Do not author top-level `contract`, `deps`, `imports`, or `body.rust` for seam kinds.
- Canonical M14 wedge loop:
  `cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/discount_policy.unit.spec --format json`
  `cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated`
  `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_policy.unit.spec`
  `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`
  `cargo run -p spec-cli -- status examples/ecommerce --format json`
  Single-file `spec test` uses an isolated internal output tree; do not pass `--output`.

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
| incomplete | ?      | Proof is incomplete (for example `unknown` evidence or an open marked-seam gate) |
| untested   | —      | No passport or no evidence field                      |
| valid      | ✓      | All checks pass                                       |

`reason` is present for non-valid, non-invalid rows. Exit code 1 for any non-valid unit or molecule test.

Marked seam unit rows may also include additive `escape_hatch_gate` metadata. In M14 the required surfaces are always `atom` and `molecule`, `atom` requires current local proof rather than historical passing evidence, and an open gate uses a stable reason like `missing required escape-hatch proof: atom, molecule`. Live `spec status` / `spec export` recompute the gate from current passport freshness and molecule evidence even if a stored passport also carries the gate field. An open gate demotes only an otherwise-`valid` marked seam to `incomplete`; a unit that is already `stale` remains `stale`.

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

For ordinary units, molecule test failure does **not** propagate to unit status. A failing
molecule test affects only the molecule test's own status row.

For marked seam units in M14, `escape_hatch_gate` is recomputed live for both `spec status` and
`spec export` from current passport freshness and molecule evidence. An open gate can demote an
otherwise-`valid` unit to `incomplete`, but it does not override a unit that is already `stale`.

This boundary prevents the "five units fail because one molecule test failed" ambiguity while still
letting marked seams require current `atom` and `molecule` proof. An ordinary unit can be `valid`
while a molecule test that covers it is failing — these are independent signals.
