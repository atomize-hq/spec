<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260512-215821.md -->
# M54: Bounded Same-Tree Chain3 TypeScript Execution Plan

Status: **implementation plan**
Milestone: **M54**
Milestone family: **bounded-typescript-execution**
Implementation readiness: **ready for bounded execution**
Plan scope: **extend the existing Bun-backed TypeScript lane to execute exactly `function.wrapper.pipeline.chain3.v1` in the same loaded tree, while preserving every current out-of-contract rejection**
Base branch: **main**
Working branch: **feat/m40-plus**
Validated at commit: **`1f04e28`**
Last rewritten: **2026-05-13**

Supersedes:

- the prior M53 closeout plan at this path
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

M54 closes a specific product mismatch that already exists in the repo.

`spec` can already classify `function.wrapper.pipeline.chain3.v1` truthfully during semantic review. The promoted family packet and chain3 fixtures already exist. The TypeScript execution lane still rejects that exact family before Bun runs because the lane only admits:

- `function.arithmetic_leaf.monotone_up.v1`
- `function.wrapper.pipeline.v1`

M54 widens the lane by one family, not by topology.

After M54:

1. a TypeScript root is admitted when semantic review classifies it as `function.wrapper.pipeline.chain3.v1`
2. the root and every required closure member must live in the same loaded unit tree
3. the root must use the exact direct-dependency tuple defined by the promoted chain3 family
4. generated TypeScript emits only the root plus the required closure members
5. the aligned chain3 fixture flips from pre-Bun rejection to successful Bun execution
6. cross-library imports, generic multi-dependency roots, molecule targets, seam kinds, and unrelated loaded units all stay rejected

If implementation starts sounding like "support any three-dependency TypeScript unit," the milestone drifted and the plan should stop.

## Current State

Observed on `feat/m40-plus` at `1f04e28`:

- `spec-core/src/validator.rs` gates TypeScript execution around the monotone-up root, the two-step wrapper root, and helper closure members
- `spec-core/src/typescript_backend.rs` renders bounded TypeScript trees for those same promoted same-tree families
- `spec-cli/tests/cli.rs` still proves `typescript_chain3_wrapper_rejects_before_bun_runs`
- `spec-core/src/semantic_review.rs` already recognizes `function.wrapper.pipeline.chain3.v1`
- the aligned chain3 fixture exists, but the maintained TypeScript execution story is incomplete

Known stop-state truth from the source design:

- `recommendation_status = insufficient_real_corpus`
- `decision_status = not_recommended`
- `decision_action = stop`
- `required_next_action = record_stop_without_new_milestone`

M54 changes backend execution truth only. It does not reopen corpus recommendation or family-analysis stop-state logic.

## What Already Exists

| Sub-problem | Existing owner | M54 action |
|---|---|---|
| TypeScript CLI entrypoint | `spec-cli/src/commands.rs`, `spec-core/src/backend_execution.rs` | reuse |
| TypeScript root validation | `spec-core/src/validator.rs` | extend exact family gate |
| Same-tree closure rendering | `spec-core/src/typescript_backend.rs` | extend bounded closure collector |
| Semantic family classification | `spec-core/src/semantic_review.rs` | reuse as source of truth |
| Existing wrapper dep contract | `validate_typescript_wrapper_dep_contract` in `spec-core/src/validator.rs` | mirror structure for chain3 |
| Existing wrapper closure proof | `typescript_tree_renders_wrapper_closure_without_unrelated_units` in `spec-core/src/typescript_backend.rs` | add chain3 sibling test |
| Chain3 fixtures | `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/...` | add and maintain TypeScript bodies only where needed |
| Current pre-Bun rejection proof | `typescript_chain3_wrapper_rejects_before_bun_runs` in `spec-cli/tests/cli.rs` | replace aligned-case expectation, preserve negative rejects |

## Scope And Non-Goals

### In Scope

- admit `function.wrapper.pipeline.chain3.v1` as a TypeScript execution root
- validate the exact same-tree direct-dependency tuple required by the promoted family
- allow the root closure to include required wrapper and leaf members already supported by the bounded lane
- render only the required closure into the generated TypeScript tree
- flip the aligned chain3 CLI proof from reject-before-Bun to pass-through-Bun
- preserve all current rejection behavior outside the new bounded family
- update docs and backlog language so the public contract stays exact

### Not In Scope

- generic same-tree multi-dependency TypeScript execution
- cross-library TypeScript helper or function imports
- TypeScript molecule execution
- TypeScript execution for `kind:data`, `kind:sum`, marked seams, or any other seam kind
- new passport schema fields
- new export schema fields
- family-analysis recommendation changes
- new package, binary, or release channel
- a maintained ecommerce chain3 TypeScript example unless implementation proves the packet fixture is too opaque

## Locked Decisions

These are contract decisions, not suggestions:

1. Chain3 support is keyed by `function.wrapper.pipeline.chain3.v1`, not by `deps.len() == 3`.
2. Chain3 support is same-tree only. Any `shared::...` dependency remains rejected.
3. Direct dependency order is part of the contract:
   - dep 1: `function.wrapper.pipeline.v1`
   - dep 2: `function.arithmetic_leaf.monotone_up.v1`
   - dep 3: `function.arithmetic_leaf.monotone_down_nonnegative.v1`
4. Closure recursion is allowed only through already-supported bounded family members.
5. `function.wrapper.pipeline.v1` may appear as a closure member under a chain3 root if it passes the existing wrapper contract.
6. `function.wrapper.pipeline.chain3.v1` may not appear as a nested closure member in M54.
7. Generated TypeScript includes each required unit once and excludes unrelated loaded units.
8. Rust and TypeScript proof remain additive and target-specific.
9. Molecule tests stay Rust-only.
10. Documentation must describe the boundary as "bounded same-tree chain3 TypeScript execution."

## Abort And Re-scope Triggers

Stop implementation and rewrite the plan if any of these become true:

1. chain3 support requires a generic graph executor instead of the current bounded closure collector
2. a chain3 root needs cross-library resolution to pass
3. molecule TypeScript execution becomes necessary to prove the feature
4. passport or export schemas need new fields
5. semantic review cannot classify the aligned fixture as `function.wrapper.pipeline.chain3.v1`
6. implementation starts changing family-analysis stop-state behavior

## Target End State

This command passes:

```bash
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec --target-language typescript
```

These still reject before Bun or before unsupported execution:

- cross-library chain3 deps
- wrong direct dep order
- wrong direct dep count
- unsupported direct dep family
- required closure member missing `body.typescript`
- unrelated loaded units leaking into emitted TypeScript
- `.test.spec --target-language typescript`
- seam-kind TypeScript targets
- generic four-dependency or otherwise out-of-family roots

## Architecture

### Admission Flow

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
    `-- everything else rejected before Bun

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

### Closure Emission Flow

```text
checkout_chain3_aligned
  |
  +-- pricing_total_wrapper_aligned
  |     |
  |     +-- pricing_discount_leaf_aligned
  |     |     |
  |     |     `-- helper closure, if authored and already supported
  |     |
  |     `-- pricing_tax_leaf_aligned
  |           |
  |           `-- helper closure, if authored and already supported
  |
  +-- pricing_tax_leaf_aligned
  |
  `-- pricing_discount_leaf_aligned

Emission rule:
  include each required unit once
  include runtime/build/test support modules
  exclude unrelated loaded units
  reject before Bun if any required member lacks valid TypeScript body
```

## Write Scope

Expected write scope:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`
- aligned chain3 fixture `.unit.spec` files
- negative chain3 fixture `.unit.spec` files only if a small authored reject case is required
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

This is a multi-file change, but it is still a bounded extension. No new crates, commands, services, or runtime dependencies should appear.

## Detailed Implementation Plan

### Phase 1: Validator Contract

File: `spec-core/src/validator.rs`

1. Add:

```rust
pub const TYPESCRIPT_CHAIN3_TARGET_COMPATIBILITY_KEY: &str =
    "function.wrapper.pipeline.chain3.v1";
```

2. Extend the root-family enum:

```rust
enum TypescriptTargetRootFamily {
    MonotoneUp,
    WrapperPipeline,
    Chain3WrapperPipeline,
}
```

3. Update `classify_typescript_target_root_family`:
   - include chain3 in the supported-key error message
   - return `Chain3WrapperPipeline` for `function.wrapper.pipeline.chain3.v1`
   - preserve unsupported semantic-review handling

4. Update `validate_typescript_execution_target_spec_with_specs`:
   - `0 | 1` direct deps only valid for `MonotoneUp`
   - `2` direct deps only valid for `WrapperPipeline`
   - `3` direct deps only valid for `Chain3WrapperPipeline`
   - any other arity rejects with explicit family-aware messaging

5. Add `validate_typescript_chain3_dep_contract`:
   - require exactly three direct deps
   - parse each dep with `DepRef::parse`
   - reject any dep with `library_alias`
   - require every dep to resolve in `specs_by_id`
   - evaluate each dep through `SemanticReviewContext::new(specs_by_id)`
   - require `support_status == supported`
   - require exact compatibility keys in exact order:
     - dep 1: `TYPESCRIPT_WRAPPER_TARGET_COMPATIBILITY_KEY`
     - dep 2: `TYPESCRIPT_MONOTONE_UP_TARGET_COMPATIBILITY_KEY`
     - dep 3: `TYPESCRIPT_WRAPPER_FIRST_DEP_COMPATIBILITY_KEY`
   - require non-empty `body.typescript` for every direct dep
   - emit M54-specific, position-specific error strings

6. Update `validate_typescript_closure_member_spec_with_specs`:
   - preserve helper closure behavior
   - preserve monotone-up closure behavior
   - allow `function.wrapper.pipeline.v1` as a closure member only when it passes the existing wrapper dep contract
   - reject chain3 as a nested closure member in M54

Definition of done for Phase 1:

- the validator admits only the exact chain3 family shape
- every out-of-family variant rejects before Bun with clear error text

### Phase 2: TypeScript Backend Closure

File: `spec-core/src/typescript_backend.rs`

1. Update the module comment so it is milestone-neutral or explicitly names both M52 and M54 truth:

```text
Bounded TypeScript backend generation for promoted same-tree function families.
```

2. Import the new chain3 compatibility key.

3. Update `collect_typescript_root_closure`:
   - detect chain3 root family
   - iterate exactly three direct deps
   - resolve each local dep
   - call `collect_typescript_closure_member` for each dep
   - rely on the validator for family shape and same-tree safety

4. Update `collect_typescript_closure_member`:
   - recurse through wrapper closure members so the chain3 wrapper pulls its leaf deps
   - preserve helper closure recursion for leaf members
   - keep `included` as a `BTreeSet` so repeated direct and nested deps emit once

5. Add a chain3 sibling to the existing wrapper-tree test:
   - root: `checkout_chain3_aligned`
   - required emitted units:
     - `pricing/checkout_chain3.ts`
     - `pricing/calculate_total.ts`
     - `pricing/apply_discount.ts`
     - `pricing/apply_tax.ts`
     - helper unit, only if used
     - runtime/build/local-test support files
   - unrelated loaded unit must not be emitted
   - relative imports must stay stable

Definition of done for Phase 2:

- the backend emits the exact same-tree chain3 closure and nothing else

### Phase 3: Fixture Truth

Files:

- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec`

Required work:

1. Add `body.typescript` to the aligned chain3 root.
2. Ensure every required aligned closure member has non-empty `body.typescript`.
3. Do not add new units just to make TypeScript generation easier.
4. Keep naming aligned with generated function aliases.

Root TypeScript body should mirror the Rust body:

```typescript
const base_total = pricing_total_wrapper_aligned(subtotal, discount_rate, tax_rate);
const surcharged_total = pricing_tax_leaf_aligned(base_total, surcharge_rate);
return pricing_discount_leaf_aligned(surcharged_total, loyalty_rate);
```

Definition of done for Phase 3:

- the aligned fixture contains enough authored TypeScript for the bounded executor to prove the feature honestly

### Phase 4: CLI Proof Wall

File: `spec-cli/tests/cli.rs`

1. Replace `typescript_chain3_wrapper_rejects_before_bun_runs` with an aligned pass proof:
   - copy the aligned chain3 fixture
   - run `spec test units/pricing/checkout_chain3_aligned.unit.spec --target-language typescript`
   - assert success
   - assert target-specific TypeScript proof surfaces if the surrounding helpers already expose that cleanly

2. Preserve or add negative CLI proofs:
   - molecule TypeScript rejection still happens before Bun
   - wrong direct dep order rejects before Bun
   - unsupported near-miss rejects before Bun
   - missing required `body.typescript` rejects before Bun
   - cross-library dep rejects before Bun, if the fixture mutation remains small and explicit

3. Do not add a generic "multi-dep TypeScript now works" proof.

Definition of done for Phase 4:

- the aligned chain3 path is green
- every out-of-contract chain3-like path still fails at the validator boundary

### Phase 5: Docs And Backlog

Files:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Required work:

1. `CHANGELOG.md`
   - add M54 note: bounded same-tree `function.wrapper.pipeline.chain3.v1` TypeScript execution
   - state explicitly that generic multi-dependency TypeScript remains unsupported

2. `README.md`
   - update the TypeScript target-language support list
   - add one chain3 command example only if it fits cleanly near existing examples

3. `TODOS.md`
   - keep cross-library helper imports deferred
   - keep generic multi-dependency TypeScript deferred
   - remove or rewrite any TODO that falsely says same-tree chain3 TypeScript is still missing

Definition of done for Phase 5:

- docs describe the exact boundary users can rely on, with no accidental generic claims

## Test Plan

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
- `typescript_tree_rejects_chain3_closure_member_missing_typescript_body`, only if backend still owns a meaningful branch after validator checks
- `typescript_tree_rejects_chain3_cross_library_dep_before_render`, only if backend still sees that path after validator checks

Preserve:

- zero-dep module rendering
- wrapper closure rendering
- helper import rendering

#### CLI Regression Tests

Add or update:

- `typescript_chain3_wrapper_executes_with_bun`
- `typescript_chain3_wrong_family_rejects_before_bun_runs`
- `typescript_chain3_missing_typescript_body_rejects_before_bun_runs`
- `typescript_chain3_wrong_dep_order_rejects_before_bun_runs`, if a small explicit fixture mutation is enough

Keep:

- `typescript_molecule_test_is_rejected_before_bun_runs`

### Code Path Coverage Diagram

```text
CODE PATH COVERAGE
==================
[+] spec-core/src/validator.rs
    |
    +-- classify_typescript_target_root_family()
    |   +-- monotone_up still accepted
    |   +-- wrapper still accepted
    |   +-- chain3 accepted only by exact compatibility key
    |   `-- unsupported family rejected with supported-key message
    |
    +-- validate_typescript_execution_target_spec_with_specs()
    |   +-- 0..1 deps valid only for monotone_up roots
    |   +-- 2 deps valid only for wrapper roots
    |   +-- 3 deps valid only for chain3 roots
    |   `-- 4+ deps rejected even if every dep has TypeScript
    |
    +-- validate_typescript_chain3_dep_contract()
    |   +-- exact tuple accepted
    |   +-- wrong dep order rejected
    |   +-- cross-library dep rejected
    |   +-- missing dep rejected
    |   +-- unsupported dep family rejected
    |   `-- missing dep body.typescript rejected
    |
    `-- validate_typescript_closure_member_spec_with_specs()
        +-- helper closure member still accepted
        +-- monotone_up closure member still accepted
        +-- wrapper closure member accepted inside chain3 closure
        `-- nested chain3 closure member rejected

[+] spec-core/src/typescript_backend.rs
    |
    +-- collect_typescript_root_closure()
    |   +-- monotone_up behavior unchanged
    |   +-- wrapper behavior unchanged
    |   `-- chain3 root collects exact deps recursively
    |
    +-- collect_typescript_closure_member()
    |   +-- wrapper member recursively collects its leaf deps
    |   +-- helper member terminates recursion
    |   `-- repeated direct and nested deps emit once
    |
    `-- render TypeScript tree
        +-- chain3 root imports wrapper, tax, discount
        +-- wrapper imports its leaves
        +-- unrelated loaded unit excluded
        `-- runtime/build/local-test support modules still emitted

[+] spec-cli/tests/cli.rs
    |
    +-- aligned chain3 target-language typescript
    |   `-- succeeds through Bun
    |
    +-- unsupported chain3-like target-language typescript
    |   +-- wrong family rejects before Bun
    |   +-- missing TypeScript body rejects before Bun
    |   `-- cross-library dep rejects before Bun
    |
    `-- molecule target-language typescript
        `-- remains rejected before Bun
```

### Regression Rule

This milestone changes existing behavior, not just net-new behavior. The aligned chain3 target currently rejects before Bun. That means the aligned green-path proof is a regression test requirement, not an optional nice-to-have.

## Failure Modes Registry

| # | Codepath | Failure mode | Planned guard |
|---|---|---|---|
| 1 | root family classification | dep-count logic accidentally admits unsupported families | exact compatibility-key gate plus wrong-family tests |
| 2 | direct dep tuple | wrong order produces plausible but incorrect runtime math | position-specific tuple validation plus negative tests |
| 3 | same-tree resolution | root pulls `shared::...` dep and generated imports cannot resolve | cross-library rejection before Bun |
| 4 | nested closure emission | wrapper emits without required leaf deps | backend closure-tree test |
| 5 | repeated deps | direct and nested leaves emit twice with unstable imports | `BTreeSet` dedupe plus tree assertions |
| 6 | missing TypeScript body | Rust-only closure member reaches generation or Bun | validator rejection before render |
| 7 | unrelated loaded units | generated tree implies broader support than intended | exact exclusion assertion |
| 8 | stale docs | users think generic multi-dep TypeScript is supported | README and CHANGELOG boundary wording |

Critical rule: any failure mode with no test and no clear pre-Bun error is a stop sign for merge.

## Performance Notes

No new runtime service or hot path is introduced. The risks are local:

- do not introduce an unbounded repeated semantic-review traversal if a scoped context or map lookup is enough
- closure collection must remain proportional to the required closure, not to every loaded unit for every dep
- keep `BTreeSet` dedupe in place
- keep CLI Bun tests targeted so `cargo test -p spec-cli` does not turn into a slow fixture matrix

## Worktree Parallelization Strategy

There is a real parallelization opportunity, but not at the start. The validator contract is the hinge. If multiple worktrees invent that contract independently, the merge pain will be self-inflicted.

### Dependency Table

| Step | Modules touched | Depends on |
|---|---|---|
| A. Validator contract | `spec-core/src/validator.rs` | none |
| B. TypeScript backend closure | `spec-core/src/typescript_backend.rs` | A |
| C. Chain3 fixture TypeScript bodies | `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/` | A |
| D. CLI proof wall | `spec-cli/tests/`, chain3 fixtures | A, B, C |
| E. Docs and backlog sync | repo docs | A, B, C, D |

### Parallel Lanes

```text
Lane 1
  A. Validator contract

Lane 2
  B. TypeScript backend closure

Lane 3
  C. Chain3 fixture TypeScript bodies

Lane 4
  D. CLI proof wall

Lane 5
  E. Docs and backlog sync
```

### Execution Order

1. Run Lane 1 first, alone.
2. After Lane 1 lands or is rebased cleanly, run Lane 2 and Lane 3 in parallel worktrees.
3. After Lanes 2 and 3 are green, run Lane 4.
4. Run Lane 5 last, after command names, proof surfaces, and exact rejection strings are stable.

### Conflict Flags

- Lane 1 and Lane 2 both touch `spec-core`. Keep them sequential.
- Lane 3 and Lane 4 both touch chain3 fixtures if CLI proofs mutate fixtures. Prefer authored fixture truth in Lane 3 and make Lane 4 read that truth instead of rewriting it.
- Lane 5 should wait. Stale docs are worse than missing docs for a short window.

### Parallelization Summary

- total workstreams: 5
- workstreams that can truly overlap: 2
- sequential gates: 3
- recommended launch pattern: `A` first, then `B + C`, then `D`, then `E`

If implementation discovers that Lane 4 needs to change validator contract or backend semantics, stop and fold that work back into the primary branch instead of pretending it is still an independent lane.

## Documentation Plan

Update docs only after tests are green.

Required outputs:

- `CHANGELOG.md` says chain3 same-tree TypeScript execution is supported
- `README.md` lists chain3 as supported and does not imply generic multi-dependency support
- `TODOS.md` continues to track the real deferred items:
  - cross-library TypeScript helper imports
  - generic multi-dependency TypeScript execution
  - any future broader portability work

## Validation Commands

Run during implementation:

```bash
cargo test -p spec-core typescript_target
cargo test -p spec-core typescript_tree
cargo test -p spec-cli typescript_chain3
```

Run before calling M54 done:

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

M54 is done when all of the following are true:

1. `spec test ...checkout_chain3_aligned.unit.spec --target-language typescript` passes.
2. The generated TypeScript tree includes only:
   - the chain3 root
   - the wrapper dep
   - the two leaf deps
   - required helpers, if authored and supported
   - runtime/build/local-test support modules
3. The generated TypeScript tree excludes unrelated loaded units.
4. The validator rejects:
   - cross-library deps
   - wrong dep order
   - wrong dep count
   - unsupported dep family
   - missing required `body.typescript`
   - nested chain3 closure members
   - generic out-of-family roots
5. Existing monotone-up and two-step wrapper TypeScript tests still pass.
6. Molecule TypeScript targets still reject before Bun.
7. `README.md`, `CHANGELOG.md`, and `TODOS.md` all describe the same exact boundary.

## Final Checklist

- [ ] Validator contract implemented exactly as specified
- [ ] Backend closure emission implemented exactly as specified
- [ ] Aligned chain3 fixtures contain maintained TypeScript bodies
- [ ] CLI aligned proof is green
- [ ] Negative CLI proofs are green
- [ ] Existing monotone-up and wrapper proofs remain green
- [ ] Docs and backlog language are synchronized
- [ ] No generic multi-dependency support slipped in by accident
