<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260512-215821.md -->
# M54: Bounded Same-Tree Chain3 TypeScript Execution Plan

Status: **implementation plan**
Milestone: **M54**
Milestone family: **bounded-typescript-execution**
Implementation readiness: **ready for bounded execution**
Plan scope: **teach the existing Bun-backed TypeScript lane to execute exactly the promoted `function.wrapper.pipeline.chain3.v1` same-tree family, while preserving all current out-of-contract rejections**
Base branch: **main**
Working branch: **feat/m40-plus**
Validated at commit: **`1f04e28`**
Last rewritten: **2026-05-13**

Supersedes:

- the M53 shared-core portability closeout plan previously maintained at this path
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260512-214117.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/ceo-plans/2026-05-12-m54-bounded-chain3-typescript.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260512-214819.md`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260512-214117.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/ceo-plans/2026-05-12-m54-bounded-chain3-typescript.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260512-214819.md`
- `TODOS.md`
- `README.md`
- `CHANGELOG.md`

Primary repo surfaces:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/*.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/{drift,under_specified,unsupported_near_miss}/units/pricing/*.unit.spec`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

## Executive Summary

M54 closes the next real product mismatch in `spec`.

The repo already understands `function.wrapper.pipeline.chain3.v1` in semantic review. The promoted family packet and chain3 fixtures exist. The current TypeScript lane still rejects that exact family before Bun runs because the executor is bounded to `function.arithmetic_leaf.monotone_up.v1` and `function.wrapper.pipeline.v1`.

This plan widens the lane by one family, not by topology.

The accepted change is:

1. admit a TypeScript root only when semantic review classifies it as `function.wrapper.pipeline.chain3.v1`
2. require the root and every closure member to be local to the same loaded unit tree
3. require the direct dependency tuple to match the promoted chain3 family shape
4. emit only the root plus required closure members
5. flip the aligned chain3 TypeScript proof from pre-Bun rejection to passing Bun execution
6. keep cross-library imports, molecule tests, seam kinds, and generic multi-dependency roots rejected

If this becomes "support any three-dependency TypeScript unit," the milestone got worse.

## Live Basis

Observed current state from `feat/m40-plus` at `1f04e28`:

- `spec-core/src/validator.rs` defines the TypeScript target gate around:
  - `function.arithmetic_leaf.monotone_up.v1`
  - `function.wrapper.pipeline.v1`
  - helper closure member support through `function.helper.identity_passthrough.v1`
- `spec-core/src/typescript_backend.rs` currently renders bounded TypeScript trees for:
  - zero or one-dep monotone-up roots
  - two-dep wrapper roots
  - exact helper closure members
- `spec-cli/tests/cli.rs` includes `typescript_chain3_wrapper_rejects_before_bun_runs`, which currently proves the aligned chain3 fixture rejects before Bun.
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec` has the exact chain3 Rust root, but no maintained `body.typescript` yet.
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec` is the first direct dep and already represents the existing two-step wrapper family shape.

Known current stop-state truth from the source design:

- `recommendation_status = insufficient_real_corpus`
- `decision_status = not_recommended`
- `decision_action = stop`
- `required_next_action = record_stop_without_new_milestone`

M54 must not reopen that family-analysis stop-state. This is backend execution truth, not corpus recommendation machinery.

## Decision This Plan Makes

M54 authorizes exactly one bounded capability expansion:

```text
TypeScript execution root eligibility:

  before M54:
    function.arithmetic_leaf.monotone_up.v1
    function.wrapper.pipeline.v1

  after M54:
    function.arithmetic_leaf.monotone_up.v1
    function.wrapper.pipeline.v1
    function.wrapper.pipeline.chain3.v1
```

M54 does not authorize:

- generic TypeScript execution for arbitrary multi-dependency roots
- cross-library TypeScript helper or function imports
- TypeScript molecule test execution
- TypeScript execution for `kind:data`, `kind:sum`, marked seams, or any other seam kind
- new proof routing semantics
- new passport schema fields
- new export schema fields
- new package, binary, or release channel
- new family-analysis recommendation or corpus-decision behavior

## Step 0: Scope Challenge

### What Already Exists

| Sub-problem | Existing owner | Reuse verdict |
|---|---|---|
| TypeScript target-language CLI path | `spec-cli/src/commands.rs`, `spec-core/src/backend_execution.rs`, `spec-core/src/typescript_backend.rs` | reuse as-is |
| TypeScript root validation | `spec-core/src/validator.rs` | extend exact family gate |
| Same-tree closure rendering | `spec-core/src/typescript_backend.rs` | extend bounded closure collector |
| Semantic family classification | `spec-core/src/semantic_review.rs` | reuse as source of truth |
| Existing wrapper dep contract | `validate_typescript_wrapper_dep_contract` in `spec-core/src/validator.rs` | mirror for chain3 |
| Existing wrapper closure proof | `typescript_tree_renders_wrapper_closure_without_unrelated_units` in `spec-core/src/typescript_backend.rs` | add chain3 sibling test |
| Chain3 fixture truth | `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/*.unit.spec` | add TypeScript bodies only where needed |
| Pre-Bun rejection proof | `typescript_chain3_wrapper_rejects_before_bun_runs` in `spec-cli/tests/cli.rs` | replace with pass proof and add negative reject cases |

### Minimum Complete Slice

The minimum complete M54 slice is:

1. Add a chain3 compatibility-key constant in `spec-core/src/validator.rs`.
2. Add a `Chain3WrapperPipeline` root-family case.
3. Add `validate_typescript_chain3_dep_contract`.
4. Teach target validation that a chain3 root must have exactly three direct deps.
5. Teach closure-member validation that a `function.wrapper.pipeline.v1` member may appear inside a chain3 root closure.
6. Teach `spec-core/src/typescript_backend.rs` to collect the chain3 root closure recursively.
7. Add `body.typescript` to the aligned chain3 root and any required aligned closure members that do not already have it.
8. Update unit tests for validator acceptance and rejection.
9. Update TypeScript backend tests for exact tree emission and unrelated-unit exclusion.
10. Update CLI tests so the aligned fixture runs through Bun while out-of-contract fixtures still reject before Bun.
11. Update README, CHANGELOG, and TODOS so docs say "bounded same-tree chain3" rather than "generic multi-dep TypeScript."

Anything smaller is fake done because the executor, proof, and docs would disagree.

Anything larger is scope growth.

### Complexity Check

Expected write scope:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`
- aligned chain3 fixture `.unit.spec` files
- possibly negative chain3 fixture `.unit.spec` files only if a precise reject case needs authored TypeScript
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

This is more than a tiny patch, but it is still below the architecture-smell threshold because there are no new crates, services, commands, schemas, or runtime dependencies.

### Search Check

No unfamiliar platform, package, or infrastructure is entering the repo.

- **[Layer 1]** reuse the current Bun-backed TypeScript lane.
- **[Layer 1]** reuse the existing semantic-review classification and validator pattern.
- **[Layer 1]** reuse the existing fixture-copy CLI test style.
- **[Layer 3]** the first-principles insight is that a family-shaped executor boundary is more truthful than a dep-count boundary. "Three deps" is an implementation accident. `function.wrapper.pipeline.chain3.v1` is the product contract.

### TODOS Cross-Reference

Keep these deferred:

- cross-library TypeScript helper imports
- generic multi-dependency TypeScript execution
- TypeScript molecule execution
- seam-kind TypeScript execution
- richer target-proof surfacing for closure members
- maintained ecommerce chain3 TypeScript example, unless implementation proves the packet fixture alone is too opaque

### Completeness Check

Choose the complete bounded version.

With AI-assisted implementation, adding the exact negative tests and docs is cheap compared with the future cost of an ambiguous TypeScript lane. The complete lake is not "generic TypeScript." The complete lake is exact chain3 support with a proof wall on both sides.

### Distribution Check

No new distributable artifact is introduced.

This is an existing `spec` CLI capability expansion. Users continue receiving it through the current build and release paths:

- `cargo install spec-cli`
- GitHub Releases
- existing repo CI

## Locked Plan Decisions

1. Chain3 support is keyed by `function.wrapper.pipeline.chain3.v1`, not by `deps.len() == 3`.
2. Chain3 support is same-tree only. Any `shared::...` dep remains rejected.
3. Direct dep order is part of the contract:
   - dep 1: `function.wrapper.pipeline.v1`
   - dep 2: `function.arithmetic_leaf.monotone_up.v1`
   - dep 3: `function.arithmetic_leaf.monotone_down_nonnegative.v1`
4. Closure recursion is allowed only through validated supported family members.
5. Closure emission includes required helper deps, but excludes unrelated loaded units.
6. Rust and TypeScript proof remain additive and target-specific.
7. Molecule tests stay Rust-only.
8. Docs must name the boundary as "bounded same-tree chain3 TypeScript execution."

## Abort And Re-scope Triggers

Stop and rewrite the plan if any of these become necessary:

1. supporting chain3 requires a generic graph executor instead of the current bounded closure collector
2. a chain3 root needs cross-library dependency resolution to pass
3. molecule TypeScript execution becomes necessary to prove the feature
4. export or passport schemas need new fields
5. semantic review cannot classify the aligned fixture as `function.wrapper.pipeline.chain3.v1`
6. the implementation starts changing family-analysis recommendation or stop-state commands

## Target End State

After M54, this command passes:

```bash
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec --target-language typescript
```

And these remain rejected before Bun or before unsupported execution:

- cross-library chain3 deps
- wrong direct dep order
- wrong direct dep count
- unsupported direct dep family
- required closure member missing `body.typescript`
- unrelated loaded units leaking into emitted TypeScript
- `.test.spec --target-language typescript`
- seam-kind TypeScript targets

## Architecture Review

### Architecture Delta

```text
CURRENT
  spec test --target-language typescript
    |
    v
  validate_typescript_execution_target_spec_with_specs()
    |
    +-- monotone_up root, deps 0..1
    |
    +-- wrapper root, deps exactly 2
    |
    `-- chain3 root, deps exactly 3
        rejected as unsupported

TARGET M54
  spec test --target-language typescript
    |
    v
  validate_typescript_execution_target_spec_with_specs()
    |
    +-- monotone_up root, deps 0..1
    |
    +-- wrapper root, deps exactly 2
    |
    `-- chain3 root, deps exactly 3
          |
          +-- dep 1: wrapper.pipeline.v1, same tree
          +-- dep 2: monotone_up.v1, same tree
          `-- dep 3: monotone_down_nonnegative.v1, same tree
```

### TypeScript Closure Flow

```text
checkout_chain3_aligned
  |
  +-- pricing_total_wrapper_aligned
  |     |
  |     +-- pricing_discount_leaf_aligned
  |     |     |
  |     |     `-- optional helper closure, if authored
  |     |
  |     `-- pricing_tax_leaf_aligned
  |           |
  |           `-- optional helper closure, if authored
  |
  +-- pricing_tax_leaf_aligned
  |     |
  |     `-- optional helper closure, if authored
  |
  `-- pricing_discount_leaf_aligned
        |
        `-- optional helper closure, if authored

Emission rule:
  include each required unit once
  include runtime/build/test support modules
  exclude unrelated units in the loaded fixture tree
```

### Required Code Changes

#### `spec-core/src/validator.rs`

Add a constant:

```rust
pub const TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY: &str =
    "function.wrapper.pipeline.chain3.v1";
```

Extend the root family enum:

```rust
enum TypescriptTargetRootFamily {
    MonotoneUp,
    WrapperPipeline,
    Chain3WrapperPipeline,
}
```

Update `classify_typescript_target_root_family`:

- include chain3 in "requires compatibility key" messages
- return `Chain3WrapperPipeline` for `function.wrapper.pipeline.chain3.v1`
- preserve unsupported semantic review handling

Update `validate_typescript_execution_target_spec_with_specs`:

- `0 | 1` deps allow only `MonotoneUp`
- `2` deps allow only `WrapperPipeline`
- `3` deps allow only `Chain3WrapperPipeline`
- any other arity rejects with the right family-specific message

Add `validate_typescript_chain3_dep_contract`:

- require exactly three deps
- parse every dep with `DepRef::parse`
- reject any `library_alias`
- require every dep to resolve in `specs_by_id`
- evaluate each dep with `SemanticReviewContext::new(specs_by_id)`
- require supported status
- require exact compatibility keys:
  - first: `TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY`
  - second: `TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY`
  - third: `TYPESCRIPT_WRAPPER_FIRST_DEP_COMPATIBILITY_KEY`
- require every direct dep to author non-empty `body.typescript`
- use clear M54 error strings, not stale M52-only language

Update `validate_typescript_closure_member_spec_with_specs`:

- keep helper, monotone-up, and wrapper-first-dep behavior
- allow `TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY` as a closure member only when it passes the existing wrapper dep contract
- do not allow chain3 as a closure member in M54

#### `spec-core/src/typescript_backend.rs`

Update the module comment from "Bounded M52" to a milestone-neutral description, or explicitly include M54:

```text
Bounded TypeScript backend generation for promoted same-tree function families.
```

Import the new chain3 constant.

Update `collect_typescript_root_closure`:

- when root family is chain3, iterate exactly three direct deps
- resolve each local dep
- call `collect_typescript_closure_member` for each dep
- rely on validator for tuple shape and same-tree guard

Update `collect_typescript_closure_member`:

- recurse through wrapper closure members so `pricing_total_wrapper_aligned` pulls its leaf deps
- preserve helper closure recursion for leaf members
- keep `included` as a `BTreeSet` to dedupe repeated direct and nested deps

Add a backend test sibling to `typescript_tree_renders_wrapper_closure_without_unrelated_units`:

- construct helper, discount, tax, wrapper, chain3, unrelated
- generate the tree with chain3 root only
- assert emitted:
  - `pricing/checkout_chain3.ts`
  - `pricing/calculate_total.ts`
  - `pricing/apply_discount.ts`
  - `pricing/apply_tax.ts`
  - helper if used
  - runtime/build/local-test support files
- assert unrelated unit is not emitted
- assert imports are relative and stable

#### Chain3 Fixture Specs

Add `body.typescript` to:

- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec`, if not already authored
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec`, if not already authored
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec`, if not already authored

The root TypeScript body must mirror the Rust body:

```typescript
const base_total = pricing_total_wrapper_aligned(subtotal, discount_rate, tax_rate);
const surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
return pricing_discount_leaf_aligned(surcharged_total, loyalty_rate);
```

Keep authored names aligned with generated function aliases. Do not introduce new fixture units just to make TypeScript easier.

#### `spec-cli/tests/cli.rs`

Replace `typescript_chain3_wrapper_rejects_before_bun_runs` with a passing aligned proof:

- copy aligned chain3 fixture
- ensure authored TypeScript exists
- run:

```bash
spec test units/pricing/checkout_chain3_aligned.unit.spec --target-language typescript
```

- assert success
- assert evidence/passport includes target-specific TypeScript proof, if the surrounding helpers already expose that check cleanly

Add or preserve negative CLI tests:

- molecule TypeScript rejection still happens before Bun
- cross-library chain3 dep rejects before Bun
- wrong chain3 direct dep order rejects before Bun
- unsupported near-miss chain3 fixture rejects before Bun
- missing required `body.typescript` rejects before Bun

Do not add a generic "multi-dep TypeScript now works" test.

### Production Failure Scenarios

| Codepath | Realistic failure | Plan coverage |
|---|---|---|
| Root family classification | unsupported chain3-like unit gets admitted by dep count | exact compatibility key required |
| Direct dep validation | wrong order produces plausible but wrong runtime math | exact tuple validation and negative test |
| Same-tree resolution | root imports `shared::pricing/apply_tax` and generated TS cannot resolve it | cross-library rejection before Bun |
| Closure recursion | wrapper dep emits without its required leaf deps | backend tree test asserts nested closure emission |
| Dedupe | direct leaf also appears inside wrapper and gets duplicated or imported inconsistently | `BTreeSet` inclusion and generated tree assertions |
| Missing TS body | Rust-only closure member reaches Bun and fails late | validator rejects non-empty `body.typescript` before generation |
| Unrelated loaded units | generator emits too much and accidentally makes unsupported code look supported | exact tree exclusion assertion |

## Code Quality Review

### DRY And Explicitness

Do not copy the wrapper validation function and tweak strings blindly. Extract only if it reduces real duplication without hiding the family contract.

Recommended shape:

- keep wrapper validation explicit
- add chain3 validation explicit
- optionally add one small helper for "parse local dep and resolve supported review" if the code otherwise repeats the same block three times

Avoid a generic `Vec<ExpectedDepFamily>` abstraction unless it is less than roughly 30 obvious lines and makes error messages clearer. The user preference is explicit over clever and minimal diff. This is exactly that case.

### Error Message Quality

Every M54 rejection should tell the maintainer:

1. what was unsupported
2. which dep or family caused it
3. that the boundary is the bounded M54 TypeScript lane

Bad:

```text
unsupported.function.v1
```

Good:

```text
TypeScript chain3 target requires direct dep 1 to resolve to function.wrapper.pipeline.v1 in M54: 'pricing/foo' resolved to unsupported.function.v1
```

Do not leave new chain3 failures saying only "M52" unless the message is shared with older behavior and still true.

### Inline Diagrams

No production code comment diagram is required if the validator and backend functions stay short and explicit.

Add an inline ASCII diagram only if `collect_typescript_root_closure` grows enough that the root and closure roles are no longer obvious. If added, put it above the chain3 branch in `spec-core/src/typescript_backend.rs`.

## Test Review

### Test Framework Detection

Runtime: Rust workspace.

Primary test commands:

```bash
cargo test -p spec-core validator::tests::typescript_target_accepts_chain3_root_with_exact_local_dep_tuple
cargo test -p spec-core typescript_backend::tests::typescript_tree_renders_chain3_closure_without_unrelated_units
cargo test -p spec-cli typescript_chain3_wrapper_executes_with_bun
```

Final proof commands:

```bash
cargo test -p spec-core
cargo test -p spec-cli
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/checkout_chain3_unsupported_near_miss.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/checkout_chain3_drift.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/checkout_chain3_under_specified.unit.spec --target-language typescript
```

The last three commands are expected to fail cleanly unless the fixture truth says otherwise. The assertion is rejection before unsupported Bun execution, not success.

### Code Path Coverage

```text
CODE PATH COVERAGE
==================
[+] spec-core/src/validator.rs
    |
    +-- classify_typescript_target_root_family()
    |   +-- [NEEDS TEST] MonotoneUp still accepted
    |   +-- [NEEDS TEST] WrapperPipeline still accepted
    |   +-- [GAP] Chain3WrapperPipeline accepted by exact compatibility key
    |   `-- [NEEDS TEST] unsupported family rejected with message listing all supported keys
    |
    +-- validate_typescript_execution_target_spec_with_specs()
    |   +-- [NEEDS TEST] 0..1 deps only valid for monotone-up roots
    |   +-- [NEEDS TEST] 2 deps only valid for wrapper roots
    |   +-- [GAP] 3 deps only valid for chain3 roots
    |   `-- [GAP] 4+ deps rejected even if every dep has TypeScript
    |
    +-- validate_typescript_chain3_dep_contract()
    |   +-- [GAP] exact tuple accepted: wrapper, monotone-up, monotone-down
    |   +-- [GAP] wrong dep order rejected
    |   +-- [GAP] cross-library dep rejected
    |   +-- [GAP] missing dep rejected
    |   +-- [GAP] unsupported dep family rejected
    |   `-- [GAP] missing dep body.typescript rejected
    |
    `-- validate_typescript_closure_member_spec_with_specs()
        +-- [NEEDS TEST] helper closure member still accepted
        +-- [NEEDS TEST] monotone-up closure member still accepted
        +-- [GAP] wrapper closure member accepted inside chain3 closure
        `-- [GAP] nested chain3 closure member rejected

[+] spec-core/src/typescript_backend.rs
    |
    +-- collect_typescript_root_closure()
    |   +-- [NEEDS TEST] monotone-up behavior unchanged
    |   +-- [NEEDS TEST] wrapper behavior unchanged
    |   `-- [GAP] chain3 root collects all exact deps recursively
    |
    +-- collect_typescript_closure_member()
    |   +-- [GAP] wrapper member recursively collects its leaf deps
    |   +-- [NEEDS TEST] helper member terminates recursion
    |   `-- [GAP] repeated direct/nested deps are emitted once
    |
    `-- render TypeScript tree
        +-- [GAP] chain3 root module imports wrapper, tax, discount
        +-- [GAP] wrapper module imports its leaves
        +-- [GAP] unrelated loaded unit excluded
        `-- [NEEDS TEST] runtime/build/local-test support modules still emitted

[+] spec-cli/tests/cli.rs
    |
    +-- spec test chain3 aligned --target-language typescript
    |   `-- [GAP] [->E2E] succeeds through Bun
    |
    +-- spec test unsupported chain3 --target-language typescript
    |   +-- [GAP] [->E2E] wrong family rejects before Bun
    |   +-- [GAP] [->E2E] missing TypeScript body rejects before Bun
    |   `-- [GAP] [->E2E] cross-library dep rejects before Bun, if fixture added
    |
    `-- spec test molecule --target-language typescript
        `-- [NEEDS TEST] [->E2E] remains rejected before Bun

---------------------------------
COVERAGE TARGET: 100% of changed branches
CURRENT PLAN GAPS: 18 explicit test requirements
E2E/CLI REQUIRED: 4 paths
EVAL REQUIRED: 0 paths
REGRESSION TESTS REQUIRED: yes, chain3 currently rejects before Bun and must flip only for aligned in-contract root
---------------------------------
```

### Required Test Additions

#### Validator Unit Tests

Add:

- `typescript_target_accepts_chain3_root_with_exact_local_dep_tuple`
- `typescript_target_rejects_chain3_wrong_dep_order`
- `typescript_target_rejects_chain3_cross_library_dep`
- `typescript_target_rejects_chain3_missing_dep`
- `typescript_target_rejects_chain3_wrong_dep_family`
- `typescript_target_rejects_chain3_dep_missing_typescript_body`
- `typescript_target_rejects_generic_four_dep_root`
- `typescript_closure_member_accepts_wrapper_pipeline_member`
- `typescript_closure_member_rejects_chain3_member`

Preserve:

- monotone-up root acceptance
- helper dep rejection cases
- wrapper root acceptance and rejection cases
- molecule TypeScript rejection

#### TypeScript Backend Unit Tests

Add:

- `typescript_tree_renders_chain3_closure_without_unrelated_units`
- `typescript_tree_rejects_chain3_closure_member_missing_typescript_body`
- `typescript_tree_rejects_chain3_cross_library_dep_before_render`

Preserve:

- zero-dep module rendering
- wrapper closure rendering
- helper import rendering

#### CLI Regression Tests

Add or update:

- `typescript_chain3_wrapper_executes_with_bun`
- `typescript_chain3_wrong_family_rejects_before_bun_runs`
- `typescript_chain3_missing_typescript_body_rejects_before_bun_runs`
- `typescript_chain3_wrong_dep_order_rejects_before_bun_runs`, if fixture mutation is small

Keep:

- `typescript_molecule_test_is_rejected_before_bun_runs`

### Test Plan Artifact

The plan-eng-review source artifact already exists:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260512-214819.md`

Use it as QA input. If implementation changes the proof surface, regenerate a fresh plan-eng-review test artifact before `/qa`.

## Failure Modes Registry

| # | Codepath | Failure mode | Test required | Error handling required | User-visible result | Critical gap |
|---|---|---|---|---|---|---|
| 1 | chain3 root classification | dep-count-based admission lets unsupported families run | validator wrong-family test | clear unsupported-family error | clear CLI stderr | yes until tested |
| 2 | direct dep tuple | wrong order produces wrong math but valid TS | wrong-order validator and CLI tests | exact dep position error | clear CLI stderr | yes until tested |
| 3 | cross-library dep | generated import path cannot resolve in Bun | cross-library reject test | pre-Bun rejection | clear CLI stderr | no if tested |
| 4 | nested wrapper closure | emitted tree misses wrapper leaf deps | backend closure tree test | generator error before Bun if missing | clear CLI stderr | yes until tested |
| 5 | missing TypeScript body | Bun sees undefined implementation or generation panics | missing body tests | validator/generator rejection | clear CLI stderr | yes until tested |
| 6 | unrelated loaded unit | unsupported unit is emitted and looks supported | unrelated exclusion assertion | no error, just exclusion | invisible to user if correct | no if tested |
| 7 | molecule target | `.test.spec` accidentally reaches Bun | existing molecule pre-Bun test | existing rejection | clear CLI stderr | no if preserved |
| 8 | stale docs | users believe generic multi-dep TS is supported | docs review in final diff | docs say exact boundary | avoids false promise | no |

## Performance Review

No new hot path or runtime service is introduced.

Performance risks are local and bounded:

- Semantic review may be evaluated multiple times during validation and closure collection. This is acceptable for fixture-scale trees, but do not introduce an unbounded repeated traversal if a simple `SemanticReviewContext` and map lookup already exists.
- Closure collection must remain O(units in required closure), not O(all loaded units per dep), after the initial loaded maps are built.
- `BTreeSet` dedupe should stay in place to prevent repeated wrapper and leaf emission.
- CLI tests that invoke Bun should stay targeted. Do not add a broad fixture matrix that makes `cargo test -p spec-cli` crawl.

## Documentation Plan

Update docs only after tests are green.

Required docs:

- `CHANGELOG.md`
  - add M54 note: bounded same-tree `function.wrapper.pipeline.chain3.v1` TypeScript execution
  - explicitly say generic multi-dependency TypeScript remains unsupported
- `README.md`
  - update TypeScript target-language support list
  - include one chain3 command example if the README already has target-language examples nearby
- `TODOS.md`
  - keep cross-library helper imports and generic multi-dep TS as deferred
  - remove or reword any TODO that says chain3 same-tree TS itself is still missing

Do not rewrite broad roadmap docs unless a specific sentence becomes false.

## NOT In Scope

| Deferred item | Rationale |
|---|---|
| Generic same-tree multi-dependency TypeScript execution | would replace family truth with topology truth |
| Cross-library TypeScript helper imports | separate portability and resolution problem |
| TypeScript molecule execution | current contract is Rust-only molecule tests |
| Seam-kind TypeScript execution | unrelated target family |
| New export or passport schema fields | target proof already supports additive target-specific evidence |
| Maintained ecommerce chain3 TypeScript example | useful later, not required if packet fixture proof is readable |
| Family-analysis recommendation changes | current stop-state must remain untouched |
| New release pipeline | existing CLI distribution is enough |

## Worktree Parallelization Strategy

There is a real parallelization opportunity, but only after the validator contract is frozen. The validator is the hinge.

### Dependency Table

| Step | Modules touched | Depends on |
|---|---|---|
| A. Validator contract | `spec-core/src/validator.rs` | none |
| B. TypeScript backend closure | `spec-core/src/typescript_backend.rs` | A |
| C. Chain3 fixture TypeScript bodies | `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/` | A |
| D. CLI proof and negative regressions | `spec-cli/tests/`, chain3 fixtures | A, B, C |
| E. Docs sync | repo docs | A, B, C, D |

### Parallel Lanes

```text
Lane A: Validator contract (sequential foundation)

After Lane A merges:
  Lane B: TypeScript backend closure
  Lane C: Chain3 fixture TypeScript bodies

After B + C merge:
  Lane D: CLI proof and negative regressions
  Lane E: Docs sync, can start once proof names and boundaries are stable
```

### Execution Order

1. Start Lane A first. Do not parallelize the root-family contract.
2. Launch Lane B and Lane C in parallel worktrees after Lane A is merged or rebased into both.
3. Launch Lane D after B and C are both green.
4. Run Lane E after the final command and error strings are known.

### Conflict Flags

- Lane B and Lane A both touch `spec-core` if started too early. Keep them sequential or coordinate tightly.
- Lane C and Lane D both touch chain3 fixtures if CLI tests mutate fixture files. Prefer authored fixture bodies in Lane C, then CLI tests read them without mutation.
- Docs should wait because stale command examples are worse than no examples.

Parallelization summary: **3 lanes after the validator foundation, 2 parallel, 3 sequential gates**.

## Implementation Checklist

### Phase 1: Validator Contract

- [ ] Add chain3 compatibility key constant.
- [ ] Extend root-family enum.
- [ ] Update compatibility-key error messages to include chain3.
- [ ] Add chain3 direct-dep validator with exact tuple semantics.
- [ ] Allow `function.wrapper.pipeline.v1` as a closure member.
- [ ] Reject chain3 as a nested closure member.
- [ ] Add validator unit tests for accept and reject cases.

### Phase 2: Backend Closure

- [ ] Update bounded TypeScript backend module docs.
- [ ] Add chain3 branch in root closure collection.
- [ ] Recurse through wrapper closure members.
- [ ] Preserve helper closure behavior.
- [ ] Add exact tree rendering test for chain3.
- [ ] Add missing body and cross-library generator rejection tests if not fully covered by validator tests.

### Phase 3: Fixture Truth

- [ ] Add `body.typescript` to aligned chain3 root.
- [ ] Ensure wrapper, tax leaf, and discount leaf aligned specs have non-empty TypeScript bodies.
- [ ] Keep local tests bounded to the supported TypeScript expect AST.
- [ ] Do not add unrelated fixture units.

### Phase 4: CLI Proof Wall

- [ ] Flip aligned chain3 CLI test from reject-before-Bun to pass-through-Bun.
- [ ] Preserve molecule reject-before-Bun test.
- [ ] Add wrong tuple, wrong family, missing TypeScript, and cross-library pre-Bun rejection tests as feasible.
- [ ] Verify target-specific TypeScript proof remains additive beside Rust proof.

### Phase 5: Docs And Backlog

- [ ] Update `CHANGELOG.md`.
- [ ] Update `README.md`.
- [ ] Update `TODOS.md`.
- [ ] Avoid broad roadmap rewrites unless directly false.

## Validation Commands

Run during implementation:

```bash
cargo test -p spec-core typescript_target
cargo test -p spec-core typescript_tree
cargo test -p spec-cli typescript_chain3
```

Run before calling the milestone done:

```bash
cargo test -p spec-core
cargo test -p spec-cli
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/checkout_chain3_unsupported_near_miss.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/checkout_chain3_drift.unit.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/checkout_chain3_under_specified.unit.spec --target-language typescript
```

Expected result:

- aligned command passes
- out-of-contract commands fail with clear bounded-lane errors
- Bun is not invoked for validator-level rejections

## Success Criteria

- `spec test ...checkout_chain3_aligned.unit.spec --target-language typescript` passes.
- Generated TypeScript tree includes the chain3 root, wrapper dep, leaf deps, required helpers, runtime module, build entry, and local tests module.
- Generated TypeScript tree excludes unrelated loaded units.
- Validator rejects:
  - cross-library deps
  - wrong dep order
  - wrong dep count
  - unsupported dep family
  - missing required `body.typescript`
  - nested chain3 closure members
- Existing monotone-up and two-step wrapper TypeScript tests still pass.
- Molecule TypeScript targets still reject before Bun.
- README, CHANGELOG, and TODOS accurately describe the bounded chain3 lane.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Choose exact chain3 family execution | Mechanical | completeness, explicit over clever | closes the real semantic-review/backend mismatch without widening topology | generic multi-dep TS |
| 2 | CEO | Keep cross-library TypeScript imports deferred | Mechanical | boil lakes, pragmatic | separate portability problem already tracked and not required for same-tree chain3 | cross-library imports in M54 |
| 3 | CEO | Keep molecule TypeScript execution out | Mechanical | minimal diff | molecule tests remain Rust-only by contract | molecule TS support |
| 4 | Eng | Require exact direct-dep tuple | Mechanical | explicit over clever | prevents a dep-count abstraction from admitting wrong math | arbitrary supported trio |
| 5 | Eng | Add negative tests before docs | Mechanical | tests non-negotiable | proof wall defines the boundary users rely on | docs-only claim |
| 6 | Eng | Parallelize only after validator foundation | Mechanical | systems over heroes | avoids conflicting interpretations of the contract across worktrees | fully parallel start |

## Cross-Phase Themes

The CEO and eng inputs agree on one theme: family truth must drive execution support. That is the high-confidence signal.

The plan should feel conservative in implementation and ambitious in proof. Conservative code, aggressive tests. Good.

## Completion Summary

- Step 0: Scope Challenge: scope accepted as exact chain3 same-tree TypeScript execution
- Architecture Review: 0 unresolved architecture issues, exact family boundary required
- Code Quality Review: 0 unresolved code-quality issues, avoid generic dep-family abstraction unless tiny and clearer
- Test Review: diagram produced, 18 explicit gaps identified for implementation
- Performance Review: 0 blocking issues, keep closure collection bounded and deduped
- NOT in scope: written
- What already exists: written
- TODOS.md updates: docs/backlog sync required during implementation
- Failure modes: 4 critical gaps until tests land
- Outside voice: unavailable in source review, single-reviewer mode
- Parallelization: 5 workstreams, 2 parallel after validator foundation, 3 sequential gates
- Lake Score: 6/6 decisions chose the complete bounded option

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|---|---|---|---:|---|---|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR | exact chain3 scope accepted, generic TS deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | - | outside voice unavailable in source review |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR WITH REQUIRED TESTS | 18 test requirements, 4 critical gaps until implemented |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | no UI scope |

**UNRESOLVED:** 0 plan decisions.

**VERDICT:** CEO + ENG CLEARED for implementation. The implementation is not done until the proof wall above is green.
