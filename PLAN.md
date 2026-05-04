<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260503-232225.md -->
# M31 - Shared-Core Extraction And Escape-Hatch Containment

Status: **authoritative implementation plan**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-04**
Supersedes: **M30 - Add Second Bounded TypeScript Family Proof**
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260504-122024.md`**
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**
Execution note: **Do not create `ORCH_PLAN.md` up front. Create it only if the post-foundation lanes below are actually split into separate worktrees.**

## Objective

Make the repository able to say one precise, enforceable thing:

> for seam units, the shared portability boundary is explicit, Rust-specific
> execution details are boxed behind that boundary, and every read-side truth
> surface stays honest about when portability claims are contaminated.

This is the full M31 claim.

M31 is not a second-language execution milestone. It is the milestone that
stops the repo from calling something "shared" when it is still leaning on
Rust-only execution machinery.

## Decision

M31 ships as a seam-only shared-core extraction milestone with one dedicated
portability contract module in `spec-core`.

That means:

1. Do **not** spread new portability logic across `passport.rs`,
   `semantic_review.rs`, `export.rs`, and `spec-cli/src/commands.rs` as separate
   ad hoc rules.
2. Introduce one explicit contract module, `spec-core/src/portability.rs`,
   that becomes the sole cross-surface source of truth for seam portability
   classification and projection.
3. Reuse the existing marker, freshness, proof, and health machinery where it
   is already correct. This is a boundary extraction, not a style rewrite.

## Problem Statement

M30 proved that additive authored `body.typescript` survives across two
promoted `kind:function` families. Good.

It did not answer the next architectural question:

- what in this repo is genuinely platform-neutral
- what is still Rust-specific execution machinery
- what must be true before a seam with Rust-only escape hatches can be treated
  as a credible portability surface

Today those answers exist, but they are scattered:

- `spec-core/src/backend_execution.rs` classifies backend-only seam markers and
  computes the backend-execution digest
- `spec-core/src/escape_hatch.rs` computes current proof surfaces and the live
  escape-hatch gate
- `spec-core/src/passport.rs` already splits authored-truth freshness from
  backend-execution freshness and projects read-side truth
- `spec-core/src/semantic_review.rs` already distinguishes backend-only meaning
  preserved vs leaked for supported seam surfaces
- `spec-core/src/export.rs` and `spec-cli/src/commands.rs` already expose those
  signals on read-side surfaces

That is too fragmented to count as a trustworthy shared-core boundary.

## Locked Decisions

These decisions remove the remaining ambiguity. They are part of the milestone
contract, not suggestions.

### 1. Dedicated contract module

Create `spec-core/src/portability.rs`.

This module owns the seam portability contract and is the only place allowed to
compose:

- backend execution markers
- backend execution digest
- current proof surfaces
- escape-hatch gate state
- portability contamination summary
- read-side portability projection inputs

`backend_execution.rs` and `escape_hatch.rs` remain reusable helpers. Read-side
consumers stop recomputing portability truth directly.

### 2. Seam-only scope

M31 covers `kind:data` and `kind:sum` only.

It does **not** redefine:

- function-family portability claims
- packet promotion semantics
- TypeScript execution semantics
- second-language proof policy

### 3. Validator policy stays split by problem class

Keep the current hard validation errors for illegal shared-surface escape
hatches on seam kinds:

- top-level `contract`
- top-level `deps`
- top-level `imports`
- top-level `body.rust`
- top-level `body.typescript`

Do **not** turn allowed backend-specific details into validation failures.

Allowed Rust-specific details remain:

- `methods[].lowering.rust.body`
- `backends.rust.derives`

They remain valid authored input, but their portability consequences must be
projected truthfully.

### 4. Marker taxonomy is fixed

The canonical seam backend marker classes remain:

- `DomainLowering`
  Meaning: Rust-specific lowering that participates in domain behavior.
  Effect: contaminating unless the broader seam projection still lands in an
  honest meaning-preserved state.

- `ProofHelperLowering`
  Meaning: Rust-specific helper/example/proof-only lowering.
  Effect: backend-only, visible, non-contaminating by itself.

- `BackendRustDerives`
  Meaning: Rust-only implementation detail with no semantic claim by itself.
  Effect: backend-only and health-neutral by itself.

M31 does not invent new marker kinds.

### 5. Reuse existing public read-side fields first

Default stance: reuse the current public fields and make them truthful through
one shared projection path:

- `freshness.backend_execution_status`
- `markers`
- `proof_coverage`
- `escape_hatch_gate`
- `semantic_review`

Add a new public JSON field only if a concrete truth gap remains after the
shared projection is wired in.

### 6. Escape-hatch containment is visible truth

If portability claims are contaminated, users must be able to see that in the
surfaces they already read:

- passport JSON
- `spec status`
- `spec export`
- semantic-review summaries

No hidden maintainer-only interpretation layer.

### 7. M32 boundary is hard

M31 stops before:

- TypeScript executable semantic review
- second-language lowering classification
- second-language prove/certify expansion
- new portability logic for `kind:function`

If implementation pressure asks for those, stop and rewrite the plan.

## Done Means

M31 is complete only when all of the following are true:

1. `spec-core` exposes one explicit seam portability contract module that
   read-side surfaces consume.
2. backend execution marker classification, proof-surface evaluation, and
   portability contamination projection are no longer re-derived in multiple
   unrelated places.
3. seam units still hard-reject illegal shared-surface escape hatches at
   validation time.
4. allowed Rust-specific seam details remain additive, but their portability
   consequences are projected truthfully through passport, export, status, and
   semantic review.
5. freshness splitting between authored truth and backend execution remains
   intact and is clearly part of the portability contract.
6. open escape-hatch proof gates remain seam-only and continue to demote only an
   otherwise-valid unit to `incomplete`.
7. supported seam semantic review continues to distinguish:
   - aligned
   - under-specified
   - backend-only meaning preserved
   - backend-only semantics leaked
8. supported-function and unsupported-function truth remain unchanged outside
   the seam portability path.
9. the roadmap doc explicitly says `M31` then `M32`, not the older `M28` /
   `M29` shape.
10. the test suite proves the boundary rather than merely compiling through it.

## NOT in Scope

The following work was considered and is explicitly deferred:

- TypeScript executable semantic review
  Reason: that is M32, not M31.
- Repo-wide second-language lowering policy
  Reason: M31 is seam containment, not generalized portability across all kinds.
- New marker classes beyond the current seam trio
  Reason: widens ontology before proving the current boundary is sufficient.
- New public CLI command for portability
  Reason: existing read-side surfaces are the intended source of truth.
- Packet promotion, coverage ranking, or recommendation-engine changes
  Reason: not the current bottleneck.
- Full privacy cleanup of legacy helper modules
  Reason: logical boundary comes first; purely mechanical visibility cleanup is
  optional unless compile-forced.
- Non-seam example expansion
  Reason: the canonical seam fixtures are enough to prove M31.
- New distribution artifacts
  Reason: the deliverable is repo truth, not a new binary/package/container.

## What Already Exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Backend-only seam marker detection | `spec-core/src/backend_execution.rs` | Reuse `collect_backend_execution_markers`, `summarize_backend_execution_markers`, and `compute_backend_execution_digest`. Do not rebuild marker discovery. |
| Escape-hatch proof surfaces and gate semantics | `spec-core/src/escape_hatch.rs` | Reuse `current_proof_surfaces` and `evaluate_escape_hatch_gate`. Move composition ownership, not the proof rules. |
| Split freshness between authored truth and backend execution | `spec-core/src/passport.rs` | Reuse `resolve_passport_freshness*`. Keep the split and route projection through the shared contract. |
| Supported seam semantic-review behavior | `spec-core/src/semantic_review.rs` | Reuse the current verdict vocabulary. Replace direct marker summarization with shared portability inputs. |
| Read-side health demotion rules | `spec-cli/src/commands.rs` | Reuse the existing `valid -> incomplete/failing` demotion behavior. Preserve the "demote only otherwise-valid rows" rule. |
| Export-side truth enrichment | `spec-core/src/export.rs` | Reuse the current export enrichment path. Route it through the same portability projection used by passports/status. |
| Seam validation guardrails | `spec-core/src/validator.rs` | Reuse current hard-reject rules. Tighten wording only if clarity is missing. |
| Canonical seam fixtures and tests | `pricing/discount_policy`, `pricing/checkout_quote`, `spec-cli/tests/cli.rs` | Reuse as the main proof bed. No new example family is needed. |
| Public milestone narrative | `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` | Rewrite in place as part of done. |

## Step 0 - Scope Challenge

This milestone touches more than 8 files. Normally that is a smell.

Here it is justified because the milestone definition is shared truth across
multiple read-side consumers. Reducing below this surface would leave at least
one of passport, semantic review, export, status, or the public roadmap telling
a stale or partial story.

The minimum honest implementation surface is:

- one new portability contract module
- the existing seam helpers it composes
- the passport projection path
- supported seam semantic review
- export/status consumers of projected truth
- validator wording only if needed
- the roadmap doc

Anything larger is scope creep. Anything materially smaller is fake confidence.

## Closed Implementation Surface

### Primary modules

- `spec-core/src/portability.rs` (new)
- `spec-core/src/backend_execution.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/export.rs`
- `spec-core/src/lib.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `PLAN.md`

### Allowed mechanical spillover

Only if compile- or fixture-forced:

- `spec-core/src/types.rs`
- `spec-core/src/molecule_evidence.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/graph.rs`
- `spec-core/src/schema/unit.spec.json`

If implementation needs new semantics outside this surface, stop and rewrite
the plan before continuing.

## Architecture

### Current shape

```text
Rust-specific seam details
backend_execution.rs     escape_hatch.rs
        |                      |
        +--------------+       |
        |              |       |
        v              v       v
   passport.rs   semantic_review.rs
        |              |
        +-------+      |
        v       v      v
   export.rs  status / commands.rs

Truth exists, but the portability boundary is implicit and spread around.
```

### Target shape

```text
Authored seam truth
data.fields / sum.variants / constructors / methods
                |
                v
          validator.rs
  hard-reject illegal shared-surface escape hatches
                |
                v
        portability.rs
 canonical seam portability contract
                |
                +-- composes backend_execution.rs
                +-- composes escape_hatch.rs
                +-- preserves freshness split from passport.rs
                +-- computes shared projection inputs
                |
                +-----------+--------------+--------------+
                v           v              v              v
          passport.rs   semantic_review.rs  export.rs   spec status
                                                    spec-cli/src/commands.rs
```

The important change is not "one more module exists." The important change is
that every read-side surface stops re-deriving portability truth differently.

## Portability Contract

`spec-core/src/portability.rs` owns seam portability composition.

It must expose enough shared structure that downstream code can consume one
projection instead of recomputing pieces. The exact Rust names may differ, but
the ownership boundary cannot.

### Required responsibilities

1. Canonical portability marker identity and summary.
2. Canonical backend-execution digest access for seam units.
3. Canonical proof-surface evaluation for atom and molecule containment.
4. Canonical read-side portability projection.
5. Canonical contamination summary for supported seam semantic review.

### Expected API shape

```text
collect_portability_markers(spec) -> ...
summarize_portability_markers(spec) -> ...
compute_portability_backend_digest(spec) -> ...
evaluate_portability_gate(spec, passport, context) -> ...
project_portability_truth(spec, passport, context) -> PortabilityProjection
```

### Non-negotiable wiring rules

- `portability.rs` may call into `backend_execution.rs` and `escape_hatch.rs`.
- `passport.rs`, `semantic_review.rs`, `export.rs`, and `spec-cli/src/commands.rs`
  must not add fresh portability logic outside that contract.
- `semantic_review.rs` must stop treating `summarize_backend_execution_markers`
  as its own private source of truth for supported seam reviews.

## Read-Side Truth Rules

These rules are locked.

### Validation

- Illegal shared-surface seam escape hatches remain hard errors.
- Allowed Rust-specific details remain valid authored input.
- Validation does not decide portability health by itself.

### Freshness

- Authored-truth freshness and backend-execution freshness remain separate.
- Seam portability claims continue to consider backend-execution staleness, not
  just authored-contract staleness.

### Escape-hatch proof gate

- Required proof surfaces remain `atom` and `molecule`.
- An open gate demotes only an otherwise-valid seam unit to `incomplete`.
- A stale unit remains `stale`, not `incomplete`.

### Semantic review

- Supported seam reviews consume the shared portability projection.
- `ProofHelperLowering` and `BackendRustDerives` remain backend-only but
  meaning-preserved unless another supported-seam semantic problem exists.
- `DomainLowering` remains the contaminating marker class for
  `backend_only_semantics_leaked`.

### Export and status

- `spec export` and `spec status` must project the same portability truth as
  the passport path.
- No read-side consumer may preserve stale portability claims that the shared
  projection would now drop.

## Implementation Plan

### Step 1 - Introduce the canonical portability contract module

Goal: create `spec-core/src/portability.rs` and move seam portability ownership
there.

Files:

- `spec-core/src/portability.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/backend_execution.rs` if helper extraction is needed
- `spec-core/src/escape_hatch.rs` if helper visibility changes are needed

Required work:

- add `pub mod portability;` in `spec-core/src/lib.rs`
- define the shared projection types
- route marker summary, backend-execution digest access, and gate composition
  through `portability.rs`
- keep `backend_execution.rs` and `escape_hatch.rs` focused on helper logic
  rather than cross-surface orchestration

Exit condition:

- downstream consumers can ask one portability module for seam portability
  truth instead of composing the pieces themselves

### Step 2 - Rewire passport projection around the portability contract

Goal: make passport truth the reference implementation for seam portability
projection.

Files:

- `spec-core/src/passport.rs`

Required work:

- route `project_passport_truth_with_context` through the portability contract
- keep `markers`, `proof_coverage`, `escape_hatch_gate`, and
  `freshness.backend_execution_status` truthful
- preserve current non-test proof-state behavior for `spec build` /
  `spec generate`
- keep `apply_projected_passport_truth` as the sink that writes the projected
  portability truth into the public passport surface

Exit condition:

- passport write and read paths consume one seam portability projection

### Step 3 - Rewire supported seam semantic review to consume the shared contract

Goal: stop `semantic_review.rs` from being a second portability classifier.

Files:

- `spec-core/src/semantic_review.rs`

Required work:

- replace direct backend marker summarization for supported seam review with
  shared portability inputs
- preserve current verdict vocabulary and compatibility-key behavior
- keep supported-function and unsupported-function paths unchanged unless
  compile-local adjustments are forced

Exit condition:

- supported seam semantic review still returns the same honest verdict classes,
  but now does so from the shared portability contract

### Step 4 - Rewire export and status/read-side health surfaces

Goal: make `spec export` and `spec status` consume the same projected truth.

Files:

- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`

Required work:

- route export passport enrichment through the shared portability projection
- keep `apply_escape_hatch_gate_to_health` and `apply_semantic_review_to_health`
  as the CLI health demotion sinks, but feed them truth produced by the shared
  contract
- preserve current "demote only otherwise-valid rows" behavior
- prove export, passport, and status agree on the same authored fixture set

Exit condition:

- export, passport, and status tell the same seam portability story

### Step 5 - Tighten seam validation wording only where clarity is missing

Goal: keep validator policy simple and explicit.

Files:

- `spec-core/src/validator.rs`

Required work:

- retain hard errors for illegal shared-surface seam escape hatches
- update wording only if it helps distinguish:
  - invalid shared-surface authored shape
  - valid backend-specific detail that later contaminates portability claims

Exit condition:

- validator errors still guard the authored boundary without pretending to be
  the full portability decision engine

### Step 6 - Rewrite the public roadmap and close out terminology

Goal: make the public doc say what the code now actually means.

Files:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`

Required work:

- rewrite the roadmap so it uses explicit `M31` / `M32` sequencing
- describe M31 as shared-core extraction plus escape-hatch containment
- describe M32 as the next executable-truth question, not a retroactive part of
  M31
- keep `PLAN.md`, the roadmap, and the landed code on the same terms, with
  `PLAN.md` treated as authority during execution rather than a worker-owned
  edit surface

Exit condition:

- the roadmap and the code tell the same story

## Code Path Diagram

```text
[1] Seam authored source
    data.fields / sum.variants / constructors / methods
        |
        +-- invalid top-level contract/deps/imports/body? --> validator hard error
        |
        `-- valid seam with optional Rust-specific details
                |
                v
[2] Portability contract
    portability.rs
        |
        +-- collect markers
        +-- compute backend-execution digest
        +-- evaluate current proof surfaces
        +-- evaluate escape-hatch gate
        +-- summarize contamination state
                |
                v
[3] Read-side projection
    passport.rs
    semantic_review.rs
    export.rs
    spec-cli/src/commands.rs
        |
        +-- freshness split preserved
        +-- gate-open -> incomplete only if base row was valid
        +-- stale beats incomplete
        +-- semantic-review meaning-preserved/leaked stays aligned
```

Every branch above needs tests.

## Test and Proof Plan

100% of the new portability codepaths must be covered. This milestone is easy
to fake with one green compile. That is not good enough.

### Code path coverage

```text
PORTABILITY CORE
===========================
[+] spec-core/src/portability.rs
    ├── marker collection matches legacy behavior
    ├── backend-execution digest stays stable for authored-only seam edits
    ├── backend-execution digest changes for backend-only seam edits
    ├── helper/example lowerings stay non-contaminating
    ├── domain lowerings stay contaminating
    └── derives stay backend-only and health-neutral

PASSPORT PROJECTION
===========================
[+] spec-core/src/passport.rs
    ├── preserve mode keeps truthful seam projection when compatibility still matches
    ├── stale backend execution drops freshness correctly
    ├── open gate demotes otherwise-valid seam rows only
    └── projected fields match portability contract output

SEMANTIC REVIEW
===========================
[+] spec-core/src/semantic_review.rs
    ├── supported sum/data aligned still pass
    ├── helper-only markers -> backend_only_meaning_preserved
    ├── domain lowering -> backend_only_semantics_leaked
    └── supported/unsupported function truth unchanged

READ-SIDE SURFACES
===========================
[+] spec-core/src/export.rs
    ├── exported passports recompute seam projection truthfully
    └── stale/open-gate seam projections do not preserve false green state

[+] spec-cli/src/commands.rs + spec-cli/tests/cli.rs
    ├── spec status reflects gate-open -> incomplete
    ├── stale beats incomplete
    ├── failing/invalid beats portability demotion
    └── export/status/passport agree for the same fixture set
```

### Required regression tests

Add or preserve tests proving:

- authored-only seam edits do not change backend-execution digest
- backend-only seam edits do change backend-execution digest
- helper/example methods do not silently flip into domain-lowering contamination
- domain-lowering seams remain visibly contaminating in semantic review
- open escape-hatch gate demotes only otherwise-valid units
- stale seam units remain stale even when the gate is also open
- export-side passport enrichment matches live status projection
- supported-function and unsupported-function truth surfaces do not regress
  while seam portability logic is extracted

### Failure modes by codepath

| Codepath | Realistic production failure | Test required | Error handling / visible truth |
|---|---|---|---|
| Portability marker projection | Helper-only lowering gets misclassified as domain contamination | Yes, unit test in `portability.rs` or extracted helper tests | Must remain visible but health-neutral unless another semantic problem exists |
| Backend-execution digest | Authored-only seam edit incorrectly flips backend freshness | Yes, regression test | Otherwise status/export show fake stale noise |
| Passport projection | Preserve mode keeps stale seam portability claims | Yes, passport projection test | Must drop stale truth instead of preserving false green state |
| Escape-hatch gate | Open gate demotes already-stale row to incomplete | Yes, CLI/status regression | `stale` must beat `incomplete` |
| Semantic review | Supported seam review re-derives markers differently from passport/status | Yes, cross-surface regression | Must consume the shared contract so verdict and health stay aligned |
| Export enrichment | Export shows stale or mismatched seam truth compared to status | Yes, export regression | Export/passport/status must agree on the same fixture set |
| Validator wording | Users cannot distinguish invalid authored shape from valid-but-contaminating backend detail | Yes, validation assertion if wording changes | Error messaging must stay explicit and not overclaim |
| Roadmap rewrite | Public docs still say M28/M29 while code ships M31 semantics | Yes, closeout review | Closeout must treat roadmap rewrite as part of done |

Critical gap rule:

If any path above lacks both a regression test and a truthful read-side surface,
the milestone is not done.

### Commands to run

Run at minimum:

```bash
cargo test -p spec-core backend_execution
cargo test -p spec-core escape_hatch
cargo test -p spec-core passport
cargo test -p spec-core semantic_review
cargo test -p spec-core export
cargo test -p spec-cli --test cli
cargo test
```

A narrower loop is fine during implementation. Done still requires full
workspace `cargo test`.

## Performance Review

There is no meaningful runtime-performance risk in M31. This is a correctness
and truth-surface alignment milestone.

The real performance risk is engineering performance:

- duplicating portability logic in multiple files creates slow, error-prone
  follow-on work
- stale truth surfaces create false-green debugging loops
- conflating authored freshness with backend-execution freshness turns every
  future portability milestone into archaeology

Recommendation: optimize for explicit single ownership, not for shaving a few
lines off the diff.

## Distribution Surface

M31 introduces no new binary, package, or container.

Its distribution surface is repo truth:

- the shared portability contract in `spec-core`
- seam portability projection in passport/export/status/read-side health
- semantic-review behavior for supported seam surfaces
- the rewritten roadmap doc

Code without those public truth surfaces is not a real M31 ship.

## Worktree Parallelization Strategy

This plan has real parallelization opportunity, but only after the portability
contract shape is locked.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| A. Portability contract extraction | `spec-core/src/` portability, backend execution, escape hatch, lib exports | - |
| B. Passport + semantic-review integration | `spec-core/src/` passport, semantic review | A |
| C. Export + status/read-side integration | `spec-core/src/` export, `spec-cli/src/` commands, `spec-cli/tests/` | A |
| D. Roadmap rewrite + closeout | `docs/`, `PLAN.md` | A, then final terminology from B/C |

### Parallel lanes

- Lane A: `A`
  Sequential foundation lane. No split before this lands.
- Lane B: `B`
  Runs after Lane A. Owns passport and semantic-review integration.
- Lane C: `C`
  Runs after Lane A in parallel with Lane B. Owns export/status/CLI truth
  surfaces.
- Lane D: `D`
  Runs after B + C. Docs last so they describe the actual landed contract.

### Execution order

Launch Lane A first.

After A is merged or otherwise stabilized, launch Lane B and Lane C in parallel
worktrees.

Merge B + C, then do Lane D last.

### Conflict flags

- Lanes B and C both depend on the final API shape from
  `spec-core/src/portability.rs`. Freeze that API in Lane A before splitting.
- Passport ownership belongs to Lane B. Lane C may consume projected truth but
  should not take opportunistic ownership of `spec-core/src/passport.rs`.
- If the contract shape is still changing, do **not** split into worktrees yet.
  Sequential execution is cheaper than merge-fighting a moving interface.

If the work is not split into worktrees, execute sequentially in the same
order:

```text
A -> B -> C -> D
```

## Completion Summary

- Step 0: Scope Challenge
  Accepted as-is, because the minimum honest surface already spans multiple
  read-side truth consumers.
- Architecture Review
  One dedicated seam portability contract module with existing helpers reused
  behind it.
- Code Quality Review
  Centralize ownership, do not rewrite working helpers for style.
- Test Review
  Explicit portability codepath diagram plus mandatory regressions above.
- Performance Review
  No runtime bottleneck. Correctness and truth-surface alignment are the real
  risks.
- NOT in scope
  Written.
- What already exists
  Written.
- Failure modes
  Written.
- Parallelization
  Four steps, two post-foundation lanes that can run in parallel.
- Distribution
  Explicitly limited to repo truth surfaces.

## Implementation Guardrail

If implementation discovers that the current seam portability story cannot be
made truthful without adding second-language executable semantics, stop.

That is not "small spillover." That is M32 trying to leak backward into M31.
