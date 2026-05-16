# M62: Bounded Corpus Run 1 for the Unsupported Callable-Triple Wrapper Dep-Topology Candidate

Status: **authoritative implementation plan**
Milestone: **M62**
Milestone family: **bounded corpus expansion**
Implementation readiness: **ready for bounded execution**
Plan scope: **add one maintained real-example unit and two promotion-relevant M20 regression units for the unsupported callable-triple wrapper dep-topology candidate centered on `examples_crosslib_app::pricing/checkout_nested_chain3`, then rerun the family-analysis proof wall and accept the truthful next decision without widening backend capability, manifest semantics, or recommendation policy**
Base branch: **main**
Working branch: **feat/m60-plus**
Validated at commit: **`0518c7a`**
Last rewritten: **2026-05-16**

Supersedes:

- the shipped M61 authority plan previously maintained at this path
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260516-131722.md`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260516-131722.md`
- `README.md`
- `TODOS.md`
- `CHANGELOG.md`
- `examples/crosslib-app/README.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

Primary repo surfaces:

- `semantic-families/corpus/rust-function.toml`
- `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`
- `examples/shared-spec/units/pricing/base_nested_chain3.unit.spec`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_total_bad_dep_topology.unit.spec`
- `spec-cli/tests/cli.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/paths.rs`
- `xtask/src/lib.rs`

## Executive Summary

M61 already shipped the bounded recursive local-plus-cross-library TypeScript lane.

That is not the open problem anymore.

The open problem is that the family-analysis surfaces still rank the same
authored pressure shape around `pricing/checkout_nested_chain3` as a top
unsupported dep-topology candidate with thin evidence:

- `real_example_hits = 1`
- `promotion_relevant_regression_hits = 0`
- hold reasons:
  - `hard_difficulty`
  - `thin_real_example_support`
  - `thin_regression_support`
- decision action:
  - `spend_corpus_run1`

M62 spends exactly that run.

It does not widen TypeScript execution. It does not promote a new family. It
does not rewrite recommendation policy. It does not add a sixth corpus source.

It does one bounded thing:

```text
add the smallest exact set of maintained real-example and regression units
needed to move the callable-triple wrapper candidate off the current
1-real / 0-regression floor, then rerun coverage, recommendation,
artifact validation, and corpus-decision and keep whatever truthful answer
the updated read-side surfaces produce
```

Under the current recommender thresholds, if the three new units land in the
target cluster exactly as intended, the post-run basis should move to:

- `real_example_hits = 2`
- `promotion_relevant_regression_hits = 2`
- no remaining `hard_difficulty`, `thin_real_example_support`, or
  `thin_regression_support` holds
- `confidence.level = "medium"`
- `decision_status = "recommended"`
- `decision_action = "pivot_to_family_promotion_run"`

That is the expected happy-path contract for M62 on today's code, not a vague
"looks better" aspiration.

## Frozen Implementation Decisions

These decisions are locked for M62. If any of them changes, the milestone scope
changed and this plan must be rewritten before implementation continues.

1. **This is a corpus-and-analysis milestone, not a backend milestone.**
   - Do not touch `spec-core/src/semantic_review.rs`, TypeScript backend
     contracts, validator semantics, or CLI target-language behavior unless the
     proof wall exposes a release-blocking defect in an existing read-side
     command path.

2. **Keep the corpus manifest frozen.**
   - Reuse the existing five sources in `semantic-families/corpus/rust-function.toml`.
   - Do not add a new source bucket, packet-fixture leverage source, or scratch
     analysis path.

3. **Add exactly one maintained real-example unit.**
   - The only new maintained real-example file is:
     - `examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`
   - Do not add a second maintained example root in this milestone.

4. **Add exactly two promotion-relevant regression units.**
   - The only new M20 regression files are:
     - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/base_nested_chain3_bad_dep_topology.unit.spec`
     - `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_nested_chain3_bad_dep_topology.unit.spec`

5. **Mirror the target unsupported pressure shape exactly.**
   - All three new units must stay in the same callable-triple wrapper
     neighborhood as `pricing/checkout_nested_chain3`:
     - dep arity `3`
     - callable dep topology class `unsupported_callable_triple`
     - contract input count `5`
     - return-bearing wrapper-like body
   - Do not spend this run on a nearby but different unsupported shape.

6. **Treat analysis artifacts as derived output only.**
   - `coverage.latest.json`, `recommendation.latest.json`, and
     `corpus-program-decision.latest.json` are refreshed outputs, not hand-edited
     inputs.
   - `examples/shared-crate/src/generated/**` and
     `examples/crosslib-app/src/generated/**` are derived proof surfaces for the
     maintained cross-library example, not hand-authored source.

7. **Keep docs edits minimal and truth-maintaining only.**
   - If existing docs remain accurate after the corpus additions, leave them
     alone.
   - If the maintained example set or proof commands become misleading, update
     only the minimum affected docs in the same PR.

## Current Validated Basis

Validated on `feat/m60-plus` at `0518c7a`.

Observed live branch truth:

- `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`
  already passes:
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`
  - `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec --target-language typescript`
- `semantic-families/corpus/rust-function.toml` already includes exactly these
  recommendation-counting sources:
  - `examples_ecommerce`
  - `m19_semantic_falsification_pack`
  - `m20_unsupported_truth_pack`
  - `examples_shared_spec`
  - `examples_crosslib_app`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
  currently reports:
  - `cluster_id = "unsupported_dep_topology-fbecce0dbe98"`
  - `overlap_family = "function.wrapper.pipeline*"`
  - `real_example_hits = 1`
  - `promotion_relevant_regression_hits = 0`
  - `boundary_only_hits = 0`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  currently reports:
  - `recommendation_status = "no_strong_candidate"`
  - `decision_status = "blocked_for_now"`
  - `top_candidate_id = "a-unsupporteddeptopology-unsupported_dep_topology-fbecce0dbe98"`
  - hold reasons:
    - `hard_difficulty`
    - `thin_real_example_support`
    - `thin_regression_support`
- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
  currently records:
  - `decision_action = "spend_corpus_run1"`
  - `decision_basis_code = "plausible_candidate_missing_evidence"`
  - `required_next_action = "author_corpus_expansion_plan"`

The repo is therefore in a truthful but thin state: the product already executes
the maintained example shape, but the recommendation surface still cannot tell
whether that shape is genuinely not ready or merely under-evidenced.

## Step 0: Scope Challenge

### Premise correction

The problem is not "TypeScript still needs more support."

The real problem is smaller:

```text
the repo already ships and proves the checkout_nested_chain3 execution shape,
but the recommendation surfaces still only see one maintained real example
and zero promotion-relevant regressions for that same callable-triple wrapper
pressure
```

If M62 expands beyond that sentence, it is overbuilt.

### What already exists

| Sub-problem | Existing owner | M62 action |
| --- | --- | --- |
| maintained real example for the target cluster | `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec` | add one second maintained unit in the same source bucket and same pressure class |
| supported nested shared callable in slot 1 | `examples/shared-spec/units/pricing/base_nested_chain3.unit.spec` | reuse as-is, do not edit shared-spec source |
| local unsupported wrapper baseline in M20 | `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_total_bad_dep_topology.unit.spec` | extend the same pack with two nested callable-triple variants |
| corpus source inventory | `semantic-families/corpus/rust-function.toml` | reuse unchanged |
| read-side coverage output | `cargo xtask family coverage --format json` | rerun and compare leverage counts |
| read-side recommendation output | `cargo xtask family recommend --format json` | rerun and compare blocker simplification |
| bounded next-step decision output | `cargo xtask family corpus-decision --format json` | rerun and require a truthful next action |

### Minimum complete slice

The minimum honest M62 slice is:

1. add one maintained `examples_crosslib_app` unit that lands in the same target
   unsupported cluster as `pricing/checkout_nested_chain3`
2. add two promotion-relevant nested bad-topology units to the existing
   `m20_unsupported_truth_pack`
3. validate the exact new units and re-run the existing M20 pack truth loop
4. rerun `coverage`, `recommend`, artifact validation, and `corpus-decision`
5. capture the pre/post delta and update docs only if the maintained example or
   proof commands became misleading

Anything smaller is fake done.

Examples:

- adding the real-example variant without regression pressure is fake done
- adding only one regression unit keeps the candidate on a suspiciously thin
  evidence story
- rerunning `coverage` and `recommend` without `corpus-decision` is fake done
- refreshing artifacts without checking whether the blocker list actually got
  simpler is fake done

### Complexity and blast radius

This milestone is small enough to stay boring:

- 3 new source-spec files
- 1 existing CLI truth-surface file updated on purpose
- 0 new source buckets
- 0 backend capability changes
- 0 policy rewrites
- 0 new classes, services, or abstractions

Likely touched authored files:

- `examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/base_nested_chain3_bad_dep_topology.unit.spec`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_nested_chain3_bad_dep_topology.unit.spec`
- `spec-cli/tests/cli.rs`
- possibly `examples/crosslib-app/README.md`, `CHANGELOG.md`, and `TODOS.md` if
  the proof wall changes public truth

That is a minimal diff for the user outcome we want: better decision quality on
an already product-relevant shape.

### Search check

No framework built-in replaces this work. The repo already owns the corpus and
analysis surfaces.

- **[Layer 1]** Reuse the current five-source manifest in `semantic-families/corpus/rust-function.toml`
- **[Layer 1]** Reuse the existing maintained cross-library root
  `pricing/checkout_nested_chain3`
- **[Layer 1]** Reuse the existing M20 pricing pack naming, field ordering, and
  unsupported-truth authoring style
- **[Layer 1]** Reuse the current `cargo xtask family coverage/recommend/corpus-decision`
  commands and artifact paths
- **[Layer 3]** Do not invent a new recommendation corpus source just to make
  one candidate look stronger

### TODOS cross-reference

`TODOS.md` already tracks the remaining TypeScript oceans after M61. M62 does
not change that backlog.

What M62 may add, but only if the post-run truth demands it:

- a family-focused follow-up if thin-evidence blockers clear and only the
  harder family decision remains
- a read-side recommendation follow-up if the evidence spend fails to improve
  decision quality at all

Do not silently grow TODOs before the new analysis basis exists.

### Completeness and distribution check

No new distributable artifact is introduced.

The complete version here is not packaging work. It is truth-completeness:

- exact corpus additions
- exact proof wall
- exact delta capture
- exact next-step decision

AI makes the full read-side rerun cheap. Do the whole thing.

## Milestone Contract

### Exact shipped behavior

After M62 lands, the repo should be able to say this and nothing broader:

```text
the callable-triple wrapper dep-topology candidate around
examples_crosslib_app::pricing/checkout_nested_chain3 is no longer judged
from a 1-real / 0-regression evidence floor; the family-analysis surfaces
now reflect one bounded evidence spend and return a clearer next step
```

This is a recommendation-quality claim, not a backend-capability claim.

### Exact source eligibility

M62 may use only the current recommendation-counting sources:

- `examples_ecommerce`
- `m19_semantic_falsification_pack`
- `m20_unsupported_truth_pack`
- `examples_shared_spec`
- `examples_crosslib_app`

It may not add a sixth source.

### Exact authored additions

#### Maintained real-example addition

Add exactly:

- `examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`

Frozen authored contract:

- `id = pricing/checkout_nested_chain3_variant`
- same 5-input `Decimal` contract as `pricing/checkout_nested_chain3`
- same three-dep callable tuple as `pricing/checkout_nested_chain3`:
  - `shared::pricing/base_nested_chain3`
  - `shared::pricing/apply_tax`
  - `shared::pricing/apply_discount`
- same wrapper-like three-stage body shape:
  - call shared nested chain3 first
  - apply one outer surcharge through `apply_tax`
  - apply one outer loyalty discount through `apply_discount`
- include both `body.rust` and `body.typescript`
- local test fixture should use the already-proven aligned chain3 numbers to
  avoid arithmetic ambiguity:
  - `checkout_nested_chain3_variant(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(970290, 4)`

No shared-spec source edits are part of this step.

#### Promotion-relevant regression additions

Add exactly:

- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/base_nested_chain3_bad_dep_topology.unit.spec`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_nested_chain3_bad_dep_topology.unit.spec`

Frozen authored contract for `base_nested_chain3_bad_dep_topology`:

- `id = pricing/base_nested_chain3_bad_dep_topology`
- inputs:
  - `subtotal`
  - `discount_rate`
  - `tax_rate`
  - `surcharge_rate`
  - `loyalty_rate`
- deps:
  - `pricing/checkout_total`
  - `pricing/apply_tax`
  - `pricing/apply_discount`
- wrapper-like body:
  - compute `checkout_total(subtotal, discount_rate, tax_rate)`
  - apply outer surcharge through `apply_tax`
  - apply loyalty discount through `apply_discount`
- local test fixture:
  - `base_nested_chain3_bad_dep_topology(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9801, 2)`

Frozen authored contract for `checkout_nested_chain3_bad_dep_topology`:

- `id = pricing/checkout_nested_chain3_bad_dep_topology`
- same 5-input contract
- deps:
  - `pricing/base_nested_chain3_bad_dep_topology`
  - `pricing/apply_tax`
  - `pricing/apply_discount`
- wrapper-like body:
  - call `base_nested_chain3_bad_dep_topology(...)`
  - apply outer surcharge through `apply_tax`
  - apply loyalty discount through `apply_discount`
- local test fixture:
  - `checkout_nested_chain3_bad_dep_topology(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(970290, 4)`

These two units exist to give the target cluster a real regression story, not a
single accidental-looking point.

### Exact preserved boundaries

M62 must not:

- change `semantic-families/corpus/rust-function.toml` semantics
- widen the supported-family vocabulary
- change TypeScript validator or backend rules
- add units under `examples/ecommerce/units`
- add units under `semantic-families/**` as recommendation leverage
- reopen arbitrary authored 4+ topology parity
- reopen molecule TypeScript execution
- reopen seam-kind TypeScript execution
- treat packet fixtures as corpus leverage
- turn this run into a policy rewrite

### Exact expected output delta

The post-run analysis basis is acceptable only if it becomes mechanically more
informative than the current floor and matches the current recommender logic.

Required coverage delta:

- the target cluster still exists as `unsupported_dep_topology-fbecce0dbe98` or
  a truthfully more precise successor with the same shape fingerprint:
  - `function_dep_arity = 3`
  - `callable_dep_topology_class = unsupported_callable_triple`
  - `contract_input_count = 5`
  - `authored_body_kind = wrapper_like`
- `real_example_hits = 2`
- `promotion_relevant_regression_hits = 2`
- `boundary_only_hits = 0`
- `source_ids` for the cluster include both:
  - `examples_crosslib_app`
  - `m20_unsupported_truth_pack`

Required recommendation delta on the current code path:

- the top candidate remains the same target candidate or a truthfully renamed
  direct successor for the same cluster
- `hold_reasons = []`
- `recommendation_status = "ranked"`
- `decision_summary.decision_status = "recommended"`
- `confidence.level = "medium"` or stronger

Required corpus-program decision delta on the current code path:

- `decision_action = "pivot_to_family_promotion_run"`
- `decision_basis_code = "promotion_ready_candidate"`
- `required_next_action = "author_family_promotion_plan"`

What does not count:

- the same candidate with the same `hard_difficulty`,
  `thin_real_example_support`, and `thin_regression_support` blocker trio
- `real_example_hits = 2` paired with `promotion_relevant_regression_hits < 2`
  because that means one or both M20 additions did not land as intended
- larger raw unsupported totals with no recommendation-state change
- a refreshed artifact set that still says `decision_action = "spend_corpus_run1"`
- any closeout that cannot explain which new unit missed the target cluster when
  the expected counts do not materialize

### Exact post-run decision matrix

The closeout must classify the result into exactly one bucket:

1. **Expected green path**
   - coverage shows `2 real / 2 regression`
   - recommendation becomes `ranked` + `recommended`
   - corpus decision becomes `pivot_to_family_promotion_run`
   - outcome: M62 succeeded and the next plan, if needed, is a bounded family
     promotion plan

2. **Yellow but acceptable diagnosis**
   - coverage shows the expected `2 real / 2 regression`
   - missing-evidence blockers clear
   - but the corpus decision pivots to `recommendation_policy_run` instead of a
     promotion run
   - outcome: M62 still succeeded as a corpus run, but the follow-up is policy,
     not more corpus spend

3. **Red, do not close**
   - coverage fails to reach `2 real / 2 regression`
   - or the recommendation still reports missing evidence
   - or `decision_action` stays `spend_corpus_run1`
   - outcome: at least one new unit did not count the way the plan expected, or
     the read-side logic regressed; inspect cluster membership before closeout

4. **Unexpected stop path**
   - `decision_action = "stop"`
   - outcome: only acceptable if the target candidate genuinely disappears from
     the actionable set and the closeout proves why; otherwise treat as red

## Architecture Review

### Dependency graph

```text
CURRENT CORPUS / ANALYSIS FLOW
==============================

examples_crosslib_app
  └── pricing/checkout_nested_chain3                [existing real example]
        ├── shared::pricing/base_nested_chain3      [existing supported nested chain3]
        ├── shared::pricing/apply_tax               [existing shared leaf]
        └── shared::pricing/apply_discount          [existing shared leaf]

m20_unsupported_truth_pack
  └── pricing/checkout_total_bad_dep_topology       [existing 2-dep unsupported regression]

semantic-families/corpus/rust-function.toml         [frozen 5-source manifest]
  ├── cargo xtask family coverage --format json
  ├── cargo xtask family recommend --format json
  └── cargo xtask family corpus-decision --format json

CURRENT RESULT
  └── unsupported_dep_topology-fbecce0dbe98
        = 1 real example / 0 promotion-relevant regressions
```

```text
M62 TARGET FLOW
===============

examples_crosslib_app
  ├── pricing/checkout_nested_chain3                [existing]
  └── pricing/checkout_nested_chain3_variant        [new]
        ├── shared::pricing/base_nested_chain3
        ├── shared::pricing/apply_tax
        └── shared::pricing/apply_discount

m20_unsupported_truth_pack
  ├── pricing/checkout_total_bad_dep_topology       [existing]
  ├── pricing/base_nested_chain3_bad_dep_topology   [new]
  │     ├── pricing/checkout_total
  │     ├── pricing/apply_tax
  │     └── pricing/apply_discount
  └── pricing/checkout_nested_chain3_bad_dep_topology [new]
        ├── pricing/base_nested_chain3_bad_dep_topology
        ├── pricing/apply_tax
        └── pricing/apply_discount

ANALYSIS
  ├── coverage.latest.json                          [refresh]
  ├── recommendation.latest.json                    [refresh]
  └── corpus-program-decision.latest.json           [refresh]

TARGET RESULT
  └── same candidate, but no longer judged from a 1 real / 0 regression floor
```

### Current flaw

The current read-side architecture is not broken in the backend sense.

The flaw is evidence scarcity around a now-product-relevant unsupported shape.
One maintained example and zero promotion-relevant regressions is too thin to
trust either a promotion push or a durable hold.

### M62 target architecture

M62 should change corpus content, not repo architecture.

The stable architecture after M62 is:

- same five-source manifest
- same existing analysis commands
- richer target-cluster leverage
- clearer stop/spend/pivot decision

If implementation starts drifting into `xtask` logic changes before the new
corpus truth is observed, that is a smell. Stop and re-scope.

### File-by-file responsibilities

| Path | Responsibility |
| --- | --- |
| `examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec` | second maintained real-example hit for the target cluster |
| `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/base_nested_chain3_bad_dep_topology.unit.spec` | intermediate nested unsupported regression pressure |
| `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_nested_chain3_bad_dep_topology.unit.spec` | outer nested unsupported regression pressure matching the target cluster |
| `spec-cli/tests/cli.rs` | public truth-surface assertions for the new maintained example count and the expanded M20 unsupported matrix |
| `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json` | refreshed leverage counts and cluster membership |
| `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json` | refreshed blocker list and next-step status |
| `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json` | refreshed stop/spend/pivot decision |
| `examples/crosslib-app/README.md`, `CHANGELOG.md`, `TODOS.md` | minimal truth maintenance only if the new maintained example set or analysis loop would otherwise be misleading |

Read-only unless a proof-wall defect forces escalation:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/lib.rs`

## Code Quality Review

### Design choices

1. **Reuse existing shapes instead of inventing new ones.**
   - The maintained variant reuses the existing `checkout_nested_chain3` dep
     tuple.
   - The M20 additions reuse the current pricing pack vocabulary and extend it
     by one intermediate nested wrapper plus one outer nested wrapper.

2. **Keep the analysis story explicit.**
   - The unit additions exist to move exact leverage counters and blocker
     states, not to "generally improve corpus quality."

3. **Keep the diff small.**
   - This should be authored-spec work plus derived artifact refresh.
   - No generic helper extraction, manifest redesign, or policy abstraction.

### DRY and maintenance rules

- Keep field order aligned with existing pricing unit specs:
  - `id`, `kind`, `intent`, `spec_version`, `contract`, `deps`, `imports`,
    `body`, `local_tests`
- Reuse the same five-argument pricing contract names everywhere:
  - `subtotal`
  - `discount_rate`
  - `tax_rate`
  - `surcharge_rate`
  - `loyalty_rate`
- Reuse the same aligned fixture numbers for all three new nested units so the
  milestone does not burn time on arithmetic drift.
- Do not duplicate doc text across README, CHANGELOG, and TODOs unless the
  public truth actually changed on that surface.
- Do not hand-edit generated artifacts outside the standard command outputs.

## Implementation Plan

### Implementation lockstep

This milestone is simplest when done in this order:

1. author the maintained real-example variant
2. author the two M20 nested bad-topology regressions
3. update the CLI truth-surface assertions that hard-code the current example and
   M20 unsupported matrices
4. validate the exact new units and the focused CLI truth wall
5. rerun the family-analysis proof wall
6. update only the docs that became misleading
7. capture the pre/post delta and decide the truthful next milestone

### Step 1. Add the maintained cross-library variant

Create:

- `examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`

Requirements:

- keep the same contract, dep tuple, and wrapper-like shape as
  `checkout_nested_chain3.unit.spec`
- include `body.rust` and `body.typescript`
- use the aligned 10% fixture and expected `Decimal::new(970290, 4)` local test
- do not add or modify any `examples/shared-spec` source unit
- treat `examples/shared-crate/src/generated/**` and
  `examples/crosslib-app/src/generated/**` as derived proof surfaces and
  refresh them before the maintained Rust proof commands

Why this step exists:

- it is the smallest honest move that can push `real_example_hits` above `1`

### Step 2. Add the two M20 nested bad-topology regressions

Create:

- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/base_nested_chain3_bad_dep_topology.unit.spec`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_nested_chain3_bad_dep_topology.unit.spec`

Requirements:

- both units stay inside the existing M20 pack
- both units use the same five-input pricing contract
- the intermediate unit wraps `checkout_total` with outer surcharge and loyalty
  stages
- the outer unit wraps the intermediate unit with the same two outer stages
- use the aligned fixture outputs:
  - base: `Decimal::new(9801, 2)`
  - outer: `Decimal::new(970290, 4)`

Why this step exists:

- it is the smallest honest move that can push
  `promotion_relevant_regression_hits` above `0` without inventing a new source

### Step 3. Update the CLI truth surfaces before rerunning proof

Edit:

- `spec-cli/tests/cli.rs`

Required assertion updates:

- extend
  `m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix`
  so `unsupported_cases` also includes:
  - `pricing/base_nested_chain3_bad_dep_topology`
  - `pricing/checkout_nested_chain3_bad_dep_topology`
  - expected reason code for both: `unsupported_dep_topology`
- update `spec_status_repo_root_honors_each_root_workspace_config` so the copied
  `crosslib-app` root expects 5 units instead of 4 and explicitly includes:
  - `pricing/checkout_nested_chain3_variant`
  - status `untested`
  - no `SPEC_UNKNOWN_LIBRARY_NAMESPACE` noise

Why this step exists:

- broad green CLI output is not enough if the repo's public truth-surface tests
  never assert the two new M20 ids or the new maintained example row

### Step 4. Re-run the unit and pack proof wall

Run:

```bash
cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --format json
cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --target-language typescript

cargo run -p spec-cli -- validate spec-cli/tests/fixtures/m20/unsupported_truth_pack/units --format json
cargo test -p spec-cli --test cli m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix -- --exact
cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact
cargo test -p spec-cli --test cli
```

Requirements:

- the maintained variant validates and passes in both Rust and TypeScript
- the M20 pack stays green as a pack
- the public unsupported reason/status matrix in CLI tests explicitly covers the
  two new M20 ids
- the repo-root crosslib status test explicitly covers the new maintained
  example row

### Step 5. Re-run coverage, recommendation, and corpus-decision

Run:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Requirements:

- refreshed artifacts validate
- the target cluster reaches exactly `2 real / 2 regression`
- the recommendation and corpus-decision outputs match one of the allowed
  buckets in the decision matrix above

If `corpus-decision` still fails on a refreshed valid basis, stop and treat that
as a release-blocking read-side defect for this milestone.

### Step 6. Apply minimal truth-maintenance docs edits

Touch docs only if the proof wall makes an existing surface misleading.

Allowed doc edits:

- `examples/crosslib-app/README.md` if it should name the new maintained variant
  or updated proof command
- `CHANGELOG.md` for the M62 corpus-run closeout
- `TODOS.md` only if the refreshed analysis basis clearly creates the next
  follow-up

Default posture:

- leave `README.md` root-level M61 capability wording alone unless it becomes
  factually incomplete or misleading

### Step 7. Capture the post-run basis

The implementation closeout must preserve:

- pre-run and post-run leverage counts for the target cluster
- pre-run and post-run blocker lists
- pre-run and post-run `corpus-decision` action
- the exact next-action sentence after M62
- the exact cluster member ids counted after the run
- whether the result landed in the expected green path, acceptable diagnosis, or
  red path

Capture this as a small delta table in the implementation closeout and any
follow-on design artifact.

## Test Review

### Test framework and proof owners

This repo’s truth loop for M62 is command-driven, not UI-driven.

Proof owners:

- `spec validate --format json` for authored-spec truth
- `spec test` for Rust and TypeScript maintained-example proof
- focused `cargo test -p spec-cli --test cli ... -- --exact` runs plus a final
  whole-file `cargo test -p spec-cli --test cli` for the public unsupported and
  example-root truth surfaces
- `cargo xtask family * --format json` for read-side analysis and decision truth

No E2E browser work. No eval suite. No new backend unit-test surface unless the
proof wall exposes a defect in existing analysis logic.

### Code path coverage diagram

```text
CORPUS AUTHORING COVERAGE
=========================

[+] examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec
    ├── [MUST PROVE] validate --format json
    ├── [MUST PROVE] spec test (Rust)
    └── [MUST PROVE] spec test --target-language typescript

[+] spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/base_nested_chain3_bad_dep_topology.unit.spec
    ├── [MUST PROVE] whole-pack validate
    └── [MUST PROVE] retained unsupported truth surfaces through cli.rs

[+] spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/checkout_nested_chain3_bad_dep_topology.unit.spec
    ├── [MUST PROVE] whole-pack validate
    └── [MUST PROVE] retained unsupported truth surfaces through cli.rs

CLI TRUTH-SURFACE COVERAGE
==========================

[+] spec-cli/tests/cli.rs
    ├── [MUST PROVE] M20 whole-pack matrix asserts both new unsupported ids with reason `unsupported_dep_topology`
    ├── [MUST PROVE] repo-root crosslib status test asserts the new maintained example row
    └── [GAP = FAIL] broad CLI green run without new-id assertions

ANALYSIS SURFACE COVERAGE
=========================

[+] cargo xtask family coverage --format json
    ├── [MUST PROVE] target cluster remains visible or becomes a more precise successor
    ├── [MUST PROVE] real_example_hits = 2
    └── [MUST PROVE] promotion_relevant_regression_hits = 2

[+] cargo xtask family recommend --format json
    ├── [MUST PROVE] hold_reasons clear completely on the current code path
    ├── [MUST PROVE] confidence rises to `medium` or stronger
    └── [GAP = FAIL] same three-part blocker list after evidence spend

[+] cargo xtask family corpus-decision --format json
    ├── [MUST PROVE] refreshed decision artifact writes and validates
    ├── [MUST PROVE] green path becomes `pivot_to_family_promotion_run`
    └── [CRITICAL] command-path failure or `spend_corpus_run1` after updated valid basis

─────────────────────────────────
COVERAGE TARGET: 12/12 proof paths green
  Authored source + CLI truth paths: 8/8
  Analysis / decision paths: 4/4
QUALITY BAR: no silent artifact refresh, no vague "looks better" closeout
─────────────────────────────────
```

### Required tests

Required proof commands:

- `cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --format json`
- `cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated`
- `cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated`
- `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec`
- `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --target-language typescript`
- `cargo run -p spec-cli -- validate spec-cli/tests/fixtures/m20/unsupported_truth_pack/units --format json`
- `cargo test -p spec-cli --test cli m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix -- --exact`
- `cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact`
- `cargo test -p spec-cli --test cli`
- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `cargo xtask family corpus-decision --format json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- `jq '.unsupported_clusters[] | select(.cluster_id=="unsupported_dep_topology-fbecce0dbe98") | {cluster_id, representative_unit_ids, source_ids, real_example_hits, promotion_relevant_regression_hits, boundary_only_hits}' .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `jq '{recommendation_status, decision_summary, top_candidate:(.ranked_candidates[0] | {candidate_id, promotion_readiness, hold_reasons, confidence, leverage})}' .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `jq '{decision_action, decision_basis_code, required_next_action, summary}' .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

No new harness family is authorized up front, but existing harnesses must be
widened where they currently hard-code the old example or M20 matrices. If those
harnesses break or stay too loose to prove the new ids, that is a defect, not
optional follow-up coverage.

### Regression rule

This milestone is a recommendation-quality regression wall.

Required regressions:

- the new maintained variant must not accidentally classify outside the target
  unsupported callable-triple cluster
- the two new M20 units must count as promotion-relevant regression pressure,
  not boundary-only noise
- the refreshed recommendation basis must not silently keep the same
  `hard_difficulty`, `thin_real_example_support`, and
  `thin_regression_support` story after the run
- `corpus-decision` must keep working on the updated basis
- the repo's public CLI truth surfaces must explicitly name the three new rows
  they are supposed to prove, not just pass green by omission

If the evidence spend lands but the old blocker story survives unchanged,
consider M62 failed until the repo explains why.

## Failure Modes Registry

| New codepath | Real production failure | Test covers it? | Error handling exists? | User-visible effect | Priority |
| --- | --- | --- | --- | --- | --- |
| maintained variant authoring | new real-example unit classifies into the wrong cluster or a supported family | must add | partial via coverage output | maintainers think evidence improved when it did not | critical |
| intermediate M20 nested unit | `base_nested_chain3_bad_dep_topology` fails to count as promotion-relevant regression leverage | must add | no direct guard beyond analysis outputs | regression count rises incorrectly or not at all | critical |
| outer M20 nested unit | the outer nested bad-topology unit lands in a neighboring unsupported shape | must add | partial | blocker reasoning stays muddy | high |
| CLI truth-surface refresh | the repo-level tests stay green but never assert the new maintained example row or new M20 ids | must add | no | false confidence that public status/export truth stayed correct | critical |
| artifact refresh | coverage/recommendation artifacts refresh but `corpus-decision` still points at stale or incompatible basis | must add | partial via validate-artifact commands | operator loop breaks at the last step | critical |
| doc truth maintenance | docs imply M62 shipped a new backend capability instead of a better evidence basis | manual review | no | users over-assume product scope | medium |
| TODO follow-up capture | the refreshed basis clearly implies a next milestone but no TODO or closeout note records it | manual review | no | next planning cycle loses context | medium |

Critical gaps to avoid:

- any outcome where the target cluster still shows `1 real / 0 regression`
- any outcome where both thin-evidence blockers survive and no explanation is
  captured
- any outcome where `corpus-decision` is broken and the milestone tries to
  close anyway

## Performance Review

This milestone should be near-zero performance risk.

Expected characteristics:

- three additional corpus units
- the same five-source manifest
- the same analysis commands
- small constant-factor work increase in coverage/recommendation passes

Guardrails:

- do not widen source discovery
- do not add packet-fixture leverage
- do not add expensive new command loops or duplicate proof commands
- do not introduce code changes in `xtask` unless the proof wall proves a real defect

## NOT in scope

- any TypeScript backend widening
- any validator or semantic-review capability widening
- new supported-family promotion
- recommendation policy rewrites
- manifest source additions
- packet-fixture recommendation leverage
- new maintained real-example roots beyond `checkout_nested_chain3_variant`
- units under `examples/ecommerce/units`
- molecule TypeScript execution
- seam-kind TypeScript execution
- repo-wide docs rewrite beyond truth maintenance
- ORCH plan rewrite in the same change

## TODOS.md updates required in the same PR

No unconditional TODO addition is authorized up front.

If the post-run truth is decisive, do exactly one of these:

1. add a follow-up TODO for the next family-focused milestone if thin-evidence
   blockers cleared and the next question is now capability/promotion-specific
2. add a follow-up TODO for recommendation-surface investigation if the evidence
   spend fails to improve decision quality
3. add nothing if the implementation closeout itself already captures a clean,
   unambiguous next action and `TODOS.md` would only duplicate it

Do not add vague "investigate later" bullets.

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| 1. maintained real-example variant | `examples/crosslib-app/` | — |
| 2. M20 nested bad-topology regressions | `spec-cli/tests/fixtures/m20/unsupported_truth_pack/` | — |
| 3. CLI truth-surface assertion updates | `spec-cli/tests/` | 1, 2 |
| 4. proof wall rerun and artifact refresh | `.semantic-family-artifacts/`, command outputs | 1, 2, 3 |
| 5. minimal truth-maintenance docs | `examples/crosslib-app/`, repo-root docs | 4 |

### Parallel lanes

- **Lane A:** Step 1, `examples/crosslib-app/` maintained real-example authoring
- **Lane B:** Step 2, M20 nested bad-topology regression authoring
- **Lane C:** Step 3 then Step 4 then Step 5, sequential CLI assertion updates,
  proof-wall rerun, and any required docs cleanup after A + B converge

### Execution order

Launch **Lane A** and **Lane B** in parallel worktrees. They do not share
modules and can be authored independently.

After both land, run **Lane C** serially:

1. update `spec-cli/tests/cli.rs` to assert the new maintained-example and M20
   truth rows
2. rerun validate/test/analysis commands
3. inspect the refreshed artifacts against the exact decision matrix
4. update docs only if the outputs changed the user-facing truth

### Conflict flags

- **Lane C** must own `examples/crosslib-app/README.md` if it changes. Do not
  let Lane A edit the README before the proof wall says it is necessary.
- **Lane C** must own `spec-cli/tests/cli.rs`. Do not split the M20 matrix
  assertion update and crosslib status assertion update across parallel lanes.
- **Lane B** and **Lane C** both affect the M20 analysis story, but only Lane B
  should author fixtures. Lane C only reads them through the proof wall.
- There is no value in splitting the two new M20 nested units across separate
  lanes. They share the same module and should land together.

## Definition of Done

M62 is done when all of the following are true:

1. the exact three new unit-spec files exist and validate
2. `checkout_nested_chain3_variant.unit.spec` passes under both Rust and
   TypeScript `spec test`
3. `spec-cli/tests/cli.rs` explicitly asserts the two new M20 unsupported ids
   and the new crosslib maintained-example row
4. the M20 unsupported truth pack stays green as a pack
5. the refreshed coverage artifact shows exactly `2 real / 2 regression` for
   the target candidate, not just "better than before"
6. the refreshed recommendation artifact matches one of the allowed post-run
   buckets and does not silently keep missing-evidence blockers
7. `cargo xtask family corpus-decision --format json` succeeds on the refreshed
   basis, its artifact validates, and its action is no longer `spend_corpus_run1`
8. no new corpus source, backend capability, or policy rewrite was smuggled in
9. docs remain truthful and do not overclaim M62 as a product-capability
   milestone
10. the implementation closeout preserves a pre/post delta table for leverage,
    blockers, cluster members, and next action

## Verification Commands

Run in this order:

```bash
cargo run -p spec-cli -- validate examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --format json
cargo run -p spec-cli -- generate examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- generate examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/checkout_nested_chain3_variant.unit.spec --target-language typescript

cargo run -p spec-cli -- validate spec-cli/tests/fixtures/m20/unsupported_truth_pack/units --format json
cargo test -p spec-cli --test cli m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix -- --exact
cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact
cargo test -p spec-cli --test cli

cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json

jq '.unsupported_clusters[] | select(.cluster_id=="unsupported_dep_topology-fbecce0dbe98") | {cluster_id, representative_unit_ids, source_ids, real_example_hits, promotion_relevant_regression_hits, boundary_only_hits}' .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
jq '{recommendation_status, decision_summary, top_candidate:(.ranked_candidates[0] | {candidate_id, promotion_readiness, hold_reasons, confidence, leverage})}' .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq '{decision_action, decision_basis_code, required_next_action, summary}' .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Expected outcome:

- the maintained variant proves in both targets
- the M20 pack and repo-root example status stay truthful
- the target cluster lands at `2 real / 2 regression`
- the recommendation becomes `ranked` + `recommended` on the current code path
- `corpus-decision` pivots away from `spend_corpus_run1`

## Completion Summary

- Step 0: Scope Challenge, complete
- Architecture: corpus pressure enrichment only, no backend or policy widening
- Code Quality: explicit reuse of existing pricing shapes and manifest sources,
  plus required CLI truth-surface widening where the repo hard-codes old rows
- Implementation Plan: 3 new units, 1 existing CLI truth-surface file, existing
  proof wall, minimal doc cleanup
- Test Review: 12/12 proof paths required, no optional half-refresh
- Performance Review: near-neutral
- NOT in scope: written
- What already exists: written
- TODOS.md updates: conditional, outcome-driven only
- Failure modes: critical gaps identified around wrong-cluster landing, silent
  CLI assertion drift, and stale decision output
- Parallelization: 5 steps, 2 independent authoring lanes, 1 serial
  assertion/proof/docs lane

This is the whole move.

Spend one bounded evidence run on the exact callable-triple wrapper candidate,
make the analysis answer sharper, and stop before this turns into backend work
or policy theater.
