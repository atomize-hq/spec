# M57: Shared-Core Portability Adoption Closeout Plan

Status: **implementation plan**
Milestone: **M57**
Milestone family: **shared-core-portability**
Implementation readiness: **ready for bounded execution**
Plan scope: **close out the remaining repo-root authority and proof-wall drift around the already-frozen `xtask/src/family/analysis_core/*` owner seam, without changing semantic stop-state behavior, CLI shape, JSON contracts, or artifact paths**
Base branch: **main**
Working branch: **feat/m40-plus**
Validated at commit: **`504b1e3`**
Last rewritten: **2026-05-13**

Supersedes:

- the stale M56 cross-library TypeScript plan previously maintained at this path
- the M57 design draft at `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260513-212023.md`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260513-212023.md`
- `.runs/m53_shared_core_portability_closeout/validation/kickoff/PLAN.md`
- `.runs/m53_shared_core_portability_closeout/acceptance-ledger.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `TODOS.md`
- `ORCH_PLAN.md`

Primary repo surfaces:

- `xtask/src/family/mod.rs`
- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/analysis_core/helper_surface.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/lib.rs`

## Executive Summary

M56 landed. That work is real, but the repo-root authority file was left behind.

The branch-level design doc for the next milestone is about shared-core portability adoption closeout on the frozen `analysis_core` seam. The current `PLAN.md` was still a full M56 Bun/TypeScript execution contract. That mismatch is worse than ugly. It points implementers at the wrong product problem.

The current code truth is narrower and more boring:

1. `xtask/src/family/analysis_core/*` is already the semantic owner surface.
2. `xtask/src/family/helper_surface.rs` and `xtask/src/family/decision_kernel.rs` are already compatibility-only passthrough shims.
3. Maintainer docs already describe that ownership story correctly.
4. The family-analysis stop-state still truthfully says `stop`.
5. The one concrete drift found during revalidation is proof-wall drift in `xtask/src/lib.rs`: one locked recommendation coverage assertion still expects pre-M56 corpus counts and currently keeps `cargo test -p xtask` red.

M57 therefore is not a new architecture milestone. It is a closeout and truth-sync milestone:

1. align repo-root authority with the actual branch direction
2. keep the frozen owner-seam story explicit
3. repair the stale proof-wall expectation that M56 invalidated
4. touch docs only if the final code diff would otherwise leave them false
5. rerun the full frozen proof floor and stop if any new drift appears

If this expands into new semantic-review policy, corpus-run spending, TypeScript work, consumer rewires, schema churn, or shared-core extraction, the plan has widened past its budget and must stop.

## Current Validated Basis

Validated from the current tree on `feat/m40-plus` at `504b1e3`.

Commands run:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Observed command truth:

- `collect_signals.sh`
  - `recommendation_status = insufficient_real_corpus`
  - `decision_status = not_recommended`
  - `decision_action = stop`
  - `decision_basis_code = no_actionable_candidate`
  - `required_next_action = record_stop_without_new_milestone`
- `cargo xtask family verify-decision-contract --format json`
  - `overall_verdict = "pass"`
  - all five checks passed
- `cargo xtask family corpus-decision --format json`
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo xtask family recommend --format json`
  - `recommendation_status = "insufficient_real_corpus"`
  - `decision_summary.decision_status = "not_recommended"`
- `cargo test -p xtask`
  - `155 passed; 1 failed`
  - current failing test:
    - `tests::recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held`
  - observed stale assertion:
    - expected source unit counts: `[6, 12, 9, 1, 2]`
    - actual source unit counts: `[6, 12, 9, 3, 3]`

Observed code truth:

- `xtask/src/family/mod.rs`
  - already exports `analysis_core` first
  - already groups `decision_kernel` and `helper_surface` under explicit compatibility-only framing
- `xtask/src/family/helper_surface.rs`
  - already a pure passthrough to `analysis_core::helper_surface`
- `xtask/src/family/decision_kernel.rs`
  - already a pure passthrough to `analysis_core::decision_contract`
- `xtask/src/family/recommend.rs`
  - already imports `analysis_core` directly
- `xtask/src/family/verify.rs`
  - already imports `analysis_core` directly
- `xtask/src/family/promotion_artifacts.rs`
  - already imports `analysis_core` directly
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
  - already describes `analysis_core/*` as the live owner surface
- `docs/recommendation_corpus_expansion_program_v0.1.md`
  - already makes the same compatibility-only distinction

That is the whole game. The semantic migration is already done. The remaining work is repo-root authority repair, proof-wall repair, and only-if-needed wording sync.

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing owner or proof surface | M57 action |
| --- | --- | --- |
| semantic owner seam | `xtask/src/family/analysis_core/*` | Reuse, do not widen |
| command-facing owner presentation | `xtask/src/family/mod.rs` | Reuse unless audit finds false wording |
| compatibility shims | `xtask/src/family/helper_surface.rs`, `xtask/src/family/decision_kernel.rs` | Preserve as pure passthroughs |
| direct consumers | `recommend.rs`, `verify.rs`, `promotion_artifacts.rs` | Read-only proof surfaces |
| command dispatch and regression proof | `xtask/src/lib.rs` | Repair stale proof expectation only |
| stop-state proof floor | `collect_signals.sh`, `verify-decision-contract`, `corpus-decision`, `cargo test -p xtask` | Reuse verbatim |
| maintainer wording | `docs/semantic_family_capability_corpus_guide_v0.1.md`, `docs/recommendation_corpus_expansion_program_v0.1.md` | Touch only if final diff makes them false |
| prior closeout precedent | `.runs/m53_shared_core_portability_closeout/*` | Reuse as implementation template, not product scope |

### Minimum complete slice

The minimum honest M57 slice is:

1. replace the stale repo-root M56 authority with the actual M57 closeout contract
2. verify that `mod.rs` and both shims still tell one ownership story
3. repair the stale `xtask/src/lib.rs` proof expectation that M56 invalidated
4. rerun the frozen stop-state proof floor
5. sync docs only if the final code diff proves a wording mismatch

Anything smaller is fake done.

Examples:

- rewriting only `PLAN.md` but leaving the red `cargo test -p xtask` proof wall untouched is fake done
- patching the red test while leaving repo-root authority aimed at the wrong milestone is fake done

Anything larger is scope growth:

- new `analysis_core` extraction
- consumer rewires
- new schema or CLI work
- new family-analysis semantics
- more TypeScript execution

### Complexity check

This is a small closeout milestone.

Expected primary write scope:

- `PLAN.md`
- `xtask/src/lib.rs`

Possible secondary write scope, only if truth requires it:

- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

No new crate. No new command. No new artifact kind. No new runtime. No new infrastructure.

### Search check

No new framework or unfamiliar subsystem is entering the repo.

- **[Layer 1]** reuse the already-shipped `analysis_core` seam
- **[Layer 1]** reuse the current proof floor and xtask test harness
- **[Layer 1]** keep the shim modules as explicit compatibility-only passthroughs
- **[Layer 3]** the first-principles insight is that M57 is smaller than the design doc's open questions made it look, because most of the seam-adoption work is already true on HEAD

### TODOS cross-reference

Relevant deferred items in `TODOS.md` remain deferred:

- generalized multi-wedge decision layer
- cross-crate family-analysis shared core
- public semantic fingerprint fields

M57 must not silently consume any of those follow-ons.

### Completeness check

Choose the complete closeout, not the cosmetic one.

The complete version is still small:

1. authority truth
2. code truth
3. proof truth
4. doc truth, only if needed

The shortcut would be "leave the red proof because it is only a stale expectation." That is not acceptable. A stale proof wall is still a lie.

### Distribution check

No new distributable artifact is introduced.

This is internal `xtask` and maintainer-surface cleanup inside the existing repo and the existing `xtask` binary.

## Exact Contract

### In scope

- repo-root authority reset from stale M56 execution scope to truthful M57 closeout scope
- proof-wall repair in `xtask/src/lib.rs` where locked corpus expectations no longer match current repo truth
- audit of `xtask/src/family/mod.rs`, `helper_surface.rs`, and `decision_kernel.rs` to confirm the owner-surface story is still single and explicit
- final stop-state parity proof via:
  - `collect_signals.sh`
  - `cargo xtask family verify-decision-contract --format json`
  - `cargo xtask family corpus-decision --format json`
  - `cargo test -p xtask`
- docs sync only when the final code diff would otherwise leave maintainer wording false

### Not in scope

- new semantic-review capability
- new family promotion work
- corpus run `1`
- recommendation-policy changes
- shared-core extraction into a new crate
- consumer rewires in `recommend.rs`, `verify.rs`, or `promotion_artifacts.rs`
- CLI flag changes
- JSON schema changes
- artifact-path changes
- TypeScript lane work
- rewriting `ORCH_PLAN.md` as part of this slice
- opportunistic cleanup of unrelated xtask tests beyond the directly observed stale closeout proof

### Locked decisions

These are not open questions anymore:

1. `xtask/src/family/analysis_core/*` remains the only semantic owner surface.
2. `xtask/src/family/helper_surface.rs` and `xtask/src/family/decision_kernel.rs` stay as compatibility-only passthrough shims in this milestone.
3. `recommend.rs`, `verify.rs`, and `promotion_artifacts.rs` are read-only proof surfaces for M57.
4. `cargo xtask family verify-decision-contract --format json` must stay `pass`.
5. `cargo xtask family corpus-decision --format json` must keep:
   - `decision_action = "stop"`
   - `decision_basis_code = "no_actionable_candidate"`
   - `required_next_action = "record_stop_without_new_milestone"`
6. The stale `xtask/src/lib.rs` recommendation coverage assertion is in scope because it currently blocks a green proof wall.
7. Docs change only if they become false after the final code diff. If they already say the truth, leave them alone.

### Abort and re-scope triggers

Stop M57 and write a new plan instead if any of these become true:

1. fixing the red proof wall requires semantic edits in `recommend.rs`, `verify.rs`, or `promotion_artifacts.rs`
2. the only way to make `cargo test -p xtask` green is to loosen assertions instead of updating them to current locked truth
3. owner-surface truth now requires a new abstraction layer or a new helper module
4. doc sync requires changing product claims rather than tightening ownership wording
5. proof parity requires changing command output, CLI flags, or artifact JSON
6. more unrelated xtask failures appear and they are not caused by the shared-core closeout blast radius

## Current vs Target State

### Current

```text
repo-root PLAN.md
  -> stale M56 TypeScript authority

analysis_core owner seam
  -> already correct

compatibility shims
  -> already correct

maintainer docs
  -> already correct

xtask proof wall
  -> one stale locked recommendation coverage assertion
```

### Target

```text
repo-root PLAN.md
  -> truthful M57 closeout authority

analysis_core owner seam
  -> unchanged, still the only semantic owner surface

compatibility shims
  -> unchanged, still explicit passthroughs

maintainer docs
  -> unchanged unless falsehood is proven

xtask proof wall
  -> green, with current corpus counts encoded explicitly
```

### Dependency graph

```text
analysis_core/*
  ├── helper_surface.rs
  ├── decision_contract.rs
  └── proof_fingerprint.rs
        │
        ├── recommend.rs
        ├── verify.rs
        ├── promotion_artifacts.rs
        ├── helper_surface.rs shim
        └── decision_kernel.rs shim

xtask/src/lib.rs
  └── command-dispatch and regression proof surface

PLAN.md
  └── repo-root authority for what implementers should do next
```

## Implementation Plan

### Phase 1: Refresh repo-root authority and freeze scope

Files:

- `PLAN.md`

Changes:

1. remove the stale M56 Bun/TypeScript execution contract from repo-root authority
2. replace it with the bounded M57 closeout contract in this file
3. pin the exact observed stop-state truth and the known stale-proof issue
4. pin the exact write scope so M57 cannot silently expand

Acceptance:

- repo-root authority points at the same problem the branch-level design doc points at
- the plan no longer authorizes TypeScript execution work
- the plan makes the stale `xtask` proof issue explicit instead of hiding it

### Phase 2: Audit owner-surface presentation and keep it boring

Files:

- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`

Changes:

1. verify `analysis_core` is still presented as the maintained owner surface
2. verify both shims are still pure passthroughs with no semantic logic
3. touch these files only if the audit finds a real falsehood
4. do not "clean them up" just because they look small enough to edit

Acceptance:

- either no code change is needed, or any edit is purely wording or presentation and does not change behavior
- no new helper layer appears
- no semantic logic moves into shims

### Phase 3: Repair the stale proof wall

Files:

- `xtask/src/lib.rs`

Changes:

1. update the locked recommendation coverage assertion to current repo truth if the kickoff audit confirms the corpus counts are now durably `[6, 12, 9, 3, 3]`
2. keep the assertion explicit, do not replace it with filesystem-driven "auto discover expected counts" logic
3. keep the existing owner-surface and compatibility-surface proof intact
4. if another nearby assertion is stale for the same reason, repair it in the same change only if it is in the same test blast radius

Acceptance:

- `tests::recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held` passes
- the test still proves locked truth instead of becoming more permissive
- the proof surface still rejects semantic drift

### Phase 4: Docs sync only if the code diff makes docs false

Files, only if needed:

- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Changes:

1. only touch wording that would be false after the final code diff
2. keep the same stop-state truth
3. keep the same owner-surface story
4. do not reopen roadmap narrative or corpus-strategy narrative

Acceptance:

- docs either stay untouched because they were already true, or land with a narrow wording sync only

### Phase 5: Final proof wall

Commands:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
cargo test -p xtask
```

Acceptance:

- `recommendation_status = insufficient_real_corpus`
- `decision_status = not_recommended`
- `decision_action = stop`
- `decision_basis_code = no_actionable_candidate`
- `required_next_action = record_stop_without_new_milestone`
- `cargo xtask family verify-decision-contract --format json` stays `pass`
- `cargo test -p xtask` is green

## Code Quality Review

### Guardrails

- No new abstraction layer.
- No semantic logic in shim files.
- No dynamic "figure out expected counts from the current tree" test logic.
- No consumer rewires.
- No comment churn that says the same thing twice with different words.
- Minimal diff wins. If `mod.rs` and the shims are already true, do not touch them.

### DRY targets

- Keep the owner-surface story singular: `analysis_core/*` owns semantics, everywhere.
- Keep the stale-proof repair local to the existing xtask proof surface. Do not duplicate the same counts or coverage expectations into a second helper.

### Technical-debt traps to avoid

- silently broadening M57 into "shared-core cleanup"
- updating the failing xtask test by deleting useful assertions
- fixing proof drift by teaching the test to accept whatever the filesystem currently says
- reopening historical M51/M53 debates that the current branch already settled

## Test Review

### Test framework detection

This repo uses the Rust test harness.

- runtime: `Cargo.toml`
- primary suite: `cargo test -p xtask`
- command parity proof: `collect_signals.sh`, `recommend`, `corpus-decision`, `verify-decision-contract`

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/mod.rs
    ├── [EXISTS] `analysis_core` presented as maintained owner surface
    └── [PLAN GUARD] compatibility block remains compatibility-only

[+] xtask/src/family/helper_surface.rs
    └── [EXISTS] passthrough-only shim over `analysis_core::helper_surface`

[+] xtask/src/family/decision_kernel.rs
    └── [EXISTS] passthrough-only shim over `analysis_core::decision_contract`

[+] xtask/src/family/recommend.rs
    └── [EXISTS] direct `analysis_core` consumer, read-only proof surface

[+] xtask/src/family/verify.rs
    └── [EXISTS] direct `analysis_core` consumer, read-only proof surface

[+] xtask/src/family/promotion_artifacts.rs
    └── [EXISTS] direct `analysis_core` consumer, read-only proof surface

[+] xtask/src/lib.rs
    ├── [EXISTS] compatibility-path regression proof from M53
    ├── [EXISTS] command-dispatch proof for `verify-decision-contract`
    └── [GAP] stale locked recommendation coverage counts after M56 corpus growth

[+] command proof floor
    ├── [EXISTS] `collect_signals.sh` stop-state parity
    ├── [EXISTS] `family corpus-decision --format json`
    ├── [EXISTS] `family verify-decision-contract --format json`
    └── [GAP] full `cargo test -p xtask` currently red because one locked assertion is stale

─────────────────────────────────
COVERAGE TARGET: 100% of the closeout blast radius
QUALITY TARGET: explicit locked truth, no dynamic expectation logic
CRITICAL GAP: stale `xtask/src/lib.rs` coverage assertion keeps the proof wall red
─────────────────────────────────
```

### Required tests to add or refresh

#### `xtask/src/lib.rs`

- refresh `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held`
  - expected source ids stay:
    - `examples_ecommerce`
    - `m19_semantic_falsification_pack`
    - `m20_unsupported_truth_pack`
    - `examples_shared_spec`
    - `examples_crosslib_app`
  - expected source unit counts update to current locked truth:
    - `[6, 12, 9, 3, 3]`
  - total function coverage stays aligned with current truth from `collect_signals.sh`

#### Read-only proof surfaces that must remain green

- `family_verify_decision_contract_help_exits_successfully`
- `family_verify_decision_contract_rejects_non_json_format_from_cli_dispatch`
- existing `analysis_core` branch tests under:
  - `family::analysis_core::helper_surface::tests::*`
  - `family::analysis_core::decision_contract::tests::*`
  - `family::analysis_core::proof_fingerprint::tests::*`

### Test command wall

Run these exact commands before calling the milestone complete:

```bash
cargo test -p xtask recommendation_command_path_writes_same_bytes_and_locked_corpus_is_ranked_with_arithmetic_ready_and_unknown_overlap_held -- --nocapture
cargo test -p xtask family::analysis_core::helper_surface::tests -- --nocapture
cargo test -p xtask family::analysis_core::decision_contract::tests -- --nocapture
cargo test -p xtask family::analysis_core::proof_fingerprint::tests -- --nocapture
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
cargo test -p xtask
```

## Performance Review

This milestone is not runtime-performance-driven.

Real performance and maintenance risks are smaller:

1. turning a locked explicit regression test into dynamic discovery would make failures slower to diagnose
2. widening the blast radius into consumer rewires would burn time on non-user-facing churn
3. rerunning the full xtask proof wall repeatedly during implementation is wasted time once the targeted stale test is repaired

Performance acceptance:

- no new filesystem crawl in tests
- no new helper that computes expectations indirectly
- one targeted test run before the full suite is enough

## Failure Modes Registry

| Failure mode | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- |
| repo-root authority still points at M56 after code closeout lands | Yes | No silent acceptance | implementers pick the wrong milestone |
| stale coverage assertion is "fixed" by removing proof value | Yes | Yes, review must reject it | green test suite that no longer protects drift |
| shim file quietly regains semantic logic | Yes | Yes, code review + proof wall | future ownership confusion |
| docs drift back into dual-ownership wording | Manual proof | No | maintainer confusion, false teaching surface |
| full xtask suite stays red for unrelated reasons | Yes | Yes, explicit re-scope | milestone cannot claim closeout |

Critical gap rule:

The milestone is blocked until the stale `xtask/src/lib.rs` proof issue is closed or a re-scope decision is made with evidence that the failure is not actually in the M57 blast radius.

## Worktree Parallelization Strategy

This plan has limited parallelization value. Most real work is sequential because the closeout is small and one proof file is the critical path.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Scope and owner-surface audit | `PLAN.md`, `xtask/src/family/`, `docs/` | — |
| Lane A: proof-wall repair | `xtask/src/lib.rs` | audit |
| Lane B: owner-surface wording sync, only if audit proves it is false | `xtask/src/family/mod.rs`, `xtask/src/family/helper_surface.rs`, `xtask/src/family/decision_kernel.rs` | audit |
| Lane C: docs sync, only if code diff makes docs false | `docs/semantic_family_capability_corpus_guide_v0.1.md`, `docs/recommendation_corpus_expansion_program_v0.1.md` | Lane A + Lane B decision |
| Final proof wall | xtask commands and tests | Lane A, optional Lane B, optional Lane C |

### Parallel lanes

- `Lane A: xtask/src/lib.rs` proof-wall repair
- `Lane B: owner-surface wording audit/sync only if the audit finds a real falsehood`
- `Lane C: docs sync only if the final code diff would otherwise leave docs false`

### Execution order

1. run the owner-surface audit first
2. if the audit confirms `mod.rs` and both shims are already truthful, skip Lane B entirely
3. execute Lane A next and get the targeted stale xtask test green
4. only if docs are now false, run Lane C
5. run the final proof wall last

### Conflict flags

- `xtask/src/lib.rs` is single-owner work. Do not split it across lanes.
- `xtask/src/family/` wording sync is only worth doing if the audit finds a real falsehood. Otherwise it is churn.
- Docs must not move ahead of proof. They are last and conditional.

Net recommendation:

Sequential implementation is preferred. Only spin a parallel docs lane if the code diff makes doc wording false.

## NOT in Scope

- semantic-review roadmap resets
- recommendation-policy changes
- corpus program redesign
- new coverage sources or corpus inputs
- changes to `collect_signals.sh`
- changes to `recommend.rs`, `verify.rs`, or `promotion_artifacts.rs` unless a real blast-radius blocker is discovered
- `ORCH_PLAN.md` rewrite
- opportunistic xtask cleanup outside the directly observed stale closeout proof

## Acceptance Checklist

- [ ] repo-root `PLAN.md` is aligned to M57 closeout scope
- [ ] `analysis_core/*` remains the only semantic owner surface
- [ ] `helper_surface.rs` and `decision_kernel.rs` remain compatibility-only passthroughs
- [ ] the stale `xtask/src/lib.rs` recommendation coverage assertion is refreshed to current locked truth
- [ ] `collect_signals.sh` still reports the same stop-state summary
- [ ] `cargo xtask family recommend --format json` still reports `insufficient_real_corpus` and `not_recommended`
- [ ] `cargo xtask family corpus-decision --format json` still reports `stop` and `no_actionable_candidate`
- [ ] `cargo xtask family verify-decision-contract --format json` still passes
- [ ] `cargo test -p xtask` is green
- [ ] docs were either left alone because they were already true or landed with a narrow wording sync only

## Completion Summary

- Step 0: scope accepted as the bounded M57 closeout slice
- Architecture: existing `analysis_core` seam reused, not reinvented
- Code quality target: explicit locked proof, no clever dynamic expectations
- Test review: one concrete stale-proof gap identified in `xtask/src/lib.rs`
- Performance review: keep the closeout local and avoid new indirection
- NOT in scope: written
- What already exists: written
- Failure modes: written, with the red proof-wall gap called out explicitly
- Parallelization: 3 possible lanes, but sequential execution is preferred and likely sufficient
- Lake Score: 5/5 recommendations choose the complete bounded closeout over the cosmetic shortcut

## Next Step

Implement the proof-wall repair first.

If that repair turns out to need semantic consumer changes, stop immediately and write a new plan. That would no longer be M57 closeout. It would be a different milestone.
