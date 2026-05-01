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
- current post-M27.5 recommendation artifact:
  `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- current corpus manifest:
  `semantic-families/corpus/rust-function.toml`

Repo truth checked while writing this plan:

- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- `examples/shared-spec/units`
- `examples/crosslib-app/units`

If any older draft, branch-local note, or stale M27.5 artifact disagrees with this
file, this file wins for M27.75 execution on `feat/m27`.

## Problem Statement

M27.5 did its job.

The recommendation engine is now honest enough to abstain. The current locked
three-source corpus yields:

- `recommendation_status = "no_strong_candidate"`
- a visible top candidate centered on `money/round`
- `promotion_readiness = "hold"`
- hold reasons that currently include thin-evidence signals

That means the next blocker is no longer policy quality. The next blocker is
corpus thinness.

The repo already contains two small, checked-in, function-only example sources
that are not in the current manifest:

- `examples/shared-spec/units`
- `examples/crosslib-app/units`

M27.75 exists to add exactly that missing real-example pressure without widening
scope into new ranking policy, new artifact contracts, or M28 portability work.

## Milestone Outcome

When M27.75 lands, the repo can truthfully claim:

- the Rust function corpus manifest expanded from `3` to `5` checked-in sources
- the new sources are real examples, not packet fixtures and not temporary test trees
- `family coverage` and `family recommend` remain deterministic
- the current `money/round` pressure becomes better-evidenced
- the engine still does not overclaim promotion readiness on unknown-overlap demand
- the branch has a stronger evidence base for the next explicit milestone choice

M27.75 does **not** claim:

- recommendation policy changed again
- the next family is definitely known
- non-function seam promotion became in scope
- packet fixtures now count toward recommendation leverage
- M28 started implicitly

## Scope

### In Scope

- expand `semantic-families/corpus/rust-function.toml` by exactly two repo-owned,
  checked-in example sources
- keep both new sources in `kind = "real_example"` and
  `counts_toward_recommendation = true`
- re-run the locked coverage and recommendation flow on the five-source manifest
- lock the new expected coverage and recommendation outcomes in `xtask` tests
- update maintainer docs that currently describe the manifest as a three-source corpus

### NOT In Scope

- changing recommendation readiness rules
- changing recommendation-analysis schema
- changing coverage schema
- adding packet fixtures to the corpus manifest
- adding `.m15` non-function seam sources to the function recommendation lane
- adding any new command surface
- starting M28 shared-core extraction
- folding explicit policy-choice work into this milestone

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Corpus source loading | `xtask/src/family/coverage.rs` `load_manifest_and_specs()` | Reuse as-is. M27.75 is manifest authoring, not loader redesign. |
| Recommendation rerun | `xtask/src/family/recommend.rs` | Reuse as-is. No policy changes in this milestone. |
| Artifact validation | `xtask/src/family/promotion_artifacts.rs` | Reuse unchanged. No schema change is allowed. |
| Current abstention proof | `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json` | Reuse as the before-state reference. |
| Candidate new sources | `examples/shared-spec/units`, `examples/crosslib-app/units` | Reuse directly. They already validate cleanly as function-only sources. |
| Cross-library example docs | `README.md`, `examples/crosslib-app/README.md` | Reuse as source-of-truth for why these examples are legitimate corpus inputs. |

### Minimum honest change

The smallest complete M27.75 diff is:

1. add exactly two checked-in real-example sources to the corpus manifest
2. lock the new five-source recommendation output in `xtask` tests
3. update corpus docs so they no longer claim the manifest has exactly three sources

Anything larger is scope creep. Anything smaller leaves the milestone under-specified.

### Complexity check

This milestone should stay under the smell threshold:

- expected touch set: `3` to `4` files
- expected new classes/services: `0`
- expected new commands: `0`

If implementation starts touching `coverage.rs`, `recommend.rs`, or
`promotion_artifacts.rs` for behavior changes, stop and re-plan. That means the
chosen sources were not actually a pure corpus-expansion wedge.

### Search check

- **[Layer 1]** Use the existing manifest and example structure. Do not invent a new
  corpus registry or source-discovery mechanism.
- **[Layer 1]** Use checked-in maintained examples before adding new synthetic fixtures.
- **[Layer 3]** The key insight from the dry run is that more real examples sharpen the
  explanation without changing policy. That is the right next move because it buys
  truth, not machinery.

### TODOS cross-reference

No existing `TODOS.md` item blocks this milestone directly.

M27.75 should create one follow-up only if the five-source rerun still leaves the
repo in `no_strong_candidate`: capture the next move as either an explicit policy
choice milestone or M28 kickoff decision. Do not silently absorb that decision here.

### Completeness check

Do the complete version now:

- manifest update
- deterministic rerun
- locked tests
- docs update

Do **not** do the shortcut version where the manifest changes and humans manually
inspect the output without test coverage. That would throw away the whole point of
the repo-owned recommendation surface.

### Distribution check

No new artifact type is introduced. Existing repo-local `xtask` commands and checked-in
analysis artifacts remain the distribution surface.

## Locked Decisions

| Decision | Lock |
|---|---|
| Add more than two new corpus sources | **Rejected.** M27.75 is intentionally a two-source wedge. |
| Change recommendation policy while expanding corpus | **Rejected.** M27.5 policy remains frozen for this milestone. |
| Change artifact schemas | **Rejected.** Coverage and recommendation schemas stay as they are post-M27.5. |
| Count packet fixtures toward recommendation leverage | **Rejected.** Only maintained real examples may increase real-example pressure here. |
| Expand into M28 portability prep | **Rejected.** M27.75 is evidence gathering, not shared-core extraction. |
| Touch runtime behavior outside manifest-driven output and locked tests | **Rejected.** If implementation needs behavior changes, stop and re-plan. |

## Locked Corpus Expansion Contract

M27.75 expands the manifest from these current sources:

- `examples/ecommerce/units`
- `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units`

to exactly these five sources, in this order:

1. `examples/ecommerce/units`
2. `spec-cli/tests/fixtures/m19/semantic_falsification_pack/units`
3. `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units`
4. `examples/shared-spec/units`
5. `examples/crosslib-app/units`

Required manifest entries:

- `id = "examples_shared_spec"`
- `path = "examples/shared-spec/units"`
- `kind = "real_example"`
- `counts_toward_recommendation = true`

- `id = "examples_crosslib_app"`
- `path = "examples/crosslib-app/units"`
- `kind = "real_example"`
- `counts_toward_recommendation = true`

The new source notes should describe them as maintained cross-library examples.

## Architecture

M27.75 is intentionally a manifest-and-proof milestone.

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
xtask tests lock the new five-source truth
```

The architectural rule is simple:

- manifest changes are the only behavior input change
- recommendation policy remains frozen
- any observed output delta must be explained by stronger corpus truth, not code-path drift

### Realistic production-style failure scenarios

| Codepath | Failure mode | Accounted for? |
|---|---|---|
| Manifest load | new source path is wrong or non-repo-relative | Yes, existing manifest path validation should fail loudly |
| Coverage projection | cross-library examples validate locally but fail in corpus semantic review | Yes, preflight validation commands are part of acceptance |
| Recommendation rerun | extra real-example pressure accidentally turns weak unknown-overlap demand back into `ranked` | Yes, locked regression assertions forbid that |
| Maintainer docs | manifest changes land but README still claims “exactly three sources” | Yes, docs update is explicitly in scope |

## Code Quality

### DRY / reuse rule

Do not add a new helper layer for “manifest expansion.”

Use the existing manifest file, existing coverage loader, existing recommendation
runner, and existing temp-workspace test helpers. This milestone is authored truth
plus locked proof, not a refactor opportunity.

### Explicit over clever

Prefer literal manifest entries and literal test assertions over computed source
lists or clever normalization. This is a five-source contract. Say the five sources plainly.

### Diagram maintenance

If any nearby ASCII diagrams or prose in `semantic-families/README.md` still describe the
manifest as “exactly three sources,” update them in the same change. Stale diagrams are a bug.

## Test Review

100% coverage for the changed behavior means every new manifest-driven branch and every new
expected artifact delta gets a locked test.

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] semantic-families/corpus/rust-function.toml
    |
    ├── [GAP] Add examples_shared_spec source entry
    └── [GAP] Add examples_crosslib_app source entry

[+] xtask/src/family/coverage.rs
    |
    ├── load_manifest_and_specs()
    │   ├── [★★★ TESTED by existing loader path] repo-relative source validation
    │   └── [GAP] five-source manifest rerun includes both new source ids
    |
    └── semantic review projection
        └── [GAP] new real-example sources contribute to leverage without code changes

[+] xtask/src/family/recommend.rs
    |
    └── existing M27.5 policy
        ├── [★★★ REGRESSION LOCK] top-level status stays non-ranked
        ├── [GAP] money/round real_example_hits becomes 2
        ├── [GAP] money/round hold_reasons collapse to [unknown_overlap_family]
        └── [GAP] second arithmetic-shape candidate remains visible and held

[+] semantic-families/README.md
    |
    └── [GAP] corpus-source section reflects five-source manifest, not three-source text

────────────────────────────────────────────────────────
COVERAGE TARGET: 100% of manifest-driven output deltas
  Critical regression locks: 4
  Docs truth surfaces: 1
  New runtime codepaths: 0
────────────────────────────────────────────────────────
```

### Required tests

Add or update `xtask/src/lib.rs` coverage in the existing family-analysis test area:

1. extend the locked recommendation workspace seed to support the two extra example sources
2. add a command-path regression that asserts the five-source rerun still writes stdout bytes
   equal to the persisted coverage and recommendation artifacts
3. assert recommendation status remains `no_strong_candidate`
4. assert the `money/round` candidate is still visible, still `hold`, now has
   `real_example_hits == 2`, and now has `hold_reasons == ["unknown_overlap_family"]`
5. assert the second arithmetic-shape candidate is visible and held with:
   `hold_reasons == ["thin_real_example_support", "thin_regression_support"]`,
   `real_example_hits == 1`, and `promotion_relevant_regression_hits == 1`
6. assert the coverage artifact `sources[]` order is exactly the five-source order locked above

### Test plan artifact

This milestone does not add a user-facing route or UI flow.

The QA-equivalent artifact is the command proof surface:

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `cargo test -p xtask recommendation_command_path -- --color never`

## Performance Review

This is a tiny corpus increase.

Expected impact:

- two additional checked-in source directories
- one extra real-example function in `examples/shared-spec`
- one extra real-example function in `examples/crosslib-app`

No new algorithmic work is justified. No caching work is justified. If the rerun becomes slow,
measure it first before inventing performance machinery.

## Failure Modes Registry

| Surface | Failure | Test required | Error handling | User-visible outcome | Critical gap? |
|---|---|---|---|---|---|
| Manifest authoring | path typo or wrong source kind | Yes | existing manifest validation | loud command failure | No |
| Cross-library example ingestion | `shared::` example loads differently under corpus review | Yes | existing coverage command failure | loud command failure | No |
| Recommendation truth | stronger corpus accidentally flips weak unknown-overlap demand to `ranked` | Yes | none, test is the protection | misleading roadmap output | **Yes if untested** |
| Docs truth | source count text drifts from manifest reality | Yes | none | maintainer confusion | No |

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Expand corpus manifest | `semantic-families/corpus/` | — |
| Lock five-source tests | `xtask/` | Expand corpus manifest contract frozen |
| Update maintainer docs | `semantic-families/`, `README.md` | Expand corpus manifest contract frozen |

### Parallel lanes

- Lane A: expand corpus manifest → update maintainer docs
- Lane B: lock five-source `xtask` tests

### Execution order

1. Freeze the exact two new source entries and the exact expected five-source output.
2. Launch Lane A and Lane B in parallel worktrees.
3. Merge both.
4. Run the full acceptance commands on the integrated branch.

### Conflict flags

No direct module overlap between `xtask/` and `semantic-families/corpus/` or docs.

The coordination risk is semantic, not textual: Lane B test assertions must match the
exact source ids, ordering, and expected output frozen by Lane A. Freeze those values first.

## Acceptance Commands

Run these from repo root:

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

Then assert, either in the locked test or with an explicit JSON check, that:

- coverage `sources[].id` equals:
  - `examples_ecommerce`
  - `m19_semantic_falsification_pack`
  - `m20_unsupported_truth_pack`
  - `examples_shared_spec`
  - `examples_crosslib_app`
- recommendation status is `no_strong_candidate`
- recommendation candidate count is `2`
- first candidate remains the `money/round` / `unsupported_function_surface` cluster
- first candidate has:
  - `promotion_readiness = "hold"`
  - `hold_reasons = ["unknown_overlap_family"]`
  - `real_example_hits = 2`
  - `promotion_relevant_regression_hits = 1`
- second candidate is the arithmetic-shape cluster with:
  - `promotion_readiness = "hold"`
  - `overlap_family = "function.arithmetic_leaf.monotone_*"`
  - `hold_reasons = ["thin_real_example_support", "thin_regression_support"]`
  - `real_example_hits = 1`
  - `promotion_relevant_regression_hits = 1`

## Completion Summary

- Step 0: Scope Challenge — accepted as a narrow manifest-and-proof milestone
- Architecture Review structure baked into the plan
- Code Quality Review structure baked into the plan
- Test Review: command-path coverage and exact output locks required
- Performance Review: no new optimization work justified
- NOT in scope: written
- What already exists: written
- Failure modes: one critical regression gap category identified if untested
- Parallelization: 2 lanes, parallel after contract freeze
- Next-step rule: if the five-source rerun still yields `no_strong_candidate`, stop and make the next milestone choice explicitly

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 0 | — | — |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**VERDICT:** NO REVIEWS YET — run `/autoplan` or the individual review skills after this root plan is accepted.
