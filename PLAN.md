<!-- plan backup: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-plan-backup-20260502-125438.md -->
# M27.9B - money/round Durable-Hold Resolution

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-02**  
Supersedes: **M27.9A - Stop-Path Closeout And Analysis Contract Recalibration**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-122109.md`**

## Summary

M27.9A already closed the fake arithmetic-ready story.

The repo's truthful baseline is now:

- `function_coverage = 28 / 17 / 0 / 11`
- `recommendation_status = "no_strong_candidate"`
- one visible held cluster remains:
  `unsupported_function_surface-e40675da6fa0`
- that cluster is represented by `money/round`

M27.9B is the narrow follow-on that removes the last fake ambiguity.

This milestone does not spend corpus run `1`.
It does not reopen semantic-review family routing.
It does not start M28.

It does one thing:

1. replace the generic `unknown_overlap_family` story for the current
   `money/round` cluster with explicit durable-hold truth

The target repo conclusion is:

- `money/round` remains visible pressure
- `money/round` is not the next family-promotion target
- `money/round` remains held because it is helper-surface pressure inside
  already-promoted arithmetic workflows, not a standalone promotable boundary
- corpus run `1` stays unspent and unauthorized by default

That is the whole milestone.

## Done Means

M27.9B is complete only when all of the following are true:

1. the current `money/round` cluster stays visible in recommendation output
2. the cluster no longer uses `unknown_overlap_family`
3. the cluster now uses `helper_surface_not_promotable`
4. `next_step_status = durable_hold`
5. `next_step_detail = helper_surface_not_promotable`
6. `recommendation_status` remains `no_strong_candidate`
7. corpus run `1` remains unspent and unauthorized by default
8. `xtask/src/lib.rs` locks the new truth end to end
9. `PLAN.md` and
   `docs/recommendation_corpus_expansion_program_v0.1.md` tell the same story

## Current Repo Truth

### Locked baseline after M27.9A

- `function_coverage.total_units = 28`
- `function_coverage.promoted_family_units = 17`
- `function_coverage.supported_unpromoted_family_units = 0`
- `function_coverage.unsupported_function_units = 11`
- `recommendation_status = "no_strong_candidate"`

### Remaining visible candidate

The current recommendation artifact still exposes one held candidate:

- candidate id:
  `z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0`
- cluster id: `unsupported_function_surface-e40675da6fa0`
- reason code: `unsupported_function_surface`
- overlap family: `unknown`
- promotion readiness: `hold`
- hold reasons:
  - `unknown_overlap_family`
- leverage:
  - `real_example_hits = 2`
  - `promotion_relevant_regression_hits = 1`
  - `boundary_only_hits = 0`
  - `total_units_in_cluster = 3`

### Why this is the right next problem

The problem is not "there is still a hold."

The problem is that the hold is still described too vaguely.

The three representative units already tell the stronger story:

- `examples/ecommerce/units/money/round.unit.spec`
  - unary helper-shaped function
  - placeholder identity body
- `examples/shared-spec/units/money/round.unit.spec`
  - unary helper-shaped function
  - real rounding implementation for sibling reuse
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/money/round.unit.spec`
  - unary helper-shaped function
  - placeholder identity body in the explicit unsupported pack

At the same time, maintainer docs already lock the important semantic truth:

- promoted arithmetic leaf families already cover zero-or-one helper deps
- packet-local `money/round` exists to preserve optional-helper topology
- `shared::money/round` and local `money/round` are the same helper-aware
  boundary for the promoted arithmetic leaf families

That means the live ambiguity is not "which family should we promote next?"

It is:

> should the repo keep treating this helper surface like latent family
> pressure, or should it say plainly that this is durable helper-only pressure
> under the current roadmap?

M27.9B answers that explicitly.

## Authority And Evidence

Primary decision inputs:

- `PLAN.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-122109.md`
- `docs/m27_5_recommendation_quality_plan_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- `examples/ecommerce/units/money/round.unit.spec`
- `examples/shared-spec/units/money/round.unit.spec`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/money/round.unit.spec`

If any older note still implies that `money/round` is an unresolved maybe-next
family, this file wins for M27.9B.

## Scope Challenge

### What already exists

| Sub-problem | Existing code / truth | Decision |
|---|---|---|
| Unsupported-cluster discovery | `xtask/src/family/coverage.rs` already groups unsupported units, computes leverage, and assigns coarse `overlap_family` | Reuse. Do not build a second analysis pass or new command. |
| Recommendation gating | `xtask/src/family/recommend.rs` already converts unsupported clusters into held or ready candidates | Reuse. Narrow the current hold story instead of inventing a parallel recommendation workflow. |
| Artifact contract enforcement | `xtask/src/family/promotion_artifacts.rs` already validates recommendation-analysis artifacts strictly | Reuse. Extend the existing contract with the new durable-hold fields and validation rules. |
| Locked proof surface | `xtask/src/lib.rs` already owns the locked-corpus command-path assertions | Reuse. Add the new durable-hold lock there. |
| Maintainer explanation of helper-aware arithmetic leaves | `semantic-families/README.md` already says optional helper deps are inside the promoted arithmetic leaf boundary | Treat as authoritative input. Do not reopen family semantics in M27.9B. |
| Program governance | `docs/recommendation_corpus_expansion_program_v0.1.md` already tracks whether corpus work is still justified | Reuse. Update the program state after durable-hold resolution lands. |

### Minimum honest change

The minimum complete M27.9B diff is:

1. enrich the existing recommendation-analysis artifact with explicit next-step
   resolution for held unsupported-function candidates
2. classify the current `money/round` cluster as durable hold for a
   machine-readable helper-surface reason
3. lock that outcome in `xtask` tests and artifact validation
4. rewrite the plan/program docs so future sessions stop treating this as
   generic unresolved family pressure

Anything smaller leaves the repo in the same ambiguous state.

### Complexity and blast radius

This milestone touches six authored files plus two derived JSON refreshes:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- `PLAN.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Derived proof surfaces expected to refresh:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

That is small enough to proceed as one milestone, but it is still one contract
surface. If implementation expands beyond those authored files plus the two
derived artifacts, stop and split the work.

### Search and boring-tech rule

- **[Layer 1]** Reuse the current coverage artifact instead of inventing a new
  `family inspect money-round` command.
- **[Layer 1]** Reuse the current recommendation-analysis artifact instead of
  adding a second governance artifact.
- **[Layer 1]** Reuse the existing locked-corpus command-path tests in
  `xtask/src/lib.rs`.
- **[EUREKA]** The remaining problem is not missing semantic support for helper
  deps. The repo already proved helper-aware arithmetic leaves. The remaining
  problem is that a helper function surface is still being described like a
  latent family-promotion candidate.

### Locked decisions

1. No new corpus run is part of M27.9B.
2. No semantic-review family routing changes are part of M27.9B.
3. `money/round` is resolved in the recommendation/governance layer, not by
   inventing a new promoted-family packet.
4. The intended outcome is a durable hold, not another "still unknown, think
   later" placeholder.
5. M28 remains out of scope during this milestone, but M27.9B may unblock the
   later M28 decision by making the current hold explicit.

### Exact file contract

| File | Responsibility | Must not happen |
|---|---|---|
| `xtask/src/family/coverage.rs` | Preserve the current cluster-discovery path while exposing enough helper-context truth for recommendation resolution | Do not add a second repo scan or new corpus source. |
| `xtask/src/family/recommend.rs` | Convert the current `money/round` cluster from generic `unknown_overlap_family` into explicit durable-hold resolution | Do not loosen ranked-status rules or rerank the corpus. |
| `xtask/src/family/promotion_artifacts.rs` | Validate the new durable-hold fields and forbid contradictory combinations | Do not make schema validation softer. |
| `xtask/src/lib.rs` | Lock the end-to-end command-path truth for the durable-hold outcome | Do not leave old `unknown_overlap_family` expectations in locked tests. |
| `PLAN.md` | Record M27.9B as durable-hold resolution work | Do not preserve "maybe corpus next" language. |
| `docs/recommendation_corpus_expansion_program_v0.1.md` | Update the program tracker so corpus run `1` remains unspent and the current hold is treated as durable helper pressure | Do not silently authorize a follow-up corpus run. |

### NOT in scope

- adding repo-owned corpus examples
  reason: the whole point of M27.9B is to avoid spending corpus budget by habit
- promoting a helper-function family packet
  reason: the current roadmap and packet inventory do not authorize a standalone
  `money/round` promotion path
- widening semantic-review support for unary helper surfaces
  reason: promoted arithmetic leaves already prove helper-aware consumption
- changing coverage accounting
  reason: `28 / 17 / 0 / 11` is already the truthful locked baseline
- starting M28 shared-core extraction
  reason: M27.9B only resolves the last lingering next-family ambiguity
- broad README or roadmap cleanup
  reason: only the surfaces needed to encode the new durable-hold truth should move

## Output Contract

M27.9B adds an explicit next-step resolution contract for held
`unsupported_function_surface` candidates.

### Required artifact contract changes

`RecommendationCandidateEntry` gains two required fields:

- `next_step_status`
- `next_step_detail`

Additive enum surface for this milestone:

- `HoldReason::HelperSurfaceNotPromotable`
- `NextStepStatus::DurableHold`
- `NextStepDetail::HelperSurfaceNotPromotable`

Because these are new required fields in the recommendation-analysis artifact,
`RECOMMENDATION_ANALYSIS_SCHEMA_VERSION` must bump when the contract lands.

### M27.9B target outcome

For the current visible `money/round` candidate, the resulting recommendation
analysis must read as:

```text
candidate_id = z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0
cluster_ids = [unsupported_function_surface-e40675da6fa0]
promotion_readiness = hold
hold_reasons = [helper_surface_not_promotable]
next_step_status = durable_hold
next_step_detail = helper_surface_not_promotable
recommendation_status = no_strong_candidate
```

### Meaning of the target outcome

`helper_surface_not_promotable` means:

- the cluster is visible and real
- the repo keeps it in the artifact
- the repo does not treat it as the next family-promotion target
- the repo does not authorize a corpus run to rescue it automatically

### Rejected alternatives inside M27.9B

- `known_overlap`
  reason: the current `money/round` cluster is not itself a promoted-family leaf
  or wrapper boundary
- `targeted_evidence_gap`
  reason: the current repo already has enough truth to say the helper surface is
  not the right next-family pressure surface

If implementation evidence contradicts this target outcome, stop and rewrite
the contract before landing. Do not silently widen scope.

## Architecture Review

### Core rule

Do not solve this with a new workflow.

Solve it inside the workflow the repo already trusts:

- coverage artifact projects the unsupported cluster
- recommendation-analysis artifact explains what that cluster means
- locked `xtask` tests prove the explanation stays true

### Layered model

```text
LAYER 1: DISCOVERY
==================
authored corpus units
    -> semantic review
    -> unsupported clusters

LAYER 2: INTERPRETATION
=======================
unsupported_function_surface cluster
    -> helper-surface resolution rule
    -> recommendation candidate next-step contract

LAYER 3: GOVERNANCE
===================
recommendation.latest.json
    -> PLAN.md
    -> recommendation_corpus_expansion_program_v0.1.md
    -> later milestone choice
```

### Data flow

```text
AUTHORED SOURCE TRUTH
=====================
examples/ecommerce::money/round
examples_shared_spec::money/round
m20_unsupported_truth_pack::money/round
        |
        v
COVERAGE PROJECTION
===================
xtask/src/family/coverage.rs
  - discover cluster
  - preserve leverage counts
  - expose helper-context facts needed for resolution
        |
        v
RECOMMENDATION RESOLUTION
=========================
xtask/src/family/recommend.rs
  - candidate remains hold
  - hold reason becomes explicit
  - next_step_status = durable_hold
        |
        v
GOVERNANCE CONTRACT
===================
recommendation.latest.json
xtask/src/lib.rs lock tests
PLAN.md
corpus expansion program tracker
```

### Dependency graph

```text
xtask/src/family/coverage.rs
    |
    v
xtask/src/family/recommend.rs
    |
    v
xtask/src/family/promotion_artifacts.rs
    |
    v
xtask/src/lib.rs
```

### State transition

```text
PRE-M27.9B
==========
money/round visible
hold reason = unknown_overlap_family
next move still ambiguous
        |
        | M27.9B
        v
POST-M27.9B
===========
money/round visible
hold reason = helper_surface_not_promotable
next_step_status = durable_hold
corpus run 1 stays unspent
```

### Architecture-specific failure scenario

If the new resolution logic lives only in docs and not in the artifact
contract, future runs will regenerate `unknown_overlap_family` and erase the
decision. That would put the repo back in the same ambiguous state while still
looking green.

This milestone is only real if the command path owns it.

## Code Quality And Complexity Guardrails

- Keep the diff boring.
- Do not create a second artifact file.
- Do not add a new `cargo xtask family ...` command.
- Do not hide the durable-hold rule behind a generic policy engine if one
  explicit helper-surface branch will do.
- If the resolution logic needs an inline diagram comment, place it next to the
  branch-heavy rule in `xtask/src/family/recommend.rs` and keep it updated with
  the code.

## Implementation Plan

### Step 0 - Freeze the M27.9A baseline as authoritative input

Before changing code, confirm the repo still reproduces:

- `28 / 17 / 0 / 11`
- `recommendation_status = "no_strong_candidate"`
- one visible held candidate:
  `unsupported_function_surface-e40675da6fa0`

Done means M27.9B starts from the already-locked post-closeout baseline, not
from memory.

### Step 1 - Surface helper-context facts during unsupported-cluster projection

Extend the existing coverage projection just enough to support truthful
recommendation resolution.

Required facts:

1. the representative units are unary helper-shaped functions
2. the current real-example pressure comes from helper surfaces already used
   inside promoted arithmetic leaf workflows
3. the cluster itself is not a new leaf or wrapper boundary

Do this inside `xtask/src/family/coverage.rs`, not in an ad hoc follow-up scan.

Done means `recommend.rs` has enough repo-owned truth to stop saying
"unknown" by default.

### Step 2 - Replace the generic hold story with durable-hold resolution

In `xtask/src/family/recommend.rs`, add the narrow resolution branch for the
current `unsupported_function_surface` case.

Decision rule for M27.9B:

- if the cluster is helper-shaped, lacks a standalone promoted-family boundary,
  and its real-example pressure is still only helper pressure inside already
  promoted arithmetic workflows, then:
  - keep `promotion_readiness = hold`
  - replace `unknown_overlap_family` with
    `helper_surface_not_promotable`
  - set `next_step_status = durable_hold`
  - set `next_step_detail = helper_surface_not_promotable`

This is explicit over clever. No scoring system. No generic resolver
abstraction. No policy engine.

Done means the command path now says what the repo should do:
do not promote this, do not spend corpus by default, move on.

### Step 3 - Tighten artifact validation around the durable-hold contract

Update `xtask/src/family/promotion_artifacts.rs` so the new fields are required
and internally consistent.

Required validation rules:

1. `helper_surface_not_promotable` is allowed only with
   `next_step_status = durable_hold`
2. `next_step_status = durable_hold` requires
   `promotion_readiness = hold`
3. `next_step_status = durable_hold` must not coexist with
   `recommendation_status = ranked`
4. the old bare `unknown_overlap_family` shape is rejected for the locked
   `money/round` command-path case
5. `RECOMMENDATION_ANALYSIS_SCHEMA_VERSION` is bumped as part of this change

Done means the schema enforces the decision instead of merely describing it.

### Step 4 - Lock the new truth in end-to-end command-path tests

Update `xtask/src/lib.rs` so the locked-corpus tests assert:

- the top-level status stays `no_strong_candidate`
- the visible candidate remains
  `unsupported_function_surface-e40675da6fa0`
- the candidate stays `hold`
- the hold reason is now `helper_surface_not_promotable`
- `next_step_status = durable_hold`
- `next_step_detail = helper_surface_not_promotable`
- no corpus-expansion authorization signal appears anywhere in the current
  command-path output

Done means future refactors cannot quietly reintroduce the generic hold story.

### Step 5 - Rewrite the planning and program ledger

Update `PLAN.md` and
`docs/recommendation_corpus_expansion_program_v0.1.md` so they both encode:

- `money/round` is still visible
- `money/round` is not the next family
- corpus run `1` stays unspent
- the repo now needs an explicit post-M27.9B milestone choice rather than more
  shadow argument over this helper surface

Done means future sessions stop trying to rescue the wrong thing.

## Test Review

### Framework and suites

Runtime: Rust workspace with `cargo test`

Suites that must move together:

- `xtask` family coverage/recommendation unit tests
- `xtask` recommendation-analysis artifact validation tests
- `xtask` locked-corpus command-path tests

No `spec-core` or `spec-cli` implementation changes should be required for
M27.9B. If those crates need edits, the milestone has already drifted.

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] xtask/src/family/coverage.rs
    |
    |- [ADD TEST] helper-surface context is exposed consistently for the
    |             current unsupported_function_surface cluster
    |- [ADD TEST] current money/round cluster still preserves leverage
    |             counts 2 / 1 / 0 / 3
    `- [ADD TEST] no new cluster ids or rankable candidates appear

[+] xtask/src/family/recommend.rs
    |
    |- [ADD TEST] helper-only visible cluster resolves to
    |             hold + helper_surface_not_promotable
    |- [ADD TEST] next_step_status = durable_hold
    |- [ADD TEST] recommendation_status remains no_strong_candidate
    `- [ADD TEST] generic unknown_overlap_family is no longer emitted
                for the locked money/round case

[+] xtask/src/family/promotion_artifacts.rs
    |
    |- [ADD TEST] durable_hold requires hold readiness
    |- [ADD TEST] helper_surface_not_promotable requires durable_hold
    |- [ADD TEST] ranked status rejects durable-hold candidates
    `- [ADD TEST] schema version bump is enforced for the new required fields

[+] xtask/src/lib.rs
    |
    |- [LOCK TEST] command path writes same bytes deterministically
    |- [LOCK TEST] visible candidate id remains e40675da6fa0
    |- [LOCK TEST] hold reason becomes helper_surface_not_promotable
    `- [LOCK TEST] next_step_status = durable_hold

[+] Governance docs
    |
    |- [MUST LAND] PLAN.md records durable helper hold explicitly
    `- [MUST LAND] corpus program tracker leaves run 1 unspent
```

### Required tests and proof assertions

1. `xtask/src/family/recommend.rs`
   - helper-surface candidate stays held
   - hold reason becomes `helper_surface_not_promotable`
   - `next_step_status = durable_hold`
   - `recommendation_status` stays `no_strong_candidate`
2. `xtask/src/family/promotion_artifacts.rs`
   - new field validation for `next_step_status`
   - invalid combinations are rejected
   - schema version bump is asserted
3. `xtask/src/lib.rs`
   - end-to-end locked-corpus command-path test updated to the new durable-hold
     truth

### Regression rule

This entire milestone is a governance regression fix.

The old regression is:

- repo truth already showed helper-aware arithmetic coverage
- the artifact still described `money/round` like unresolved next-family pressure

The new regression tests must prove that cannot happen again.

### Test plan artifact

During implementation verification, write the QA-facing artifact to:

- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-eng-review-test-plan-{timestamp}.md`

That artifact should tell QA to verify:

- the visible candidate still exists
- the cluster id and leverage counts did not change
- the hold reason changed from generic overlap ambiguity to explicit helper
  non-promotability
- `next_step_status = durable_hold`
- corpus run `1` is still not authorized by the resulting docs and artifacts

## Failure Modes

| Codepath | Realistic failure | Test required? | Error handling exists? | Silent if missed? | Critical gap? |
|---|---|---|---|---|---|
| coverage helper-context facts are wrong | recommend layer classifies the candidate with the wrong next-step meaning | yes | test-only | yes | **yes** |
| recommend layer keeps `unknown_overlap_family` fallback | repo appears to resolve the issue in docs but not in artifacts | yes | no | yes | **yes** |
| artifact validation accepts contradictory fields | future changes can emit `durable_hold` fields with `ranked` top-level status | yes | test-only | yes | **yes** |
| schema version is not bumped | consumers cannot distinguish old vs new recommendation-analysis bytes | yes | no | yes | **yes** |
| locked command-path tests are not updated | branch truth drifts back to ambiguity after a refactor | yes | test-only | yes | **yes** |
| program doc silently authorizes corpus run 1 | future sessions spend evidence budget on a problem already resolved as durable hold | yes | no | yes | **yes** |

If any one of those lands, the repo will make the wrong milestone decision
while still looking green.

## Performance And Operational Review

There is no meaningful user-facing runtime risk here.

The real risks are:

- artifact determinism drift
- governance drift
- needless extra repo scans

Operational rules:

- keep the helper-resolution logic in the existing coverage/recommend path
- do not add a second full workspace traversal if current cluster facts are
  already sufficient
- refresh the two analysis artifacts once after tests pass
- do not hand-edit the derived JSON files

## Worktree Parallelization Strategy

This milestone has limited but real parallelization.

The core `xtask` contract changes are sequential. The governance rewrite can run
in parallel only after the durable-hold wording is frozen.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Freeze baseline and inspect current artifact bytes | `xtask/`, `.semantic-family-artifacts/` | - |
| Add helper-context projection | `xtask/src/family/` | baseline confirmed |
| Add durable-hold recommendation resolution | `xtask/src/family/` | helper-context projection |
| Add schema validation and lock tests | `xtask/src/family/`, `xtask/src/` | durable-hold recommendation resolution |
| Rewrite plan/program governance language | repo root docs, `docs/` | durable-hold terminology frozen |
| Final proof and artifact refresh | `xtask/`, `.semantic-family-artifacts/`, docs | all prior steps complete |

### Parallel lanes

- Lane A: implementation contract
  `coverage.rs -> recommend.rs -> promotion_artifacts.rs -> lib.rs`
- Lane B: governance docs
  `PLAN.md -> docs/recommendation_corpus_expansion_program_v0.1.md`

### Execution order

1. Launch Lane A first.
2. Start Lane B only after Lane A fixes the output vocabulary:
   `helper_surface_not_promotable` + `durable_hold`.
3. Merge Lane B only after Lane A proves the artifact fields and tests match
   the new outcome.
4. Run the proof loop once on the integrated branch.

### Conflict flags

- Steps inside `xtask/src/family/` are not parallel-safe. Keep them sequential.
- Do not split `xtask/src/family/recommend.rs` and
  `xtask/src/family/promotion_artifacts.rs` across separate workers. They are
  one contract surface.
- Lane B must not invent terminology that Lane A does not emit.

### Practical recommendation

Use two workstreams:

- Workstream 1: `xtask` implementation plus lock tests
- Workstream 2: plan/program rewrite after the durable-hold wording is fixed

Then integrate once and refresh the derived analysis artifacts.

## Proof Loop

Run in this exact order:

```bash
cargo test -p xtask -- --color never

cargo xtask family coverage --format json
cargo xtask family recommend --format json

cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

Acceptance rule:

- `xtask` tests pass
- coverage generation succeeds
- recommendation generation succeeds
- both artifacts validate
- the visible candidate remains `unsupported_function_surface-e40675da6fa0`
- the hold reason is `helper_surface_not_promotable`
- `next_step_status = durable_hold`
- the top-level output remains `no_strong_candidate`

If any one of those fails, stop before touching the program ledger again.

## Acceptance Criteria

M27.9B is accepted only if the implementation branch proves all of the
following together:

1. the current `money/round` cluster stays visible in recommendation output
2. the cluster no longer uses `unknown_overlap_family`
3. the cluster now uses `helper_surface_not_promotable`
4. `next_step_status = durable_hold`
5. `next_step_detail = helper_surface_not_promotable`
6. `recommendation_status` remains `no_strong_candidate`
7. corpus run `1` remains unspent and unauthorized by default
8. `xtask/src/lib.rs` locks the new truth end to end
9. `PLAN.md` and the corpus-expansion program tracker tell the same story

## Next Step After M27.9B

After M27.9B lands, the repo should stop asking whether `money/round` is the
next family.

That question is answered.

The next decision becomes a separate, explicit roadmap choice:

- do we have any other real next-family pressure worth planning
- or is it time to advance the post-M27.5 M28 architectural decision

That follow-on choice is downstream of M27.9B and is not part of this file.
