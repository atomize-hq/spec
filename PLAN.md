# M27.8 - Crosslib Arithmetic Confirmation Pack

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-01**

## Summary

M27.8 is a one-wedge corpus-expansion milestone.

The job is to add exactly one maintained cross-library real example,
`examples/crosslib-app/units/pricing/apply_tax.unit.spec`, then prove that this
single new example is enough to move the current recommendation output from
`no_strong_candidate` to `ranked` without changing corpus policy, coverage
logic, recommendation logic, or artifact schemas.

If this one-example wedge lands and the ranked output matches the locked values
below, corpus expansion stops and the next milestone becomes promotion-focused.
If it does not land, corpus expansion also stops and the repo re-plans from the
mismatch.

## Plan Authority

This file is the authoritative next-run contract after M27.75.

Primary sources:

- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `semantic-families/corpus/rust-function.toml`

Repo truth checked while writing this plan:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- `examples/crosslib-app/spec.toml`
- `examples/crosslib-app/units/pricing/apply_discount.unit.spec`
- `examples/shared-spec/units/money/round.unit.spec`
- `examples/crosslib-app/units/.gitignore`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/money/round.unit.spec`

Live repo truth rechecked while writing this plan:

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

Detached-worktree dry-run truth captured on `2026-05-01` is also part of this
contract. The exact expected output deltas below come from that proof run.

If any stale note, tracker intuition, orchestration note, or branch-local memory
disagrees with this file, this file wins.

## Problem Statement

M27.75 proved the five-source corpus is honest enough to abstain.

Current truth is:

- `recommendation_status = "no_strong_candidate"`
- `unsupported_function_surface-e40675da6fa0` is still held only for
  `unknown_overlap_family`
- `unsupported_arithmetic_shape-2694b2baf65b` is still held for
  `thin_real_example_support` and `thin_regression_support`

The key repo-truth conclusion is:

- the `money/round` cluster is not waiting on more corpus to resolve
  `unknown_overlap_family`
- the arithmetic unsupported cluster is still waiting on corpus

That follows directly from current code:

1. `xtask/src/family/coverage.rs` derives overlap family from unsupported reason
   codes plus stored shape fingerprints.
2. The current `unsupported_function_surface-e40675da6fa0` cluster stays
   `unknown` because its reason code is `unsupported_function_surface` and its
   fingerprint does not advertise `arithmetic_like` or `wrapper_like`.
3. Adding more examples to that same cluster changes leverage counts, not the
   inferred overlap family.
4. The arithmetic cluster already resolves to
   `function.arithmetic_leaf.monotone_*` with `difficulty.tier = "adjacent"`.
5. Under `xtask/src/family/recommend.rs`, one more real-example hit in that
   arithmetic cluster is enough to clear both current hold reasons and make the
   candidate `ready` at `confidence.level = "medium"`.

So the smallest honest next wedge is not another manifest expansion, not another
regression-pack padding run, not a policy change, and not M28 shared-core work.
It is one maintained repo-owned real example in the already-counted
`examples_crosslib_app` source, shaped to land in
`unsupported_arithmetic_shape-2694b2baf65b`.

## Milestone Outcome

When M27.8 lands, the repo can truthfully claim:

- the Rust recommendation corpus still uses the same five manifest sources
- `examples_crosslib_app` grows from `1` function unit to `2`
- `function_coverage.total_units` grows from `27` to `28`
- `function_coverage.unsupported_function_units` grows from `12` to `13`
- the arithmetic-shape cluster gains a second real-example hit
- the arithmetic-shape cluster becomes `promotion_readiness = "ready"`
- the top-level recommendation flips from `no_strong_candidate` to `ranked`
- the `money/round` unknown-overlap cluster remains visible and honestly held
- the next milestone should be promotion-focused, not another blind corpus run

M27.8 does **not** claim:

- recommendation policy changed
- overlap-family inference changed
- the `money/round` cluster is now understood
- shared-core extraction started
- second-language proof started
- the multi-run tracker needs to be rewritten mid-run

## Scope

### In Scope

- add exactly one maintained real-example unit under
  `examples/crosslib-app/units`
- make that new unit count through the existing `examples_crosslib_app` source
- whitelist the new passport in `examples/crosslib-app/units/.gitignore`
- update the locked `xtask` command-path test to the new ranked truth
- rerun source proof, cross-library build proof, coverage, recommendation,
  artifact validation, and `xtask` tests

### NOT In Scope

- editing `semantic-families/corpus/rust-function.toml`
- changing `xtask/src/family/coverage.rs`
- changing `xtask/src/family/recommend.rs`
- changing `xtask/src/family/promotion_artifacts.rs`
- changing artifact schemas
- rewriting `docs/recommendation_corpus_expansion_program_v0.1.md`
- adding new regression fixtures
- doing M28 work

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Current five-source corpus | `semantic-families/corpus/rust-function.toml` | Reuse as-is. No new source ids. |
| Arithmetic cross-library real example shape | `examples/crosslib-app/units/pricing/apply_discount.unit.spec` | Reuse as the authored template. |
| Shared helper dependency path | `examples/crosslib-app/spec.toml` and `examples/shared-spec/units/money/round.unit.spec` | Reuse as-is. The sibling-library route already exists. |
| Arithmetic regression anchor | `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec` | Reuse as the regression-side representative. |
| Recommendation thresholds | `xtask/src/family/recommend.rs` | Reuse as-is. This milestone is evidence, not policy. |
| Locked command-path proof | `xtask/src/lib.rs`, test `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()` | Extend and rename. Do not create a parallel lock. |

### Minimum honest change

The smallest complete diff is:

1. one new authored unit spec in `examples/crosslib-app/units/pricing/`
2. one `.gitignore` whitelist update so the passport is intentional repo truth
3. one `xtask` command-path lock update to the new ranked output

Anything larger is scope creep. Anything smaller leaves the milestone under-locked.

### Complexity check

This plan touches exactly three tracked source files and introduces zero new Rust
modules, services, or abstraction layers.

That is the right size. Boring on purpose.

### Completeness check

Do the complete version now:

- add the real example
- prove it through the real cross-library build path
- lock the exact coverage and recommendation deltas in `xtask`
- refresh the generated proof surfaces that the repo treats as maintained truth

Do **not** do the shortcut version where someone drops in a spec file, eyeballs
`recommendation.latest.json`, and calls it done.

### Exit rule baked into scope

This run is intentionally single-use.

- If the repo lands the locked ranked result below, stop corpus expansion and
  move to a promotion-focused milestone.
- If the run fails to produce the locked ranked result, stop and re-plan.

At that point, the remaining blocker is still the structurally unknown-overlap
cluster, which more of the same corpus will not resolve under current code.

## Locked Decisions

| Decision | Lock |
|---|---|
| Add a sixth corpus source | **Rejected.** The existing five-source manifest is enough for this wedge. |
| Add regression fixtures instead of a real example | **Rejected.** One real example is the smallest honest lever. |
| Touch recommendation logic or schemas | **Rejected.** This run is evidence-only. |
| Chase `unknown_overlap_family` with more `money/round`-like hits | **Rejected.** Current code will not reinterpret that cluster through leverage alone. |
| Pre-commit to another corpus run after this one | **Rejected.** This run ends the corpus question one way or the other. |
| Begin M28 now | **Rejected.** The detached dry run shows a smaller wedge still changes the answer materially. |

## Exact File Contract

### Tracked source files

These are the only tracked source files M27.8 should need to change:

1. `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
2. `examples/crosslib-app/units/.gitignore`
3. `xtask/src/lib.rs`

### Generated proof artifacts

These are expected to change during acceptance reruns, but they are proof
outputs, not manually authored source:

1. `examples/crosslib-app/units/pricing/apply_tax.spec.passport.json` (new)
2. `examples/crosslib-app/units/pricing/apply_discount.spec.passport.json` (fresh proof)
3. `examples/shared-spec/units/money/round.spec.passport.json` (fresh proof)
4. `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
5. `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

### Explicit non-touch list

These files were checked and should not need source edits for M27.8:

- `semantic-families/corpus/rust-function.toml`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/**`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/**`
- `semantic-families/README.md`

## Locked Corpus Expansion Contract

Add exactly one new unit:

`examples/crosslib-app/units/pricing/apply_tax.unit.spec`

That unit must remain a maintained real example, not a fixture-shaped fake. It
must use the existing sibling-library helper path so it honestly pressures the
cross-library arithmetic unsupported shape.

Required authored shape:

```yaml
id: pricing/apply_tax
kind: function
spec_version: "0.3.0"
intent:
  why: Apply tax while importing the shared round helper from a sibling spec library.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
deps:
  - shared::money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let taxed = subtotal + subtotal * rate;
        round(taxed).max(Decimal::ZERO)
    }
local_tests:
  - id: happy_path
    expect: apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
```

This exact shape matters because it is the dry-run shape that clustered with
`unsupported_arithmetic_shape-2694b2baf65b`.

Required ignore rule update:

In `examples/crosslib-app/units/.gitignore`, add:

```gitignore
!pricing/apply_tax.spec.passport.json
```

Without that line, the new proof artifact stays hidden and the example is not
maintained repo truth.

## Locked Expected Output Deltas

These values were observed in the detached-worktree dry run and are now part of
the implementation contract.

### Coverage artifact deltas

The rerun must keep the same five source ids, in the same order:

1. `examples_ecommerce`
2. `m19_semantic_falsification_pack`
3. `m20_unsupported_truth_pack`
4. `examples_shared_spec`
5. `examples_crosslib_app`

The rerun must change source unit counts to:

- `6`
- `12`
- `9`
- `1`
- `2`

The rerun must produce:

- `function_coverage.total_units = 28`
- `function_coverage.promoted_family_units = 15`
- `function_coverage.supported_unpromoted_family_units = 0`
- `function_coverage.unsupported_function_units = 13`

The `family_coverage[]` list must remain unchanged in substance:

- still exactly three promoted families
- still `unit_count = 5` for each family
- still the same source ids as before

This run adds unsupported demand evidence only. It does not change supported
family coverage.

### Arithmetic cluster delta

`unsupported_arithmetic_shape-2694b2baf65b` must become:

- representative units:
  - `examples_crosslib_app::pricing/apply_discount`
  - `examples_crosslib_app::pricing/apply_tax`
  - `m20_unsupported_truth_pack::pricing/apply_tax_arithmetic_shape`
- `source_ids = ["examples_crosslib_app", "m20_unsupported_truth_pack"]`
- `real_example_hits = 2`
- `promotion_relevant_regression_hits = 1`
- `boundary_only_hits = 0`
- `overlap_family = "function.arithmetic_leaf.monotone_*"`
- `candidate_status = "rankable"`

### Unknown-overlap cluster delta

`unsupported_function_surface-e40675da6fa0` must stay materially unchanged:

- representative units:
  - `examples_ecommerce::money/round`
  - `examples_shared_spec::money/round`
  - `m20_unsupported_truth_pack::money/round`
- `real_example_hits = 2`
- `promotion_relevant_regression_hits = 1`
- `boundary_only_hits = 0`
- `overlap_family = "unknown"`
- `candidate_status = "rankable"`

That immobility is part of the point of this run.

### Recommendation artifact deltas

The rerun must produce:

- `recommendation_status = "ranked"`
- exactly `2` ranked candidates

First candidate:

- cluster id `unsupported_arithmetic_shape-2694b2baf65b`
- `promotion_readiness = "ready"`
- `hold_reasons = []`
- `real_example_hits = 2`
- `promotion_relevant_regression_hits = 1`
- `boundary_only_hits = 0`
- `total_units_in_cluster = 3`
- `difficulty.tier = "adjacent"`
- `confidence.level = "medium"`

Second candidate:

- cluster id `unsupported_function_surface-e40675da6fa0`
- `promotion_readiness = "hold"`
- `hold_reasons = ["unknown_overlap_family"]`
- `real_example_hits = 2`
- `promotion_relevant_regression_hits = 1`
- `boundary_only_hits = 0`
- `total_units_in_cluster = 3`
- `difficulty.tier = "hard"`
- `confidence.level = "low"`

The key behavioral claim is:

- one additional maintained real example is enough to make arithmetic demand
  promotion-worthy under the existing rules
- the repo still refuses to fake understanding of the `money/round` cluster

That is the whole milestone.

## Architecture

M27.8 is a source-unit-and-proof milestone. No runtime redesign is allowed.

```text
FIVE-SOURCE CORPUS FLOW, SAME MANIFEST
======================================
examples/crosslib-app/units/pricing/apply_tax.unit.spec   (new real example)
        |
        v
existing manifest source: examples_crosslib_app
        |
        v
xtask family coverage --format json
        |
        +--> same manifest
        +--> crosslib source unit count: 1 -> 2
        +--> coverage.latest.json
        |
        v
xtask family recommend --format json
        |
        +--> same recommendation policy
        +--> arithmetic cluster: held -> ready
        +--> recommendation.latest.json
        |
        v
xtask command-path lock in xtask/src/lib.rs
        |
        v
repo truth: ranked arithmetic candidate, unknown money/round still held
```

Architectural rules:

- the manifest stays frozen
- recommendation code stays frozen
- the new output must be explained entirely by one additional maintained real example
- cross-library proof order must build `shared-spec` first

## Implementation Plan

### Step 1 - Author the new cross-library real example

Create `examples/crosslib-app/units/pricing/apply_tax.unit.spec` with the exact
authored shape locked above.

Constraints:

- do not invent a new helper path
- do not change `examples/crosslib-app/spec.toml`
- keep the shape obviously arithmetic-like
- keep the unit a maintained example, not a synthetic fixture

### Step 2 - Whitelist the passport intentionally

Edit `examples/crosslib-app/units/.gitignore`.

Add the exact whitelist line for `apply_tax.spec.passport.json`.

Constraints:

- do not broaden the ignore file
- do not replace the existing `apply_discount` whitelist
- this is a one-passport change

### Step 3 - Update the locked `xtask` proof

Edit `xtask/src/lib.rs` in the existing family-analysis test area.

Required changes:

1. rename
   `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()`
   to
   `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held()`
2. keep the same deterministic-byte checks for stdout-versus-written-artifact and
   first-run-versus-second-run equality
3. update assertions from `no_strong_candidate` to the locked ranked result

That test must assert:

- coverage source ids and order stay unchanged
- coverage source unit counts are `6, 12, 9, 1, 2`
- recommendation status is `ranked`
- ranked candidate count is `2`
- first candidate is arithmetic and is `ready`
- first candidate has:
  - `hold_reasons == []`
  - `real_example_hits == 2`
  - `promotion_relevant_regression_hits == 1`
  - `total_units_in_cluster == 3`
  - `confidence.level == Medium`
- second candidate is the `money/round` cluster and remains held with exactly:
  - `hold_reasons == [UnknownOverlapFamily]`
  - `real_example_hits == 2`
  - `promotion_relevant_regression_hits == 1`
  - `total_units_in_cluster == 3`

Do not add a second parallel command-path lock for the same flow.

### Step 4 - Run the integrated proof loop

After tracked source edits are complete, run the proof loop in this exact order:

1. build the shared library output first
2. run exact-unit proof for the new cross-library unit
3. build the cross-library app output
4. run the cross-library crate tests
5. rerun coverage
6. rerun recommendation
7. validate both artifacts
8. run `xtask` tests
9. confirm the observed outputs still match the locked values above

## Code Quality

### DRY / reuse rule

Do not add new abstractions for this run.

Reuse:

- the existing five-source manifest
- the existing sibling-library example structure
- the existing command-path regression test

### Explicit over clever

Prefer one literal new spec file and literal `xtask` assertions over helper-heavy
test refactors. This is a one-unit corpus contract. Say the one unit plainly.

### Minimal-diff rule

If implementation starts reaching for helper refactors in `xtask` or shared spec
authoring utilities, stop. That is not this milestone.

### Build-order rule

The proof loop must build `examples/shared-spec/units` into
`examples/shared-crate/src/generated` before testing or building the new
cross-library unit.

This is not optional. Without that order, `crosslib-app` proof can fail with a
missing generated shared module.

## Test Review

100% coverage for the changed behavior means every new truth claim is locked in
tests or generated proof artifacts.

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] examples/crosslib-app/units/pricing/apply_tax.unit.spec
    |
    ├── [GAP] add one maintained arithmetic-like cross-library real example
    └── [GAP] prove the unit through exact-unit spec test

[+] examples/crosslib-app/units/.gitignore
    |
    └── [GAP] whitelist the new passport so proof becomes maintained truth

[+] xtask/src/lib.rs
    |
    └── recommendation command-path lock
        ├── [★★★ REGRESSION LOCK] stdout bytes == written artifact bytes
        ├── [★★★ REGRESSION LOCK] first run == second run bytes
        ├── [GAP] coverage source counts become 6 / 12 / 9 / 1 / 2
        ├── [GAP] recommendation status flips to ranked
        ├── [GAP] arithmetic cluster becomes first and ready
        └── [★★★ REGRESSION LOCK] money/round cluster stays held and unknown

[+] proof artifacts
    |
    ├── [GAP] new apply_tax passport is generated and tracked
    ├── [GAP] shared round passport is refreshed through required build order
    ├── [GAP] coverage.latest.json matches locked ranked values
    └── [GAP] recommendation.latest.json matches locked ranked values

─────────────────────────────────
COVERAGE: 0 pre-existing gaps allowed at exit
CRITICAL LOCKS: 4
REQUIRED PROOF SURFACES: 4
─────────────────────────────────
```

### Required proof commands

Run these commands in this order:

1. `cargo run -p spec-cli -- build examples/shared-spec/units --output examples/shared-crate/src/generated`
2. `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec`
3. `cargo run -p spec-cli -- build examples/crosslib-app/units --output examples/crosslib-app/src/generated`
4. `cargo test --manifest-path examples/crosslib-app/Cargo.toml`
5. `cargo xtask family coverage --format json`
6. `cargo xtask family recommend --format json`
7. `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
8. `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
9. `cargo test -p xtask -- --color never`

### Required assertions at acceptance

- `apply_tax.unit.spec` proves successfully
- the shared-library-first build order is preserved
- the crosslib crate still tests clean
- coverage stdout bytes equal the written `coverage.latest.json`
- recommendation stdout bytes equal the written `recommendation.latest.json`
- rerunning recommendation is byte-stable
- `xtask` test locks the ranked result and the held `money/round` truth

## Performance Review

No performance work is justified here.

This run adds one unit to one existing source. If this feels slow, the fix is
not new caching or code motion. The fix is discipline.

## Failure Modes Registry

| Surface | Failure | Test required | Error handling | User-visible outcome | Critical gap? |
|---|---|---|---|---|---|
| Source-unit shape | new unit classifies into the wrong cluster or a supported family | Yes | `xtask` lock failure | fake recommendation improvement or no improvement | **Yes if untested** |
| Passport visibility | new passport remains ignored | Yes | git diff plus acceptance loop | maintained example looks half-landed | No |
| Cross-library proof order | `crosslib-app` proof runs before shared generated output exists | Yes | command failure | missing-module error during proof | **Yes if untested** |
| Recommendation truth | arithmetic candidate stays held or status stays `no_strong_candidate` | Yes | `xtask` lock failure | milestone claim is false | **Yes if untested** |
| Scope discipline | manifest or policy changes creep in | Yes | code review | result becomes uninterpretable | No |

## Worktree Parallelization Strategy

This milestone has two implementation workstreams, but only one safe parallel
window.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Author new crosslib unit and whitelist passport | `examples/crosslib-app/units/` | — |
| Update command-path proof | `xtask/` | unit shape locked |
| Integrated proof loop and artifact regeneration | `examples/crosslib-app/`, `examples/shared-spec/`, `.semantic-family-artifacts/`, `xtask/` | both prior steps |

### Parallel lanes

- Lane A: `examples/crosslib-app/units/`
  - create `pricing/apply_tax.unit.spec`
  - update `.gitignore`
- Lane B: `xtask/`
  - update the existing command-path regression lock

### Execution order

1. Freeze the exact `apply_tax` unit shape first.
2. Launch Lane A and Lane B in parallel worktrees.
3. Merge both lanes into one integration branch.
4. Run the full acceptance loop only from the integrated branch.

### Conflict flags

- Lane A and Lane B are safe in parallel because they touch disjoint module
  roots: `examples/crosslib-app/units/` versus `xtask/`.
- Artifact regeneration is **not** parallel-safe. It must run only after both
  lanes merge because it writes shared derived proof surfaces.

## Acceptance Commands

Run from repo root:

```bash
cargo run -p spec-cli -- build examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- build examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo test --manifest-path examples/crosslib-app/Cargo.toml

tmpdir=$(mktemp -d)
cargo xtask family coverage --format json > "$tmpdir/coverage.stdout.json"
cmp -s "$tmpdir/coverage.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"

cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.json"
cmp -s "$tmpdir/recommend.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"

cargo test -p xtask -- --color never
```

Then confirm the generated JSON reflects the locked values in
`Locked Expected Output Deltas`.

## Done Criteria

M27.8 is done only when all of the following are true:

- exactly three tracked source files changed
- no explicit non-touch file required source edits
- the new `apply_tax` unit exists with the locked authored shape
- the new passport is intentionally whitelisted
- the command-path test in `xtask/src/lib.rs` locks the ranked result
- coverage and recommendation artifacts validate
- coverage and recommendation stdout bytes match the written artifact bytes
- arithmetic becomes the first ranked ready candidate
- `money/round` remains held for `unknown_overlap_family`
- the integrated rerun matches the detached-worktree dry-run contract

## Completion Summary

- Step 0: Scope Challenge — accepted as a one-unit real-example wedge
- Architecture Review structure baked into the execution contract
- Code Quality Review structure baked into the execution contract
- Test Review: exact command-path coverage and output locks required
- Performance Review: no optimization work justified
- NOT in scope: written
- What already exists: written
- Failure modes: five concrete traps called out
- Parallelization: 2 lanes, 1 safe parallel window, integration rerun serialized
- Stop rule after this run:
  - if the locked ranked truth lands, stop corpus work and move to a
    promotion-focused milestone
  - if the locked ranked truth does not land, also stop corpus work and re-plan

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 0 | — | — |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**VERDICT:** NO REVIEWS YET. This file now reads as the implementation contract,
but no separate review logs have been recorded yet.
