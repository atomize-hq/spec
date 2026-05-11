<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260511-110901.md -->
# M49: Reusable Seam Semantic-Review Substrate, Slice 1 Implementation Plan

Status: **implementation plan**  
Milestone: **M49**  
Milestone family: **semantic-review-core**  
Implementation readiness: **ready for bounded execution**  
Plan scope: **generalize supported seam routing away from literal unit ids, keep seam vocabulary explicit**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Execution precondition: **clean worktree**  
Last rewritten: **2026-05-11**

Supersedes:
- the prior repo-root plan captured at [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260511-110901.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260511-110901.md)

Primary source artifacts:
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-105634.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-105634.md)
- [ORCH_PLAN.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md)
- [docs/recommendation_corpus_expansion_program_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md)
- [docs/semantic_family_capability_corpus_guide_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md)

Primary repo surfaces:
- [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs)
- [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs)
- [spec-core/src/typescript_backend.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/typescript_backend.rs)
- [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs)
- [spec-cli/tests/cli.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/cli.rs)

Companion test artifact:
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-110938.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-110938.md)

## Executive Summary

M48 finished servant architecture work. M49 needs to move back to the product core.

The repo already ships real seam semantic review for one `sum` surface and one `data` surface. The problem is not that seam review is fake. The problem is that the supported path still enters through literal unit ids in `supported_surface_for_spec(...)`, which means the current capability is example-specific instead of family-specific.

This plan fixes exactly that. It generalizes supported seam routing one level up, proves the same supported seam families on unseen unit ids, keeps the authored and executable vocabulary intentionally narrow, and preserves downstream status/export/passport behavior. No new ontology. No generic seam understanding claim. No adjacent architecture side quest.

If this lands cleanly, the repo will be able to say something honest and stronger than it can say today: supported seam review is no longer tied to `pricing/discount_policy` and `pricing/checkout_quote`, but it is still bounded to the exact semantic families the evaluator actually understands.

## Decision This Plan Makes

This plan authorizes exactly one slice:

1. Replace literal unit-id seam routing in `semantic_review.rs` with explicit supported seam-family routing.
2. Adopt canonical family keys:
   - `sum.discount_strategy.v1`
   - `data.pricing_quote.v1`
3. Preserve backward compatibility during one migration window:
   - `Preserve` accepts either the canonical key or the legacy key for the same family.
   - Legacy keys are:
     - `sum.discount_policy.v1`
     - `data.checkout_quote.v1`
4. `Refresh` always emits the new canonical family key.
5. Add unseen-unit-id proof for both seam families and prove read-side truth surfaces do not regress.
6. Keep the supported seam vocabulary exact:
   - same supported variants, fields, constructors, method ids, and body-shape classifiers
   - no renamed-vocabulary support in this milestone

This plan does not authorize:

- generic seam understanding
- new supported function families
- corpus expansion or promotion work
- shared-core portability follow-on work
- CLI/schema redesign
- TypeScript parity expansion
- new abstraction layers, traits, or module splits beyond this slice

## Live Validated Basis

Validated from the current tree on `feat/m40-plus` at commit `151f1e9` by reading the active implementation.

Observed truth:

- `supported_surface_for_spec(...)` in [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs) still hard-codes:
  - `pricing/discount_policy`
  - `pricing/checkout_quote`
- `family_b_deps_are_supported(...)` already treats supported `sum` and `data` surfaces as valid dependency surfaces for wrapper-family evaluation.
- `project_semantic_review_with_context(...)` preserves supported truth only when `evaluator_scope` and `compatibility_key` match exactly, which means key migration must be handled explicitly.
- `spec-core/src/export.rs`, `spec-cli/src/commands.rs`, and `spec-cli/tests/cli.rs` already encode preserve-vs-refresh truth-surface behavior and will fail loudly if compatibility handling changes sloppily.
- `spec-core/src/typescript_backend.rs` gates bounded TypeScript support off supported semantic review. It is a proof wall, not new write scope.

This is the whole opportunity. The evaluators are already real. The missing move is reusable routing plus compatibility-proof discipline.

## Step 0: Scope Challenge

### What Already Exists

| Sub-problem | Existing owner | Reuse verdict |
| --- | --- | --- |
| supported seam routing entry point | `spec-core/src/semantic_review.rs::supported_surface_for_spec` | reuse, replace literal id checks with family detection |
| supported sum evaluator | `evaluate_supported_sum_semantic_review(...)` | reuse, route into it through a family contract |
| supported data evaluator | `evaluate_supported_checkout_quote_data_review(...)` | reuse, route into it through a family contract |
| preserve vs refresh projection | `project_semantic_review_with_context(...)` | reuse, extend to canonical-plus-legacy alias matching |
| wrapper dependency support | `family_b_deps_are_supported(...)` | reuse, make it depend on family-routed seams instead of example ids |
| passport/export preserve behavior | `spec-core/src/export.rs` | proof wall only |
| CLI status/build/test/export projection | `spec-cli/src/commands.rs` | proof wall only |
| bounded TypeScript semantic gate | `spec-core/src/typescript_backend.rs` | proof wall only |
| end-to-end truth-surface regression coverage | `spec-cli/tests/cli.rs` | extend, do not redesign |

### Minimum Complete Slice

This is the smallest honest implementation:

1. introduce a seam-family layer
2. route supported seams by semantic family instead of literal id
3. choose and ship canonical seam family keys
4. preserve legacy keys only in `Preserve`
5. prove unseen unit ids for both seam families
6. prove no read-side drift in export/status/passport/TypeScript gating

Anything smaller is a refactor with no substrate gain. Anything larger turns a lake into an ocean.

### Complexity, Completeness, and Distribution

- Primary production write scope: `1` file, [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs)
- Allowed production proof-surface touches: `4` files, only if required to preserve truthful behavior
- Required test/proof surfaces: `4` files
- New crates, services, schema families, or runtime infrastructure: `0`
- Distribution work: `0`, this milestone changes semantic review internals only
- Complete version vs shortcut: choose the complete version, because alias-aware preserve logic plus unseen-id proof is the real deliverable

### Locked Plan Decisions

These decisions are resolved and should not be reopened mid-implementation:

1. Canonical seam family keys are `sum.discount_strategy.v1` and `data.pricing_quote.v1`.
2. Legacy keys are accepted only in `Preserve`, never emitted by `Refresh`.
3. The milestone stays one-file-first. No new `semantic_review/` submodule tree.
4. Detection is family-by-shape only. It must not consult literal unit ids, file paths, or fuzzy intent text.
5. Detection remains explicit and vocabulary-bound. No fuzzy intent inference, no widened synonym set, no approximate matching.
6. Unseen-id proof is required for both seam families before the slice is done.
7. Wrapper-family support must keep working when its seam deps are routed through the new family layer.
8. Function names may be cleaned up for clarity, but canonical compatibility keys and preserve semantics are the real contract.
9. If landing this requires changing CLI JSON shape, export bundle schema, or TypeScript target policy, stop and re-scope.

### Abort and Re-scope Triggers

Stop and write a follow-on plan if any of these become necessary:

1. a new seam family key needs new schema fields or new public JSON shape
2. `spec-cli` command behavior must change beyond compatibility preservation
3. supported seam detection cannot be expressed without widening the supported vocabulary
4. `semantic_review.rs` needs to split into a new framework just to hold two seam families
5. a downstream proof wall needs semantic behavior changes instead of simple alias-preserve support

## Architecture and Ownership

### Supported Seam Family Contract

Use one explicit family layer inside [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs):

```rust
enum SupportedSeamFamily {
    SumDiscountStrategyV1,
    DataPricingQuoteV1,
}

enum SupportedSurface {
    Function(SupportedFunctionFamily),
    Seam(SupportedSeamFamily),
    Unsupported(UnitKind),
}
```

Required helper functions:

- `supported_seam_surface(...) -> Option<SupportedSeamFamily>`
- `detect_sum_discount_strategy_family(...) -> bool`
- `detect_data_pricing_quote_family(...) -> bool`
- `canonical_seam_compatibility_key(...) -> &'static str`
- `legacy_seam_compatibility_keys(...) -> &'static [&'static str]`
- `supported_surface_matches_existing_review(...) -> bool`

That last helper is important. It centralizes the migration rule so preserve logic does not fork quietly.

Detection inputs are also part of the contract:

- allowed: authored seam shape, lowered executable shape, existing explicit classifier helpers
- not allowed: `spec.spec.id`, path-name heuristics, intent-text substring matching, new synonym tables

Naming rule:

- if `evaluate_supported_checkout_quote_data_review(...)` gets renamed for clarity, that is acceptable
- if it stays named for the old example unit, that is also acceptable
- either way, the exported compatibility keys and the supported-family detection behavior are the source of truth

### Routing Flow

```text
evaluate_semantic_review_with_context(...)
  │
  └── supported_surface_for_spec(...)
        ├── supported_function_surface(...) -> existing function routing
        ├── supported_seam_surface(...)
        │     ├── detect_sum_discount_strategy_family(...)
        │     └── detect_data_pricing_quote_family(...)
        └── Unsupported(UnitKind)
              │
              ├── Function -> unsupported_function_review(...)
              └── Sum/Data -> unsupported_surface_review(...)

project_semantic_review_with_context(...)
  │
  ├── Preserve -> accept canonical key or legacy alias for matching seam family
  └── Refresh  -> emit canonical family key only
```

### Family Boundaries

`SumDiscountStrategyV1` still means the current exact authored and executable shape:

- variants: `none`, `percentage`, `fixed_amount`
- methods: `discount_amount`, `discounted_subtotal`
- same explicit classifier expectations already encoded in the evaluator

`DataPricingQuoteV1` still means the current exact authored and executable shape:

- fields: `subtotal`, `discount_rate`, `tax_rate`
- constructor: `new`
- methods: `discounted_subtotal`, `total`
- same supported body classifiers already used for `checkout_quote`

Anything outside that vocabulary remains unsupported in M49. That is good discipline, not missing ambition.

### Dependency Graph

```text
semantic_review.rs
  ├── SupportedFunctionFamily routing                (unchanged contract)
  ├── SupportedSeamFamily routing                    (new explicit layer)
  │     ├── SumDiscountStrategyV1
  │     └── DataPricingQuoteV1
  ├── evaluate_supported_sum_semantic_review(...)   (reused)
  ├── evaluate_supported_checkout_quote_data_review(...) (reused)
  └── project_semantic_review_with_context(...)
          │
          ├── export.rs                    preserve/read-side truth wall
          ├── spec-cli commands.rs         status/build/test/export truth wall
          ├── typescript_backend.rs        bounded target eligibility wall
          └── spec-cli/tests/cli.rs        end-to-end regression wall
```

### Invariants

All of these must still be true after the slice lands:

1. wrapper-family semantic review still accepts supported seam deps
2. unsupported renamed seam vocabulary still stays unsupported
3. `Preserve` never invents fresh supported seam truth
4. `Refresh` never emits legacy seam keys
5. stale or failing base health still outranks semantic-read-side optimism
6. bounded TypeScript gating stays identical except for consuming canonical family truth where relevant

### Contract Freeze Gate

Parallel follow-on work does not start until all of these are locked in `semantic_review.rs` on the main working branch:

1. `SupportedSeamFamily` variant names
2. canonical keys:
   - `sum.discount_strategy.v1`
   - `data.pricing_quote.v1`
3. legacy keys:
   - `sum.discount_policy.v1`
   - `data.checkout_quote.v1`
4. preserve matching policy: canonical-or-legacy only for the matching family
5. refresh policy: canonical key only
6. near-miss policy: renamed vocabulary stays unsupported

This freeze gate is the handoff boundary. If any of these are still moving, do not start parallel proof-wall work.

## Implementation Contract

### Primary Write Scope

Primary write scope is:

- [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs)

Production changes allowed there:

1. add `SupportedSeamFamily`
2. replace `SumDiscountPolicy` / `DataCheckoutQuote` surface variants with family routing
3. add canonical-plus-legacy compatibility helpers
4. update preserve matching logic
5. update wrapper dep support to recognize the new seam surface variant
6. add unseen-id and alias-preserve tests in the same file

### Allowed Proof-Surface Touches

Allowed only if needed to preserve truthfulness:

- [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs)
- [spec-core/src/typescript_backend.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/typescript_backend.rs)
- [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs)
- [spec-cli/tests/cli.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/cli.rs)

Expected use:

- mostly tests
- minimal alias-aware preservation if current equality checks are duplicated elsewhere
- no behavior widening, no output-shape redesign

### Forbidden Scope

Do not touch:

- `xtask/src/family/*`
- repo-root authority docs beyond this plan
- schema versions
- export JSON contract
- TypeScript target-lane product scope
- molecule execution behavior
- new public CLI flags

## Implementation Sequence

### Step 1: Introduce canonical seam-family routing

Inside `semantic_review.rs`:

1. add `SupportedSeamFamily`
2. change `SupportedSurface` to use `Seam(SupportedSeamFamily)`
3. add canonical and legacy key helpers
4. add seam-family-to-evaluator-scope mapping

Definition of done:

- no literal unit-id seams remain in the `SupportedSurface` enum
- compatibility key policy is centralized in helper functions, not inlined across matches
- the six-item contract freeze gate above is fully decided and does not change after this step merges

### Step 2: Replace literal unit-id routing with semantic family detection

Replace:

- `UnitKind::Sum if spec.spec.id == "pricing/discount_policy"`
- `UnitKind::Data if spec.spec.id == "pricing/checkout_quote"`

With:

- `UnitKind::Sum` -> detect `SumDiscountStrategyV1` by current authored plus executable shape
- `UnitKind::Data` -> detect `DataPricingQuoteV1` by current authored plus executable shape

Constraint:

- detectors may reuse existing packet-build helpers and existing role-match helpers
- detectors must not read or branch on `spec.spec.id`
- detectors must not widen accepted vocabulary

Definition of done:

- an unseen unit id with the same supported shape routes to a supported seam family
- a renamed-field or renamed-method near miss still routes to unsupported

### Step 3: Keep evaluator logic explicit and bounded

Do not invent a generic seam evaluator abstraction. Keep the current evaluators and route into them by family.

Allowed refactor:

- parameterize compatibility key emission
- rename evaluator entry points if that improves clarity

Not allowed:

- trait-based evaluator registry
- dynamic rule tables
- splitting into a new architecture layer for two families

Definition of done:

- current canonical fixtures still produce the same verdicts
- evaluator readability remains one-sitting readable

### Step 4: Ship the compatibility-key migration window

Required behavior:

- `Refresh` emits:
  - `sum.discount_strategy.v1`
  - `data.pricing_quote.v1`
- `Preserve` accepts:
  - canonical key for matching family
  - legacy key for matching family
- `Preserve` still drops:
  - mismatched family key
  - unsupported review on supported seam surface
  - stale invented supported review

Definition of done:

- legacy fresh passports survive `status` and `export` on preserve paths during the migration window
- a fresh refresh writes canonical keys only

### Step 5: Prove downstream truth surfaces

Required proof walls:

1. `spec-core/src/export.rs`
2. `spec-cli/src/commands.rs`
3. `spec-core/src/typescript_backend.rs`
4. `spec-cli/tests/cli.rs`

Goal:

- read-side truth remains truthful
- semantic health demotion behavior remains unchanged
- TypeScript bounded-lane gating does not regress when supported seam truth exists in context
- downstream proof-wall edits stay behavioral-noop except for alias-aware preserve compatibility and canonical refresh expectations

### Step 6: Finish with proof-first validation

Required command set:

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-core export
cargo test -p spec-core typescript_backend
cargo test -p spec-cli cli
```

If any alias-preserve logic touches shared projection behavior in a broader way, run:

```bash
cargo test -p spec-core
cargo test -p spec-cli
```

## Code Quality Guardrails

This plan is explicit over clever on purpose.

- Keep the seam-family contract in one file.
- Prefer small private helpers over generic registries.
- Reuse existing packet-build and classifier functions instead of duplicating authored/executable parsing.
- Do not create new near-identical sum/data detectors if a narrow helper can express the shared alias policy.
- Do not copy compatibility-key matching logic into export or CLI code if the semantic-review layer can answer the question once.

The user preference here is obvious: engineered enough, minimal diff, aggressively DRY, but no premature abstraction circus.

## Test Review

100 percent coverage is the goal for the changed code paths. This slice changes semantic routing, projection compatibility, and downstream truth preservation. Every branch listed below needs proof.

### Code Path Coverage

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/semantic_review.rs
    │
    ├── supported_surface_for_spec()
    │   ├── [EXISTING] canonical sum id -> supported seam
    │   ├── [EXISTING] canonical data id -> supported seam
    │   ├── [ADD]      unseen sum id, same variants + methods -> SumDiscountStrategyV1
    │   ├── [ADD]      unseen data id, same fields + methods -> DataPricingQuoteV1
    │   └── [ADD]      renamed-vocabulary near miss -> Unsupported(UnitKind)
    │
    ├── family_b_deps_are_supported()
    │   ├── [EXISTING] wrapper deps accept canonical seam ids
    │   └── [ADD]      wrapper deps accept unseen seam ids routed by family
    │
    ├── project_semantic_review_with_context(Preserve)
    │   ├── [EXISTING] canonical current key preserved
    │   ├── [ADD]      legacy sum key preserved for matching family
    │   ├── [ADD]      legacy data key preserved for matching family
    │   ├── [ADD]      canonical key preserved for matching family
    │   └── [ADD]      mismatched seam family key dropped
    │
    └── project_semantic_review_with_context(Refresh)
        ├── [ADD]      sum refresh emits sum.discount_strategy.v1
        └── [ADD]      data refresh emits data.pricing_quote.v1

[+] spec-core/src/export.rs
    │
    ├── load_passports_for_specs()
    │   ├── [EXISTING] canonical current data key preserved
    │   ├── [ADD]      legacy sum key preserved on unseen-id family match
    │   └── [ADD]      legacy data key preserved on unseen-id family match
    │
    └── build_export_bundle()
        ├── [EXISTING] preserve does not invent missing supported data review
        └── [ADD]      preserve carries legacy seam key through migration window

[+] spec-core/src/typescript_backend.rs
    │
    └── validate_typescript_tree_spec()
        └── [ADD]      supported seam truth in shared context does not break bounded TS validation

[+] spec-cli/tests/cli.rs
    │
    ├── status/export preserve matrix
    │   ├── [ADD]      legacy seam passport survives status on preserve path
    │   ├── [ADD]      legacy seam passport survives export on preserve path
    │   └── [ADD]      refresh rewrites passport semantic_review to canonical family key
    │
    └── seam health semantics
        ├── [EXISTING] incomplete gate still demotes otherwise-valid seam
        └── [EXISTING] stale seam still reports stale after authored change

─────────────────────────────────
PLANNED NEW COVERAGE: 15 paths
  semantic_review.rs: 8
  export.rs: 3
  typescript_backend.rs: 1
  cli.rs: 3
CRITICAL REGRESSION TESTS: 4
  preserve legacy sum key
  preserve legacy data key
  refresh canonical sum key
  refresh canonical data key
─────────────────────────────────
```

### User-Visible and Command-Visible Flows

```text
TRUTH SURFACE FLOW COVERAGE
===========================
[+] spec status --format json
    ├── [ADD] legacy seam review in passport remains visible during preserve window
    ├── [EXISTING] stale authored change still drops fresh supported truth
    └── [EXISTING] incomplete escape-hatch gate still wins over green wishful thinking

[+] spec export
    ├── [ADD] legacy seam review survives preserve projection
    └── [ADD] refreshed seam review emits canonical family key only

[+] spec test / passport write path
    └── [ADD] refreshed seam review rewrites passport semantic_review key canonically

[+] bounded TypeScript lane
    └── [ADD] supported seam context does not poison TS eligibility logic
```

### Required Test Files and Assertions

| File | Test additions required |
| --- | --- |
| `spec-core/src/semantic_review.rs` | unseen sum id routes to canonical family, unseen data id routes to canonical family, legacy preserve alias accepted, refresh emits canonical key, wrapper deps still supported through family routing, renamed-vocabulary near misses stay unsupported |
| `spec-core/src/export.rs` | passport preserve accepts legacy seam keys for matching family, export bundle preserve keeps legacy key alive during migration |
| `spec-core/src/typescript_backend.rs` | semantic review context containing family-routed seam support does not regress bounded TS validation |
| `spec-cli/tests/cli.rs` | command matrix for preserve vs refresh on seam passports using legacy and canonical keys |

### Test Plan Artifact

The companion QA-oriented artifact lives at:

- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-110938.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-110938.md)

It should stay short and command-focused so `/qa` or `/qa-only` can consume it directly.

## Failure Modes

| Codepath | Real failure mode | Test coverage required | Error handling today | User-visible impact | Critical gap if missed |
| --- | --- | --- | --- | --- | --- |
| seam detection | unseen seam id still falls back to unsupported | yes | falls back silently to unsupported | supported substrate claim is fake | yes |
| preserve alias matching | legacy fresh passport dropped during status/export | yes | current exact-match preserve will drop it | user loses previously valid proof after upgrade | yes |
| refresh key emission | refresh keeps writing legacy keys | yes | none, would look green but block migration | public contract stays repo-specific forever | yes |
| wrapper dep support | family-B wrapper no longer recognizes seam deps | yes | wrapper route falls out of supported subset | previously supported wrappers degrade to unsupported | yes |
| near-miss rejection | renamed seam vocabulary accidentally accepted | yes | none if detection widens too far | evaluator overclaims understanding | yes |
| TS bounded lane | shared supported seam context trips TS validation | yes | generator errors early | unrelated TypeScript workflow regresses | no |

## Performance Review

No major runtime or memory risk is justified here, but there are two guardrails:

1. Do not parse the same executable body repeatedly inside the same evaluation path if a local helper can hold the result once. `status`, `export`, and `test` already walk many specs, so accidental double work compounds.
2. Do not introduce new cross-spec scans outside the existing dependency-resolution and context lookups. This slice should stay constant-factor work on top of the current evaluator, not a new graph walk.

If implementation keeps the change inside existing packet-build and classifier flow, performance should remain effectively unchanged.

## NOT in Scope

These were considered and are explicitly deferred:

- generic renamed seam vocabulary support, because M49 is about reusability of the current honest subset, not wider semantic inference
- new seam families, because live recommendation state still does not authorize family-promotion theater
- CLI/schema changes, because the value here is semantic substrate truth, not surface churn
- TypeScript seam support expansion, because the current TS lane is a proof wall only
- cross-crate/shared-core extraction, because this is not another servant-architecture milestone
- docs or README rewrites outside brief key-name updates if needed after landing, because they do not block the core slice

## Worktree Parallelization Strategy

This plan has a real parallelization opportunity after the semantic contract is frozen.

### Dependency Table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Core seam-family routing and alias policy | `spec-core::semantic_review` | — |
| B. Export and TypeScript proof walls | `spec-core::export`, `spec-core::typescript_backend` | A |
| C. CLI preserve/refresh proof matrix | `spec-cli::commands`, `spec-cli::tests` | A |

Lane ownership is strict:

- `Lane A` owns `spec-core/src/semantic_review.rs`
- `Lane B` owns `spec-core/src/export.rs` and `spec-core/src/typescript_backend.rs`
- `Lane C` owns `spec-cli/src/commands.rs` and `spec-cli/tests/cli.rs`

If a lane needs to edit another lane's owned file, stop and collapse back to sequential execution.

### Parallel Lanes

- `Lane A`: Step A, sequential, establishes the contract and canonical key names.
- `Lane B`: Step B, can start after Lane A freezes key names and preserve semantics.
- `Lane C`: Step C, can start after Lane A for command-surface proof.

ASCII execution map:

```text
Lane A  semantic_review.rs
  │
  ├── freeze family enum + canonical keys + legacy aliases + preserve/refresh rules
  │
  ├──────────────┬──────────────
  │              │
  ▼              ▼
Lane B         Lane C
export.rs      commands.rs + cli.rs
typescript     preserve/refresh matrix
proof walls
  │              │
  └──────┬───────┘
         ▼
   integrated validation
```

### Execution Order

1. Launch `Lane A` first.
2. Do not launch downstream work just because `Lane A` compiles. Launch it only after the contract freeze gate is explicitly satisfied.
3. Once `Lane A` compiles and the contract freeze gate is locked, launch `Lane B` and `Lane C` in parallel worktrees.
4. Merge `Lane B` and `Lane C`.
5. Run the full proof command set once on the integrated branch.

### Conflict Flags

- `Lane B` and `Lane C` both depend on the exact canonical key strings from `Lane A`. Freeze those strings before parallel work starts.
- `Lane B` and `Lane C` should not both edit `spec-core/src/semantic_review.rs`. If they do, parallelization failed and should be collapsed back to sequential.
- `Lane C` will be noisy if `Lane A` has not already stabilized the preserve-vs-refresh expectations. Do not let CLI tests become the place where the semantic contract is decided.

## Implementation Checklist

1. Replace seam `SupportedSurface` id variants with `SupportedSeamFamily`.
2. Add canonical and legacy seam key helpers.
3. Route sum seams by semantic family, not literal id.
4. Route data seams by semantic family, not literal id.
5. Freeze the seam contract: enum names, canonical keys, legacy keys, preserve rules, refresh rules, near-miss rejection rules.
6. Keep current evaluator vocabulary exact.
7. Update preserve logic to accept canonical plus legacy key aliases for the same family.
8. Update refresh logic to emit canonical keys only.
9. Prove wrapper-family deps still accept family-routed seams.
10. Add export preserve tests for legacy seam passports.
11. Add CLI status/export/test matrix tests for legacy and canonical seam keys.
12. Add TS-context regression proof if needed.
13. Run targeted package tests, then full package tests if any proof wall needed broader touch.

## Completion Summary

- Step 0: Scope Challenge, scope accepted as-is
- Architecture Review: 1 core architecture change, keep it one-file-first
- Code Quality Review: explicit-over-clever guardrails written
- Test Review: coverage diagram produced, 15 planned new paths identified
- Performance Review: 0 major findings, 2 guardrails
- NOT in scope: written
- What already exists: written
- Failure modes: 5 critical gaps flagged
- Parallelization: 3 steps, 2 downstream lanes parallel after core contract freeze
- Lake Score: 5/5 major recommendations chose the complete option over the shortcut

## Recommended Next Action

Execute `Lane A` first in [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs). Treat the contract freeze gate as the mandatory handoff point. Do not touch export, CLI, or TypeScript proof walls until the canonical family keys, legacy aliases, and preserve-vs-refresh rules are frozen.
