<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260510-162051.md -->
# M46 - Make Helper-Aware Monotone-Up TypeScript Execution Real

Status: **authority plan candidate**
Milestone family: **second-language-backend**
Implementation readiness: **ready-now**
Next artifact kind: **authority_plan**
Autoplan ready: **yes**
Base branch: **main**
Working branch: **feat/m40-plus**
Last rewritten: **2026-05-10**
Primary sources:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260510-162051.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-163500.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m45_bounded_typescript_lane/closeout.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/README.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/typescript_backend.rs`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs`
Supersedes: **M45 - Make TypeScript Real For One Bounded Monotone-Up Lane**
Related test artifact:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-163500.md`

## Executive Verdict

M45 made one honest TypeScript lane real, but only for the degenerate `deps: []` version of monotone-up.

M46 closes the next actual product-truth gap and nothing more:

- keep `kind:function` only
- keep Bun only
- keep atom tests only
- keep `function.arithmetic_leaf.monotone_up.v1` only
- expand from `deps: []` to `deps: [] | [one helper dep]`
- require that helper dep to classify as `function.helper.identity_passthrough.v1`
- require the helper unit to exist in the same generated output tree
- keep wrapper execution, molecule execution, multi-dep execution, seam kinds, and cross-library TypeScript resolution out of scope

This is not broader TypeScript parity. It is one bounded follow-on that makes the shipped product line up with repo truth the repo already claims.

## Product Truth Gap

Right now the repo says two incompatible things:

- the TypeScript execution lane exists for monotone-up units, but only for `deps: []`
- the promoted monotone-up family and semantic-review truth already treat the optional `money/round` helper shape as a real supported shape

That means the semantic family contract says "this unit shape is real" while the product says "you still cannot run it in the TypeScript lane."

That mismatch is the milestone. Fix the product. Do not widen the milestone.

## Repo Truth Basis

### Live code surfaces

- `spec-core/src/validator.rs` still hard-rejects dependency-bearing TypeScript targets with the frozen M45 `deps: []` rule.
- `spec-core/src/typescript_backend.rs` is explicitly built around the zero-dep monotone-up lane.
- `spec-core/src/semantic_review.rs` already supports helper units under `function.helper.identity_passthrough.v1`.
- `spec-cli/src/commands.rs` still contains legacy CLI-side TypeScript rendering helpers that should not remain a second source of generator truth.
- `README.md` still documents the TypeScript lane as `deps: []` only.
- `semantic-families/README.md` already documents the monotone-up family as allowing the optional helper shape.
- `examples/ecommerce/units/pricing/apply_tax.unit.spec` still has no helper dep today, so the current canonical example does not exercise the new topology.

### Branch truth

- branch anchor is `feat/m40-plus`
- M45 landed at `ce0e16d`
- M45 closeout already identified dead CLI-side TS helper code as real cleanup debt
- the next honest move is backend truth, not more family-selection work

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing owner | M46 action |
|---|---|---|
| target-language routing | `spec-core/src/types.rs`, `spec-cli/src/commands.rs` | preserve |
| Bun build/test execution | `spec-core/src/pipeline.rs` | preserve |
| additive target-proof storage | `spec-core/src/passport.rs`, `spec-core/src/export.rs` | preserve |
| monotone-up family routing | `spec-core/src/semantic_review.rs` | preserve |
| helper-family routing | `spec-core/src/semantic_review.rs` | reuse as eligibility gate |
| bounded TypeScript generation | `spec-core/src/typescript_backend.rs` | extend from zero-dep to one-helper topology |
| direct-dep validation | `spec-core/src/validator.rs` | replace blanket ban with bounded helper-aware rule |
| legacy CLI-side TS rendering helpers | `spec-cli/src/commands.rs` | remove or reduce to thin routing |

### Minimum complete change

M46 is complete only if all of this lands together:

1. TypeScript target eligibility allows zero deps or exactly one direct helper dep.
2. The allowed helper dep is proven semantically through `function.helper.identity_passthrough.v1`, not by string name alone.
3. TypeScript generation emits helper imports and a truthful build/test entry path for the one-helper topology.
4. TypeScript atom tests execute a helper-aware monotone-up unit end to end.
5. Passports, status, and export keep target-specific proof separation unchanged.
6. The old CLI-side TypeScript generator path stops being a second truth surface.
7. The proof wall is refreshed so a real helper-aware unit is exercised, not just theorized.
8. README and CHANGELOG describe the bounded lane exactly, with no wrapper or multi-dep overclaim.

If any of those is missing, M46 either stays dishonest or widens into more than one lake.

### Complexity check

This milestone touches multiple files, but it is still the smallest honest end-to-end slice because all touched files sit on one already-shipped seam:

- validator gate
- TypeScript backend generation
- CLI orchestration
- proof persistence
- proof wall tests
- docs

The overbuilt versions are:

- generic dep-graph scheduling
- wrapper execution
- cross-library TS resolution
- multi-helper support
- any new backend abstraction layer

Reject all of them in M46.

### Completeness check

The complete bounded version is still cheap enough to do now:

- one dep topology
- one helper-family gate
- one import-resolution path
- one atom-test expansion
- one generator-owner cleanup
- one proof wall refresh

The shortcuts are bad shortcuts:

- allow one dep without helper-family validation
- allow helper execution but leave duplicate generator ownership alive
- update docs without updating the example or fixtures
- ship happy path only without pre-Bun negative coverage

Do the complete bounded version.

## Locked Decisions

### 1. M46 stays monotone-up only

M46 extends only `function.arithmetic_leaf.monotone_up.v1`.

It does not add `function.wrapper.pipeline.v1` execution in `spec`.

### 2. Helper topology is exactly zero deps or one direct dep

Eligible TypeScript units may have:

- `deps: []`, or
- exactly one direct dep

More than one dep remains out of scope.

### 3. The one direct dep must classify as helper passthrough truth

When one direct dep exists, the dep unit must classify to:

- `function.helper.identity_passthrough.v1`

Do not special-case `money/round` by raw id. Consume semantic-review truth.

### 4. Helper support is same-generated-tree only

M46 supports helper imports only when the helper unit is present in the same loaded unit set and generated output tree.

It does not add cross-library TypeScript import resolution.

### 5. The TypeScript test surface stays atom-only

`.test.spec` remains unsupported for `--target-language typescript`.

M46 only expands local atom-test execution enough to cover a monotone-up unit that calls one helper.

### 6. One backend owns TypeScript generation truth

`spec-core/src/typescript_backend.rs` becomes the only generator source of truth for the TypeScript tree.

`spec-cli/src/commands.rs` keeps orchestration and flag routing only.

## Architecture Contract

### Current to target flow

```text
M45
  spec test --target-language typescript
    -> reject deps
    -> run only zero-dep monotone-up units

M46
  spec test --target-language typescript
    -> validate kind:function
    -> validate monotone-up family
    -> allow deps == 0 or deps == 1
    -> if deps == 1, require helper family support
    -> require helper unit present in same generated tree
    -> generate helper import edge in TS output
    -> bun build
    -> bun local_tests
    -> write target_proofs.typescript only
```

### Ownership table

| Module | Owns after M46 | Must not own |
|---|---|---|
| `spec-core/src/validator.rs` | helper-aware TS eligibility, dep-count rule, helper-family gate, missing-helper preflight | TS file emission |
| `spec-core/src/typescript_backend.rs` | helper-aware import generation, build entry wiring, local-test harness emission | Bun execution policy |
| `spec-core/src/pipeline.rs` | Bun build/test runners | family eligibility policy |
| `spec-cli/src/commands.rs` | target-language routing, command orchestration, error surfacing | duplicate TS generator/runtime helpers |
| `spec-core/src/passport.rs` | additive target-proof persistence | CLI wording |
| `spec-core/src/export.rs` | additive proof projection | target selection |

### Non-negotiable invariants

- Rust remains the default target everywhere.
- `.test.spec` remains unsupported for TypeScript.
- M46 never promotes wrapper execution implicitly.
- M46 never adds multi-dep or generic dep-graph execution.
- TypeScript proof never overwrites Rust proof.
- The helper dep must be semantically supported, not just syntactically present.

## File-By-File Implementation Contract

| File | Required change | Done when |
|---|---|---|
| `spec-core/src/validator.rs` | replace blanket `deps: []` rejection with zero-or-one-helper rule, helper-family gate, missing-helper-in-tree rejection, updated molecule-wrapper rejection wording | TypeScript eligibility errors are topology-aware and fail before Bun |
| `spec-core/src/typescript_backend.rs` | emit helper import edges, include helper module in generated tree, keep local test harness truthful for helper-aware units | helper-aware monotone-up unit builds and tests in TS lane |
| `spec-cli/src/commands.rs` | route entirely through backend-owned TS generation and remove or reduce dead helper-generation code | no duplicate TypeScript generator ownership remains |
| `spec-core/src/pipeline.rs` | no semantic widening, only coverage or plumbing changes if needed | Bun path stays stable |
| `spec-core/src/passport.rs` | preserve target-proof separation under helper-aware runs | Rust proof remains untouched after TS execution |
| `spec-core/src/export.rs` | preserve additive proof export behavior | export remains honest after TS helper-aware run |
| `spec-cli/tests/cli.rs` | add helper-aware positive path plus pre-Bun negative topology coverage | product surface is locked by integration coverage |
| `examples/ecommerce/units/pricing/apply_tax.unit.spec` or packet fixture set | refresh at least one canonical proof source to actually use the helper topology | proof wall exercises the new lane for real |
| `README.md` | update bounded-lane docs from `deps: []` to helper-aware monotone-up | user-facing truth matches product |
| `CHANGELOG.md` | record the exact widened boundary and retained exclusions | release truth is explicit |

## Ordered Implementation Plan

### Step 1. Lock the validator contract first

Change `spec-core/src/validator.rs` so the TypeScript gate enforces exactly this rule:

- zero deps is still valid
- one dep is valid only if that dep classifies to `function.helper.identity_passthrough.v1`
- one dep is invalid if the helper is absent from the loaded unit set
- two or more deps are invalid
- wrapper and molecule targets remain invalid

Do not start by editing docs or tests first. Freeze the policy surface first.

### Step 2. Extend the TypeScript backend, not the CLI

Change `spec-core/src/typescript_backend.rs` so the generated TS tree can compile and execute one helper-aware monotone-up unit.

That means:

- emit the helper module into the generated tree
- emit the correct relative import edge
- preserve the current local-test entrypoint model
- keep the implementation topology-specific, not generic

Do not add generic scheduling or dependency-graph machinery.

### Step 3. Collapse duplicate generator ownership

Change `spec-cli/src/commands.rs` so it stops owning a second TypeScript rendering path.

After M46, command code should route and report. Backend code should generate.

### Step 4. Refresh the proof wall

Refresh one real proof source so the helper topology is exercised end to end:

- canonical ecommerce example, or
- semantic-family packet fixtures, or
- both if needed for coverage symmetry

The milestone is not done if the code supports helper-aware execution but the checked proof surfaces never exercise it.

### Step 5. Add positive and negative product coverage

Update `spec-cli/tests/cli.rs` and any targeted library tests so the lane is locked on both sides:

- one positive helper-aware monotone-up execution case
- wrong helper family rejection
- missing helper from loaded tree rejection
- dep count > 1 rejection
- molecule rejection still intact
- Rust/TS proof separation still intact

### Step 6. Move docs with the product

Update `README.md` and `CHANGELOG.md` only after the code and proof wall are truthful.

Docs must say exactly what the lane supports and exactly what it still rejects.

## Test Review

### Coverage diagram

```text
CLI target-language parse
  -> wrong command/flag                                 [existing]
  -> typescript target selected
       -> kind != function                             [existing]
       -> family != monotone_up                        [existing]
       -> dep count > 1                                [GAP]
       -> dep count == 1, wrong helper family          [GAP]
       -> dep count == 1, helper missing from tree     [GAP]
       -> molecule target                              [existing]
       -> helper-aware monotone-up unit
            -> helper import edge emitted correctly    [GAP]
            -> bun build passes                        [GAP]
            -> local atom tests pass                   [GAP]
            -> target_proofs.typescript refreshed      [expand existing]
            -> rust proof remains untouched            [existing]
            -> status reads TS proof only              [expand existing]
            -> export preserves additive truth         [existing]
```

### Proof wall

```bash
cargo test
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_plus_tax.test.spec --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units --target-language typescript
cargo run -p spec-cli -- export examples/ecommerce/units --format json
```

### Expected proof-wall outcomes

- helper-aware monotone-up unit passes in the TypeScript lane when its helper unit is present in the same generated tree
- `spec status --target-language typescript` still reads `target_proofs.typescript` only
- `.test.spec` still fails before Bun runs for TypeScript
- unsupported helper topologies fail before Bun runs with stable messages
- export keeps additive target-proof truth without merge bugs

### Required new tests

#### Integration

- add one end-to-end CLI success test for helper-aware monotone-up execution
- add one CLI test for wrong helper family pre-Bun rejection
- add one CLI test for missing helper from loaded tree pre-Bun rejection
- add one CLI test for dep count > 1 pre-Bun rejection
- expand stale-status coverage so helper-aware TS proof goes stale when either the unit or helper changes

#### Library / module tests

- add targeted validator coverage for the zero-or-one-helper rule
- add targeted TS backend coverage for helper import path emission
- preserve existing proof-separation coverage in `passport.rs` and `export.rs`

### Test plan artifact

Primary QA artifact stays:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-163500.md`

That artifact already captures the exact QA-facing interactions and edge cases. M46 should stay aligned to it, not invent a parallel QA story.

## Failure Modes Registry

| Codepath | Realistic failure mode | Test required? | Error handling required? | User-visible outcome required? |
|---|---|---|---|---|
| helper-aware validator gate | accepts wrong helper family | yes | yes, pre-Bun failure | clear error |
| helper-aware validator gate | accepts helper ref absent from loaded tree | yes | yes, pre-Bun failure | clear error |
| helper-aware validator gate | silently accepts dep count > 1 | yes | yes, pre-Bun failure | clear error |
| TS backend import generation | emits broken relative import | yes | build failure path already exists | clear build failure |
| TS local test harness | helper-aware unit compiles but tests run against incomplete tree | yes | yes, generate full tree | clear failure |
| CLI/backend split ownership | CLI helper path diverges from backend path | yes | fixed by consolidation | n/a after cleanup |
| proof persistence | TS run overwrites Rust proof | yes, preserve existing coverage | existing additive storage must remain | silent corruption must be impossible |
| canonical example / fixtures | proof wall never exercises helper topology | yes | fix by refreshing proof source | no fake green |

**Critical gap rule:** any failure mode that has no test, no explicit failure path, and can produce silent success is a release blocker for M46.

## Worktree Parallelization Strategy

This plan has limited but real parallelization opportunity. The core backend changes share `spec-core/src` and `spec-cli/src`, so most runtime work is sequential. The proof assets and doc updates can be prepared in parallel once the validator contract is frozen.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| lock TS helper-aware validator contract | `spec-core/src`, `spec-core semantic review surfaces` | — |
| extend TS backend generation and harness | `spec-core/src` | lock TS helper-aware validator contract |
| consolidate CLI generator ownership and preserve proof surfaces | `spec-cli/src`, `spec-core/src` | extend TS backend generation and harness |
| refresh examples, packet fixtures, and CLI integration tests | `examples/`, `semantic-families/`, `spec-cli/tests/` | lock TS helper-aware validator contract |
| finalize docs and changelog | `README.md`, `CHANGELOG.md` | consolidate CLI generator ownership and preserve proof surfaces |

### Parallel lanes

Lane A: lock TS helper-aware validator contract -> extend TS backend generation and harness -> consolidate CLI generator ownership and preserve proof surfaces

Lane B: refresh examples, packet fixtures, and CLI integration tests

Lane C: finalize docs and changelog

### Execution order

1. Launch Lane A first. The validator contract is the dependency anchor for everything else.
2. After Step 1 in Lane A is frozen, launch Lane B in a parallel worktree. That work can author the helper-aware example, fixtures, and most integration coverage while Lane A finishes backend and CLI ownership work.
3. Merge Lane A first.
4. Rebase Lane B on top of Lane A, then fix any error-message or path assertion drift.
5. Run the proof wall.
6. Launch Lane C only after proof-wall behavior is final, then merge docs/changelog last.

### Conflict flags

- Lane A and Lane B both depend on exact validator wording. Expect assertion churn if Lane B starts before Step 1 is frozen.
- Lane A and Lane B both indirectly touch TypeScript-path expectations. Rebase Lane B after Lane A lands.
- Lane A and Lane C both influence public wording. Keep README/CHANGELOG edits last so docs do not promise behavior that code does not yet ship.

## NOT in scope

- `function.wrapper.pipeline.v1` execution in `spec`
- any function family beyond monotone-up
- more than one direct dep
- cross-library TypeScript dep resolution
- seam-kind TypeScript execution
- `.test.spec` TypeScript execution
- `spec validate --target-language`
- `spec export --target-language`
- generic backend abstraction work

These items are already the right kind of deferred work for `TODOS.md`. Do not quietly pull them into M46.

## What already exists

- target-specific proof storage already works
- Bun build and local test runners already work
- helper semantic family truth already exists
- zero-dependency monotone-up TS execution already works
- export and status already understand additive proof truth

M46 is extending a real lane, not inventing one.

## Acceptance Criteria

M46 is complete only if all of the following are true:

1. The M45 zero-dep lane still passes unchanged.
2. A monotone-up unit with one helper passthrough dep can execute in the TypeScript lane.
3. Helper eligibility is enforced semantically through `function.helper.identity_passthrough.v1`.
4. Helper absence from the loaded tree fails before Bun runs.
5. More than one dep fails before Bun runs.
6. `.test.spec` remains unsupported for TypeScript.
7. Target-proof separation remains unchanged and honest.
8. Duplicate CLI-side TypeScript generator logic is gone or reduced to thin routing.
9. The proof wall exercises a real helper-aware unit instead of a zero-dep stand-in.
10. README and CHANGELOG describe the new bounded lane exactly, with no wrapper or multi-dep claim.

## Open Risks

- the helper family is runtime-supported but still unpromoted; M46 must consume that truth without inventing a new packet workflow
- `spec-cli/src/commands.rs` already mixes live routing and legacy TS helper logic, so cleanup may touch more surface area than the feature itself
- if the example and packet fixtures diverge, the proof wall can become green in one place and stale in another

## One-Line Summary

M46 should make the first realistic monotone-up TypeScript unit executable by allowing exactly one supported helper passthrough dep, while keeping every broader TypeScript ambition out of scope and locking the result with proof, tests, docs, and worktree-aware execution order.
