# M44 - Freeze The Shared-Core Portability Contract

Status: **authority plan**  
Milestone family: **architecture-follow-on**  
Implementation readiness: **ready-now**  
Next artifact kind: **authority_plan**  
Autoplan ready: **yes**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Last rewritten: **2026-05-10**  
Source design doc: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260510-080847.md`**  
Supersedes: **M43 - Promote `function.helper.identity_passthrough.v1`**

## Executive Verdict

M44 is the contract-freeze milestone that has to land before any broader portability or second-language story gets louder.

The repo already has the raw pieces:

- seam-specific validator restrictions
- backend-only marker collection
- escape-hatch proof gating
- portability projection
- semantic review consumption
- passport / export / status read-side truth

What it does not have is one explicit owner for the seam portability policy. Right now the boundary is real in practice but still partly distributed across `validator.rs`, `portability.rs`, `semantic_review.rs`, and read-side consumers. That is close enough to work, but not clean enough to trust when the next backend or portability claim arrives.

M44 fixes that by adding one new module inside `spec-core`, moving the policy there, rewiring all consumers to read from it, and proving that the current truth stays intact.

This is a real code milestone. Not docs-only. Not a new crate. Not TypeScript execution. One bounded extraction lake.

## Problem Statement

M43 closed the last honest Rust-family promotion loop on this branch.

The current branch state already says:

- promoted Rust families are not the blocker
- the family-analysis stop-state is real and should stay real
- the next architectural risk is lying about what is shared-core truth versus what is still Rust-only backend execution detail

That risk is visible in the code today:

- `spec-core/src/validator.rs` hard-rejects illegal seam authored shapes inline
- `spec-core/src/backend_execution.rs` classifies raw backend markers
- `spec-core/src/escape_hatch.rs` computes the atom / molecule gate
- `spec-core/src/portability.rs` projects markers, contamination, digest, and gate state
- `spec-core/src/semantic_review.rs` uses that truth to decide supported seam meaning
- `spec-core/src/passport.rs`, `spec-core/src/export.rs`, and `spec-cli/src/commands.rs` surface the projected truth
- `xtask/src/family/analysis_core/*` already demonstrates the right pattern on the family-analysis side: one explicit shared decision seam, many consumers

The missing artifact is not more analysis. The missing artifact is one explicit portability contract owner inside `spec-core`.

## Current Repo Truth

### Live code surfaces

- `spec-core/src/backend_execution.rs` already distinguishes:
  - `DomainLowering`
  - `ProofHelperLowering`
  - `BackendRustDerives`
- `spec-core/src/escape_hatch.rs` already treats seam proof requirements as:
  - `atom`
  - `molecule`
- `spec-core/src/portability.rs` already projects:
  - portability markers
  - contamination summary
  - backend execution digest
  - proof surfaces
  - escape-hatch gate
- `spec-core/src/semantic_review.rs` already differentiates:
  - backend-only meaning preserved
  - backend-only semantics leaked
  - supported seam subsets
- `spec-core/src/validator.rs` already enforces:
  - `kind:data` and `kind:sum` may not use top-level `contract`
  - `kind:data` and `kind:sum` may not use top-level `deps`
  - `kind:data` and `kind:sum` may not use top-level `imports`
  - `kind:data` and `kind:sum` may not use top-level `body.typescript`
  - `kind:data` and `kind:sum` must leave top-level `body.rust` empty
- `xtask/src/family/analysis_core/*` already proves the repo knows how to freeze a shared decision contract without prematurely extracting a new crate

### Roadmap truth

`docs/ai_promotion_and_multilanguage_milestones_v0.1.md` already says the ordering is:

1. make the seam portability contract explicit
2. contain Rust-specific lowering and escape-hatch detail
3. only then discuss broader second-language work honestly

That roadmap text is directionally correct. M44 is the code milestone that makes it true in the implementation.

## Resolved Decisions

These were still somewhat open in the design draft. They are now locked.

### 1. M44 is not docs-only

M44 must include one small structural extraction in `spec-core`. If the repo only rewrites docs and leaves policy distributed across multiple modules, the portability boundary is still a slogan.

### 2. The extraction lives in `spec-core`, not a new crate

Add exactly one new module:

- `spec-core/src/portability_contract.rs`

Do not create a new workspace crate. Do not split `spec-core`. Do not introduce packaging or dependency choreography for a boundary that is still stabilizing.

### 3. No new machine-readable gate in M44

M44 does **not** add a new top-level portability schema, a new passport gate kind, or a new CLI mode.

The current truth surfaces are sufficient:

- raw backend markers
- contamination summary
- backend execution digest
- current proof surfaces
- `escape_hatch_gate`

The job here is to centralize policy ownership behind those surfaces, not to invent a second contract layer.

### 4. The family-analysis stop-state stays frozen

M44 must not reopen family selection. `xtask/src/family/analysis_core/*` remains precedent, not implementation scope, except for wording or integration changes forced by compile-time or docs alignment.

### 5. No broader TypeScript claim

Allowed authored backend-specific detail remains allowed authored detail. It does **not** become shared portability-safe truth. M44 must preserve that line explicitly.

## What Already Exists

| Sub-problem | Existing owner | M44 action |
|---|---|---|
| Family-analysis decision seam | `xtask/src/family/analysis_core/*` | reuse as precedent, do not expand |
| Raw backend-only marker collection | `spec-core/src/backend_execution.rs` | preserve ownership, reclassify through contract helpers where needed |
| Escape-hatch proof gate | `spec-core/src/escape_hatch.rs` | preserve ownership, align wording/helpers only if needed |
| Portability projection | `spec-core/src/portability.rs` | preserve ownership, remove inline seam-policy assumptions |
| Semantic portability verdict consumption | `spec-core/src/semantic_review.rs` | preserve ownership, consume centralized policy |
| Passport / export / status read-side truth | `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs` | preserve ownership, remove local policy drift |
| Shared authored seam restrictions | `spec-core/src/validator.rs` | move rule ownership into `portability_contract.rs` |
| Roadmap and packet-facing milestone language | `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`, `semantic-families/README.md` | rewrite to match landed boundary |

## Scope

### In scope

1. Add `spec-core/src/portability_contract.rs` as the sole owner of seam portability policy.
2. Move seam-kind helpers and shared authored-surface rules behind that module.
3. Move portability classification rules behind that module:
   - what counts as shared authored seam surface
   - what counts as backend-only but non-contaminating detail
   - what counts as contaminating domain lowering
4. Rewire `validator.rs`, `backend_execution.rs`, `escape_hatch.rs`, `portability.rs`, `semantic_review.rs`, `passport.rs`, `export.rs`, and `spec-cli/src/commands.rs` to consume the centralized contract.
5. Add direct and command-path regressions that prove the moved policy did not change truth accidentally.
6. Refresh roadmap and maintainer-facing docs so the code story and written story match.

### Not in scope

- new family promotion work
- recommendation-policy changes
- corpus-analysis changes
- a new crate split
- first-class TypeScript backend execution
- widening supported function portability claims
- changing `spec generate/build/test` ownership boundaries
- broad `xtask/src/family/*` cleanup unrelated to compile or wording parity
- a schema-version bump unless an additive read-side field change becomes unavoidable

If a schema bump becomes necessary during implementation, stop and make it explicit. The default assumption for M44 is **no schema bump**.

## Architecture Contract

### New module

`spec-core/src/portability_contract.rs` becomes the only place allowed to answer these questions:

- Is this unit a seam portability participant?
- Which authored surfaces are legal shared seam input?
- Which authored surfaces are illegal shared seam shape?
- Which backend markers are backend-only but still honest?
- Which backend markers contaminate portability claims?
- Which stable reason strings or helper text should consumers reuse?

### Ownership table

| Module | Owns after M44 | Must not own |
|---|---|---|
| `portability_contract.rs` | seam-kind helpers, shared authored-surface policy, marker classification policy, stable helper wording | file IO, CLI rendering, artifact loading, cargo commands, proof evaluation |
| `backend_execution.rs` | raw marker detection and backend digest material | portability verdict policy, validator policy, semantic policy wording |
| `escape_hatch.rs` | atom / molecule requirement set and open / closed gate evaluation | marker classification policy, shared authored-surface policy |
| `portability.rs` | composition of markers, digest, proof surfaces, gate, contamination summary | local seam-policy duplication |
| `semantic_review.rs` | semantic verdict logic using projected portability truth | inline classification tables for shared vs backend-only semantics |
| `validator.rs` | hard validation using centralized seam-shape rules | inline duplicate seam restriction strings or rule tables |
| `passport.rs` / `export.rs` / `spec-cli/src/commands.rs` | projection of already-computed truth | local policy branches about what counts as portability-safe |
| `xtask/src/family/analysis_core/*` | family-analysis decision logic | seam portability policy for `spec-core` |

### Dependency graph

```text
AUTHORED SEAM SPEC
      |
      v
portability_contract.rs
      |
      +--> validator.rs
      +--> backend_execution.rs
      +--> escape_hatch.rs
      +--> portability.rs
             ^        ^
             |        |
             +--------+
       raw markers   proof gate

portability.rs
      |
      +--> semantic_review.rs
      +--> passport.rs
      +--> export.rs
      +--> spec-cli/src/commands.rs

parallel precedent, unchanged in scope:
xtask/src/family/analysis_core/*
```

### Non-negotiable invariants

- `kind:data` and `kind:sum` still reject illegal top-level shared-surface authored shapes as validation errors.
- Allowed backend-specific authored detail remains valid input.
- Allowed backend-specific authored detail does **not** become portability-safe shared truth automatically.
- Helper-only lowering remains backend-only but non-contaminating.
- Domain lowering remains contaminating.
- Escape-hatch proof still requires both fresh `atom` and current `molecule`.
- CLI, passport, export, and semantic review must agree on the same underlying portability truth.

## File-By-File Implementation Contract

| File | Change | Why |
|---|---|---|
| `spec-core/src/portability_contract.rs` | create new typed policy owner | centralize seam portability rules |
| `spec-core/src/lib.rs` | export the new module | make contract available to all consumers |
| `spec-core/src/validator.rs` | replace inline seam restriction ownership with contract calls | remove duplicated policy text and drift risk |
| `spec-core/src/backend_execution.rs` | keep raw marker collection, align any policy naming with contract helpers | preserve raw signal ownership while separating policy |
| `spec-core/src/escape_hatch.rs` | keep gate ownership, optionally reuse stable helper wording | avoid gate-policy duplication |
| `spec-core/src/portability.rs` | replace local seam checks and local classification assumptions with contract calls | keep projection layer as projection only |
| `spec-core/src/semantic_review.rs` | consume centralized marker / contamination meaning | prevent semantic-review-only portability drift |
| `spec-core/src/passport.rs` | ensure projected proof coverage / markers / gate all flow from centralized contract | keep passport truth aligned |
| `spec-core/src/export.rs` | same as passport, for export bundles | keep export truth aligned |
| `spec-cli/src/commands.rs` | same as passport/export, for status and CLI rendering | keep status/export/reporting aligned |
| `spec-cli/tests/cli.rs` | add command-path regressions | catch live read-side drift |
| `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` | update milestone wording to the landed boundary | keep roadmap honest |
| `semantic-families/README.md` | keep packet-facing boundary wording honest | prevent packet docs from implying wider portability |
| `README.md` | update only if user-facing wording changed materially | keep public surface aligned |
| `CHANGELOG.md` | update only if user-visible truth changed | avoid fake churn |

## Implementation Sequence

### Step 1. Freeze the portability contract API

Create `spec-core/src/portability_contract.rs` with:

- seam-kind helper(s)
- shared authored-surface rule helpers
- backend-marker classification helpers
- stable string helpers only where multiple consumers would otherwise duplicate the same rule explanation

Do **not** add:

- artifact loading
- cargo execution
- CLI formatting
- read-side rendering
- dynamic config

Step 1 is done when the call surface is stable enough that downstream work can proceed in parallel.

### Step 2. Move validator-owned seam policy behind the contract

Replace inline `kind:data` and `kind:sum` rule ownership in `spec-core/src/validator.rs` so the validator becomes a caller, not a policy author.

Behavior must stay the same:

- illegal top-level seam shape still fails validation
- top-level `body.rust` on seam kinds still fails validation
- backend-specific lowering stays allowed only in the existing backend-specific surfaces

### Step 3. Rewire raw marker and projection layers

Update:

- `spec-core/src/backend_execution.rs`
- `spec-core/src/portability.rs`
- `spec-core/src/escape_hatch.rs`

Rules for this step:

- `backend_execution.rs` keeps raw marker collection ownership
- `portability.rs` stops deriving its own seam policy
- `escape_hatch.rs` does not become a portability classifier

This step is a refactor of ownership, not a behavior expansion.

### Step 4. Rewire semantic and read-side consumers

Update:

- `spec-core/src/semantic_review.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`

Goal:

- semantic review, passport, export, and status all consume the same centralized meaning of:
  - marker classification
  - contamination
  - gate interpretation

No consumer is allowed to invent a local version of "shared vs backend-only vs contaminating."

### Step 5. Lock regressions before docs

Add or refresh tests first, before widening doc claims.

The milestone is not complete if docs say the boundary is explicit but the test suite does not prove it.

### Step 6. Refresh roadmap and maintainer-facing docs

Required documentation updates:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `semantic-families/README.md`

Conditional updates:

- `README.md` if the user-facing wording on portability truth became more explicit
- `CHANGELOG.md` if command-visible truth changed in a user-noticeable way

### Step 7. Run the proof wall and fix parity drift

Do not stop at compile-green. M44 is complete only when the full proof wall is green and all read-side surfaces agree.

## Test Review

### Required proof wall

```bash
cargo test
cargo run -p spec-cli -- validate examples/ecommerce/units --format json
cargo run -p spec-cli -- test examples/ecommerce/units
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

### Expected preserved truth

- seam validator still rejects illegal top-level authored seam shapes
- helper-only lowering still projects as backend-only but non-contaminating
- domain lowering still contaminates portability claims
- escape-hatch gate still opens when atom proof is stale or molecule proof is missing
- family-analysis stop-state still points away from inventing a new family

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/portability_contract.rs
    |
    ├── seam-kind helpers
    │   └── direct unit tests required
    │
    ├── shared authored-surface rules
    │   └── validator-backed regressions required
    │
    └── contamination policy helpers
        └── projection / semantic-review regressions required

[~] spec-core/src/validator.rs
    |
    ├── kind:data shared-surface rejection
    │   └── must still hard-fail through contract calls
    │
    └── kind:sum shared-surface rejection
        └── must still hard-fail through contract calls

[~] spec-core/src/backend_execution.rs
    |
    ├── collect_backend_execution_markers()
    │   ├── proof-helper marker preserved
    │   ├── domain-lowering marker preserved
    │   └── rust-derives marker preserved
    │
    └── compute_backend_execution_digest()
        └── authored-only seam edits must not change digest

[~] spec-core/src/escape_hatch.rs
    |
    ├── current_proof_surfaces()
    │   ├── fresh atom path preserved
    │   └── current molecule path preserved
    │
    └── evaluate_escape_hatch_gate()
        └── stale atom / missing molecule regression required

[!] spec-core/src/portability.rs
    |
    ├── marker projection
    │   └── contract-owner parity required
    │
    ├── contamination summary
    │   └── helper-only vs domain-lowering split must be locked
    │
    └── full portability projection
        └── passport / export / status parity required

[!] spec-core/src/semantic_review.rs
    |
    ├── supported seam portability summary
    │   └── must consume contract-owned meaning
    │
    ├── backend-only meaning preserved
    │   └── regression required
    │
    └── backend-only semantics leaked
        └── regression required

[!] spec-core/src/passport.rs + export.rs + spec-cli/src/commands.rs
    |
    ├── projected markers
    │   └── parity required
    │
    ├── proof coverage / gate projection
    │   └── parity required
    │
    └── status / export / text rendering
        └── command-path regressions required
```

### Required new or refreshed regressions

1. Direct unit tests for `portability_contract.rs`.
2. Validator regressions proving `kind:data` and `kind:sum` restrictions still hard-fail through the new contract owner.
3. Projection regressions proving:
   - helper-only lowering is backend-only but not contaminating
   - domain lowering contaminates portability claims
4. Escape-hatch regressions proving stale atom or missing molecule proof opens the gate.
5. Passport / export / status parity tests proving all read-side surfaces agree on markers, proof coverage, and gate state.
6. CLI command-path regressions in `spec-cli/tests/cli.rs`.
7. `xtask` parity checks proving M44 did not reopen family-analysis semantics.

### Regression rule

Any behavior that was already true before M44 and could silently become looser or inconsistent during this extraction gets a regression test. No exceptions.

That includes:

- seam validation strictness
- contamination classification
- escape-hatch gate projection
- read-side parity across passport / export / status
- frozen family-analysis stop-state

## Code Quality Constraints

This plan is intentionally biased toward explicit over clever.

Rules:

- one new module, not a general-purpose abstraction tower
- no second new policy layer
- no new crate
- no broad renaming sweep unless compile-forced
- no consumer-local copies of seam policy after M44
- keep the diff focused on policy extraction plus consumer rewiring

Specific code-quality targets:

- remove duplicated seam-policy assumptions
- keep raw marker collection and policy classification separate
- keep validation, projection, gate evaluation, and semantic review responsibilities distinct
- do not mix structural extraction with second-language experimentation

## Performance Constraints

M44 is not a performance milestone, but it can accidentally add churn if done badly.

Do not add:

- filesystem work to read-side projection paths
- cargo process work to portability projection paths
- caching or memoization layers
- new persisted artifacts just to hold portability summaries

Expected cost profile:

- marker collection remains linear in seam method count
- projection remains cheap and in-process
- existing `xtask` artifact reuse remains unchanged

## Failure Modes Registry

| Codepath | Failure mode | Test required | Error handling required | User-visible effect if broken |
|---|---|---:|---:|---|
| seam validator contract | illegal top-level seam shape silently passes | Y | Y | false green validation |
| backend marker classification | helper-only and domain-lowering collapse into the same meaning | Y | Y | wrong portability verdict |
| backend digest | authored-only edits change backend digest | Y | Y | false stale backend-execution signal |
| escape-hatch gate | stale atom or missing molecule still projects as closed | Y | Y | false green seam portability truth |
| semantic review | backend-only detail gets treated as portability-safe shared meaning | Y | Y | wrong semantic verdict |
| passport / export / status | read-side surfaces disagree on markers or gate state | Y | Y | conflicting repo truth |
| docs and roadmap | milestone language claims broader portability than code proves | Y | N | maintainers steer the roadmap wrong |
| family-analysis precedent | M44 mutates stop-state semantics accidentally | Y | Y | repo invents a fake next-family move |

Critical gap rule:

- Any row above without a regression test is a release blocker for M44.
- Any bug that can produce a silent green portability claim is a release blocker for M44.

## Worktree Parallelization Strategy

This plan has one required serial gate, then a real parallel window, then one final integration lane.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Step 1. Freeze portability contract API | `spec-core/src/` | — |
| Step 2. Move validator and raw marker consumers onto the contract | `spec-core/src/` | Step 1 |
| Step 3. Rewire semantic and read-side projection consumers | `spec-core/src/`, `spec-cli/src/` | Step 1 |
| Step 4. Refresh docs and compatibility surfaces | `docs/`, `semantic-families/`, `README.md`, `CHANGELOG.md`, `examples/` | Step 1 |
| Step 5. Merge, rerun proof wall, and fix parity drift | `spec-core/src/`, `spec-cli/src/`, `xtask/src/family/`, `docs/` | Steps 2, 3, 4 |

### Parallel lanes

- Lane 0: Step 1
  - Owns: `spec-core/src/portability_contract.rs`, `spec-core/src/lib.rs`
  - Output: stable contract API that downstream lanes must obey

- Lane A: Step 2
  - Owns: `spec-core/src/validator.rs`, `spec-core/src/backend_execution.rs`
  - Goal: remove validator-owned policy duplication and align raw marker classification calls

- Lane B: Step 3
  - Owns: `spec-core/src/escape_hatch.rs`, `spec-core/src/portability.rs`, `spec-core/src/semantic_review.rs`, `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs`, `spec-cli/tests/cli.rs`
  - Goal: make all read-side and semantic consumers use the centralized contract

- Lane C: Step 4
  - Owns: `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`, `semantic-families/README.md`, optionally `README.md`, optionally `CHANGELOG.md`, any refreshed example truth surfaces
  - Goal: update written truth after the contract API is frozen

- Lane D: Step 5
  - Owns: integration, proof wall, parity cleanup
  - Goal: merge the prior lanes and make the whole milestone green together

### Execution order

1. Launch Lane 0 first. Do not parallelize before the contract API is frozen.
2. Once Step 1 is stable, launch Lanes A, B, and C in parallel worktrees.
3. Merge Lane A before finalizing Lane B if B needs any last call-shape or helper adjustments.
4. Merge Lane C once the wording matches the landed boundary. It does not need to wait for proof-wall completion unless the code truth changed again during integration.
5. Run Lane D last on the merged branch and rerun the entire proof wall.

### Conflict flags

- Lanes A and B both touch `spec-core/src/`, so Lane 0 must freeze names and signatures first.
- Lane B is the highest merge-risk lane because it touches both `spec-core` and `spec-cli`.
- Lane D must own any parity cleanup after the parallel merges. Do not let A or B keep rebasing after D starts.
- Lane C is low-conflict, but it must not over-claim. Docs must match the landed code, not the hoped-for next milestone.

## Documentation Contract

The documentation work is part of the milestone, not garnish.

Required updates:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
  - keep the shared-core / escape-hatch ordering honest
  - reflect that M44 centralized the portability contract in `spec-core`
- `semantic-families/README.md`
  - preserve the M31 / M32 style boundary language without implying broader portability than the code proves

Conditional updates:

- `README.md`
  - only if command-visible portability wording changed for users
- `CHANGELOG.md`
  - only if the milestone changes user-visible truth surfaces or milestone labeling in a way that should be recorded publicly

## Acceptance Criteria

M44 is complete only if all of the following are true:

1. `spec-core/src/portability_contract.rs` exists and is the sole policy owner for seam portability rules.
2. `spec-core/src/validator.rs` no longer owns duplicated seam-policy rules inline.
3. `spec-core/src/backend_execution.rs` still owns raw marker detection and backend digest material.
4. `spec-core/src/escape_hatch.rs` still owns proof-surface gate computation.
5. `spec-core/src/portability.rs` composes truth from centralized contract helpers instead of local seam assumptions.
6. `spec-core/src/semantic_review.rs`, `spec-core/src/passport.rs`, `spec-core/src/export.rs`, and `spec-cli/src/commands.rs` all project the same portability truth.
7. Helper-only lowering still remains backend-only but non-contaminating.
8. Domain lowering still contaminates portability claims.
9. The escape-hatch gate still requires fresh atom proof and current molecule proof.
10. The proof wall passes.
11. `xtask` family-analysis stop-state truth remains unchanged.
12. The updated docs match the landed code boundary exactly.
13. The implementation did not widen into new crate extraction, second-language execution, or fresh family-choice work.

## Definition Of Done

M44 is done when a maintainer can answer all of these questions by pointing at one place in the code, not by narrating cross-file tribal knowledge:

- What is legal shared seam authored shape?
- What is backend-only but still honest?
- What contaminates portability claims?
- Which modules own validation, gate evaluation, projection, and semantic verdicts?
- Why does status/export/passport say what they say?

If those answers still require "well, validator does one part, portability does another part, and semantic review kind of knows the rest," then M44 did not finish the job.

## Completion Summary

This plan is the smallest complete version of the lake.

It adds one explicit policy owner. It keeps every other module in its current role. It locks the regressions that matter. It updates the roadmap to match the code. It gives the repo one honest portability contract instead of several half-implicit ones.

That is the whole milestone.
