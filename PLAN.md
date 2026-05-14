# M58: Bounded Nested Chain3 Closure TypeScript Execution Plan

Status: **authoritative implementation plan**  
Milestone: **M58**  
Milestone family: **second-language-backend**  
Implementation readiness: **ready for bounded execution**  
Plan scope: **ship exactly one new TypeScript capability: a chain3 root may use a same-tree `function.wrapper.pipeline.chain3.v1` in chain3 direct dep slot 1, and the backend may recurse through that validated nested chain3 closure, without widening to generic multi-dependency graphs, molecule execution, seam kinds, or target-language validate/export**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Validated at commit: **`6c7caf3`**  
Last rewritten: **2026-05-14**

Supersedes:

- the stale M57 shared-core closeout plan previously maintained at this path
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260514-074521.md`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260514-074521.md`
- `README.md`
- `TODOS.md`
- `AGENTS.md`
- `ORCH_PLAN.md`

Primary repo surfaces:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/*`
- `README.md`
- `TODOS.md`

## Executive Summary

The old repo-root plan was solving the wrong problem.

M56 already shipped bounded TypeScript execution for:

1. helper-aware monotone-up roots
2. wrapper roots
3. flat chain3 roots
4. direct cross-library helper, wrapper, and chain3 roots

M57 already closed the shared-core ownership loop. The remaining gap is narrower and more concrete:

```text
outer_chain3
├── nested_chain3        <- still rejected today
├── monotone_up
└── monotone_down
```

The current lane cannot execute that shape because the validator still freezes chain3 slot 1 to `function.wrapper.pipeline.v1` and still rejects `function.wrapper.pipeline.chain3.v1` as a closure member.

M58 fixes exactly that wall. Nothing else.

The contract after M58 is:

```text
chain3 slot 1 = wrapper OR same-tree chain3
chain3 slot 2 = monotone_up
chain3 slot 3 = monotone_down_nonnegative
```

Everything outside that shape stays out:

- no generic DAG policy
- no arbitrary recursive graphs
- no cross-library recursive chain3 closure
- no target-language validate/export
- no molecule TypeScript execution

This plan is therefore not “generic multi-dependency TypeScript execution.” It is one bounded recursive family widen in the existing Bun-backed lane.

## Current Validated Basis

Validated from HEAD source review on `feat/m40-plus` at `6c7caf3`.

Observed repo truth:

- `README.md`
  - flat TypeScript chain3 roots are documented as supported
  - generic multi-dependency TypeScript execution is still explicitly deferred
  - nested chain3 closure members are still explicitly unsupported
- `TODOS.md`
  - M52 through M56 are recorded as complete
  - generic multi-dependency TypeScript execution remains deferred
- `spec-core/src/validator.rs`
  - `validate_typescript_execution_target_spec_with_specs(...)` admits chain3 roots only when direct dep slot 1 resolves to `function.wrapper.pipeline.v1`
  - `validate_typescript_closure_member_spec_with_specs(...)` rejects closure members with compatibility key `function.wrapper.pipeline.chain3.v1`
  - the current proof surface includes `typescript_closure_member_rejects_chain3_member`
- `spec-core/src/typescript_backend.rs`
  - wrapper closure recursion is already implemented
  - closure collection already recurses over validated members
  - chain3 closure recursion is unreachable today because validation blocks the shape first
  - unrelated loaded units are already kept out through `included: BTreeSet<usize>`
- `spec-cli/tests/cli.rs`
  - the TypeScript lane already has strong pre-Bun rejection coverage for wrong family, wrong dep order, missing `body.typescript`, and molecule rejection
  - Bun-backed success coverage exists for flat chain3, not nested chain3
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/*`
  - the maintained aligned proof pack is flat
  - no maintained same-tree nested chain3 proof exists yet

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing owner or proof surface | M58 action |
| --- | --- | --- |
| root eligibility for flat chain3 | `validate_typescript_chain3_dep_contract(...)` in `spec-core/src/validator.rs` | widen slot 1 only, keep slots 2 and 3 frozen |
| closure-member gating | `validate_typescript_closure_member_spec_with_specs(...)` | admit chain3 only through the exact same bounded slot-1 contract |
| recursive tree inclusion | `collect_typescript_closure_member(...)` in `spec-core/src/typescript_backend.rs` | extend recursion to validated nested chain3 members |
| duplicate suppression | `included: BTreeSet<usize>` in `typescript_backend.rs` | reuse unchanged |
| flat chain3 proof harness | `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/*` | add one maintained nested aligned path, not a second fixture system |
| CLI TypeScript regression harness | `spec-cli/tests/cli.rs` | add nested green-path and preserved red-path coverage |
| public contract wording | `README.md`, `TODOS.md` | rewrite exactly one TypeScript-lane claim, keep generic multi-dep deferred |

### Minimum complete slice

The minimum honest M58 slice is:

1. replace the stale repo-root M57 plan with this M58 contract
2. add one maintained same-tree nested chain3 proof shape inside the existing aligned fixture pack
3. widen TypeScript chain3 slot 1 to allow `function.wrapper.pipeline.v1` or same-tree `function.wrapper.pipeline.chain3.v1`
4. allow chain3 as a closure member only when that exact bounded slot-1 contract holds
5. recurse through validated nested chain3 members in the generated TypeScript tree
6. preserve all existing pre-Bun rejection behavior for out-of-contract recursive shapes
7. update `README.md` and `TODOS.md` so public wording says exactly what shipped

Anything smaller is fake done.

Examples:

- changing validator admission without backend recursion is fake done
- making one nested fixture pass without preserved rejection coverage is fake done
- shipping code without updating the README nested-chain3 sentence is fake done

### Complexity and blast radius

This milestone stays small only if it remains inside the existing TypeScript lane surfaces.

Expected write scope:

- `PLAN.md`
- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`
- `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/*.unit.spec`
- `README.md`
- `TODOS.md`

No new crate. No new command. No new artifact type. No new schema. No new runtime.

The diff will touch more files than the logic size suggests because proof fixtures and docs are part of the product surface here. That is acceptable. A new abstraction layer or a new fixture universe is not.

### Search check

No unfamiliar framework or infra component enters the repo in M58.

- **[Layer 1]** reuse the existing validator helpers and TypeScript-lane constants in `spec-core/src/validator.rs`
- **[Layer 1]** reuse the existing recursive inclusion walker and `included` dedupe set in `spec-core/src/typescript_backend.rs`
- **[Layer 1]** reuse the existing chain3 aligned fixture pack instead of inventing a new proof surface
- **[Layer 3]** the first-principles insight is that nested chain3 is only reachable by widening chain3 slot 1, not by adding “generic closure support”

### TODOS cross-reference

This plan must preserve the current deferred boundary in `TODOS.md`:

- generic multi-dependency TypeScript execution stays deferred
- no second follow-on is created unless implementation pressure proves a need for slot-2/slot-3 recursion or cross-library recursive closure

M58 must not silently absorb the generic DAG milestone.

### Completeness check

Choose the complete bounded version, not the shortcut.

The complete version is still small:

1. green path for same-tree nested chain3
2. preserved red paths for wrong nested shapes
3. updated docs and backlog wording

The shortcut is “make one fixture pass and skip the rejection wall.” That saves almost nothing and leaves the lane undefined. Not acceptable.

### Distribution check

No new distributable artifact is introduced.

This remains an existing `spec` CLI capability widen:

- `cargo install spec-cli`
- existing GitHub Releases
- existing CI
- existing target-proof storage in passports

## Milestone Contract

### Exact shipped behavior

After M58:

- a TypeScript chain3 root may use a same-tree chain3 unit in direct dep slot 1
- that nested chain3 unit may itself recurse again through the same bounded rule
- every nested chain3 still uses the frozen ordered tuple:
  - slot 1 = `function.wrapper.pipeline.v1` or same-tree `function.wrapper.pipeline.chain3.v1`
  - slot 2 = `function.arithmetic_leaf.monotone_up.v1`
  - slot 3 = `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- every reachable direct or recursive member must author non-empty `body.typescript`
- recursive closure collection stays limited to reachable validated members in the loaded unit set

### Same-tree rule

“Same-tree” means:

- the nested chain3 dep is authored as a local dep id like `pricing/base_nested_chain3_aligned`
- it does not use a library-qualified dep such as `shared::pricing/base_nested_chain3_aligned`
- it resolves from the same loaded unit set as the root
- the validator enforces that locality on the slot-1 recursive path, not by path heuristics in the backend

### In scope

- widen TypeScript chain3 slot 1 from wrapper-only to wrapper-or-same-tree-chain3
- keep that widen bounded to the TypeScript lane only
- preserve slot 2 as monotone-up and slot 3 as monotone-down-nonnegative
- allow a chain3 closure member only when it satisfies the exact bounded slot-1 contract
- recurse through validated nested chain3 members during TypeScript tree generation
- keep helper recursion behavior unchanged under monotone-up and monotone-down members
- add one maintained nested aligned proof path in the existing chain3 family fixture pack
- add CLI proof for:
  - nested green path
  - wrong first-slot family rejection
  - wrong dep order rejection
  - missing nested `body.typescript` rejection
  - cross-library nested chain3 rejection
- update `README.md` and `TODOS.md` to name the new bounded recursive slice precisely

### Not in scope

- generic multi-dependency TypeScript execution
- arbitrary supported-function DAG execution
- widening any slot other than chain3 slot 1
- cross-library recursive chain3 closure as a general rule
- molecule TypeScript execution
- seam-kind TypeScript execution
- `spec validate --target-language`
- `spec export --target-language`
- passport, status, or export schema changes
- new semantic-family promotion work
- `ORCH_PLAN.md` refresh as part of this plan-only rewrite

## Locked Decisions

These are not open questions for M58:

1. Bun remains the only TypeScript runtime contract.
2. The TypeScript lane still speaks in family language, not generic graph language.
3. The recursive widen is only:
   - chain3 slot 1
   - same loaded tree
   - same promoted family set
4. The milestone is an execution-contract widen, not a semantic-review model rewrite.
5. Public wording must continue to say generic multi-dependency execution is deferred.
6. The backend must not infer locality or reachability that the validator did not already approve.
7. The implementation stays explicit. No generic “allowed family graph” registry for M58.

## Architecture Review

### Current vs target lane shape

Current shape:

```text
outer_chain3 root
├── slot 1: wrapper only
├── slot 2: monotone_up
└── slot 3: monotone_down

closure members allowed:
- wrapper
- monotone_up
- monotone_down
- helper

closure members rejected:
- chain3
```

Target M58 shape:

```text
outer_chain3 root
├── slot 1: wrapper OR same-tree chain3
│   ├── if wrapper:
│   │   ├── monotone_down
│   │   └── monotone_up
│   │       └── helper?
│   └── if chain3:
│       ├── slot 1: wrapper OR same-tree chain3
│       ├── slot 2: monotone_up
│       │   └── helper?
│       └── slot 3: monotone_down
├── slot 2: monotone_up
│   └── helper?
└── slot 3: monotone_down
    └── helper?
```

Still out:

```text
- chain3 in slot 2
- chain3 in slot 3
- arbitrary new families in any slot
- cross-library recursive chain3
- unrelated loaded units entering the TypeScript tree
```

### Concrete module boundaries

```text
authored .unit.spec
        |
        v
spec-core/src/validator.rs
  - validate_typescript_execution_target_spec_with_specs(...)
  - validate_typescript_chain3_dep_contract(...)
  - validate_typescript_closure_member_spec_with_specs(...)
        |
        v
spec-core/src/typescript_backend.rs
  - collect_typescript_root_closure(...)
  - collect_typescript_closure_member(...)
  - generate_typescript_tree(...)
        |
        v
spec-cli/tests/cli.rs
  - pre-Bun rejection tests
  - Bun-backed success tests
        |
        v
README.md / TODOS.md
  - public contract
```

### File-by-file implementation contract

| Surface | Current behavior | Required M58 change | Must stay true after the change |
| --- | --- | --- | --- |
| `spec-core/src/validator.rs` | chain3 roots require slot 1 = wrapper; closure members reject chain3 | widen slot 1 only, and admit chain3 closure members only when the same bounded slot-1 contract holds | slots 2 and 3 stay frozen; cross-library recursive chain3 still rejects before Bun |
| `spec-core/src/typescript_backend.rs` | wrapper recursion works; chain3 recursion is unreachable | recurse through validated nested chain3 members | unrelated loaded units still stay out of the generated tree |
| `spec-cli/tests/cli.rs` | flat chain3 green path and red paths exist | add nested green path and preserved recursive rejection wall | all new recursive rejections happen before Bun |
| `semantic-families/.../fixtures/aligned/*` | flat aligned chain3 fixture only | add one maintained nested aligned path | no second fixture universe, no cross-library recursive proof |
| `README.md` | nested chain3 closure explicitly unsupported | replace that sentence with the exact bounded recursive rule | generic multi-dep still explicitly deferred |
| `TODOS.md` | generic multi-dep deferred | keep the defer line, do not overclaim M58 | M58 is not recorded as generic graph support |

### Opinionated architecture recommendation

Keep this implementation boring.

Recommended shape:

1. split slot-1 handling from slot-2 and slot-3 handling inside `validate_typescript_chain3_dep_contract(...)`
2. keep slot-2 and slot-3 validation exactly as they are
3. extend the closure-member matcher with one explicit `chain3` arm
4. extend `collect_typescript_closure_member(...)` with one explicit `chain3` recursion branch

Do not build a generic “TypeScript family graph” registry. This repo does not need that abstraction yet. M58 spends no innovation tokens on cleverness.

### Public wording delta

`README.md` must stop saying nested chain3 closure is unsupported and instead say the exact new rule:

- chain3 roots still require exactly three direct deps
- slot 1 may be wrapper or same-tree chain3
- slot 2 and slot 3 stay fixed to monotone-up then monotone-down-nonnegative
- recursive chain3 closure is same-tree only
- generic multi-dependency execution is still unsupported

`TODOS.md` must keep the deferred line aimed at generic multi-dependency TypeScript execution. Do not replace that defer with language that sounds like graph parity shipped.

## Code Quality Review

### Keep the diff minimal

Minimal-diff recommendation:

- modify the existing chain3 contract helper, do not invent a new validator subsystem
- add the recursion branch inside `collect_typescript_closure_member(...)`, do not refactor the whole inclusion walker
- keep proof in existing unit-test and CLI-test files
- reuse the existing aligned chain3 fixture directory

### DRY rule

Avoid duplicating “supported TypeScript family” logic across three places in slightly different words.

The right DRY level is small and local:

- one explicit slot-1 contract helper in `validator.rs`
- one explicit chain3 recursion branch in `typescript_backend.rs`
- one shared phrasing update for the recursive contract error surface

The wrong DRY level is a new generic abstraction that hides which exact families M58 admits.

### Error-handling and naming rule

All new rejection paths must fail before Bun runs and must stay lane-specific.

Required behavior:

- nested chain3 wrong slot or wrong family: TypeScript lane error, not a generic semantic-review failure
- cross-library nested chain3: explicit rejection that says recursive chain3 remains same-tree only in M58
- missing nested `body.typescript`: same pre-Bun contract failure class as existing M55/M56 errors

Where semantics changed, update stale `in M55` / `in M56` wording to `in M58`. Do not leave the new recursive rule guarded by stale milestone text.

### Diagram maintenance rule

If any nearby ASCII diagram or explanatory comment in touched files becomes stale, update it in the same change. Stale diagrams are worse than no diagrams.

## Test Review

100% coverage is the target for the new behavior slice.

### Test framework

- runtime: Rust / Cargo
- unit tests: inline `#[test]` in `spec-core`
- integration tests: `spec-cli/tests/cli.rs`
- live TypeScript execution proof: Bun-backed CLI integration tests

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/validator.rs
    │
    ├── validate_typescript_chain3_dep_contract(...)
    │   ├── [EXISTING] flat chain3 slot 1 == wrapper, slot 2 == up, slot 3 == down
    │   ├── [GAP]      slot 1 == same-tree nested chain3 should pass
    │   ├── [GAP]      slot 1 == cross-library nested chain3 should fail
    │   ├── [GAP]      slot 1 == wrong family should fail with M58 wording
    │   └── [GAP]      nested chain3 missing body.typescript should fail pre-Bun
    │
    └── validate_typescript_closure_member_spec_with_specs(...)
        ├── [EXISTING] wrapper closure member passes
        ├── [EXISTING] chain3 closure member fails
        └── [GAP]      chain3 closure member passes only under bounded M58 contract

[+] spec-core/src/typescript_backend.rs
    │
    ├── collect_typescript_closure_member(...)
    │   ├── [EXISTING] wrapper recursion
    │   ├── [EXISTING] helper recursion
    │   ├── [GAP]      nested chain3 recursion
    │   └── [GAP]      unrelated loaded units still excluded when nested chain3 exists
    │
    └── generate_typescript_tree(...)
        └── [GAP]      nested chain3 tree emits outer root + nested chain3 + reachable deps only
```

### User-flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Happy path
    │
    └── [GAP] [→CLI] spec test <nested_chain3_unit> --target-language typescript passes

[+] Contract rejections
    │
    ├── [GAP] [→CLI] nested chain3 with wrong first-slot family fails before Bun
    ├── [GAP] [→CLI] nested chain3 with wrong dep order fails before Bun
    ├── [GAP] [→CLI] nested chain3 with missing nested body.typescript fails before Bun
    └── [GAP] [→CLI] nested cross-library chain3 fails before Bun

[+] Tree integrity
    │
    └── [GAP] [→UNIT] unrelated loaded units stay out of generated TypeScript tree
```

### Required proof additions by file

1. `spec-core/src/validator.rs`
   - replace the current “chain3 closure member rejects” proof with bounded recursive admission coverage:
     - same-tree nested chain3 allowed
     - cross-library nested chain3 rejected
     - wrong first-slot family rejected
     - missing nested `body.typescript` rejected
   - keep flat-chain3 root coverage green
2. `spec-core/src/typescript_backend.rs`
   - add unit coverage proving:
     - nested chain3 closure inclusion works
     - unrelated loaded units still stay excluded
     - repeated reachable members are still deduped
3. `spec-cli/tests/cli.rs`
   - add Bun-backed success coverage for the maintained nested aligned fixture
   - add pre-Bun rejection coverage for:
     - wrong nested first-slot family
     - wrong nested dep order
     - missing nested `body.typescript`
     - cross-library nested chain3
4. `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/`
   - add one outer nested-chain3 root unit
   - add one nested chain3 member unit
   - reuse the existing aligned wrapper and leaf units

### Maintained fixture shape

Use the existing aligned bucket. Add exactly two new units:

```text
pricing/checkout_nested_chain3_aligned
├── pricing/base_nested_chain3_aligned      (chain3, local)
├── pricing/pricing_tax_leaf_aligned        (up)
└── pricing/pricing_discount_leaf_aligned   (down)

pricing/base_nested_chain3_aligned
├── pricing/pricing_total_wrapper_aligned   (wrapper)
├── pricing/pricing_tax_leaf_aligned        (up)
└── pricing/pricing_discount_leaf_aligned   (down)
```

That proves the exact recursive slot-1 contract and nothing broader.

### Regression rule

This is a regression-style contract change against the old rejection wall.

That means the green nested same-tree case lands with regression proof in the same change. No deferral.

### QA handoff checklist

Affected CLI commands and proofs:

- `spec test <nested-chain3-unit> --target-language typescript`
- `spec build <aligned fixture root> --target-language typescript`
- `spec status <aligned fixture root> --target-language typescript`

Critical behaviors to verify:

- same-tree nested chain3 executes successfully
- recursive dep order stays frozen
- cross-library recursive chain3 rejects before Bun
- missing nested `body.typescript` rejects before Bun
- unrelated loaded units never leak into the emitted TypeScript tree

## Performance Review

This change must not alter the asymptotic class of the TypeScript tree walk.

Expected characteristics:

- traversal remains bounded to reachable loaded units
- `included: BTreeSet<usize>` continues to dedupe repeated reachable members
- recursion depth is bounded by the validated acyclic authored graph
- no new cache is needed

Performance risks to avoid:

- do not rescan the entire loaded set to find nested members
- do not add a second inclusion structure beside `included`
- do not emit duplicate modules for repeated nested chain3 reuse

## Failure Modes

| New codepath | Real failure | Test covers it? | Error handling exists? | User-visible outcome |
| --- | --- | --- | --- | --- |
| nested chain3 slot-1 admission | cross-library nested chain3 is admitted by mistake | required | required | clear rejection before Bun |
| closure-member recursion | nested chain3 reachable deps are not included | required | compile/test proof | build failure with specific lane context |
| dedupe and inclusion boundary | unrelated loaded units leak into generated tree | required | unit-test proof | silent over-inclusion if untested |
| missing nested TypeScript body | runtime hits Bun instead of rejecting early | required | required | clear rejection before Bun |
| wrong nested dep order | lane accepts a graph outside the frozen family contract | required | required | clear rejection before Bun |

Critical gap rule:

If any one of the first four rows lands without a direct test, M58 is not ready.

## Implementation Sequence

### 1. Freeze authority

- keep this `PLAN.md` as the repo-root contract
- leave `ORCH_PLAN.md` untouched for now

### 2. Add the maintained nested aligned proof

- add `pricing/base_nested_chain3_aligned.unit.spec`
- add `pricing/checkout_nested_chain3_aligned.unit.spec`
- keep the proof same-tree and local
- reuse existing aligned wrapper and leaf units

### 3. Widen validator slot 1 only

- update `validate_typescript_chain3_dep_contract(...)`
- separate slot-1 handling from slot-2 and slot-3 handling
- allow slot 1 to resolve to:
  - `function.wrapper.pipeline.v1`
  - `function.wrapper.pipeline.chain3.v1` when same-tree local
- keep slot 2 and slot 3 unchanged
- make the cross-library recursive rejection explicit in the validator

### 4. Widen closure-member admission

- update `validate_typescript_closure_member_spec_with_specs(...)`
- admit chain3 closure members only through the same bounded slot-1 contract
- preserve helper, wrapper, monotone-up, and monotone-down rules unchanged

### 5. Recurse through nested chain3 in the backend

- update `collect_typescript_closure_member(...)`
- when the validated closure member is chain3:
  - recurse into its three direct deps
  - continue to use `collect_typescript_closure_member(...)` for each dep
  - rely on `included` to avoid duplicate emission
- keep unrelated loaded-unit exclusion tests green

### 6. Lock the proof wall

- extend validator tests in `spec-core/src/validator.rs`
- extend backend tree tests in `spec-core/src/typescript_backend.rs`
- extend Bun-backed and pre-Bun CLI proof in `spec-cli/tests/cli.rs`

### 7. Rewrite public wording

- `README.md`
  - replace the nested-chain3 unsupported sentence
  - name the exact new rule:
    - chain3 slot 1 may be wrapper or same-tree chain3
    - slot 2 and slot 3 stay fixed
    - generic multi-dep still unsupported
- `TODOS.md`
  - keep generic multi-dependency TypeScript execution deferred
  - do not record M58 as generic graph support

## Acceptance Commands

Run these before calling M58 done:

```bash
cargo test -p spec-core typescript_nested_chain3
cargo test -p spec-core typescript_tree_renders_nested_chain3
cargo test -p spec-cli --test cli typescript_nested_chain3
```

Then run the maintained fixture proof directly:

```bash
cargo run -p spec-cli -- test semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_nested_chain3_aligned.unit.spec --target-language typescript
```

Finally run the whole lane-facing regression surface:

```bash
cargo test -p spec-core
cargo test -p spec-cli --test cli
```

Acceptance is not complete until all three gates are green:

1. validator recursive admission and rejection proofs are green
2. generated TypeScript tree includes the nested chain3 closure and excludes unrelated units
3. Bun-backed nested-chain3 execution passes while all recursive red paths still fail before Bun

## NOT in scope

- cross-library nested chain3 recursion
  - rationale: that is a second widen and would blur the same-tree boundary immediately
- slot-2 or slot-3 recursive widening
  - rationale: that becomes a new family rule, not the M58 seam
- generic DAG execution policy
  - rationale: still an ocean, still explicitly deferred
- molecule TypeScript execution
  - rationale: unrelated product surface with its own proof model
- target-language validate/export
  - rationale: no need to widen CLI shape to ship this recursive slice

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| validator slot-1 widen | `spec-core/src/` | — |
| backend nested recursion | `spec-core/src/` | validator slot-1 widen |
| nested aligned fixture plus CLI proof | `semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/`, `spec-cli/tests/` | — |
| docs plus backlog sync | repo-root docs | validator slot-1 widen, backend nested recursion, CLI proof |

### Parallel lanes

- Lane A: validator slot-1 widen → backend nested recursion
  - sequential, shared `spec-core/src/`
- Lane B: nested aligned fixture plus CLI proof
  - parallelizable, but it must coordinate exact fixture ids and final error wording with Lane A
- Lane C: docs plus backlog sync
  - sequential after A and B merge, because public wording must match shipped behavior exactly

### Execution order

Launch Lane A and Lane B in parallel worktrees.

Merge both only after:

1. validator behavior is stable
2. nested fixture ids are locked
3. CLI proof expectations match final validator wording

Then run Lane C for docs and backlog wording.

### Conflict flags

- Lane A and Lane B do not share files, but they do share fixture ids and validator-error wording
- avoid editing `README.md` from either parallel lane, keep that in Lane C
- do not split validator and backend into separate worktrees, both live in `spec-core/src/`

## Completion Summary

- Step 0: Scope Challenge
  - scope accepted as a bounded slot-1 recursive widen only
- Architecture Review
  - one required contract clarification: nested chain3 is only reachable by widening slot 1
- Code Quality Review
  - explicit-over-clever implementation required, no generic graph registry
- Test Review
  - maintained nested proof path required
  - Bun-backed green path required
  - pre-Bun rejection wall must stay strong
- Performance Review
  - no new asymptotic risk if recursion stays on the existing included-set walker
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

## Why This Plan Wins

It reads like one execution contract now, not a mix of stale milestone cleanup notes and partial review commentary.

It names the exact M58 behavior widen, ties that behavior to concrete files and proof gates, preserves the bounded family-shaped story, and leaves no room to accidentally smuggle generic graph execution into the TypeScript lane. That is the right bar for this milestone.
