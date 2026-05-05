# M34 - Stop-Spend-Pivot Decision Contract

Status: **authoritative implementation plan**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-05**  
Supersedes: **M33 - Recommendation-Quality Promotion Decisions**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260504-233336.md`**  
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**  
Program tracker: **`docs/recommendation_corpus_expansion_program_v0.1.md`**  
Capability guide: **`docs/semantic_family_capability_corpus_guide_v0.1.md`**  
Live analysis basis: **`.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`**  
Execution note: **Do not create `ORCH_PLAN.md` up front. This is one bounded `xtask` + docs milestone. Split work only after the schema and command names are frozen.**  
Foundation precondition: **Start from commit `e7df8cbccfec0d7359d58d32ef17eaacd5a10946` or a direct descendant that preserves the closed M33 truth surface.**

## Objective

Turn the current truthful M33 output into one explicit machine-readable next-step
decision.

After M34, a maintainer or agent should be able to answer one operational
question without re-reading the corpus tracker, design doc, or chat history:

> Should the repo spend corpus run `1`, keep it unspent, or pivot away from
> corpus work now?

That answer must be emitted as a bounded artifact, not left as prose inference.

## Decision

M34 ships as a **bounded sibling decision artifact** under the existing family
analysis tree:

- path:
  `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- producer command:
  `cargo xtask family corpus-decision --format json`
- input:
  `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

This is the smallest complete move.

Do not reopen recommendation policy. Do not parse markdown program trackers at
runtime. Do not widen into corpus accounting, family promotion execution, or
shared-core implementation.

M34 consumes the fixed M33 analysis artifact, derives one bounded next-step
decision, validates it, writes it deterministically, and documents what that
decision means.

## Problem Statement

M33 closed the recommendation-honesty problem.

The live branch now truthfully says:

- `recommendation_status = "no_strong_candidate"`
- `decision_summary.decision_status = "not_recommended"`
- top visible pressure is
  `unsupported_function_surface-e40675da6fa0`
- durable blocker is `helper_surface_not_promotable`
- `missing_evidence = []`
- `stale_evidence = []`

That is good output, but it still leaves one repo-level operator question
unresolved:

- spend corpus run `1`
- keep it unspent
- pivot away from corpus work

Right now the repo can say "not this family." It still cannot say "therefore do
this next" in a machine-readable, bounded, deterministic way.

That is the whole M34 gap.

## Live Basis

The current branch basis is fixed and must remain the canonical wedge during
implementation:

```json
{
  "recommendation_status": "no_strong_candidate",
  "decision_status": "not_recommended",
  "top_candidate_id": "z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0",
  "open_blockers": ["helper_surface_not_promotable"],
  "missing_evidence": [],
  "stale_evidence": []
}
```

The expected live M34 output for that basis is:

- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `decision_basis_code = "durable_non_promotable_helper_surface"`
- `required_next_action = "author_architecture_follow_on_plan"`

This is deliberate. The current blocker is not missing corpus. It is that the
visible helper-surface pressure is real but not promotable.

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Coverage truth | `xtask/src/family/coverage.rs` and `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json` | Reuse unchanged. M34 does not rescan the corpus itself. |
| Recommendation analysis truth | `xtask/src/family/recommend.rs` and `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json` | Reuse as the fixed input surface. M34 consumes this artifact instead of recomputing policy inside a new subsystem. |
| Artifact schema + validation | `xtask/src/family/promotion_artifacts.rs` | Extend with one new artifact kind, one new schema, and one new validation path. Do not create a second validator stack. |
| Artifact paths + atomic writes | `xtask/src/family/paths.rs` and `write_bytes_atomically(...)` | Reuse directly. The new decision artifact lives under the existing `analysis/` directory. |
| CLI command dispatch | `xtask/src/lib.rs` | Extend with one new `family corpus-decision` command. Do not add a separate binary. |
| Maintainer-facing truth surfaces | `semantic-families/README.md`, `docs/recommendation_corpus_expansion_program_v0.1.md`, `docs/semantic_family_capability_corpus_guide_v0.1.md` | Update wording so the repo explains stop vs spend vs pivot without reopening M33 semantics. |

### Minimum change set

The minimum complete implementation is:

1. add one new analysis artifact path constant
2. add one new artifact schema + validator
3. add one new CLI command that loads the existing recommendation analysis and
   emits the bounded decision contract
4. add tests for the live wedge and the contradictory-state guards
5. update the maintainer docs that describe recommendation and corpus-program
   outcomes

Anything beyond that is scope leak.

### Complexity check

This milestone should stay within roughly these touched areas:

- `xtask/src/lib.rs`
- `xtask/src/family/paths.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/recommend.rs`
- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `PLAN.md`

That is already a real but bounded diff. Do not add:

- a markdown parser for the program tracker
- a new runtime crate
- a new artifact directory outside `analysis/`
- a new command family outside `xtask family`

### Search check

This is a Layer 1 extension of the repo's existing artifact pipeline.

No new framework, concurrency model, storage layer, or distribution path is
introduced. The right move is to extend the current `xtask` artifact contract,
not to invent a second decision system.

### Completeness check

A prose-only closeout is not enough.

The complete bounded version is:

- machine-readable artifact
- validator coverage
- deterministic write behavior
- docs aligned to the same vocabulary
- live wedge proof

That is the lake. Boil it.

### Distribution check

No new package, binary, service, or CI lane is required.

The output is one additional repo-local JSON artifact layered onto the current
analysis flow.

## Locked Decisions

### 1. M33 recommendation analysis is fixed input

M34 reads the already-written recommendation analysis artifact.

Do not rerun coverage or recommendation inside the decision command. The point
is to consume fixed truth, not silently recompute it.

### 2. The decision contract is a sibling artifact, not an M33 schema rewrite

Write the new contract to:

- `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`

Do not widen `recommendation.latest.json` into a second semantic domain. That
artifact remains the recommendation-analysis surface. M34 consumes it and emits
the next-step decision beside it.

### 3. The command surface is one new read-side command

Add:

```bash
cargo xtask family corpus-decision --format json
```

Rules:

- only `--format json` is supported
- it loads the current recommendation-analysis artifact
- it validates the basis before deriving a decision
- it writes the new `corpus-program-decision.latest.json` artifact

### 4. Decision vocabulary is explicit and bounded

`decision_action` may be exactly one of:

- `stop`
- `spend_corpus_run_1`
- `pivot_to_family_promotion_run`
- `pivot_to_recommendation_policy_run`
- `pivot_to_architecture_shared_core_follow_on`

Do not allow arbitrary strings.

### 5. Decision basis vocabulary is explicit and bounded

`decision_basis_code` may be exactly one of:

- `promotion_ready_candidate`
- `plausible_candidate_missing_evidence`
- `durable_non_promotable_helper_surface`
- `no_actionable_candidate`
- `policy_interpretation_blocker`

These are the machine-readable reasons for the action.

### 6. Required next action is explicit and bounded

`required_next_action` may be exactly one of:

- `record_stop_without_new_milestone`
- `author_corpus_expansion_plan`
- `author_family_promotion_plan`
- `author_recommendation_policy_plan`
- `author_architecture_follow_on_plan`

Do not store free-form workflow prose as the only operational field.

### 7. Rejected alternatives are required but kept small

The artifact must include a bounded `rejected_alternatives` array covering the
two top-level branches not chosen.

Each entry includes:

- `action`
- `reason_code`
- `summary`

This keeps agent handoff honest without turning the contract into an essay.

### 8. Pivot targets stay milestone-class level, not milestone-id level

M34 names the next **class** of milestone, not a future milestone number.

Use:

- `family_promotion_run`
- `recommendation_policy_run`
- `architecture_shared_core_follow_on`

Do not hard-code a roadmap number in the artifact itself.

### 9. Stop has exact semantics

`stop` means:

- keep corpus run `1` unspent
- do not authorize another corpus-expansion milestone
- do not automatically authorize a pivot milestone either
- record the hold state as the current truthful endpoint

This is different from pivot.

### 10. Spend has exact semantics

`spend_corpus_run_1` means:

- the repo has a plausible candidate
- the missing information is still evidence-shaped
- one more explicitly scoped corpus run is justified

It does **not** mean "corpus forever."

### 11. Live wedge mapping is frozen

For the current basis:

- `decision_status = "not_recommended"`
- only blocker is `helper_surface_not_promotable`
- missing/stale evidence are both empty

the emitted action must be:

- `pivot_to_architecture_shared_core_follow_on`

If implementation cannot produce that deterministically, the milestone is not
done.

### 12. Docs must not over-claim

Every updated doc must keep these boundaries explicit:

- the repo is emitting a next-step decision contract
- that contract consumes M33 truth
- M34 does not spend corpus run `1` by default
- M34 does not promote a new family
- M34 does not implement shared-core follow-on work

## Artifact Contract

### Artifact path

```text
.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

### Artifact kind and schema

Add one new artifact kind:

- `corpus_program_decision`

Add one new schema version:

- `CORPUS_PROGRAM_DECISION_SCHEMA_VERSION = 1`

### Canonical JSON shape

```json
{
  "schema_version": 1,
  "artifact_kind": "corpus_program_decision",
  "generated_at": "2026-05-05T03:00:00Z",
  "analysis_basis_path": ".semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json",
  "analysis_basis_sha256": "sha256...",
  "basis_snapshot": {
    "recommendation_status": "no_strong_candidate",
    "decision_status": "not_recommended",
    "top_candidate_id": "z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0",
    "open_blockers": ["helper_surface_not_promotable"],
    "missing_evidence": [],
    "stale_evidence": [],
    "warnings": ["regression_warning"]
  },
  "decision_action": "pivot_to_architecture_shared_core_follow_on",
  "decision_basis_code": "durable_non_promotable_helper_surface",
  "pivot_target_class": "architecture_shared_core_follow_on",
  "required_next_action": "author_architecture_follow_on_plan",
  "summary": "Corpus run 1 should remain unspent; the current visible pressure is durable helper-surface hold, so the next move is an architecture follow-on plan rather than more corpus.",
  "rejected_alternatives": [
    {
      "action": "stop",
      "reason_code": "no_actionable_candidate",
      "summary": "Reject stop because the repo does have a clear next class of work."
    },
    {
      "action": "spend_corpus_run_1",
      "reason_code": "plausible_candidate_missing_evidence",
      "summary": "Reject spend because the current blocker is not missing corpus evidence."
    }
  ]
}
```

### Validation invariants

The validator must reject:

1. missing or invalid `analysis_basis_path`
2. missing or mismatched `analysis_basis_sha256`
3. unknown `decision_action`, `decision_basis_code`, `required_next_action`, or
   `pivot_target_class`
4. `spend_corpus_run_1` when the basis has:
   - `decision_status != "blocked_for_now"`, or
   - empty `missing_evidence` and empty `stale_evidence`
5. `pivot_to_family_promotion_run` unless the basis supports a promotion-ready
   next move
6. `pivot_to_architecture_shared_core_follow_on` unless the basis is
   non-corpus-shaped and the visible blocker is not an evidence gap
7. `stop` when a more specific pivot or spend action is already justified by the
   basis
8. missing `rejected_alternatives`
9. duplicate or contradictory rejected alternatives
10. a `pivot_target_class` field on non-pivot actions

## Decision Derivation Rules

Apply these rules in order. The first matching rule wins.

| Basis condition | Emitted action | Basis code | Required next action |
|---|---|---|---|
| `decision_status = recommended` | `pivot_to_family_promotion_run` | `promotion_ready_candidate` | `author_family_promotion_plan` |
| `decision_status = blocked_for_now` and the basis carries missing/stale evidence or a targeted evidence gap | `spend_corpus_run_1` | `plausible_candidate_missing_evidence` | `author_corpus_expansion_plan` |
| `decision_status = not_recommended`, the blocker is `helper_surface_not_promotable`, and missing/stale evidence are both empty | `pivot_to_architecture_shared_core_follow_on` | `durable_non_promotable_helper_surface` | `author_architecture_follow_on_plan` |
| `decision_status = not_recommended`, the blocker is recommendation/policy interpretation rather than evidence or architecture | `pivot_to_recommendation_policy_run` | `policy_interpretation_blocker` | `author_recommendation_policy_plan` |
| no candidate-specific action is justified | `stop` | `no_actionable_candidate` | `record_stop_without_new_milestone` |

This mapping is intentionally small.

If future repo truth requires a new branch, that is a new milestone. Do not
smuggle it into M34.

## Architecture Review

### System shape

M34 is a read-side analysis extension. No write-path semantic truth changes.

```text
semantic-families/corpus/rust-function.toml
        |
        v
cargo xtask family coverage --format json
        |
        v
analysis/coverage.latest.json
        |
        v
cargo xtask family recommend --format json
        |
        v
analysis/recommendation.latest.json
        |
        v
cargo xtask family corpus-decision --format json
        |
        v
analysis/corpus-program-decision.latest.json
```

### Code ownership and module boundaries

```text
xtask/src/lib.rs
    CLI dispatch only
        |
        v
xtask/src/family/recommend.rs
    load validated recommendation basis
    derive bounded corpus-program decision
        |
        v
xtask/src/family/promotion_artifacts.rs
    schema types
    validate-artifact support
        |
        v
xtask/src/family/paths.rs
    artifact path constant
    atomic write helpers
```

### Realistic production failure scenarios

1. The basis artifact is stale, missing, or manually edited into contradiction.
   M34 must fail validation before writing a decision artifact.
2. The basis says `blocked_for_now` but missing/stale evidence arrays are empty.
   M34 must reject `spend_corpus_run_1` rather than silently guessing.
3. The live helper-surface wedge regresses back to a corpus-shaped gap.
   M34 must emit the new truthful action, not preserve the old one.
4. The docs drift and start claiming that pivot means M34 implements the follow-on.
   Docs must explicitly say M34 names the next class of work, not the work
   itself.

## Implementation Plan

### Step 1 - Freeze vocabulary, path, and live wedge

Touch:

- `PLAN.md`

Lock:

- artifact path
- command name
- action vocabulary
- basis-code vocabulary
- required-next-action vocabulary
- live expected output

This step is complete when there is no remaining ambiguity about the JSON shape
or the live wedge outcome.

### Step 2 - Add the artifact schema and validator

Touch:

- `xtask/src/family/paths.rs`
- `xtask/src/family/promotion_artifacts.rs`

Required changes:

1. add the artifact path constant
2. add `PromotionArtifactKind::CorpusProgramDecision`
3. add the schema version constant
4. add the serde struct(s) for the new artifact
5. add `validate(...)` for the new artifact
6. extend artifact-path classification so `family validate-artifact` knows the
   new path

Do not add a second validation entrypoint.

### Step 3 - Add the decision builder command

Touch:

- `xtask/src/lib.rs`
- `xtask/src/family/recommend.rs`

Required changes:

1. add `FamilyCommand::CorpusDecision`
2. accept only `--format json`
3. load the current recommendation-analysis artifact from disk
4. validate the basis artifact before deriving the decision
5. derive the bounded decision contract from the rules table above
6. write the latest artifact atomically
7. preserve deterministic bytes when the basis is unchanged

Keep the implementation in the current family-analysis code path. Do not create
an orchestration subsystem.

### Step 4 - Prove the live wedge and the contradictory-state guards

Touch:

- `xtask/src/lib.rs`
- optionally small helper additions in existing `xtask` test support only if
  required

Required tests:

1. live helper-surface wedge emits
   `pivot_to_architecture_shared_core_follow_on`
2. promotion-ready basis emits `pivot_to_family_promotion_run`
3. evidence-gap basis emits `spend_corpus_run_1`
4. empty/no-action basis emits `stop`
5. contradictory action/basis combinations are rejected by validation
6. repeated command runs write byte-identical output when the basis is unchanged
7. non-JSON format is rejected with the current CLI style

### Step 5 - Sync the maintainer docs

Touch:

- `semantic-families/README.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`

Required wording updates:

1. recommendation analysis remains the M33 truth input
2. corpus-program decision is the M34 next-step output
3. stop vs spend vs pivot meanings are explicit
4. the current live wedge keeps corpus run `1` unspent and points to an
   architecture follow-on class
5. M34 does not claim that the follow-on has already been implemented

## Code Quality Review

### DRY guardrails

Do not duplicate:

- artifact path normalization logic
- artifact validation entrypoints
- recommendation-basis loading
- atomic write behavior

If the new decision artifact needs "latest artifact load + deterministic bytes"
behavior, reuse the current patterns from `recommend.rs` rather than inventing a
parallel helper stack.

### Explicit over clever

Prefer:

- one small derivation function with a rules table feel
- one bounded validator
- one artifact struct

Avoid:

- dynamic rule engines
- stringly typed free-form pivot targets
- doc parsing at runtime
- implicit inference from absent fields alone

### Engineered enough

The right level of engineering here is:

- schema-backed
- validator-backed
- deterministic
- test-covered

The wrong level is:

- new crate
- new registry
- generic workflow DSL
- program tracker parser

## Test Review

### Test framework

This repo already uses Rust tests in `xtask/src/lib.rs` and related modules.
That remains the primary lock surface.

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] cargo xtask family corpus-decision --format json
    |
    +- load recommendation basis artifact
    |  +- [REQ TEST] missing file -> command fails
    |  +- [REQ TEST] invalid schema -> command fails
    |  +- [REQ TEST] mismatched sha/path -> validator fails
    |
    +- derive decision contract
    |  +- [REQ TEST] recommended -> pivot_to_family_promotion_run
    |  +- [REQ TEST] blocked_for_now + evidence gap -> spend_corpus_run_1
    |  +- [REQ TEST] not_recommended + helper_surface_not_promotable + no evidence gaps
    |  |              -> pivot_to_architecture_shared_core_follow_on
    |  +- [REQ TEST] no actionable candidate -> stop
    |  +- [REQ TEST] policy-shaped blocker -> pivot_to_recommendation_policy_run
    |
    +- validate decision contract
    |  +- [REQ TEST] spend without evidence gap -> reject
    |  +- [REQ TEST] pivot without pivot_target_class -> reject
    |  +- [REQ TEST] stop when a stronger action is justified -> reject
    |  +- [REQ TEST] missing rejected_alternatives -> reject
    |
    +- write latest artifact
       +- [REQ TEST] first run writes artifact
       +- [REQ TEST] second identical run writes byte-identical output
```

### Maintainer flow coverage

```text
MAINTAINER FLOW COVERAGE
===========================
[+] Truthful stop/spend/pivot workflow
    |
    +- cargo xtask family coverage --format json
    +- cargo xtask family recommend --format json
    +- cargo xtask family corpus-decision --format json
    +- cargo xtask family validate-artifact \
         .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
    +- cargo xtask family validate-artifact \
         .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

### Regression rule

The highest-priority regression test is the live wedge:

- current basis:
  `no_strong_candidate` + `not_recommended` +
  `helper_surface_not_promotable` + no missing/stale evidence
- expected output:
  `pivot_to_architecture_shared_core_follow_on`

If that regresses, M34 has failed the exact problem it is supposed to solve.

### Exact test additions

Add or extend tests so they prove:

1. command dispatch recognizes `family corpus-decision`
2. the new artifact validates on the happy path
3. invalid combinations are rejected at `validate-artifact`
4. current branch truth emits the expected pivot output
5. deterministic re-run behavior matches the repo's existing analysis commands

## Failure Modes

| Codepath | Production failure | Test required? | Error handling required? | User-visible outcome |
|---|---|---:|---:|---|
| Load basis artifact | basis file missing or unreadable | Yes | Yes | CLI exits non-zero with bounded error, no decision artifact written |
| Validate basis snapshot | basis schema drift or manual corruption | Yes | Yes | CLI refuses to guess and points at invalid input |
| Derive spend action | evidence-gap inference fires when there is no evidence gap | Yes | Yes | rejected as contradictory state, not silent spend authorization |
| Derive pivot action | helper-surface durable hold incorrectly maps to `stop` | Yes | Yes | regression test catches wrong next-step output |
| Validate written artifact | pivot target absent or unknown | Yes | Yes | `validate-artifact` fails on the decision artifact |
| Docs sync | docs imply M34 implemented the follow-on work | Yes, via targeted grep or review pass | Yes | maintainer confusion; block merge until wording is corrected |

### Critical gap rule

Any branch that:

- emits a decision action,
- has no validation,
- and can silently authorize the wrong next milestone

is a critical gap.

M34 closes that gap only if the live wedge and contradictory states are both
locked by tests.

## Performance Review

M34 is cheap if it stays read-side only.

Performance rules:

1. do not rescan the corpus inside `family corpus-decision`
2. do not rerun recommendation logic from raw sources
3. read one basis artifact, derive one decision, write one artifact
4. preserve deterministic latest-byte behavior to avoid unnecessary churn

Expected cost is one JSON read, one validation pass, one decision derivation,
and one JSON write. Anything slower means the scope drifted.

## NOT in Scope

The following work was considered and is explicitly deferred:

- Spending corpus run `1`
  Reason: M34 decides whether that run is justified. It does not perform the run.
- Recommendation-policy redesign
  Reason: M33 already closed recommendation honesty; M34 consumes that output.
- Family promotion execution changes
  Reason: pivot naming is in scope, promotion execution mechanics are not.
- Corpus manifest or leverage-accounting changes
  Reason: that is evidence policy work, not next-step contract work.
- Shared-core follow-on implementation
  Reason: M34 may point to that class of work, but it does not start it.
- Runtime parsing of `docs/recommendation_corpus_expansion_program_v0.1.md`
  Reason: markdown is the human ledger, not the machine input surface.
- New artifact trees outside `analysis/`
  Reason: M34 stays inside the current family-analysis contract.

## Worktree Parallelization Strategy

Parallelism exists, but it is limited.

All executable logic clusters under `xtask/src/family/`, so the code lane is
mostly sequential. The only clean peel-off lane is docs, and only after the
schema and vocabulary are frozen.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Freeze contract vocabulary and path | `PLAN.md` | — |
| Add artifact schema and validator | `xtask/src/family/`, `xtask/src/lib.rs` | Freeze contract vocabulary and path |
| Add command derivation and deterministic write path | `xtask/src/family/`, `xtask/src/lib.rs` | Add artifact schema and validator |
| Add live-wedge and contradiction tests | `xtask/src/family/`, `xtask/src/lib.rs` | Add command derivation and deterministic write path |
| Sync maintainer docs | `docs/`, `semantic-families/` | Freeze contract vocabulary and path |

### Parallel lanes

- Lane A: freeze contract -> schema/validator -> command derivation -> tests
  (sequential, shared `xtask/src/family/`)
- Lane B: docs sync
  (independent after schema freeze, touches `docs/` and `semantic-families/`)

### Execution order

1. Do the contract freeze first.
2. Launch Lane A and Lane B in parallel only after the JSON shape, path, and
   vocabulary are locked.
3. Merge Lane B after Lane A passes if doc text needs final command-output
   wording polish.

### Conflict flags

- Do **not** split Lane A into multiple code worktrees. Everything meaningful
  shares `xtask/src/family/` and `xtask/src/lib.rs`.
- Lane B should avoid editing `PLAN.md` after Lane A starts. Treat this plan as
  frozen once implementation begins.

## Acceptance Commands

Run these commands against the live branch:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
cargo test -p xtask corpus_decision
cargo test -p xtask recommendation_policy_durable_holds_helper_surface_candidate
```

If the repo's test names differ once implementation lands, keep the same proof
intent:

- command dispatch test
- live helper-surface wedge test
- contradictory-state validation test
- deterministic re-run test

## Done Means

M34 is complete only when all of the following are true:

1. `cargo xtask family corpus-decision --format json` writes
   `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
2. the new artifact validates through the existing `family validate-artifact`
   surface
3. the current live basis emits
   `pivot_to_architecture_shared_core_follow_on`
4. `recommended`, `blocked_for_now`, helper-surface durable hold, and stop
   outcomes are all covered by tests
5. contradictory action/basis combinations are rejected by validation
6. re-running the command on unchanged basis input preserves deterministic bytes
7. docs explain stop vs spend vs pivot using the same exact vocabulary as the
   JSON artifact
8. the diff does not widen into corpus execution, policy redesign, family
   promotion execution, or shared-core implementation

That is the full M34 claim.
