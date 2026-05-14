# M59: Semantic-Review-Driven Local TypeScript Function Graph Execution Plan

Status: **authoritative implementation plan**  
Milestone: **M59**  
Milestone family: **second-language-backend**  
Implementation readiness: **ready for bounded execution**  
Plan scope: **ship exactly one new TypeScript capability: any same-tree `kind:function` unit that classifies to a shipped supported function family, authors non-empty `body.typescript`, and stays inside a fully loaded local closure may execute through a semantic-review-driven local graph lane; preserve the existing M55-M58 family-shaped direct cross-library helper, wrapper, and chain3 lanes unchanged; do not widen to arbitrary per-node dep arity, new semantic families, molecule execution, seam kinds, or target-language validate/export**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Validated at commit: **`bd55d0f`**  
Last rewritten: **2026-05-14**

Supersedes:

- the stale M58 bounded nested chain3 plan previously maintained at this path
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260514-135734.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260514-074521.md`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260514-135734.md`
- `README.md`
- `TODOS.md`
- `AGENTS.md`
- `ORCH_PLAN.md`

Primary repo surfaces:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `README.md`
- `TODOS.md`

## Executive Summary

M59 is not "arbitrary generic TypeScript execution."

M59 is one narrower, honest widen:

```text
generic at the graph level
not generic at the node-shape level
```

The local TypeScript lane should stop hard-coding eligibility as "monotone-up, wrapper, or chain3 root plus a closure-family allowlist." Instead, it should admit any same-tree local `kind:function` root whose reachable closure:

- resolves entirely from the loaded unit set
- stays local, not `shared::...`
- stays within the shipped supported semantic-review function families
- authors non-empty `body.typescript` at every reachable node
- remains acyclic

Everything broader stays out:

- arbitrary 4+ dep authored function units
- new semantic families
- generic recursive cross-library graphs
- molecule TypeScript execution
- seam-kind TypeScript execution
- `spec validate --target-language`
- `spec export --target-language`

The user-visible gain is still real:

- local helper roots can execute
- local monotone-down roots can execute
- local wrapper and chain3 roots no longer depend on a closure-member family table
- shared same-tree subgraphs are admitted and deduped when every reachable node is already semantically supported

The existing M55-M58 direct cross-library helper, wrapper, and chain3 lanes remain a preserved portability contract. M59 adds a new local graph lane. It does not replace the old portability lane.

## Current Validated Basis

Validated from HEAD source review on `feat/m40-plus` at `bd55d0f`.

Observed repo truth:

- `README.md`
  - the bounded TypeScript lane still describes roots in family-slot language
  - direct cross-library helper, wrapper, and chain3 execution are documented as bounded exceptions
  - molecule TypeScript execution, seam kinds, and target-language validate/export remain out
- `TODOS.md`
  - `Generic multi-dependency TypeScript execution` is still the one open late-lane TypeScript item
  - the wording is directionally right, but now too fuzzy for an honest M59 close
- `spec-core/src/validator.rs`
  - `validate_typescript_execution_target_spec_with_specs(...)` still selects by root family and direct dep tuple
  - `validate_typescript_closure_member_spec_with_specs(...)` still acts like a closure-member family allowlist gate
  - direct cross-library helper, wrapper, and chain3 validators already exist and should be preserved
  - cycle detection already exists through `detect_cycles(...)` and `detect_qualified_cycles(...)`
- `spec-core/src/typescript_backend.rs`
  - the module banner still describes a bounded M52-style family exception lane
  - root and closure traversal still branch by specific family arms
  - `included: BTreeSet<usize>` already gives the right dedupe primitive for shared local subgraphs
- `spec-core/src/semantic_review.rs`
  - supported function-family truth already exists and is queryable through `evaluate_semantic_review_with_context(...)`
  - `effective_support_status()` already gives the supported versus unsupported contract M59 needs
  - unsupported dep topology still rejects the broader oceans M59 is explicitly not shipping
- `spec-cli/tests/cli.rs`
  - the lane already has pre-Bun rejection coverage and Bun-backed green-path coverage
  - coverage is still centered on the bounded family-shaped roots plus same-tree nested chain3

## Step 0: Scope Challenge

### Premise correction

The current design pressure is correct. The loose phrase is not.

This phrase is too ambiguous:

```text
generic multi-dependency TypeScript execution
```

In this repo, that cannot honestly mean:

- any authored function with any dep count
- arbitrary function topology outside the shipped supported families
- recursive cross-library TypeScript graph execution

For M59 it means exactly this:

```text
same-tree local graph execution over the shipped supported function families,
with semantic review owning per-node shape truth
```

That is a bounded lake. The rest is ocean.

### What already exists

| Sub-problem | Existing owner or proof surface | M59 action |
| --- | --- | --- |
| root CLI entry point | `spec-cli/src/commands.rs` target-language flow | reuse |
| local and qualified dep parsing | `DepRef` in `spec-core/src/types.rs` and validator helpers | reuse |
| supported-function truth | `evaluate_semantic_review_with_context(...)` in `spec-core/src/semantic_review.rs` | reuse as authority |
| direct cross-library helper lane | `validate_typescript_helper_dep_contract(...)` | preserve unchanged |
| direct cross-library wrapper lane | `validate_typescript_wrapper_dep_contract(...)` | preserve unchanged |
| direct cross-library chain3 lane | `validate_typescript_chain3_dep_contract(...)` | preserve unchanged |
| cycle detection | `detect_cycles(...)` and `detect_qualified_cycles(...)` in `spec-core/src/validator.rs` | reuse |
| local closure dedupe | `included: BTreeSet<usize>` in `spec-core/src/typescript_backend.rs` | reuse |
| target-proof routing | `target_proofs.typescript` plumbing in passports and status | reuse |
| CLI regression harness | `spec-cli/tests/cli.rs` and existing fixture helpers | extend |

### Minimum complete slice

The minimum honest M59 slice is:

1. keep per-node supported shape owned by semantic review
2. add one explicit local graph lane for TypeScript execution
3. widen local root eligibility to the shipped supported function family set
4. validate the reachable local closure graph-wide instead of via a closure-member family table
5. preserve the direct cross-library helper, wrapper, and chain3 lanes exactly as shipped
6. prove shared-subgraph dedupe and unrelated-unit exclusion
7. update `README.md` and `TODOS.md` so "generic" now means graph-level genericity over shipped supported families, not arbitrary node-shape parity

Anything smaller is fake done.

Examples:

- docs-only widening is fake done
- backend-only traversal without validator proof-wall updates is fake done
- local graph validation without preserved cross-library regression proof is fake done

### Complexity and blast radius

Expected write scope:

- `PLAN.md`
- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`
- one dedicated local graph fixture tree under `spec-cli/tests/fixtures/`
- `README.md`
- `TODOS.md`

Expected non-write scope:

- `spec-core/src/semantic_review.rs`
- passport schema
- status/export schema
- CLI flags
- Bun runtime contract

This is the right size. It touches the lane owner files and public contract surfaces without inventing new infrastructure.

### TODOS cross-reference

`TODOS.md` currently says generic multi-dependency TypeScript execution is deferred.

After M59 lands, that defer must narrow to the actual remaining oceans:

- arbitrary authored 4+ dep function topology
- new supported semantic families
- generic recursive cross-library function graphs

The TODO must stop implying that all graph-shaped TypeScript execution is still out after M59.

### Completeness and distribution check

Choose the complete bounded version, not the shortcut.

The complete version includes:

1. local helper root proof
2. local monotone-down root proof
3. local graph-wide closure validation
4. shared-subgraph dedupe proof
5. preserved cross-library regression proof
6. public wording that removes ambiguity

No new distributable artifact is introduced. This is still an existing `spec` CLI capability widen:

- existing install path remains `cargo install spec-cli`
- existing GitHub Releases remain the distribution surface
- existing CI remains the build surface

## Milestone Contract

### Exact shipped behavior

After M59:

- any same-tree local `kind:function` unit may execute in the TypeScript lane when:
  - it authors non-empty `body.typescript`
  - it resolves to a supported semantic review
  - every reachable dep resolves from the loaded unit set
  - every reachable dep stays local, not `shared::...`
  - every reachable unit is `kind:function`
  - every reachable unit authors non-empty `body.typescript`
  - every reachable unit resolves to a supported semantic review
  - the reachable closure is acyclic
- the supported local root set for M59 is the shipped supported function family set already recognized by the repo for this lane
- direct cross-library TypeScript execution remains bounded and explicit:
  - one helper import lane
  - one wrapper direct-root lane
  - one chain3 direct-root lane
  - the same-tree nested chain3 slot-1 rule inside the chain3 portability lane
- generic local traversal is graph-driven, not closure-family-driven
- arbitrary per-node dep arity remains out because semantic review still rejects the broader ocean M59 is not trying to solve

### Correct definition of "generic" for M59

For this repo, M59 generic means:

```text
the closure walker is generic over the existing supported local function graph
```

It does not mean:

```text
any authored function with any dep list can now run in TypeScript
```

That second claim requires new semantic-family promotion work. M59 does not do that work.

### Exact allowed topology

Allowed M59 local graph:

```text
root (supported local function family)
├── local dep A (supported local function family)
│   ├── local dep C
│   └── local dep D
├── local dep B
│   └── local dep D    <- shared subgraph allowed, emitted once
└── local dep E
```

Where all of the following are true:

1. every node is `kind:function`
2. every edge is local to the loaded unit set
3. every node resolves to a supported semantic review
4. every node authors non-empty `body.typescript`
5. the closure is acyclic
6. only reachable nodes are emitted into the generated TypeScript tree
7. each node's authored shape is still bounded by the shipped supported semantic-review function families

### Exact topologies still out

M59 must keep these out:

- arbitrary 4+ dep authored function units
- any authored function whose dep topology falls outside the shipped supported semantic families
- any reachable node with `semantic_review.support_status = unsupported`
- any reachable node missing `body.typescript`
- any generic local-graph path containing `shared::...` edges
- generic recursive cross-library DAGs
- molecule TypeScript execution
- seam-kind TypeScript execution
- `spec validate --target-language`
- `spec export --target-language`
- any claim that TypeScript execution now has arbitrary function-graph parity

## NOT in scope

- arbitrary 4+ dep authored function units
  - rationale: semantic review still rejects them; pretending otherwise would be fake done
- new supported semantic families
  - rationale: M59 is executor-lane work, not family-governance work
- generic recursive cross-library graphs
  - rationale: that is a second widen and would blur the docs immediately
- molecule TypeScript execution
  - rationale: separate product surface with its own proof model
- seam-kind TypeScript execution
  - rationale: separate ontology and runtime contract
- `spec validate --target-language`
  - rationale: no need to widen CLI shape to ship M59
- `spec export --target-language`
  - rationale: same

## Architecture Review

### Current vs target lane shape

Current M58 shape:

```text
root target request
  -> classify root as monotone_up / wrapper / chain3
  -> validate direct dep tuple for that family
  -> allow closure members only from a hard-coded family allowlist
  -> recurse through family-specific branches
```

Target M59 shape:

```text
root target request
  |
  +-- direct or reachable shared:: dep required?
  |     |
  |     +-- yes -> existing M55-M58 portability lanes
  |     |          - helper direct-root lane
  |     |          - wrapper direct-root lane
  |     |          - chain3 direct-root lane
  |     |
  |     +-- no  -> new M59 local graph lane
  |                - root is local kind:function
  |                - root has supported semantic review
  |                - walk reachable local deps
  |                - every reachable node has supported semantic review
  |                - every reachable node has body.typescript
  |                - cycle rejection stays pre-Bun
  |                - emit reachable closure once
  |
  +-- Bun build/test
```

Still out:

```text
- arbitrary 4+ dep function nodes
- generic recursive cross-library DAGs
- molecule execution
- seam-kind execution
- validate/export target-language widening
```

### Lane-selection rule

The validator must make one explicit choice:

1. if the requested execution shape needs the existing direct cross-library helper, wrapper, or chain3 portability contract, route through the preserved M55-M58 validators
2. otherwise, if the root is a local `kind:function` unit, route through the new local graph lane
3. never try to stretch the old family-tuple validators to simulate generic local graph execution

That split keeps the code honest:

- portability remains an explicit exception surface
- local graph execution becomes the general same-tree surface

### Concrete module boundaries

```text
authored .unit.spec
        |
        v
spec-core/src/semantic_review.rs
  - supported-function truth
  - unsupported dep-topology truth
        |
        v
spec-core/src/validator.rs
  - lane selection
  - local graph proof wall
  - preserved cross-library portability wall
        |
        v
spec-core/src/typescript_backend.rs
  - reachable closure collection
  - shared-subgraph dedupe
  - TypeScript tree emission
        |
        v
spec-cli/tests/cli.rs
  - Bun-backed green path
  - pre-Bun red paths
  - preserved cross-library regressions
        |
        v
README.md / TODOS.md
  - public contract
```

### File-by-file implementation contract

| Surface | Current behavior | Required M59 change | Must stay true after the change |
| --- | --- | --- | --- |
| `spec-core/src/validator.rs` | root admission is family-shaped; local closure admission is tied to a closure-member family table | add a local graph-validation path driven by semantic-review support plus local-only closure rules | direct cross-library helper, wrapper, and chain3 roots still pass exactly as shipped |
| `spec-core/src/typescript_backend.rs` | root and closure traversal branch on root-family and closure-family arms | add a local graph collector that walks validated reachable local deps generically | shared-subgraph dedupe still uses `included`; unrelated units stay out |
| `spec-core/src/semantic_review.rs` | already owns supported dep-topology truth | no logic change required for M59 | the broader oceans remain unsupported |
| `spec-cli/tests/cli.rs` | proves family-shaped roots plus M58 nested chain3 | add local supported-root and local-graph proof, plus preserved cross-library regressions | all new failures still happen before Bun |
| `README.md` | TypeScript lane wording is narrower than the corrected M59 contract | rewrite the local lane versus portability lane wording | docs must not imply arbitrary node-shape parity |
| `TODOS.md` | remaining TypeScript defer is still coarse | narrow the remaining backlog to the real oceans left after M59 | do not erase honest backlog |

### Public wording contract

`README.md` must say three things clearly:

1. local roots may now be any shipped supported local function family with `body.typescript`
2. local traversal is semantic-review-driven and same-tree only
3. direct cross-library execution is still limited to the existing helper, wrapper, and chain3 portability lanes

`TODOS.md` must stop using "generic multi-dependency TypeScript execution" as a fuzzy bucket and instead name the real remaining work:

- arbitrary authored 4+ dep function topology
- new supported semantic families
- generic cross-library recursive function graphs

### Opinionated architecture recommendation

Keep this implementation boring.

Recommended shape:

1. preserve the current direct cross-library helper, wrapper, and chain3 validators as the portability lane
2. add one explicit local graph validator path instead of stretching the old family helpers
3. add one explicit local graph collector instead of stacking more special cases into the current family-shaped collector
4. keep semantic review as the single owner of per-node supported topology truth

Do not build:

- a new generic graph registry
- a target-language-specific semantic-family clone
- a second cycle detector

## Code Quality Review

### Keep the diff minimal

Minimal-diff rules:

- preserve the existing cross-library helper, wrapper, and chain3 validator helpers
- add one local graph validation entry instead of refactoring the entire validator subsystem
- add one local graph collection entry instead of teaching the current family-specific collector new tricks forever
- keep proof in existing unit and integration test files, plus one dedicated local graph fixture surface

### DRY rule

Avoid duplicating "supported TypeScript function unit" truth across semantic review, validator, backend, tests, and docs.

The right DRY level is:

- semantic review owns per-node supported-family truth
- validator owns lane selection plus pre-Bun contract checks
- backend owns closure collection only

The wrong DRY level is:

- re-encoding supported family rules again in backend traversal
- growing the closure-member family table into a second semantic-review system

### Error-handling and naming rule

All new rejection paths must fail before Bun runs and must use precise lane wording.

Required behavior:

- local graph lane plus reachable `shared::...` dep -> explicit local-lane portability rejection
- local graph lane plus unsupported deep member -> explicit supported-semantic-review rejection
- local graph lane plus missing deep `body.typescript` -> explicit pre-Bun TypeScript-body rejection
- local graph lane plus cycle -> existing cycle error surface, still pre-Bun
- local root with unsupported dep topology -> semantic-review unsupported topology, not a vague "generic graph" error

Also fix milestone drift in error strings where needed. Leaving stale `M52` language in M59-only codepaths is sloppy and will confuse users.

### Diagram maintenance rule

If nearby ASCII diagrams or comments in `typescript_backend.rs` or `validator.rs` become stale, update them in the same change. The current module banner in `typescript_backend.rs` will be stale after M59 and must be rewritten.

## Test Review

100% coverage is the goal for the new behavior slice.

### Test framework and proof surfaces

- runtime: Rust / Cargo
- unit tests: inline `#[test]` in `spec-core`
- integration tests: `spec-cli/tests/cli.rs`
- live TypeScript execution proof: Bun-backed CLI integration tests

Do not overload a semantic-family packet fixture to prove a graph-lane contract it was not built to explain.

Recommended proof surface:

- add a dedicated local graph fixture tree under `spec-cli/tests/fixtures/`

Why:

- M59 is topology and lane-selection work, not family-packet promotion work
- the proof needs multiple supported families in one local graph
- the fixture should be obviously local-only and graph-oriented

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/validator.rs
    │
    ├── validate_typescript_execution_target_spec_with_specs(...)
    │   ├── [EXISTING] family-shaped direct cross-library root lanes
    │   ├── [GAP]      local helper root passes
    │   ├── [GAP]      local monotone-down root passes
    │   ├── [GAP]      local wrapper root passes through local graph lane
    │   ├── [GAP]      local chain3 root passes through local graph lane
    │   ├── [GAP]      local root with unsupported dep topology still rejects
    │   └── [GAP]      local reachable shared:: dep rejects before Bun
    │
    └── local reachable-closure validation
        ├── [GAP]      unsupported deep member rejects
        ├── [GAP]      missing deep body.typescript rejects
        ├── [GAP]      cycle rejects before Bun
        └── [GAP]      missing reachable dep rejects before Bun

[+] spec-core/src/typescript_backend.rs
    │
    ├── local graph closure collection
    │   ├── [GAP]      reachable local closure is included
    │   ├── [GAP]      shared subgraph is emitted once
    │   └── [GAP]      unrelated loaded units stay excluded
    │
    └── legacy portability lane collection
        └── [REGRESSION] M55-M58 helper, wrapper, and chain3 roots still behave unchanged
```

### User-flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Local root execution
    │
    ├── [GAP] [→CLI] helper root executes with Bun
    ├── [GAP] [→CLI] monotone-down root executes with Bun
    ├── [GAP] [→CLI] monotone-up root with helper executes with Bun
    ├── [GAP] [→CLI] wrapper root executes with Bun
    └── [GAP] [→CLI] chain3 root executes with Bun

[+] Local graph contract rejections
    │
    ├── [GAP] [→CLI] deep member missing body.typescript rejects before Bun
    ├── [GAP] [→CLI] deep member unsupported by semantic review rejects before Bun
    ├── [GAP] [→CLI] local cycle rejects before Bun
    ├── [GAP] [→CLI] reachable shared:: dep rejects before Bun
    └── [GAP] [→CLI] unsupported authored topology still rejects before Bun

[+] Regression wall
    │
    ├── [REGRESSION] existing cross-library helper root still passes
    ├── [REGRESSION] existing cross-library wrapper root still passes
    └── [REGRESSION] existing cross-library chain3 root still passes
```

### Required proof additions by surface

1. `spec-core/src/validator.rs`
   - add local-root admission coverage for helper and monotone-down
   - add local generic closure rejection coverage for:
     - deep unsupported member
     - deep missing `body.typescript`
     - reachable `shared::...` dep
     - unsupported authored topology still rejected
   - keep direct cross-library helper, wrapper, and chain3 coverage green
2. `spec-core/src/typescript_backend.rs`
   - add unit coverage proving:
     - local reachable closure inclusion
     - shared-subgraph dedupe
     - unrelated-unit exclusion
     - legacy portability-lane closure remains unchanged
3. `spec-cli/tests/cli.rs`
   - add Bun-backed green-path coverage for local helper, monotone-down, monotone-up, wrapper, and chain3 roots
   - add pre-Bun local graph rejection coverage for the red paths above
   - preserve M55-M58 cross-library green paths
4. `spec-cli/tests/fixtures/typescript_local_supported_graph/`
   - add one local graph fixture tree that includes:
     - helper root
     - monotone-down root
     - monotone-up root with helper
     - wrapper root
     - chain3 root
     - one shared-subgraph reuse path

### Maintained fixture shape

Use one dedicated local graph fixture tree with obvious same-tree ids.

Suggested shape:

```text
units/
├── money/
│   └── round.unit.spec
└── pricing/
    ├── apply_discount.unit.spec
    ├── apply_tax.unit.spec
    ├── calculate_total.unit.spec
    ├── checkout_total.unit.spec
    └── display_total.unit.spec
```

What this proves:

- helper root eligibility
- monotone-down root eligibility
- local graph traversal across the shipped supported function families already admitted by the lane
- shared-subgraph dedupe through reused local members

### Regression rule

This is a regression-sensitive lane change.

That means:

- every new green path lands with proof in the same change
- every preserved M55-M58 portability lane is exercised in the same change
- no doc update ships before the regression wall is green

### Test plan artifact

Write the QA handoff artifact here:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260514-141636.md`

That artifact is part of this planning output, not follow-up garnish.

### QA handoff checklist

Affected commands and proofs:

- `spec test <local-helper-root.unit.spec> --target-language typescript`
- `spec test <local-monotone-down-root.unit.spec> --target-language typescript`
- `spec test <local-wrapper-root.unit.spec> --target-language typescript`
- `spec test <local-chain3-root.unit.spec> --target-language typescript`
- `spec build <local-graph-fixture-dir> --target-language typescript`
- `spec status <local-graph-fixture-dir> --target-language typescript`

Critical behaviors to verify:

- local helper root is now valid in the TypeScript lane
- local monotone-down root is now valid in the TypeScript lane
- local graph traversal includes exactly the reachable local closure
- repeated reachable members are emitted once
- deep unsupported or un-authored members reject before Bun
- direct cross-library helper, wrapper, and chain3 roots remain green

## Performance Review

M59 must not worsen the asymptotic class of the TypeScript tree walk.

Expected characteristics:

- traversal remains bounded to reachable loaded units
- `included: BTreeSet<usize>` continues to dedupe repeated reachable members
- no new global scans per recursive step
- cycle rejection continues to happen before codegen

Performance rules:

- do not recompute semantic review repeatedly for the same node unless the surrounding callsite already has that truth in hand
- do not add a second dedupe structure beside `included`
- do not turn local graph validation into an O(n^2) repeated closure walk
- do not regress the fast path for the existing cross-library portability lanes

## Failure Modes

| New codepath | Real failure | Test covers it? | Error handling exists? | User-visible outcome |
| --- | --- | --- | --- | --- |
| local root admission | helper or monotone-down root still rejected by stale root-family logic | required | required | clear pre-Bun rejection if broken |
| local graph closure validation | unsupported deep member slips through because only the root is checked | required | required | misleading Bun or runtime failure if broken |
| local graph closure validation | deep member missing `body.typescript` reaches codegen | required | required | clear pre-Bun rejection required |
| local graph portability wall | reachable `shared::...` dep is silently admitted into the local lane | required | required | scope explosion and dishonest docs |
| closure collection | shared local subgraph is emitted twice | required | required | duplicate module or import noise |
| closure boundary | unrelated loaded units leak into the generated tree | required | required | silent over-inclusion if untested |
| regression wall | direct cross-library wrapper or chain3 roots route through the wrong lane and break | required | required | real user regression |

Critical gap rule:

If any of the first five rows lands without direct proof, M59 is not ready.

## Implementation Sequence

### 1. Freeze the contract

Touched surfaces:

- `PLAN.md`

Required outcome:

- this file becomes the single implementation authority
- no stale "arbitrary generic graph" wording remains

### 2. Add the dedicated local graph proof surface

Touched surfaces:

- `spec-cli/tests/fixtures/`
- `spec-cli/tests/cli.rs`

Required outcome:

- one maintained local fixture tree exists
- fixture ids are stable before validator and backend assertions get wired to them

### 3. Split the local graph lane from the portability lane in the validator

Touched surfaces:

- `spec-core/src/validator.rs`

Required outcome:

- direct cross-library helper, wrapper, and chain3 validators remain intact
- one explicit local-root path is added in `validate_typescript_execution_target_spec_with_specs(...)`
- lane selection is now local-graph versus portability, not "more cases in the old root-family switch"

### 4. Validate the local reachable closure graph-wide

Touched surfaces:

- `spec-core/src/validator.rs`

Required outcome:

- semantic review is reused for supported-family truth
- cycle detection is reused
- reachable non-local deps reject
- reachable unsupported semantic review rejects
- reachable missing `body.typescript` rejects

### 5. Replace local family-shaped closure traversal with dep-driven traversal

Touched surfaces:

- `spec-core/src/typescript_backend.rs`

Required outcome:

- a local graph collector walks reachable local deps generically
- `included` remains the dedupe mechanism
- the preserved portability-lane collector behavior stays intact

### 6. Lock the proof wall

Touched surfaces:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`

Required outcome:

- validator tests exist
- backend tree tests exist
- Bun-backed CLI green paths exist
- CLI red paths for local-graph rejections exist
- direct cross-library regressions are rerun

### 7. Rewrite public wording

Touched surfaces:

- `README.md`
- `TODOS.md`

Required outcome:

- docs distinguish the new local graph lane from the preserved cross-library portability lanes
- docs explicitly say arbitrary node-shape parity is still out
- the TODO backlog now names the real remaining oceans

## Acceptance Commands

Run these before calling M59 done:

```bash
cargo test -p spec-core typescript
cargo test -p spec-cli --test cli typescript
```

Then run the maintained local graph proof directly:

```bash
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/checkout_total.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/money/round.unit.spec --target-language typescript
cargo run -p spec-cli -- test spec-cli/tests/fixtures/typescript_local_supported_graph/units/pricing/apply_discount.unit.spec --target-language typescript
```

Then run the preserved portability regressions:

```bash
cargo test -p spec-cli --test cli typescript_cross_library
```

Finally run the full lane-facing regression surface:

```bash
cargo test -p spec-core
cargo test -p spec-cli --test cli
```

Acceptance is not complete until all four gates are green:

1. local supported-family roots pass
2. local graph-wide rejection wall stays pre-Bun
3. shared-subgraph dedupe and unrelated-unit exclusion are proven
4. existing cross-library helper, wrapper, and chain3 lanes still pass unchanged

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| local graph fixture authoring and CLI proof | `spec-cli/tests/`, `spec-cli/tests/fixtures/` | — |
| validator lane split and local graph validation | `spec-core/src/` | — |
| backend local graph collector and dedupe proof | `spec-core/src/` | validator lane split |
| docs and backlog sync | repo-root docs | validator lane split, backend collector, CLI proof |

### Parallel lanes

- Lane A: validator lane split -> backend local graph collector
  - sequential, shared `spec-core/src/`
- Lane B: local graph fixture authoring and CLI proof
  - parallelizable, but it must coordinate fixture ids and final error wording with Lane A
- Lane C: docs and backlog sync
  - sequential after A and B merge, because public wording must match shipped behavior exactly

### Execution order

Launch Lane A and Lane B in parallel worktrees.

Merge both only after:

1. local fixture ids are locked
2. validator wording is stable
3. CLI proof expectations match the final lane split

Then run Lane C for `README.md` and `TODOS.md`.

### Conflict flags

- Lane A and Lane B do not share files, but they do share:
  - fixture ids
  - expected error wording
  - exact definition of local-versus-portability lane selection
- do not split validator and backend into separate worktrees, both live in `spec-core/src/`
- do not let either parallel lane edit `README.md`; keep docs in Lane C

### Recommended ownership

- Lane A owner: `spec-core/src/`
- Lane B owner: `spec-cli/tests/` and `spec-cli/tests/fixtures/`
- Lane C owner: repo-root docs only

If a worker needs to change a module outside its lane, it should stop and hand the change back rather than creating cross-lane merge conflict bait.

## Completion Summary

- Step 0: Scope Challenge
  - scope accepted as semantic-review-driven same-tree local graph execution over shipped supported families only
- Architecture Review
  - one critical clarification locked: graph-generic does not mean arbitrary node-shape generic
- Code Quality Review
  - explicit lane split required, no second semantic-review system
- Test Review
  - dedicated local graph proof surface required
  - helper and monotone-down root proofs required
  - shared-subgraph dedupe proof required
  - preserved cross-library regression wall required
- Performance Review
  - no new asymptotic risk if traversal stays reachable-only and deduped
- NOT in scope
  - written
- What already exists
  - written
- Failure modes
  - written with critical-gap rule
- Parallelization
  - 3 lanes total
  - 1 parallel launch wave
  - 2 sequential follow-on stages
- Lake Score
  - the complete bounded option was chosen over the shortcut on every major decision

## Why This Plan Wins

It turns the design doc into an execution contract the codebase can actually honor.

It keeps the good ambition:

- stop hard-coding the local TypeScript lane as a few special families forever

It removes the bad ambiguity:

- no fake promise that arbitrary authored function graphs suddenly work

It preserves what users already have:

- direct cross-library helper, wrapper, and chain3 roots

And it gives implementation a clean ownership split:

- semantic review owns node truth
- validator owns lane selection and pre-Bun contract checks
- backend owns closure collection
- CLI proof owns end-to-end confidence

That is specific enough to build, test, and ship without lying in the docs.
