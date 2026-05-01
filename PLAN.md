# M27.75 - Small Corpus Expansion Pack

Status: **implementation contract**
Base branch: **main**
Working branch: **feat/m27**
Last rewritten: **2026-05-01**

## Plan Authority

This file is the authoritative M27.75 execution plan for `feat/m27`.

Primary sources:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `docs/m27_5_recommendation_quality_plan_v0.1.md`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `semantic-families/corpus/rust-function.toml`

Repo truth checked while writing this plan:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- `examples/shared-spec/units/money/round.unit.spec`
- `examples/crosslib-app/units/pricing/apply_discount.unit.spec`
- `examples/crosslib-app/README.md`
- `README.md`

This plan also includes one disposable-workspace proof run on 2026-05-01 with the
two proposed manifest entries added locally outside the repo. That dry run is the
source of the exact five-source expectations locked below.

If any older draft, branch-local note, or stale M27.5 artifact disagrees with this
file, this file wins for M27.75 execution on `feat/m27`.

## Problem Statement

M27.5 fixed the honesty problem. It did not fix the evidence problem.

Today the locked three-source corpus is truthful enough to abstain:

- `recommendation_status = "no_strong_candidate"`
- the top visible candidate is the `money/round` unsupported-function-surface cluster
- that candidate is still `promotion_readiness = "hold"`
- the current hold reasons still reflect thin support

That means the next blocker is corpus thinness, not policy quality.

The repo already contains two maintained, checked-in, function-only example sources
that are not yet part of the Rust function manifest:

- `examples/shared-spec/units`
- `examples/crosslib-app/units`

M27.75 exists to add exactly those two real examples, re-run the existing analysis
unchanged, and lock the new truth in tests and docs. Nothing more.

## Milestone Outcome

When M27.75 lands, the repo can truthfully claim:

- the Rust function corpus manifest expands from `3` sources to `5`
- both new sources are repo-owned maintained examples and count as `real_example`
- `cargo xtask family coverage --format json` remains deterministic
- `cargo xtask family recommend --format json` remains deterministic
- the `money/round` unsupported cluster gains a second real-example hit
- a second held arithmetic-shape candidate becomes visible
- the engine still does not overclaim promotion readiness
- the next milestone choice has a stronger evidence base than M27.5

M27.75 does **not** claim:

- recommendation policy changed
- a new family is now promotion-ready
- packet fixtures count as recommendation corpus
- non-function seam promotion entered scope
- M28 shared-core extraction started

## Scope

### In Scope

- expand `semantic-families/corpus/rust-function.toml` by exactly two sources
- keep both new sources as `kind = "real_example"` and
  `counts_toward_recommendation = true`
- re-run coverage and recommendation on the five-source manifest with no policy change
- lock the new five-source truth in `xtask/src/lib.rs`
- update maintainer docs that still describe the manifest as a three-source corpus

### NOT In Scope

- changing recommendation readiness rules
- changing coverage or recommendation schema
- changing `xtask/src/family/coverage.rs` behavior
- changing `xtask/src/family/recommend.rs` behavior
- counting semantic-family packet fixtures toward recommendation leverage
- adding any new CLI surface
- starting M28 portability or shared-core work
- sneaking policy-choice work into this corpus-expansion milestone

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Corpus source loading | `xtask/src/family/coverage.rs` `load_manifest_and_specs()` | Reuse as-is. This is manifest authoring, not loader redesign. |
| Recommendation rerun | `xtask/src/family/recommend.rs` | Reuse as-is. Policy stays frozen. |
| Recommendation artifact validation | `xtask/src/family/promotion_artifacts.rs` | Reuse as-is. No schema work is allowed here. |
| Current abstention baseline | `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json` | Reuse as the before-state reference. |
| Candidate new real examples | `examples/shared-spec/units`, `examples/crosslib-app/units` | Reuse directly. They already exist and are maintained. |
| Current command-path lock | `xtask/src/lib.rs` `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()` | Extend, do not replace. |
| Current maintainer docs | `semantic-families/README.md` | Update the stale three-source statement here and nowhere else unless another stale claim is found. |

### Minimum honest change

The smallest complete M27.75 diff is:

1. add exactly two manifest entries
2. update the existing locked `xtask` recommendation command-path test and seed helper
3. update `semantic-families/README.md` so it no longer lies about the manifest size

Anything larger is scope creep. Anything smaller leaves the milestone under-specified.

### Complexity check

This stays below the smell threshold:

- tracked source files expected to change: `3`
- generated proof artifacts expected to change locally during acceptance: `2`
- new classes/services: `0`
- new commands: `0`

If implementation starts changing coverage or recommendation logic, stop and re-plan.
That would mean the examples were not a pure corpus-expansion wedge after all.

### Search check

- **[Layer 1]** Use the existing manifest format. Do not invent discovery or auto-registration.
- **[Layer 1]** Use the maintained example trees already in the repo before inventing new fixtures.
- **[Layer 3]** The right move is more truthful evidence, not more ranking machinery.

### TODOS cross-reference

No existing `TODOS.md` entry blocks this milestone directly.

M27.75 creates exactly one possible follow-up: if the five-source rerun still yields
`no_strong_candidate`, capture the next move explicitly as either a policy-choice
milestone or an M28 kickoff decision. Do not absorb that choice here.

### Completeness check

Do the complete version now:

- manifest update
- exact test lock update
- docs truth update
- end-to-end acceptance rerun

Do **not** do the shortcut version where maintainers change the manifest and eyeball
the output manually. This repo's whole value is locked truthful machine output.

### Distribution check

No new artifact type is introduced. The delivery surface remains the existing repo,
`xtask` commands, and generated analysis artifacts under
`.semantic-family-artifacts/family-promotion/analysis/`.

## Locked Decisions

| Decision | Lock |
|---|---|
| Add more than two new corpus sources | **Rejected.** This is intentionally a two-source wedge. |
| Change recommendation policy while expanding corpus | **Rejected.** M27.5 policy stays frozen. |
| Change artifact schemas | **Rejected.** M27.75 is not schema work. |
| Count packet fixtures toward recommendation leverage | **Rejected.** Only maintained real examples count here. |
| Change runtime behavior outside manifest-driven output and test locks | **Rejected.** If behavior changes are needed, this was mis-scoped. |
| Expand into M28 portability prep | **Rejected.** Evidence gathering only. |

## Locked Corpus Expansion Contract

M27.75 expands the manifest from these current source ids:

1. `examples_ecommerce`
2. `m19_semantic_falsification_pack`
3. `m20_unsupported_truth_pack`

to exactly these five source ids, in this order:

1. `examples_ecommerce`
2. `m19_semantic_falsification_pack`
3. `m20_unsupported_truth_pack`
4. `examples_shared_spec`
5. `examples_crosslib_app`

Required new manifest entries:

```toml
[[sources]]
id = "examples_shared_spec"
path = "examples/shared-spec/units"
kind = "real_example"
counts_toward_recommendation = true
note = "Maintained sibling-library helper example."

[[sources]]
id = "examples_crosslib_app"
path = "examples/crosslib-app/units"
kind = "real_example"
counts_toward_recommendation = true
note = "Maintained cross-library app example."
```

The notes should describe these sources as maintained real examples, not fixtures,
spikes, or temporary corpus padding.

## Exact File Contract

### Tracked source files

These are the only tracked source files M27.75 should need to change:

1. `semantic-families/corpus/rust-function.toml`
2. `xtask/src/lib.rs`
3. `semantic-families/README.md`

### Generated proof artifacts

These are expected to change during acceptance reruns, but they are proof outputs,
not authored source:

1. `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
2. `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

### Explicit non-touch list

These files were checked and should not need source edits for M27.75:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `README.md`
- `examples/crosslib-app/README.md`

## Locked Expected Output Deltas

These values were observed in a disposable five-source proof run and are now part of
the implementation contract.

### Coverage artifact deltas

The five-source rerun must produce:

- `sources[].id` in this order:
  - `examples_ecommerce`
  - `m19_semantic_falsification_pack`
  - `m20_unsupported_truth_pack`
  - `examples_shared_spec`
  - `examples_crosslib_app`
- `sources[].unit_count` in this order:
  - `6`
  - `12`
  - `9`
  - `1`
  - `1`
- `function_coverage.total_units = 27`
- `function_coverage.promoted_family_units = 15`
- `function_coverage.supported_unpromoted_family_units = 0`
- `function_coverage.unsupported_function_units = 12`
- `non_function_coverage` remains:
  - `total_units = 2`
  - `supported_sum_units = 1`
  - `supported_data_units = 1`
  - `other_units = 0`

The `family_coverage[]` list must remain unchanged in substance:

- still exactly three promoted families
- still `unit_count = 5` for each family
- still the same source ids as before

That is important. The two new sources increase unsupported-demand evidence. They do
not add new supported-family coverage.

### Unsupported cluster deltas

The five-source rerun must surface these two rankable clusters:

1. `unsupported_function_surface-e40675da6fa0`
   - representative units:
     - `examples_ecommerce::money/round`
     - `examples_shared_spec::money/round`
     - `m20_unsupported_truth_pack::money/round`
   - `overlap_family = "unknown"`
   - `real_example_hits = 2`
   - `promotion_relevant_regression_hits = 1`
   - `boundary_only_hits = 0`
   - `candidate_status = "rankable"`

2. `unsupported_arithmetic_shape-2694b2baf65b`
   - representative units:
     - `examples_crosslib_app::pricing/apply_discount`
     - `m20_unsupported_truth_pack::pricing/apply_tax_arithmetic_shape`
   - `overlap_family = "function.arithmetic_leaf.monotone_*"`
   - `real_example_hits = 1`
   - `promotion_relevant_regression_hits = 1`
   - `boundary_only_hits = 0`
   - `candidate_status = "rankable"`

### Recommendation artifact deltas

The five-source rerun must produce:

- `recommendation_status = "no_strong_candidate"`
- exactly `2` ranked candidates

First candidate:

- cluster id `unsupported_function_surface-e40675da6fa0`
- `promotion_readiness = "hold"`
- `hold_reasons = ["unknown_overlap_family"]`
- `real_example_hits = 2`
- `promotion_relevant_regression_hits = 1`
- `boundary_only_hits = 0`
- `total_units_in_cluster = 3`

Second candidate:

- cluster id `unsupported_arithmetic_shape-2694b2baf65b`
- `promotion_readiness = "hold"`
- `hold_reasons = ["thin_real_example_support", "thin_regression_support"]`
- `real_example_hits = 1`
- `promotion_relevant_regression_hits = 1`
- `boundary_only_hits = 0`
- `total_units_in_cluster = 2`

The key behavioral claim is:

- stronger real-example pressure makes the top candidate cleaner
- the engine still refuses to overclaim readiness

That is the whole milestone.

## Architecture

M27.75 is a manifest-and-proof milestone. No runtime redesign is allowed.

```text
FIVE-SOURCE CORPUS FLOW
=======================
semantic-families/corpus/rust-function.toml
        |
        v
xtask family coverage --format json
        |
        +--> load_manifest_and_specs()
        +--> semantic review on each source unit
        +--> coverage.latest.json
        |
        v
xtask family recommend --format json
        |
        +--> reuse existing M27.5 policy unchanged
        +--> recommendation.latest.json
        |
        v
xtask tests lock the five-source truth
```

Architectural rules:

- manifest changes are the only behavior input change
- recommendation policy remains frozen
- any output delta must be explained by stronger corpus truth, not code-path drift
- generated analysis artifacts remain proof surfaces, not authored source

## Implementation Plan

### Step 1 - Expand the manifest

Edit `semantic-families/corpus/rust-function.toml` only.

Add the two locked entries in the locked order above.

Do not reorder existing sources. Do not rename existing source ids. Do not touch
schema version or target lane.

### Step 2 - Extend the locked `xtask` proof

Edit `xtask/src/lib.rs` in the existing family-analysis test area.

Required changes:

1. update `seed_locked_recommendation_workspace(...)` so it copies:
   - `examples/shared-spec/units`
   - `examples/crosslib-app/units`
2. extend `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()`
   so it asserts the five-source truth exactly

That test must now assert:

- coverage source ids and order
- coverage source unit counts and order
- recommendation candidate count is `2`
- top-level status remains `no_strong_candidate`
- the `money/round` cluster now has `real_example_hits == 2`
- the `money/round` candidate now has exactly `hold_reasons == ["unknown_overlap_family"]`
- the arithmetic-shape candidate exists and is held with exactly:
  - `hold_reasons == ["thin_real_example_support", "thin_regression_support"]`
  - `real_example_hits == 1`
  - `promotion_relevant_regression_hits == 1`

Do not create a second parallel test for the same command path unless the existing
test becomes unreadable. Prefer extending the current lock.

### Step 3 - Update maintainer docs

Edit `semantic-families/README.md` only where it currently lies about the corpus.

Required doc changes:

- update `Corpus Source Kinds`
- replace the stale sentence saying the locked M27 manifest contains exactly three sources
- list the new five-source manifest explicitly
- make it clear that `examples/shared-spec/units` and `examples/crosslib-app/units`
  are maintained `real_example` sources

No root `README.md` change is required unless a new stale three-source claim is introduced.

### Step 4 - Run the integrated proof loop

After the tracked source edits are complete:

1. rerun coverage
2. rerun recommendation
3. validate both artifacts
4. run `xtask` tests
5. confirm the observed outputs still match the locked values above

## Code Quality

### DRY / reuse rule

Do not add helper abstractions for manifest expansion.

Use:

- the existing manifest file
- the existing command-path test
- the existing temp-workspace seed helper

This milestone is authored truth plus locked proof. It is not a refactor invitation.

### Explicit over clever

Prefer literal manifest entries and literal assertions over computed source lists or
indirection. This is a five-source contract. Say the five sources plainly.

### Diagram maintenance

If any nearby prose or diagrams in `semantic-families/README.md` still describe the
manifest as three sources, update them in the same change. Stale diagrams are a bug.

## Test Review

100% coverage for the changed behavior means every manifest-driven output delta is
locked in tests.

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] semantic-families/corpus/rust-function.toml
    |
    ├── [GAP] Add examples_shared_spec source entry
    └── [GAP] Add examples_crosslib_app source entry

[+] xtask/src/lib.rs
    |
    ├── seed_locked_recommendation_workspace()
    │   ├── [GAP] Copy examples/shared-spec/units
    │   └── [GAP] Copy examples/crosslib-app/units
    |
    └── recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()
        ├── [★★★ REGRESSION LOCK] status stays no_strong_candidate
        ├── [GAP] coverage source ids expand from 3 to 5 in exact order
        ├── [GAP] money/round cluster real_example_hits becomes 2
        ├── [GAP] money/round hold_reasons collapse to [unknown_overlap_family]
        ├── [GAP] second arithmetic-shape candidate becomes visible and held
        └── [GAP] candidate count becomes exactly 2

[+] semantic-families/README.md
    |
    └── [GAP] corpus-source section reflects five-source manifest truth

────────────────────────────────────────────────────────
COVERAGE TARGET: 100% of manifest-driven output deltas
  Critical regression locks: 5
  Docs truth surfaces: 1
  New runtime codepaths: 0
────────────────────────────────────────────────────────
```

### Required tests

Use the existing command-path regression test in `xtask/src/lib.rs`.

Required assertions:

1. source ids equal the five locked ids in order
2. source unit counts equal `6, 12, 9, 1, 1`
3. recommendation status remains `no_strong_candidate`
4. ranked candidate count equals `2`
5. the `money/round` candidate remains first and held
6. that first candidate now has:
   - `hold_reasons == ["unknown_overlap_family"]`
   - `real_example_hits == 2`
   - `promotion_relevant_regression_hits == 1`
7. the arithmetic candidate is present and has:
   - `overlap_family == "function.arithmetic_leaf.monotone_*"`
   - `hold_reasons == ["thin_real_example_support", "thin_regression_support"]`
   - `real_example_hits == 1`
   - `promotion_relevant_regression_hits == 1`

### Test plan artifact

This milestone does not add a UI route or user-facing flow.

The proof surface is command-path plus artifact truth:

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `cargo test -p xtask recommendation_command_path -- --color never`

## Performance Review

This is a tiny corpus increase:

- one new real-example helper unit
- one new real-example cross-library function unit

No new algorithmic work is justified. No caching work is justified. If the rerun
looks slower, measure first. Do not spend an innovation token on performance theater.

## Failure Modes Registry

| Surface | Failure | Test required | Error handling | User-visible outcome | Critical gap? |
|---|---|---|---|---|---|
| Manifest authoring | wrong path, wrong id, or wrong source kind | Yes | existing manifest validation | loud command failure | No |
| Temp-workspace seed | test forgets to copy one new example tree | Yes | test failure | false green on old corpus shape | **Yes if untested** |
| Recommendation truth | stronger corpus accidentally flips weak unknown-overlap demand to `ranked` | Yes | none, test is the protection | misleading roadmap output | **Yes if untested** |
| Recommendation ordering | second arithmetic candidate exists but does not stay visible in output | Yes | none, test is the protection | hidden evidence change | **Yes if untested** |
| Docs truth | README still says three sources | Yes | none | maintainer confusion | No |

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Freeze manifest contract | `semantic-families/corpus/` | — |
| Extend command-path proof | `xtask/` | Freeze manifest contract |
| Update maintainer docs | `semantic-families/` | Freeze manifest contract |

### Parallel lanes

- Lane A: freeze manifest contract → update maintainer docs
- Lane B: extend `xtask` command-path proof

### Execution order

1. Freeze the exact two manifest entries and the exact five-source expectations.
2. Launch Lane A and Lane B in parallel worktrees.
3. Merge both.
4. Run the full acceptance loop on the integrated branch.

### Conflict flags

No direct module overlap exists between `xtask/` and the manifest/docs lane.

The real risk is semantic coordination:

- Lane B assertions must match the exact ids, notes, and output contract frozen by Lane A.
- Do not start Lane B before the manifest contract is frozen.

## Acceptance Commands

Run from repo root:

```bash
tmpdir=$(mktemp -d)
cargo xtask family coverage --format json > "$tmpdir/coverage.stdout.json"
cmp -s "$tmpdir/coverage.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/coverage.latest.json"

cargo xtask family recommend --format json > "$tmpdir/recommend.stdout.json"
cmp -s "$tmpdir/recommend.stdout.json" ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"
cargo xtask family validate-artifact ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json"

cargo test -p xtask -- --color never
```

Then confirm the generated JSON reflects the locked values in `Locked Expected Output Deltas`.

## Completion Summary

- Step 0: Scope Challenge — accepted as a narrow manifest-and-proof milestone
- Architecture Review structure baked into the execution contract
- Code Quality Review structure baked into the execution contract
- Test Review: exact command-path coverage and output locks required
- Performance Review: no optimization work justified
- NOT in scope: written
- What already exists: written
- Failure modes: three critical regression categories if left untested
- Parallelization: 2 lanes, parallel after manifest contract freeze
- Next-step rule: if the five-source rerun still yields `no_strong_candidate`, stop and make the next milestone choice explicitly

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 0 | — | — |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**VERDICT:** NO REVIEWS YET — this file now embeds the eng-review execution structure, but no separate review logs have been recorded yet.
