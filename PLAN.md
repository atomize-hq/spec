# M33 - Recommendation-Quality Promotion Decisions

Status: **authoritative implementation plan**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-04**  
Supersedes: **M32 - One Bounded Second-Language Promotion Path**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260504-201833.md`**  
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**  
Program tracker: **`docs/recommendation_corpus_expansion_program_v0.1.md`**  
Capability guide: **`docs/semantic_family_capability_corpus_guide_v0.1.md`**  
Execution note: **Do not create `ORCH_PLAN.md` up front. Split into worktrees only if the schema contract is frozen and there is still enough isolated docs/test work to justify it.**  
Foundation precondition: **Start from publish SHA `6a1051b601487710d631031171cfde92810f1581` or a direct descendant that still preserves the closed M32 artifact truth.**

## Objective

Make the repo able to emit recommendation artifacts that carry the promotion
decision argument, not just cluster visibility.

After M33, a maintainer should be able to open the current recommendation
artifact and answer five questions without stitching together chat context or
adjacent proof files:

1. Is a family recommended right now?
2. If not, is it blocked for now or simply not the next move?
3. What exact evidence is present, missing, or stale?
4. What specifically blocks promotion?
5. What changed since the last truthful recommendation?

That is the full M33 claim.

## Decision

M33 ships as a bounded recommendation-quality hardening pass over the existing
analysis and family-promotion artifact chain.

That means:

1. The main user surface is still the existing artifact tree under
   `.semantic-family-artifacts/family-promotion/`.
2. The primary visible payoff is a better
   `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`,
   not a new family or a new proof lane.
3. M33 reuses the current M27/M32 coverage, ranking, validation, and promotion
   artifact codepaths. It extends them explicitly instead of creating a new
   policy subsystem.
4. M33 keeps bounded second-language honesty. No artifact may imply repo-wide
   multi-language readiness because M32 only proved one bounded lane for
   `function.arithmetic_leaf.monotone_up.v1`.

## Problem Statement

M32 is closed. Good.

The repo now has a real bounded second-language proof path, and the current
analysis artifact is mechanically honest. But it is still too thin as a
decision surface.

Today the current analysis artifact says:

- path:
  `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `schema_version = 3`
- `recommendation_status = "no_strong_candidate"`
- one visible candidate:
  `unsupported_function_surface-e40675da6fa0`
- that candidate is a durable hold centered on `money/round`
- the current explanation is spread across:
  `promotion_readiness`, `hold_reasons`, `next_step_status`,
  `next_step_detail`, leverage counts, and implicit knowledge of why helper
  surfaces are not promotable

That output is truthful, but it still makes the maintainer do too much
interpretation work.

The missing value is not more proof plumbing. The missing value is a cleaner
judgment surface:

- recommended
- blocked for now
- not recommended
- why
- what evidence is stale or missing
- what changed since the last run

That is the gap M33 closes.

## Locked Decisions

### 1. M32 is treated as earned

Do not spend M33 budget re-proving the same bounded TypeScript lane with new
milestone prose.

M33 starts from the closed M32 state and improves the decision artifacts built
on top of it.

### 2. Recommendation quality is the product surface

The primary M33 output is better recommendation and family-promotion artifact
truth.

Do not widen into:

- a new promoted family
- corpus-expansion run `1`
- generic policy or approval workflow machinery
- broad TypeScript support claims
- `spec-core` semantic-family runtime expansion

### 3. Keep the current analysis artifact path

The current maintainer entrypoint stays:

- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

M33 makes that artifact more useful. It does not hide the new truth in a new
parallel artifact family.

### 4. Delta lives inside the primary analysis artifact

The current recommendation artifact is the thing maintainers already inspect.
That is where the change summary belongs.

Do not add a separate sibling delta artifact unless implementation proves the
embedded form is impossible to keep deterministic.

### 5. Preserve the current ranking view as a compatibility layer

`recommendation_status` stays in the analysis artifact as the existing
machine-facing ranking summary.

M33 adds a new maintainer-facing decision layer on top of it rather than
replacing the field outright. This keeps the diff smaller and avoids breaking
consumers that only understand the M27/M32 vocabulary.

### 6. Decision vocabulary is explicit and bounded

M33 introduces one new top-level decision verdict:

- `recommended`
- `blocked_for_now`
- `not_recommended`

And it uses a bounded blocker/evidence vocabulary derived from existing truth:

- `unknown_overlap_family`
- `hard_difficulty`
- `thin_real_example_support`
- `thin_regression_support`
- `helper_surface_not_promotable`
- `missing_evidence`
- `stale_evidence`
- `regression_warning`

Do not turn this into a generic rules engine with arbitrary human-authored
policies.

### 7. Family-scoped promotion artifacts must carry the same basis

The family-scoped artifact chain under
`.semantic-family-artifacts/family-promotion/<family>/...` must reference the
analysis basis that justified the action:

- which analysis artifact was used
- what its verdict was
- which blockers were already open
- whether any evidence was stale or missing at emission time

Execution and blocker artifacts do not recompute recommendation policy. They
carry forward the chosen basis honestly.

### 8. Coverage accounting is not being redesigned

M33 reuses the current coverage artifact and unsupported-cluster projection.

Do not change corpus manifest policy, source-kind leverage rules, or cluster
discovery as side work unless implementation proves a blocker in the existing
inputs.

### 9. Wrapper regression pressure stays indirect

Wrapper-pipeline pressure remains recommendation input, not a special
top-level promotion policy.

If wrapper regressions matter for a recommendation, they should appear through
existing leverage and evidence fields plus a bounded `regression_warning`, not
through a custom wrapper-only policy lane.

### 10. Docs must stay narrow about second-language support

Every updated doc must keep the M32 boundary explicit:

- one bounded second-language pilot exists
- recommendation artifacts may discuss that proof
- no M33 artifact may imply broad repo-wide target-language readiness

## Done Means

M33 is complete only when all of the following are true:

1. the analysis artifact still validates at its existing path and now includes
   a top-level decision summary, explicit evidence state, and delta from the
   last truthful artifact
2. the current `money/round` helper-surface path becomes easier to explain from
   the artifact alone, without extra maintainer interpretation
3. the analysis artifact can distinguish:
   - recommended
   - blocked for now
   - not recommended
4. missing evidence and stale evidence are explicit fields, not implied through
   absent counts or adjacent artifact inspection
5. family-scoped recommendation artifacts carry forward the analysis basis and
   remain honest about bounded support
6. promotion execution and blocker artifacts can point back to the decision
   basis that started the run
7. validators reject contradictory combinations such as:
   - `decision_status = "recommended"` while the first candidate is still held
   - stale evidence omitted from a blocked recommendation
   - family-scoped artifacts that disagree with their analysis basis
8. docs explain the new vocabulary and the current truthful wedge without
   over-claiming broader capability
9. the implementation is proven on one real current path, not only on synthetic
   fixture-only cases

## NOT in Scope

The following work was considered and is explicitly deferred:

- Promoting a new family packet
  Reason: M33 improves the decision surface, not the supported-family set.
- Spending corpus-expansion run `1`
  Reason: the current program tracker explicitly says each run needs its own
  contract, and M33 is not a corpus-growth run.
- Changing source-kind leverage rules
  Reason: that is coverage policy work, not decision-surface work.
- Broad repo-wide TypeScript support messaging
  Reason: M32 proved one bounded family lane only.
- Starting seam-kind target-language work
  Reason: that would reopen the M31/M32 boundary and is a different milestone.
- Approval workflow machinery, overrides, or RBAC
  Reason: that is policy-system work, not bounded recommendation-quality work.
- Rewriting `xtask` command names
  Reason: the current command surface already exists and should be reused.
- `spec-core` reviewer capability changes
  Reason: M33 consumes current runtime truth; it does not expand supported
  semantic families.

## What Already Exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Coverage and unsupported-cluster truth | `xtask/src/family/coverage.rs` plus `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json` | Reuse as the evidence input. Do not invent a second coverage pipeline. |
| Recommendation ranking and hold logic | `xtask/src/family/recommend.rs` | Reuse the current ranking kernel and extend it with an explicit decision layer plus delta logic. |
| Artifact schemas and validation | `xtask/src/family/promotion_artifacts.rs` | Reuse the existing serde models and path-aware validators. Extend them explicitly. |
| Artifact paths and deterministic writes | `xtask/src/family/paths.rs` and `write_bytes_atomically` | Reuse the existing artifact tree and deterministic write behavior. |
| Family-scoped recommendation emission | `run_refresh_recommendation(...)` in `xtask/src/family/promotion_artifacts.rs` | Reuse, but make it analysis-basis-aware instead of acting like an isolated thin packet. |
| Promotion execution and blocker emission | `run_emit_promotion_execution(...)` and `run_emit_promotion_blocker(...)` in `xtask/src/family/promotion_artifacts.rs` | Reuse, but thread through decision-basis truth. |
| CLI and schema regression bed | `xtask/src/lib.rs` tests around `validate-artifact`, recommendation analysis, promotion execution, and blocker reports | Reuse as the main lock-test surface. Add M33 fixtures there rather than creating a second test universe. |
| Maintainer explanation surface | `semantic-families/README.md`, `docs/semantic_family_capability_corpus_guide_v0.1.md`, and `docs/recommendation_corpus_expansion_program_v0.1.md` | Reuse as the human-facing truth surface. Update wording to match the new artifact vocabulary exactly. |
| Live truthful wedge | current `recommendation.latest.json` durable hold for `unsupported_function_surface-e40675da6fa0` | Use this as the canonical M33 example. If the new artifact cannot explain this path well, M33 is not done. |

## Step 0 - Scope Challenge

This plan likely touches more than 8 files.

Normally that is a smell. Here it is justified because the missing value is
cross-artifact decision truth, and that truth already spans:

- recommendation analysis schema
- family-scoped recommendation schema
- promotion execution schema
- blocker schema
- validators
- CLI regression tests
- maintainer docs

The minimum honest M33 change is:

1. keep current coverage and ranking discovery intact
2. add an explicit decision layer to the analysis artifact
3. make missing and stale evidence first-class fields
4. embed a deterministic delta from the last truthful analysis artifact
5. thread the decision basis through downstream family-promotion artifacts
6. update validators and docs so the new story is enforceable

Anything smaller is just better prose around the same thin output.

### Complexity check

This is a multi-file change, but it does **not** justify a new subsystem.

The right implementation is boring:

- extend current serde structs
- extend current validators
- extend current recommend projection
- update the existing CLI regression bed
- update the current docs

No new service, no database, no new command family.

### Search check

This is a Layer 1 change.

The repo already has the exact building blocks it needs:

- existing artifact paths
- existing validator model
- existing deterministic file writes
- existing recommendation projection
- existing family-promotion artifact chain

Do not roll a custom comparison store or approval-state side channel when the
repo already has a path-stable artifact tree.

### TODOS cross-reference

`docs/recommendation_corpus_expansion_program_v0.1.md` explicitly says each run
needs its own high-rigor plan and that the open question is whether more
evidence is needed or the blocker is now recommendation interpretation.

M33 answers the interpretation side of that question.

### Completeness check

The shortcut version would be:

- rename a few statuses
- add nicer prose to the README

That is not enough.

The complete version is still cheap here:

- schema truth
- validator truth
- delta truth
- downstream artifact truth
- docs truth
- regression coverage

Do the complete version.

### Distribution check

M33 introduces no new binary, package, or service.

Its distribution surface is the existing artifact tree plus repo docs.

That means the implementation is only real if a maintainer can consume the new
decision quality through the current workflow, not by learning a side system.

## Architecture Review

### Chosen artifact model

M33 keeps the current artifact paths and makes the data model more explicit.

#### 1. Analysis artifact

Path stays:

- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

Current shape:

- `schema_version = 3`
- ranking-oriented view only

M33 change:

- bump to `schema_version = 4`
- keep existing ranking fields
- add a top-level `decision_summary`
- add a top-level `evidence_summary`
- add a top-level `delta_from_previous`

#### 2. Family-scoped recommendation artifact

Path stays:

- `.semantic-family-artifacts/family-promotion/<family>/recommendation.latest.json`

Current shape:

- thin family-scoped recommendation packet with inventory basis

M33 change:

- bump to `schema_version = 2`
- add `analysis_basis_path`
- add `analysis_basis_sha256`
- add `decision_status`
- add carried blocker/evidence fields from the chosen analysis basis

#### 3. Promotion execution and blocker artifacts

Paths stay:

- `.semantic-family-artifacts/family-promotion/<family>/<run-id>/promotion.execution.json`
- `.semantic-family-artifacts/family-promotion/<family>/<run-id>/blocker.report.json`

Current shape:

- execution-step truth only

M33 change:

- bump both to `schema_version = 2`
- add `analysis_basis_path`
- add `analysis_basis_sha256`
- add `decision_status_at_start`
- add `open_blockers_at_start`
- add `missing_or_stale_evidence_at_start`

These artifacts do not become a policy engine. They only preserve the decision
basis that justified the run.

### Decision vocabulary contract

The new decision layer is:

- `recommended`
  The current first candidate is promotion-worthy now.
- `blocked_for_now`
  A plausible target exists, but missing or stale evidence or explicit blockers
  prevent recommending promotion yet.
- `not_recommended`
  The current visible pressure should not drive the next family decision, even
  if the cluster remains visible.

Exact rules:

1. `recommended` requires:
   - first candidate `promotion_readiness = ready`
   - `confidence.level` is `medium` or `high`
   - no required evidence is stale or missing
2. `blocked_for_now` applies when:
   - there is a plausible candidate, and
   - the recommendation is being held by blocker reasons or freshness gaps
3. `not_recommended` applies when:
   - there is no plausible next-family action, or
   - the visible candidate is a durable helper-surface hold like the current
     `money/round` path

### Evidence model

M33 splits decision blockers from evidence state.

That means:

- blocker reasons explain **why the decision is held**
- evidence state explains **what proof is present, missing, or stale**
- warnings explain **what still deserves caution even if the verdict is usable**

The artifact should not force the reader to infer freshness from missing paths or
silent count changes.

### Change-awareness contract

Delta is computed against the last validated analysis artifact at the same path.

`delta_from_previous` must include at minimum:

- `previous_generated_at`
- `previous_decision_status`
- `previous_recommendation_status`
- `decision_changed`
- `top_candidate_changed`
- `reasons_added[]`
- `reasons_cleared[]`
- `evidence_changes[]`
- one single-line human-readable summary

If there is no prior artifact, the delta block must say that explicitly instead
of fabricating a baseline.

### Architecture ASCII diagram

```text
CORPUS + CURRENT REPO TRUTH
===========================
semantic-families/corpus/rust-function.toml
        +
coverage.rs
        |
        v
coverage.latest.json
        |
        v
recommend.rs
        |
        +--> ranked_candidates[]            (existing ranking view)
        +--> decision_summary              (new M33 verdict)
        +--> evidence_summary              (new M33 evidence state)
        +--> delta_from_previous           (new M33 change view)
        |
        v
analysis/recommendation.latest.json
        |
        +--> refresh-promotion-recommendation
        |       |
        |       v
        |   <family>/recommendation.latest.json
        |
        +--> emit-promotion-execution / emit-promotion-blocker
                |
                v
        promotion.execution.json / blocker.report.json
```

## Implementation Plan

### Step 1 - Freeze the decision schema contract

Primary files:

- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`

Work:

1. Add the new decision/evidence/delta structs and enums.
2. Bump schema versions exactly where M33 changes artifact meaning.
3. Extend validators so contradictory combinations fail fast.
4. Keep `recommendation_status` as a compatibility field in the analysis
   artifact.
5. Keep all artifact paths unchanged.

Acceptance for Step 1:

- every changed artifact type has a validator that enforces the M33 rules
- no artifact path changes
- old contradictions now fail in tests instead of relying on maintainer judgment

### Step 2 - Build the M33 decision projection in `recommend.rs`

Primary files:

- `xtask/src/family/recommend.rs`

Work:

1. Derive the new `decision_status` from the existing candidate ranking,
   readiness, confidence, and durable-hold logic.
2. Project blocker reasons separately from evidence state.
3. Load the previous validated analysis artifact if it exists.
4. Compute `delta_from_previous` deterministically.
5. Keep deterministic byte reuse when the normalized logical output has not
   changed.

Acceptance for Step 2:

- the current `money/round` durable-hold path renders as
  `decision_status = "not_recommended"` with explicit explanation
- a held but still plausible candidate path can render
  `decision_status = "blocked_for_now"`
- unchanged logical output still reuses prior bytes where the existing
  determinism contract allows it

### Step 3 - Thread the decision basis through downstream artifacts

Primary files:

- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`

Work:

1. Update `refresh-promotion-recommendation` so the family-scoped artifact
   records which analysis artifact justified the chosen family and what that
   basis said.
2. Update execution and blocker artifact emission so they carry the same basis.
3. Ensure downstream validators reject family-scoped artifacts that disagree
   with the referenced analysis basis.
4. Keep target-language truth explicit so M32 honesty is preserved.

Acceptance for Step 3:

- family-scoped recommendation artifacts cite the analysis basis directly
- execution and blocker artifacts preserve that basis without recomputing policy
- no downstream artifact implies repo-wide multi-language readiness

### Step 4 - Lock the docs and canonical wedge

Primary files:

- `semantic-families/README.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `CHANGELOG.md`

Work:

1. Teach the new M33 decision vocabulary exactly once and reuse the same wording
   everywhere.
2. Document the current truthful wedge:
   the helper-surface `money/round` path is visible but not the next family.
3. Explain how `recommended`, `blocked_for_now`, and `not_recommended` differ.
4. Keep the M32 bounded second-language claim narrow and explicit.

Acceptance for Step 4:

- a maintainer reading the docs sees the same vocabulary the artifacts emit
- docs and artifacts describe the same current wedge
- no doc widens M32 into broad TypeScript readiness

## Code Quality Review

The biggest code-quality risk here is not under-engineering. It is semantic
duplication.

If the M33 decision rules are implemented twice, once in `recommend.rs` and once
again inside downstream emission code, the artifact chain will drift.

So the quality bar is:

1. one source of decision truth in the analysis projection
2. downstream artifacts only carry forward that truth
3. validators enforce consistency instead of each writer inventing its own rules

This is a minimal-diff plan.

Do not introduce:

- a new policy module tree
- free-form JSON extension maps
- artifact-specific copies of the same decision rules

## Test Review

100% branch coverage for the new decision states is required.

### Code path coverage

```text
ANALYSIS ARTIFACT
=================
[+] xtask/src/family/recommend.rs
    ├── [ADD] emits `decision_status = recommended`
    ├── [ADD] emits `decision_status = blocked_for_now`
    ├── [ADD] emits `decision_status = not_recommended`
    ├── [ADD] computes `delta_from_previous` when prior artifact exists
    ├── [ADD] emits explicit "no previous artifact" delta when baseline absent
    └── [ADD] preserves deterministic output when logical recommendation is unchanged

ARTIFACT VALIDATION
===================
[+] xtask/src/family/promotion_artifacts.rs
    ├── [ADD] rejects `recommended` when first candidate is still held
    ├── [ADD] rejects blocked recommendations that omit required blocker or evidence state
    ├── [ADD] rejects family-scoped recommendation artifacts with mismatched analysis basis
    ├── [ADD] rejects execution/blocker artifacts missing carried decision-basis fields
    └── [ADD] accepts truthful M33 artifacts on existing paths

DOWNSTREAM EMISSION
===================
[+] xtask/src/family/promotion_artifacts.rs
    ├── [ADD] family-scoped recommendation copies analysis-basis verdict
    ├── [ADD] promotion execution carries analysis basis without widening support claims
    └── [ADD] blocker artifact preserves open blockers and stale/missing evidence
```

### User-flow coverage

```text
MAINTAINER DECISION FLOW
========================
[+] Current analysis artifact
    ├── [ADD] current `money/round` durable-hold path reads as `not_recommended`
    ├── [ADD] candidate-with-gaps path reads as `blocked_for_now`
    └── [ADD] truly promotion-ready path reads as `recommended`

CHANGE AWARENESS
================
[+] Re-run recommendation after evidence changes
    ├── [ADD] status flip is visible in `delta_from_previous`
    ├── [ADD] blocker reasons added/cleared are visible
    └── [ADD] stale evidence is called out explicitly

PROMOTION CHAIN
===============
[+] Maintainer picks a family and starts a promotion run
    ├── [ADD] family-scoped recommendation cites the analysis basis
    ├── [ADD] execution artifact preserves starting verdict
    └── [ADD] blocker artifact preserves starting blockers instead of forcing re-interpretation
```

### Required regression tests

Add or preserve tests proving:

- the analysis artifact can render all three new decision verdicts
- the current `money/round` helper-surface wedge is `not_recommended`, not a
  vague held recommendation
- missing evidence and stale evidence are explicit fields, not inferred
- `delta_from_previous` is accurate when:
  - there is no prior artifact
  - only blockers change
  - the top candidate changes
  - the top-level decision changes
- family-scoped recommendation artifacts reject mismatched analysis-basis paths
  or hashes
- execution and blocker artifacts preserve the analysis basis fields
- existing path validation rules still hold
- bounded M32 target-language truth is preserved in downstream artifacts

### Failure modes by codepath

| Codepath | Realistic failure | Test required | Error handling / visible truth |
|---|---|---|---|
| Analysis projection | Artifact says `recommended` while the first candidate is still held | Yes | Validator must reject the artifact |
| Delta projection | Artifact claims "no change" even though blocker reasons changed | Yes | `delta_from_previous` must diff reason sets deterministically |
| Freshness handling | Recommendation uses stale basis but does not say so | Yes | `evidence_summary` must carry `stale_evidence` explicitly |
| Family-scoped recommendation emission | Chosen family artifact silently diverges from the analysis basis | Yes | Validator must compare basis path/hash and fail |
| Promotion execution emission | Execution artifact loses the starting blocker context | Yes | Artifact must carry the open blockers and evidence state at start |
| Blocker emission | Blocker report explains runtime failure but not pre-existing recommendation blockers | Yes | Artifact must preserve the decision basis and unresolved blockers |
| Docs | README or roadmap implies broad TypeScript readiness | Yes | Review plus doc update must keep the M32 boundary explicit |

Critical gap rule:

If any new decision path lacks both a regression test and an explicit visible
truth field, M33 is not done.

### Commands to run

Run at minimum:

```bash
cargo xtask family coverage --format json
cargo xtask family recommend --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo test -p xtask family_refresh_promotion_recommendation
cargo test -p xtask artifact_schema_
cargo test -p xtask recommendation_
cargo test -p xtask
```

The narrow loop can be smaller while implementing. Done still requires the full
artifact validation loop plus the relevant `xtask` regression bed.

## Performance Review

There is no meaningful runtime hot-path risk in M33.

The real risks are engineering-performance risks:

- recomputing decision policy in multiple places
- adding unstable delta output that defeats deterministic writes
- forcing maintainers to read more artifacts, not fewer

Keep the implementation boring:

- one extra validated read of the previous analysis artifact
- one projection pass
- one set of validators

Do not turn M33 into a stateful history system.

## Distribution Surface

M33 introduces no new binary, package, container, or service.

Its distribution surface is:

- the current analysis artifact
- the family-scoped promotion artifacts
- the validator contract
- the maintainer docs

If a maintainer still needs hidden chat context to explain the current
recommendation after M33 lands, then M33 did not actually ship.

## Worktree Parallelization Strategy

This plan has a narrow parallelization window, but the core code changes stay
mostly sequential because they share the same artifact contract.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| A. Schema and validator freeze | `xtask/src/family/promotion_artifacts.rs`, `xtask/src/family/paths.rs` | - |
| B. Decision projection and delta logic | `xtask/src/family/recommend.rs` | A |
| C. Downstream artifact propagation and CLI regression coverage | `xtask/src/family/promotion_artifacts.rs`, `xtask/src/lib.rs` | A, B |
| D. Docs and closeout | `semantic-families/README.md`, `docs/`, `CHANGELOG.md` | A, B, C |

### Parallel lanes

- Lane A: `A -> B -> C`
  Sequential critical path. All three steps are tightly coupled through the
  shared artifact contract.
- Lane B: `D`
  Optional docs lane. It can begin only after Step A freezes the vocabulary,
  but it must not merge until Steps B and C are complete.

### Execution order

Launch the sequential code lane first:

```text
A -> B -> C
```

If the vocabulary is frozen and a second worktree is useful, run docs in
parallel late:

```text
(A complete) -> B + partial D -> C -> finalize D
```

### Conflict flags

- `xtask/src/family/promotion_artifacts.rs` belongs to the sequential lane.
  Do not split ownership of that file across worktrees.
- Docs must not guess the final field names before the validator contract is
  frozen.
- If implementation discovers that `promotion_artifacts.rs` needs repeated late
  edits after docs start, collapse back to single-lane execution.

## Completion Summary

- Step 0: Scope Challenge
  Accepted as-is. The minimum honest diff already spans analysis artifacts,
  downstream promotion artifacts, validators, tests, and docs.
- Architecture Review
  One bounded recommendation-quality pass over the existing artifact chain.
- Code Quality Review
  One source of decision truth, downstream carry-forward only, no new policy
  subsystem.
- Test Review
  Full decision-state, delta-state, and downstream artifact coverage required.
- Performance Review
  No runtime bottleneck; determinism drift and duplicated logic are the real
  risks.
- NOT in scope
  Written.
- What already exists
  Written.
- Failure modes
  Written.
- Parallelization
  One sequential code lane plus one optional late docs lane.
- Distribution
  Explicitly limited to current artifact paths and docs.

## Implementation Guardrail

If implementation discovers that M33 cannot produce a trustworthy recommendation
decision surface without also changing:

- corpus accounting policy
- `spec-core` family capability
- a new family promotion
- broad target-language semantics

stop.

That is not "small spillover." That is a different milestone trying to leak into
a bounded M33 plan.
