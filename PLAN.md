<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260501-203959.md -->
# M27.9 - Cross-Library Arithmetic Helper Alignment

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-02**

## Summary

M27.75 and M27.8R did their job.

The refreshed repo truth is now:

- `function_coverage = 28 / 15 / 0 / 13`
- `recommendation_status = "ranked"`
- first ranked candidate:
  `unsupported_arithmetic_shape-2694b2baf65b`
  with `promotion_readiness = "ready"`
- second ranked candidate:
  `unsupported_function_surface-e40675da6fa0`
  with `promotion_readiness = "hold"` for `unknown_overlap_family`

M27.9 is the next honest move because the ready arithmetic candidate is not
pointing at a missing corpus slice. It is pointing at a mismatch between:

1. the already-promoted arithmetic packet contract, which says zero-or-one
   helper dep is part of the family shape, and
2. the current runtime semantic-review / corpus-analysis truth, which still
   treats real cross-library helper-dep arithmetic leaves as unsupported demand.

That is fake unsupported pressure. Fix it.

M27.9 will align cross-library helper-dep arithmetic leaves with the already
promoted arithmetic families, refresh the M20 unsupported truth pack where it is
now obsolete, and re-lock the M27 analysis outputs from that new truthful
baseline.

If that alignment attempt fails, stop before changing recommendation policy or
inventing a new family id.

## Plan Authority

Primary decision inputs:

- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/family.toml`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/family.toml`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec`
- `examples/crosslib-app/units/pricing/apply_discount.unit.spec`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`

Latest design context:

- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260501-202640.md`

## Problem Statement

The repo has already promoted two arithmetic leaf families:

- `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `function.arithmetic_leaf.monotone_up.v1`

Those packets already claim:

- `dep_min = 0`
- `dep_max = 1`
- `requires_supported_function_deps = false`
- summary text that explicitly says zero-or-one helper dep

The semantic reviewer also already proves the local helper-dep form as
supported:

- `apply_discount_function_spec()` with `deps = ["money/round"]`
- `apply_tax_function_spec()` with `deps = ["money/round"]`

But the five-source corpus still routes these real examples into
`unsupported_arithmetic_shape-2694b2baf65b`:

- `examples_crosslib_app::pricing/apply_discount`
- `examples_crosslib_app::pricing/apply_tax`
- `m20_unsupported_truth_pack::pricing/apply_tax_arithmetic_shape`

That means the current ready arithmetic candidate is likely not a new family
discovery. It is a classifier / projection mismatch.

M27.9 fixes that exact mismatch.

## Step 0 - Scope Challenge

### 0A. What already exists

| Sub-problem | Existing code / truth | Reuse decision |
|---|---|---|
| Arithmetic family contract | `semantic-families/function.arithmetic_leaf.monotone_*/family.toml` already allows zero-or-one helper dep | Reuse. Do not mint a duplicate family id first. |
| Supported local helper-dep classifier truth | `spec-core/src/semantic_review.rs` tests route local `money/round` arithmetic leaves to promoted families | Reuse as the canonical behavior target. |
| Cross-library dependency plumbing | `examples/crosslib-app/spec.toml`, `shared::money/round` loader/generator/validator coverage in `spec-cli/tests/cli.rs` | Reuse. Do not redesign library loading. |
| Corpus accounting and recommendation policy | `xtask/src/family/coverage.rs` and `xtask/src/family/recommend.rs` already produce deterministic outputs | Reuse. This milestone is not a recommendation-policy rewrite. |
| Current regression signal | `spec-cli/tests/fixtures/m20/unsupported_truth_pack/.../apply_tax_arithmetic_shape.unit.spec` | Rewrite or reclassify, because it stops being truthful if M27.9 succeeds. |

### 0B. Minimum honest change

The minimum honest change is:

1. align semantic review so `shared::money/round` counts as the same optional
   helper-dep shape already supported locally
2. update the obsolete M20 unsupported arithmetic fixture truth
3. refresh the locked M27 coverage / recommendation expectations in tests
4. document the new truthful interpretation boundary

Anything smaller leaves a fake unsupported cluster in the corpus.

Anything larger is scope creep.

### 0C. Complexity check

Target footprint:

- no new service or subsystem
- no new packet family
- no new artifact schema
- no new CLI command
- expected touched tracked files: **5 to 7**

If implementation starts touching more than 8 tracked files or adds a new family
packet directory, stop and reduce scope.

### 0D. Search check

- **[Layer 1]** Reuse the existing promoted arithmetic packet contract. The repo
  already spent the innovation token on those families.
- **[Layer 1]** Reuse the existing cross-library dep system. This is not a
  `spec.toml` loading milestone.
- **[EUREKA]** The ready arithmetic candidate is not evidence that the repo
  needs another arithmetic packet. It is evidence that packet truth and runtime
  routing truth are out of sync for cross-library helper deps.

### 0E. TODOS cross-reference

No existing `TODOS.md` entry blocks M27.9 directly.

Do not bundle unrelated TODO debt into this milestone.

### 0F. Completeness check

Do the complete version now:

- production fix
- fixture truth repair
- locked artifact expectation refresh
- regression coverage on semantic review, CLI surfaces, and xtask analysis

Do not land a partial semantic-review fix without the truth-pack and analysis
locks.

### 0G. Distribution check

No new artifact type ships to users.

This milestone remains repo-internal:

- semantic review truth
- corpus analysis truth
- maintainer documentation truth

## Premises

These are the premises this plan assumes:

1. The refreshed `M27.8R` artifact truth is authoritative for the pre-M27.9
   baseline.
2. The existing promoted arithmetic packets already express the intended helper
   dep policy.
3. A real `shared::money/round` helper dep should be semantically equivalent to
   the already-supported local `money/round` helper dep for arithmetic leaves.
4. If premise 3 is false in code, the right response is to stop and write a
   narrower family-boundary policy, not to silently over-widen the classifier.

## Alternatives Considered

### Approach A - Another Corpus Run

Rejected.

The tracker already says Stop Rule A is met. More corpus would make the repo
work around a classifier mismatch instead of fixing it.

### Approach B - New Helper-Aware Arithmetic Family Packet

Rejected as the first move.

The existing promoted arithmetic packets already claim the relevant helper-dep
boundary. Minting a new packet before testing classifier parity would duplicate
family surface and spend an innovation token badly.

### Approach C - Align Runtime Semantic Review With Existing Packet Truth

Chosen.

This is the smallest change that explains the current ready arithmetic
candidate, removes fake unsupported pressure if the premise is correct, and
preserves recommendation-policy integrity.

## Dream State

```text
CURRENT
=======
promoted arithmetic packets say:
  zero-or-one helper dep is supported
          │
          ├── local helper form routes as supported
          └── cross-library helper form appears as unsupported arithmetic demand

M27.9
=====
align classifier + corpus truth with promoted packet contract
          │
          ├── cross-library helper arithmetic routes to promoted leaves
          ├── obsolete M20 unsupported arithmetic fixture is repaired
          └── coverage/recommendation outputs are re-locked from that truth

POST-M27.9 IDEAL
================
ready arithmetic candidate is gone because it has been absorbed into support
          │
          ├── coverage promoted units increase
          ├── unsupported units decrease
          └── remaining blocker is whatever unsupported pressure survives honestly
```

## NOT in Scope

- another corpus-expansion run
  reason: this milestone spends the current evidence, it does not gather more
- M28 shared-core extraction
  reason: still downstream of truthful next-family / next-boundary resolution
- recommendation policy or schema changes
  reason: M27.5 already locked that surface
- new arithmetic family ids or packet directories
  reason: first prove the current promoted family contract is sufficient
- `money/round` overlap-family resolution
  reason: that remains the next blocker only after arithmetic fake pressure is removed
- wrapper / chain3 family changes
  reason: no evidence says those boundaries are wrong here

## Exact File Contract

### Production files expected to change

1. `spec-core/src/semantic_review.rs`
2. `semantic-families/README.md`

### Test / fixture / lock files expected to change

3. `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec`
4. `spec-cli/tests/cli.rs`
5. `xtask/src/lib.rs`

### Derived artifacts expected to change during proof

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

### File-by-file responsibility

| File | Responsibility | Must not happen |
|---|---|---|
| `spec-core/src/semantic_review.rs` | normalize optional helper-dep arithmetic semantics so cross-library helper refs can route the same way as local helper refs, while preserving current routing precedence and current unsupported near-miss behavior | do not widen wrapper or chain3 routing, do not change unrelated unsupported reason behavior |
| `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec` | stop claiming unsupported truth for a shape that becomes supported, either by replacing it with a still-unsupported arithmetic near-miss or by moving it into the supported expectation matrix | do not leave a knowingly false unsupported fixture behind |
| `spec-cli/tests/cli.rs` | lock public `status` / `export` / passport-facing semantic truth for the reclassified arithmetic shape and the repaired M20 pack | do not weaken current public unsupported reason-matrix coverage |
| `xtask/src/lib.rs` | refresh locked M27 coverage/recommendation expectations to the new truthful baseline | do not change recommendation heuristics or artifact schema here |
| `semantic-families/README.md` | state plainly that promoted arithmetic packets already cover zero-or-one helper deps, including the cross-library helper-aware interpretation after M27.9 | do not turn README into milestone theory |

## Architecture Contract

### Core rule

M27.9 is a truth-alignment milestone, not a recommendation-policy milestone.

The implementation must follow this order:

1. fix semantic classification truth
2. repair obsolete regression / corpus fixtures
3. refresh analysis outputs
4. lock the new outputs in tests

Never reverse that order.

### Data flow

```text
AUTHORED UNIT
============
examples/crosslib-app/units/pricing/apply_discount.unit.spec
examples/crosslib-app/units/pricing/apply_tax.unit.spec
spec-cli/tests/fixtures/m20/.../apply_tax_arithmetic_shape.unit.spec
        │
        ▼
spec-core::semantic_review
  - dep shape classification
  - helper-dep normalization
  - family route selection
        │
        ├── supported arithmetic leaf route
        │     -> compatibility_key = monotone_down / monotone_up
        │
        └── unsupported.function.v1
              -> unsupported_arithmetic_shape
        │
        ▼
passport / status / export read surfaces
        │
        ▼
xtask family coverage
        │
        ▼
xtask family recommend
```

### Dependency graph

```text
spec-core/src/semantic_review.rs
    │
    ├── drives semantic review stored in passports
    ├── drives status/export public surfaces through spec-cli
    └── drives M27 coverage clustering through xtask coverage

spec-cli/tests/cli.rs
    │
    └── locks public-facing semantic truth

xtask/src/lib.rs
    │
    └── locks analysis truth for coverage + recommendation

semantic-families/README.md
    │
    └── documents the promoted family contract the code is supposed to honor
```

### Security / blast-radius assessment

Blast radius is moderate and contained:

- semantic review classification for arithmetic leaves
- passport/status/export projections that surface semantic review
- family coverage / recommendation analysis

No auth, network, filesystem-escape, or deployment risk changes.

The failure mode is semantic misclassification, not user-data loss.

## Implementation Steps

### Step 1 - Lock the semantic boundary before editing code

Read and restate in code comments / tests what is already true:

- promoted arithmetic packets allow zero-or-one helper dep
- local helper form is supported
- unsupported control-flow arithmetic near-miss must stay unsupported

This is the contract basis. No implementation before that basis is visible in
the touched tests.

### Step 2 - Align helper-dep classification in semantic review

Update `spec-core/src/semantic_review.rs` so the arithmetic leaf route treats
cross-library helper refs like `shared::money/round` as the same optional helper
shape category already recognized for local helper refs like `money/round`.

Important:

- preserve zero-or-one helper dep limit
- preserve straight-line-only restriction
- preserve no-branching / no-loops restriction
- preserve current route precedence:
  `chain3 -> wrapper -> monotone_down -> monotone_up`
- preserve current unsupported outcomes for control-flow near-miss fixtures

### Step 3 - Repair obsolete M20 unsupported truth

If the helper-aware arithmetic shape becomes supported, the current M20 fixture
`pricing/apply_tax_arithmetic_shape` can no longer remain in the unsupported
matrix.

Repair options are intentionally narrow:

1. replace it with a still-unsupported arithmetic near-miss that exercises the
   same family neighborhood truthfully, or
2. move it into the supported expectation matrix and add a new unsupported
   arithmetic fixture separately

Choose the option with the smallest diff that keeps the public M20 reason
matrix truthful.

### Step 4 - Refresh public CLI truth locks

Update `spec-cli/tests/cli.rs` so:

- cross-library semantic review surfaces reflect supported monotone arithmetic
  compatibility keys where appropriate
- the whole-pack M20 status/export expectation matrix remains truthful
- no stale unsupported-arithmetic expectation survives

### Step 5 - Refresh M27 analysis locks

After semantic truth and fixtures are repaired, rerun:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
```

Then update `xtask/src/lib.rs` locked expectations to the new truthful baseline.

### Step 6 - Refresh documentation

Update `semantic-families/README.md` so it plainly matches the runtime truth:

- promoted arithmetic packets cover zero-or-one helper deps
- packet-local `money/round` exists to model helper-aware shape truth
- cross-library helper-aware arithmetic examples now align with that promoted
  boundary

## Expected Output Delta

If premise 3 is correct, M27.9 should produce this high-confidence delta:

### Coverage

- `function_coverage.total_units` stays `28`
- `function_coverage.promoted_family_units` rises from `15` to `18`
- `function_coverage.supported_unpromoted_family_units` stays `0`
- `function_coverage.unsupported_function_units` falls from `13` to `10`

Reason:

- the current arithmetic cluster has `total_units_in_cluster = 3`
- if that cluster is truly adjacent because of a classifier mismatch, all three
  units should leave unsupported coverage

### Recommendation

- top-level `recommendation_status` should fall from `ranked` to
  `no_strong_candidate`
- `unsupported_arithmetic_shape-2694b2baf65b` should disappear from ranked
  candidates
- `unsupported_function_surface-e40675da6fa0` should remain visible and held
  for `unknown_overlap_family`

That is not regression. That is the point.

It means the repo consumed the fake ready arithmetic demand by aligning it with
already-promoted support.

### Stop gate if outputs disagree

If the arithmetic cluster does not collapse exactly as expected:

1. stop after semantic-review evidence capture
2. diff the per-unit semantic review outputs for:
   - both crosslib arithmetic examples
   - the M20 arithmetic-shape fixture
3. write the observed mismatch into the plan follow-up
4. do **not** patch recommendation policy to force the desired answer

## Architecture Review

### Findings

`[P1] (confidence: 9/10) spec-core/src/semantic_review.rs — the plan must treat
cross-library helper-dep normalization as the primary change surface, not
coverage/recommendation heuristics.`

Recommendation:

- keep the fix in semantic review first
- let coverage and recommendation reproject from that truth

Why:

- explicit over clever
- minimal diff
- avoids building a second interpretation layer in xtask

### Error and Rescue Registry

| Failure | Where it appears | Rescue |
|---|---|---|
| Cross-library helper ref still counts as unsupported dep topology | `spec-core/src/semantic_review.rs` tests, crosslib CLI semantic review | stop and inspect helper-dep normalization, do not adjust coverage heuristics |
| M20 unsupported pack becomes internally contradictory | `spec-cli/tests/cli.rs` unsupported truth matrix | repair fixture truth before refreshing xtask locks |
| Coverage numbers change in a way not equal to `+3 / -3` | `xtask/src/lib.rs` locked analysis assertions | capture unit-level semantic reviews and stop |
| Recommendation remains `ranked` for arithmetic after semantic fix | `xtask/src/lib.rs` | inspect surviving unsupported units before touching recommend policy |

## Code Quality Review

### Findings

`[P2] (confidence: 8/10) semantic-families/README.md + code tests — packet
contract text and runtime classifier behavior must stay literally aligned, or
the repo will keep rediscovering fake unsupported demand.`

Recommendation:

- document the exact helper-dep boundary where the code changes
- keep docs update in the same milestone

Why:

- systems over heroes
- stale docs here create future false-positive roadmap pressure

### DRY rule

Do not duplicate helper-dep interpretation logic in both:

- semantic review, and
- xtask coverage clustering

The clustering layer must continue consuming semantic-review truth, not
re-deriving arithmetic support from raw dep shapes.

## Test Review

### Test framework

Rust workspace with `cargo test`.

Primary suites touched:

- `spec-core` unit tests
- `spec-cli` integration tests
- `xtask` integration / lock tests

### Code path coverage diagram

```text
CODE PATH COVERAGE
==================
[+] spec-core/src/semantic_review.rs
    │
    ├── arithmetic leaf with local helper dep
    │   └── [★★★ KEEP] supported monotone route stays unchanged
    │
    ├── arithmetic leaf with cross-library helper dep
    │   └── [GAP][REGRESSION] must route the same as local helper dep
    │
    ├── arithmetic near-miss with control flow
    │   └── [★★★ KEEP] stays unsupported_control_flow
    │
    └── arithmetic leaf routing precedence
        └── [★★★ KEEP] chain3 -> wrapper -> monotone_down -> monotone_up

[+] spec-cli public surfaces
    │
    ├── passport semantic_review projection for repaired M20 fixture
    │   └── [GAP] update whole-pack truth matrix
    │
    ├── status/export semantic_review projection for cross-library arithmetic
    │   └── [GAP] add explicit supported compatibility-key assertions
    │
    └── cross-library repo-root status health
        └── [GAP] ensure both crosslib arithmetic units are represented truthfully

[+] xtask family analysis
    │
    ├── coverage latest snapshot
    │   └── [GAP] lock new totals: 28 / 18 / 0 / 10
    │
    ├── recommendation latest snapshot
    │   └── [GAP] lock new top-level status: no_strong_candidate
    │
    └── ranked candidates ordering
        └── [GAP] arithmetic candidate removed, money/round remains held
```

### User-flow / maintainer-flow coverage

```text
MAINTAINER FLOW COVERAGE
========================
[+] run semantic review on arithmetic helper shape
    ├── local helper dep -> supported
    ├── cross-library helper dep -> [GAP] newly supported
    └── control-flow near-miss -> unsupported

[+] run whole-pack M20 status/export
    └── [GAP] pack still truthful after arithmetic support expansion

[+] run five-source M27 analysis
    ├── coverage reprojects from semantic truth
    ├── recommendation reprojects from coverage truth
    └── [GAP] no stale ranked arithmetic candidate survives
```

### Required test additions

1. `spec-core/src/semantic_review.rs`
   Add regression tests for:
   - cross-library helper-dep monotone-down route
   - cross-library helper-dep monotone-up route
   - unchanged unsupported control-flow near-miss route

2. `spec-cli/tests/cli.rs`
   Add or update tests for:
   - repaired M20 whole-pack public truth matrix
   - cross-library supported semantic-review compatibility keys in public
     surfaces
   - repo-root status/export expectations if crosslib unit count / identities
     changed

3. `xtask/src/lib.rs`
   Update locked artifact expectations for:
   - coverage counts
   - ranked candidate count and identities
   - recommendation status

### Test plan artifact

The implementation must also write this artifact during review / verification:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-eng-review-test-plan-20260501-204533.md`

### Regression rule

This milestone changes existing behavior. That makes regression tests mandatory.

No AskUserQuestion. No shortcut.

## Failure Modes

| Codepath | Realistic production failure | Test? | Error handling? | Silent? | Critical gap? |
|---|---|---|---|---|---|
| cross-library arithmetic leaf semantic review | still classified unsupported because helper dep normalization only handles local ids | must add | no explicit runtime error, just wrong semantic truth | yes | **yes** |
| repaired M20 fixture matrix | whole-pack status/export still expects unsupported arithmetic shape | must add | test-only | no | no |
| coverage snapshot refresh | promoted count changes partially, leaving mixed truth | must add | lock diff catches | no | no |
| recommendation refresh | arithmetic candidate remains ranked because a stale fixture still feeds unsupported cluster | must add | lock diff catches | no | no |

Critical gap:

- If cross-library arithmetic leaves still project as unsupported and there is no
  direct regression test, the repo will silently continue steering roadmap
  decisions from false pressure. That is a critical gap.

## Performance Review

No material runtime performance risk is introduced.

Why:

- no new database or network path
- no new algorithmic fan-out in coverage/recommendation
- semantic review classification cost should remain same-order

The only performance rule:

- do not add duplicate whole-workspace semantic-review passes just to special-case
  cross-library helper deps

Keep the change inside the existing review path.

## Proof Loop

Run in this order:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo test -p xtask -- --color never

cargo xtask family coverage --format json
cargo xtask family recommend --format json

cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
```

If any lock disagrees with the expected post-M27.9 baseline, stop and capture
the mismatch before editing recommendation policy.

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Semantic boundary alignment | `spec-core/src/`, `semantic-families/` | — |
| Public surface regression updates | `spec-cli/tests/` | Semantic boundary alignment |
| Analysis lock refresh | `xtask/src/` | Semantic boundary alignment, public surface regression updates |

### Parallel lanes

- Lane A: semantic boundary alignment -> README contract refresh
  sequential, shared `spec-core/src/` / `semantic-families/`
- Lane B: public CLI regression updates
  starts after Lane A, shared semantic truth assumptions
- Lane C: xtask analysis lock refresh
  starts after Lane A and B, because it consumes final semantic + fixture truth

### Execution order

Launch sequence is mostly sequential:

1. Lane A
2. Lane B
3. Lane C

This is effectively a single-lane implementation with one dependent test lane
and one final lock-refresh lane.

### Conflict flags

- `spec-core/src/semantic_review.rs` is the root shared module. Do not parallelize
  multiple workers against it.
- `spec-cli/tests/cli.rs` and `xtask/src/lib.rs` can be separate only after the
  semantic truth is stable.

## Completion Summary

- Step 0: Scope Challenge — scope accepted as-is, no corpus expansion, no new family packet
- Architecture Review: 2 issues found, both resolved by keeping the fix in semantic review and repairing obsolete fixture truth
- Code Quality Review: 1 issue found, resolved by requiring docs + code parity
- Test Review: diagram produced, 7 concrete gaps identified
- Performance Review: 0 material issues found
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 0 proposed, no new deferred work belongs in this milestone yet
- Failure modes: 1 critical gap flagged
- Outside voice: available but skipped for this focused plan-eng-review pass
- Parallelization: 3 lanes, 0 truly parallel, 3 sequential
- Lake Score: 4/4 recommendations chose the complete option

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Treat M27.9 as alignment of existing promoted arithmetic truth, not new family invention | mechanical | explicit over clever | packet contracts already encode zero-or-one helper dep | new packet first |
| 2 | CEO | Spend the current arithmetic evidence on promotion/policy work, not corpus run 1 | mechanical | bias toward action | tracker Stop Rule A is already met | more corpus by default |
| 3 | CEO | Keep `money/round` resolution out of M27.9 | taste | pragmatic | it is the next blocker only after arithmetic fake pressure is removed | multi-problem milestone |
| 4 | Eng | Make `spec-core/src/semantic_review.rs` the primary production change surface | mechanical | minimal diff | downstream systems already consume semantic-review truth | coverage/recommendation heuristics patch |
| 5 | Eng | Repair the obsolete M20 unsupported arithmetic fixture in the same milestone | mechanical | choose completeness | otherwise the repo keeps a known false regression pack | postpone fixture cleanup |
| 6 | Eng | Lock exact post-M27.9 output deltas in xtask tests | mechanical | systems over heroes | roadmap steering depends on deterministic artifact truth | manual interpretation after rerun |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | clean | narrowed M27.9 to classifier/promotion alignment, rejected corpus rerun and new packet detour |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | issues_open | 10 issues/gaps total, 1 critical silent-truth gap, exact post-M27.9 artifact delta locked |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | no UI scope |

**UNRESOLVED:** 0  
**VERDICT:** CEO REVIEW CLEAN, ENG REVIEW OPEN — the plan is implementable, but implementation is only complete when the semantic-truth gap is closed and the new artifact baseline is re-locked.
