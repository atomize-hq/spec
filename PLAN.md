# M39 - Verification Consumer Probe After M38

Status: **authoritative implementation plan**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-06**  
Supersedes: **M38 - Architecture Follow-On Trigger Gating After M37**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260505-231151.md`**  
Frozen baseline commit: **`a9e6d7252a2e7dc1d9c53f14ad65b1b8d685c5dc`**  
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**  
Program tracker: **`docs/recommendation_corpus_expansion_program_v0.1.md`**  
Capability guide: **`docs/semantic_family_capability_corpus_guide_v0.1.md`**  
M38 closeout: **`.runs/m38_trigger_gating/closeout.md`**  
Execution note: **M39 is a bounded verification-consumer probe. It does not move `decision_kernel.rs`, does not widen public artifact schemas, does not rescan raw corpus inputs, and does not claim shared-core pressure unless repo-root orchestration actually adopts the new verifier.**

## Objective

Turn the repeated shell-level verification path around the frozen helper-surface
decision tuple into one truthful in-tree maintainer surface:

```text
cargo xtask family verify-decision-contract --format json
```

M39 is not an extraction milestone. It is a bounded probe with one question:

> does the repo have a real third consumer of the existing family decision
> semantics, or only repeated shell glue around the same proof floor?

The milestone succeeds only if the answer is backed by implementation,
orchestration adoption, parity proof, and an explicit closeout verdict.

## Executive Verdict

The honest M39 implementation is a thin read-side verifier plus standing
adoption, not a new kernel and not a generalized contract framework.

The repo already has almost all of the semantics M39 needs:

- `xtask/src/family/promotion_artifacts.rs` already validates both artifacts
- `CorpusProgramDecisionArtifact::validate(...)` already enforces:
  - basis snapshot parity against the validated analysis basis
  - derived decision tuple parity against
    `derive_corpus_program_decision_contract(...)`
- `xtask/src/family/decision_kernel.rs` already owns the kernel truth

That means M39 should add one new command surface, expose its checks as
structured JSON, and replace the standing shell ladder in `ORCH_PLAN.md` if,
and only if, the command earns that adoption.

If adoption lands and sticks, the repo has a credible third-consumer signal. If
it does not, the kernel stays local and M39 says so plainly.

## Live Baseline

Live HEAD on `feat/corpus-expansion` is
`a9e6d7252a2e7dc1d9c53f14ad65b1b8d685c5dc`.

The current standing verification path is still shell-first:

- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
- `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- repeated `jq` assertions in `ORCH_PLAN.md` around:
  - lines `340-350`
  - lines `582-592`
  - lines `938-947`

The frozen M38 helper-surface floor remains:

- `recommendation_status = "no_strong_candidate"`
- `decision_summary.decision_status = "not_recommended"`
- `decision_summary.open_blockers = ["helper_surface_not_promotable"]`
- `decision_action = "pivot_to_architecture_shared_core_follow_on"`
- `decision_basis_code = "durable_non_promotable_helper_surface"`
- `required_next_action = "author_architecture_follow_on_plan"`

## Problem Statement

Today the repo has kernel semantics that are reused in practice, but the
maintainer-facing verification story is still split across:

- two `validate-artifact` calls
- repeated semantic `jq` assertions
- copy-paste proof blocks in `ORCH_PLAN.md`

That is close to a third consumer, but not enough to claim one yet.

The missing proof is standing adoption. M39 needs to answer one narrow
question:

> is there a real in-tree verification consumer that maintainers will actually
> use for the standing helper-surface proof floor?

If yes, the repo has real pressure. If no, the kernel remains local and the
repo avoids another fake abstraction detour.

## Frozen Premises

1. M38 already decided that deeper extraction is not justified on current repo
   truth.
2. The only honest reason to revisit extraction is new consumer pressure, not
   momentum.
3. The verification path is the strongest nearby candidate for a third
   consumer because it repeatedly reads the same emitted analysis and decision
   semantics.
4. That candidate counts only if the repo adopts it in standing orchestration
   surfaces, not if it appears once in a scratch experiment.
5. M39 must stay read-side only. Recomputing coverage or recommendation from
   raw corpus inputs would be fake scope.
6. The command output can be structured JSON, but M39 must not introduce a new
   persisted artifact class or widen any public artifact schema.

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| CLI family subcommand dispatch | `xtask/src/lib.rs` | Reuse. Add exactly one new subcommand branch. |
| Family module registration | `xtask/src/family/mod.rs` | Reuse. Add one new module export only. |
| Kernel truth for basis snapshot and derived decision tuple | `xtask/src/family/decision_kernel.rs` | Reuse unchanged. M39 reads from it, not around it. |
| Recommendation and decision latest paths | `xtask/src/family/paths.rs`, `xtask/src/family/recommend.rs` | Reuse unchanged. M39 reads the same canonical latest paths. |
| Artifact structs and validators | `xtask/src/family/promotion_artifacts.rs` | Reuse directly. This is the main leverage point. |
| Existing operator proof floor | `ORCH_PLAN.md` repeated validation + `jq` blocks | Replace with the new command once parity is proven. |
| Existing trigger context | `TODOS.md` post-M37 follow-ups | Reuse unchanged. M39 is evidence gathering, not trigger expansion. |
| Existing milestone record | `.runs/m38_trigger_gating/` | Reuse as the frozen semantic floor and truth source. |

### Minimum complete change set

The minimum complete M39 is:

1. add `cargo xtask family verify-decision-contract --format json`
2. implement the verifier as a read-side command over canonical latest
   artifacts
3. expose machine-readable pass/fail details for:
   - recommendation-analysis artifact validity
   - corpus-program-decision artifact validity
   - basis snapshot parity
   - derived decision tuple parity
   - frozen helper-surface floor parity
4. add targeted tests plus one end-to-end parity proof
5. replace the named shell `jq` ladders in `ORCH_PLAN.md`
6. record one explicit closeout verdict for whether the third-consumer claim
   was proven

Anything beyond that is scope leak.

### Complexity check

This plan stays below the smell line on purpose:

- one new module under `xtask/src/family/`
- one CLI wiring update
- one orchestration-doc adoption update
- no new crate
- no new public schema
- no new decision policy branch
- no movement of `decision_kernel.rs`

If M39 grows into a framework for arbitrary verification contracts, stop. That
is the exact overbuild this milestone exists to reject.

### Search check

**[Layer 1]** Reuse `CorpusProgramDecisionArtifact::validate(...)` as the parity
truth source instead of recreating that logic in shell or in a second decision
path.

**[Layer 1]** Reuse the canonical latest paths already wired through
`recommend.rs` and `paths.rs`. Do not add path override flags in M39.

**[Layer 3]** The command exists to prove adoption pressure, not to become a
mini framework. Keep the surface narrow and frozen to the current helper-surface
floor.

**[EUREKA]** The existing validator already proves basis and derived-tuple
parity. The new command is valuable because it consolidates operator behavior
and makes that proof legible, not because it discovers new semantics.

### TODOS cross-reference

Relevant standing TODOs already exist:

1. `Cross-crate family-analysis shared core`
2. `Generalized multi-wedge decision layer`
3. `Public semantic fingerprint fields`

M39 should not add new architecture TODOs. It should either strengthen or
weaken the existing shared-core trigger through evidence.

### Completeness check

A half-plan would stop at "add a command."

The complete version includes:

- exact CLI contract
- exact JSON output contract
- exact failure reason vocabulary
- exact codepath and maintainer-flow coverage
- exact `ORCH_PLAN.md` adoption points
- exact parity proof
- exact closeout verdict rules
- explicit worktree parallelization boundaries

That is the lake. Boil it.

### Distribution check

No new package, binary, image, or release channel is introduced.

Distribution for M39 means maintainer adoption inside existing repo workflows:

- `cargo xtask ...` command surface
- `ORCH_PLAN.md` standing verification blocks
- accepted run proof log and closeout record

If those surfaces do not adopt the new command, the consumer claim is not
proven.

## Approved Scope

M39 includes exactly six deliverables:

1. this authoritative `PLAN.md`
2. a new read-side verifier command:
   `cargo xtask family verify-decision-contract --format json`
3. tests for pass and failure cases on the frozen helper-surface floor
4. adoption of the command in the named `ORCH_PLAN.md` proof blocks
5. one accepted-run parity proof against the old shell path
6. one explicit closeout verdict on whether the third-consumer claim is proven

## NOT in scope

The following work is explicitly deferred:

- moving `xtask/src/family/decision_kernel.rs`
  Reason: M39 is testing consumer pressure, not responding to it yet.
- new shared-core extraction
  Reason: the milestone exists to decide whether that pressure is real.
- generalized verification of arbitrary decision floors
  Reason: that would spend complexity before the consumer is proven.
- path override flags for ad hoc artifact locations
  Reason: M39 is about the standing repo surface, not arbitrary files.
- rescanning coverage or recomputing recommendation from raw inputs
  Reason: that would create a second semantics path and fake the probe.
- public schema or artifact-kind expansion
  Reason: the new JSON is command output, not a new persisted artifact contract.
- corpus run `1` spending or family recommendation policy changes
  Reason: M39 is read-side only.

## Architecture Review

### System boundary

M39 keeps the existing write-side architecture intact and adds one read-side
verification consumer:

```text
CURRENT WRITE-SIDE + NEW READ-SIDE SHAPE
========================================

family coverage
    │
    ▼
family recommend --format json
    │ writes recommendation.latest.json
    ▼
family corpus-decision --format json
    │ writes corpus-program-decision.latest.json
    ▼
existing artifact validators + decision kernel
    │
    ├── existing shell proof path
    │     ├── validate-artifact recommendation.latest.json
    │     ├── validate-artifact corpus-program-decision.latest.json
    │     └── repeated jq assertions
    │
    └── NEW: family verify-decision-contract --format json
          ├── reads recommendation.latest.json
          ├── reads corpus-program-decision.latest.json
          ├── runs existing validators
          ├── recomputes expected basis snapshot from validated analysis
          ├── recomputes expected decision tuple from validated analysis
          ├── checks frozen helper-surface floor
          └── emits one structured pass/fail report
```

### Module boundaries

The implementation stays intentionally boring:

| Module | Responsibility |
|---|---|
| `xtask/src/lib.rs` | Add `VerifyDecisionContract` subcommand and dispatch only. |
| `xtask/src/family/mod.rs` | Register the new verifier module only. |
| `xtask/src/family/verify.rs` | Own all command-specific read-side logic and JSON rendering. |
| `xtask/src/family/decision_kernel.rs` | Remains the semantic source of truth, unchanged. |
| `xtask/src/family/promotion_artifacts.rs` | Remains the artifact contract and validator source of truth, reused directly. |
| `ORCH_PLAN.md` | Replace repeated shell assertions with the consolidated command once parity is proven. |

### Command contract

The command surface is frozen for M39:

```text
cargo xtask family verify-decision-contract --format json
```

Rules:

- only `--format json` is supported
- no path override flags
- reads only:
  - `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`
  - `.semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json`
- exits `0` only when every required check passes
- exits non-zero on any validation failure, drift, or floor mismatch

### JSON output contract

The command output is structured JSON to stdout. It is not a new persisted
artifact.

Required top-level fields:

| Field | Meaning |
|---|---|
| `command` | exact command name, `family verify-decision-contract` |
| `format` | `json` |
| `recommendation_analysis_path` | canonical latest analysis path |
| `corpus_program_decision_path` | canonical latest decision path |
| `checks` | object containing one result object per required check |
| `overall_verdict` | `pass` or `fail` |
| `failure_reasons` | stable machine-readable reason list, empty on pass |

Required `checks.*.status` values:

- `pass`
- `fail`

Required `checks` entries:

1. `recommendation_analysis_validation`
2. `corpus_program_decision_validation`
3. `basis_snapshot_parity`
4. `derived_decision_parity`
5. `frozen_helper_surface_floor`

Required machine failure reasons:

- `missing_recommendation_analysis_artifact`
- `missing_corpus_program_decision_artifact`
- `invalid_artifact_json`
- `invalid_artifact_contract`
- `basis_snapshot_mismatch`
- `derived_decision_mismatch`
- `frozen_helper_surface_evidence_not_current`
- `frozen_helper_surface_floor_mismatch`

The verifier should also report field-level mismatch detail inside the relevant
check object. That makes failures debuggable without inventing a new error
taxonomy.

### Artifact-field audit

The probe is feasible under current constraints because the emitted artifacts
already contain the read-side truth the verifier needs:

- recommendation-analysis artifact already carries:
  - `recommendation_status`
  - `decision_summary.decision_status`
  - `decision_summary.top_candidate_id`
  - `decision_summary.open_blockers`
  - `evidence_summary.missing_evidence`
  - `evidence_summary.stale_evidence`
- corpus-program-decision artifact already carries:
  - `analysis_basis_path`
  - `analysis_basis_sha256`
  - `basis_snapshot`
  - `decision_action`
  - `decision_basis_code`
  - `pivot_target_class`
  - `required_next_action`

That is enough to recompute the basis snapshot and derived decision contract
from the validated analysis artifact and compare them against the emitted
decision artifact without touching raw corpus inputs.

### Adoption boundary

M39 counts as a consumer probe only if the new command replaces the repeated
shell ladder in standing repo-root surfaces:

1. `ORCH_PLAN.md` baseline verification block around lines `340-350`
2. `ORCH_PLAN.md` later verification block around lines `582-592`
3. `ORCH_PLAN.md` final verification wall around lines `938-947`
4. accepted M39 run proof output under `.runs/`

If implementation lands but these surfaces stay on the old shell ladder, the
repo has not proven a third honest consumer.

## Code Quality Review

### Quality bar

The code should be explicit, minimal, and hostile to duplication:

1. keep all verifier-specific logic in one new module
2. call existing validators instead of duplicating contract rules
3. compute parity details directly from validated artifacts so the JSON can name
   what failed
4. do not add a generic verification framework, traits, or registry
5. do not thread this command through unrelated family modules

### DRY decisions

- Reuse `FamilyRecommendationAnalysisArtifact::validate(...)` and
  `CorpusProgramDecisionArtifact::validate(...)` as the contract floor.
- Reuse `corpus_program_basis_snapshot(...)` and
  `derive_corpus_program_decision_contract(...)` for parity checks.
- Reuse canonical latest paths from `paths.rs`.
- Keep verifier-local JSON result structs inside `verify.rs`. They do not
  belong in `promotion_artifacts.rs` because they are command output, not repo
  artifacts.

### Explicit-over-clever decisions

- One new module is preferred over hiding the command inside `recommend.rs`.
- Stable reason strings are preferred over serializing raw Rust error text.
- Fixed-path verification is preferred over configurable path plumbing.
- A small local result model is preferred over trying to reuse every artifact
  struct for a different purpose.

## Implementation Plan

The implementation order is strict because later steps depend on a frozen
command contract, not a moving draft.

### Step 1 - Wire the CLI surface

Files:

- `xtask/src/lib.rs`
- `xtask/src/family/mod.rs`

Changes:

1. add `VerifyDecisionContract { format: String }` to `FamilyCommand`
2. dispatch it to `verify::run(workspace_root, &format)`
3. export `pub mod verify;` from `xtask/src/family/mod.rs`

Acceptance:

- `cargo xtask family verify-decision-contract --help` shows the new subcommand
- non-`json` format exits with the same invalid-input behavior as existing
  family commands

### Step 2 - Implement the verifier module

File:

- `xtask/src/family/verify.rs`

Responsibilities:

1. load both canonical latest artifacts
2. deserialize them with stable failure categorization
3. validate them using the existing artifact validators
4. recompute expected basis snapshot from validated analysis
5. recompute expected derived decision contract from validated analysis
6. compare observed vs expected parity fields
7. verify the frozen helper-surface floor exactly
8. emit structured JSON and return the correct exit status

Implementation rule:

- the verifier may read artifact files and call existing validation helpers
  only
- it must not call coverage collection, recommendation generation, or any other
  write-side path

### Step 3 - Freeze the helper-surface floor in command logic

The verifier must assert all of these together:

- `recommendation_status == no_strong_candidate`
- `decision_summary.decision_status == not_recommended`
- `decision_summary.open_blockers == [helper_surface_not_promotable]`
- `evidence_summary.missing_evidence == []`
- `evidence_summary.stale_evidence == []`
- `decision_action == pivot_to_architecture_shared_core_follow_on`
- `decision_basis_code == durable_non_promotable_helper_surface`
- `required_next_action == author_architecture_follow_on_plan`

Rationale:

- M39 is intentionally frozen to the M38 floor
- this prevents the probe from silently broadening into a general policy
  checker

### Step 4 - Add tests before orchestration adoption

Primary test home:

- `xtask/src/family/verify.rs` unit tests
- `xtask/src/lib.rs` CLI dispatch test coverage if needed

Required test categories:

1. happy path on the frozen helper-surface floor
2. missing recommendation artifact
3. missing decision artifact
4. invalid JSON
5. artifact-contract validation failure
6. basis snapshot mismatch
7. derived decision mismatch
8. stale or missing evidence on the analysis artifact
9. frozen floor mismatch with one tuple field changed
10. CLI rejects non-`json` format

### Step 5 - Replace the standing orchestration ladder

Primary file:

- `ORCH_PLAN.md`

Changes:

1. replace the repeated validate + `jq` clusters at the named blocks with the
   new command
2. keep any surrounding branch, sha, and status commands that provide useful
   proof context
3. capture verifier stdout to a proof file during the accepted run

Target shape:

```bash
cargo xtask family verify-decision-contract --format json | tee \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
```

Optional explicit JSON gate if the proof flow wants it:

```bash
jq -e '.overall_verdict == "pass"' \
  .runs/m39_verification_consumer_probe/verify-decision-contract.stdout.json
```

### Step 6 - Close with an explicit verdict

M39 closes with exactly one of:

1. `candidate third consumer observed, but the kernel still stays local`
2. `third honest consumer proven`
3. `keep the kernel local`

The closeout record must explain which adoption bar was or was not met.

## Test Review

100% of new codepaths and maintainer-facing behavior must be covered in the
plan before implementation starts. This is not optional cleanup work.

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] xtask/src/lib.rs
    │
    └── FamilyCommand::VerifyDecisionContract
        ├── [REQ TEST] format == "json" -> dispatches verifier
        └── [REQ TEST] format != "json" -> invalid input error

[+] xtask/src/family/verify.rs
    │
    ├── load recommendation.latest.json
    │   ├── [REQ TEST] file exists + valid JSON
    │   ├── [REQ TEST] missing file -> missing_recommendation_analysis_artifact
    │   └── [REQ TEST] invalid JSON -> invalid_artifact_json
    │
    ├── load corpus-program-decision.latest.json
    │   ├── [REQ TEST] file exists + valid JSON
    │   ├── [REQ TEST] missing file -> missing_corpus_program_decision_artifact
    │   └── [REQ TEST] invalid JSON -> invalid_artifact_json
    │
    ├── validate analysis artifact
    │   ├── [REQ TEST] valid artifact -> pass
    │   └── [REQ TEST] invalid contract -> invalid_artifact_contract
    │
    ├── validate decision artifact
    │   ├── [REQ TEST] valid artifact -> pass
    │   └── [REQ TEST] invalid contract -> invalid_artifact_contract
    │
    ├── recompute expected basis snapshot
    │   ├── [REQ TEST] exact parity -> pass
    │   └── [REQ TEST] field drift -> basis_snapshot_mismatch
    │
    ├── recompute expected derived decision tuple
    │   ├── [REQ TEST] exact parity -> pass
    │   └── [REQ TEST] field drift -> derived_decision_mismatch
    │
    └── enforce frozen helper-surface floor
        ├── [REQ TEST] exact M38 floor -> pass
        ├── [REQ TEST] missing/stale evidence -> frozen_helper_surface_evidence_not_current
        └── [REQ TEST] tuple drift -> frozen_helper_surface_floor_mismatch
```

### Maintainer flow coverage

```text
MAINTAINER FLOW COVERAGE
========================
[+] Standing proof floor
    │
    ├── [REQ TEST] old shell path passes on frozen artifact set
    ├── [REQ TEST] new verifier passes on same artifact set
    └── [REQ TEST] both surfaces fail on the same intentionally drifted fixture

[+] ORCH_PLAN adoption
    │
    ├── [REQ TEST] named verification blocks replaced with verifier command
    └── [REQ TEST] accepted proof flow captures machine-readable verifier output

[+] Regression protection
    │
    └── [CRITICAL] if any existing artifact contract change breaks verifier parity,
        implementation must add or update a regression test before merge
```

### Required implementation-time commands

Baseline proof:

```bash
cargo test -p xtask
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
```

M39 verifier proof:

```bash
cargo xtask family verify-decision-contract --format json
```

Parity proof:

```bash
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.recommendation_status == "no_strong_candidate"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.decision_status == "not_recommended"' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_summary.open_blockers == ["helper_surface_not_promotable"]' \
  .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json
jq -e '.decision_action == "pivot_to_architecture_shared_core_follow_on"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.decision_basis_code == "durable_non_promotable_helper_surface"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
jq -e '.required_next_action == "author_architecture_follow_on_plan"' \
  .semantic-family-artifacts/family-promotion/analysis/corpus-program-decision.latest.json
cargo xtask family verify-decision-contract --format json
```

### Acceptance checklist

M39 is not complete until all of these are true:

1. the verifier returns `0` on the frozen M38 floor
2. the verifier returns non-zero on each modeled failure category
3. command JSON includes stable `checks` and `failure_reasons`
4. `ORCH_PLAN.md` named proof blocks use the verifier instead of the repeated
   `jq` ladder
5. parity with the old shell path is recorded on the same artifact set
6. closeout picks one and only one allowed verdict

## Failure Modes Registry

| Failure mode | Test coverage required | Error handling required | Operator-visible result |
|---|---|---|---|
| recommendation artifact missing | yes | map to `missing_recommendation_analysis_artifact` | clear fail in stdout JSON + non-zero exit |
| decision artifact missing | yes | map to `missing_corpus_program_decision_artifact` | clear fail in stdout JSON + non-zero exit |
| invalid JSON in either artifact | yes | map to `invalid_artifact_json` | clear fail in stdout JSON + non-zero exit |
| artifact schema drift | yes | map to `invalid_artifact_contract` | clear fail in stdout JSON + non-zero exit |
| analysis basis evidence becomes stale | yes | map to `frozen_helper_surface_evidence_not_current` | clear fail naming evidence freshness, not silent pass |
| basis snapshot field drift | yes | map to `basis_snapshot_mismatch` | clear fail with mismatched field list |
| derived decision tuple drift | yes | map to `derived_decision_mismatch` | clear fail with mismatched field list |
| frozen helper-surface tuple drift | yes | map to `frozen_helper_surface_floor_mismatch` | clear fail with observed vs expected tuple |

Critical gap rule:

- any failure mode that could currently become a silent false green is a merge
  blocker for M39
- the verifier exists specifically to eliminate that silent-failure class from
  the operator path

## Performance And Operability Review

M39 adds no runtime hot path. It is an operator command over two JSON files.

Expected performance shape:

- two file reads
- two JSON deserializations
- existing validation calls
- two small semantic recomputations
- one JSON write to stdout

Performance risks:

- effectively none at repo scale
- the main operability risk is human, not CPU: the command must stay easier to
  understand than the shell ladder it replaces

Operability guardrails:

1. fixed canonical paths keep the command honest
2. stable failure reasons keep CI and humans aligned
3. structured stdout keeps proof logs machine-readable
4. non-zero exit on any drift keeps ORCH verification trustworthy

## Worktree Parallelization Strategy

This plan has one real parallel seam after the verifier contract is frozen.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| CLI + verifier implementation | `xtask/src/`, `xtask/src/family/` | — |
| Verifier tests and parity fixtures | `xtask/src/`, `xtask/src/family/`, `.semantic-family-artifacts/` | CLI + verifier implementation |
| ORCH adoption rewrite | repo-root docs | verifier command contract frozen |
| Closeout proof capture | `.runs/`, repo-root docs | verifier tests green, ORCH adoption drafted |
| Final milestone authority update | `PLAN.md`, optional closeout docs | all prior steps |

### Parallel lanes

Lane A: CLI + verifier implementation -> verifier tests and parity fixtures  
Lane B: ORCH adoption rewrite -> closeout proof capture  
Lane C: final milestone authority update

### Execution order

1. Launch Lane A first. The command contract and JSON shape must freeze before
   any doc adoption is trustworthy.
2. Once Lane A has the command interface and failure reason vocabulary locked,
   launch Lane B in parallel to replace the named `ORCH_PLAN.md` proof blocks.
3. Merge Lane A and Lane B, run the full parity proof, then finish Lane C with
   the accepted closeout wording.

### Conflict flags

- Lane A owns `xtask/src/` and must stay single-owner.
- Lane B is safe in parallel after the command contract freezes because it
  touches repo-root docs, not Rust modules.
- Lane C should wait for both prior lanes because it is the authoritative
  summary lane and should not race the actual implementation verdict.

If the team chooses not to update `ORCH_PLAN.md` in the same change, then the
correct answer is:

`Sequential implementation only, because the consumer claim cannot be proven without doc adoption.`

## Deliverables

M39 is complete only when all of these exist:

1. this authoritative `PLAN.md`
2. `cargo xtask family verify-decision-contract --format json`
3. tests covering happy path plus all named failure reasons
4. updated `ORCH_PLAN.md` proof blocks using the verifier
5. one accepted-run parity proof log or closeout record
6. one explicit verdict on whether the third-consumer claim is proven

## Success Criteria

M39 is done only when all of these are true:

1. the new verifier works on the live M38 floor without widening semantics
2. the verifier proves the same pass/fail result as the old shell path on the
   same artifact set
3. the verifier returns stable machine reasons for all modeled failures
4. repo-root `ORCH_PLAN.md` adopts the verifier in the named standing proof
   blocks
5. the closeout can truthfully state one of:
   - candidate third consumer observed
   - third honest consumer proven
   - keep the kernel local

## Closeout Decision Table

| Outcome | Rule |
|---|---|
| `candidate third consumer observed, but the kernel still stays local` | the command works and parity passes, but standing repo-root adoption is incomplete or not yet merged |
| `third honest consumer proven` | the command works, parity passes, and the named `ORCH_PLAN.md` proof blocks adopt it as the standing path |
| `keep the kernel local` | parity fails, adoption fails, or the command requires enough special casing that it is not an honest reusable consumer |

## Completion Summary

- Step 0 - Scope Challenge: scope accepted as-is for a bounded verification
  consumer probe
- Architecture Review: one new read-side consumer, zero new write-side
  semantics
- Code Quality Review: reuse validators and kernel truth, avoid framework creep
- Test Review: full codepath and maintainer-flow coverage defined
- Performance Review: no runtime hot path, operator-surface only
- NOT in scope: written
- What already exists: written
- TODOS.md handling: existing post-M37 follow-up TODOs remain authoritative
- Failure modes: all non-happy paths named with stable machine reasons
- Parallelization: 3 lanes total, 1 real parallel seam after command contract freeze
- Lake Score: complete option chosen, including contract, parity, adoption, and
  closeout verdict

## Next Actions

1. Treat this file as the authoritative M39 execution boundary.
2. Implement the verifier as a thin read-side command in `xtask/src/family/verify.rs`.
3. Prove parity against the current shell path on the same frozen artifact set.
4. Replace the named `ORCH_PLAN.md` proof blocks only after the command contract
   is frozen.
5. Close with an explicit verdict about whether the third-consumer claim was
   actually proven.
