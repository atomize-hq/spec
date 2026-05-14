# M56: Bounded Direct Cross-Library Wrapper and Chain3 TypeScript Roots Plan

Status: **implementation plan**  
Milestone: **M56**  
Milestone family: **bounded-typescript-execution**  
Implementation readiness: **ready for bounded execution**  
Plan scope: **extend the existing Bun-backed TypeScript lane to allow direct cross-library wrapper and chain3 root deps for the already-supported families, without widening beyond that contract**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Validated at commit: **`b8c5bbf`**  
Last rewritten: **2026-05-13**

Supersedes:

- the prior M55 plan at this path
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260513-150038.md`
- the prior M55 design doc and test-plan artifacts captured in `~/.gstack/projects/atomize-hq-spec/`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260513-150038.md`
- `ORCH_PLAN.md`
- `TODOS.md`
- `README.md`
- `CHANGELOG.md`

Primary repo surfaces:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`
- `examples/crosslib-app/spec.toml`
- `examples/crosslib-app/units/`
- `examples/shared-spec/units/`
- `examples/crosslib-app/README.md`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

## Executive Summary

M55 fixed the fake helper-import wall. The Bun-backed TypeScript lane now truthfully supports:

- monotone-up roots
- same-tree wrapper roots
- same-tree chain3 roots
- cross-library helper imports in the one legal helper slot

The next explicit product gap is narrower than generic portability and more useful than another planning loop:

> allow direct cross-library root deps for the already-supported `function.wrapper.pipeline.v1` and `function.wrapper.pipeline.chain3.v1` families, while preserving the exact family tuples, Bun-only runtime, atom-only proof model, and every broader TypeScript ban

That is the whole milestone.

This plan does not authorize generic cross-library TypeScript execution. It does not authorize arbitrary dependency graphs. It does not authorize molecule TypeScript, seam kinds, nested chain3 closure execution, `spec validate --target-language`, or `spec export --target-language`.

## Problem Statement

Today the product story is still awkward at the root-dependency seam.

The current TypeScript lane can execute closure members that reuse a cross-library helper after load, but it still rejects a wrapper root or chain3 root when one of that root's direct dep slots is `shared::...`. That means the repo can honestly say "cross-library helper reuse works" while still blocking the next obvious real-world composition shape.

For a user, the gap looks arbitrary:

1. author cross-library leaves in a sibling spec library
2. compose them into a real wrapper or chain3 root in the app library
3. run Rust and succeed
4. run `spec test --target-language typescript`
5. fail before Bun because the direct dep lives in a sibling library

That is a fake wall. M56 removes it without pretending the whole execution model is now generic.

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing surface | M56 action |
| --- | --- | --- |
| Sibling-library loading and `[libraries]` config | `examples/crosslib-app/spec.toml`, shipped M9 cross-library support | Reuse |
| Cross-library helper-import validation | `spec-core/src/validator.rs` M55 TypeScript target rules | Extend carefully |
| Bounded TypeScript closure generation | `spec-core/src/typescript_backend.rs` | Extend direct root-dep resolution only |
| Wrapper family contract | `function.wrapper.pipeline.v1`, validator constants, CLI tests | Reuse, do not widen |
| Chain3 family contract | `function.wrapper.pipeline.chain3.v1`, validator constants, CLI tests | Reuse, do not widen |
| Real sibling-library example | `examples/crosslib-app/`, `examples/shared-spec/` | Extend with one maintained wrapper proof |
| Existing same-tree chain3 fixture machinery | `spec-cli/tests/cli.rs`, chain3 fixtures in family packets | Reuse for focused chain3 cross-library proof |

### Minimum change set

The minimum honest implementation is:

1. admit `shared::...` in direct wrapper dep slots when the dep order, family tuple, and `body.typescript` rules still match exactly
2. admit `shared::...` in direct chain3 dep slots under the same frozen tuple rules
3. resolve those direct deps through the already-loaded sibling-library set, not a second TypeScript-only resolver
4. render correct library-aware TypeScript imports for direct cross-library root deps and their bounded closures
5. prove one maintained real wrapper root in `examples/crosslib-app`
6. prove one focused chain3 cross-library root through CLI/integration coverage
7. preserve all existing same-tree and helper-import green paths
8. update docs only after the proof wall is green

Anything broader is scope creep.

### Complexity check

This work touches more than 8 files once examples, tests, and docs are counted. That is acceptable only because the implementation still stays inside two existing code seams:

- validation and admission in `spec-core/src/validator.rs`
- closure collection and import rendering in `spec-core/src/typescript_backend.rs`

No new crates, services, commands, schema surfaces, or runtime channels are allowed.

### Search check

No new framework, concurrency model, or infrastructure pattern is being introduced. This is not a "search for a new solution" milestone. This is a "do not accidentally widen the existing solution" milestone.

Recommendation class: **[Layer 1]** reuse the existing validator and loaded-unit truth. If the implementation starts inventing a second resolver path just for TypeScript, that is a regression, not innovation.

### TODOS cross-reference

This plan executes the current deferred item in `TODOS.md`:

- `Direct cross-library wrapper and chain3 TypeScript roots`

This plan must continue to defer:

- `Generic multi-dependency TypeScript execution`
- molecule TypeScript execution
- seam-kind TypeScript execution
- nested chain3 closure members

### Completeness check

The complete version is still bounded. The shortcut would be landing validator support and a small synthetic test while leaving the maintained example and docs half-true.

The lake for M56 is:

- one maintained real wrapper proof
- one real chain3 proof surface
- the full negative wall
- truthful docs and backlog language

Boil that lake. Do not stop at the happy path.

### Distribution check

No new artifact type is introduced. Distribution remains the existing `spec` CLI via current cargo install and GitHub release paths.

## Current State

Observed on `feat/m40-plus` at `b8c5bbf`:

- `README.md` still says direct cross-library wrapper roots and direct cross-library chain3 roots remain unsupported in the bounded TypeScript lane.
- `TODOS.md` explicitly defers direct cross-library wrapper and chain3 TypeScript roots after M55.
- `examples/crosslib-app` currently proves cross-library helper imports only through `pricing/apply_tax.unit.spec`.
- `spec-core/src/validator.rs` still hard-rejects direct cross-library wrapper deps and direct cross-library chain3 deps with M55-specific messages.
- `spec-core/src/typescript_backend.rs` still parses wrapper and chain3 direct deps as local-only in both root-closure collection and module import rendering.
- `spec-cli/tests/cli.rs` already has the test harness structure for cross-library helper imports, same-tree wrapper execution, and same-tree chain3 execution.

Governance truth remains unchanged:

- the semantic-review/family-analysis stop state stays frozen
- this milestone is backend product truth only
- no new shared-core architecture story is opened here

## Exact Product Contract

### In scope

- direct cross-library direct deps for `function.wrapper.pipeline.v1`
- direct cross-library direct deps for `function.wrapper.pipeline.chain3.v1`
- mixed local-plus-cross-library dep tuples for those same roots, as long as tuple order and family classification stay exact
- reuse of already-shipped M55 helper-import behavior inside any loaded direct-dep closure
- correct TypeScript import rendering for sibling-library direct deps and their bounded loaded closures
- bounded validation and generator errors for:
  - unresolved library alias
  - missing imported unit
  - wrong dep family
  - wrong dep order
  - wrong dep count
  - missing imported `body.typescript`
- one maintained green wrapper root in `examples/crosslib-app`
- one chain3 cross-library proof in focused CLI/integration coverage
- preservation of additive, target-specific Rust and TypeScript proof

### Not in scope

- generic cross-library TypeScript execution
- generic multi-dependency TypeScript execution
- new supported function families
- molecule TypeScript execution
- seam-kind TypeScript execution
- nested chain3 closure-member support
- `spec validate --target-language`
- `spec export --target-language`
- new runtimes or package-manager support beyond Bun
- schema changes in status, export, or passports

## Locked Decisions

These are contract decisions, not suggestions:

1. M56 widens only the library location of already-legal direct root deps.
2. Wrapper roots still require exactly two direct deps in the fixed order:
   - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
   - `function.arithmetic_leaf.monotone_up.v1`
3. Chain3 roots still require exactly three direct deps in the fixed order:
   - `function.wrapper.pipeline.v1`
   - `function.arithmetic_leaf.monotone_up.v1`
   - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
4. Cross-library root-dep resolution must reuse the loaded library set and current dep parsing model. No second TypeScript-only resolver.
5. Closure collection stays bounded to the resolved root deps and the already-supported closure-member rules.
6. Nested `function.wrapper.pipeline.chain3.v1` closure members remain unsupported.
7. The maintained public example must prove a direct cross-library wrapper root.
8. The chain3 proof will live in focused CLI/integration coverage, not the public example, unless implementation proves that adding it to `examples/crosslib-app` costs only one extra authored unit and does not degrade README legibility. Default plan: keep it out of the public example.
9. Docs must keep saying "bounded direct cross-library wrapper and chain3 roots" and must keep every broader ban explicit.

## Abort And Re-scope Triggers

Stop implementation and rewrite the plan if any of these become true:

1. cross-library root-dep support requires a generic graph executor instead of the current bounded closure collector
2. import rendering requires a second resolver stack separate from the loaded-unit truth
3. the chain3 proof requires nested chain3 closure support
4. passport, export, or status schemas need new fields
5. the only way to prove the public wrapper example is through test-only mutation or temporary `body.typescript` injection
6. the only truthful docs wording becomes "cross-library TypeScript support" instead of the exact bounded claim

## What Already Exists

| Sub-problem | Existing code or flow | Reuse or change |
| --- | --- | --- |
| Root-family admission for TypeScript | `spec-core/src/validator.rs` constants and `validate_typescript_*` helpers | Change in place |
| Direct-dep closure collection | `spec-core/src/typescript_backend.rs` root and closure walkers | Change in place |
| Local-only dep parsing | `parse_local_typescript_dep(...)` call sites in backend | Replace only where direct root deps widen |
| Cross-library helper example | `examples/crosslib-app/units/pricing/apply_tax.unit.spec` | Preserve |
| Shared helper unit with `body.typescript` | `examples/shared-spec/units/money/round.unit.spec` | Preserve |
| CLI proof pattern for TypeScript lane | `spec-cli/tests/cli.rs` `--target-language typescript` coverage | Extend |
| Same-tree chain3 proof harness | `copy_m21_chain3_fixture(...)`, related tests in `spec-cli/tests/cli.rs` | Reuse for focused chain3 cross-library proof |

## Architecture Review

### Current vs target admission flow

```text
CURRENT M55
  wrapper root
    -> exactly two direct deps
    -> both direct deps must be local
    -> closure may reuse shared helper after load

  chain3 root
    -> exactly three direct deps
    -> all direct deps must be local
    -> closure may reuse shared helper after load

TARGET M56
  wrapper root
    -> exactly two direct deps
    -> each direct dep may be local or sibling-library
    -> tuple order and family classification stay frozen
    -> loaded closure follows existing bounded rules

  chain3 root
    -> exactly three direct deps
    -> each direct dep may be local or sibling-library
    -> tuple order and family classification stay frozen
    -> loaded closure follows existing bounded rules
```

### Data flow

```text
spec test <unit> --target-language typescript
  |
  +-- validator.rs
  |     +-- classify root family
  |     +-- validate exact dep arity
  |     +-- parse each direct dep as local or qualified sibling dep
  |     +-- resolve dep from loaded unit set
  |     +-- validate dep family, order, and body.typescript
  |
  +-- typescript_backend.rs
  |     +-- collect bounded closure from resolved root deps
  |     +-- keep nested chain3 closure-member ban
  |     +-- emit library-aware relative imports
  |
  +-- generated __spec_ts tree
  |
  `-- bun build/test
```

### Dependency graph

```text
examples/crosslib-app root unit
  -> local dep OR shared::dep
       -> resolved LoadedSpec from local library set
       -> semantic review classification
       -> body.typescript presence gate
       -> bounded closure walk
       -> generated import path
```

### Architectural opinion

The safest design is boring and explicit:

- widen dep parsing at the validator and backend seam
- keep family logic tuple-specific
- keep closure recursion exactly as bounded today
- keep one loaded-unit truth source

Do not abstract this into a generic cross-library execution framework. That would spend an innovation token on the wrong milestone.

## Implementation Plan

### Phase 1: Freeze the validator contract

Files:

- `spec-core/src/validator.rs`

Changes:

1. Replace the M55 wrapper and chain3 "local-only" direct-dep validation with library-aware direct-dep validation.
2. Preserve exact arity, order, family, and `body.typescript` enforcement.
3. Keep the error wall narrow and explicit:
   - alias missing
   - imported unit missing
   - wrong family in slot N
   - wrong dep count
   - missing `body.typescript`
4. Keep molecule rejection, seam-kind rejection, and nested chain3 bans unchanged.

Acceptance:

- all new direct cross-library wrapper and chain3 positives validate
- all wrong-family, wrong-order, wrong-count, unresolved-alias, and missing-body negatives reject before Bun
- same-tree wrapper and chain3 positives still validate

### Phase 2: Extend bounded TypeScript closure collection and import rendering

Files:

- `spec-core/src/typescript_backend.rs`

Changes:

1. Replace local-only parsing for wrapper and chain3 root deps with library-aware direct-dep resolution.
2. Keep closure inclusion bounded to:
   - the root
   - its resolved direct deps
   - the already-supported closure members below those deps
3. Render stable relative import paths for sibling-library units without emitting unrelated loaded units.
4. Keep helper-import behavior exactly as shipped in M55.

Acceptance:

- generated tree contains every required local and sibling-library unit exactly once
- generated tree excludes unrelated loaded units
- nested chain3 closure members still reject

### Phase 3: Add proof surfaces

Files:

- `examples/shared-spec/units/...`
- `examples/crosslib-app/units/...`
- `spec-cli/tests/cli.rs`

Changes:

1. Add the minimum authored shared leaves needed for a real direct cross-library wrapper root.
2. Add one maintained app-library wrapper root in `examples/crosslib-app`.
3. Keep the public example README legible.
4. Add focused chain3 cross-library proof coverage in `spec-cli/tests/cli.rs` using a controlled fixture path instead of inflating the public example by default.
5. Refresh or add negative fixtures for:
   - wrong dep order
   - wrong dep family
   - missing imported `body.typescript`
   - unresolved alias
   - missing imported unit

Acceptance:

- direct cross-library wrapper root passes in the maintained example
- direct cross-library chain3 root passes in focused CLI coverage
- M55 helper-import example still passes
- same-tree wrapper and chain3 proofs still pass

### Phase 4: Docs and backlog truth

Files:

- `README.md`
- `examples/crosslib-app/README.md`
- `CHANGELOG.md`
- `TODOS.md`

Changes:

1. Update the bounded TypeScript lane section to include direct cross-library wrapper and chain3 roots.
2. Keep generic multi-dep execution, molecule TypeScript, seam kinds, nested chain3 closure support, and broader cross-library claims explicitly deferred.
3. Remove or rewrite the M55-era TODO entry now that M56 lands.

Acceptance:

- README, CHANGELOG, example docs, and TODO inventory all tell the same product story

## Code Quality Review

### Guardrails

- Extend existing validator helpers instead of adding a parallel TypeScript-only validation subsystem.
- Prefer one new qualified-dep parsing path reused by both validator and backend over duplicated ad hoc branching.
- Keep milestone-specific constants honest. If a message still says "local-only" after M56, that is a correctness bug.
- Do not add new abstraction layers unless the same helper is used in both `validator.rs` and `typescript_backend.rs`.
- Update nearby comments and ASCII diagrams when contract language changes from M55 to M56.

### DRY targets

- direct-dep slot validation for wrapper and chain3 should share the same qualified resolution pattern, with family/slot expectations supplied as data
- import path rendering for sibling-library units should reuse the same unit-path normalization logic for wrapper and chain3 roots

### Technical-debt traps to avoid

- special-casing wrapper and chain3 in three different places with slightly different dep-resolution rules
- silently reusing helper-import wording for root-dep failures
- adding tests that prove only the positive path while leaving the old negative wall stale

## Test Review

### Test framework detection

The repo is Rust-first:

- runtime: `Cargo.toml`
- primary suites: `cargo test`, `spec-cli/tests/cli.rs`, inline unit tests in `spec-core`
- TypeScript proof is executed through CLI integration tests that shell into the Bun-backed lane

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/validator.rs
    |
    ├── wrapper root direct dep admission
    │   ├── [GAP] direct cross-library positive
    │   ├── [GAP] mixed local + shared tuple
    │   ├── [GAP] wrong family in shared slot
    │   ├── [GAP] wrong order across local/shared tuple
    │   ├── [GAP] missing imported body.typescript
    │   └── [GAP] unresolved alias / missing imported unit
    |
    └── chain3 root direct dep admission
        ├── [GAP] direct cross-library positive
        ├── [GAP] mixed local + shared tuple
        ├── [GAP] wrong family in slot 1/2/3
        ├── [GAP] wrong order
        ├── [GAP] wrong dep count
        └── [GAP] missing imported body.typescript

[+] spec-core/src/typescript_backend.rs
    |
    ├── wrapper root closure collection
    │   ├── [GAP] includes cross-library dep exactly once
    │   └── [GAP] excludes unrelated loaded units
    |
    └── chain3 root closure collection
        ├── [GAP] includes shared direct deps + bounded closures
        └── [GAP] preserves nested chain3 rejection

[+] spec-cli/tests/cli.rs
    |
    ├── [GAP] [→E2E] maintained wrapper example passes in TS lane
    ├── [GAP] [→E2E] focused chain3 cross-library root passes in TS lane
    ├── [GAP] same-tree wrapper still passes
    ├── [GAP] same-tree chain3 still passes
    ├── [GAP] M55 helper-import example still passes
    ├── [GAP] wrong-order rejection happens before Bun
    ├── [GAP] wrong-family rejection happens before Bun
    ├── [GAP] missing-body rejection happens before Bun
    ├── [GAP] unresolved-alias rejection happens before Bun
    └── [GAP] missing-imported-unit rejection happens before Bun

─────────────────────────────────
COVERAGE TARGET: 100% of new root-dep branches
QUALITY TARGET: validator unit tests + backend unit tests + CLI proof wall
CRITICAL GAPS: all direct cross-library wrapper/chain3 paths are currently unproven
─────────────────────────────────
```

### Required tests to add

#### `spec-core/src/validator.rs`

- positive wrapper admission with one or both direct deps in `shared::...`
- positive chain3 admission with one or more direct deps in `shared::...`
- mixed local/shared tuple ordering
- wrong family in each slot
- wrong order across local/shared tuples
- unresolved alias
- missing imported unit
- missing imported `body.typescript`

#### `spec-core/src/typescript_backend.rs`

- wrapper tree renders sibling-library direct imports correctly
- chain3 tree renders sibling-library direct imports correctly
- included tree excludes unrelated loaded units when cross-library deps are present
- nested chain3 closure-member rejection still fires

#### `spec-cli/tests/cli.rs`

- maintained cross-library wrapper example succeeds with `spec test ... --target-language typescript`
- focused chain3 cross-library fixture succeeds with `spec test ... --target-language typescript`
- wrong-order rejection happens before Bun
- wrong-family rejection happens before Bun
- unresolved alias rejection happens before Bun
- missing imported unit rejection happens before Bun
- missing imported `body.typescript` rejection happens before Bun
- regression coverage for same-tree wrapper, same-tree chain3, and M55 helper-import paths

### Test command wall

Run these exact commands before docs land:

```bash
cargo test -p spec-core validator::tests -- --nocapture
cargo test -p spec-core typescript_backend::tests -- --nocapture
cargo test -p spec-cli --test cli typescript
cargo run -p spec-cli -- test examples/crosslib-app/units/<new-wrapper-root>.unit.spec --target-language typescript
```

If the chain3 proof lives in a focused fixture, add the exact fixture command to the final PR notes and `ORCH_PLAN.md`.

## Performance Review

This milestone is not performance-driven, but there are still two real failure risks:

1. cross-library dep resolution could accidentally rescan or duplicate loaded units during closure collection
2. import rendering could emit redundant units and grow generated trees beyond the bounded closure

Performance acceptance:

- no O(N²) "scan the full loaded set for each dep" loops in new hot paths when a map lookup already exists
- no duplicate generated modules for the same resolved unit id
- no unrelated loaded units included once one shared dep appears

## Failure Modes Registry

| Failure mode | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- |
| Direct cross-library dep widens beyond exact family tuples | Yes | Yes, bounded validator rejection | Clear pre-Bun error |
| Alias resolves in validator but import path rendering fails later | Yes | Yes | Clear generator failure, never silent |
| Backend includes unrelated loaded units after one shared dep | Yes | No silent acceptance | Generated tree diff catches it |
| Wrong-family dep in shared slot slips through | Yes | Yes | Clear pre-Bun error |
| Missing imported `body.typescript` slips past validator | Yes | Yes | Clear pre-Bun error |
| Nested chain3 closure becomes accidentally allowed | Yes | Yes | Clear pre-Bun error |

Critical gap rule:

Any path with no validator test, no backend tree test, and no CLI proof is a release blocker for M56.

## Worktree Parallelization Strategy

This plan has parallelization value, but only after the validator contract is frozen. `validator.rs` is the blast-radius seam and must go first.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Validator contract freeze | `spec-core/src/` | — |
| Backend closure and import rendering | `spec-core/src/` | Validator contract freeze |
| Wrapper example and CLI proof wall | `examples/shared-spec/`, `examples/crosslib-app/`, `spec-cli/tests/` | Validator contract freeze |
| Docs and backlog truth | repo docs root, `examples/crosslib-app/README.md` | Backend + proof wall green |

### Parallel lanes

- Lane A: validator contract freeze, then backend closure/import work
- Lane B: after validator freeze, example authoring + CLI proof wall
- Lane C: docs and backlog updates after A and B are merged

Formatted:

- `Lane A: validator.rs freeze -> typescript_backend.rs changes` (sequential, shared `spec-core/src/`)
- `Lane B: examples/shared-spec + examples/crosslib-app + spec-cli/tests/cli.rs` (parallel after validator freeze)
- `Lane C: README.md -> examples/crosslib-app/README.md -> CHANGELOG.md -> TODOS.md` (sequential, docs-last)

### Execution order

1. Do validator freeze in the primary branch.
2. Launch Lane A and Lane B in parallel worktrees from that frozen head.
3. Merge Lane A and Lane B.
4. Run the full proof wall.
5. Launch Lane C only after proof is green.

### Conflict flags

- `validator.rs` and `typescript_backend.rs` both live in `spec-core/src/`, so validator freeze and backend work must not run in parallel before the freeze lands.
- `spec-cli/tests/cli.rs` is a single high-conflict file. Keep one owner.
- The public example and docs should not be split across multiple lanes. Example truth and docs wording drift together fast.

## NOT in Scope

- Generic cross-library TypeScript execution because it would lie about the bounded lane.
- Generic multi-dependency execution because the current product contract is family-shaped, not graph-shaped.
- Molecule TypeScript because this milestone is still atom-only.
- Seam kinds because the TypeScript lane remains `kind:function` only.
- Nested chain3 closure support because it is a separate widening with different risk.
- Schema work because the current proof surfaces are sufficient.

## Acceptance Checklist

The milestone is done only when all of these are true:

- direct cross-library wrapper roots pass in the bounded TypeScript lane
- direct cross-library chain3 roots pass in the bounded TypeScript lane
- exact wrapper and chain3 dep tuples stay enforced even when some slots are `shared::...`
- wrong dep order, wrong dep count, wrong family, unresolved alias, missing imported unit, and missing imported `body.typescript` all fail before Bun
- same-tree wrapper roots still pass
- same-tree chain3 roots still pass
- M55 helper-import behavior still passes
- generated tree remains bounded and excludes unrelated loaded units
- README, CHANGELOG, TODOs, and example docs all tell the same M56 story

## Completion Summary

- Step 0: Scope Challenge, completed. Scope accepted as the bounded M56 extension, not reduced beyond the design doc.
- Architecture Review: one core architectural rule, reuse existing loaded-unit truth and do not build a second resolver.
- Code Quality Review: one core quality rule, keep tuple enforcement and import resolution explicit, not clever.
- Test Review: full branch diagram included; all new root-dep branches require validator + backend + CLI proof coverage.
- Performance Review: bounded tree generation and duplicate-unit prevention are the only meaningful performance risks.
- NOT in scope: written.
- What already exists: written.
- Failure modes: written, with release-blocking critical-gap rule.
- Parallelization: 3 lanes total, 1 sequential gate then 2 parallelizable workstreams, then docs-last.
- Lake Score: 5/5 recommendations choose the complete bounded option over the shortcut.
