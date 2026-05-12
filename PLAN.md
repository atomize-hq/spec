# M52: Bounded Same-Tree Wrapper TypeScript Execution Implementation Plan

Status: **implementation plan**
Milestone: **M52**
Milestone family: **second-language-backend**
Implementation readiness: **ready for bounded execution**
Plan scope: **extend the existing M46 TypeScript executor so `spec` can generate, build, test, and record target proof for the supported same-tree `function.wrapper.pipeline.v1` family without widening to cross-library imports, chain3, molecule execution, or generic arbitrary multi-dep TypeScript**
Base branch: **main**
Working branch: **feat/m40-plus**
Last rewritten: **2026-05-12**

Supersedes:
- the prior repo-root M51 shared-core portability plan previously maintained at this path

Primary source artifacts:
- [docs/m52_bounded_same_tree_wrapper_typescript_execution_design_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/m52_bounded_same_tree_wrapper_typescript_execution_design_v0.1.md)
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/checkpoints/20260506-181701-semantic-review-milestone-reset.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/checkpoints/20260506-181701-semantic-review-milestone-reset.md)
- [TODOS.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/TODOS.md)
- [README.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/README.md)
- [CHANGELOG.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/CHANGELOG.md)
- [docs/ai_promotion_and_multilanguage_milestones_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/ai_promotion_and_multilanguage_milestones_v0.1.md)

Primary repo surfaces:
- [spec-core/src/typescript_backend.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/typescript_backend.rs)
- [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs)
- [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs)
- [spec-cli/tests/cli.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/cli.rs)
- [examples/ecommerce/units/pricing/apply_discount.unit.spec](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/examples/ecommerce/units/pricing/apply_discount.unit.spec)
- [examples/ecommerce/units/pricing/apply_tax.unit.spec](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/examples/ecommerce/units/pricing/apply_tax.unit.spec)
- [examples/ecommerce/units/pricing/calculate_total.unit.spec](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/examples/ecommerce/units/pricing/calculate_total.unit.spec)
- [semantic-families/function.wrapper.pipeline.v1/candidate.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/function.wrapper.pipeline.v1/candidate.md)

## Executive Summary

M46 proved the executor.

It did not prove the first same-tree dependency closure.

`spec` can already execute one bounded TypeScript target lane for monotone-up leaves plus an optional helper dep. The semantic reviewer already supports more than that, including `function.wrapper.pipeline.v1`, but the TypeScript execution path still rejects the canonical wrapper closure before Bun runs. The live failure is concrete:

- `spec test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript` passes
- `spec test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript` fails because `pricing/apply_discount` is not eligible for the current bounded lane

That is the whole M52 problem.

M52 is the smallest honest second-language follow-on:

1. admit `function.wrapper.pipeline.v1` as a TypeScript execution target
2. require its direct dep closure to stay local to the same loaded tree
3. reuse the existing Bun runtime, passport proof surfaces, and semantic family authority
4. explicitly refuse chain3, cross-library imports, molecule tests, and generic arbitrary multi-dep claims

## Decision This Plan Makes

This plan authorizes exactly one milestone:

1. Widen the TypeScript validator and generator from "monotone-up leaf root only" to "supported wrapper pipeline root plus its exact same-tree local closure."
2. Keep the widened rule family-shaped, not dep-count-shaped.
3. Add authored `body.typescript` only where the canonical wrapper closure and maintained wrapper packet fixtures require it.
4. Prove the widened contract through CLI coverage, packet fixtures, and refreshed TypeScript target proof surfaces.

This plan does not authorize:

- cross-library TypeScript dep resolution
- generic multi-dependency TypeScript execution
- chain3 TypeScript execution
- `.test.spec --target-language typescript`
- seam-kind TypeScript execution
- `spec validate --target-language`
- `spec export --target-language`
- new shared-core extraction work
- renewed family-choice or corpus-program work

## Live Validated Basis

Validated from the current tree on `feat/m40-plus` on 2026-05-12.

Commands run:

```bash
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript
```

Observed command truth:

- `cargo xtask family recommend --format json`
  - `recommendation_status = "insufficient_real_corpus"`
  - `decision_summary.decision_status = "not_recommended"`
- `cargo xtask family corpus-decision --format json`
  - `decision_action = "stop"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `spec test ... apply_tax.unit.spec --target-language typescript`
  - generated 5 files
  - `bun build` passed
  - `bun test` passed
- `spec test ... calculate_total.unit.spec --target-language typescript`
  - failed before Bun with:
    - `unit 'pricing/apply_discount' is not eligible for the bounded M46 TypeScript lane: body.typescript is required`

Observed code truth:

- [spec-core/src/typescript_backend.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/typescript_backend.rs) still constrains the generator to:
  - `kind:function`
  - compatibility key `function.arithmetic_leaf.monotone_up.v1`
  - `deps: []` or exactly one direct local helper dep
- [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs) hard-codes the same M46 target gate with stable TypeScript-specific errors.
- [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs) already recognizes:
  - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - `function.arithmetic_leaf.monotone_up.v1`
  - `function.helper.identity_passthrough.v1`
  - `function.wrapper.pipeline.v1`
  - `function.wrapper.pipeline.chain3.v1`
- [semantic-families/function.wrapper.pipeline.v1/candidate.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/function.wrapper.pipeline.v1/candidate.md) freezes the wrapper packet as a straight-line two-call wrapper over two supported semantic deps in one packet-local tree.

That is enough basis. Another recommendation-governance milestone would be fake. The missing product truth is wrapper execution in `spec`.

## Step 0: Scope Challenge

### What Already Exists

| Sub-problem | Existing owner | Reuse verdict |
| --- | --- | --- |
| TS CLI target entry points | `spec-cli/src/commands.rs` | reuse |
| TS runtime and generated helper modules | `spec-core/src/typescript_backend.rs` | reuse, extend narrowly |
| target-proof passport storage | `spec-core/src/passport.rs` | reuse |
| wrapper family semantic boundary | `spec-core/src/semantic_review.rs` | reuse as authority |
| wrapper packet truth fixtures | `semantic-families/function.wrapper.pipeline.v1/**` | reuse, widen with authored TS bodies only where needed |
| canonical same-tree wrapper example | `examples/ecommerce/units/pricing/*` | reuse, widen the closure honestly |

### Minimum Complete Slice

The minimum honest slice is:

1. validator admits exactly one new TypeScript target family: `function.wrapper.pipeline.v1`
2. generator includes the wrapper and the exact same-tree local dep closure required for execution
3. canonical wrapper closure authors the TypeScript bodies the executor now depends on
4. CLI proof demonstrates wrapper TypeScript success and explicit rejection of out-of-scope variants
5. docs explain the new scope without implying generic multi-dep TypeScript

Anything smaller is fake done.

If the repo still fails on the canonical wrapper unit after the code change, or if it succeeds only by broadening to arbitrary supported multi-dep trees, the milestone failed.

### Complexity Check

This is a bounded lake:

- core executor surfaces:
  - `typescript_backend.rs`
  - `validator.rs`
  - `commands.rs`
  - `cli.rs`
- authored truth surfaces:
  - canonical ecommerce wrapper closure
  - wrapper packet fixtures
- docs:
  - README and backlog wording

No new service. No new runtime manager. No new output schema. No new cross-library policy.

### Search Check

No unfamiliar framework is entering the repo.

- **Layer 1**: reuse the existing Bun runtime, target-proof surfaces, and semantic-review routing.
- **Layer 1**: reuse the existing wrapper family boundary instead of inventing a new executor-only classifier.
- **Layer 3**: the executor should only widen where the semantic family contract is already strong enough to keep the claim honest.

### Completeness Check

Choose the complete version:

- executor truth
- canonical example truth
- packet fixture truth
- CLI proof truth
- docs truth

Do not ship the shortcut where only the validator widens, or only the canonical example grows TS bodies. The repo must be able to explain and prove the full new contract in one pass.

### TODOS Cross-Reference

Relevant deferred items remain deferred:

- wrapper TypeScript execution in `spec` -> spent by M52
- cross-library TypeScript helper imports -> still deferred
- generic multi-dependency TypeScript execution -> still deferred
- escape-hatch gate before broader second-language work -> still deferred

M52 should not create a new architecture TODO unless implementation reveals a missing proof surface that cannot fit inside the named write scope.

### Locked Plan Decisions

These are frozen for M52:

1. The widened TypeScript executor remains family-shaped.
2. `function.wrapper.pipeline.v1` is in scope.
3. `function.wrapper.pipeline.chain3.v1` is out of scope.
4. Cross-library TypeScript dep resolution is out of scope.
5. `.test.spec --target-language typescript` remains unsupported.
6. Rust remains the default target. TypeScript proof stays additive.
7. The Bun toolchain contract stays as-is.
8. No new `validate` or `export` target-language surface is added.

### Abort And Re-scope Triggers

Stop and re-scope if any of these become necessary:

1. the executor needs cross-library dep loading or alias resolution
2. the rule must widen to arbitrary supported multi-dep closures to make the canonical wrapper pass
3. chain3 turns out to be required for the first honest wrapper proof
4. passport or status storage needs a schema redesign rather than using the existing target-proof surface
5. the runtime contract needs new package-manager or config-file ownership beyond Bun

## Target End State

The post-M52 repo must tell one consistent story:

- TypeScript execution in `spec` supports:
  - monotone-up leaf roots
  - the supported same-tree wrapper pipeline family
- wrapper execution means:
  - exactly two direct local deps
  - same loaded unit set
  - same generated tree
  - exact supported family combination expected by the wrapper packet
- the canonical ecommerce wrapper example proves that contract
- the docs say exactly that, and no more

## Architecture Delta

```text
CURRENT
  TS target root = monotone_up leaf
    -> optional one local helper
    -> generate tree
    -> bun build
    -> bun local tests

M52
  TS target root = monotone_up leaf OR wrapper.pipeline.v1
    -> if wrapper:
       -> exactly two direct local supported deps
       -> include root + exact closure in generated tree
       -> run bun build and bun local tests
       -> refresh target_proofs.typescript

STILL OUT
  chain3
  cross-library deps
  molecule tests
  seam kinds
  generic supported multi-dep graphs
```

## In-Scope Files

Core code:

- `spec-core/src/typescript_backend.rs`
- `spec-core/src/validator.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`

Authored truth:

- `examples/ecommerce/units/pricing/apply_discount.unit.spec`
- `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- `examples/ecommerce/units/pricing/calculate_total.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/**`

Docs:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

## Out-Of-Scope Files

Do not touch unless a hard blocker proves the plan wrong:

- cross-library example libraries
- seam-kind units and seam portability code
- packet families other than `function.wrapper.pipeline.v1`
- recommendation-policy and shared-core planning docs
- CLI schema/export machinery beyond target-proof reuse

## Execution Order

1. Freeze the TypeScript wrapper eligibility contract in validator tests first.
2. Widen the generator and closure handling in `typescript_backend.rs`.
3. Adjust CLI generation and test coverage if the closure role needs explicit routing.
4. Add authored TypeScript bodies to the canonical wrapper closure and wrapper packet fixtures.
5. Land end-to-end CLI proofs for wrapper success and non-goal rejection.
6. Update README, CHANGELOG, and TODO wording to match the landed contract.

## Proof Floor

Required code-level proof:

- validator regressions for wrapper target eligibility
- generator regressions for wrapper closure emission
- CLI end-to-end success on the canonical wrapper unit
- CLI rejection coverage for at least one out-of-scope path

Required product-level proof:

```bash
cargo test -p spec-core typescript
cargo test -p spec-cli wrapper -- --nocapture
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript
```

If the final test still fails because the wrapper closure is missing authored TypeScript bodies or because the validator still classifies the closure under the wrong role, M52 is not done.

## Success Criteria

M52 is done only when all of these are true:

- `spec test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript` passes
- the wrapper packet can prove aligned TypeScript execution inside `spec`
- wrapper execution still rejects cross-library and generic arbitrary multi-dep widening
- passport target proofs remain additive and target-specific
- README explains the widened TypeScript lane as wrapper-family same-tree execution only

## Not In Scope

Deferred explicitly:

- chain3 TypeScript execution
- cross-library TypeScript helper imports
- generic multi-dependency TypeScript execution
- molecule TypeScript execution
- seam-kind TypeScript execution

Those are later milestones. M52 should not quietly spend them.
