# M60: Normalized-Required-Arg Wrapper Family Execution Plan

Status: **authoritative implementation plan**  
Milestone: **M60**  
Milestone family: **semantic-review substrate**  
Implementation readiness: **ready for bounded execution**  
Plan scope: **ship exactly one new supported function family, `function.wrapper.pipeline.normalized_required_arg.v1`, for the bounded case where a two-step wrapper normalizes the second dep's required argument with `param.max(Decimal::ZERO)`; preserve `function.wrapper.pipeline.v1` as the strict raw-argument sibling; update maintained examples, regression fixtures, family-analysis read-side truth, and public docs in the same PR; do not widen to generic expression understanding, new dep topology, new TypeScript execution behavior, new seam families, or a corpus-program reopen**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Validated at commit: **`f401d49`**  
Last rewritten: **2026-05-15**

Supersedes:

- the shipped M59 authority plan previously maintained at this path
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260514-192715.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260514-135734.md`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260514-192715.md`
- `README.md`
- `TODOS.md`
- `CHANGELOG.md`
- `CLAUDE.md`
- `semantic-families/function.wrapper.pipeline.v1/candidate.md`

Primary repo surfaces:

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/**`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/**`
- `examples/ecommerce/units/pricing/**`
- `semantic-families/**`
- `README.md`
- `TODOS.md`
- `CHANGELOG.md`

## Executive Summary

M59 finished the local TypeScript graph widen. That work is done.

M60 is not another TypeScript milestone, not a family-analysis milestone, and not generic wrapper expression support.

M60 is one narrower product truth widen:

```text
support exactly one additional wrapper topology:
dep_b(dep_a(...), normalized_required_arg)
where normalized_required_arg == param.max(Decimal::ZERO)
```

The new family key is:

```text
function.wrapper.pipeline.normalized_required_arg.v1
```

This is the right next lake because the pressure is already real in the repo:

- `semantic-families/function.wrapper.pipeline.v1/candidate.md` calls out this exact near miss
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/checkout_net_total_unsupported_near_miss.unit.spec` uses the same boundary
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/calculate_total.unit.spec` uses the same boundary

The milestone succeeds only if the repo ends in a truthful state:

- the new family classifies one maintained real example
- the old wrapper family stays strict
- current unsupported fixtures that now fall inside the new family are either migrated or rewritten so the unsupported packs remain honest
- family-analysis read-side commands still say the corpus program is stopped, not reopened
- README, CHANGELOG, and TODOS all describe the new boundary without implying generic expression understanding

## Frozen Implementation Decisions

These decisions are locked for M60. If any of them changes, the milestone scope changed and the plan must be rewritten before implementation continues.

1. **Ship a sibling family, not a widened raw family.**
   - Add `function.wrapper.pipeline.normalized_required_arg.v1`
   - Keep `function.wrapper.pipeline.v1` raw-arg only

2. **Admit exactly one required-arg normalization surface.**
   - Supported: `param.max(Decimal::ZERO)`
   - Unsupported: literals, arithmetic, chained methods, multi-input expressions, and multi-arg normalization

3. **Add one maintained real example and keep the old canonical example intact.**
   - Add `examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec`
   - Do not rewrite `examples/ecommerce/units/pricing/calculate_total.unit.spec`

4. **Repair the promoted unsupported fixtures in place, do not move them.**
   - Keep the current file paths and ids stable
   - Rewrite only the body expressions so the fixtures remain honest owners of `unsupported_required_argument_expression`
   - Exact replacement shapes:
     - `m19_semantic_falsification_pack::billing/checkout_net_total_unsupported_near_miss`
       -> `regional_rate.max(Decimal::ZERO).round_dp(4)`
     - `m20_unsupported_truth_pack::pricing/calculate_total`
       -> `tax_rate + Decimal::ZERO`

5. **Mirror the existing wrapper packet layout instead of inventing a new packet shape.**
   - Create `semantic-families/function.wrapper.pipeline.normalized_required_arg.v1/`
   - Reuse the same four buckets: `aligned`, `drift`, `under_specified`, `unsupported_near_miss`
   - Reuse the same packet-local leaf naming conventions as `function.wrapper.pipeline.v1`

6. **Freeze the public docs sentence.**
   - Use one exact statement everywhere:
     - "M60 adds one supported wrapper family for `apply_tax(discounted, tax_rate.max(Decimal::ZERO))`; broader required-argument expressions remain unsupported."

## Current Validated Basis

Validated on `feat/m40-plus` at `f401d49` with:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Observed live branch truth:

- `function_coverage.total_units = 31`
- `function_coverage.promoted_family_units = 23`
- `function_coverage.supported_unpromoted_family_units = 0`
- `function_coverage.unsupported_function_units = 8`
- `recommendation_status = "insufficient_real_corpus"`
- `decision_status = "not_recommended"`
- `corpus_program_decision.decision_action = "stop"`
- `verify-decision-contract.overall_verdict = "pass"`
- the unsupported cluster `unsupported_required_argument_expression` still contains:
  - `m19_semantic_falsification_pack::billing/checkout_net_total_unsupported_near_miss`
  - `m20_unsupported_truth_pack::pricing/calculate_total`

That last bullet is the actual wedge. Today those shapes are unsupported. M60 makes one of them supported under a new explicit key, then repairs the regression packs so unsupported truth remains honest.

## Step 0: Scope Challenge

### Premise correction

The problem is not "wrappers need computed expressions now."

The real problem is smaller:

```text
the semantic reviewer has no truthful supported family for
wrapper.pipeline + one normalized required arg
```

If this milestone widens beyond that sentence, it is overbuilt.

### What already exists

| Sub-problem | Existing owner | M60 action |
| --- | --- | --- |
| supported-family routing | `SupportedFunctionRoute` in `spec-core/src/semantic_review.rs` | extend with one new explicit sibling route |
| supported-family compatibility key emission | `SupportedSurface::compatibility_key()` in `spec-core/src/semantic_review.rs` | add one new key |
| strict raw-arg wrapper proof | `function.wrapper.pipeline.v1` tests and packet | preserve unchanged |
| existing normalized near-miss examples | M19 + M20 fixture packs and wrapper candidate doc | migrate or rewrite so they stay truthful |
| maintained canonical pricing example | `examples/ecommerce/units/pricing/calculate_total.unit.spec` plus molecule tests | add one new guarded-tax example instead of mutating the old raw-arg canonical example |
| family-analysis read-side surfaces | `cargo xtask family coverage/recommend/corpus-decision/verify-decision-contract` | refresh outputs, keep stop-state honest |
| unsupported-function projection rules | `spec-cli/tests/cli.rs` whole-pack status/export tests | update to match the new supported boundary and replacement unsupported cases |

### Minimum complete slice

The minimum honest M60 slice is:

1. add one new supported family key and route
2. classify exactly one admitted normalized required-arg expression surface, `param.max(Decimal::ZERO)`
3. keep `function.wrapper.pipeline.v1` strict on raw required args only
4. add one maintained real example in `examples/ecommerce`
5. add one dedicated semantic-family packet for the new family with aligned, drift, under-specified, and unsupported-near-miss buckets
6. repair M19 and M20 so the unsupported packs still contain genuinely unsupported shapes after the promotion
7. update README, TODOS, and CHANGELOG in the same PR

Anything smaller is fake done.

Examples:

- adding the new route without the maintained example is fake done
- adding the example without packet truth is fake done
- promoting the old M20 unsupported case without replacing unsupported coverage is fake done
- updating semantic review but not family-analysis commands is fake done

### Complexity and blast radius

This milestone crosses more than 8 files. That normally smells.

It is still the right size because the extra files are proof and truth surfaces, not new infrastructure:

- one core classifier file
- one CLI integration test file
- two existing regression fixture trees
- one maintained example tree
- one new semantic-family packet directory
- three public docs

The complete version is only modestly larger than the shortcut, and the shortcut would leave the repo lying about supported versus unsupported truth. Boil the lake.

### Search check

No framework built-in replaces this work. This is repo-owned semantic classifier logic.

- **[Layer 1]** Reuse the current supported-route architecture in `spec-core/src/semantic_review.rs`
- **[Layer 1]** Reuse the current family packet system in `semantic-families/**`
- **[Layer 1]** Reuse the current unsupported truth pack and stale-proof CLI assertions in `spec-cli/tests/cli.rs`
- **[Layer 3]** The right design is not a generic expression matcher. The right design is one explicit family boundary because the repo sells truthful semantic families, not clever AST tolerance

### TODOS cross-reference

`TODOS.md` already tracks the remaining post-M59 TypeScript oceans. M60 must not reopen any of them.

The same PR should update `TODOS.md` so it says:

- normalized-required-arg wrapper support shipped in M60
- broader required-arg normalization remains deferred
- generic expression support remains deferred
- arbitrary 4+ dep topology remains deferred

### Completeness and distribution check

No new distributable artifact is introduced.

This remains a capability widen inside the existing `spec` CLI and existing GitHub release surface. Distribution work is already in place. The complete version here is proof completeness, not packaging work.

## Milestone Contract

### Exact shipped behavior

After M60:

- a `kind:function` wrapper may classify to `function.wrapper.pipeline.normalized_required_arg.v1` when:
  - it has the same two-dep wrapper topology as the current wrapper family
  - dep 1 is the monotone-down discount leaf
  - dep 2 is the monotone-up tax leaf
  - the second dep receives:
    - the first dep's output as its primary value argument
    - exactly one required argument derived from exactly one declared input
    - that derivation is exactly `param.max(Decimal::ZERO)`
- the old raw-arg wrapper family remains:
  - `function.wrapper.pipeline.v1`
  - still raw-arg only
  - still rejects normalized required-arg expressions
- broader expressions still map to `unsupported_required_argument_expression`
- family-analysis commands still end in stop-state unless a separate future milestone produces enough new real-example pressure

### Exact admitted surface

| Shape | Outcome |
| --- | --- |
| `apply_tax(discounted, tax_rate)` | `function.wrapper.pipeline.v1` |
| `apply_tax(discounted, tax_rate.max(Decimal::ZERO))` | `function.wrapper.pipeline.normalized_required_arg.v1` |
| `apply_tax(discounted, Decimal::ZERO)` | `unsupported_required_argument_expression` |
| `apply_tax(discounted, tax_rate + Decimal::ZERO)` | `unsupported_required_argument_expression` |
| `apply_tax(discounted, tax_rate.max(Decimal::ZERO).round_dp(4))` | `unsupported_required_argument_expression` |
| `apply_tax(discounted, regional_rate.max(Decimal::ZERO))` when the authored input is `tax_rate` | semantic drift or unsupported, depending on the authored/body mismatch |

### Exact maintained example seed

Add:

```text
examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec
```

Authored story:

- intent: return checkout total after discounting the subtotal, then apply tax using a rate normalized to nonnegative
- deps:
  - `pricing/apply_discount`
  - `pricing/apply_tax`
- body:
  - bind discounted subtotal once
  - call `apply_tax(discounted, tax_rate.max(Decimal::ZERO))`

The existing `examples/ecommerce/units/pricing/calculate_total.unit.spec` stays in place as the strict raw-arg wrapper example. M60 adds a sibling example. It does not silently rewrite the old canonical family out from under existing docs.

### Explicit non-goals

M60 does not include:

- generic wrapper expression support
- arithmetic or chained-method normalization beyond `param.max(Decimal::ZERO)`
- multiple normalized required arguments
- required-arg normalization composed from multiple authored inputs
- generic fanout or reducer families
- arbitrary 4+ dep topology parity
- corpus-program reopen by default
- new TypeScript execution behavior
- seam family expansion
- `spec validate --target-language`
- `spec export --target-language`

## Architecture Review

### Dependency graph

```text
                     +----------------------------------+
                     | spec-core/src/semantic_review.rs |
                     +----------------------------------+
                       | add compatibility key
                       | add explicit route
                       | add normalized-arg classifier
                       v
        +-----------------------------+     +-----------------------------+
        | Supported function routing  | --> | semantic review emission    |
        | chain3 -> normalized -> raw |     | support_status / key / body |
        +-----------------------------+     +-----------------------------+
                       |
                       +--------------------+
                                            |
                                            v
             +-----------------------------------------------+
             | Proof surfaces                                |
             | - spec-core unit tests                        |
             | - spec-cli/tests/cli.rs                       |
             | - semantic-families/new packet               |
             | - M19/M20 fixture pack repair                 |
             | - examples/ecommerce guarded-tax seed         |
             +-----------------------------------------------+
                                            |
                                            v
             +-----------------------------------------------+
             | Read-side truth                               |
             | - family coverage                             |
             | - family recommend                            |
             | - corpus decision remains stop                |
             | - README / TODOS / CHANGELOG                  |
             +-----------------------------------------------+
```

### Routing order

Add a new route adjacent to the current wrapper route. Recommended order:

```text
WrapperPipelineChain3
WrapperPipelineNormalizedRequiredArg
WrapperPipeline
ArithmeticLeafMonotoneDownNonnegative
ArithmeticLeafMonotoneUp
HelperIdentityPassthrough
```

Why this order:

- the normalized route is a sibling, not a post-hoc exception inside raw wrapper logic
- keeping it adjacent to `WrapperPipeline` makes the boundary obvious to maintainers
- evaluating it before the raw wrapper route reduces the chance of future silent widening if the raw route becomes more permissive later

### Core classification flow

```text
LoadedSpec
  -> supported_surface_for_spec(...)
    -> SupportedFunctionRoute::WrapperPipelineNormalizedRequiredArg.try_match(...)
      -> authored wrapper topology check
      -> dep semantic-family check
      -> normalized required-arg surface check
      -> body verdict:
           aligned | semantic_drift | under_specified | unsupported
```

### File-by-file responsibilities

- `spec-core/src/semantic_review.rs`
  - add the new compatibility key constant
  - add the new `SupportedFunctionRoute`
  - add an explicit family variant with a readable name, not another opaque `FamilyD`
  - factor normalized required-arg classification into a small explicit helper
  - keep the old raw wrapper classifier strict
- `spec-cli/tests/cli.rs`
  - update whole-pack truth assertions
  - add stale-proof coverage for the new family if needed
  - keep unsupported-function read-side neutrality honest for the replacement unsupported cases
- `semantic-families/function.wrapper.pipeline.normalized_required_arg.v1/**`
  - add `family.toml`
  - add `candidate.md`
  - add aligned / drift / under_specified / unsupported_near_miss packet fixtures
- `examples/ecommerce/units/pricing/**`
  - add the guarded-tax seed
  - keep existing molecule coverage or add one small molecule only if needed for the example story
- `spec-cli/tests/fixtures/m19/**` and `spec-cli/tests/fixtures/m20/**`
  - remove newly promoted shapes from unsupported ownership
  - replace them with still-unsupported shapes that preserve the same reason code

## Code Quality Review

### Design choices

1. **Add one explicit family variant, not a generic expression subsystem.**
   This matches explicit-over-clever and minimal diff. The milestone is about truthful family naming, not AST ambition.

2. **Do not rename all existing `FamilyA/B/C` internals in M60.**
   That cleanup can happen later if it becomes valuable. Renaming everything here would mix structural cleanup with behavioral change.

3. **Name the new route and compatibility key explicitly.**
   Example acceptable internal names:
   - `SupportedFunctionRoute::WrapperPipelineNormalizedRequiredArg`
   - `SupportedFunctionFamily::WrapperPipelineNormalizedRequiredArg`

4. **Keep the normalized surface helper tiny and local.**
   One helper that recognizes `param.max(Decimal::ZERO)` is enough. Anything broader burns an innovation token for no user value today.

5. **Treat M19 and M20 as contract surfaces, not throwaway fixtures.**
   If a promoted family invalidates a current unsupported fixture, that fixture must move or change in the same PR.

6. **Prefer in-place fixture rewrites over id churn.**
   Keep the current M19 and M20 unsupported file paths stable and change only the required-arg expression. This minimizes CLI fixture fallout and keeps the historical test surfaces legible.

### DRY and maintenance rules

- reuse current wrapper topology helpers where possible
- reuse current unsupported reason code plumbing
- do not duplicate coverage logic between the new family packet and the maintained example beyond the minimum proof each surface needs
- keep docs phrasing identical across README, CHANGELOG, and TODOS for the admitted surface
- keep packet layout identical to the existing raw wrapper sibling unless a concrete proof failure forces divergence

## Implementation Plan

### Step 1. Add the new semantic family route and compatibility key

Files:

- `spec-core/src/semantic_review.rs`

Changes:

1. add the new compatibility key string:
   - `function.wrapper.pipeline.normalized_required_arg.v1`
2. add the new family enum variant
3. add the new route immediately before the raw wrapper route
4. emit the new key on supported aligned, drift, and under-specified reviews for that family

Acceptance:

- the new route is reachable
- raw wrapper cases still classify to `function.wrapper.pipeline.v1`
- normalized cases no longer fall through to unsupported by default

### Step 2. Implement bounded normalized required-arg classification

Files:

- `spec-core/src/semantic_review.rs`

Changes:

1. add one explicit classifier for the second dep's required argument
2. admit exactly `param.max(Decimal::ZERO)`
3. keep broader expressions under `unsupported_required_argument_expression`
4. preserve aligned / drift / under-specified verdict behavior for the new family

Acceptance:

- aligned case returns the new family key
- authored/body mismatch still returns semantic drift
- vague authored truth still returns under-specified
- unsupported expressions still map to the stable unsupported reason code

### Step 3. Add the maintained ecommerce seed

Files:

- `examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec`
- any directly related example molecule or passport refresh artifacts produced by the normal loop

Changes:

1. add the new unit spec
2. keep the existing raw-arg `calculate_total.unit.spec` unchanged
3. prove the new seed through the standard CLI loop

Acceptance:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec --format json
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec
cargo run -p spec-cli -- status examples/ecommerce --format json
```

### Step 4. Add the family packet and repair existing unsupported packs

Files:

- `semantic-families/function.wrapper.pipeline.normalized_required_arg.v1/**`
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/**`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/**`

Changes:

1. create `semantic-families/function.wrapper.pipeline.normalized_required_arg.v1/` by mirroring the sibling `function.wrapper.pipeline.v1/` packet layout:
   - `candidate.md`
   - `family.toml`
   - `fixtures/aligned/**`
   - `fixtures/drift/**`
   - `fixtures/under_specified/**`
   - `fixtures/unsupported_near_miss/**`
2. keep the packet-local leaf fixture naming identical to the sibling packet:
   - `pricing_discount_leaf_{bucket}.unit.spec`
   - `pricing_tax_leaf_{bucket}.unit.spec`
3. keep the wrapper fixture naming identical to the sibling packet inside the new directory:
   - `pricing_total_wrapper_{bucket}.unit.spec`
4. rewrite the promoted unsupported fixtures in place instead of moving ids across packs:
   - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/checkout_net_total_unsupported_near_miss.unit.spec`
     - keep id and file path
     - change the tax/fee argument expression to `regional_rate.max(Decimal::ZERO).round_dp(4)`
   - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/calculate_total.unit.spec`
     - keep id and file path
     - change the tax argument expression to `tax_rate + Decimal::ZERO`
5. preserve `unsupported_required_argument_expression` ownership for both rewritten fixtures
6. do not rename fixture files in M19 or M20 unless a test harness hard-requires it

Acceptance:

- the new packet proves the new family
- M19 and M20 remain truthful unsupported packs under the same ids and file paths
- no unsupported pack case accidentally turns green without being intentionally re-homed

### Step 5. Refresh CLI truth and public docs

Files:

- `spec-cli/tests/cli.rs`
- `README.md`
- `TODOS.md`
- `CHANGELOG.md`

Changes:

1. update CLI integration tests for the new family and repaired unsupported packs
2. update README supported-family inventory
3. update TODOS to reflect M60 shipped and broader expression support still deferred
4. update CHANGELOG unreleased entry

Acceptance:

```bash
cargo test -p spec-cli --test cli
```

- CLI truth assertions match the promoted family plus the in-place M19/M20 replacements
- docs all use the same frozen wording for the admitted surface and the deferred boundary

### Step 6. Run the final proof wall and capture the post-change basis

Files:

- none authored; verification and generated artifacts only

Changes:

1. run the maintained example proof loop
2. run the spec-core and spec-cli proof suites
3. run the family-analysis read-side commands
4. record the post-change counts that replace the "Current Validated Basis" snapshot in the final landing pass

Acceptance:

```bash
cargo test -p spec-core semantic_review
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json

cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec --format json
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec
cargo run -p spec-cli -- status examples/ecommerce --format json
```

Expected result:

- coverage shows the new family honestly
- recommendation still says stop
- corpus decision still says stop
- verify-decision-contract still passes

## Test Review

### Test framework and proof owners

This repo's proof wall is Rust-native:

- unit-style semantic-review tests in `spec-core/src/semantic_review.rs`
- CLI integration tests in `spec-cli/tests/cli.rs`
- fixture-pack truth through `spec-cli/tests/fixtures/**`
- family-analysis read-side proof through `cargo xtask family ...`
- maintained example proof through `cargo run -p spec-cli -- validate/test/status ...`

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/semantic_review.rs
    |
    ├── SupportedFunctionRoute order
    |   ├── [EXISTING] raw wrapper route
    |   └── [ADD]      normalized-required-arg sibling route
    |
    ├── wrapper topology contract
    |   ├── [EXISTING] raw dep pair validation
    |   └── [ADD]      same dep pair + normalized required-arg validation
    |
    ├── required-arg expression classifier
    |   ├── [EXISTING] raw param accepted
    |   ├── [ADD]      `param.max(Decimal::ZERO)` accepted
    |   ├── [ADD]      arithmetic expression rejected
    |   ├── [ADD]      chained method expression rejected
    |   └── [ADD]      literal replacement rejected
    |
    └── verdict emission
        ├── [EXISTING] aligned / drift / under-specified / unsupported for raw wrapper
        └── [ADD]      aligned / drift / under-specified / unsupported for normalized sibling

READ-SIDE TRUTH COVERAGE
===========================
[+] examples/ecommerce guarded-tax seed
    ├── [ADD] validate JSON routes to supported family
    ├── [ADD] spec test refreshes passport truth
    └── [ADD] status/export surfaces stay honest

[+] m19 semantic falsification pack
    ├── [ADD] promoted near miss is removed or rewritten
    └── [ADD] replacement unsupported case still emits `unsupported_required_argument_expression`

[+] m20 unsupported truth pack
    ├── [ADD] promoted case is removed or rewritten
    └── [ADD] whole-pack status/export matrix stays truthful

[+] family-analysis commands
    ├── [ADD] coverage includes the new family
    ├── [ADD] recommendation remains stop-state
    ├── [ADD] corpus decision remains stop-state
    └── [ADD] verify-decision-contract remains pass
```

### Required tests

Add or update the following proof:

1. `spec-core/src/semantic_review.rs`
   - aligned normalized case routes to `function.wrapper.pipeline.normalized_required_arg.v1`
   - drift normalized case stays in the new family with `semantic_drift`
   - under-specified normalized case stays in the new family with `under_specified`
   - old raw wrapper case still routes to `function.wrapper.pipeline.v1`
   - arithmetic required-arg expression stays unsupported
   - chained-method required-arg expression stays unsupported
   - literal required-arg expression stays unsupported

2. `spec-cli/tests/cli.rs`
   - guarded-tax example validate/test/status loop
   - M19 replacement unsupported case stays unsupported on status/export/test refresh
   - M20 whole-pack truth matrix reflects the promoted family and the replacement unsupported case

3. family packet proof
   - aligned
   - drift
   - under_specified
   - unsupported_near_miss

### Regression rule

This milestone reclassifies current repo truth. That makes regression tests mandatory.

Required regressions:

- raw wrapper family still rejects normalized required args
- unsupported packs remain unsupported after the promotion
- coverage/recommend/corpus-decision do not silently claim a corpus reopen

## Failure Modes Registry

| New codepath | Real production failure | Test covers it? | Error handling exists? | User-visible effect | Priority |
| --- | --- | --- | --- | --- | --- |
| new route insertion | route order is wrong and normalized case still falls to unsupported | must add | yes, via existing unsupported projection | maintainer sees false unsupported result | high |
| new family classifier | old raw wrapper family silently widens too | must add | no automatic protection without regression | maintainer loses precise family boundary | critical |
| M19 fixture repair | promoted shape remains in unsupported pack | must add | no | false negative in unsupported proof surfaces | critical |
| M20 fixture repair | whole-pack truth matrix still expects old unsupported review | must add | no | status/export tests go red or lie | high |
| ecommerce seed | example exists but never gets exercised in the normal loop | must add | partial | docs claim real-example backing that does not exist | high |
| docs update | README says "computed arguments supported" too broadly | manual review + doc diff | no | users over-assume feature breadth | medium |
| family-analysis refresh | coverage updates but recommend/corpus decision logic drifts | must add | yes via verifier | maintainers get false next-action guidance | high |

Critical gaps to avoid:

- any path with no test, no replacement fixture, and silent public-contract drift

## Performance Review

This milestone should be performance-neutral if implemented correctly.

Expected characteristics:

- one extra supported-function route, constant-factor only
- no new graph traversal depth
- no new cross-file loading behavior
- no new read-side artifact format

Guardrails:

- do not reparse broader expression trees than necessary
- keep the normalized required-arg helper bounded to the existing wrapper classifier path
- avoid introducing a generic expression-normalization abstraction that every supported route now pays for

## NOT in scope

- generic expression-tolerant wrapper support, because M60 is one named family boundary, not an expression engine
- any new TypeScript execution behavior, because this milestone is semantic review only
- any corpus-program restart, because live family-analysis truth still says stop
- any new seam family or non-function surface, because the pressure is in `kind:function`
- any rename-only cleanup of all existing `FamilyA/B/C` internals, because that is structural churn without milestone value
- any change to `examples/ecommerce/units/pricing/calculate_total.unit.spec`, because the raw-arg canonical example should remain the strict sibling

## TODOS.md updates required in the same PR

1. mark the new normalized-required-arg wrapper family as shipped in the post-M59 follow-up area
2. explicitly defer:
   - broader required-arg normalization surfaces
   - multiple normalized required args
   - generic computed required-arg support
3. remove any wording that still implies the specific `max(Decimal::ZERO)` wrapper shape is unsupported

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| 1. classifier route + key | `spec-core/src/` | — |
| 2. maintained example + packet + fixture migration | `examples/ecommerce/units/`, `semantic-families/`, `spec-cli/tests/fixtures/` | 1, exact family key and exact replacement shapes frozen |
| 3. CLI truth assertions | `spec-cli/tests/` | 1 and 2 |
| 4. docs and release notes | repo-root docs: `README.md`, `TODOS.md`, `CHANGELOG.md`, `PLAN.md` | 1, frozen docs sentence and replacement shapes |
| 5. core proof refresh | `spec-core/src/`, workspace test commands | 1 and 2 |
| 6. final proof wall and basis capture | workspace commands / generated artifacts | 2, 3, 4, 5 |

### Parallel lanes

- **Lane A:** Step 1, classifier route + key, sequential inside `spec-core/src/`
- **Lane B:** Step 2, maintained example + packet + fixture migration, sequential across `examples/ecommerce/units/`, `semantic-families/`, and `spec-cli/tests/fixtures/`
- **Lane C:** Step 4, docs and release notes, sequential inside repo-root docs after the wording and replacement shapes are frozen
- **Lane D:** Step 3, CLI truth assertions, after Lane A and Lane B converge
- **Lane E:** Step 5, core proof refresh, after Lane A and Lane B converge
- **Lane F:** Step 6, final proof wall and basis capture, after C + D + E converge

### Execution order

Launch **Lane A** first.

Once the exact family key, route name, docs sentence, and admitted unsupported replacement shapes are frozen, launch **Lane B** and **Lane C** in parallel worktrees.

After Lane B lands, run **Lane D** for CLI truth assertions and **Lane E** for core proof refresh.

After B, C, D, and E merge, run **Lane F** serially for the maintained-example loop, family-analysis commands, and final basis capture.

### Conflict flags

- **Lane B** and **Lane D** both touch the broad `spec-cli/tests/` module area, even if not the same files. Sequence D after B to avoid fixture/test expectation drift.
- **Lane A** and **Lane E** both touch `spec-core/src/semantic_review.rs` and its proof expectations. Treat E as downstream proof-only work after A, not a parallel edit lane.
- **Lane C** should not start before the exact compatibility key, frozen docs sentence, and unsupported replacement shapes are frozen, or the docs will drift from the code.
- Do not split `semantic-families/**`, `spec-cli/tests/fixtures/**`, and `examples/ecommerce/units/**` into separate uncoordinated lanes. They encode the same product boundary and should move together.

## Definition of Done

M60 is done when all of the following are true:

1. `spec-core/src/semantic_review.rs` classifies the new bounded wrapper shape to `function.wrapper.pipeline.normalized_required_arg.v1`
2. the old `function.wrapper.pipeline.v1` family still rejects normalized required args
3. the maintained ecommerce seed exists and proves through `validate`, `test`, and `status`
4. the new semantic-family packet exists with aligned, drift, under-specified, and unsupported-near-miss buckets
5. M19 and M20 no longer contain newly supported shapes in their unsupported packs
6. `cargo xtask family coverage --format json` reflects the new family honestly
7. `cargo xtask family recommend --format json` still says no new corpus action is recommended
8. `cargo xtask family corpus-decision --format json` still says stop
9. `cargo xtask family verify-decision-contract --format json` passes
10. README, TODOS, and CHANGELOG all describe the exact admitted surface and exact remaining boundary

## Verification Commands

Run in this order:

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-cli --test cli

cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec --format json
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total_guarded_tax.unit.spec
cargo run -p spec-cli -- status examples/ecommerce --format json

cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Expected outcome:

- all tests green
- the guarded-tax example routes to the new family
- repaired M19 and M20 packs stay truthful
- family-analysis remains in stop-state, with no accidental "go build more corpus" drift

## Completion Summary

- Step 0: Scope Challenge, complete
- Architecture: one new family route, not a generic expression engine
- Code Quality: explicit sibling family, minimal diff, no broad refactor, stable fixture ids
- Test Review: full proof wall defined across unit, CLI, packet, example, and family-analysis surfaces
- Performance Review: constant-factor only, no new graph traversal
- NOT in scope: written
- What already exists: written
- TODOS.md updates: required in same PR
- Failure modes: critical gaps identified
- Parallelization: 6 steps, 2 early authoring lanes after the classifier freeze, 2 downstream proof lanes, 1 final convergence lane

This is the whole game. Ship one new truthful family, repair the repo surfaces that depended on it being unsupported, and stop there.
