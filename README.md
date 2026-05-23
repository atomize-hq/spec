# spec

`spec` is a Rust workspace for authoring semantic units in `*.unit.spec` files, validating them with JSON Schema, normalizing them into internal IR, and generating readable Rust source.

## Workflow

1. Author a `.unit.spec` file.
2. Validate it with the CLI.
3. Build: `spec build` validates, generates, and compiles in one step.
4. Test: `spec test` runs the full pipeline and writes observed evidence to unit passports plus molecule evidence artifacts.
5. Export: `spec export` emits a machine-readable JSON bundle for downstream tooling.
6. Plan: `spec plan validate` and `spec plan export` turn authored change intent into truthful local-library impact data.

## Workspace

- `spec-core`: parsing, validation, normalization, generation, pipeline, and export primitives
- `spec-cli`: CLI for `validate`, `generate`, `build`, `test`, `export`, and `plan`
- `examples/ecommerce`: a small single-library example with pricing units
- `examples/shared-spec`, `examples/shared-crate`, `examples/crosslib-app`: a direct sibling-library example for `[libraries]` and `shared::...` deps

## Project docs

- [`CHANGELOG.md`](CHANGELOG.md): shipped release history
- [`PLAN.md`](PLAN.md): active implementation roadmap, with shipped milestone context below the current contract
- [`DECISIONS.md`](DECISIONS.md): project-level decisions that stay stable across releases
- [`TODOS.md`](TODOS.md): backlog and follow-up inventory
- [`AGENTS.md`](AGENTS.md): agent workflow and machine-readable `spec` authoring loop
- [`CLAUDE.md`](CLAUDE.md): lightweight routing rules for Claude/Codex sessions in this repo
- [`docs/rust_v1_contract_stack.md`](docs/rust_v1_contract_stack.md): Rust V1 contract-stack index and the frozen I3.5 command wall
- [`examples/ecommerce/README.md`](examples/ecommerce/README.md): local example walkthrough
- [`examples/crosslib-app/README.md`](examples/crosslib-app/README.md): cross-library example walkthrough
- [`docs/north_star_v0.2.md`](docs/north_star_v0.2.md), [`docs/high_level_technical_architecture_v0.2.md`](docs/high_level_technical_architecture_v0.2.md), and [`docs/roadmap_and_release_shape_v0.1.md`](docs/roadmap_and_release_shape_v0.1.md): historical design context from the pre-ship planning phase
- [`.implemented/`](.implemented/): archived milestone release plans and early design artifacts

## Quickstart

```bash
cargo test
cargo run -p spec-cli -- validate examples/ecommerce/units
cargo run -p spec-cli -- generate examples/ecommerce/units
```

After installation, the binary is `spec`:

```bash
cargo install spec-cli
spec validate examples/ecommerce/units
spec generate examples/ecommerce/units
```

## Benchmarks (M68)

Rust V1 benchmark mechanics are repo-root read surfaces, not authored spec truth. The authored benchmark registry lives in `benchmarks/labels.json`; readability review anchors live in `benchmarks/reviews/*.readability.review.json`; and `spec benchmark snapshot <benchmark-id>` writes derived snapshots under `benchmarks/snapshots/`.

The frozen I8 proof wall after I7 is:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- status . --format json
```

Repo-root surfaces remain available, but they have a narrower contract:

- `cargo run -p spec-cli -- status . --format json` is a supported broad-scope inventory surface only. Treat its scope authority as `inventory_only`, not as the fresh-clone proof default.
- `cargo run -p spec-cli -- export .` is unsupported for this workspace shape and must fail with `SPEC_UNSUPPORTED_SCOPE` instead of emitting a fake aggregate bundle.

The seeded roster is:

- `BENCH-ECOM`: active positive benchmark rooted at `examples/ecommerce/units`
- `BENCH-CROSSLIB`: active companion-negative benchmark rooted at `examples/crosslib-app/units`
- `BENCH-SERVICE`: active positive benchmark rooted at `examples/service/units`

The ratified Rust V1 line after I7 is:

- in: synchronous supported function families plus plain data and sum seams, proven by `BENCH-ECOM` and `BENCH-SERVICE`
- deferred to `V1.1`: bounded generics
- deferred to `V1.1`: async flows, runtime adapters, and IO-owned boundaries
- preserved negative wall: `BENCH-CROSSLIB` stays visible but never counts as positive supported credit

Path scope is intentional:

- Exact benchmark-root scopes are the proof-authoritative default and emit full projections with digests, benchmark status, gate status, and readability review state.
- Broad repo-root scope still emits a full projection, but only as `inventory_only` reporting.
- Namespace and single-file scopes emit partial projections only. Partial scope never emits positive supported credit, even when the covered carrier is individually `valid`.
- Companion-negative cases stay visible but never count as positive supported credit.

Writer versus reader wall:

- `spec status` and `spec export` only project benchmark truth; they do not write passports, molecule evidence, or benchmark proof.
- `spec benchmark snapshot <id>` writes only snapshot artifacts under `benchmarks/snapshots/`.
- `.spec.passport.json` and `*.test.evidence.json` remain the read-only proof inputs for benchmark accounting during M68.

`spec export` now emits `schema_version: 4` and additively includes top-level `benchmarks[]` under the same contract as `spec status` for the same scope.

## Bounded TypeScript lane (M61)

`spec` now exposes one bounded TypeScript execution lane. It is intentionally narrow:

M61 extends the bounded Bun-backed TypeScript lane to recursive local-plus-cross-library closure across the already-supported function families, while preserving family-specific direct-dep contracts, additive proof, atom-only execution, and the broader bans on arbitrary 4+ topology parity and molecule TypeScript execution.

- Bun is the only TypeScript prerequisite. The lane shells out to `bun`; no alternate Node, npm, or tsx contract is supported.
- Eligible roots are `kind: function` specs whose reachable closure resolves from the loaded unit set, classifies to already-supported semantic-review function families, and authors non-empty `body.typescript` at every reachable unit.
- Closure collection is semantic-review-driven and dep-driven. It recurses across same-tree and direct sibling-library deps, dedupes repeated local-plus-cross-library subgraphs, and excludes unrelated loaded units from the generated TypeScript tree.
- The preserved portability exceptions remain in place for direct cross-library roots. A `function.arithmetic_leaf.monotone_up.v1` root may still use exactly one direct cross-library helper dep that classifies to `function.helper.identity_passthrough.v1`, authors non-empty `body.typescript`, and resolves from the loaded unit set and generated tree.
- The preserved direct cross-library wrapper lane remains frozen to the `function.wrapper.pipeline.v1` family with the direct dep tuple `function.arithmetic_leaf.monotone_down_nonnegative.v1` then `function.arithmetic_leaf.monotone_up.v1`.
- The preserved direct cross-library chain3 lane remains frozen to the `function.wrapper.pipeline.chain3.v1` family with the direct dep tuple `function.wrapper.pipeline.v1` or `function.wrapper.pipeline.chain3.v1` in slot 1, then `function.arithmetic_leaf.monotone_up.v1`, then `function.arithmetic_leaf.monotone_down_nonnegative.v1`.
- Family-specific direct dep contracts remain frozen even when closure collection recurses across library boundaries. The M60 required-arg wrapper family stays bounded to its current supported shape rather than widening the lane to arbitrary wrapper argument forms.
- Rust remains the default target. TypeScript proof is additive only; it writes `target_proofs.typescript` without replacing the Rust proof surface.
- TypeScript execution is atom-only. Only `local_tests` run in this lane; `.test.spec` molecule tests remain unsupported for `--target-language typescript` and fail before Bun runs.
- The accepted `local_tests.expect` grammar is deliberately small: `<current_unit>(Decimal::new(int, scale), ...) == Decimal::new(int, scale)`. The left-hand side must be a direct call to the current unit, and both the arguments and expected value must use integer-literal `Decimal::new(...)` forms.

Commands available in this lane:

```bash
spec generate <units-dir> --target-language typescript
spec build <units-dir> --target-language typescript
spec test <path> --target-language typescript
spec status <unit-or-root> --target-language typescript
```

`spec status <unit-or-root> --target-language typescript` reports target-specific proof only. In M61, a root-level status over a mixed example like `examples/ecommerce` or `examples/crosslib-app` may stay non-green unless every unit in scope is either freshly proven in the bounded recursive local-plus-cross-library lane, covered by the preserved portability contracts, or truthfully remains outside those lanes.

The generated helper filenames remain frozen in M61:

- `__spec_ts/runtime.ts`
- `__spec_ts/build_entry.ts`
- `__spec_ts/local_tests.ts`

Commands and surfaces that do not widen for M61:

- `spec validate` does not accept `--target-language`
- `spec export` does not accept `--target-language`
- The broader TypeScript oceans still explicitly deferred are arbitrary authored 4+ direct-dep topology parity, new semantic-family promotion, molecule TypeScript execution, and seam-kind TypeScript execution.

## Spec format

Each unit is a YAML document with these common required fields:

- `id`: hierarchical unit id like `pricing/apply_discount`
- `kind`: authored unit shape, currently `function`, `data`, or `sum`
- `intent.why`: why the unit exists

Kind-specific authored fields:

| `kind` | Required authored fields | Optional authored fields | Forbidden top-level fields |
| --- | --- | --- | --- |
| `function` | `contract`, `body.rust` | `deps`, `imports`, `local_tests`, `links` | none |
| `data` | `data.fields`, one or more `constructors`, one or more `methods` | `local_tests`, `links`, `backends.rust` | `contract`, `deps`, `imports`, `body.rust` |
| `sum` | `sum.variants`, one or more `methods` | `local_tests`, `links`, `backends.rust` | `contract`, `deps`, `imports`, `body.rust`, `constructors` |

For `kind: function`, `spec` generates the complete `pub fn` signature from `contract.inputs` and `contract.returns`. A minimal unit with a contract looks like:

```yaml
id: pricing/apply_tax
kind: function
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }
```

This generates:

```rust
pub fn apply_tax(subtotal: Decimal, rate: Decimal) -> Decimal {
    let taxed = subtotal + subtotal * rate;
    round(taxed)
}
```

For `kind: data`, one `.unit.spec` file authors a top-level data seam with shared fields plus one or more nested constructors and one or more nested methods. A minimal seam based on the canonical M50 `pricing/pricing_quote` example looks like:

```yaml
id: pricing/pricing_quote
kind: data
intent:
  why: Quote a checkout total from subtotal plus discount and tax rates.
data:
  fields:
    subtotal:
      type: rust_decimal::Decimal
    discount_rate:
      type: rust_decimal::Decimal
    tax_rate:
      type: rust_decimal::Decimal
constructors:
  - id: new
    intent:
      why: Create a quote from explicit subtotal and rates.
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
        discount_rate: rust_decimal::Decimal
        tax_rate: rust_decimal::Decimal
    initializes:
      subtotal: subtotal
      discount_rate: discount_rate
      tax_rate: tax_rate
methods:
  - id: discounted_subtotal
    intent:
      why: Return the discounted subtotal before tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/apply_discount
    lowering:
      rust:
        body: |
          {
              apply_discount(self.subtotal, self.discount_rate)
          }
  - id: total
    intent:
      why: Return the final checkout total after discount and tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              apply_tax(self.discounted_subtotal(), self.tax_rate)
          }
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
```

`kind: data` keeps shared semantics in `data.fields`, `constructors`, and `methods`. Rust-specific lowering stays inside `methods[].lowering.rust.body` and `backends.rust`.

`kind: sum` follows the same seam boundary: keep shared semantics in `sum.variants` and `methods`. Enum cases are authored via `sum.variants`, not `constructors`. Rust-specific lowering stays inside `methods[].lowering.rust.body` and `backends.rust`.

## Migrating from 0.2.x

In 0.2.x, `body.rust` contained the full function declaration:

```yaml
# 0.2.x format (no longer valid)
body:
  rust: |
    pub fn apply_tax(subtotal: Decimal, rate: Decimal) -> Decimal {
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }
```

In 0.3.0, strip the `pub fn name(params) -> ReturnType` line and keep only the body block. Move the parameter names and types into `contract.inputs`, and the return type into `contract.returns`:

```yaml
# 0.3.0 format
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
body:
  rust: |
    {
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }
```

Running `spec validate` on a 0.2.x unit will emit a clear migration error pointing to the file.

## Example

The ecommerce example demonstrates the canonical M13 migration wedge alongside the existing pricing units, three molecule tests, one checked-in plan artifact, and tracked molecule evidence so the shipped example stays green on a fresh clone:

- `money/round`
- `pricing/apply_discount`
- `pricing/apply_tax`
- `pricing/calculate_total`
- `pricing/discount_strategy` (`kind: sum`)
- `pricing/pricing_quote` (`kind: data`)
- `pricing/checkout_flow`
- `pricing/discount_strategy_checkout_flow`
- `pricing/discount_plus_tax`
- `plans/refactors/checkout-tax-refactor.plan.spec`
- `examples/ecommerce/src/raw_baseline/pricing/discount_strategy.rs` (hand-written Rust baseline for the canonical pricing sum seam)
- `examples/ecommerce/src/raw_baseline/pricing/pricing_quote.rs` (hand-written Rust baseline for the canonical pricing data seam)

Recorded adversarial calibration for the canonical wedge is locked: `pricing/discount_strategy` scored `19`, `pricing/pricing_quote` scored `16`, and `pricing/discount_plus_tax` scored `14`. Canonical wedge remains `pricing/discount_strategy`.

The example crate is intentionally minimal. It provides a realistic place to keep unit specs, hand-written Rust baselines for the M12 and M13 migration wedges, and a Rust project scaffold that can host generated output. The checked-in `pricing/*.test.evidence.json` files are generated artifacts for this canonical example, not hand-authored source.

## Commands

```bash
spec validate <unit-or-root>              # directory or single .unit.spec: schema + semantic validation
spec validate <unit-or-root> --no-strict  # downgrade missing deps to warnings
spec validate <unit-or-root> --format json # machine-readable JSON output for agents
spec generate <units-dir>                 # directory only: emit .rs files (default: {crate_root}/src/generated)
spec generate <units-dir> --output <dir>  # explicit output directory

spec build <units-dir>                    # directory only: validate → generate → cargo build
spec build <units-dir> --output <dir>     # explicit output directory
spec test  <path>                         # directory, single .unit.spec, or single .test.spec
spec test  <path> --output <dir>          # explicit output directory for directory-scoped test runs
spec test  <path/to/unit.unit.spec>       # scope to a single unit (filter by module path)
spec test  <path/to/test.test.spec>       # run one molecule test and write only its .test.evidence.json

spec status <unit-or-root>                # directory or single .unit.spec: per-root unit and molecule-test health
spec status <unit-or-root> --format json  # machine-readable status for agents

spec export <unit-or-root>                # directory or single .unit.spec: emit JSON bundle to stdout
spec export <unit-or-root> --output <file> # write JSON bundle to file

spec plan validate <file>                 # validate one .plan.spec file and compute local impact
spec plan validate <file> --format json   # machine-readable plan validation + computed impact
spec plan export <file>                   # emit dedicated plan bundle to stdout
spec plan export <file> --output <file>   # write dedicated plan bundle to file
```

Canonical ecommerce example loop from the repo root:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/discount_strategy.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
```

`validate` checks schema and semantic rules. `--no-strict` downgrades missing internal deps to warnings for validation only. `generate` always remains strict, is directory-scoped only, and emits `.rs` files under the output directory while managing `mod.rs` files plus the `.spec-generated` safety marker.

`spec build` and directory-scoped `spec test` wrap the full pipeline so you can validate, generate, and compile in one step. `spec build` is directory-scoped only. `spec test` updates each unit's `.spec.passport.json` with observed local-test evidence and writes co-located `*.test.evidence.json` artifacts for molecule tests. Passports persist a `freshness_anchor` snapshot as the proof anchor from the last unit test run, while `freshness` is a live projection against the current spec. Marked seam passports may also carry additive `escape_hatch_gate` metadata. In M14 that gate requires both `atom` and `molecule` proof; `atom` is present only when the authored local tests pass and the seam's projected freshness is still current, and an open gate uses a stable reason like `missing required escape-hatch proof: molecule`.

Semantic review for `kind:function` is bounded to a small shipped family vocabulary, not arbitrary function understanding. The current supported function family keys are `function.helper.identity_passthrough.v1`, `function.arithmetic_leaf.monotone_down_nonnegative.v1`, `function.arithmetic_leaf.monotone_up.v1`, `function.wrapper.pipeline.normalized_required_arg.v1`, `function.wrapper.pipeline.v1`, and `function.wrapper.pipeline.chain3.v1`. In the canonical ecommerce and TypeScript fixture flows, `money/round`, `pricing/apply_discount`, `pricing/apply_tax`, `pricing/calculate_total`, and the bounded chain3 forms refresh to those family keys when their authored and executable shapes fit honestly. M60 adds one supported wrapper family for apply_tax(discounted, tax_rate.max(Decimal::ZERO)); broader required-argument expressions remain unsupported.

M20 also makes unsupported-function truth explicit. The public fields are exactly `semantic_review.support_status`, `semantic_review.unsupported_reason_codes`, and `semantic_review.rewrite_hints`. New supported reviews write `support_status: supported`; unsupported function reviews write `support_status: unsupported`. Consumers should branch on `semantic_review.support_status == "unsupported"` instead of inferring unsupported state from `verdict` or `evaluator_scope`, though legacy reviews without `support_status` still fall back to `evaluator_scope` plus `unsupported.*.v1` compatibility-key inference.

Unsupported near-miss function shapes remain additive-only, health-neutral, and keyed as `unsupported.function.v1`. Only `spec test` refreshes semantic review truth. `spec build`, `spec generate`, `spec status`, and `spec export` only project stored truth; they do not mint new supported-function semantic review. Fresh unsupported function proof is preserved on read-side surfaces such as `spec status` and `spec export`, while stale unsupported function proof is dropped there and only the underlying freshness or stale health remains visible.

When you pass a single `.unit.spec` file to `spec validate`, `spec test`, or `spec export`, the CLI stays scoped to that exact unit. When you pass a single `.test.spec` file to `spec test`, the CLI runs only that molecule test and writes only that test's evidence artifact. Sibling `.test.spec` files are otherwise loaded for directory invocations.

Single-file `spec test` runs generate Rust into an isolated internal build surface. They do not rewrite your checked-out `src/generated/` tree, and `--output` is accepted only for directory-scoped `test` runs.

`spec export` emits a machine-readable JSON bundle containing all units, passports, dependency graph edges, and warnings for any passports that could not be read. Stored `escape_hatch_gate` metadata may appear in passports, but live `spec status` and `spec export` recompute the gate from current passport freshness and molecule evidence rather than trusting the stored field blindly. Canonical seam `proof_coverage` also adds the `molecule` surface only when a covering molecule evidence artifact is both current and passing.

`spec plan validate` and `spec plan export` are single-file commands. They accept exactly one `.plan.spec` file, resolve the enclosing library root by walking up to the directory that owns `units/`, validate the authored change set against the current graph, and compute advisory impact for the current local library only. `add` changes stay truthful by contributing `unresolved[]` entries instead of fabricated impact. The repo ships a checked-in example at `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec`.

The `--output` path for `generate`/`build`/`test` must resolve to a directory inside your project root. Paths that escape the project root are rejected as a safety guardrail to prevent accidental deletion of files outside the project.

**Nextest:** `spec test` parses standard `cargo test` output format only. `cargo nextest` uses a different output format and is not supported. Running `spec test` in a project configured for nextest will produce `status: "unknown"` for all local tests. Use standard `cargo test`.

For both `.unit.spec` and directory-scoped `.test.spec` validation, the path segment `molecule_tests` is reserved. Molecule tests generate `molecule_tests.rs` per namespace, so allowing that literal segment in an authored spec ID would create module/file collisions in generated output.

## AI-Native Usage

`spec` is especially useful when an AI agent is the one making the edit loop. The toolchain gives the agent a structured contract to follow, a machine-readable validation result to fix against, and a passport plus molecule-evidence trail that records what was actually observed to pass. In this repo, the canonical ecommerce example ships both unit passports and molecule evidence so `cargo run -p spec-cli -- status examples/ecommerce/units --format json` is the trustworthy fresh-clone starting point. Repo-root `status . --format json` remains useful when you want broad inventory, but it is intentionally `inventory_only`.

The loop is simple: inspect status, validate the exact unit, edit the `.unit.spec`, build to catch Rust-level issues, then test to write fresh evidence. Single-file `validate` and `test` stay on that unit and do not pull sibling molecule tests into the run.

```bash
spec validate examples/ecommerce/units --format json
```

```json
{
  "schema_version": 3,
  "status": "invalid",
  "errors": [
    {
      "unit": "shipping/calculate",
      "code": "SPEC_MISSING_DEP",
      "dep": "currency/convert",
      "path": "units/shipping/calculate.unit.spec"
    }
  ],
  "warnings": []
}
```

That JSON form is meant for agents: parse `status`, `errors`, and `warnings` instead of scraping terminal prose. Pre-validation workspace-config failures, including broken `[libraries]` entries, also stay in this JSON contract for `validate --format json`.
`warnings` is a structured array in `schema_version: 3`, so machine consumers can key off stable warning codes instead of parsing display text.

For molecule tests, `covers` declares the semantic exercised-unit set. Add explicit `.test.spec` `imports` when the Rust body needs names in scope. If `imports` is omitted, validation emits a deprecation warning and generation temporarily falls back to cover-derived implicit imports for backward compatibility.

For plan artifacts, the machine-readable loop is:

```bash
spec plan validate path/to/checkout-tax-refactor.plan.spec --format json
```

Checked-in example:

```bash
spec plan validate examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec --format json
```

Valid plan responses reuse the same top-level envelope and add `plan_id` plus derived `computed_impact`:

```json
{
  "schema_version": 3,
  "status": "valid",
  "errors": [],
  "warnings": [],
  "plan_id": "checkout-tax-refactor",
  "computed_impact": {
    "status": "partial",
    "units": ["pricing/apply_tax", "pricing/calculate_total"],
    "molecule_tests": ["pricing/checkout_flow", "pricing/discount_plus_tax"],
    "unresolved": [
      {
        "unit": "pricing/tiered_rate",
        "action": "add",
        "reason": "current graph has no node for action=add"
      }
    ]
  }
}
```

`spec plan export` emits a dedicated bundle with `{schema_version, spec_version, exported_at, plan, computed_impact, warnings}` and does not change the existing `spec export` bundle contract.

`spec status` uses simple symbols so you can scan a whole tree quickly. Any unit or molecule test whose status is not `valid` exits with code `1`.

`spec status --format json` emits `schema_version: 3` and groups results by discovered library root:

```json
{
  "schema_version": 3,
  "roots": [
    {
      "root": ".",
      "units": [{ "id": "pricing/apply_tax", "status": "valid", "errors": [] }],
      "molecule_tests": [{ "id": "pricing/checkout_flow", "status": "untested", "reason": "no molecule evidence", "errors": [] }]
    }
  ],
  "units": [{ "id": "pricing/apply_tax", "status": "valid", "errors": [] }]
}
```

`roots[].units` and `roots[].molecule_tests` are the authoritative per-root planes. The top-level `units` array remains as a flattened compatibility view for existing consumers. Workspace-config failures that happen before any row can be computed surface as top-level `loader_errors` entries instead of raw stderr text, and zero discovered roots is a non-green status result.

For marked seam units, `status` rows may also include additive `escape_hatch_gate` metadata. In M14 the required surfaces are always `["atom", "molecule"]`, and live `spec status` / `spec export` recompute that gate from current passport freshness and molecule evidence. An open gate can demote an otherwise-`valid` marked seam to `incomplete`, but a seam that is already `stale` remains `stale`.

- `✓` valid
- `✗` invalid or failing
- `~` stale
- `?` incomplete (for example `unknown` evidence or an open marked-seam gate)
- `—` untested

Use the companion skill at [`.claude/skills/spec/SKILL.md`](.claude/skills/spec/SKILL.md) when you want the full workflow spelled out for an AI coding session.

## Validation error codes

`spec validate --format json` returns error objects with a `code` field. These are the recognized codes returned by the current CLI:

| Code | Description |
|------|-------------|
| `SPEC_IO` | Filesystem I/O error |
| `SPEC_INVALID_UTF8` | File is not valid UTF-8 |
| `SPEC_YAML_PARSE` | YAML syntax error in the unit file |
| `SPEC_JSON` | JSON serialization/deserialization error |
| `SPEC_SCHEMA_VALIDATION` | Unit file failed JSON Schema validation |
| `SPEC_SEMANTIC_VALIDATION` | Unit passed schema but failed a semantic rule |
| `SPEC_RUST_KEYWORD` | An `id` segment is a Rust reserved keyword |
| `SPEC_DUPLICATE_ID` | Two unit files share the same `id` |
| `SPEC_DEP_COLLISION` | A dep collides with another generated callable name, including another dep or the owning unit function |
| `SPEC_MISSING_DEP` | A declared dep has no matching unit in the spec set |
| `SPEC_UNKNOWN_LIBRARY_NAMESPACE` | A dep references a library alias that is not configured in `[libraries]` |
| `SPEC_CROSS_LIBRARY_DEP_NOT_FOUND` | A cross-library dep has no matching unit in the resolved library set |
| `SPEC_LIBRARY_CRATE_ALIAS_MISSING` | The root crate is missing the Cargo dependency alias required by a cross-library dep |
| `SPEC_LIBRARY_PATH_NOT_FOUND` | A `[libraries]` entry points to a path that does not exist |
| `SPEC_LIBRARY_OUT_OF_ROOT` | A `[libraries]` entry resolves outside the repo root |
| `SPEC_LIBRARY_ALIAS_SELF` | A `[libraries]` entry points back to the invoking library root |
| `SPEC_DUPLICATE_LIBRARY_ROOT` | Two `[libraries]` aliases resolve to the same canonical root |
| `SPEC_CYCLIC_DEP` | Units form a dependency cycle |
| `SPEC_CROSS_LIBRARY_CYCLE` | Units form a dependency cycle across library boundaries |
| `SPEC_USE_STATEMENT_IN_BODY` | `body.rust` contains a `use` statement — move it to `imports` or `deps` |
| `SPEC_BODY_RUST_MUST_BE_BLOCK` | `body.rust` failed to parse as a Rust block expression |
| `SPEC_BODY_RUST_LOOKS_LIKE_FN_DECLARATION` | `body.rust` contains the full `pub fn` signature — keep only the body block (see migration guide) |
| `SPEC_LOCAL_TEST_EXPECT_NOT_EXPR` | A `local_tests[].expect` value is not a valid Rust expression |
| `SPEC_DUPLICATE_LOCAL_TEST_ID` | Two local tests in the same unit share the same `id` |
| `SPEC_CONTRACT_TYPE_INVALID` | A `contract.inputs` or `contract.returns` type is not valid Rust |
| `SPEC_CONTRACT_INPUT_NAME_INVALID` | A `contract.inputs` key is not a valid Rust identifier |
| `SPEC_TRAVERSAL` | Error walking the units directory tree |
| `SPEC_GENERATOR` | Code generation failure |
| `SPEC_OUTPUT_DIR` | Output directory creation or safety check failed |
| `SPEC_MISSING_MARKER` | Output dir lacks the `.spec-generated` marker — refusing to clean |
| `SPEC_MOLECULE_CROSS_LIBRARY_COVERS_UNSUPPORTED` | A molecule test `covers` entry references another library, which remains unsupported in M9 |
| `SPEC_RESERVED_UNIT_NAME` | A slash-delimited spec `id` contains a reserved segment such as `molecule_tests` |
| `SPEC_PLAN_DIRECTORY_INPUT` | `spec plan validate/export` received a directory instead of a single `.plan.spec` file |
| `SPEC_PLAN_OUTSIDE_LIBRARY_ROOT` | The `.plan.spec` file does not live under any resolved library root |
| `SPEC_PLAN_SYMLINK_ESCAPE` | The `.plan.spec` path escapes the repo/library root through a symlink |
| `SPEC_PLAN_CROSS_LIBRARY_UNIT` | A plan authored a cross-library `changes[].unit` or acceptance unit ref, which M10 forbids |
| `SPEC_PLAN_DUPLICATE_CHANGE_UNIT` | The same `changes[].unit` appears more than once in one plan file |
| `SPEC_PLAN_UNIT_MISSING_FOR_ACTION` | `modify` or `remove` references a unit that does not exist in the current graph |
| `SPEC_PLAN_UNIT_ALREADY_EXISTS_FOR_ADD` | `add` references a unit that already exists in the current graph |
| `SPEC_PLAN_MOLECULE_TEST_NOT_FOUND` | A plan acceptance target references a molecule test id that does not exist in the current library |

## Consuming Generated Code

Generated units import internal deps with `use crate::...` paths. The consuming crate must
re-export the generated module tree from its root so those paths resolve consistently:

```rust
mod generated;
pub use generated::*;
```

The ecommerce example uses this pattern in
[`examples/ecommerce/src/main.rs`](examples/ecommerce/src/main.rs).

## Workspace Config

An optional `spec.toml` at the repo root can relax `local_tests[].expect` validation for trusted workspaces and configure pipeline defaults:

```toml
[validation]
allow_unsafe_local_test_expect = false

[pipeline]
crate_root = "."          # path to Cargo.toml containing your crate (default: auto-detected)
cargo_target_dir = "target"  # cargo target dir (default: temp dir per run)
timeout_secs = 60         # abort cargo build/test if it exceeds this many seconds
```

When `allow_unsafe_local_test_expect = true`, `local_tests[].expect` still must parse as a Rust expression, but block, unsafe, closure, and other otherwise-rejected expression forms are allowed.

`spec build` and `spec test` auto-detect the nearest member crate (`[package]` Cargo.toml without `[workspace]`) to scope cargo to the right crate in a workspace. Override with `--crate-root <path>` or `[pipeline].crate_root` in `spec.toml`.
