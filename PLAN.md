<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260503-232225.md -->
# M30 - Add Second Bounded TypeScript Family Proof

Status: **authoritative implementation plan**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-04**
Supersedes: **M29R - Additive Body Contract Recovery**
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260503-231926.md`**
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**
Public proof baseline: **GitHub Actions run `25296654428` on `6c100d05902519634dc5002445036a050b506934`**
Execution note: **No `ORCH_PLAN.md` is required up front. Create one only if the optional parallel lanes below are actually split into separate worktrees.**

## Objective

Prove exactly one new thing:

> the shared authored `body.typescript` contract for promoted `kind:function`
> families also holds for `function.wrapper.pipeline.v1`, without introducing
> repo-wide TypeScript support, a new packet root, or hidden family-specific
> routing.

This is the complete M30 claim. Nothing broader should ship under this
milestone.

## Decision

M30 uses `function.wrapper.pipeline.v1` as the second bounded TypeScript proof.

That family is the right forcing function because it is already promoted,
already packetized, and materially different from the existing monotone-up
pilot:

- it has two supported semantic deps
- it threads values through a wrapper body
- it relies on packet-local leaf units, not a single arithmetic leaf shape

If the shared authored TypeScript contract survives here, the repo can
truthfully claim cross-family generalization across two different promoted
`kind:function` shapes.

## Problem Statement

M29R answered the first question. The repo now has public evidence that
additive authored `body.typescript` can live on the shared `kind:function`
surface and coexist with the Rust-default lane without hidden `spec_version`
tricks.

The repo does not yet know whether that result generalizes beyond one family.

Today the TypeScript target gate in `xtask/src/family/prove.rs` accepts
`--target-language typescript` only for
`function.arithmetic_leaf.monotone_up.v1`. At the same time, the repo already
contains a promoted wrapper packet with registered harness ownership, prove and
certify suites, copied-fixture regressions, and scaffold smoke contracts.

That means the missing proof is narrow and concrete:

- not "add TypeScript support everywhere"
- not "add a TypeScript corpus manifest"
- not "widen `spec build/test` to TypeScript"

The missing proof is whether the existing shared authored contract survives a
second promoted family on the current family prove and certify path.

There is one explicit truth wrinkle to fix while doing that work:
`spec-cli/tests/m14_regressions.rs` still injects monotone-up TypeScript bodies
at test time when fixtures are missing them. M30 must not repeat that pattern
for wrapper proof. Committed packet bytes must be the truth source.

## Hard Boundaries

The following invariants are part of the milestone contract:

1. `semantic-families/function.wrapper.pipeline.v1/` remains the only
   authoritative packet root for the second proof.
2. Committed wrapper packet bytes, not tests, author TypeScript truth.
3. The TypeScript allowlist remains explicit and bounded to exactly two
   promoted families in M30:
   `function.arithmetic_leaf.monotone_up.v1` and
   `function.wrapper.pipeline.v1`.
4. Wrapper prove and certify must reuse the existing family harness, suite
   slugs, and artifact paths. No TypeScript-specific suite namespace and no new
   artifact tree.
5. CI failure attribution must remain family-local. A red wrapper pilot must be
   distinguishable from a red monotone-up pilot.
6. M30 must not widen beyond promoted `kind:function` families.

## Done Means

M30 is complete only when all of the following are true:

1. The committed wrapper packet fixtures truthfully carry additive
   `body.typescript` wherever the second proof depends on it, across all four
   required buckets:
   `aligned`, `drift`, `under_specified`, `unsupported_near_miss`.
2. The wrapper packet stays self-contained. If wrapper proof depends on
   packet-local discount and tax leaves, those packet-local leaves also carry
   truthful additive TypeScript bodies.
3. `xtask/src/family/scaffold.rs` and its smoke-contract tests stop describing
   wrapper starters as Rust-only if the committed packet now claims TypeScript
   truth.
4. Wrapper semantic-review tests prove authored `body.typescript` survives the
   second-family path, the same way the repo already proves that for
   monotone-up.
5. Wrapper truth-surface, corpus, and regression suites all stay green on
   committed fixture bytes.
6. Rust prove and certify for `function.wrapper.pipeline.v1` still pass:
   `cargo xtask family prove function.wrapper.pipeline.v1`
   `cargo xtask family certify function.wrapper.pipeline.v1`
7. TypeScript prove and certify for `function.wrapper.pipeline.v1` pass:
   `cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript`
   `cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript`
8. The existing monotone-up TypeScript proof still passes unchanged.
9. Public CI on the pushed SHA shows three truthful signals:
   ordinary workspace green, `monotone_up_pilot` green, and
   `wrapper_pipeline_pilot` green.
10. Closeout can truthfully say one shared authored TypeScript contract now
    holds across two promoted `kind:function` families with different shapes:
    `function.arithmetic_leaf.monotone_up.v1` and
    `function.wrapper.pipeline.v1`.

## NOT in Scope

The following work was considered and is explicitly deferred:

- `semantic-families/corpus/typescript-function.toml`
  Reason: corpus-analysis symmetry is not the question M30 is answering.
- Repo-wide `spec build --target-language typescript`
  Reason: M30 is family-local proof, not repo-wide CLI support.
- Repo-wide `spec test --target-language typescript`
  Reason: same blast-radius control.
- TypeScript support for `kind:data`
  Reason: M30 remains bounded to promoted `kind:function` families only.
- TypeScript support for `kind:sum`
  Reason: same boundary.
- TypeScript support for `.test.spec`
  Reason: molecule tests remain Rust-only.
- Third TypeScript family proof
  Reason: two families is the complete M30 claim.
- New packet roots such as `semantic-families-typescript/`
  Reason: one packet root per family must remain authoritative.
- Recommendation or coverage-analysis changes
  Reason: the maintainer corpus stays Rust-only in M30.
- npm publishing, runtime packaging, or external TypeScript distribution
  Reason: this is still an internal proof milestone.

## What Already Exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Shared authored TypeScript contract for `kind:function` | `spec-core/src/schema/unit.spec.json`, `spec-core/src/types.rs`, `spec-core/src/semantic_review.rs` | Reuse. Do not redesign the shared contract unless wrapper proof reveals a real gap. |
| Registered wrapper family harness and suite ownership | `xtask/src/family/harness.rs` | Reuse. Keep suite slugs and artifact ownership stable. |
| Wrapper packet root and Rust fixtures | `semantic-families/function.wrapper.pipeline.v1/**` | Reuse. Add additive TypeScript truth inside the existing packet. |
| Wrapper scaffold and smoke-contract coverage | `xtask/src/family/scaffold.rs`, `xtask/src/lib.rs` | Reuse. Update the wrapper starter contract so it matches committed packet truth. |
| Wrapper truth-surface, corpus, and regression suites | `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs`, `spec-core/src/semantic_review.rs` | Reuse. Extend the existing suites instead of inventing a new harness. |
| Existing monotone-up TypeScript pilot | `xtask/src/family/prove.rs`, `xtask/src/family/certify.rs`, `.github/workflows/ci.yml`, `semantic-families/function.arithmetic_leaf.monotone_up.v1/**` | Reuse as the bounded template, not the permanent one-family exception. |
| Rust-only maintainer corpus analysis | `semantic-families/corpus/rust-function.toml`, `xtask/src/family/coverage.rs`, `xtask/src/family/recommend.rs` | Reuse unchanged. |

## Closed Implementation Surface

### Primary implementation modules

- `semantic-families/function.wrapper.pipeline.v1/**`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/family/harness.rs`
- `xtask/src/family/scaffold.rs`
- `xtask/src/lib.rs`
- `.github/workflows/ci.yml`
- `PLAN.md`

### Allowed mechanical spillover

Mechanical spillover is allowed only if it is a direct compile or expectation
fix caused by the primary surface above and it does not widen the milestone.

Likely spillover sites, only if forced:

- `spec-core/src/types.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/passport.rs`

If M30 requires semantic changes outside the primary surface, stop and rewrite
the plan before continuing implementation.

## Architecture

```text
Committed wrapper packet truth
semantic-families/function.wrapper.pipeline.v1/**
    │
    ├── packet-local leaf units
    │     pricing_discount_leaf_*
    │     pricing_tax_leaf_*
    │
    ├── packet-local wrapper units
    │     pricing_total_wrapper_*
    │
    └── authored bodies
          body.rust + body.typescript
                    │
                    v
Shared loader / validator / semantic-review path
spec-core shared kind:function contract
                    │
                    ├── wrapper semantic review
                    ├── truth-surface status/export behavior
                    └── copied-fixture read-side regressions
                              │
                              v
xtask family prove/certify
                    │
                    ├── wrapper family harness
                    ├── explicit target-language allowlist
                    └── existing wrapper artifact surfaces
                              │
                              v
Public CI on pushed SHA
    ordinary workspace lane
    monotone_up_pilot
    wrapper_pipeline_pilot
```

The architecture is intentionally boring. M30 wins by proving the current
family machinery generalizes one family further, not by adding new TypeScript
infrastructure.

## Packet Truth Matrix

The wrapper packet work is not "add TypeScript somewhere." It is a fixed
12-file truth update plus scaffold alignment.

| Bucket | Required unit specs |
|---|---|
| `aligned` | `pricing_discount_leaf_aligned.unit.spec`, `pricing_tax_leaf_aligned.unit.spec`, `pricing_total_wrapper_aligned.unit.spec` |
| `drift` | `pricing_discount_leaf_drift.unit.spec`, `pricing_tax_leaf_drift.unit.spec`, `pricing_total_wrapper_drift.unit.spec` |
| `under_specified` | `pricing_discount_leaf_under_specified.unit.spec`, `pricing_tax_leaf_under_specified.unit.spec`, `pricing_total_wrapper_under_specified.unit.spec` |
| `unsupported_near_miss` | `pricing_discount_leaf_unsupported_near_miss.unit.spec`, `pricing_tax_leaf_unsupported_near_miss.unit.spec`, `pricing_total_wrapper_unsupported_near_miss.unit.spec` |

Rules for those authored TypeScript bodies:

1. Each TypeScript body must mirror that bucket's Rust body, not the aligned
   case copied everywhere.
2. The leaf units must remain truthful packet-local deps for the wrapper units.
3. `unsupported_near_miss` remains additive-only and health-neutral. The
   TypeScript body can exist, but it must not accidentally promote the shape
   into supported behavior.
4. No test may synthesize missing TypeScript bodies for wrapper fixtures at
   runtime.

## Implementation Plan

### Step 1 - Make the wrapper packet bytes truthful

Goal: move the second-family TypeScript truth into committed wrapper packet
bytes.

Modules:

- `semantic-families/function.wrapper.pipeline.v1/**`
- `xtask/src/family/scaffold.rs`
- `xtask/src/lib.rs`

Required edits:

1. Add additive `body.typescript` to all twelve wrapper packet unit specs in
   the matrix above.
2. Keep the wrapper leaf and wrapper bodies semantically aligned with their Rust
   versions bucket by bucket.
3. Update wrapper scaffold starter generation so a newly scaffolded wrapper
   packet matches the committed family contract.
4. Update wrapper smoke-contract tests so the aligned starter explicitly proves
   the expected TypeScript presence and content where relevant.

Exit criteria:

- committed wrapper packet bytes carry truthful additive TypeScript across all
  four buckets
- wrapper scaffold output is no longer Rust-only if the committed packet is not
- wrapper smoke tests still lock the starter shape

### Step 2 - Prove the shared truth surfaces consume wrapper TypeScript honestly

Goal: prove the shared loader, semantic-review path, and read-side surfaces
preserve the second-family authored TypeScript truth without regressions.

Modules:

- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- `xtask/src/family/harness.rs`

Required edits:

1. Add a wrapper semantic-review assertion parallel to
   `monotone_up_classifier_reads_authored_typescript_without_spec_version_sentinel`.
   The wrapper variant must prove authored `body.typescript` is read through the
   shared authored packet surface and cited as `body.typescript`.
2. Keep the existing wrapper classifier and routing tests green:
   `wrapper_pipeline_classifier_*`
3. Keep the existing wrapper truth-surface suite green in `spec-cli/tests/cli.rs`:
   - `wrapper_pipeline_truth_surface_command_matrix_preserves_until_spec_test_refresh`
   - `wrapper_pipeline_truth_surface_stale_status_and_export_preserve_last_proven_review`
   - `wrapper_pipeline_truth_surface_unsupported_near_miss_command_matrix_stays_neutral`
4. Extend copied-wrapper fixture coverage in `spec-cli/tests/m14_regressions.rs`
   so the wrapper corpus and regression suites are grounded entirely in committed
   packet bytes, not runtime mutation helpers.
5. If new wrapper test names are added, update `xtask/src/family/harness.rs`
   expected suite membership so prove and certify remain locked to the intended
   tests.

Exit criteria:

- wrapper authored TypeScript is proven visible to semantic review
- wrapper truth-surface suite remains honest through status and export
- wrapper copied-fixture regressions stay green on committed bytes
- no new runtime injection helper exists for wrapper proof

### Step 3 - Widen the target-language gate only far enough for wrapper proof

Goal: make prove and certify accept wrapper TypeScript proof and nothing more.

Modules:

- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`

Required edits:

1. Extend the `validate_target_language` allowlist in
   `xtask/src/family/prove.rs` from one family to exactly two:
   `function.arithmetic_leaf.monotone_up.v1`
   `function.wrapper.pipeline.v1`
2. Preserve the current rejection behavior for every other family.
3. Do not create new TypeScript-specific suite names, report names, or artifact
   paths. `family certify` should continue to inherit the prove gate through the
   existing call chain.

Exit criteria:

- wrapper prove and certify accept `--target-language typescript`
- non-wrapper, non-monotone-up families still fail fast on that flag
- artifact ownership and report paths stay unchanged

### Step 4 - Expose the second-family proof in CI without hiding attribution

Goal: make the second family publicly provable on the pushed SHA while keeping
diagnosis obvious.

Modules:

- `.github/workflows/ci.yml`

Required edits:

1. Add a dedicated `wrapper_pipeline_pilot` job.
2. Keep `monotone_up_pilot` as a separate job.
3. Update downstream release jobs that currently depend on `[test, monotone_up_pilot]`
   so they also depend on `wrapper_pipeline_pilot`.
4. Keep the ordinary `test` lane intact. Do not move wrapper proof into the
   general workspace lane.

Expected wrapper pilot commands:

```bash
cargo test -p spec-core --lib wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test cli wrapper_pipeline_truth_surface_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_corpus_ -- --color never
cargo xtask family prove function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family certify function.wrapper.pipeline.v1
cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript
```

Exit criteria:

- wrapper pilot is a distinct public CI signal
- monotone-up pilot remains distinct
- release gating depends on all required proof jobs

## Test Coverage and Proof Map

```text
M30 proof coverage
==================

[+] Packet bytes
    ├── 12 wrapper fixture unit specs carry additive TS
    └── wrapper scaffold + smoke contract match committed packet truth

[+] Shared semantic-review path
    ├── wrapper_pipeline_classifier_* stays green
    └── new wrapper authored-typescript assertion proves body.typescript is read

[+] Shared truth-surface path
    ├── wrapper_pipeline_truth_surface_* stays green
    ├── stale status/export preserves last proven review
    └── unsupported near miss stays additive-only and neutral

[+] Copied-fixture read-side path
    ├── wrapper_pipeline_corpus_aligned_fixture_projects_valid_state
    ├── wrapper_pipeline_corpus_drift_fixture_projects_failing_state
    ├── wrapper_pipeline_corpus_under_specified_fixture_projects_incomplete_state
    └── wrapper_pipeline_corpus_unsupported_near_miss_stays_additive_only_and_neutral

[+] Certify regression path
    ├── wrapper_pipeline_regression_read_side_surfaces_are_not_shadowed
    └── wrapper_pipeline_regression_unsupported_near_miss_stays_additive_only_and_neutral

[+] Target-language gate
    ├── wrapper prove/certify typescript accepted
    ├── monotone_up prove/certify typescript still accepted
    └── all other families still rejected

[+] Public proof
    ├── ordinary workspace lane green
    ├── monotone_up_pilot green
    └── wrapper_pipeline_pilot green
```

### Verification matrix

| Surface | Command floor | Why it matters |
|---|---|---|
| Wrapper scaffold contract | `cargo test -p xtask family_smoke_accepts_committed_wrapper_pipeline_scaffold_surfaces -- --color never` | proves starter output matches the committed packet contract |
| Wrapper semantic review | `cargo test -p spec-core --lib wrapper_pipeline_ -- --color never` | proves wrapper classification and new authored-TypeScript assertion |
| Wrapper truth surfaces | `cargo test -p spec-cli --test cli wrapper_pipeline_truth_surface_ -- --color never` | proves status and export preserve truthful review semantics |
| Wrapper copied-fixture corpus | `cargo test -p spec-cli --test m14_regressions wrapper_pipeline_corpus_ -- --color never` | proves all four buckets classify correctly from committed bytes |
| Wrapper certify regression | `cargo test -p spec-cli --test m14_regressions wrapper_pipeline_regression_ -- --color never` | proves read-side surfaces are not shadowed |
| Wrapper Rust family proof | `cargo xtask family prove function.wrapper.pipeline.v1` and `cargo xtask family certify function.wrapper.pipeline.v1` | proves M30 did not break the Rust-default path |
| Wrapper TypeScript family proof | `cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript` and `cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript` | proves the second-family claim directly |
| Existing monotone-up pilot | `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript` and `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript` | proves M30 did not buy the second family by regressing the first |

## Error and Rescue Registry

| Risk | Where it shows up | Early signal | Rescue path |
|---|---|---|---|
| Wrapper packet gains TypeScript in only some buckets | `semantic-families/function.wrapper.pipeline.v1/fixtures/**` | one bucket passes while another still lacks `body.typescript` | stop and finish packet truth across all twelve unit specs before widening CI |
| Wrapper proof depends on runtime fixture mutation | `spec-cli/tests/m14_regressions.rs` | new helper injects or rewrites wrapper TypeScript bodies during the test | reject the helper and move the truth into committed packet bytes |
| Wrapper scaffold lies about the family contract | `xtask/src/family/scaffold.rs`, `xtask/src/lib.rs` | scaffold smoke test passes while committed packet has stronger authored surfaces | update starter templates and smoke assertions in the same change as the packet |
| TypeScript widening leaks beyond two families | `xtask/src/family/prove.rs` | a third family starts accepting `--target-language typescript` | revert to the explicit two-family allowlist |
| CI hides which family broke | `.github/workflows/ci.yml` | a single combined pilot lane turns red | split back to `monotone_up_pilot` and `wrapper_pipeline_pilot` before calling M30 done |

## Failure Modes Registry

| Failure mode | Test coverage required | Error handling expectation | User-visible effect if missed |
|---|---|---|---|
| Wrapper fixtures claim TS proof but one bucket lacks `body.typescript` | copied-fixture corpus suite must hit all four buckets | fail in review and family proof, not silently | false green family claim |
| Wrapper authored TypeScript is present but not read through shared authored packet surfaces | new wrapper authored-TypeScript semantic-review assertion | fail in semantic-review suite | hidden regression in shared contract loading |
| Wrapper TypeScript prove passes on a bespoke path only | prove/certify commands must reuse current harness and report paths | fail by plan review before implementation completes | fake second-family success |
| Combined CI lane masks whether monotone-up or wrapper failed | distinct pilot jobs in workflow | separate jobs and downstream `needs` wiring | slow diagnosis and misleading public proof |
| Unsupported near-miss becomes health-bearing after TS addition | truth-surface and corpus unsupported tests must stay green | preserve additive-only neutral behavior | policy regression on unsupported surfaces |

Critical gap rule for M30: no new failure mode may remain untested if it would
allow the repo to make a false "two-family proof" claim.

## Parallelization Strategy

There is one narrow parallel window. The milestone is not fully parallel, but it
is also not purely sequential if worktree execution is planned carefully.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Lane A: packet truth + scaffold alignment | `semantic-families/function.wrapper.pipeline.v1/`, `xtask/src/family/scaffold.rs`, `xtask/src/lib.rs` | — |
| Lane B: semantic-review + truth-surface + copied-fixture proof | `spec-core/`, `spec-cli/tests/`, `xtask/src/family/harness.rs` | Lane A |
| Lane C: target-language gate + CI wiring | `xtask/src/family/prove.rs`, `xtask/src/family/certify.rs`, `.github/workflows/ci.yml` | Lane A |
| Lane D: final merge verification and cleanup | whole closed implementation surface | Lanes B and C |

### Parallel lanes

- `Lane A`: packet truth + scaffold alignment
  This must land first because every later proof lane depends on committed
  wrapper packet bytes and the finalized starter contract.
- `Lane B`: semantic-review + truth-surface + copied-fixture proof
  Starts after Lane A. Sequential inside the lane because `spec-core/`,
  `spec-cli/tests/`, and `xtask/src/family/harness.rs` are tightly coupled.
- `Lane C`: target-language gate + CI wiring
  Starts after Lane A. Sequential inside the lane because the prove gate and CI
  expectations should move together.
- `Lane D`: final verification in the main worktree
  Runs after B and C merge.

### Execution order

1. Finish `Lane A` first.
2. Launch `Lane B` and `Lane C` in parallel worktrees.
3. Merge B and C.
4. Run the full verification floor in the main worktree.
5. Fix only merge fallout or expectation drift in `Lane D`.

### Conflict flags

- `Lane B` owns `xtask/src/family/harness.rs`. `Lane C` must not edit that file.
- If `Lane C` discovers it also needs harness expectation edits, collapse B and C
  back into one sequential lane. Do not accept overlapping ownership of
  `xtask/src/family/`.
- `Lane A` must freeze bucket file names and unit ids before B or C start, or
  both later lanes will rebase on moving packet paths.

## Exact Verification Sequence

Run this exact merged-state verification sequence before calling M30 done:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo run -p spec-cli -- generate examples/ecommerce/units --output examples/ecommerce/src/generated
cargo check --manifest-path examples/ecommerce/Cargo.toml
cargo test -p xtask family_smoke_accepts_committed_wrapper_pipeline_scaffold_surfaces -- --color never
cargo test -p spec-core --lib wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test cli wrapper_pipeline_truth_surface_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_regression_ -- --color never
cargo test -p spec-core --lib monotone_up_classifier_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_regression_ -- --color never
cargo xtask family smoke function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1
cargo xtask family certify function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

This is the verification floor. If one of these commands no longer represents
the real proof path, fix the plan or the implementation before closing M30.

## Closeout Questions

M30 closes only after the author can answer these questions plainly:

1. Did one shared authored TypeScript contract survive on two promoted
   `kind:function` families?
2. Did ordinary CI, `monotone_up_pilot`, and `wrapper_pipeline_pilot` all pass
   on the pushed SHA?
3. Did the wrapper proof reuse the existing family registry, packet root,
   harness, and artifact paths?
4. What exact question is still unanswered after M30?

Allowed closeout verdicts:

- `EXPAND`
  The second family proof passed cleanly and the repo is ready to consider the
  next bounded TypeScript question.
- `NARROW`
  The wrapper proof mostly worked, but one additional bounded follow-on is
  required before broader expansion is honest.
- `STOP`
  The second proof required a new packet root, repo-wide target-language
  support, or hidden family-specific routing to pass.

Anything softer is not a real milestone closeout.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAN | 1 proposal, 1 accepted, 0 deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAN | 3 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | no UI scope in M30 |

- **DX:** 1 review path through `/autoplan`, clean. TTHW: ~20 min -> ~10 min.
- **UNRESOLVED:** 0
- **VERDICT:** CEO + ENG CLEARED, ready to implement M30 on the closed surface above.
