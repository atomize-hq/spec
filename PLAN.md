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

M44 is not another family-promotion milestone.

The branch already proved the Rust family lane honestly enough to stop. The next honest move is to freeze one explicit portability contract for seam units so the repo can say, in code and in read-side truth, which semantics are shared, which details are Rust-only backend execution, and which escape hatches contaminate portability claims.

This milestone is **not** docs-only. It includes one small structural extraction inside `spec-core`: add one explicit portability-contract owner, route every seam-portability consumer through it, and keep `xtask` family-analysis logic as precedent rather than reopening it.

## Problem Statement

M43 finished the last obvious Rust-family lake on this branch.

Current repo truth now says:

- promoted Rust families are no longer the bottleneck
- family-analysis can stop honestly instead of inventing a next family
- the remaining architectural ambiguity is the seam between shared semantic truth and Rust-only backend execution detail

That ambiguity is visible in code today:

- `xtask/src/family/analysis_core/` already freezes the family-analysis decision seam for helper-surface durable hold, decision derivation, and proof-fingerprint normalization
- `spec-core/src/backend_execution.rs` already identifies backend-only seam markers
- `spec-core/src/escape_hatch.rs` already computes the atom/molecule gate for seam units
- `spec-core/src/portability.rs` already projects markers, contamination, digests, and gate state
- `spec-core/src/semantic_review.rs`, `spec-core/src/passport.rs`, and CLI read-side surfaces already consume that truth
- `spec-core/src/validator.rs` still hardcodes part of the shared-surface contract inline as literal seam restrictions

So the seam exists. The problem is that the contract is still spread across multiple files and partly implied by maintainers knowing where to look.

If M44 does nothing, the next backend or read-side consumer will either duplicate these rules or quietly smuggle Rust-specific assumptions into a fake shared core.

## Repo Truth Basis

### Live evidence

- `xtask/src/family/analysis_core/helper_surface.rs` already freezes the durable helper-surface hold and follow-on decision tuple.
- `xtask/src/family/analysis_core/decision_contract.rs` already derives corpus-program decisions from validated analysis truth without path or IO concerns.
- `spec-core/src/backend_execution.rs` already distinguishes `DomainLowering`, `ProofHelperLowering`, and `BackendRustDerives`.
- `spec-core/src/escape_hatch.rs` already makes seam proof surfaces explicit: `atom` and `molecule`.
- `spec-core/src/portability.rs` already exposes marker summaries, contamination summaries, backend digests, and escape-hatch gate projection.
- `spec-core/src/validator.rs` already enforces that `kind:data` and `kind:sum` must keep top-level `contract`, `deps`, `imports`, and `body.rust` out of the shared authored seam.
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` still says shared-core extraction plus escape-hatch containment must come before broader second-language work.

### What already exists

| Sub-problem | Existing owner | M44 decision |
|---|---|---|
| Family-analysis decision seam | `xtask/src/family/analysis_core/*` | reuse as precedent, no new `xtask` seam work unless compile-forced |
| Raw backend-only seam markers | `spec-core/src/backend_execution.rs` | keep as the raw marker collector |
| Escape-hatch proof gate | `spec-core/src/escape_hatch.rs` | keep as the sole gate owner |
| Portability truth projection | `spec-core/src/portability.rs` | keep as the read-side projection owner |
| Seam semantic verdict composition | `spec-core/src/semantic_review.rs` | keep as consumer, remove inline policy drift |
| Passport/status/export truth surfaces | `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs` | keep as consumers, not policy authors |
| Shared authored-surface restrictions | `spec-core/src/validator.rs` | extract into one explicit contract owner |

## Scope Challenge

### 0A. Premise challenge

1. The right problem is not “do more TypeScript now.” The right problem is “freeze the truth boundary that TypeScript would have to obey.”
2. The right problem is not “extract a new crate.” The code already has one bounded `spec-core` seam. Make that seam explicit before spending packaging overhead.
3. The right problem is not “rewrite family-analysis again.” `xtask` already has the reusable decision seam. M44 should reuse that precedent, not reopen it.

### 0B. Minimum complete change set

M44 should do the complete version of this lake:

1. define one explicit portability-contract owner in `spec-core`
2. move seam authored-surface rules and portability-policy tables behind that owner
3. thread that owner through validator, portability projection, semantic review, passport, export, and status surfaces
4. lock the contract with regression tests and command-path proofs
5. rewrite roadmap and user-facing docs so the repo story matches the code

Anything smaller is fake completeness. It would save almost no CC time and leave the same ambiguity behind.

### 0C. Complexity check

This milestone will likely touch more than 8 files, but that is justified because it is one contract-change lake with one new module and many consumers.

The smell threshold still applies:

- **acceptable**: one new module inside `spec-core`, no new crate, no new artifact type
- **not acceptable**: adding a second new abstraction layer, a new workspace crate, or any second-language execution surface

### 0D. Distribution check

No new end-user distribution artifact is introduced here.

The distribution surface is repo truth:

- `spec-core` source boundaries
- CLI read-side output truth
- roadmap and README language

## Accepted Scope

M44 is complete only if all of this lands together:

1. Add one explicit contract owner at `spec-core/src/portability_contract.rs`.
2. Move seam shared-surface ownership rules out of `validator.rs` literals and behind that contract.
3. Move portability-policy classification behind that contract:
   - what is shared authored seam truth
   - what is backend-only but non-contaminating
   - what is contaminating domain lowering
4. Rewire `backend_execution.rs`, `escape_hatch.rs`, `portability.rs`, `semantic_review.rs`, `passport.rs`, `export.rs`, and `spec-cli/src/commands.rs` to consume the explicit contract instead of re-deriving local policy.
5. Preserve current family-analysis stop-state truth by keeping `xtask/src/family/analysis_core/*` unchanged except for import or wording changes forced by compile or docs alignment.
6. Lock the command-path proof wall for seam portability truth in validation, test, status, and export flows.
7. Rewrite the roadmap and active docs so the public story matches the landed boundary.

## Not In Scope

- new family promotion work
- recommendation-policy or corpus-expansion changes
- a new workspace crate or cross-crate shared portability library
- first-class TypeScript backend execution
- widening supported function portability claims
- new JSON schema versions unless a read-side contract change forces one explicit additive field update
- broad cleanup of unrelated `xtask/src/family/*` code
- any change to `spec generate/build/test` ownership beyond read-side truth projection

## Architecture Review

### Locked architectural decision

M44 includes **one small extraction**, not a docs-only freeze:

- add `spec-core/src/portability_contract.rs`
- make it the sole owner of seam portability policy
- keep `backend_execution.rs`, `escape_hatch.rs`, and `portability.rs` as the execution, proof-gate, and projection layers

This is the exact answer to the design doc’s open question. The milestone needs one code move to prove the seam is real. It does **not** need a new crate.

### Locked target boundary

```text
spec-core/src/
  portability_contract.rs   <-- NEW, sole policy owner
  backend_execution.rs      <-- raw marker extraction + digest
  escape_hatch.rs           <-- atom/molecule gate computation
  portability.rs            <-- projected portability truth
  semantic_review.rs        <-- verdict consumer
  validator.rs              <-- hard authored-shape enforcement consumer
  passport.rs               <-- passport/status projection consumer
  export.rs                 <-- export projection consumer
  lib.rs                    <-- module wiring

spec-cli/src/
  commands.rs               <-- status/export CLI consumer

xtask/src/family/
  analysis_core/*           <-- reuse as precedent, do not widen
```

### Exact ownership map

| Module | Owns after M44 | Must not own |
|---|---|---|
| `portability_contract.rs` | seam-kind helpers, allowed shared authored surfaces, allowed backend-only surfaces, contamination policy table, stable reason strings/helpers for shared vs Rust-only classification | file IO, cargo commands, artifact loading, latest-artifact reuse, read-side rendering |
| `backend_execution.rs` | detection of raw backend markers and backend digest material | verdict policy, gate policy, validator policy text |
| `escape_hatch.rs` | proof-surface requirements and open/closed gate evaluation | marker classification policy, semantic verdict policy |
| `portability.rs` | composition of markers, contamination summary, gate, and digest into projected portability truth | inline seam policy duplication |
| `semantic_review.rs` | semantic verdict generation using projected portability truth | ownership of seam policy tables |
| `validator.rs` | hard validation using contract-owned seam rules | hand-maintained duplicate seam policy strings |
| `passport.rs`, `export.rs`, `spec-cli/src/commands.rs` | projection of current truth only | new local policy branches about what is shared vs Rust-only |
| `xtask/src/family/analysis_core/*` | family-analysis decision semantics | seam portability policy for `spec-core` |

### Exact move and rewire plan

| Current owner | Surface | Destination or action | Why |
|---|---|---|---|
| `validator.rs` | `kind:data` and `kind:sum` shared-surface restrictions | move rule ownership into `portability_contract.rs`; keep validator as caller | one source of truth |
| `portability.rs` | local `is_seam` helper and seam ownership assumptions | move seam-kind helper to `portability_contract.rs` | no more local re-derivation |
| `semantic_review.rs` | inline portability policy branches that assume which markers contaminate | switch to contract-owned classification helpers | keep semantic review a consumer |
| `backend_execution.rs` | marker kind enums and digest inputs | keep local, but classify through contract-owned policy names where needed | raw signals stay separate from verdict policy |
| `escape_hatch.rs` | required surfaces and gate wording | keep local, but align wording and shared helpers with contract owner | gate remains explicit |
| `passport.rs` / `export.rs` / `commands.rs` | projection of markers, gate, proof coverage | rewire only as needed to consume explicit contract/projected truth | keep read-side behavior aligned |
| roadmap/docs | milestone wording and README seam language | update to reflect exact landed boundary | public story must match code |

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
                |
                +--> semantic_review.rs
                +--> passport.rs
                +--> export.rs
                +--> spec-cli/src/commands.rs

parallel precedent, unchanged in scope:
xtask/src/family/analysis_core/*
```

### Architecture constraints

- No `Path`, `fs`, `Write`, or repo-root knowledge inside `portability_contract.rs`.
- No new policy duplication inside `semantic_review.rs`, `validator.rs`, or CLI projection code.
- No new crate extraction.
- No change to family-analysis stop-state semantics.
- No widening of what counts as portability-safe.

### Architectural verdict

Recommendation: add one explicit contract module inside `spec-core` and stop there.

Why:

- it is the smallest complete move
- it aligns with the already-landed `xtask` `analysis_core` precedent
- it keeps the change reversible
- it avoids spending an innovation token on packaging before the seam settles

## Code Quality Review

### Problems this plan removes

1. **Scattered seam policy**
   - today the seam rules live partly in validator literals, partly in portability projections, and partly in semantic-review branches
   - after M44 the seam policy has one named owner

2. **Consumer drift risk**
   - today passport/export/status consumers can stay green while silently disagreeing with validator or semantic-review assumptions
   - after M44 all of them consume the same portability contract

3. **Fake shared-core language**
   - today the code has the pieces, but the contract is still partly a story
   - after M44 the contract is embodied in one module and one policy table

### Implementation rules

- move policy before tweaking behavior
- do not mix structural extraction with second-language logic
- keep existing public read-side behavior stable unless a deliberate portability-truth correction is named in this plan
- prefer small helper extraction over broad renames
- preserve explicit naming over clever abstractions

## Implementation Plan

### Step 1. Freeze the contract surface

- Add `spec-core/src/portability_contract.rs`.
- Define seam-kind helpers and portability policy helpers there.
- Add module wiring in `spec-core/src/lib.rs`.
- Keep the contract small and typed.

### Step 2. Move shared authored-surface rules behind the contract

- Replace inline `kind:data` and `kind:sum` portability rule ownership in `spec-core/src/validator.rs`.
- Keep hard validation behavior the same:
  - illegal shared-surface authored shapes remain validation errors
  - allowed backend-only details remain valid input, not automatic portability-safe truth

### Step 3. Rewire raw marker and gate layers to consume the contract

- Update `spec-core/src/backend_execution.rs` to align raw marker naming with the contract.
- Update `spec-core/src/escape_hatch.rs` only where shared helpers or wording are needed.
- Update `spec-core/src/portability.rs` to consume the contract instead of local seam assumptions.

### Step 4. Rewire read-side truth consumers

- Update `spec-core/src/semantic_review.rs`.
- Update `spec-core/src/passport.rs`.
- Update `spec-core/src/export.rs`.
- Update `spec-cli/src/commands.rs` if status/export JSON or text projection needs one centralized portability path.

### Step 5. Lock examples, fixtures, and docs

- Refresh any example artifacts whose projected portability truth changes.
- Update `README.md` seam wording if needed.
- Rewrite `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` so the shared-core/escape-hatch ladder matches the landed boundary.
- Update `CHANGELOG.md` only if user-facing read-side truth changes.

### Step 6. Run the full proof wall

- prove validator, semantic-review, passport, export, and status parity
- prove family-analysis stop-state remains unchanged
- prove no new portability claims were accidentally widened

## Test Review

### Required proof loop

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

Expected preserved truth:

- seam validator still rejects illegal top-level shared-surface authored shapes
- helper-only lowering remains backend-only but not contaminating
- domain lowering remains contaminating
- escape-hatch gate still requires fresh `atom` and `molecule` proof
- family-analysis stop-state still points to architecture follow-on, not another family

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/portability_contract.rs
    |
    |-- seam-kind helpers
    |   `-- [GAP] new direct unit tests required
    |
    |-- shared-authored-surface rules
    |   `-- [GAP] validator-backed regression tests required
    |
    `-- contamination policy table
        `-- [GAP] semantic-review and portability projection tests required

[~] spec-core/src/validator.rs
    |
    |-- kind:data shared-surface rejection
    |   `-- [PASS TESTED] existing seam restriction coverage, refresh through contract owner
    |
    `-- kind:sum shared-surface rejection
        `-- [PASS TESTED] existing seam restriction coverage, refresh through contract owner

[~] spec-core/src/backend_execution.rs
    |
    |-- collect_backend_execution_markers()
    |   |-- [PASS TESTED] proof-helper marker
    |   |-- [PASS TESTED] domain-lowering marker
    |   `-- [PASS TESTED] backend-rust-derives marker
    |
    `-- compute_backend_execution_digest()
        `-- [PASS TESTED] authored-only edits do not change digest

[~] spec-core/src/escape_hatch.rs
    |
    |-- current_proof_surfaces()
    |   |-- [PASS TESTED] fresh atom path
    |   `-- [PASS TESTED] current molecule path
    |
    `-- evaluate_escape_hatch_gate()
        |-- [PASS TESTED] closed gate
        `-- [GAP] stale atom + missing molecule parity regression after contract extraction

[!] spec-core/src/portability.rs
    |
    |-- collect/summarize markers
    |   `-- [GAP] direct contract-owner parity tests required
    |
    |-- summarize_portability_contamination()
    |   `-- [GAP] helper-only vs domain-lowering split must be locked
    |
    `-- project_portability_truth()
        `-- [GAP] status/export/passport shared projection parity required

[!] spec-core/src/semantic_review.rs
    |
    |-- supported seam portability summary
    |   `-- [GAP] direct consumer parity with contract owner required
    |
    |-- backend-only meaning preserved
    |   `-- [PASS TESTED] existing helper-marker verdict coverage
    |
    `-- backend-only semantics leaked
        `-- [PASS TESTED] existing domain-lowering contamination coverage

[!] spec-core/src/passport.rs + export.rs + spec-cli/src/commands.rs
    |
    |-- projected markers
    |   `-- [GAP] read-side parity tests required
    |
    |-- proof coverage / gate projection
    |   `-- [GAP] status/export current-truth parity required
    |
    `-- CLI JSON/text rendering
        `-- [GAP] command-path regression tests required

---------------------------------
REQUIRED NEW TESTS:
1. direct unit tests for portability_contract helpers
2. validator regressions proving rules now route through the contract owner
3. portability projection regressions for helper-only vs contaminating lowering
4. passport/export/status parity tests
5. stale-proof gate regressions
6. xtask stop-state parity regression proving M44 did not reopen family choice
---------------------------------
```

### Required new tests

1. `spec-core` unit tests for `portability_contract.rs`.
2. Validator regressions proving `kind:data` and `kind:sum` restrictions still hard-fail through the new contract owner.
3. Portability projection regressions proving:
   - helper-only lowering is backend-only but non-contaminating
   - domain lowering contaminates portability claims
4. Escape-hatch regressions proving stale atom or missing molecule proof opens the gate.
5. Passport/export/status parity tests proving all read-side surfaces agree on markers, proof coverage, and gate state.
6. CLI command-path tests in `spec-cli/tests/cli.rs` proving `validate`, `status`, and `export` remain truthful.
7. `xtask` parity proof proving `family recommend`, `family corpus-decision`, and `verify-decision-contract` remain unchanged.

## Performance Review

This milestone is not performance-driven, but it can create accidental churn if done sloppily.

Performance constraints:

- do not add filesystem or cargo-process work to the read-side path
- keep marker collection and contamination summarization linear in seam method count
- avoid recomputing the same portability summary multiple times inside one projection path when a shared local result will do
- keep `xtask` latest-artifact fingerprint reuse unchanged

Performance anti-goals:

- no caching layer
- no global memoization
- no new persisted artifact just for portability summaries

## Failure Modes Registry

| Codepath | Failure mode | Rescued? | Test? | User sees? | Logged? |
|---|---|---:|---:|---|---:|
| seam validator contract | illegal top-level seam shape becomes a warning or silently passes | N | Y | explicit validate failure required | Y |
| backend marker classification | proof-helper and domain-lowering markers collapse into one meaning | N | Y | wrong portability verdict unless caught | Y |
| escape-hatch gate | stale atom or missing molecule proof still projects as closed | N | Y | false green portability truth | Y |
| semantic-review projection | backend-only detail gets treated as portability-safe shared semantics | N | Y | wrong semantic verdict | Y |
| passport/export/status parity | different read-side surfaces disagree on markers or gate state | N | Y | conflicting repo truth | Y |
| roadmap/docs | docs claim portability is solved more broadly than code proves | N | Y | maintainers plan the wrong next milestone | Y |
| family-analysis precedent | M44 accidentally mutates `xtask` stop-state semantics | N | Y | next-family truth reopens incorrectly | Y |

Critical-gap rule:

- any row with `Test = N` is unacceptable for M44
- any silent green portability claim is unacceptable for M44

## Worktree Parallelization Strategy

This milestone has one real contract-freeze gate, one safe code-parallel window, and one serialized integration finish.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Step 1. Freeze portability contract API | `spec-core/src/` | — |
| Step 2. Enforce contract in validation and raw marker layers | `spec-core/src/` | Step 1 |
| Step 3. Rewire read-side projection consumers | `spec-core/src/`, `spec-cli/src/` | Step 1 |
| Step 4. Refresh docs and example truth surfaces | `docs/`, `README.md`, `CHANGELOG.md`, `examples/` | Step 1 |
| Step 5. Integrate, rerun proof wall, and fix parity drift | `spec-core/src/`, `spec-cli/src/`, `xtask/src/family/`, `docs/` | Steps 2, 3, 4 |

### Parallel lanes

- Lane 0: Step 1
  Freeze the API first. This is the contract that every later lane must obey.
- Lane A: Step 2
  Validation plus raw backend-marker alignment. This lane owns `portability_contract.rs`, `validator.rs`, `backend_execution.rs`, and `lib.rs`.
- Lane B: Step 3
  Read-side consumer rewiring. This lane owns `escape_hatch.rs`, `portability.rs`, `semantic_review.rs`, `passport.rs`, `export.rs`, `spec-cli/src/commands.rs`, and CLI tests.
- Lane C: Step 4
  Docs, roadmap, and example truth surfaces. This lane can run independently once the contract is frozen.
- Lane D: Step 5
  Final integration and proof wall.

### Execution order

- Launch Lane 0 first and freeze the portability-contract API.
- After Step 1 lands, launch Lanes A, B, and C in parallel worktrees.
- Merge Lane A first if Lane B needs any final contract call-shape adjustments.
- Merge Lane C whenever docs and example updates are green.
- Run Lane D last on the merged result to rerun the proof wall and fix any parity drift.

### Conflict flags

- Lanes A and B both touch `spec-core/src/`, so Step 1 must freeze names and function signatures before launch.
- Lanes B and D both touch `spec-cli/src/commands.rs` and read-side parity tests, so D must wait.
- Lane C is low-conflict, but it must not invent milestone language that outruns the landed code.

## Docs And Roadmap Updates

Required documentation updates:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
  - rewrite the shared-core and escape-hatch milestone text so it matches the landed M44 boundary
- `README.md`
  - keep seam wording honest if any read-side truth surfaces become more explicit
- `CHANGELOG.md`
  - only if user-facing command truth or milestone status language changes

The docs update is part of the milestone, not cleanup garnish. If the code lands and the roadmap still tells the older story, M44 is incomplete.

## Completion Criteria

M44 is complete only if all of the following are true:

1. `spec-core/src/portability_contract.rs` exists and is the sole seam portability policy owner.
2. `validator.rs` no longer owns duplicated seam portability policy inline.
3. `backend_execution.rs`, `escape_hatch.rs`, and `portability.rs` consume the explicit contract without changing their basic responsibilities.
4. `semantic_review.rs`, `passport.rs`, `export.rs`, and CLI read-side surfaces project the same portability truth.
5. Existing helper-only and contaminating-domain-lowering verdict behavior remains correct.
6. The escape-hatch gate still requires fresh atom and molecule proof.
7. `xtask` family-analysis stop-state truth remains unchanged.
8. The roadmap and active docs match the landed code boundary.
9. The implementation did not widen into second-language execution, new crate extraction, or fresh family-choice work.

## Completion Summary

If M44 lands cleanly, the repo stops hand-waving about portability.

Maintainers get one explicit place in `spec-core` that defines the seam contract. Validator, semantic review, passport, export, and status all read from the same policy instead of re-deriving it. `xtask` keeps its already-explicit family-analysis seam untouched, and the roadmap stops implying that broader portability or second-language execution has already been earned.
