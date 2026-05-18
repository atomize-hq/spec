# M64: Retire the False Same-Tree Nested Chain3 Regression Thesis, Preserve the Honest Cross-Library Candidate, and Refresh Truthful Family Analysis

Status: **authoritative implementation plan**  
Milestone: **M64**  
Milestone family: **truth reset and analysis correction**  
Implementation readiness: **ready for execution**  
Plan scope: **lock the attempted M63 same-tree nested regression pair as supported `chain3` truth, lock the maintained cross-library nested example as the remaining honest `unsupported_dep_topology` pressure, then refresh coverage, recommendation, and corpus-decision from that corrected split**  
Base branch: **main**  
Working branch: **feat/m60-plus**  
Validated at commit: **`a761e28`**  
Last rewritten: **2026-05-17**

Supersedes:

- the previous repo-root M63 authority plan formerly kept at this path
- the broad retirement thesis in `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-134358.md` that treated the whole candidate as retired

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-134358.md`
- `.runs/m63_truth_correction_run1/blocked-summary.md`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec`
- `examples/shared-spec/units/pricing/base_nested_chain3.unit.spec`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- `TODOS.md`

## Primary Decision

M64 does not widen semantic review and does not retire the whole unsupported
callable-triple story.

M64 does one narrower thing:

```text
same-tree nested regression pair     -> prove supported chain3 truth
cross-library maintained real example -> preserve unsupported dep topology truth
analysis artifacts                    -> refresh from that corrected split
```

That is the whole milestone.

## Executive Summary

M63 died for the right reason and almost taught the wrong lesson.

The right lesson:

- the attempted same-tree nested regression pair does **not** publish as
  `unsupported_dep_topology`
- the reviewer routes the inner shape to supported `function.wrapper.pipeline.chain3.v1`
  with verdict `aligned`
- the reviewer routes the outer shape to supported `function.wrapper.pipeline.chain3.v1`
  with verdict `under_specified`

The wrong lesson:

- that result does **not** retire the whole callable-triple candidate
- the maintained cross-library example
  `examples_crosslib_app::pricing/checkout_nested_chain3`
  still publishes `unsupported.function.v1`
  with `unsupported_reason_codes = ["unsupported_dep_topology"]`
- the checked-in analysis artifacts still rank that cross-library example as the
  live thin candidate

So M64 is not "retire everything."

M64 is the smaller honest move:

```text
retire the fake same-tree unsupported regression thesis
keep the honest cross-library unsupported callable-triple truth
prove both sides in the real read-side surfaces that actually carry those truths
refresh analysis from the corrected basis
accept whatever truthful next recommendation remains
```

No backend widening. No new family key. No manifest churn. No fake regression
pack. No mixed-truth fixture folder with a lying name.

## Current Validated Truth

Validated on `feat/m60-plus` at `a761e28`.

### 1. The M63 blocked-state evidence is real

`.runs/m63_truth_correction_run1/blocked-summary.md` proves:

- `pricing/base_nested_chain3_bad_dep_topology`
  published as supported `function.wrapper.pipeline.chain3.v1`
  with verdict `aligned`
- `pricing/checkout_nested_chain3_bad_dep_topology`
  published as supported `function.wrapper.pipeline.chain3.v1`
  with verdict `under_specified`
- making `spec-cli/tests/cli.rs` assert those units as
  `unsupported_dep_topology` would have been a lie

That part stays.

### 2. The maintained cross-library example is still unsupported today

Direct root status truth from:

```bash
cargo run -p spec-cli -- status examples/crosslib-app --format json
```

Current result for `pricing/checkout_nested_chain3`:

- `status: valid`
- `semantic_review.verdict: under_specified`
- `semantic_review.compatibility_key: unsupported.function.v1`
- `semantic_review.support_status: unsupported`
- `semantic_review.unsupported_reason_codes: ["unsupported_dep_topology"]`

Direct root export truth from:

```bash
cargo run -p spec-cli -- export examples/crosslib-app
```

Current result:

- `units[].semantic_review` is not the authority here
- the authoritative read-side review lives on
  `passports[].semantic_review`
- the passport for `pricing/checkout_nested_chain3` still carries the same
  unsupported dep-topology review

That distinction matters. Status rows and exported passports are the truthful
surfaces here. Exported `units[]` is not the surface to assert against for this
milestone.

### 3. The copied repo-root status path is a different proof surface

`spec_status_repo_root_honors_each_root_workspace_config` in `spec-cli/tests/cli.rs`
copies tracked example roots into a temp repo and asserts multi-root discovery
and workspace-config behavior.

That test currently proves:

- cross-library namespace loading works
- copied roots are discovered correctly
- `crosslib-app` still has exactly `4` units
- copied repo-root status stays non-green because the copied crosslib wrapper
  unit is `untested`

That test is still useful, but it is **not** the authority for the direct-root
`valid + unsupported_dep_topology` truth above. M64 must keep those surfaces
separate instead of blurring them.

### 4. The checked-in analysis artifacts still point at the cross-library candidate

Current `coverage.latest.json` includes:

- `cluster_id = "unsupported_dep_topology-fbecce0dbe98"`
- `shape_fingerprint = unsupported_callable_triple`
- `representative_unit_ids = ["examples_crosslib_app::pricing/checkout_nested_chain3"]`
- `real_example_hits = 1`
- `promotion_relevant_regression_hits = 0`

Current `recommendation.latest.json` and
`corpus-program-decision.latest.json` still say:

- `recommendation_status = "no_strong_candidate"`
- `top_candidate_id = "a-unsupporteddeptopology-unsupported_dep_topology-fbecce0dbe98"`
- `decision_action = "spend_corpus_run1"`

So the honest live story is:

```text
same-tree attempted regression pair -> supported chain3
cross-library maintained example    -> unsupported dep topology
analysis basis                      -> still built from the cross-library example
```

## Problem Statement

The repo currently has two different nested callable-triple stories, and M63
treated them like one problem.

They are not one problem.

1. **Same-tree nested callable-triple shapes**
   already route through supported `function.wrapper.pipeline.chain3.v1`
   when all three deps are supported and the body is a straight-line let-threaded
   chain.

2. **The maintained cross-library nested example**
   `pricing/checkout_nested_chain3`
   still routes to `unsupported.function.v1`
   with `unsupported_dep_topology`.

M63 tried to use a same-tree regression pair to strengthen a cross-library
unsupported candidate. That is why it blocked. The regression pair was proving
the opposite thesis.

M64 fixes that category error.

## Step 0: Scope Challenge

### Premise correction

The design doc was right about one crucial point:

- the M63 same-tree regression pair cannot truthfully count as
  `unsupported_dep_topology`

But it overreached on the repo-wide conclusion.

The complete truthful premise is:

```text
retire the same-tree regression pair as unsupported evidence
do not retire the maintained cross-library unsupported example
refresh analysis from that split
```

Anything broader is sloppy.

### What already exists

| Sub-problem | Existing owner | M64 action |
| --- | --- | --- |
| same-tree supported `chain3` routing logic | `spec-core/src/semantic_review.rs` | reuse shipped routing and add exact tests before considering any production logic change |
| semantic-review chain3 fixture builders | `wrapper_pipeline_chain3_spec`, `family_b_context`, `m21_chain3_fixture_specs` in `spec-core/src/semantic_review.rs` | reuse the existing fixture style instead of inventing a new test harness |
| CLI command/test helpers | `cargo_available`, `temp_repo_dir`, `write_spec`, `run_in`, `parse_stdout_json`, `read_passport_json` in `spec-cli/tests/cli.rs` | reuse for temp-fixture proof instead of adding committed fixture files |
| copied multi-root repo proof | `spec_status_repo_root_honors_each_root_workspace_config` in `spec-cli/tests/cli.rs` | strengthen only for root-discovery invariants, not as the sole semantic truth authority |
| maintained cross-library nested example | `examples/crosslib-app/units/pricing/checkout_nested_chain3.unit.spec` plus `examples/shared-spec/units/pricing/base_nested_chain3.unit.spec` | preserve as the current honest unsupported real example |
| M63 blocked-state evidence | `.runs/m63_truth_correction_run1/blocked-summary.md` | treat as the authority explaining why same-tree regression pressure must be retired |
| family-analysis decision wall | `cargo xtask family coverage/recommend/validate-artifact/corpus-decision` | rerun unchanged after truth surfaces are corrected |

### Minimum complete slice

The minimum complete M64 slice is:

1. add focused semantic-review proof for the exact same-tree inner and outer
   shapes that blocked M63
2. add focused CLI truth-surface proof that those same-tree shapes publish
   honest supported `chain3` truth through status and exported passports
3. add or strengthen direct cross-library CLI proof so the maintained example is
   explicitly asserted as still `unsupported_dep_topology` in direct-root status
   and exported passports
4. keep the repo-root workspace-config proof honest and separate
5. rerun the family-analysis wall and validate the refreshed artifacts
6. capture the before/after delta so the next milestone is chosen from truth,
   not memory

Anything smaller is fake done.

Examples:

- proving the same-tree pair only in `spec-core` but not in public CLI surfaces
  is fake done
- asserting the cross-library example only in the copied repo-root test while
  never pinning the direct-root `valid` truth is fake done
- asserting export `units[]` instead of export `passports[]` is fake done,
  because that is not the truthful surface today
- refreshing analysis without explicitly pinning the cross-library example's
  current unsupported truth is fake done

### Complexity, blast radius, distribution, completeness

This plan stays boring on purpose.

- authored code surfaces changed: `2`
  - `spec-core/src/semantic_review.rs`
  - `spec-cli/tests/cli.rs`
- derived artifact surfaces refreshed: `3`
  - `coverage.latest.json`
  - `recommendation.latest.json`
  - `corpus-program-decision.latest.json`
- optional docs truth-maintenance: at most `1`
  - `TODOS.md`
- new classes or services: `0`
- new committed `.unit.spec` fixtures: `0`
- new distributable artifacts: `0`

Minimal diff. Explicit over clever. Full proof over shortcut.

There is no distribution work here because M64 creates no new binary, package,
container, or external artifact type beyond the already-existing analysis JSON.

## Authority Contract

This plan is the only authority for M64.

Everything else is context unless it is brought into alignment with this file.

- `PLAN.md` is the implementation authority.
- `ORCH_PLAN.md` is stale M63 orchestration context until rewritten later.
- M64 does **not** retire the whole unsupported callable-triple problem.
- M64 **does** retire the attempted same-tree regression pair as unsupported
  evidence.
- M64 preserves the current cross-library unsupported example as live truth
  unless the reviewer itself changes, which is out of scope here.
- M64 is a truth reset and read-side correction milestone, not a backend or
  family-promotion milestone.

### Exact source eligibility

M64 may modify only these authored source surfaces during core implementation:

| Surface | Responsibility | Required outcome |
| --- | --- | --- |
| `spec-core/src/semantic_review.rs` | semantic-review truth proof | new focused tests prove the same-tree pair routes to supported `chain3`; no production routing change unless a failing proof forces one |
| `spec-cli/tests/cli.rs` | public read-side truth proof | new focused same-tree CLI truth test; direct cross-library unsupported truth assertion; repo-root config proof kept honest and separate |

Derived outputs refreshed after proof:

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

Optional truth-maintenance after proof:

- `TODOS.md` only if the refreshed analysis reveals a new follow-up that would
  otherwise be lost

### Explicit no-touch surfaces

M64 does **not** edit:

- `examples/crosslib-app/units/**`
- `examples/shared-spec/units/**`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/**`
- `semantic-families/**`
- `xtask/src/family/**`
- `spec-core/src/typescript_backend.rs`
- `spec-core/src/validator.rs`

If M64 needs any of those, stop and re-scope.

## Architecture Review

### System direction

```text
CURRENT
  same-tree attempted regression pair -> supported chain3
  cross-library maintained example    -> unsupported dep topology
  analysis artifacts                  -> still rank the cross-library example

M64
  prove the same-tree pair as supported chain3 in semantic-review tests
  prove the same-tree pair as supported chain3 in CLI status/export passport surfaces
  prove the cross-library example still stays unsupported in direct-root status/export passport surfaces
  keep repo-root copied example coverage focused on workspace-config and discovery behavior
  rerun coverage/recommend/corpus-decision from that split

AFTER M64
  no fake same-tree unsupported pressure remains
  only honest cross-library pressure feeds the candidate analysis
  next wedge comes from refreshed truth, even if the answer is still "blocked"
```

### Dependency graph

```text
spec-core/src/semantic_review.rs
  └── semantic reviewer truth
      ├── same-tree nested pair -> supported chain3
      └── cross-library nested example -> unsupported dep topology

spec-cli/tests/cli.rs
  ├── temp same-tree proof project
  │   ├── spec test
  │   ├── status --format json
  │   └── export (passport semantic_review assertions)
  ├── direct copied example proof
  │   ├── status examples/crosslib-app --format json semantics
  │   └── export examples/crosslib-app passport semantics
  └── repo-root copied example proof
      └── workspace-config discovery, unit counts, and namespace hygiene

.semantic-family-artifacts/family-promotion/analysis/*.json
  └── refreshed only after the truth wall above is green
```

### Production-style failure scenarios

- If same-tree routing silently flips back to unsupported, analysis will
  overcount fake pressure and the next milestone will chase the wrong wedge.
- If the cross-library example silently flips to supported, the repo will
  pretend a live unsupported boundary no longer exists and promote from a false
  clean state.
- If repo-root copied tests are treated as the only authority, direct-root truth
  can drift without anyone noticing because `untested` and `valid` are not the
  same proof surface.

## Implementation Plan

### Step 1: Lock the same-tree truth in `spec-core`

Add focused semantic-review tests in `spec-core/src/semantic_review.rs`.

Required exact tests:

1. `same_tree_nested_chain3_inner_routes_to_supported_chain3`
   - constructs the exact inner M63 shape
   - uses the existing semantic-review fixture style already present in this
     file, not a one-off ad hoc builder
   - asserts:
     - `verdict == aligned`
     - `compatibility_key == function.wrapper.pipeline.chain3.v1`
     - `support_status == supported`

2. `same_tree_nested_chain3_outer_routes_to_supported_chain3_under_specified`
   - constructs the exact outer M63 shape
   - asserts:
     - `verdict == under_specified`
     - `compatibility_key == function.wrapper.pipeline.chain3.v1`
     - `support_status == supported`
     - `reason_codes` contains `OutsideHonestSupportedSubset`

Guardrail:

- if these tests pass without production code changes, keep the diff test-only
- if they fail, change only the smallest semantic-review surface needed to make
  the shipped behavior match the already observed M63 blocked-state truth
- no routing reorder, no new family, no cross-library widening

### Step 2: Lock the public CLI truth surfaces in `spec-cli`

M64 needs three separate proof surfaces in `spec-cli/tests/cli.rs`.

#### Step 2A: Temp same-tree truth project

Add one new focused CLI integration test:

- `nested_same_tree_chain3_truth_surfaces_publish_honest_supported_truth`

Implementation contract:

- guard with `if !cargo_available() { return; }`
- use existing helpers:
  - `temp_repo_dir`
  - `write_spec`
  - `run_in`
  - `parse_stdout_json`
  - `read_passport_json`
- author the same-tree inner and outer units in a temp project inside the test
- run:
  - `spec test units --output src/generated --crate-root .`
  - `spec status units --format json`
  - `spec export units`
- assert:
  - the inner unit publishes supported `chain3` truth
  - the outer unit publishes supported `chain3` plus `under_specified` truth
  - neither unit publishes `unsupported_dep_topology`
  - `export` assertions read from `passports[]`, not from `units[]`

This test is the public proof that the retired same-tree thesis is actually
retired in read-side surfaces, not just inside semantic-review unit tests.

#### Step 2B: Direct cross-library maintained-example proof

Add one direct-root CLI proof that pins the live unsupported candidate in the
surface that currently carries it.

Preferred test shape:

- add a new focused test in `spec-cli/tests/cli.rs` that copies
  `examples/crosslib-app` and `examples/shared-spec` into a temp area, then
  runs the same direct-root commands the operator would run against
  `examples/crosslib-app`

Required assertions:

- `status examples/crosslib-app --format json`
  - `pricing/checkout_nested_chain3`
    - `status == valid`
    - `semantic_review.compatibility_key == unsupported.function.v1`
    - `semantic_review.support_status == unsupported`
    - `semantic_review.unsupported_reason_codes == ["unsupported_dep_topology"]`
- `export examples/crosslib-app`
  - assert the same review on the passport for
    `pricing/checkout_nested_chain3`

Do not assert this truth from export `units[]`. That would be the wrong surface.

#### Step 2C: Keep repo-root copied proof honest

Keep `spec_status_repo_root_honors_each_root_workspace_config`, but tighten its
role.

Required assertions:

- `crosslib-app` still has exactly `4` units
- `pricing/checkout_nested_chain3` still appears
- copied repo-root status remains `untested` for that unit
- `SPEC_UNKNOWN_LIBRARY_NAMESPACE` remains absent

Do **not** use this test as the only proof of the maintained unsupported
candidate. Its job is workspace-config discovery and namespace hygiene, not the
direct-root semantic truth contract.

### Step 3: Refresh the analysis wall

After Steps 1 and 2 are green, rerun:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

Required read-side comparison:

- before M64:
  - same-tree pair was informally treated as unsupported regression pressure
  - checked-in artifacts still ranked the cross-library example as the thin
    unsupported callable-triple candidate
- after M64:
  - no same-tree regression proof remains in the public truth story
  - the analysis either:
    - still ranks the cross-library candidate honestly, or
    - moves to a different truthful next wedge

Both outcomes are acceptable. Lying is not.

### Step 4: Close out the truthful next action

If the refreshed analysis still lands on:

- `a-unsupporteddeptopology-unsupported_dep_topology-fbecce0dbe98`
- `no_strong_candidate`
- `spend_corpus_run1`

then the closeout must say that plainly and capture only the honest next move:

- author a truthful **cross-library** regression if one exists, or
- open a separate reviewer-widening design if the product truly wants to admit
  this cross-library nested topology as supported

Do **not** carry forward the retired same-tree regression thesis into `TODOS.md`.

## Code Quality Review

This is a small-scope truth correction. The code-quality bar is therefore
simple and strict:

- **No duplicate harnesses unless the second call site earns it.**
  Reuse existing builders and helpers first. If the new CLI proof needs local
  temp-project authoring, keep that helper narrow and colocated in
  `spec-cli/tests/cli.rs`.
- **No committed fixture churn for an uncommitted thesis.**
  Do not repurpose `spec-cli/tests/fixtures/m20/unsupported_truth_pack/**`.
  That fixture pack would become semantically dishonest.
- **Assert against the right surface.**
  In export JSON, the semantic review authority for this milestone is
  `passports[].semantic_review`, not `units[].semantic_review`.
- **Prefer explicit assertions over broad snapshot noise.**
  Pin the exact unit ids, compatibility keys, support status, and reason-code
  vectors that matter. Do not add loose "contains string" assertions.
- **Keep the diff test-first unless proof fails.**
  A production semantic-review code change is allowed only if the new exact
  tests reveal that current behavior no longer matches the already observed M63
  blocked truth.

## Test Review

100% coverage is the goal for the new truth split.

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] spec-core/src/semantic_review.rs
    │
    ├── evaluate_semantic_review_with_context()
    │   ├── family_c_deps_are_supported()
    │   │   ├── [ADD TEST] same_tree_nested_chain3_inner_routes_to_supported_chain3
    │   │   └── [ADD TEST] same_tree_nested_chain3_outer_routes_to_supported_chain3_under_specified
    │   │
    │   └── unsupported_function_dep_topology_diagnostic()
    │       └── [PROVE VIA CLI] cross-library checkout_nested_chain3 remains unsupported_dep_topology
    │
    └── supported route precedence
        └── [EXISTING] chain3 precedence tests already lock chain3 before lower families

[+] spec-cli/tests/cli.rs
    │
    ├── temp same-tree truth project
    │   ├── spec test
    │   ├── status units --format json
    │   └── export units
    │       └── [ADD TEST] nested_same_tree_chain3_truth_surfaces_publish_honest_supported_truth
    │
    ├── direct cross-library maintained example
    │   ├── status examples/crosslib-app --format json
    │   └── export examples/crosslib-app
    │       └── [ADD TEST] direct crosslib nested chain3 unsupported truth stays pinned
    │
    └── copied repo-root example status
        └── [STRENGTHEN TEST] spec_status_repo_root_honors_each_root_workspace_config

[+] family analysis
    │
    ├── coverage
    ├── recommendation
    └── corpus-decision
        └── [REFRESH] validate the new basis instead of assuming the old one
```

### Verification command matrix

Use focused selectors while iterating, then run the broader wall once the exact
targets are green.

```bash
# Step 1
cargo test -p spec-core same_tree_nested_chain3_inner_routes_to_supported_chain3 -- --exact
cargo test -p spec-core same_tree_nested_chain3_outer_routes_to_supported_chain3_under_specified -- --exact

# Step 2A
cargo test -p spec-cli --test cli nested_same_tree_chain3_truth_surfaces_publish_honest_supported_truth -- --exact

# Step 2B
cargo test -p spec-cli --test cli direct_crosslib_nested_chain3_unsupported_truth_stays_pinned -- --exact

# Step 2C
cargo test -p spec-cli --test cli spec_status_repo_root_honors_each_root_workspace_config -- --exact

# Broad local proof after focused tests pass
cargo test -p spec-core semantic_review
cargo test -p spec-cli --test cli

# Analysis wall
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

### Failure modes registry

| Codepath | Real production-style failure | Test covers it? | Error handling exists? | Silent if missed? |
| --- | --- | --- | --- | --- |
| same-tree inner routing | reviewer accidentally regresses this shape back to unsupported | yes, Step 1 exact test | yes, semantic review surfaces expose it | yes, analysis would lie again |
| same-tree outer routing | reviewer flips the outer shape to aligned or unsupported instead of under_specified supported chain3 | yes, Step 1 + Step 2A | yes, semantic review surfaces expose it | yes, public proof would claim the wrong family |
| cross-library maintained example | cleanup work accidentally retires the live unsupported candidate | yes, Step 2B | yes, status JSON and exported passport expose it | yes, refreshed analysis would pick the wrong next wedge |
| repo-root copied proof | engineers treat copied repo-root `untested` status as equivalent to direct-root `valid` status | yes, Step 2C plus plan-level separation of surfaces | no automatic guard outside tests | yes, future plan writers could assert the wrong thing |
| export assertion surface | test reads `units[].semantic_review` and reports false negative or false clean result | yes, Step 2A and 2B require passport assertions | no | yes, wrong surface means wrong truth |
| analysis refresh | tests pass but artifacts stay stale | yes, validate-artifact commands | yes, validators fail | yes, next planning step would be based on old data |
| fixture honesty | same-tree proof gets shoved into `m20_unsupported_truth_pack` anyway | covered by no-touch guard, not by runtime | no | yes, the fixture name would lie to future readers |

Critical gap rule:

- if any same-tree shape still publishes `unsupported_dep_topology` after Step 2A,
  stop the milestone
- if the cross-library maintained example stops publishing
  `unsupported_dep_topology` in Step 2B without an intentional reviewer change,
  stop the milestone
- if a test can only be made green by asserting export `units[].semantic_review`,
  stop and re-scope because the proof surface is wrong

## Performance Review

No product runtime changes. This is proof and analysis work.

The only performance risk is needless local and CI drag.

Rules:

- land targeted exact tests first
- use exact selectors while iterating
- do not run the full CLI integration wall on every edit
- refresh the three analysis artifacts only after the exact semantic-review and
  CLI truth tests are green
- prefer copied-example and temp-project proof over broad repo-root command
  matrices during iteration

## NOT in scope

- making `examples_crosslib_app::pricing/checkout_nested_chain3` supported
  `chain3`
- widening semantic review to support cross-library nested callable-triple
  topologies
- reviving the M63 worktree-only regression specs as committed repo fixtures
- changing `spec-cli/tests/fixtures/m20/unsupported_truth_pack` into a mixed
  supported-and-unsupported pack
- backend, validator, or TypeScript execution changes
- manifest, family packet, or promotion-registry changes
- a new corpus rerun story that still depends on the retired same-tree thesis

## Worktree Parallelization Strategy

This plan has two implementation lanes, then one integration lane.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| semantic-review proof | `spec-core/src/` | — |
| CLI truth-surface proof | `spec-cli/tests/` | — |
| analysis refresh and closeout | `.semantic-family-artifacts/`, `TODOS.md` | semantic-review proof, CLI truth-surface proof |

### Parallel lanes

- `Lane A`: semantic-review proof in `spec-core/src/semantic_review.rs`
- `Lane B`: CLI truth-surface proof in `spec-cli/tests/cli.rs`
- `Lane C`: analysis refresh, delta capture, and optional `TODOS.md` truth maintenance after A + B merge

### Execution order

Launch `Lane A` and `Lane B` in parallel worktrees.

Merge both.

Then run `Lane C` on the integrated state.

### Conflict flags

- `Lane A` and `Lane B` touch different module directories, so merge conflict
  risk is low
- `Lane B` itself should stay sequential inside one worktree because the same
  file owns the temp same-tree proof, the direct cross-library proof, and the
  repo-root copied proof
- `Lane C` must be sequential because analysis artifacts must be generated from
  the merged final truth wall, not from partial state

## Exit Criteria

M64 is successful only if all of the following are true:

1. `spec-core/src/semantic_review.rs` has focused exact tests proving the
   same-tree nested pair publishes supported `chain3` truth.
2. `spec-cli/tests/cli.rs` has a focused temp-project truth test proving the
   same-tree pair publishes supported `chain3` truth through CLI status and
   exported passports.
3. `spec-cli/tests/cli.rs` has a direct cross-library truth test proving
   `pricing/checkout_nested_chain3` remains `unsupported_dep_topology` in
   direct-root status and exported passports.
4. `spec_status_repo_root_honors_each_root_workspace_config` remains explicit
   about repo-root discovery and namespace hygiene without pretending to be the
   direct-root semantic authority.
5. The three family-analysis artifacts are rerun and pass validation.
6. The closeout records whether the same cross-library candidate remains live or
   whether a different truthful next wedge emerged.
7. No same-tree unsupported regression thesis survives in code, tests, docs, or
   artifacts.

## Completion Summary

- Step 0: Scope Challenge, resolved to a narrower and more truthful split
- Architecture Review: no new architecture, just a corrected truth wall and a
  clearer separation of proof surfaces
- Code Quality Review: minimal diff, no new committed fixtures, assert against
  exported passports instead of the wrong export unit surface
- Test Review: exact semantic-review proof, exact temp CLI proof, exact direct
  cross-library proof, explicit repo-root config proof, refreshed analysis wall
- Performance Review: targeted test execution only, no runtime impact
- NOT in scope: written
- What already exists: written
- Failure modes: explicit, with stop conditions
- Parallelization: 3 lanes total, 2 parallel then 1 sequential
- Lake Score: the complete option wins, because skipping the direct cross-library
  proof or the analysis refresh would save minutes and cost the next milestone
  its truth
