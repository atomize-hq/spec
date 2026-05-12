# M52: Bounded Same-Tree Wrapper TypeScript Execution Implementation Plan

Status: **implementation plan**
Milestone: **M52**
Milestone family: **second-language-backend**
Implementation readiness: **ready for bounded execution**
Plan scope: **extend the existing M46 TypeScript executor so `spec` can generate, build, test, and record TypeScript target proof for the supported same-tree `function.wrapper.pipeline.v1` family without widening to cross-library imports, chain3, molecule execution, or generic arbitrary multi-dependency execution**
Base branch: **main**
Working branch: **feat/m40-plus**
Last rewritten: **2026-05-12**

Supersedes:
- the prior repo-root M51 shared-core portability plan previously maintained at this path

Primary source artifacts:
- `docs/m52_bounded_same_tree_wrapper_typescript_execution_design_v0.1.md`
- `TODOS.md`
- `README.md`
- `CHANGELOG.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `~/.gstack/projects/atomize-hq-spec/checkpoints/20260506-181701-semantic-review-milestone-reset.md`

Primary repo surfaces:
- `spec-core/src/typescript_backend.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/semantic_review.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `examples/ecommerce/units/pricing/apply_discount.unit.spec`
- `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- `examples/ecommerce/units/pricing/calculate_total.unit.spec`
- `semantic-families/function.wrapper.pipeline.v1/candidate.md`

## Executive Summary

M46 proved that `spec` can run one bounded TypeScript lane.

It did not prove the first same-tree wrapper closure.

The repo already knows how to classify `function.wrapper.pipeline.v1` semantically, but the TypeScript execution path still rejects the canonical `pricing/calculate_total` wrapper before Bun runs because its direct deps are validated under the old monotone-up-root-plus-optional-helper contract.

M52 fixes exactly that mismatch.

This plan lands one honest widening:

1. admit `function.wrapper.pipeline.v1` as a TypeScript execution target
2. require its direct dependency closure to stay local to the same loaded unit set and generated tree
3. reuse the existing Bun runtime, CLI surfaces, and `target_proofs.typescript` storage
4. explicitly refuse chain3, cross-library TypeScript deps, molecule tests, seam kinds, and generic multi-dep execution

If implementation broadens from "supported wrapper family in one local tree" to "multiple deps are generally fine now," M52 failed.

## Live Validated Basis

Validated from the current tree on `feat/m40-plus` on 2026-05-12.

Commands:

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
  - passes today
- `spec test ... calculate_total.unit.spec --target-language typescript`
  - fails before Bun with:
  - `unit 'pricing/apply_discount' is not eligible for the bounded M46 TypeScript lane: body.typescript is required`

Observed code truth:

- `spec-core/src/validator.rs`
  - hard-codes the current M46 TypeScript target gate around `function.arithmetic_leaf.monotone_up.v1`
  - permits `deps: []` or exactly one direct local helper dep
  - rejects cross-library deps, missing local helper deps, and wrong helper family
- `spec-core/src/typescript_backend.rs`
  - validates every emitted TypeScript unit under the current target/helper-role assumptions
  - only renders zero-dep or one-helper import topology
- `spec-core/src/semantic_review.rs`
  - already supports:
  - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - `function.arithmetic_leaf.monotone_up.v1`
  - `function.helper.identity_passthrough.v1`
  - `function.wrapper.pipeline.v1`
  - `function.wrapper.pipeline.chain3.v1`
- `examples/ecommerce/units/pricing/calculate_total.unit.spec`
  - is the canonical same-tree wrapper target
- `examples/ecommerce/units/pricing/apply_discount.unit.spec`
  - currently lacks `body.typescript`
- `semantic-families/function.wrapper.pipeline.v1/candidate.md`
  - freezes the truthful wrapper family as a straight-line two-call wrapper over supported local deps

That is enough basis. Another family-choice or recommendation-governance milestone would be fake motion. The missing product truth is bounded wrapper execution in `spec`.

## Decision This Plan Makes

M52 authorizes exactly one milestone:

1. widen the TypeScript validator from "monotone-up leaf root only" to "supported wrapper pipeline root plus its exact same-tree local closure"
2. widen the TypeScript tree generator so wrapper roots may import both required direct local deps
3. keep the widening family-shaped, not dep-count-shaped
4. add authored `body.typescript` only where the canonical wrapper closure and maintained wrapper packet fixtures now require it
5. prove the widened contract through Rust unit tests, CLI integration tests, fixture truth, and target-proof evidence refresh

M52 does not authorize:

- cross-library TypeScript dependency resolution
- generic arbitrary multi-dependency TypeScript execution
- `function.wrapper.pipeline.chain3.v1` execution
- `.test.spec --target-language typescript`
- seam-kind TypeScript execution
- `spec validate --target-language`
- `spec export --target-language`
- new runtime/tooling beyond Bun
- shared-core extraction work
- renewed family-choice or corpus-program work

## Step 0: Scope Challenge

### What Already Exists

| Sub-problem | Existing owner | Reuse verdict |
| --- | --- | --- |
| TypeScript CLI target entry points | `spec-cli/src/commands.rs` | reuse |
| TypeScript runtime and generated helper modules | `spec-core/src/typescript_backend.rs` | reuse, extend narrowly |
| TypeScript target-proof storage | `spec-core/src/passport.rs` via `spec-cli/src/commands.rs` | reuse |
| supported wrapper family authority | `spec-core/src/semantic_review.rs` | reuse as the only semantic authority |
| canonical same-tree wrapper example | `examples/ecommerce/units/pricing/*` | reuse, widen honestly |
| maintained wrapper packet truth | `semantic-families/function.wrapper.pipeline.v1/**` | reuse, add bounded TypeScript parity |
| TypeScript CLI regression harness | `spec-cli/tests/cli.rs` | reuse and extend |
| validator/backend unit harnesses | `spec-core/src/validator.rs`, `spec-core/src/typescript_backend.rs` tests | reuse and extend |

### Minimum Complete Slice

The minimum honest slice is:

1. validator admits exactly one new TypeScript target family: `function.wrapper.pipeline.v1`
2. generator can emit the wrapper root and the exact same-tree direct-dep closure it needs
3. the canonical ecommerce wrapper closure authors the missing TypeScript bodies
4. maintained wrapper packet fixtures can prove the aligned bounded TypeScript lane
5. CLI/passport/status flows continue to write additive target-specific TypeScript proof
6. docs explain the widened lane without implying generic multi-dep support

Anything smaller is fake done.

### Scope Reduction Decision

Do not invent a generic closure planner.

Do not add a new executor abstraction just to support one family.

Do not refactor semantic review routing.

The smallest acceptable implementation is a bounded extension to the current validator and generator that introduces one new role model:

- target root
- helper dep
- wrapper closure member

That is enough.

### Complexity Check

This is a bounded lake, not an ocean.

Expected write scope:

- core executor surfaces
  - `spec-core/src/validator.rs`
  - `spec-core/src/typescript_backend.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
- authored truth surfaces
  - `examples/ecommerce/units/pricing/apply_discount.unit.spec`
  - `examples/ecommerce/units/pricing/apply_tax.unit.spec`
  - `examples/ecommerce/units/pricing/calculate_total.unit.spec`
  - `semantic-families/function.wrapper.pipeline.v1/**`
- docs
  - `README.md`
  - `CHANGELOG.md`
  - `TODOS.md`

No new service, no new crate, no new schema version, no new runtime manager, no new distribution pipeline.

### Search Check

No unfamiliar platform is entering the repo.

- **[Layer 1]** Reuse Bun, the existing CLI entry points, and the existing passport target-proof surface.
- **[Layer 1]** Reuse semantic review compatibility keys as the sole truth for what families are eligible.
- **[Layer 3]** The executor should widen only where the family contract is already strong enough to keep the claim honest.

### TODOS Cross-Reference

Relevant deferred items already exist and remain deferred:

- wrapper TypeScript execution in `spec` -> spent by M52
- cross-library TypeScript helper imports -> still deferred
- generic multi-dependency TypeScript execution -> still deferred
- molecule TypeScript execution -> still deferred
- seam-kind TypeScript execution -> still deferred

M52 should close the wrapper-execution TODO and leave the rest untouched.

### Completeness Check

Choose the complete version.

Land executor truth, authored truth, fixture truth, CLI proof truth, and docs truth in the same milestone. Do not ship the shortcut where the validator widens but the canonical example still cannot prove the new contract, or where the example passes but maintained packet truth and docs still lie.

### Distribution Check

No new distributable artifact is introduced in M52.

This is a behavior change inside the existing `spec` binary. Existing build and release pipelines are sufficient. Nothing new needs to be published beyond the repo and normal binary release flow.

### Locked Plan Decisions

These are frozen for M52:

1. The widened TypeScript executor remains family-shaped.
2. `function.wrapper.pipeline.v1` is in scope.
3. `function.wrapper.pipeline.chain3.v1` is out of scope.
4. Cross-library TypeScript dep resolution is out of scope.
5. `.test.spec --target-language typescript` remains unsupported.
6. Rust remains the default target. TypeScript proof stays additive.
7. Bun remains the only TypeScript runtime/tooling contract.
8. No new `validate` or `export` target-language surface is added.

### Abort And Re-scope Triggers

Stop and re-scope if any of these become necessary:

1. the executor needs cross-library dep loading or alias resolution
2. the rule must widen to arbitrary supported multi-dep graphs to make the canonical wrapper pass
3. chain3 turns out to be required for the first honest wrapper proof
4. passport or status storage needs a schema redesign rather than reusing `target_proofs.typescript`
5. the runtime contract needs `npm`, `package.json`, `tsconfig.json`, or alternate toolchain ownership beyond Bun

## Target End State

After M52, the repo must tell one consistent story:

- TypeScript execution in `spec` supports:
  - monotone-up leaf roots
  - the supported same-tree wrapper pipeline family
- wrapper execution means:
  - exactly two direct local deps
  - same loaded unit set
  - same generated tree
  - exact dep-family tuple:
    - first dep: `function.arithmetic_leaf.monotone_down_nonnegative.v1`
    - second dep: `function.arithmetic_leaf.monotone_up.v1`
- the canonical ecommerce wrapper example proves that contract
- maintained wrapper packet fixtures prove the same contract
- docs say exactly that, and no more

## Architecture Review

### Architecture Delta

```text
CURRENT M46
  TS target root = monotone_up leaf
    -> deps: [] or one local helper dep
    -> generate root/helper modules
    -> bun build
    -> bun local tests
    -> write target_proofs.typescript

M52
  TS target root = monotone_up leaf OR wrapper.pipeline.v1
    -> if wrapper root:
       -> exactly two direct local deps
       -> both deps already classify to the supported local families
       -> include wrapper + exact direct closure in generated tree
       -> bun build
       -> bun local tests
       -> write additive target_proofs.typescript

STILL OUT
  chain3
  cross-library deps
  molecule tests
  seam kinds
  generic supported multi-dep graphs
```

### Component Boundaries

`spec-core/src/validator.rs`
- owns target eligibility and rejection reasons
- must determine whether a root spec is:
  - current M46 monotone-up lane
  - new M52 wrapper lane
  - unsupported

`spec-core/src/typescript_backend.rs`
- owns tree membership and import generation
- must stop assuming every included unit is either a root or a helper
- must render wrapper roots with two direct dep imports
- must allow closure-member units whose role is "validated local dep of a validated wrapper root"

`spec-cli/src/commands.rs`
- owns `generate/build/test/status` TypeScript plumbing
- must keep additive proof routing unchanged
- must not create new target-language command surfaces

`spec-cli/tests/cli.rs`
- owns the end-to-end proof wall
- must prove success for the canonical wrapper path and rejection for bounded non-goals

### Execution Contract

#### Validator contract

TypeScript target eligibility branches by supported compatibility key:

- `function.arithmetic_leaf.monotone_up.v1`
  - keep the M46 rule
- `function.wrapper.pipeline.v1`
  - require exactly two direct deps
  - both deps must be local to the same loaded unit set
  - both deps must already exist in the validated spec context
  - dep 1 must classify to `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - dep 2 must classify to `function.arithmetic_leaf.monotone_up.v1`
  - both deps must author non-empty `body.typescript`
  - the wrapper root must author non-empty `body.typescript`
  - local test expectations remain inside the bounded translated grammar

The validator must reject:

- cross-library deps
- missing local deps
- wrong dep arity
- wrong dep family combination
- chain3 roots
- any other "supported somehow" ambiguity

#### Tree-generation contract

The generator must distinguish three roles:

1. monotone-up target root
2. helper dep for a monotone-up root
3. wrapper closure member for a wrapper root

It must not widen to arbitrary graph walking.

For wrapper roots, emit:

- wrapper module
- direct dep module A
- direct dep module B
- shared runtime files already frozen in M46
- local test harness importing the wrapper root and closure members as needed

It must not emit unrelated same-tree units just because they were loaded.

#### CLI and proof-routing contract

`spec generate/build/test/status --target-language typescript`
- remain the only widened surfaces
- keep Rust proof and TypeScript proof distinct
- continue writing `target_proofs.typescript`

`spec validate --target-language`
- remains unsupported

`spec export --target-language`
- remains unsupported

`.test.spec --target-language typescript`
- remains rejected before Bun runs

#### Security and blast-radius contract

No new external execution surface is added. The blast radius is local:

- validator behavior
- generated TypeScript import topology
- bounded Bun invocation
- additive proof refresh

The worst-case failure is a false-positive widening to unsupported graphs. That is why every rule in M52 stays family-based and explicitly negative on everything else.

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

Do not touch unless a blocker proves this plan wrong:

- cross-library example libraries
- seam portability code
- semantic-family packets other than `function.wrapper.pipeline.v1`
- recommendation-policy and shared-core planning docs
- CLI export/validate schema machinery

## Code Quality Review

### Guardrails

1. Do not encode M52 as "allow two deps now."
2. Do not introduce a generic multi-role dependency planner.
3. Prefer one explicit helper for role classification over a new abstraction layer.
4. Reuse existing constants or add narrowly named M52-specific constants where current M46 messages would become false.
5. Keep the diff minimal. The goal is an honest widening, not a cleanup campaign.

### Naming and structure requirements

- If new constants are introduced, they must describe the bounded wrapper lane explicitly.
- If validator logic branches by compatibility key, keep that logic close to current TypeScript-specific validation instead of scattering it through generic semantic code.
- If generator logic needs new helpers, keep them inside `typescript_backend.rs` unless two or more modules genuinely need them.

### Diagram maintenance

No nearby code ASCII diagrams appear to require sync today, but if any inline comment diagrams are introduced during implementation, they become part of the change and must stay accurate.

## Implementation Plan

### Step 1: Freeze validator behavior

Update `spec-core/src/validator.rs` so TypeScript eligibility is explicit for:

- monotone-up leaf lane
- helper-aware monotone-up lane
- wrapper same-tree lane

Acceptance:

- wrapper roots are accepted only when the exact M52 contract holds
- chain3, cross-library, wrong-family, wrong-arity, and missing-dep paths reject with stable TypeScript-specific errors
- `.test.spec --target-language typescript` remains rejected before Bun runs

### Step 2: Widen TypeScript tree generation

Update `spec-core/src/typescript_backend.rs` so it can:

- validate wrapper roots and closure members under the correct role
- render two direct imports for wrapper roots
- include only the exact validated local closure
- keep the existing runtime filenames and local test harness structure

Acceptance:

- `pricing/calculate_total` no longer fails because `pricing/apply_discount` is validated under the wrong role
- unrelated same-tree units are not emitted
- monotone-up helper behavior does not regress

### Step 3: Preserve CLI and target-proof behavior

Update `spec-cli/src/commands.rs` only as needed to keep the widened tree flowing through:

- `generate`
- `build`
- `test`
- `status`

Acceptance:

- additive `target_proofs.typescript` still refreshes without replacing Rust proof
- no new target-language command surfaces are introduced
- single-unit TypeScript runs remain the primary proof path

### Step 4: Author the missing TypeScript truth

Update:

- canonical ecommerce wrapper closure
- maintained wrapper packet fixtures

Required authored changes:

- add `body.typescript` where the widened executor now depends on it
- keep authored TypeScript semantically aligned with existing Rust truth
- add targeted negative fixtures only if validator or CLI rejection coverage truly needs them

Acceptance:

- canonical ecommerce wrapper closure proves the new lane honestly
- aligned wrapper packet can prove the same lane

### Step 5: Align docs and backlog wording

Update:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Acceptance:

- README describes the M52 lane as wrapper-family same-tree execution only
- CHANGELOG records the widening without overstating parity
- TODOS removes the spent wrapper execution deferral and leaves later TypeScript wedges deferred

## Test Review

### Test framework and proof surfaces

Primary frameworks already in the repo:

- Rust unit tests in `spec-core`
- Rust CLI integration tests in `spec-cli/tests/cli.rs`
- end-to-end Bun proof exercised through `spec test --target-language typescript`

M52 should extend existing tests. No new test framework is needed.

### Code Path Coverage

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/validator.rs
    │
    ├── current monotone-up target eligibility
    │   └── [EXISTING] covered today
    │
    ├── wrapper root eligibility
    │   ├── [GAP] accept exact same-tree wrapper family
    │   ├── [GAP] reject wrong dep arity
    │   ├── [GAP] reject cross-library dep
    │   ├── [GAP] reject missing local dep
    │   ├── [GAP] reject wrong dep family combination
    │   └── [GAP] reject chain3 root
    │
    └── molecule target rejection
        └── [EXISTING] must stay green

[+] spec-core/src/typescript_backend.rs
    │
    ├── monotone-up root tree emission
    │   └── [EXISTING] covered today
    │
    ├── wrapper root tree emission
    │   ├── [GAP] render two direct dep imports
    │   ├── [GAP] allow wrapper closure-member role
    │   └── [GAP] exclude unrelated loaded units
    │
    └── helper role emission
        └── [EXISTING] must not regress

[+] spec-cli/tests/cli.rs
    │
    ├── canonical apply_tax TypeScript success
    │   └── [EXISTING] covered today
    │
    ├── canonical calculate_total TypeScript success
    │   └── [GAP] new critical regression test
    │
    ├── wrapper fixture aligned TypeScript success
    │   └── [GAP] new end-to-end fixture test
    │
    ├── target-specific status/passport proof after wrapper run
    │   └── [GAP] prove additive refresh for wrapper root
    │
    ├── wrapper rejection before Bun
    │   ├── [GAP] wrong-family or malformed-closure rejection
    │   └── [EXISTING PATTERN] near-miss-before-Bun harness can be reused
    │
    └── molecule rejection before Bun
        └── [EXISTING] must stay green
```

### User and operator flow coverage

```text
USER / OPERATOR FLOW COVERAGE
===========================
[+] Maintainer runs canonical wrapper proof
    spec test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript
    ├── [GAP] must pass end-to-end
    └── [GAP] must refresh target_proofs.typescript on the wrapper passport

[+] Maintainer runs wrapper packet proof
    spec test <aligned wrapper fixture> --target-language typescript
    ├── [GAP] aligned packet path must pass
    └── [GAP] near-miss packet path must reject before Bun if outside M52 contract

[+] Maintainer runs mixed status after wrapper proof
    spec status <unit-or-root> --target-language typescript --format json
    ├── [GAP] wrapper root should report valid when freshly proven
    └── [EXISTING PATTERN] unrelated unproven units may still keep root-level mixed status non-green
```

### Required test additions

Add or extend tests for:

1. validator acceptance of an exact same-tree wrapper root
2. validator rejection of:
   - wrong dep arity
   - cross-library dep
   - missing local dep
   - wrong dep family combination
   - chain3 root
3. backend tree generation:
   - wrapper root imports both direct deps
   - closure members are allowed under the correct role
   - unrelated loaded units are not emitted
4. CLI end-to-end:
   - `pricing/calculate_total.unit.spec --target-language typescript` succeeds
   - aligned wrapper fixture succeeds
   - wrapper rejection happens before Bun for one bounded non-goal
   - wrapper target-specific status/passport proof refreshes additively

### Regression rule

The canonical regression is already known:

- `apply_tax` succeeds today
- `calculate_total` fails today for the wrong reason

M52 must add a regression test for that exact failure-to-success transition. No skipping. That is the proof that the milestone actually fixed the product bug.

## Failure Modes Registry

| Failure mode | Test required | Error handling required | User-visible outcome | Critical gap if missing |
| --- | --- | --- | --- | --- |
| validator widens to arbitrary multi-dep graphs | yes | yes | silent over-claim if absent | yes |
| cross-library dep slips through wrapper lane | yes | yes | false support claim | yes |
| wrapper dep lacks `body.typescript` | yes | yes | pre-Bun failure should be clear | yes |
| generator emits unrelated same-tree units | yes | no special runtime handling | false "tree-wide TS support" claim | yes |
| chain3 root accidentally becomes eligible | yes | yes | scope leak | yes |
| target_proofs.typescript overwrites Rust proof | yes | yes | proof corruption | yes |
| docs claim generic multi-dep support | no code test, doc review required | n/a | user learns the wrong contract | yes |

Any failure mode with no test, no bounded rejection, and a silent scope leak is a release blocker for M52.

## Performance Review

Expected performance impact is small, but still review it.

Likely hotspots:

- repeated semantic-review lookup while validating wrapper closure members
- extra tree-membership checks during TypeScript generation

Requirements:

1. keep closure walking bounded to the exact direct-dep slice
2. do not introduce whole-graph traversal for every TypeScript unit
3. reuse existing context maps where possible instead of rebuilding them per unit

There is no caching milestone here. The right answer is bounded work, not more machinery.

## Worktree Parallelization Strategy

Parallelization is available after the execution contract is frozen.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| contract freeze | `spec-core/src/validator.rs`, `spec-core/src/typescript_backend.rs`, `spec-cli/src/commands.rs`, plan docs | — |
| executor widening | `spec-core/`, `spec-cli/` | contract freeze |
| authored truth widening | `examples/ecommerce/`, `semantic-families/function.wrapper.pipeline.v1/`, docs | contract freeze |
| integration proof | repo root, generated outputs, passports | executor widening, authored truth widening |

### Parallel lanes

Lane A: contract freeze -> executor widening -> integration

Lane B: contract freeze -> authored truth widening -> integration

Lane A and Lane B can run in parallel only after the parent freezes:

- exact validator contract
- exact generator tree-membership contract
- exact file ownership

### Execution order

1. Parent freezes contract and file ownership in the main worktree.
2. Launch Lane A and Lane B in parallel worktrees.
3. Merge Lane A first because it defines the executable contract.
4. Rebase or merge Lane B on top.
5. Run authoritative integrated proof in the parent or integration worktree.

### Conflict flags

- Lane A and Lane B must not both edit `spec-cli/src/commands.rs`.
- Lane B docs edits must not restate executor rules differently than Lane A implements.
- If wrapper packet fixture tests require new CLI harness helpers, that helper belongs to Lane A unless it is pure fixture data.

## Execution Order

1. Freeze the wrapper eligibility contract in validator/backend terms.
2. Implement validator support for bounded wrapper roots.
3. Implement backend tree-emission support for wrapper closure members.
4. Adjust CLI plumbing only where required for the widened tree.
5. Add authored TypeScript bodies to the canonical wrapper closure and maintained wrapper packet fixtures.
6. Add end-to-end proof coverage for canonical success, fixture success, and bounded rejection.
7. Update README, CHANGELOG, and TODO wording to match the landed contract.

## Proof Floor

Required code-level proof:

- validator regressions for wrapper target eligibility
- backend regressions for wrapper closure emission
- CLI end-to-end success on the canonical wrapper unit
- CLI end-to-end success on the aligned wrapper packet
- CLI rejection coverage for at least one out-of-scope wrapper path
- target-specific status/passport proof coverage for a wrapper root

Required product-level proof:

```bash
cargo test -p spec-core typescript
cargo test -p spec-cli wrapper -- --nocapture
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript
```

Optional final read-side check:

```bash
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
```

## Success Criteria

M52 is done only when all of these are true:

- `spec test examples/ecommerce/units/pricing/calculate_total.unit.spec --target-language typescript` passes
- the aligned wrapper packet can prove TypeScript execution through `spec`, not just semantic review
- wrapper execution still rejects cross-library and generic arbitrary multi-dep widening
- chain3 still rejects clearly
- `target_proofs.typescript` remains additive and target-specific
- the generated TypeScript tree includes only the validated local closure
- README explains the widened TypeScript lane as wrapper-family same-tree execution only

## NOT in scope

Deferred explicitly:

- chain3 TypeScript execution
- cross-library TypeScript helper imports
- generic multi-dependency TypeScript execution
- molecule TypeScript execution
- seam-kind TypeScript execution
- `spec validate --target-language`
- `spec export --target-language`

Those are later milestones. M52 should not quietly spend them.

## Completion Summary

- Step 0: Scope Challenge -> scope accepted as a bounded family-shaped widening
- Architecture Review -> contract is explicit across validator, backend, CLI, proof, and docs
- Code Quality Review -> no new abstraction layer authorized
- Test Review -> regression matrix and required proof floor defined
- Performance Review -> bounded direct-closure work only
- NOT in scope -> written
- What already exists -> written
- Failure modes -> critical gaps identified up front
- Parallelization -> 2 lanes after contract freeze
- Lake Score -> complete option chosen across executor, fixtures, proof, and docs
