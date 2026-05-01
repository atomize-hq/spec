<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m27-autoplan-restore-20260430-212234.md -->
# M27.5 - Recommendation Quality Hardening

Status: **implementation contract**
Base branch: **main**
Working branch: **feat/m27**
Last rewritten: **2026-04-30**

## Plan Authority

This file is the authoritative M27.5 execution plan for `feat/m27`.

Primary sources:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `docs/m27_5_recommendation_quality_plan_v0.1.md`
- approved design doc:
  `~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m27-design-20260430-173836.md`

Repo truth checked while rewriting this plan:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/paths.rs`
- `xtask/src/lib.rs`
- current locked corpus manifest:
  `semantic-families/corpus/rust-function.toml`

If any older draft, branch-local note, or superseded M27 plan disagrees with this
file, this file wins for M27.5 execution on `feat/m27`.

## Problem Statement

M27 already landed the discovery engine.

The repo can now:

- account for checked-in corpus coverage
- cluster unsupported function pressure
- rank rankable candidates
- emit deterministic machine-readable analysis artifacts

The live failure is narrower.

The current recommendation analysis can still emit
`recommendation_status = "ranked"` even when the top candidate is weak:

- `overlap_family = "unknown"`
- `difficulty.tier = "hard"`
- `real_example_hits = 1`
- `promotion_relevant_regression_hits = 1`

That is mechanically consistent with current M27 code. It is not good enough for
roadmap steering.

M27.5 exists to add a trust gate on top of M27 discovery so the repo can say:
"I found pressure" without overclaiming "promote this next."

## Milestone Outcome

When M27.5 lands, the repo can truthfully claim:

- `family recommend` remains deterministic
- weak candidates stay visible in analysis output
- promotion-worthiness is stricter than raw discoverability
- recommendation artifacts explain why a candidate is on hold
- the current locked corpus can honestly yield `no_strong_candidate`

M27.5 does **not** claim:

- corpus expansion is solved
- the ranking policy is globally optimal
- the next family is definitely known
- M28 shared-core extraction should start immediately

## Scope

### In Scope

- tighten recommendation-analysis policy only
- keep the existing M27 command surface unchanged
- add a promotion-readiness layer above current discovery output
- make hold reasons machine-readable in recommendation analysis
- upgrade recommendation-analysis validation to schema version `2`
- add deterministic regression coverage for the current `money/round` failure
- update maintainer docs so `ranked` has a sharper meaning

### NOT In Scope

- adding new corpus sources
- changing coverage accounting semantics
- changing unsupported-function fingerprint generation
- ranking non-function seams
- promoting the next family packet
- beginning M28 portability work
- adding a dashboard, UI, or separate binary
- changing release/distribution infrastructure

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Coverage collection | `xtask/src/family/coverage.rs` | Reuse as-is. M27.5 must not reimplement or broaden coverage accounting. |
| Recommendation generation | `xtask/src/family/recommend.rs` | Reuse the existing projection path, but split it into explicit discovery and readiness phases. |
| Artifact validation | `xtask/src/family/promotion_artifacts.rs` | Reuse the validator surface and extend only recommendation-analysis rules. |
| Artifact paths | `xtask/src/family/paths.rs` | Reuse unchanged. The path contract is already correct. |
| CLI coverage and artifact tests | `xtask/src/lib.rs` | Reuse existing test harness style and add focused M27.5 regressions. |
| Maintainer docs | `semantic-families/README.md` | Reuse and tighten wording around what `ranked` now means. |

### Minimum honest change

The smallest complete M27.5 diff is:

1. keep M27 discovery intact
2. enrich recommendation-analysis schema with readiness and hold reasons
3. adjudicate each candidate as `ready` or `hold`
4. ensure held candidates remain visible but cannot falsely drive `ranked`
5. prove the current `money/round` case demotes to `no_strong_candidate`

Anything less is just a nicer explanation of the same weak behavior.

### Complexity check

This plan stays under the smell line if implemented correctly.

- expected production files touched: `3-5`
- expected new modules: `0`
- expected new crates/binaries/services: `0`

Target blast radius:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- optional tiny helper edits in `xtask/src/family/coverage.rs` only if borrow or
  sorting ergonomics demand them

That is "engineered enough." Not a rewrite, not a hack.

### Search check

- **[Layer 1]** Keep the existing `xtask family coverage` and `xtask family recommend`
  command surfaces. No new command buys enough value to justify the cognitive cost.
- **[Layer 1]** Reuse the current artifact path and validation entrypoint. Changing
  paths here would burn review cycles for zero product gain.
- **[Layer 3]** The real insight is not "ranking must get smarter." It is "ranking
  needs an abstention layer." That is why readiness adjudication sits *after*
  discovery instead of replacing it.

### TODOS cross-reference

`TODOS.md` contains many repo-level follow-ups, but none block M27.5 directly.
The one related future thread is already acknowledged by the milestone itself:
if the tightened engine still yields `no_strong_candidate`, the next honest move
is corpus expansion or an explicit policy decision, not automatic M28.

### Completeness check

M27.5 should do the complete version now:

- schema contract
- validation contract
- policy logic
- deterministic regression coverage
- maintainer docs

Deferring tests or docs here would save minutes and cost confidence. Not worth it.

### Distribution check

No new distributable artifact is introduced. The existing repo-owned `xtask` CLI
remains the delivery vehicle, so there is no new CI/CD or packaging work in scope.

## Locked Decisions

| Decision | Lock |
|---|---|
| Introduce a new `family analyze` command | **Rejected.** Command surface stays unchanged. |
| Change coverage artifact schema while changing recommendation policy | **Rejected.** Coverage remains M27 truth. |
| Bump one shared artifact schema constant for every artifact type | **Rejected.** Recommendation-analysis must version independently so coverage semantics stay unchanged. |
| Hide weak candidates entirely | **Rejected.** Visibility and promotion-worthiness are different jobs. |
| Keep current leverage-first sorting even when a candidate is on hold | **Rejected.** Ready candidates must sort ahead of held candidates or the trust gate is ambiguous. |
| Fold corpus expansion into M27.5 | **Rejected.** If corpus is too thin, that becomes the *next* milestone or decision. |

## Architecture

### Dependency graph

```text
                    +----------------------------------+
                    | semantic-families/corpus/*.toml |
                    +----------------+-----------------+
                                     |
                                     v
                         +-----------+------------+
                         | coverage::collect_*    |
                         | M27 truth, unchanged   |
                         +-----------+------------+
                                     |
                                     v
                       +-------------+--------------+
                       | recommend.rs                |
                       | layer 1: discovered cand.   |
                       | layer 2: readiness policy   |
                       +-------------+--------------+
                                     |
                   +-----------------+------------------+
                   |                                    |
                   v                                    v
    +--------------+----------------+    +--------------+----------------+
    | recommendation.latest.json    |    | validate-artifact             |
    | schema_version = 2            |    | promotion_artifacts.rs rules  |
    +--------------+----------------+    +--------------+----------------+
                   |                                    |
                   +-----------------+------------------+
                                     |
                                     v
                           +---------+---------+
                           | xtask/src/lib.rs  |
                           | regression tests  |
                           +-------------------+
```

### Data flow

```text
locked corpus
   |
   v
coverage artifact (M27 semantics, unchanged)
   |
   v
discoverable rankable clusters
   |
   v
promotion-readiness adjudication
   |                    \
   |                     \__ hold reasons[] assigned
   v
ready-first sorting
   |
   v
top-level recommendation_status
   |
   v
stdout + recommendation.latest.json (schema v2)
```

### File responsibilities

| File | Responsibility | Must not do |
|---|---|---|
| `xtask/src/family/recommend.rs` | implement two-layer recommendation flow | do not reimplement coverage accounting |
| `xtask/src/family/promotion_artifacts.rs` | define schema v2 fields and validation rules | do not widen unrelated artifact contracts |
| `xtask/src/lib.rs` | add focused regression and validator tests | do not silently rewrite unrelated M26/M27 assertions |
| `semantic-families/README.md` | explain the new meaning of `ranked` | do not turn README into milestone theory |

## Implementation Plan

### Step 1 - Split schema versioning by artifact type

Current problem:

- `promotion_artifacts.rs` uses one shared `SCHEMA_VERSION = 1`
- M27.5 only wants recommendation-analysis to move to schema `2`
- coverage semantics must stay unchanged

Required change:

- replace the single shared schema constant with artifact-specific constants
- keep:
  - family recommendation artifact at current schema
  - coverage artifact at current schema
  - promotion execution/blocker artifacts at current schema
- bump only `FamilyRecommendationAnalysisArtifact` to schema `2`

Recommendation:

- do this explicitly with small named constants near the artifact definitions
- do **not** build a generic version registry abstraction

Minimal diff beats cleverness here.

### Step 2 - Extend recommendation-analysis schema

Add to `RecommendationCandidateEntry`:

- `promotion_readiness`
- `hold_reasons[]`

New enums in `promotion_artifacts.rs`:

- `PromotionReadiness`
  - `ready`
  - `hold`
- `HoldReason`
  - `unknown_overlap_family`
  - `hard_difficulty`
  - `thin_real_example_support`
  - `thin_regression_support`

Validation rules:

- `promotion_readiness = "ready"` requires `hold_reasons == []`
- `promotion_readiness = "hold"` requires `hold_reasons` to be non-empty
- `recommendation_status = "ranked"` requires at least one ranked candidate and
  the first candidate must be `ready`

### Step 3 - Make recommendation logic explicitly two-layer

`recommend.rs` should become conceptually:

1. **Discovery projection**
   - map `UnsupportedClusterEntry` -> candidate entry with leverage, difficulty,
     confidence, overlap, rationale
2. **Readiness adjudication**
   - assign `promotion_readiness`
   - assign `hold_reasons[]`
3. **Ordering + status**
   - sort ready candidates before hold candidates
   - keep current leverage ordering inside each readiness bucket
   - derive top-level `recommendation_status`

Implementation guidance:

- keep this inside `recommend.rs`
- prefer a couple of explicit helper functions over a new module
- acceptable helpers:
  - `project_candidate(...)`
  - `adjudicate_readiness(...)`
  - `recommendation_status_for(...)`

### Step 4 - Lock policy rules

#### Promotion-readiness rules

A candidate is forced to `hold` when any of these are true:

- `overlap_family == "unknown"`
- `difficulty.tier == "hard"` and `real_example_hits < 2`
- `real_example_hits == 0`
- `real_example_hits == 1` and `promotion_relevant_regression_hits < 3`
- `promotion_relevant_regression_hits <= 1` and `real_example_hits <= 1`

Mapped hold reasons:

- unknown overlap -> `unknown_overlap_family`
- hard difficulty with insufficient real examples -> `hard_difficulty`
- weak real-example bar -> `thin_real_example_support`
- weak regression bar -> `thin_regression_support`

Deliberate exclusion from the fresh root plan:

- `single_source_pressure` is deferred from M27.5 because the current serialized
  coverage artifact does not expose `promotion_relevant_source_count`, and this plan
  explicitly keeps coverage semantics unchanged. Reintroduce it only in a later
  milestone that intentionally widens the coverage contract.

#### Confidence rules

`high` only when:

- `real_example_hits >= 3`
- `overlap_family != "unknown"`

`medium` only when:

- `real_example_hits >= 2` and `overlap_family != "unknown"`
- or `real_example_hits == 1`, `promotion_relevant_regression_hits >= 3`,
  `difficulty.tier != "hard"`, and `overlap_family != "unknown"`

Otherwise:

- `low`

#### Recommendation status rules

Evaluate in this order:

1. `ranked` when the first sorted candidate satisfies the shared ranked
   predicate:
   `promotion_readiness == "ready"` and `confidence.level` is `medium` or `high`
2. `insufficient_real_corpus` when every discoverable candidate is `hold` and
   every candidate has `real_example_hits == 0`
3. `no_strong_candidate` when at least one discoverable candidate exists, every
   candidate is `hold`, and at least one candidate has `real_example_hits > 0`

### Step 5 - Keep coverage semantics unchanged

Coverage is not the bug.

Do **not**:

- change `FamilyCoverageArtifact` schema
- add readiness fields to coverage
- reclassify unsupported clusters in `coverage.rs`
- widen the corpus manifest or source labeling rules

The only acceptable `coverage.rs` edits are tiny helper extractions or comments
that make the recommendation layer easier to implement without changing output.

### Step 6 - Add deterministic regression coverage

`xtask/src/lib.rs` must gain focused tests for:

- unknown-overlap hard candidate with one real example -> `hold`
- no discoverable candidates -> `insufficient_real_corpus`
- discoverable-but-held candidates -> `no_strong_candidate`
- known-overlap adjacent candidate with strong evidence -> `ranked`
- validator accepts schema v2 recommendation-analysis artifact
- current locked corpus no longer returns `ranked`

The last one is the real regression gate. If that test is missing, the milestone
is not done.

### Step 7 - Update maintainer docs

`semantic-families/README.md` must say, plainly:

- `ranked` means promotion-worthy next-family pressure
- visible held candidates are not errors
- `no_strong_candidate` is an honest outcome

That wording matters because the product surface here is the artifact contract plus
the maintainer interpretation of it.

## Test Review

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] xtask/src/family/promotion_artifacts.rs
    |
    ├── FamilyRecommendationAnalysisArtifact::validate()
    │   ├── [GAP] schema_version == 2 accepted for recommendation analysis
    │   ├── [GAP] ranked requires the first candidate to be `ready` with
    │   │       confidence `medium` or `high`
    │   ├── [GAP] hold requires non-empty hold_reasons[]
    │   └── [GAP] ready requires empty hold_reasons[]
    |
    └── RecommendationCandidateEntry::validate()
        ├── [GAP] promotion_readiness + hold_reasons[] consistency
        └── [GAP] duplicate/empty hold reasons rejected or normalized deliberately

[+] xtask/src/family/recommend.rs
    |
    ├── discovery projection
    │   └── [★★ TESTED by existing M27 path] rankable clusters still project into candidates
    ├── readiness adjudication
    │   ├── [GAP] unknown overlap -> hold
    │   ├── [GAP] hard + thin real support -> hold
    │   └── [GAP] strong adjacent known-overlap candidate -> ready
    ├── ready-first sorting
    │   └── [GAP] ready candidate sorts ahead of stronger-but-held candidate
    └── recommendation_status_for(...)
        ├── [GAP] ready + medium/high confidence -> ranked
        ├── [GAP] all hold + some real examples -> no_strong_candidate
        └── [GAP] all hold + zero real examples -> insufficient_real_corpus

[+] locked corpus rerun
    |
    └── [GAP] [→E2E] current three-source corpus returns no_strong_candidate and
               money/round cluster remains visible with hold reasons
```

### User-flow style coverage for maintainer outcomes

```text
MAINTAINER FLOW COVERAGE
========================
[+] maintainer runs `cargo xtask family recommend --format json`
    ├── [GAP] stdout bytes exactly match written artifact bytes
    ├── [GAP] visible held candidates still appear in ranked_candidates[]
    └── [GAP] top-level status reflects readiness, not raw leverage

[+] maintainer runs `cargo xtask family validate-artifact <path>`
    ├── [GAP] valid schema v2 artifact passes
    └── [GAP] invalid ready/hold combination fails with actionable error
```

Coverage target for M27.5:

- every new readiness rule gets a direct unit-style test
- every new validator rule gets a negative test
- one locked-corpus regression proves the real motivating failure is fixed
- one command-path test proves stdout bytes match the written artifact bytes
- one command-path test proves the locked corpus artifact contains visible held
  candidates plus `recommendation_status = "no_strong_candidate"`

### Test files

- `xtask/src/lib.rs`
  - extend existing family artifact and recommend tests
- no separate test crate
- no new end-to-end harness outside current `xtask` test style

Required command-path tests:

- run `cargo xtask family recommend --format json` in a temp workspace and assert
  stdout bytes equal `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- run the locked-corpus recommendation flow and assert:
  - `recommendation_status == "no_strong_candidate"`
  - the `money/round` candidate is still present
  - the candidate is `hold`
  - hold reasons include:
    - `unknown_overlap_family`
    - `hard_difficulty`
    - `thin_real_example_support`

## Failure Modes Registry

| Codepath | Realistic production failure | Test required | Error handling required | User-visible outcome |
|---|---|---|---|---|
| schema split | accidentally bumping coverage schema while changing recommendation analysis | yes | yes | validator churn and broken artifact consumers |
| readiness adjudication | held candidate gets no hold reason | yes | yes | silent ambiguity in artifact output |
| ready-first ordering | ready candidate stays behind held candidate | yes | yes | false `no_strong_candidate` or misleading top slot |
| recommendation status | `ranked` still emitted on thin evidence | yes | yes | roadmap steering bug |
| locked corpus regression | money/round no longer visible at all | yes | yes | hidden pressure instead of honest abstention |

Critical gap rule:

- any readiness path with no test and no explicit artifact validation is a **critical gap**

## Worktree Parallelization Strategy

This plan has bounded parallelization.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| schema + validator update | `xtask/src/family/`, `xtask/src/lib.rs` | — |
| recommendation policy update | `xtask/src/family/`, `xtask/src/lib.rs` | schema + validator update |
| docs update | `semantic-families/` | recommendation policy wording settled |

### Parallel lanes

- Lane A: schema + validator update -> recommendation policy update
  sequential, shared `xtask/src/family/`
- Lane B: docs update
  independent once terminology is frozen

### Execution order

- Launch Lane A first
- once readiness field names and status rules are frozen, launch Lane B in parallel
- merge Lane B after Lane A proves the final wording

### Conflict flags

- `recommend.rs` and `promotion_artifacts.rs` are the same module lane, do not split them
- `xtask/src/lib.rs` tests depend on final field names, keep them in Lane A

## Deferred to TODOS.md

- If M27.5 still yields `no_strong_candidate`, capture the next step as a dedicated
  corpus-expansion or policy-decision milestone rather than folding it into this plan.
- If post-M27.5 reviewers still struggle to interpret held candidates, consider a later
  read-side summary improvement. Not now.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | intake | replace old M27 root plan with a fresh M27.5 root plan | mechanical | completeness | current `PLAN.md` was authoritative for the wrong milestone | keep stale M27 plan |
| 2 | scope | keep command surface unchanged | mechanical | explicit over clever | new commands add cognitive load without fixing the trust bug | `family analyze` |
| 3 | architecture | split recommendation into discovery + readiness layers inside `recommend.rs` | taste | pragmatic | preserves minimal diff while making the policy seam explicit | new module or full rewrite |
| 4 | schema | version recommendation-analysis independently | mechanical | minimal diff | coverage semantics must remain M27 truth | global schema bump |
| 5 | ranking | sort ready candidates ahead of held candidates | mechanical | explicit over clever | otherwise a held candidate can still shadow a valid recommendation | leverage-only sort |
| 6 | review | defer `single_source_pressure` from the root M27.5 slice | mechanical | minimal diff | current serialized coverage contract does not expose the source-count signal without widening M27 semantics | hidden in-memory side channel or coverage schema change |
| 7 | review | reorder status derivation so `insufficient_real_corpus` is reachable | mechanical | explicit over clever | subset conditions must be checked first or the branch is dead | ambiguous status ladder |

## Acceptance Gates

M27.5 is complete only when all of the following are true:

1. `cargo xtask family recommend --format json` remains deterministic.
2. Recommendation analysis still prints to stdout and writes identical bytes to
   `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`.
3. `cargo xtask family validate-artifact <path>` accepts the schema v2 recommendation-analysis artifact.
4. Weak candidates remain visible instead of disappearing.
5. A candidate with `overlap_family = "unknown"` and `difficulty = "hard"` does not cause top-level `ranked` on thin evidence.
6. The current locked three-source corpus yields `no_strong_candidate`.
7. A stronger adjacent known-overlap candidate can still yield `ranked`.
8. Coverage artifact semantics remain unchanged from M27.

## Post-M27.5 Branch Rule

After M27.5, the repo must make one of two explicit next moves:

- if the tightened engine still yields `no_strong_candidate`, the next milestone
  is a small corpus-expansion pack or explicit human policy choice
- if the tightened engine yields a genuinely ready candidate, the next milestone
  is that family promotion

M28 does **not** begin automatically.

## Completion Summary

- Step 0: Scope Challenge — accepted as a narrow `xtask` policy hardening milestone
- Architecture Review: two major architecture locks, per-artifact schema split and reachable status ladder
- Code Quality Review: keep explicit helpers, avoid new module churn
- Test Review: diagram produced, targeted readiness, validator, and command-path regression gaps identified
- Performance Review: no material runtime risk beyond deterministic sorting and artifact serialization
- NOT in scope: written
- What already exists: written
- TODOS.md updates: follow-up candidates identified, none bundled now
- Failure modes: readiness ambiguity treated as critical if untested
- Outside voice: unavailable in this session, host subagent review incorporated
- Parallelization: 2 lanes, 1 primary sequential lane + 1 docs lane
- Lake Score: complete option chosen

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | clean | 3 issues found, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**UNRESOLVED:** 0

**VERDICT:** ENG CLEARED — fresh M27.5 plan is ready to implement.
