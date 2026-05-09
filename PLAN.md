<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260509-155219.md -->
# M41 - Helper-Surface Semantic Review Substrate

Status: **authority plan**  
Milestone family: **semantic-review-substrate**  
Implementation readiness: **ready-now**  
Next artifact kind: **authority_plan**  
Autoplan ready: **yes**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Last rewritten: **2026-05-09**  
Source design doc: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260509-163237.md`**  
Related test plan: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260509-163237.md`**

## Executive Verdict

M40+ is done.

The next honest move is one bounded capability expansion in the runtime semantic reviewer for the existing `money/round` helper wedge. The repo already proved this wedge is real, already proved it should not be promoted as the next semantic-family packet, and still reports it to users as generic unsupported pressure. That is the lie M41 retires.

M41 ships exactly one new supported helper route end to end:

- runtime semantic review in `spec-core`
- read-side truth in passport, status, and export surfaces
- operator truth in `xtask` inventory, coverage, and recommendation outputs

This is a substrate milestone, not a packet-promotion milestone.

## Design Basis

This plan follows the approved design doc at:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260509-163237.md`

The design doc locks four decisions:

1. The helper wedge is a real repo-visible surface.
2. The blocker is reviewer capability, not missing corpus.
3. The right fix is runtime-supported substrate truth, not the next promoted packet.
4. The complete version includes read-side truth refresh, not just a classifier tweak.

## Live Repo Basis

Current repo truth:

- `examples/shared-spec/units/money/round.spec.passport.json` still records `compatibility_key = "unsupported.function.v1"`.
- `examples/ecommerce/units/money/round.unit.spec` and `examples/shared-spec/units/money/round.unit.spec` are the live helper examples.
- `xtask/src/family/analysis_core/helper_surface.rs` freezes the current helper wedge fingerprint:
  - `function_dep_arity = 0`
  - `contract_input_count = 1`
  - `has_return = true`
  - `authored_body_kind = "neither"`
- `spec-core/src/semantic_review.rs` currently exposes four supported function routes and no helper route.
- `xtask/src/family/inventory.rs` and `xtask/src/family/coverage.rs` already have supported-unpromoted plumbing.
- `cargo xtask family recommend --format json` currently reports the helper cluster as `helper_surface_not_promotable`.

That is enough evidence. More corpus does not help. Better reviewer understanding does.

## Scope Challenge

### What already exists

| Sub-problem | Existing owner | M41 decision |
|---|---|---|
| runtime supported function routing | `spec-core/src/semantic_review.rs` | extend the existing route table, do not add a second evaluator |
| helper wedge fingerprinting | `xtask/src/family/analysis_core/helper_surface.rs` | reuse as the retirement proof source, do not widen it into policy |
| supported-unpromoted coverage bucket | `xtask/src/family/coverage.rs` | reuse directly, do not invent a new coverage category |
| runtime inventory publication | `xtask/src/family/inventory.rs` | add one explicit metadata entry, do not fake a packet |
| recommendation hold logic | `xtask/src/family/recommend.rs` plus `xtask/src/lib.rs` tests | retire helper pressure by reclassification, not by patching around it |
| read-side projection behavior | existing passport/status/export behavior plus `spec-cli/tests/cli.rs` | default expectation is test-only proof refresh, not new CLI behavior |
| canonical helper examples | `examples/ecommerce/units/money/round.unit.spec`, `examples/shared-spec/units/money/round.unit.spec` | anchor the route on real examples, not synthetic-only fixtures |

### Minimum complete change

M41 is complete only if all of this lands together:

1. `spec-core/src/semantic_review.rs` gains one explicit supported helper route for the current zero-dep, one-input, one-return identity/passthrough helper shape.
2. The new route participates in the existing supported verdict ladder:
   - aligned
   - under specified
   - semantic drift
   - unsupported near miss remains unsupported
3. `xtask` inventory, coverage, and recommendation outputs treat the helper wedge as supported-unpromoted substrate truth instead of unsupported pressure.
4. Checked-in read surfaces prove the new truth:
   - canonical helper passport
   - status/export projection coverage
   - xtask regression coverage
5. The milestone does not invent a promotion story:
   - no new `semantic-families/function.*` packet
   - no `family new/smoke/prove/certify` loop
   - no broader helper taxonomy

### Concrete blast radius

M41 should stay inside these concrete files unless proof exposes a real bug:

| Area | Expected files |
|---|---|
| runtime reviewer | `spec-core/src/semantic_review.rs` |
| read-side proof | `spec-cli/tests/cli.rs`, `examples/shared-spec/units/money/round.spec.passport.json` |
| operator truth | `xtask/src/family/inventory.rs`, `xtask/src/family/coverage.rs`, `xtask/src/lib.rs` |
| docs and teaching surfaces | `semantic-families/README.md`, `CHANGELOG.md` |

That is 8 concrete files across 4 ownership zones.

Default expectation: no production `spec-cli/src/*.rs` changes. If read-side tests expose a real projection bug, stop and amend the plan instead of silently widening scope.

### Stop conditions

Stop and rewrite the plan if implementation tries to do any of this:

- add a new packet under `semantic-families/`
- introduce a second helper route
- introduce a generic helper taxonomy
- add new CLI flags or schema versions
- touch corpus manifests or decision-kernel policy instead of retiring the wedge through runtime support
- widen the route beyond the current decimal helper shape without new design approval

### TODO cross-reference

Relevant existing TODOs:

- `Cross-crate family-analysis shared core`
- `Generalized multi-wedge decision layer`

Neither blocks M41. Both stay deferred. M41 should make both smaller by removing fake unsupported pressure from the helper lane.

### Completeness check

The tempting shortcut is a `spec-core`-only route addition.

Reject that.

If runtime truth changes but status/export/passport and inventory/coverage/recommendation continue to report unsupported helper pressure, the product becomes less trustworthy. The full blast radius is still a small lake, and this repo is already set up to carry the complete version.

## Architecture Contract

### Locked target boundary

```text
authored helper unit (.unit.spec)
            |
            v
spec-core/src/semantic_review.rs
  SupportedFunctionRoute::HelperIdentityPassthrough
            |
            +--> supported SemanticReview verdicts
            |
            +--> spec-cli passport/status/export projection truth
            |
            +--> xtask inventory runtime_supported_routes
            |
            +--> xtask family coverage supported_unpromoted counts
            |
            `--> recommendation no longer sees helper wedge as unsupported pressure
```

### Route contract

Add one new runtime-supported function route:

- route marker: `HelperIdentityPassthrough`
- compatibility key: `function.helper.identity_passthrough.v1`

The route must remain brutally narrow:

- zero deps
- exactly one input
- input type is `Decimal`
- return type is `Decimal`
- authored semantics are identity/passthrough helper intent
- executable body is consistent with passthrough semantics

Anything with control flow, additional deps, extra inputs, or broader helper semantics stays unsupported.

### Routing-order decision

Add the route at the end of `SUPPORTED_FUNCTION_ROUTING_ORDER`, immediately before terminal unsupported fallback.

Why:

- lowest shadow risk
- minimal churn to existing route precedence
- the shape cannot steal wrapper or arithmetic routes because it is zero-dep and one-input

### File ownership map

| File | Owns after M41 | Must not own |
|---|---|---|
| `spec-core/src/semantic_review.rs` | route recognition, compatibility key, verdict ladder, routing order | packet promotion logic, inventory policy |
| `spec-cli/tests/cli.rs` | preserve/refresh/status/export assertions for supported helper truth | semantic-review classifier logic |
| `examples/shared-spec/units/money/round.spec.passport.json` | canonical checked-in proof surface | new policy or new schema |
| `xtask/src/family/inventory.rs` | runtime inventory metadata for the helper route | fake packet scaffolding |
| `xtask/src/family/coverage.rs` | supported-unpromoted counts and helper-cluster retirement truth | new coverage classes |
| `xtask/src/lib.rs` | integration expectations for inventory/coverage/recommendation | runtime classifier logic |
| `semantic-families/README.md` | operator-facing explanation of supported-unpromoted helper truth | packet-promotion guidance for M41 |
| `CHANGELOG.md` | release teaching surface | design rationale beyond shipped outcome |

## Implementation Plan

### Step 1: Add the runtime route in `spec-core`

Files:

- `spec-core/src/semantic_review.rs`

Do:

- add `HelperIdentityPassthrough` to the supported route enum and routing order
- add the compatibility key constant
- implement the exact route-match predicate for the current helper shape
- prove all four verdict states for the helper route:
  - aligned
  - under specified
  - semantic drift
  - unsupported near miss

Do not:

- create a helper mini-framework
- widen the matcher beyond the current decimal helper wedge
- reorder existing wrapper/arithmetic routes

Done when:

- helper-aligned examples return supported helper truth
- near misses stay on `unsupported.function.v1`

### Step 2: Refresh read-side proof surfaces

Files:

- `spec-cli/tests/cli.rs`
- `examples/shared-spec/units/money/round.spec.passport.json`

Do:

- add regression coverage proving fresh helper proof survives preserve/status/export projection
- refresh the checked-in helper passport to the new compatibility key and supported review truth

Do not:

- widen CLI behavior casually
- change production `spec-cli/src/*.rs` unless tests expose a real bug

Done when:

- a fresh helper passport projects supported helper truth consistently across read surfaces
- stale or mismatched proof still demotes exactly the way existing supported routes do

### Step 3: Refresh operator truth in `xtask`

Files:

- `xtask/src/family/inventory.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/lib.rs`

Do:

- publish the helper route as runtime-supported and supported-unpromoted
- add inventory metadata for the helper route
- update coverage and recommendation expectations so the helper cluster retires by reclassification
- keep promoted-family counts unchanged

Do not:

- invent a new inventory category
- patch recommendation with a one-off exception
- fake a promoted family for the helper route

Done when:

- helper units move from unsupported pressure to supported-unpromoted truth
- recommendation stops surfacing `helper_surface_not_promotable` as live pressure for this wedge

### Step 4: Refresh repo teaching surfaces

Files:

- `semantic-families/README.md`
- `CHANGELOG.md`

Do:

- document that the helper wedge is now supported substrate truth
- document that M41 does not create a new promoted packet

Do not:

- rewrite family-promotion guidance broadly
- imply generic helper understanding

Done when:

- a maintainer can read the repo docs and understand why the helper wedge is now supported but still unpromoted

## Test Review

### Architecture diagram

```text
money/round.unit.spec
      |
      v
evaluate_semantic_review()
      |
      +--> supported helper verdicts in spec-core tests
      |
      +--> passport/status/export projection in spec-cli tests
      |
      +--> runtime_supported_routes inventory in xtask tests
      |
      +--> coverage supported_unpromoted counts in xtask tests
      |
      `--> recommendation no longer reports helper unsupported pressure
```

### Code-path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] spec-core helper route
    |
    ├── [GAP] aligned helper review
    ├── [GAP] vague helper intent -> under specified
    ├── [GAP] contradictory helper body -> semantic drift
    └── [GAP] control-flow near miss -> unsupported

[+] spec-cli read-side projection
    |
    ├── [GAP] preserve fresh helper passport truth
    ├── [GAP] status projects supported helper review
    └── [GAP] export projects supported helper review

[+] xtask operator truth
    |
    ├── [GAP] inventory lists helper route as supported-unpromoted
    ├── [GAP] coverage moves helper units out of unsupported pressure
    └── [GAP] recommendation stops surfacing helper hold text

[+] docs / teaching surfaces
    |
    ├── [GAP] semantic-families README explains supported-unpromoted helper truth
    └── [GAP] changelog records the user-visible semantic review shift
```

### Required proof loop

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-cli --test cli
cargo test -p xtask
cargo xtask family inventory --format json
cargo xtask family coverage --format json
cargo xtask family recommend --format json
```

### Expected post-M41 truth

- `money/round`-style helper units return supported helper semantic review truth
- helper route appears in `runtime_supported_routes`
- helper route appears in `supported_unpromoted_families`
- helper units stop contributing to unsupported helper-cluster pressure
- no new packet is introduced

Coverage expectation:

- current snapshot in repo tests is `28 / 17 / 0 / 11`
- expected post-M41 snapshot is `28 / 17 / 3 / 8`

If the corpus shifts before landing, preserve the same semantic delta instead of forcing stale absolute numbers:

- `promoted_family_units` unchanged
- `supported_unpromoted_family_units` increases by the helper-unit count being retired
- `unsupported_function_units` decreases by the same amount

### Test plan artifact

Authoritative QA handoff artifact:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260509-163237.md`

## Failure Modes Registry

| Surface | Failure | Test required | Error handling exists | User-visible impact | Severity |
|---|---|---|---|---|---|
| `semantic_review.rs` | route overmatches and falsely supports arbitrary helpers | yes | no runtime guard beyond matcher | silent false confidence | high |
| `semantic_review.rs` | route undermatches and leaves `money/round` unsupported | yes | fallback remains unsupported | obvious product lie remains | high |
| `spec-cli` read surfaces | supported helper proof is dropped or misprojected | yes | existing stale logic should apply | user sees inconsistent truth across commands | high |
| `xtask` inventory | runtime route exists but inventory omits it | yes | metadata validation exists | maintainer sees stale operator truth | medium |
| `xtask` coverage | helper units remain in unsupported pressure | yes | none beyond test expectations | recommendation continues to teach the wrong lesson | high |
| docs | repo still describes helper wedge as unsupported pressure | yes | none | human readers cargo-cult the old story | medium |

Critical gap rule:

Any path that has no test, no guardrail, and would silently over-claim semantic support is a release blocker for M41.

## Error & Rescue Registry

| ID | Gap | Rescue |
|---|---|---|
| ER-1 | helper route overmatches | tighten matcher and keep explicit near-miss tests |
| ER-2 | helper route undermatches | anchor tests and checked-in passport on canonical helper examples |
| ER-3 | runtime truth diverges from operator truth | land inventory and coverage refresh in the same milestone |
| ER-4 | preserve/export drops helper truth | add explicit read-side regression coverage before changing docs |
| ER-5 | recommendation still surfaces retired pressure | update xtask integration expectations instead of policy prose |

## Worktree Parallelization Strategy

This plan has real parallelization value because the work splits cleanly across module boundaries once the route contract is fixed.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| 1. Runtime helper route | `spec-core/src/` | — |
| 2. Read-side proof refresh | `spec-cli/tests/`, `examples/shared-spec/units/money/` | 1 |
| 3. Operator truth refresh | `xtask/src/family/`, `xtask/src/` | 1 |
| 4. Docs and release notes | `semantic-families/`, repo root docs | 2, 3 |

### Parallel lanes

- Lane A: Step 1, runtime helper route in `spec-core/src/`
- Lane B: Step 2, read-side proof refresh in `spec-cli/tests/` and `examples/shared-spec/units/money/`, starts after Lane A defines the compatibility key and matcher contract
- Lane C: Step 3, operator truth refresh in `xtask/src/family/` and `xtask/src/`, starts after Lane A defines the compatibility key and route marker
- Lane D: Step 4, docs in `semantic-families/` and repo root docs, starts after Lanes B and C settle the final user-visible truth

### Execution order

1. Launch Lane A first. It is the contract-setting lane.
2. After Lane A lands or reaches a stable patch, launch Lane B and Lane C in parallel worktrees.
3. Merge Lane B and Lane C.
4. Run Lane D last so docs describe the actual landed truth, not the intended truth.

### Conflict flags

- Lanes B and C should not touch the same modules. Keep the helper passport update in Lane B only.
- Lane C owns `xtask` expectations. Lane B should not patch `xtask` fixtures or counts.
- Lane D should not start early. Otherwise it will race the final route name, counts, or recommendation wording.

If the implementation ends up touching production `spec-cli/src/*.rs`, collapse Lane B back into sequential execution after Lane A. That is a scope-change signal, not a free parallel win.

## NOT in scope

- promoting a helper semantic-family packet
- generic helper understanding
- additional helper routes
- non-decimal helper types
- corpus expansion or manifest changes
- new CLI flags or output schemas
- cross-crate shared-core extraction
- second-language backend execution work

## Dream State Delta

After M41, the repo is not done with helper semantics. It is done with one helper lie.

What improves:

- helper pressure stops polluting the unsupported-function lane
- maintainers stop reading architecture-follow-on policy as a substitute for missing runtime support
- developers see truthful helper support directly in runtime, status, export, passport, inventory, and coverage surfaces

What stays deferred:

- broader helper semantics
- cross-crate shared-core reuse
- multi-wedge decision-kernel generalization
- second-language expansion

## Cross-Phase Theme

Retire the lie completely.

This milestone only wins if the helper wedge is removed from every surface that claims to tell the truth. Runtime-only support is not a smaller correct version. It is a split-brain version.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | Scope | Make M41 a substrate milestone, not a promotion milestone | mechanical | explicit over clever | frozen analysis already says helper surface is real but not promotable | packet promotion |
| 2 | Scope | Carry read-side truth in the same milestone | mechanical | choose completeness | runtime-only support would make the repo less trustworthy | `spec-core`-only fix |
| 3 | Architecture | Keep the route limited to one decimal identity/passthrough helper shape | mechanical | pragmatic | retires the visible wedge without creating a helper taxonomy | generic helper support |
| 4 | Architecture | Append the route at the end of supported routing order | taste | minimal diff | lowest shadow risk and least routing churn | inserting earlier in route order |
| 5 | Tooling | Publish helper truth as supported-unpromoted | mechanical | DRY | existing plumbing already models runtime-supported but unpromoted routes | new inventory category |
| 6 | Testing | Require read-side and xtask regressions, not just runtime tests | mechanical | boil the lake | status/export/passport/inventory/coverage are the user-facing truth surfaces | skipping blast-radius proof |
| 7 | Execution | Split post-route work into read-side and xtask lanes | taste | systems over heroes | parallelism is safe only after the route contract is fixed | fully sequential implementation |

## Completion Summary

| Review surface | Result |
|---|---|
| Step 0: Scope challenge | scope accepted, no reduction needed |
| Architecture review | one new route, no new framework |
| Code quality review | no new abstraction layer, explicit naming, minimal diff |
| Test review | full coverage diagram written, runtime + read-side + xtask proof required |
| Performance review | negligible runtime risk, no extra passes allowed |
| What already exists | written |
| NOT in scope | written |
| Failure modes | written |
| Error & rescue | written |
| Parallelization | 4 steps, 4 lanes, 2 parallel post-route lanes |
| Test plan artifact | linked |
| Lake score | chose the complete option over the shortcut |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | drafted | helper wedge is a substrate capability gap, not a packet-promotion lane |
| Codex Review | `codex review` | Independent 2nd opinion | 0 | unavailable | session policy did not allow delegated outside-voice review |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | drafted | complete blast radius is runtime support plus read-side and xtask truth refresh |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | no user-facing UI scope |
| DX Review | `/plan-devex-review` | Developer-facing truth surfaces | 1 | drafted | helper truth must change everywhere developers actually read it |

**VERDICT:** READY TO IMPLEMENT. M41 is now a single cohesive implementation contract with explicit scope, proof, and worktree parallelization boundaries.
