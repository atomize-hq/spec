<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260505-112225.md -->

# M35 - Architecture Shared-Core Follow-On

Status: **authoritative implementation plan**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-05**  
Supersedes: **M34 - Stop-Spend-Pivot Decision Contract**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260505-110352.md`**  
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**  
Program tracker: **`docs/recommendation_corpus_expansion_program_v0.1.md`**  
Capability guide: **`docs/semantic_family_capability_corpus_guide_v0.1.md`**  
Reality-alignment prerequisite: **land validated M34 commit `df15e3e392be30a13b10f028eb19e4286c931523` from `ws/m34-int` onto this branch before any M35-only edits**  
Execution note: **do not create a new crate, a new artifact family, or a generic decision engine. This milestone extracts one bounded shared truth inside the existing `xtask` family layer.**

## Objective

Turn the current helper-surface special case into one explicit shared-core
classification that both recommendation analysis and corpus-program decision
derivation consume.

After M35, the repo should still reach the same live wedge conclusion:

- do **not** spend corpus run `1`
- do **not** promote a new family
- do pivot to `author_architecture_follow_on_plan`

But it must reach that conclusion through one named reusable truth surface
instead of embedding the helper-surface rule separately in family-local policy
code.

## Decision

M35 ships as a **bounded shared-core extraction inside `xtask/src/family/`**.

The implementation has two phases:

1. **Reality alignment**
   Bring the validated M34 command and artifact contract from `ws/m34-int`
   onto `feat/corpus-expansion` unchanged in behavior.

2. **Shared-core extraction**
   Extract a single helper-surface classification module that answers:

   > Is this visible unsupported-function pressure a durable,
   > non-promotable helper surface?

   That classification becomes the shared truth consumed by:

   - recommendation derivation in `xtask/src/family/recommend.rs`
   - corpus-program decision derivation in the M34 command path
   - bounded invariant tests that prove both consumers stay aligned

This is the smallest complete move.

## Problem Statement

The repo already knows something important:

- the visible unsupported pressure is real
- the blocker is not missing evidence
- the current shape is a helper surface, not the next promotable family

Today that truth is not owned cleanly.

The helper-surface rule currently exists as inline recommendation logic in
`xtask/src/family/recommend.rs`, plus schema-level tuple invariants in
`xtask/src/family/promotion_artifacts.rs`, plus doc prose that explains what the
verdict means. That is enough to describe the result, but not enough to make the
reason reusable.

The branch split makes the problem worse. The validated M34 outcome exists on
`ws/m34-int` at `df15e3e392be30a13b10f028eb19e4286c931523`, but the working
branch does not yet contain:

- `cargo xtask family corpus-decision --format json`
- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

So M35 cannot treat M34 as ambient background truth. It must first make the
working branch real, then extract the bounded shared-core concept on top of that
real surface.

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Runtime-supported family inventory | `xtask/src/family/inventory.rs`, `xtask/src/family/harness.rs`, `xtask/src/family/routing.rs` | Reuse unchanged. M35 is not an inventory milestone. |
| Corpus coverage projection | `xtask/src/family/coverage.rs` and `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json` | Reuse unchanged. M35 must not rescan or reinterpret the corpus outside existing coverage flow. |
| Recommendation analysis | `xtask/src/family/recommend.rs` and `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json` | Reuse as the current consumer that still owns discovery inputs. Replace only the helper-surface special case with shared classification. |
| Recommendation/read-side schemas | `xtask/src/family/promotion_artifacts.rs` | Reuse for artifact structs and tuple validation. Do not move artifact ownership elsewhere. |
| Artifact paths and atomic writes | `xtask/src/family/paths.rs` and `write_bytes_atomically(...)` | Reuse directly. M35 should not invent a new artifact root. |
| M34 command surface | validated commit `df15e3e392be30a13b10f028eb19e4286c931523` on `ws/m34-int` | Reuse by landing it first, not by re-implementing M34 from scratch inside M35. |
| Prior shared projection pattern | `spec-core/src/portability.rs` | Reuse as a design pattern only: one projection module, multiple consumers, shared tests. Do **not** move helper-surface truth into `spec-core`. |
| Maintainer-facing docs | `semantic-families/README.md`, `docs/recommendation_corpus_expansion_program_v0.1.md`, `docs/semantic_family_capability_corpus_guide_v0.1.md` | Update wording only where M35 changes the explanation of where helper-surface truth lives. |

### Minimum change set

The minimum complete implementation is:

1. land exact M34 commit `df15e3e392be30a13b10f028eb19e4286c931523` or a
   byte-equivalent merge result onto `feat/corpus-expansion`
2. add one new shared module under `xtask/src/family/` for helper-surface
   classification
3. rewire `recommend.rs` to use that shared classifier instead of inline helper
   surface shape logic
4. rewire the landed M34 corpus-decision path to use the same classifier for the
   `durable_non_promotable_helper_surface` basis
5. add cross-consumer regression tests proving both consumers stay aligned on
   the live wedge and on non-helper counterexamples
6. update the three maintainer docs so the repo explains the extracted truth
   honestly

Anything beyond that is scope leak.

### Complexity check

This milestone should stay inside roughly these touched areas:

- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs` (new)
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs` only if the landed M34 merge requires it
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `PLAN.md`

That is a real diff, but still one bounded lane. The smell threshold is crossed
only because M35 must first import the already-validated M34 surface. That is
acceptable. Adding a second new module, a new crate, or new runtime storage is
not.

### Search check

**[Layer 1]** Reuse the existing `xtask` artifact pipeline, the existing M33
recommendation analysis, and the existing M34 command contract from
`ws/m34-int`.

**[Layer 1]** Reuse the repo's existing shared-projection pattern from
`spec-core/src/portability.rs`: one bounded module, explicit projection type,
multiple read-side consumers, cross-consumer tests.

**[EUREKA]** Do **not** extract this into `spec-core`.

That sounds "more shared" on paper, but it is the wrong layer. Helper-surface
non-promotability is not authored spec truth. It is family-analysis policy over
existing promotion artifacts. Pushing it into `spec-core` would spend an
innovation token to make the layering less honest.

### TODOS cross-reference

No open item in `TODOS.md` blocks M35 directly.

Relevant nearby backlog items already reinforce the same boundary:

- avoid widening milestone scope when a narrower truth surface exists
- prefer explicit projection modules over scattered recomposition
- keep generated analysis/read-side surfaces deterministic

M35 should add new TODOs only if it deliberately defers broader decision-engine
work or future helper-surface generalization.

### Completeness check

A prose-only explanation is not enough.

The complete bounded version is:

- M34 is actually present on the working branch
- helper-surface classification has one code owner
- recommendation derivation and corpus-decision derivation both call that owner
- shared regression tests prove they do not drift apart
- docs explain the new boundary accurately

That is the lake. Boil it.

### Distribution check

No new package, binary, container image, or release pipeline is required.

The deliverable is internal repo architecture truth layered onto the existing
`cargo xtask family ...` command family.

## Locked Decisions

### 1. M35 starts by landing the validated M34 result

Do not rebuild M34 by memory.

First bring commit `df15e3e392be30a13b10f028eb19e4286c931523` from
`ws/m34-int` onto `feat/corpus-expansion`, then build M35 on top of that exact
surface.

### 2. Shared-core extraction stays in `xtask/src/family/`

The extraction belongs beside the recommendation and decision consumers that use
it now.

Do not:

- move it to `spec-core`
- create a new crate
- invent a repo-wide "decision engine"

### 3. The extracted truth is one bounded classification

M35 extracts exactly one concept:

- `durable_non_promotable_helper_surface`

It does **not** extract:

- a generalized family recommendation framework
- all future corpus-spend logic
- all future promotion-policy routing

### 4. Recommendation owns discovery inputs, shared core owns classification

`recommend.rs` still owns candidate discovery inputs such as:

- overlap family
- leverage counts
- shape fingerprint
- primary reason code

The shared module owns only the helper-surface classification decision over that
input.

### 5. Corpus decision owns action mapping

The shared module should say:

- this candidate is, or is not, a durable non-promotable helper surface

The corpus-decision command should translate that classification into M34's
action vocabulary:

- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `decision_basis_code = "durable_non_promotable_helper_surface"`
- `required_next_action = "author_architecture_follow_on_plan"`

Do not bury workflow action strings inside the shared classifier itself.

### 6. Artifact validators validate tuple consistency, not independent semantics

`promotion_artifacts.rs` should keep validating bounded field consistency:

- durable hold tuple stays durable hold
- helper-surface decision tuple stays aligned

It should not become a second semantic classifier that re-derives the helper
shape from raw fields independently of the shared module.

### 7. No new public artifact family is required

M35 can prove the extraction is real by consumer rewiring and tests.

Do not add a new standalone `helper-surface.latest.json` artifact unless the
implementation proves there is no smaller honest move.

## Proposed Shared-Core Shape

Add one new module:

- `xtask/src/family/helper_surface.rs`

That module owns:

- a small normalized input struct for helper-surface classification
- one explicit classification enum
- one classifier function
- adapters or constructors needed by the existing consumers

Target shape:

```rust
pub(crate) enum HelperSurfaceDisposition {
    DurableNonPromotable,
}

pub(crate) struct HelperSurfaceSignal {
    pub primary_reason_code: UnsupportedFunctionReasonCode,
    pub overlap_family: String,
    pub real_example_hits: usize,
    pub shape_fingerprint: String,
}

pub(crate) fn classify_helper_surface(
    signal: &HelperSurfaceSignal,
) -> Option<HelperSurfaceDisposition>;
```

Design rules:

1. The classifier is explicit and wedge-specific.
2. The classifier returns `None` for all non-helper cases.
3. The classifier does not emit corpus actions or recommendation statuses.
4. The classifier is pure and testable with fixture-sized inputs.

This is engineered enough. One module. One enum. One signal type. No framework.

## Target Architecture

```text
semantic-families/corpus/rust-function.toml
        |
        v
cargo xtask family coverage --format json
        |
        v
coverage.latest.json
        |
        v
recommend.rs
  ├─ candidate discovery
  ├─ leverage + overlap computation
  ├─ helper_surface::classify_helper_surface(...)
  └─ recommendation.latest.json
                |
                v
      cargo xtask family corpus-decision --format json
                |
                ├─ load recommendation artifact
                ├─ map top candidate -> HelperSurfaceSignal
                ├─ helper_surface::classify_helper_surface(...)
                └─ corpus-program-decision.latest.json

promotion_artifacts.rs
  ├─ validates recommendation tuple consistency
  └─ validates corpus-decision tuple consistency

docs/
  └─ explain that helper-surface non-promotability now has one code owner
```

### Why this boundary is right

- `coverage.rs` still owns observation truth
- `recommend.rs` still owns recommendation assembly
- M34 command still owns operator action mapping
- the shared module owns only the helper-surface classification

That is the architectural seam M35 exists to create.

## Implementation Plan

### Phase 1 - Reality alignment

1. Bring `df15e3e392be30a13b10f028eb19e4286c931523` from `ws/m34-int` onto
   `feat/corpus-expansion`.
2. Verify the branch now contains:
   - `cargo xtask family corpus-decision --format json`
   - `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
   - the exact M34 action vocabulary and docs
3. Freeze behavior. Do not change M34 semantics during the import.

Exit gate:

- M34 command exists on this branch
- the live wedge still emits the validated M34 decision

### Phase 2 - Extract helper-surface shared core

1. Add `xtask/src/family/helper_surface.rs`.
2. Move the current helper-surface shape predicate out of `recommend.rs` and
   into the new module.
3. Keep the rule byte-for-byte equivalent in meaning:
   - `UnsupportedFunctionReasonCode::UnsupportedFunctionSurface`
   - `overlap_family == "unknown"`
   - `real_example_hits > 0`
   - the current helper/no-deps fingerprint shape
4. Export the module from `xtask/src/family/mod.rs`.

Exit gate:

- there is exactly one code owner for helper-surface classification
- `recommend.rs` no longer contains an inline helper-shape classifier

### Phase 3 - Rewire consumers

#### Recommendation consumer

Rewire `recommend.rs` so the durable-hold path is driven by the shared
classifier result, not by an in-file predicate.

The outward recommendation artifact should remain unchanged:

- durable helper surfaces still land in
  `next_step_status = durable_hold`
- `next_step_detail = helper_surface_not_promotable`
- `hold_reasons` still include `helper_surface_not_promotable`

#### Corpus-decision consumer

Rewire the landed M34 command path so the helper-surface pivot basis is produced
from the same classifier.

The outward corpus-program decision artifact should remain unchanged for the
live wedge:

- `decision_action = pivot_to_architecture_shared_core_follow_on`
- `decision_basis_code = durable_non_promotable_helper_surface`
- `required_next_action = author_architecture_follow_on_plan`

Exit gate:

- both consumers call the same shared helper-surface classifier
- neither consumer re-implements the helper-surface rule locally

### Phase 4 - Docs, verification, and closeout

1. Update `semantic-families/README.md` to explain that helper-surface pressure
   is real, but non-promotability is now produced from one shared analysis
   boundary.
2. Update `docs/recommendation_corpus_expansion_program_v0.1.md` to distinguish:
   - recommendation analysis as input truth
   - shared helper-surface classification as the architectural hinge
   - corpus-decision as operator-action output
3. Update `docs/semantic_family_capability_corpus_guide_v0.1.md` so maintainers
   understand why more corpus does not change this wedge by itself.
4. Update `PLAN.md` completion notes only after all verification commands pass.

## Code Quality Rules

### DRY rule

There must be one helper-surface classifier.

Reject any implementation that leaves:

- one version in `recommend.rs`
- a second version in the corpus-decision path
- a third version in validators

That is exactly the architecture debt M35 is supposed to remove.

### Explicit over clever

Do not introduce:

- trait-based classifier registries
- macro-generated decision tables
- generic policy pipelines

Use named structs and `match` blocks. A tired maintainer should read the module
in 30 seconds.

### Minimal diff

Prefer:

- one new module
- local rewiring in existing files
- tests beside the current `xtask` test surface

Do not turn M35 into a test-harness rewrite or a command-layout refactor.

## Test Review

### Test framework

This repo is Rust-first. M35 verification stays in the existing Cargo/xtask test
surface:

- `cargo test -p xtask ...`
- targeted artifact-generation commands under `cargo xtask family ...`

No new test framework is needed.

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/helper_surface.rs
    |
    ├── [REQ TEST] classify helper surface wedge -> DurableNonPromotable
    ├── [REQ TEST] non-helper fingerprint -> None
    ├── [REQ TEST] overlap family known -> None
    └── [REQ TEST] zero real-example hits -> None

[+] xtask/src/family/recommend.rs
    |
    ├── [REQ TEST] shared classifier hit -> durable_hold tuple
    ├── [REQ TEST] targeted evidence gap path still emits targeted_evidence_gap
    └── [REQ TEST] ready promotion path remains unchanged

[+] cargo xtask family corpus-decision --format json
    |
    ├── [REQ TEST] durable helper-surface wedge -> architecture pivot decision
    ├── [REQ TEST] evidence-gap wedge -> spend_corpus_run_1
    ├── [REQ TEST] promotion-ready wedge -> family-promotion action
    ├── [REQ TEST] contradictory recommendation tuple -> hard error
    └── [REQ TEST] missing recommendation artifact -> hard error

[+] promotion_artifacts.rs validators
    |
    ├── [REQ TEST] durable_hold recommendation tuple validates
    ├── [REQ TEST] helper-surface tuple with wrong next_step_status rejects
    └── [REQ TEST] corpus-decision durable_non_promotable basis tuple validates
```

### User flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Maintainer reads the live wedge
    |
    ├── [REQ TEST] coverage -> recommend -> corpus-decision stays deterministic
    └── [REQ TEST] decision artifact bytes stay stable on repeat run

[+] Maintainer asks "should we spend corpus run 1?"
    |
    ├── [REQ TEST] answer is pivot_to_architecture_shared_core_follow_on
    └── [REQ TEST] docs match the emitted decision vocabulary

[+] Maintainer encounters malformed or contradictory state
    |
    ├── [REQ TEST] corpus-decision refuses silent fallback
    └── [REQ TEST] error points at recommendation tuple inconsistency
```

### Regression rule

M35 is partly a regression-protection milestone.

The highest-priority regression to guard is:

- recommendation and corpus-decision both used to agree on the live helper
  wedge after M34
- after extraction, they must still agree

If an extraction changes the emitted live wedge outcome, that is a blocking
regression and requires a regression test before merge.

### Required tests to add

1. Shared-classifier unit tests in the new helper module covering:
   - exact live helper wedge
   - each non-helper counterexample
2. Recommendation regression tests proving the same fixture still yields:
   - durable hold
   - `helper_surface_not_promotable`
3. Corpus-decision regression tests proving the same fixture still yields:
   - architecture pivot
   - `durable_non_promotable_helper_surface`
4. Cross-consumer consistency test:
   - one shared fixture
   - recommendation emits durable hold
   - corpus-decision emits architecture pivot
5. Contradictory-state tests:
   - tuple says durable hold but helper classifier returns `None`
   - command must error, not guess

## Failure Modes Registry

| Codepath | Realistic failure | Test coverage required | Error handling required | User-visible outcome |
|---|---|---|---|---|
| M34 merge skipped | M35 compiles against a branch that does not have `family corpus-decision` at all | yes | yes, fail at opening gate | explicit blocker, not silent fallback |
| Shared classifier drift | recommendation says durable hold but corpus-decision chooses a different basis | yes | yes | explicit failing test before merge |
| Fingerprint parse failure | helper-shaped candidate stops classifying because serialized fingerprint shape changed | yes | yes | explicit non-classification plus failing regression if live wedge changes |
| Validator mismatch | emitted tuple no longer matches schema invariants | yes | yes | hard validation error |
| Doc drift | docs tell maintainers to spend corpus run `1` while command says pivot | yes | yes, grep-based doc assertions | visible inconsistency blocked before merge |

Critical gap rule:

If any helper-surface failure mode has no test **and** no hard error path,
M35 is incomplete.

## Performance Review

M35 should stay read-side and cheap.

Rules:

1. `family corpus-decision` must continue reading the existing recommendation
   artifact. It must not rerun coverage or recommendation analysis internally.
2. The shared classifier must be pure and in-memory. No filesystem access.
3. Shape-fingerprint parsing remains tiny. This is not a throughput bottleneck.
4. Deterministic artifact reuse remains intact. Repeated runs on unchanged input
   should preserve byte-identical outputs where current infrastructure already
   guarantees that.

There is no legitimate reason for M35 to change runtime complexity class.

## What Already Exists

The plan intentionally reuses the repo's existing truth surfaces:

- `xtask/src/family/recommend.rs` already identifies the helper-surface wedge
- `xtask/src/family/promotion_artifacts.rs` already encodes the durable-hold
  tuple vocabulary
- `ws/m34-int` already contains the validated corpus-decision command contract
- `spec-core/src/portability.rs` already demonstrates the repo's preferred
  shared-projection pattern

M35 succeeds by making these surfaces line up, not by replacing them.

## NOT in scope

- spending corpus run `1`
- promoting a new family
- redesigning the full recommendation engine
- moving family-analysis policy into `spec-core`
- creating a new shared crate
- introducing a new artifact family just to prove sharing exists
- changing inventory, coverage, prove, certify, or routing behavior outside the
  bounded helper-surface wedge
- broad doc rewrites unrelated to helper-surface non-promotability

## Verification Commands

Run these in merged M35 state:

```bash
cargo test -p xtask helper_surface -- --color never
cargo test -p xtask corpus_decision -- --color never
cargo test -p xtask recommend -- --color never
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
```

Then assert the live wedge still says:

```bash
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

And confirm docs stay aligned:

```bash
rg -n 'helper_surface_not_promotable|durable_non_promotable_helper_surface|author_architecture_follow_on_plan|corpus run `1`' \
  semantic-families/README.md \
  docs/recommendation_corpus_expansion_program_v0.1.md \
  docs/semantic_family_capability_corpus_guide_v0.1.md
```

## Worktree Parallelization Strategy

This plan has one real dependency wall and then two bounded parallel lanes.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Reality alignment | `xtask/src/`, `semantic-families/`, `docs/` | — |
| Shared classifier extraction | `xtask/src/family/` | Reality alignment |
| Corpus-decision consumer rewiring + tests | `xtask/src/`, `xtask/src/family/` | Reality alignment, shared classifier extraction |
| Docs alignment | `semantic-families/`, `docs/` | Reality alignment |

### Parallel lanes

Lane A: reality alignment → shared classifier extraction → corpus-decision rewiring  
Lane B: docs alignment (starts after reality alignment, independent of code until final wording freeze)

Sequential note:

- Lane A is sequential because `xtask/src/lib.rs` and `xtask/src/family/` are
  the primary codepath and test surface.
- Lane B can run in parallel once M34 vocabulary is present on branch and the
  shared classifier name is frozen.

### Execution order

1. Launch **Reality alignment** first. No parallelism yet.
2. After M34 is landed and the shared classifier name is frozen, launch:
   - **Lane A** shared classifier extraction
   - **Lane B** docs alignment
3. Merge Lane B into the parent only after Lane A's field names and vocabulary
   are final.
4. Run final corpus-decision rewiring tests and verification sequentially on the
   parent branch.

### Conflict flags

- `xtask/src/lib.rs` is a conflict magnet. Keep all command-dispatch and major
  xtask regression tests in one lane.
- `xtask/src/family/recommend.rs` and the new helper module must stay in the
  same lane. Splitting them would create merge churn for no benefit.
- Docs can run in parallel, but only after the names
  `durable_non_promotable_helper_surface` and
  `author_architecture_follow_on_plan` are frozen.

## Completion Summary

- Step 0: Scope Challenge — scope accepted as bounded shared-core extraction
- Architecture Review: one new module, no new crate, M34 land-first dependency
- Code Quality Review: one DRY mandate, one code owner for helper-surface truth
- Test Review: coverage diagram produced, shared-consumer regression suite
  required
- Performance Review: no new complexity class, no corpus rescan allowed
- NOT in scope: written
- What already exists: written
- Failure modes: explicit critical-gap rule defined
- Parallelization: 2 lanes after 1 sequential opening gate
- Lake Score: choose the complete version, land M34 + extract shared truth +
  prove cross-consumer alignment

## Done when

1. `feat/corpus-expansion` contains the validated M34 command and artifact
   surface.
2. The repo has exactly one helper-surface classifier under
   `xtask/src/family/`.
3. Recommendation derivation and corpus-decision derivation both use that
   classifier.
4. The live wedge still deterministically yields the M34 architecture-pivot
   outcome.
5. No new artifact family, crate, or generalized decision framework was added.
6. Tests and docs prove the extraction is real and non-regressive.
