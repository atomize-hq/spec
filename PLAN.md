<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260501-203959.md -->
# M27.9 - Cross-Library Arithmetic Helper Alignment

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-01**

## Summary

M27.9 is not a new-family milestone.

It is a truth-alignment milestone that makes the runtime semantic reviewer,
public CLI truth surfaces, and M27 coverage / recommendation outputs agree with
what the promoted arithmetic packets already claim:

- zero-or-one helper dep is supported
- packet-local `money/round` and cross-library `shared::money/round` are the
  same semantic shape for arithmetic leaves
- control-flow arithmetic near-misses stay unsupported

If that alignment works, the current ready arithmetic candidate disappears for
the honest reason: it was fake unsupported pressure caused by mismatched truth
surfaces.

## Current Repo Truth

The refreshed pre-M27.9 baseline is:

- `function_coverage = 28 / 15 / 0 / 13`
- `recommendation_status = "ranked"`
- first ranked candidate:
  `unsupported_arithmetic_shape-2694b2baf65b`
  with `promotion_readiness = "ready"`
- second ranked candidate:
  `unsupported_function_surface-e40675da6fa0`
  with `promotion_readiness = "hold"` for `unknown_overlap_family`

That baseline is real as of the refreshed M27.8R rerun. M27.9 consumes it.

## Plan Authority

Primary decision inputs:

- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/family.toml`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/family.toml`
- `semantic-families/README.md`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_tax_arithmetic_shape.unit.spec`
- `examples/crosslib-app/units/pricing/apply_discount.unit.spec`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`

Latest design context:

- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260501-202640.md`

## Problem Statement

The promoted arithmetic packets already say cross-library helper-aware
arithmetic leaves should fit the family boundary.

The runtime and analysis layers still disagree.

Today, these real units still contribute to
`unsupported_arithmetic_shape-2694b2baf65b`:

- `examples_crosslib_app::pricing/apply_discount`
- `examples_crosslib_app::pricing/apply_tax`
- `m20_unsupported_truth_pack::pricing/apply_tax_arithmetic_shape`

That is the whole M27.9 problem.

The repo is currently inventing unsupported demand that the promoted packet
contract already covers.

## Scope Challenge

### What already exists

| Sub-problem | Existing code / truth | Decision |
|---|---|---|
| Arithmetic family boundary | `semantic-families/function.arithmetic_leaf.monotone_*/family.toml` already allows zero-or-one helper dep | Reuse. Do not mint a new family. |
| Local helper-dep semantic truth | `spec-core/src/semantic_review.rs` already routes local `money/round` arithmetic leaves into promoted families | Reuse as the target behavior. |
| Cross-library dep plumbing | `examples/crosslib-app/spec.toml` plus existing cross-library loader / validator / export coverage | Reuse. No library-loading redesign. |
| Public semantic truth surfaces | `spec-cli/tests/cli.rs` already locks `status`, `export`, and passport truth | Extend. Do not invent a second truth layer. |
| Coverage / recommendation projection | `xtask/src/lib.rs` already locks deterministic analysis outputs | Refresh only. No policy rewrite. |
| Current regression signal | `spec-cli/tests/fixtures/m20/unsupported_truth_pack/.../apply_tax_arithmetic_shape.unit.spec` | Repair. It becomes obsolete if M27.9 succeeds. |

### Minimum honest change

The minimum complete change set is:

1. align semantic review so `shared::money/round` is treated the same as local
   `money/round` for arithmetic leaves
2. repair the obsolete M20 unsupported arithmetic fixture truth
3. refresh CLI truth locks
4. refresh M27 coverage / recommendation locks
5. update `semantic-families/README.md` so docs match runtime truth

Anything smaller leaves known false unsupported pressure in the repo.

### Complexity rule

Target footprint:

- no new family packet
- no new artifact schema
- no new CLI command
- no new subsystem
- 5 to 7 tracked files touched is healthy

If implementation expands past 8 tracked files, or introduces a new packet
directory, stop and reduce scope.

### Search rule

- **[Layer 1]** Reuse the promoted arithmetic packet contract.
- **[Layer 1]** Reuse the existing cross-library dep system.
- **[EUREKA]** The ready arithmetic candidate is not evidence for a missing
  family. It is evidence that packet truth and runtime routing truth disagree.

### Completeness rule

Do the complete version now:

- classifier fix
- fixture repair
- CLI truth lock refresh
- xtask truth lock refresh
- docs update
- regression coverage

Do not land only the classifier tweak and leave the repo to silently keep stale
fixture and analysis truth.

## Premises

1. The refreshed M27.8R artifacts are the authoritative pre-M27.9 baseline.
2. The promoted arithmetic packets already express the intended helper-dep
   boundary.
3. `shared::money/round` should be semantically equivalent to local
   `money/round` for arithmetic leaves.
4. If premise 3 is false in code, the right move is to stop and document the
   narrower boundary, not to patch recommendation policy to fake the desired
   answer.

## Alternatives Rejected

### More corpus first

Rejected.

Stop Rule A is already satisfied. More corpus would hide a classifier mismatch
behind more examples.

### New helper-aware arithmetic family packet

Rejected.

That would duplicate a family boundary the promoted packets already claim.

### Recommendation-policy patch first

Rejected.

If semantic truth is wrong, fixing `xtask` heuristics first would create two
different interpretations of the same authored units.

## NOT in Scope

- another corpus-expansion run
  reason: M27.9 spends the current evidence, it does not gather more
- M28 shared-core extraction
  reason: still downstream of truthful next-family resolution
- recommendation policy or artifact schema changes
  reason: this milestone is about truth, not policy
- new arithmetic family ids or packet directories
  reason: first prove the current promoted families are sufficient
- `money/round` overlap-family resolution
  reason: that remains a separate blocker after fake arithmetic pressure is gone
- wrapper or chain3 family changes
  reason: no evidence says those boundaries are wrong here
- unrelated TODO debt
  reason: this milestone is already narrow and should stay that way

## Exact File Contract

### Tracked files expected to change

1. `spec-core/src/semantic_review.rs`
2. `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/`
3. `spec-cli/tests/cli.rs`
4. `xtask/src/lib.rs`
5. `semantic-families/README.md`

### Default fixture decision

The M20 pack should remain an unsupported truth pack.

That means the preferred repair is:

1. remove the supported arithmetic-shape claim from the current M20 fixture slot
2. replace it with a truthfully named unsupported arithmetic near-miss fixture
   in the same directory, ideally a control-flow near-miss
3. update consuming CLI assertions to match that repaired truth

Do not keep a supported arithmetic unit inside the unsupported pack just to
minimize diff count. That saves lines and loses honesty.

### Derived artifacts expected to refresh

- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

### File-by-file responsibility

| File | Responsibility | Must not happen |
|---|---|---|
| `spec-core/src/semantic_review.rs` | Treat cross-library helper refs as the same optional-helper arithmetic shape already supported locally, while preserving route precedence and unsupported near-miss behavior | Do not widen wrapper or chain3 routing. Do not change recommendation logic here. |
| `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/` | Keep the M20 pack truthfully unsupported after the classifier fix | Do not leave a knowingly supported arithmetic unit in the unsupported pack. |
| `spec-cli/tests/cli.rs` | Lock public `status`, `export`, and passport truth for cross-library arithmetic and the repaired M20 pack | Do not weaken existing unsupported-reason coverage. |
| `xtask/src/lib.rs` | Re-lock M27 coverage and recommendation outputs against the new truthful baseline | Do not change recommendation heuristics or schema. |
| `semantic-families/README.md` | State plainly that promoted arithmetic leaves already cover zero-or-one helper deps, including cross-library helper-aware examples after M27.9 | Do not turn README into milestone theory. |

## Architecture Contract

### Core rule

M27.9 fixes semantic truth first, then lets every downstream surface reproject
from that truth.

Never reverse that order.

### Data flow

```text
AUTHORED UNITS
==============
examples/crosslib-app/units/pricing/apply_discount.unit.spec
examples/crosslib-app/units/pricing/apply_tax.unit.spec
spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/*
        │
        ▼
spec-core::semantic_review
  - helper-dep normalization
  - arithmetic family routing
  - unsupported near-miss detection
        │
        ├── supported route
        │     -> monotone_down_nonnegative.v1
        │     -> monotone_up.v1
        │
        └── unsupported route
              -> unsupported.function.v1
        │
        ▼
spec-cli read surfaces
  - passport semantic_review
  - spec status
  - spec export
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
    ├── source of classifier truth
    ├── consumed by spec-cli public semantic-review surfaces
    └── consumed by xtask coverage and recommendation analysis

spec-cli/tests/cli.rs
    │
    └── locks what maintainers and agents actually see

xtask/src/lib.rs
    │
    └── locks roadmap-steering analysis outputs

semantic-families/README.md
    │
    └── documents the contract the code must honor
```

### Routing invariants

These are hard requirements, not suggestions:

- zero-or-one helper dep only
- no control flow for arithmetic leaf support
- route precedence remains:
  `chain3 -> wrapper -> monotone_down -> monotone_up`
- local and cross-library helper refs must collapse to the same arithmetic leaf
  interpretation
- unsupported arithmetic near-miss behavior must remain intact

## Implementation Plan

### Step 1 - Lock the boundary in tests before changing behavior

Make the current contract explicit in tests:

- promoted arithmetic packets allow zero-or-one helper dep
- local helper-dep arithmetic leaves are already supported
- cross-library helper-dep arithmetic leaves are supposed to align with them
- control-flow arithmetic leaves stay unsupported

Done means the tests clearly encode the intended boundary before the production
logic changes.

### Step 2 - Align semantic-review helper normalization

Update `spec-core/src/semantic_review.rs` so helper refs like
`shared::money/round` are treated as the same optional-helper arithmetic shape
as local `money/round`.

Preserve:

- zero-or-one helper-dep limit
- no-control-flow requirement
- route precedence
- existing unsupported near-miss diagnostics

Done means both cross-library arithmetic examples route to the same promoted
compatibility keys as their local equivalents.

### Step 3 - Repair the M20 unsupported truth pack

Repair the M20 fixture pack to stay truthfully unsupported after Step 2.

Default path:

1. replace the current supported arithmetic-shape example with a control-flow
   arithmetic near-miss
2. rename the fixture and unit id if needed so the unsupported meaning is
   truthful on its face
3. keep the pack's job unchanged: it remains an unsupported truth pack

Done means the M20 pack still exercises unsupported arithmetic truth, but no
longer lies about a shape that the repo now supports.

### Step 4 - Refresh CLI truth locks

Update `spec-cli/tests/cli.rs` so public semantic-review surfaces show the new
truth:

- cross-library arithmetic leaves project as supported
- the repaired M20 pack projects as unsupported for the right reason
- no stale unsupported-arithmetic expectation survives in `status`, `export`, or
  passport-facing reads

Done means maintainers and agents see the same truth the runtime classifier now
stores.

### Step 5 - Refresh M27 analysis locks

After semantic truth and M20 truth are fixed, rerun:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
```

Then update `xtask/src/lib.rs` to lock the refreshed output.

Done means the M27 analysis layer is consuming the repaired semantic truth, not
stale unsupported pressure.

### Step 6 - Refresh maintainer docs

Update `semantic-families/README.md` so the written contract matches the code:

- promoted arithmetic families already cover zero-or-one helper deps
- packet-local `money/round` exists to model that helper-aware shape
- cross-library helper-aware arithmetic examples align with that promoted
  boundary after M27.9

Done means the next maintainer does not rediscover fake roadmap pressure from
stale docs.

## Expected Output Delta

If the premise is correct, the locked outputs should change exactly like this.

### Coverage

- `function_coverage.total_units` stays `28`
- `function_coverage.promoted_family_units` rises from `15` to `18`
- `function_coverage.supported_unpromoted_family_units` stays `0`
- `function_coverage.unsupported_function_units` falls from `13` to `10`

### Recommendation

- top-level `recommendation_status` changes from `ranked` to
  `no_strong_candidate`
- `unsupported_arithmetic_shape-2694b2baf65b` disappears from ranked candidates
- `unsupported_function_surface-e40675da6fa0` remains visible and held for
  `unknown_overlap_family`

### Stop gate

If the delta is not exactly `+3 promoted / -3 unsupported`, stop.

Capture the per-unit semantic-review outputs for:

- `examples/crosslib-app/units/pricing/apply_discount.unit.spec`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- the repaired M20 arithmetic near-miss fixture

Then document the mismatch and do not patch recommendation policy to force the
desired answer.

## Test Review

### Framework and suites

Runtime: Rust workspace with `cargo test`

Suites that must move together:

- `spec-core` unit tests
- `spec-cli` integration tests
- `xtask` lock / analysis tests

### Coverage diagram

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
    │   └── [★★★ KEEP] stays unsupported
    │
    └── arithmetic route precedence
        └── [★★★ KEEP] chain3 -> wrapper -> monotone_down -> monotone_up

[+] spec-cli public truth surfaces
    │
    ├── passport semantic_review projection for cross-library arithmetic
    │   └── [GAP] add explicit supported compatibility-key assertions
    │
    ├── status/export projection for the repaired M20 pack
    │   └── [GAP] assert unsupported truth for the repaired near-miss
    │
    └── crosslib repo-root status/export rows
        └── [GAP] ensure both cross-library arithmetic units project truthfully

[+] xtask family analysis
    │
    ├── coverage latest snapshot
    │   └── [GAP] lock 28 / 18 / 0 / 10
    │
    ├── recommendation latest snapshot
    │   └── [GAP] lock no_strong_candidate
    │
    └── ranked candidates
        └── [GAP] arithmetic candidate removed, money/round remains held
```

### Required test additions

1. `spec-core/src/semantic_review.rs`
   - cross-library monotone-down regression test
   - cross-library monotone-up regression test
   - unchanged control-flow unsupported regression test
   - precedence non-shadowing regression test

2. `spec-cli/tests/cli.rs`
   - public supported semantic-review assertions for cross-library arithmetic
   - repaired M20 unsupported truth assertions for `status`, `export`, and
     passport projections
   - repo-root cross-library row assertions if visible unit set changes

3. `xtask/src/lib.rs`
   - coverage counts
   - recommendation status
   - ranked candidate identities and ordering

### Regression rule

This is an existing-behavior change.

Regression tests are mandatory. No shortcut.

### Test plan artifact

During implementation verification, regenerate an eng-review artifact at:

- `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-eng-review-test-plan-{timestamp}.md`

The artifact should tell QA exactly what to verify:

- cross-library arithmetic surfaces now classify as supported
- repaired M20 pack still classifies as unsupported
- M27 analysis no longer ranks the arithmetic candidate

## Failure Modes

| Codepath | Realistic failure | Test required? | Error handling exists? | Silent if missed? | Critical gap? |
|---|---|---|---|---|---|
| cross-library helper normalization | runtime still treats `shared::money/round` differently from local `money/round` | yes | no explicit runtime error, only wrong semantic truth | yes | **yes** |
| repaired M20 pack | fixture remains supported or uses misleading naming while living in unsupported pack | yes | test-only | yes, if not asserted | **yes** |
| CLI truth surfaces | semantic-review fix lands but `status` / `export` expectations remain stale | yes | test-only | no | no |
| xtask coverage refresh | coverage changes partially and leaves mixed promoted / unsupported truth | yes | lock diff catches | no | no |
| recommendation refresh | stale fixture truth keeps arithmetic candidate ranked | yes | lock diff catches | no | no |

Critical meaning here is simple: if the repo keeps silently steering roadmap
decisions from false unsupported pressure, the milestone failed.

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

Acceptance rule:

- all three test suites pass
- both analysis commands succeed
- both generated artifacts validate
- locked output delta matches the expected `+3 / -3`

If any one of those fails, stop and inspect semantic truth first.

## Worktree Parallelization Strategy

This milestone has a parallelization section because the review format expects
one, but the honest answer is: there is very little safe parallelism here.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Semantic truth alignment | `spec-core/src/` | — |
| Unsupported pack repair | `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/` | Semantic truth alignment |
| Public CLI truth locks | `spec-cli/tests/` | Semantic truth alignment, unsupported pack repair |
| Analysis lock refresh | `xtask/src/` | Semantic truth alignment, public CLI truth locks |
| Maintainer doc refresh | `semantic-families/` | Semantic truth alignment |

### Parallel lanes

- Lane A: semantic truth alignment
  `spec-core/src/`
- Lane B: unsupported pack repair -> public CLI truth locks
  starts after Lane A because it depends on final classifier behavior
- Lane C: maintainer doc refresh
  can start after Lane A, but should merge only after the exact boundary is
  proven in tests
- Lane D: analysis lock refresh
  starts after Lane B because it consumes final public truth

### Execution order

1. Launch Lane A first.
2. After Lane A is green enough to prove the boundary, launch Lane B and Lane C
   in parallel if you want two worktrees.
3. Merge Lane B and Lane C.
4. Run Lane D last.

### Conflict flags

- `spec-core/src/semantic_review.rs` is the root shared module. Do not split it
  across workers.
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/` and
  `spec-cli/tests/cli.rs` are logically one lane because fixture truth and
  public assertions move together.
- `xtask/src/lib.rs` must stay last. Refreshing it early just bakes stale truth
  into the final lock.

### Practical recommendation

If speed matters less than merge safety, run this milestone sequentially.

The only worthwhile parallel split is:

- Worker 1: Lane A
- Worker 2: Lane C after Lane A lands

Everything else is coupled enough that forced parallelism buys pain, not speed.

## Acceptance Criteria

M27.9 is done only when all of the following are true:

1. cross-library arithmetic leaves route to the same promoted family keys as
   their local-helper equivalents
2. unsupported arithmetic control-flow near-misses remain unsupported
3. the M20 unsupported pack remains truthfully unsupported
4. CLI `status`, `export`, and passport-facing semantic-review surfaces match
   the new runtime truth
5. M27 coverage moves from `28 / 15 / 0 / 13` to `28 / 18 / 0 / 10`
6. recommendation status changes from `ranked` to `no_strong_candidate`
7. the arithmetic ready candidate disappears
8. `unsupported_function_surface-e40675da6fa0` remains held for
   `unknown_overlap_family`
9. `semantic-families/README.md` matches the new runtime truth literally

## TODOS.md Impact

No new TODO belongs in `TODOS.md` yet.

If M27.9 fails, the follow-up is not "misc cleanup." It is a new explicit plan
for whichever boundary or policy mismatch survived.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Treat M27.9 as truth alignment, not a new family milestone | mechanical | explicit over clever | promoted packets already encode the target boundary | new packet first |
| 2 | CEO | Spend the current arithmetic evidence now instead of doing another corpus run | mechanical | bias toward action | Stop Rule A is already satisfied | more corpus by reflex |
| 3 | CEO | Keep `money/round` overlap-family resolution out of M27.9 | taste | pragmatic | it remains a separate blocker after fake arithmetic pressure is removed | multi-problem milestone |
| 4 | Eng | Make `spec-core/src/semantic_review.rs` the primary production change surface | mechanical | minimal diff | every downstream surface already consumes semantic-review truth | patch xtask heuristics first |
| 5 | Eng | Keep M20 as an unsupported pack and repair it truthfully instead of parking a supported unit inside it | mechanical | choose completeness | pack naming and pack truth must agree | supported unit in unsupported pack |
| 6 | Eng | Lock exact `+3 promoted / -3 unsupported` output deltas in tests | mechanical | systems over heroes | roadmap steering depends on deterministic artifact truth | manual interpretation after rerun |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | clean | narrowed M27.9 to truth alignment, rejected corpus rerun and new-family detour |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | issues_open | 10 gaps total, 2 critical truth gaps, exact post-M27.9 delta locked |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | no UI scope |

**UNRESOLVED:** 0  
**VERDICT:** CEO REVIEW CLEAN, ENG REVIEW OPEN. The plan is implementable. The milestone is only complete when semantic truth, M20 truth, and M27 analysis truth all agree.
