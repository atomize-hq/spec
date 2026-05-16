# M61: Bounded Recursive Cross-Library TypeScript Function-Graph Execution Plan

Status: **authoritative implementation plan**  
Milestone: **M61**  
Milestone family: **bounded TypeScript execution**  
Implementation readiness: **ready for bounded execution**  
Plan scope: **extend the existing Bun-backed TypeScript lane from same-tree local supported-function graphs plus direct cross-library portability exceptions to recursive local-plus-cross-library closure across the already-supported function families; preserve family-specific direct-dep contracts, additive proof, atom-only execution, and all broader non-goals**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Validated at commit: **`96d2ee9`**  
Last rewritten: **2026-05-15**

Supersedes:

- the shipped M60 authority plan previously maintained at this path
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260515-113145.md`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260515-113145.md`
- `README.md`
- `TODOS.md`
- `CHANGELOG.md`
- `examples/crosslib-app/README.md`

Primary repo surfaces:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `examples/crosslib-app/units/**`
- `examples/shared-spec/units/**`
- `README.md`
- `TODOS.md`
- `CHANGELOG.md`
- `examples/crosslib-app/README.md`

## Executive Summary

M59 shipped the same-tree local TypeScript graph lane.

M55 and M56 preserved three narrow cross-library exceptions:

- one direct helper import for `function.arithmetic_leaf.monotone_up.v1`
- one direct wrapper-root path for `function.wrapper.pipeline.v1`
- one direct chain3-root path for `function.wrapper.pipeline.chain3.v1`

M58 widened same-tree recursive chain3 closure, but it kept recursive shared closure out.

That leaves the next real product gap:

```text
the repo can already execute direct cross-library roots,
but it still cannot recurse truthfully through a shared function graph
once a supported TypeScript root depends on another shared supported function
that itself has deps
```

M61 fixes exactly that gap.

It does not claim generic cross-library parity. It does not claim arbitrary authored 4+ topology support. It does not claim molecule TypeScript execution. It does not change semantic-family meaning.

It ships one bounded contract:

```text
recursive local-plus-cross-library closure across the current supported
function families, with owner-library-qualified dep resolution and the
same frozen family-specific dep contracts at every recursive depth
```

## Frozen Implementation Decisions

These decisions are locked for M61. If any of them changes, the milestone scope changed and this plan must be rewritten before implementation continues.

1. **Unify the validator and generator around one recursive closure story.**
   - Replace the current split between same-tree local recursion and direct portability exceptions.
   - Do not keep two separate closure models and bolt recursion onto only one of them.

2. **Resolve closure membership by qualified identity, not raw unit id.**
   - Reuse library-qualified identity concepts already present in `spec-core/src/validator.rs`.
   - Do not allow the TypeScript collector to choose a shared unit by first-loaded or last-loaded raw-id coincidence.

3. **Keep the supported family set frozen to what the repo already ships today.**
   - `function.helper.identity_passthrough.v1`
   - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
   - `function.arithmetic_leaf.monotone_up.v1`
   - `function.wrapper.pipeline.v1`
   - `function.wrapper.pipeline.normalized_required_arg.v1`
   - `function.wrapper.pipeline.chain3.v1`

4. **Preserve family-specific dep contracts exactly.**
   - Helper family rules stay helper-family rules.
   - Wrapper family rules stay wrapper-family rules.
   - Chain3 family rules stay chain3-family rules.
   - Normalized-required-arg wrapper rules stay the M60 rules.
   - Recursive shared closure widens location, not semantics.

5. **Promote one maintained recursive-shared proof owner in the cross-library example.**
   - Use checked-in example specs under `examples/crosslib-app/units/` and `examples/shared-spec/units/`.
   - Do not leave the only green proof inside temporary CLI test scaffolding.

6. **Keep the public contract sentence tight and identical across docs.**
   - Use this exact sentence:
     - `M61 extends the bounded Bun-backed TypeScript lane to recursive local-plus-cross-library closure across the already-supported function families, while preserving family-specific direct-dep contracts, additive proof, atom-only execution, and the broader bans on arbitrary 4+ topology parity and molecule TypeScript execution.`

## Current Validated Basis

Validated on `feat/m40-plus` at `96d2ee9`.

Observed live branch truth:

- `spec-core/src/validator.rs` still splits target validation between:
  - `validate_typescript_local_graph_root_spec_with_specs(...)`
  - `validate_typescript_portability_target_spec_with_specs(...)`
- `typescript_target_uses_local_graph_lane(...)` still rejects any root with a `shared::...` dep from the local recursive lane.
- `validate_typescript_chain3_first_dep_family(...)` still enforces the M58 same-tree-only slot-1 recursive chain3 rule for shared deps.
- `spec-core/src/typescript_backend.rs` still splits closure collection between:
  - `collect_typescript_local_graph_member_closure(...)`
  - `collect_typescript_portability_root_closure(...)`
- `build_typescript_loaded_specs_by_id(...)`, `build_typescript_spec_indices_by_key(...)`, and `resolve_typescript_dep_spec(...)` still rely on raw unit ids plus a limited authored-key exception path rather than first-class qualified node identity.
- `spec-cli/tests/cli.rs` already contains:
  - passing proof for direct cross-library helper, wrapper, and chain3 roots
  - a negative regression for recursive cross-library nested chain3 in slot 1
- `README.md` and `TODOS.md` still explicitly defer generic recursive cross-library function graphs.

The repo is therefore in a truthful but awkward state: the direct-root exceptions are real, but the next natural recursive shared case still fails before Bun.

## Step 0: Scope Challenge

### Premise correction

The problem is not "TypeScript support needs to be broader."

The real problem is smaller:

```text
the bounded TypeScript lane already knows enough semantic truth to execute
recursive supported function graphs, but the cross-library closure contract
still stops at direct root exceptions
```

If M61 expands beyond that sentence, it is overbuilt.

### What already exists

| Sub-problem | Existing owner | M61 action |
| --- | --- | --- |
| target-language CLI surface | `spec-cli/src/commands.rs` | reuse existing `--target-language typescript` flow |
| pre-Bun target validation | `spec-core/src/validator.rs` | replace the root-depth split with recursive qualified closure validation |
| TypeScript closure collection | `spec-core/src/typescript_backend.rs` | replace raw-id collection with qualified closure membership and dedupe |
| semantic family truth | `spec-core/src/semantic_review.rs` | reuse as-is, no new family promotion |
| direct cross-library example | `examples/crosslib-app/units/**`, `examples/shared-spec/units/**` | extend into one maintained recursive-shared proof path |
| regression harness | `spec-cli/tests/cli.rs` | convert one current red path into green, keep the other red paths |
| public contract wording | `README.md`, `TODOS.md`, `CHANGELOG.md`, `examples/crosslib-app/README.md` | replace the generic recursive-cross-library defer line with the exact M61 claim |

### Minimum complete slice

The minimum honest M61 slice is:

1. unify TypeScript target validation around recursive qualified closure
2. unify TypeScript closure collection around recursive qualified closure
3. extend root-family handling so M60 normalized-required-arg wrappers are legal closure members and roots in the TypeScript lane when their existing family contract is satisfied
4. add one maintained recursive-shared example path in `examples/crosslib-app` plus `examples/shared-spec`
5. update CLI regressions so one recursive shared path is green and the preserved red paths still fail before Bun
6. update README, TODOS, CHANGELOG, and `examples/crosslib-app/README.md` in the same PR

Anything smaller is fake done.

Examples:

- adding qualified lookup without a maintained example is fake done
- adding an example without generator and validator convergence is fake done
- converting one CLI negative test to green without preserving the rejection wall is fake done
- widening the recursive lane without handling normalized-required-arg wrappers is fake done because M60 already shipped that family

### Complexity and blast radius

This milestone touches more than 8 files. That normally smells.

It is still the right size because the extra files are proof and contract surfaces, not new infrastructure:

- one validator contract file
- one TypeScript backend collector file
- one semantic family inventory file only as a consumer boundary, not as a widened semantic subsystem
- one CLI integration test file
- one maintained cross-library example README
- two example unit trees
- three repo-root docs

The complete version is only modestly larger than the shortcut, and the shortcut would leave the repo lying about recursive shared support. Boil the lake.

### Search check

No framework built-in replaces this work. This is repo-owned semantic-routing and bounded TypeScript lowering logic.

- **[Layer 1]** Reuse `QualifiedUnitRef` and existing cross-library dep identity concepts already in `spec-core/src/validator.rs`
- **[Layer 1]** Reuse current supported-family routing from `spec-core/src/semantic_review.rs`
- **[Layer 1]** Reuse the current `spec-cli/tests/cli.rs` cross-library proof scaffolding
- **[Layer 3]** The right design is not a generic graph-policy engine. The right design is a qualified recursive closure contract over the already-supported function families

### TODOS cross-reference

`TODOS.md` currently says the remaining TypeScript oceans after M59 and M60 include generic recursive cross-library function graphs.

M61 should narrow that defer line, not erase all remaining oceans. After landing, `TODOS.md` should say:

- recursive closure across the current supported family set shipped in M61
- arbitrary authored 4+ direct-dep topology parity remains out
- new semantic-family promotion remains out
- molecule TypeScript execution remains out
- seam-kind TypeScript execution remains out

### Completeness and distribution check

No new distributable artifact is introduced.

This remains a capability widen inside the existing `spec` CLI and existing release surface. The complete version here is proof completeness, not packaging work.

## Milestone Contract

### Exact shipped behavior

After M61:

- a supported `kind:function` TypeScript root may recurse through a reachable graph that mixes local and shared units
- every reachable unit must:
  - resolve from the loaded unit set in owner-library context
  - classify to one of the current supported function families
  - author non-empty `body.typescript`
  - satisfy its existing family-specific dep contract
- recursive closure may cross libraries multiple times
- recursive closure must still be finite, deduped, and limited to the loaded unit set
- unrelated loaded units must still stay out of the emitted TypeScript tree

### Exact root eligibility

M61 widens recursive closure depth. It does not create a new "any supported function can be a root anywhere" rule.

- existing local-only TypeScript roots stay legal exactly where they already work today
- direct cross-library root handling stays limited to the root families the lane already ships, plus the M60 normalized-wrapper family
- `function.helper.identity_passthrough.v1` remains closure-only in M61. It may appear as a reachable helper member where the existing helper rule allows it, but it is not a standalone TypeScript execution target
- recursive shared support means a legal root may now traverse into further legal local or shared members. It does not create generic root parity across all families or authored topologies

### Exact preserved boundaries

These must still reject before Bun:

- any reachable unit with unsupported semantic review
- any reachable unit without `body.typescript`
- any reachable non-`kind:function` unit
- any dep that cannot resolve in the correct library context
- any reachable wrapper member whose direct dep order or family mix is wrong
- any reachable chain3 member whose direct dep order or family mix is wrong
- any attempt to execute `.test.spec` with `--target-language typescript`
- any attempt to imply `spec validate --target-language`
- any attempt to imply `spec export --target-language`
- any attempt to claim arbitrary authored 4+ direct-dep root topology support

### Exact maintained example seed

Promote a checked-in recursive-shared chain3 proof owner by turning the current temporary CLI helper shapes into maintained example specs:

```text
examples/shared-spec/units/pricing/calculate_total.unit.spec
examples/shared-spec/units/pricing/base_nested_chain3.unit.spec
examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec
```

Authored story:

- `shared::pricing/calculate_total` remains the shared wrapper-level subtotal calculator
- `shared::pricing/base_nested_chain3` becomes a shared recursive chain3 member that itself depends on shared supported functions
- app-side `pricing/checkout_nested_chain3` becomes the maintained cross-library recursive root

The current temporary helper in `spec-cli/tests/cli.rs` is the prototype. M61 promotes that shape into the maintained example tree.

### Exact family handling

TypeScript root and closure handling must recognize all current supported function families:

| Family | M61 role |
| --- | --- |
| `function.helper.identity_passthrough.v1` | closure-only helper leaf, zero deps, never a standalone TypeScript root in M61 |
| `function.arithmetic_leaf.monotone_down_nonnegative.v1` | supported leaf, helper-free, root/member where already legal today |
| `function.arithmetic_leaf.monotone_up.v1` | supported leaf root/member, zero deps or one helper dep under the existing helper rule |
| `function.wrapper.pipeline.v1` | supported wrapper root/member under the existing two-dep tuple |
| `function.wrapper.pipeline.normalized_required_arg.v1` | supported wrapper root/member under the existing M60 normalized required-arg contract |
| `function.wrapper.pipeline.chain3.v1` | supported chain3 root/member under the existing three-dep tuple |

M61 does not invent new family meaning. It only allows these current meanings to recurse across shared closure truthfully.

## Architecture Review

### Dependency graph

```text
                           +----------------------------------+
                           | spec-core/src/semantic_review.rs |
                           +----------------------------------+
                                      | existing truth only
                                      v
 +-----------------------------+   +------------------------------------+
 | spec-core/src/validator.rs  |-->| Recursive closure eligibility      |
 +-----------------------------+   | - qualified dep resolution         |
 | current local/portability   |   | - family-specific dep contracts    |
 | split must collapse into    |   | - body.typescript required         |
 | one recursive contract      |   | - supported semantic review only   |
 +-----------------------------+   +------------------------------------+
                                      |
                                      v
 +------------------------------------+   +------------------------------+
 | spec-core/src/typescript_backend.rs|-->| Emitted TS member set        |
 +------------------------------------+   | - qualified dedupe           |
 | current raw-id + authored-key      |   | - unrelated units excluded   |
 | lookup must become qualified       |   | - imports still callable-safe|
 +------------------------------------+   +------------------------------+
                                      |
                                      v
                   +-----------------------------------------------+
                   | Proof surfaces                                |
                   | - spec-cli/tests/cli.rs                       |
                   | - examples/crosslib-app/units/**             |
                   | - examples/shared-spec/units/**              |
                   | - README / TODOS / CHANGELOG                 |
                   +-----------------------------------------------+
```

### Current flaw

Today the TypeScript lane has two separate mental models:

```text
Model A: same-tree local graph recursion
Model B: direct cross-library portability exceptions
```

That split leaks into both validation and collection. It creates a misleading product boundary:

- recursion works only when everything stays local
- cross-library works only when the root shape falls into a preserved direct exception

That is why the temporary recursive shared chain3 case still fails before Bun.

### M61 target architecture

```text
root
  `- recursive closure walker
      |- resolve dep in owner library context
      |- fetch exact loaded unit by qualified identity
      |- require supported semantic review
      |- require body.typescript
      |- validate family-specific dep contract
      |- recurse through local or shared deps
      `- dedupe by qualified node identity
```

### Owner-library resolution contract

Validator and collector must share the same dep-resolution story. No split-brain behavior is allowed here.

```text
for each dep edge:
  1. parse authored dep
  2. determine owning library context of the current node
  3. resolve the dep against that context
  4. produce one qualified node identity
  5. validate or collect that exact node
```

Non-negotiable rules:

- local deps resolve relative to the owning library of the current node, not the CLI invocation root
- explicit `shared::...` deps resolve to that shared library identity, never by raw-id coincidence
- validator and collector must either call the same helper or use byte-for-byte equivalent qualified-resolution rules
- once a qualified node is chosen, all later recursion and dedupe use that qualified identity rather than re-resolving by raw unit id

### File-by-file responsibilities

- `spec-core/src/validator.rs`
  - replace `typescript_target_uses_local_graph_lane(...)` as the root split gate
  - add one recursive validation path for target roots and closure members
  - extend root-family handling to include `function.wrapper.pipeline.normalized_required_arg.v1`
  - keep helper, wrapper, normalized-wrapper, and chain3 dep contracts explicit and separate
- `spec-core/src/typescript_backend.rs`
  - replace raw-id-driven closure membership with qualified identity
  - stop collecting local recursion and cross-library portability through separate top-level paths
  - keep rendering and import emission mostly intact
- `spec-core/src/semantic_review.rs`
  - no new family work
  - treat as truth source only
- `spec-cli/tests/cli.rs`
  - convert the recursive shared nested chain3 helper shape from a red path into the new green path
  - keep preserved red paths for wrong family, wrong order, missing body, unresolved dep, and molecule rejection
- `examples/shared-spec/units/**`
  - add maintained shared recursive members
- `examples/crosslib-app/units/**`
  - add maintained recursive root
- `README.md`, `TODOS.md`, `CHANGELOG.md`, `examples/crosslib-app/README.md`
  - update the public contract

## Code Quality Review

### Design choices

1. **Use a small qualified-identity helper, not a general graph framework.**
   This matches explicit-over-clever and minimal diff. M61 needs truthful lookup, not a new subsystem.

2. **Delete the local-vs-portability branch at the decision level, not just at the docs level.**
   Keeping both code paths and sprinkling recursion onto one of them will rot immediately.

3. **Keep family-specific validators separate.**
   Wrapper, normalized-wrapper, helper, and chain3 rules should stay obvious. Do not compress them into a generic "arity + supported deps" abstraction in this milestone.

4. **Treat the M60 normalized-required-arg wrapper as a first-class supported TypeScript family.**
   If M61 ignores it, the repo will have one supported family that semantic review knows about and the TypeScript lane silently does not.

5. **Promote the current CLI-only recursive shared prototype into the maintained example tree.**
   That gives docs and tests one shared truth owner instead of duplicating logic forever.

6. **Keep doc phrasing identical everywhere.**
   This repo already teaches through README and examples. Drift here becomes product drift.

### DRY and maintenance rules

- reuse `QualifiedUnitRef` semantics rather than inventing a second qualified-id type
- reuse the existing dep-contract validators where possible
- do not duplicate shared recursive example logic in both docs and temporary fixtures if the example can own it
- keep the red-path Bun-precheck pattern consistent with current CLI tests
- keep the maintained example focused, not a new zoo of every recursive case

## Implementation Plan

### Implementation lockstep

The implementation is only honest if these move together:

1. validator root and closure rules
2. collector root and closure rules
3. maintained recursive-shared example proof
4. CLI regressions and public docs

Do not land a validator-only or collector-only half-state. The repo would compile, but the contract would still be ambiguous.

### Step 1. Replace the validator split with recursive qualified closure validation

Files:

- `spec-core/src/validator.rs`

Changes:

1. stop using `typescript_target_uses_local_graph_lane(...)` as the root branch for M61 behavior
2. replace `validate_typescript_local_graph_root_spec_with_specs(...)` and `validate_typescript_portability_target_spec_with_specs(...)` with one recursive root-validation flow
3. add explicit dep resolution in owner-library context
4. extend root-family handling to admit:
   - `function.arithmetic_leaf.monotone_up.v1`
   - `function.wrapper.pipeline.v1`
   - `function.wrapper.pipeline.normalized_required_arg.v1`
   - `function.wrapper.pipeline.chain3.v1`
5. keep closure-member family checks explicit and fail-fast before Bun

Acceptance:

- a supported local-only root still validates
- a supported direct cross-library root still validates
- a supported recursive shared root now validates
- wrong family, wrong order, missing body, unresolved dep, and molecule rejection still fail before Bun

### Step 2. Replace raw-id closure collection with qualified recursive membership

Files:

- `spec-core/src/typescript_backend.rs`

Changes:

1. replace `build_typescript_loaded_specs_by_id(...)` and `build_typescript_spec_indices_by_key(...)` with qualified lookup structures
2. replace `resolve_typescript_dep_spec(...)` with owner-library-qualified resolution
3. collapse `collect_typescript_local_graph_root_closure(...)`, `collect_typescript_local_graph_member_closure(...)`, `collect_typescript_portability_root_closure(...)`, and `collect_typescript_closure_member(...)` behind one qualified recursive collector story
4. dedupe reachable members by qualified identity, not raw id
5. keep emitted file paths and import rendering stable unless the qualified collector proves they must change

Acceptance:

- two units with the same local id in different libraries do not collide in closure membership
- recursive shared closure includes only the reachable qualified members
- unrelated loaded units remain excluded from the emitted tree
- generated imports still resolve correctly in the emitted TS tree
- validator and collector no longer disagree about which loaded unit a dep edge names

### Step 3. Extend TypeScript family handling to include the M60 normalized wrapper

Files:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`

Changes:

1. add a TypeScript target compatibility constant for `function.wrapper.pipeline.normalized_required_arg.v1`
2. teach root-family classification to recognize it as a wrapper-class family
3. teach closure-member validation to route it through the existing wrapper-family dep contract
4. keep broader required-arg expression widening out of scope

Acceptance:

- M60 normalized-wrapper specs can participate in the recursive TypeScript lane when otherwise eligible
- raw wrapper and normalized wrapper remain distinct semantic families
- no new required-arg expression surfaces become legal

### Step 4. Promote a maintained recursive-shared example

Files:

- `examples/shared-spec/units/pricing/calculate_total.unit.spec`
- `examples/shared-spec/units/pricing/base_nested_chain3.unit.spec`
- `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`
- generated artifacts refreshed by the normal `spec` loop only after source spec edits

Changes:

1. add checked-in shared recursive members that match the existing temporary CLI helper story
2. add one app-side recursive shared root
3. keep the current direct-root proof owners (`apply_tax`, `calculate_total`) intact
4. add the new recursive root to the example README proof commands

Acceptance:

```bash
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --target-language typescript
```

- all three paths pass
- the new recursive root is maintained in repo state, not created only by test helpers

### Step 5. Refresh CLI regression truth

Files:

- `spec-cli/tests/cli.rs`

Changes:

1. convert the current recursive shared nested chain3 rejection helper into the new green path
2. keep preserved red-path tests for:
   - unsupported shared recursive member
   - wrong dep order inside a shared recursive member
   - missing `body.typescript` on a shared recursive member
   - unresolved shared dep
   - molecule TypeScript rejection
3. add at least one regression that proves owner-library-qualified resolution when local and shared same-id units coexist

Acceptance:

```bash
cargo test -p spec-cli --test cli
```

- the new recursive green path passes
- Bun-precheck failures still happen before Bun
- qualified identity regressions are covered

### Step 6. Update the public contract and backlog wording

Files:

- `README.md`
- `examples/crosslib-app/README.md`
- `TODOS.md`
- `CHANGELOG.md`

Changes:

1. replace the M59 direct-root-only recursive defer wording with the exact M61 claim
2. add the new recursive example command to `examples/crosslib-app/README.md`
3. narrow the TODO backlog from "generic recursive cross-library function graphs remain out" to the smaller remaining oceans:
   - arbitrary authored 4+ topology parity
   - new semantic-family promotion
   - molecule TypeScript execution
   - seam-kind TypeScript execution
4. update CHANGELOG with the shipped user-facing contract

Acceptance:

- docs all use the same frozen sentence
- docs do not imply arbitrary graph parity
- docs still call out molecule TypeScript rejection and additive proof

### Step 7. Run the final proof wall and capture the post-change basis

Files:

- none authored; verification and generated artifacts only

Changes:

1. run `spec-core` proof for validator and TypeScript backend paths
2. run `spec-cli` proof for the cross-library example and regressions
3. run the maintained recursive example commands
4. refresh any checked-in proof artifacts produced by the standard source-spec loop

Acceptance:

```bash
cargo test -p spec-core validator
cargo test -p spec-core typescript_backend
cargo test -p spec-cli --test cli

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --target-language typescript
cargo run -p spec-cli -- status examples/crosslib-app/units --target-language typescript --format json
```

Expected result:

- the recursive shared root passes under Bun
- status remains target-specific and additive
- unrelated units do not appear in the emitted TS tree
- preserved red-path regressions still fail before Bun

## Test Review

### Test framework and proof owners

This repo's proof wall is Rust-native:

- unit-style validator and collector tests in `spec-core`
- CLI integration tests in `spec-cli/tests/cli.rs`
- maintained checked-in example proof in `examples/crosslib-app` and `examples/shared-spec`
- read-side truth through `spec status ... --target-language typescript --format json`

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/validator.rs
    |
    ├── root target eligibility
    |   ├── [EXISTING] local-only supported root
    |   ├── [EXISTING] direct cross-library helper-assisted leaf root
    |   ├── [EXISTING] direct cross-library wrapper root
    |   ├── [EXISTING] direct cross-library chain3 root
    |   └── [ADD]      recursive shared root
    |
    ├── closure-member family checks
    |   ├── [EXISTING] helper dep contract
    |   ├── [EXISTING] wrapper dep contract
    |   ├── [EXISTING] chain3 dep contract
    |   └── [ADD]      normalized-required-arg wrapper contract in TS lane
    |
    └── rejection wall
        ├── [ADD] unsupported shared member
        ├── [ADD] wrong dep order in shared member
        ├── [ADD] unresolved shared dep in owner context
        ├── [ADD] missing body.typescript in shared member
        └── [EXISTING] molecule TypeScript rejection

[+] spec-core/src/typescript_backend.rs
    |
    ├── closure membership
    |   ├── [EXISTING] local-only recursion
    |   ├── [EXISTING] direct cross-library roots
    |   └── [ADD]      recursive shared membership via qualified identity
    |
    ├── dedupe
    |   ├── [EXISTING] raw-id/member set
    |   └── [ADD]      qualified-identity member set
    |
    └── emission
        ├── [EXISTING] import rendering
        └── [ADD]      same-id local/shared units resolve to the correct owner

USER FLOW COVERAGE
===========================
[+] examples/crosslib-app direct proof
    ├── [EXISTING] apply_tax direct helper path
    ├── [EXISTING] calculate_total direct wrapper path
    └── [ADD]      checkout_nested_chain3 recursive shared path

[+] Error states
    ├── [ADD] unsupported shared recursive member
    ├── [ADD] missing shared body.typescript
    ├── [ADD] unresolved shared dep alias/unit
    └── [EXISTING] molecule target-language rejection

─────────────────────────────────
COVERAGE TARGET: all new recursive closure paths covered
CRITICAL REGRESSIONS: owner-context lookup, recursive shared green path,
rejection-before-Bun wall, normalized-wrapper TS eligibility
─────────────────────────────────
```

### Required tests

1. `spec-core/src/validator.rs`
   - recursive shared root validates when all reachable members are eligible
   - recursive shared closure rejects unsupported member before Bun
   - recursive shared closure rejects missing `body.typescript` before Bun
   - recursive shared chain3 member still rejects wrong slot-1 family/order
   - normalized-required-arg wrapper is accepted as a TypeScript root and closure member when otherwise eligible

2. `spec-core/src/typescript_backend.rs`
   - recursive shared closure includes the reachable maintained example graph
   - qualified identity dedupes correctly when local and shared units share the same local id
   - unrelated loaded units are excluded from the emitted tree

3. `spec-cli/tests/cli.rs`
   - `checkout_nested_chain3.unit.spec` passes under `--target-language typescript`
   - wrong family/wrong order/missing body/unresolved shared dep still fail before Bun
   - molecule TypeScript rejection still fails before Bun

4. maintained example loop
   - cross-library example README commands stay truthful and green

### Regression rule

This milestone converts an existing red path into a green path. Regression tests are mandatory.

Required regressions:

- the old negative recursive shared nested chain3 case becomes green only for the exact supported shape
- preserved red paths still reject before Bun
- normalized-required-arg wrappers do not stay accidentally unsupported in the TypeScript lane
- owner-library-qualified lookup does not regress back to raw-id selection

## Failure Modes Registry

| New codepath | Real production failure | Test covers it? | Error handling exists? | User-visible effect | Priority |
| --- | --- | --- | --- | --- | --- |
| recursive validator rewrite | root still takes the stale local-vs-portability branch | must add | partial | user sees false rejection of a supported recursive shared graph | critical |
| qualified lookup | local/shared same-id unit resolves to the wrong implementation | must add | no silent safeguard today | Bun runs the wrong logic and the repo lies | critical |
| normalized wrapper TS eligibility | M60 family stays unsupported in the TypeScript lane | must add | no | shipped supported family behaves inconsistently across surfaces | high |
| shared recursive chain3 green path | maintained example still exists only in CLI helper code | must add | no | docs claim capability without a checked-in proof owner | high |
| emitted member set | unrelated loaded units leak into the generated TS tree | must add | partial | output no longer matches the bounded contract | high |
| public docs | README says "cross-library recursive TypeScript graphs now work" too broadly | manual review + doc diff | no | users over-assume topology parity | medium |
| molecule boundary | recursive widen accidentally allows `.test.spec` TS execution | must keep regression | yes via validator | users get ambiguous half-working molecule behavior | high |

Critical gaps to avoid:

- any qualified-identity path with no regression and silent wrong-unit selection
- any recursive shared green-path claim without a checked-in proof owner
- any M60 normalized-wrapper omission from the TypeScript lane

## Performance Review

This milestone should be performance-neutral or near-neutral if implemented correctly.

Expected characteristics:

- one recursive closure walker instead of two separate walkers
- slightly richer dep-resolution keys due to qualified identity
- no new file discovery behavior
- no new artifact formats

Guardrails:

- do not introduce repeated semantic-review recomputation across the same closure members if a simple per-run cache is enough
- do not add a generic graph-policy abstraction that every target path pays for
- keep the closure walk bounded to reachable deps only
- keep the maintained example small so CLI proof cost stays reasonable

## NOT in scope

- arbitrary authored 4+ direct-dep topology parity
- new semantic-family promotion
- generic graph-policy or portability frameworks
- molecule TypeScript execution
- seam-kind TypeScript execution
- `spec validate --target-language`
- `spec export --target-language`
- non-Bun TypeScript toolchains
- a rewrite of Rust proof surfaces or passport schemas

## TODOS.md updates required in the same PR

1. mark recursive local-plus-cross-library closure across the current supported family set as shipped in M61
2. explicitly defer:
   - arbitrary authored 4+ direct-dep topology parity
   - new semantic-family promotion
   - molecule TypeScript execution
   - seam-kind TypeScript execution
3. remove wording that still implies all recursive cross-library closure is out
4. keep wording that broader TypeScript oceans still remain

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| 1. recursive validator contract | `spec-core/src/` | — |
| 2. recursive TS collector + qualified lookup | `spec-core/src/` | 1, exact validation contract frozen |
| 3. maintained recursive example | `examples/shared-spec/`, `examples/crosslib-app/` | 1, exact family and rejection rules frozen |
| 4. CLI regression refresh | `spec-cli/tests/` | 1, 2, 3 |
| 5. docs and release notes | repo-root docs + `examples/crosslib-app/README.md` | 1, 3, frozen contract sentence |
| 6. final proof wall and artifact refresh | workspace test commands / generated artifacts | 2, 3, 4, 5 |

### Parallel lanes

- **Lane A:** Step 1 then Step 2, sequential inside `spec-core/src/`
- **Lane B:** Step 3, maintained recursive example work in `examples/shared-spec/` and `examples/crosslib-app/`
- **Lane C:** Step 5, docs and release notes after the contract sentence and example file names are frozen
- **Lane D:** Step 4, CLI regression refresh after Lane A and Lane B converge
- **Lane E:** Step 6, final proof wall after A + B + C + D converge

### Execution order

Launch **Lane A** first. The validator and collector contract must freeze before parallel work is safe.

Once the qualified-identity model, supported family list, and maintained example file names are frozen, launch **Lane B** and **Lane C** in parallel worktrees.

After Lane B lands and Lane A is green, run **Lane D** for CLI truth.

After A, B, C, and D merge, run **Lane E** serially for the final proof wall.

### Conflict flags

- **Lane A** is not parallelizable internally. `spec-core/src/validator.rs` and `spec-core/src/typescript_backend.rs` define the same contract and will conflict if split too early.
- **Lane B** and **Lane D** both affect the cross-library example story. Do not run them independently from stale example shapes.
- **Lane C** must wait for the exact contract sentence and example file names. Otherwise docs will drift from code and tests.
- Do not split `examples/shared-spec/**` and `examples/crosslib-app/**` into separate lanes. They are one proof owner.

## Definition of Done

M61 is done when all of the following are true:

1. a supported TypeScript root can recurse through a loaded local-plus-shared supported function graph and pass under Bun
2. recursive shared closure resolves deps in owner library context rather than ambiguous raw-id lookup
3. every reachable unit still requires supported semantic review and non-empty `body.typescript`
4. helper, wrapper, normalized-wrapper, and chain3 family contracts stay frozen and enforced at recursive depth
5. unrelated loaded units are excluded from the emitted TypeScript tree
6. `examples/crosslib-app` contains a maintained recursive-shared green path
7. preserved red-path regressions exist for wrong family, wrong order, missing body, unresolved shared dep, and molecule rejection
8. target-specific proof remains additive and truthful on status and passport surfaces
9. README, TODOS, CHANGELOG, and `examples/crosslib-app/README.md` all describe the exact M61 boundary
10. the closeout sentence can honestly say:
    - `recursive local-plus-cross-library closure across the current supported TypeScript family set now works`
    - without implying arbitrary graph parity

## Verification Commands

Run in this order:

```bash
cargo test -p spec-core validator
cargo test -p spec-core typescript_backend
cargo test -p spec-cli --test cli

cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/calculate_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --target-language typescript
cargo run -p spec-cli -- status examples/crosslib-app/units --target-language typescript --format json
```

Expected outcome:

- validator and collector tests are green
- CLI regressions are green
- the maintained recursive-shared root passes under Bun
- status remains target-specific and additive
- preserved red-path cases still reject before Bun

## Completion Summary

- Step 0: Scope Challenge, complete
- Architecture: replace the local-vs-portability split with one recursive qualified closure contract
- Code Quality: explicit qualified lookup, explicit family validators, no generic graph subsystem
- Test Review: full proof wall defined across validator, collector, CLI, and maintained example surfaces
- Performance Review: near-neutral, bounded to reachable closure and simple qualified lookup
- NOT in scope: written
- What already exists: written
- TODOS.md updates: required in same PR
- Failure modes: critical gaps identified around wrong-unit selection and false product claims
- Parallelization: 6 steps, 1 core lane, 2 parallel authoring lanes, 1 downstream CLI lane, 1 final proof lane

This is the whole move. Make recursive shared closure honest, keep the family meanings frozen, prove it in the maintained example, and stop there.
