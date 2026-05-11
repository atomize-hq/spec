# M51: Shared-Core Portability Adoption Implementation Plan

Status: **implementation plan**  
Milestone: **M51**  
Milestone family: **shared-core-portability**  
Implementation readiness: **ready for bounded execution**  
Plan scope: **finish post-M48 seam adoption by rewiring the last command-facing proof consumers onto `xtask/src/family/analysis_core/*`, demoting shim modules to explicit compatibility-only status, updating command-facing docs to match, and re-proving the frozen stop-state contract without changing behavior**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Last rewritten: **2026-05-11**

Supersedes:
- the prior repo-root M50 plan previously maintained at this path
- the design draft at [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-170532.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-170532.md)

Primary source artifacts:
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-170532.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-170532.md)
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-213500.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-213500.md)
- [ORCH_PLAN.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md)
- [.runs/m48_shared_core_portability_slice1_lane_a/closeout.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m48_shared_core_portability_slice1_lane_a/closeout.md)
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/checkpoints/20260506-181701-semantic-review-milestone-reset.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/checkpoints/20260506-181701-semantic-review-milestone-reset.md)
- [TODOS.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/TODOS.md)

Primary repo surfaces:
- [xtask/src/family/analysis_core/mod.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/analysis_core/mod.rs)
- [xtask/src/family/analysis_core/helper_surface.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/analysis_core/helper_surface.rs)
- [xtask/src/family/analysis_core/decision_contract.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/analysis_core/decision_contract.rs)
- [xtask/src/family/analysis_core/proof_fingerprint.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/analysis_core/proof_fingerprint.rs)
- [xtask/src/family/mod.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs)
- [xtask/src/family/helper_surface.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs)
- [xtask/src/family/decision_kernel.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs)
- [xtask/src/family/recommend.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs)
- [xtask/src/family/verify.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/verify.rs)
- [xtask/src/family/promotion_artifacts.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs)
- [xtask/src/lib.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs)
- [docs/semantic_family_capability_corpus_guide_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md)
- [docs/recommendation_corpus_expansion_program_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md)
- [docs/ai_promotion_and_multilanguage_milestones_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/ai_promotion_and_multilanguage_milestones_v0.1.md)

## Executive Summary

M48 already settled the seam.

`xtask/src/family/analysis_core/*` is the semantic owner surface. The repo already behaves that way in the direct command consumers. What is still wrong is that the repo does not *present* one ownership story everywhere:

- `xtask/src/family/mod.rs` still exports `decision_kernel` and `helper_surface` as peer modules
- `xtask/src/lib.rs` tests still import and call `decision_kernel` at the proof wall
- command-facing docs still describe the shims as semantic owners

That is the whole M51 problem.

M51 is a bounded adoption milestone. It does not widen the seam, extract a crate, change CLI behavior, spend corpus run `1`, or reopen family-selection logic. It finishes the adoption work that M48 deliberately left behind:

1. the proof wall points directly at `analysis_core`
2. the shims are visibly compatibility-only if they remain
3. the module topology stops teaching the pre-freeze ownership model
4. command-facing docs match the real dependency graph
5. the frozen stop-state contract still passes unchanged

## Decision This Plan Makes

This plan authorizes exactly one milestone:

1. Retarget the remaining command-facing proof consumers from shim-owned vocabulary to `analysis_core`.
2. Demote `xtask/src/family/helper_surface.rs` and `xtask/src/family/decision_kernel.rs` to explicit compatibility-only surfaces.
3. Make `xtask/src/family/mod.rs` present `analysis_core` as the maintained seam surface instead of exporting the old shim topology as peer truth.
4. Update the small set of maintainer-facing docs that still teach the pre-freeze ownership split.
5. Re-run the existing `xtask` proof floor and prove that the stop-state contract did not move.

This plan does not authorize:

- new family promotion work
- new corpus ranking logic
- new stop-state semantics
- new CLI commands, flags, JSON schemas, or artifact paths
- moving local path or write-policy logic into `analysis_core`
- new crate extraction
- repo-root roadmap rewrites
- second-language expansion
- a broad docs cleanup outside the named ownership surfaces

## Live Validated Basis

Validated from the current tree on `feat/m40-plus` at commit `21c3f31`.

Commands run:

```bash
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Observed command truth:

- `collect_signals.sh`
  - `recommendation_status = insufficient_real_corpus`
  - `decision_status = not_recommended`
  - `decision_action = stop`
  - `decision_basis_code = no_actionable_candidate`
  - `required_next_action = record_stop_without_new_milestone`
- `cargo xtask family recommend --format json`
  - `recommendation_status = "insufficient_real_corpus"`
  - `decision_summary.decision_status = "not_recommended"`
  - `ranked_candidates = []`
- `cargo xtask family corpus-decision --format json`
  - `decision_action = "stop"`
  - `decision_basis_code = "no_actionable_candidate"`
  - `required_next_action = "record_stop_without_new_milestone"`
- `cargo xtask family verify-decision-contract --format json`
  - `overall_verdict = "pass"`
  - all five checks pass

Observed code truth:

- [xtask/src/family/recommend.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/recommend.rs), [xtask/src/family/verify.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/verify.rs), and [xtask/src/family/promotion_artifacts.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs) already consume `analysis_core` directly.
- [xtask/src/family/helper_surface.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs) is currently a pure re-export shim over `analysis_core::helper_surface`.
- [xtask/src/family/decision_kernel.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs) is currently a pure re-export shim over `analysis_core::decision_contract`.
- [xtask/src/family/mod.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs) still exports `analysis_core`, `decision_kernel`, and `helper_surface` as peer modules.
- [xtask/src/lib.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs) still imports `decision_kernel` in the test module and still calls `decision_kernel::corpus_program_basis_snapshot(...)` at two proof-wall sites.
- [docs/semantic_family_capability_corpus_guide_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md) and [docs/recommendation_corpus_expansion_program_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md) still describe `helper_surface.rs` and `decision_kernel.rs` as semantic owners.

That is the real gap. Not seam discovery. Adoption.

## Step 0: Scope Challenge

### What Already Exists

| Sub-problem | Existing owner | Reuse verdict |
| --- | --- | --- |
| semantic owner boundary | `xtask/src/family/analysis_core/*` | reuse, do not redefine |
| direct production consumers | `recommend.rs`, `verify.rs`, `promotion_artifacts.rs` | reuse, keep behavior fixed |
| compatibility shims | `xtask/src/family/helper_surface.rs`, `xtask/src/family/decision_kernel.rs` | reuse only as passthroughs |
| command-facing module topology | `xtask/src/family/mod.rs` | reuse, but stop advertising shim ownership as peer truth |
| command proof wall | `xtask/src/lib.rs` tests | reuse, retarget the last shim-based calls |
| stop-state artifacts | `recommendation.latest.json`, `corpus-program-decision.latest.json`, `verify-decision-contract` output | reuse unchanged, prove parity |
| maintainer narrative | command-facing docs under `docs/` | reuse, update only stale ownership wording |

### Minimum Complete Slice

The minimum honest slice is:

1. remove the proof wall's dependency on `decision_kernel` as semantic owner truth
2. make the shim files explicitly compatibility-only
3. make `family/mod.rs` expose the current ownership graph honestly
4. update only the docs that still teach the old ownership split
5. re-run the frozen stop-state proof floor

Anything smaller is fake done.

If the code says `analysis_core`, but the tests or docs still teach `decision_kernel` as the owner path, the repo is still split-brained.

### Complexity Check

This slice touches more than eight files. That is acceptable because the blast radius is narrow and coherent:

- `xtask/src/family/`
- `xtask/src/lib.rs`
- three command-facing docs

No new services. No new crates. No new runtime paths. No new infra.

Recommendation: accept the bounded blast radius and reject any semantic widening beyond seam-adoption cleanup.

### Search Check

No unfamiliar infrastructure, concurrency model, or framework pattern is being introduced.

- **[Layer 1]** Prefer the existing Rust module boundary over inventing a new abstraction.
- **[Layer 1]** Prefer direct `analysis_core` imports over a new wrapper layer.
- **[Layer 3]** First-principles rule: ownership, proof, and docs must match the actual dependency graph or the next maintainer loses time for no product gain.

Search does not change the recommendation. The repo already has the right seam. This is adoption work.

### TODOS Cross-Reference

Relevant deferred items remain deferred:

- `Generalized multi-wedge decision layer`
- `Cross-crate family-analysis shared core`
- `Public semantic fingerprint fields`

M51 does not spend any of those tokens.

Default expectation: M51 should not create new TODOs. If implementation reveals a hidden non-test consumer outside the named write scope, stop and re-scope instead of quietly appending more cleanup work.

### Completeness Check

Choose the complete version:

- code truth
- proof truth
- docs truth
- stop-state parity proof

Do not ship the shortcut where only the tests move or only the docs move. The delta is minutes with CC. The cost of leaving the split around is repeated archaeology.

### Distribution Check

No new artifact type is introduced.

Distribution stays exactly the same:

- `cargo xtask family recommend --format json`
- `cargo xtask family corpus-decision --format json`
- `cargo xtask family verify-decision-contract --format json`

No CI, packaging, or install-path work is required.

### Locked Plan Decisions

These are frozen for M51:

1. `analysis_core/*` remains the only semantic owner surface.
2. `helper_surface.rs` and `decision_kernel.rs` may remain only as compatibility passthroughs. They are not allowed to regain ownership.
3. `family/mod.rs` must stop presenting the shim topology as peer semantic truth.
4. `xtask/src/lib.rs` must stop calling `decision_kernel::corpus_program_basis_snapshot(...)` as the proof-wall source of truth.
5. Command names, flags, JSON schemas, artifact locations, and stop-state outputs do not change.
6. Docs update only where they would otherwise lie about current ownership.
7. M51 is not a crate-extraction milestone.
8. M51 is not a second-language milestone.

### Abort and Re-scope Triggers

Stop and re-scope if any of these become necessary:

1. a public command name, flag, or JSON schema needs to change
2. the stop-state outputs change from `insufficient_real_corpus` / `not_recommended` / `stop`
3. the work requires moving path, write, or artifact policy into `analysis_core`
4. a non-test consumer outside the named write scope still depends on the shims as owners
5. the docs fix expands into repo-root roadmap or plan-authority rewrites

## Target End State

### Ownership Model

The post-M51 repo must tell one consistent story:

- `analysis_core/*` is the only semantic owner surface
- shims, if retained, are compatibility-only passthroughs
- local command consumers depend on the seam but do not redefine it
- tests and docs present the same ownership graph the code implements

### Semantic Owner Surfaces

These remain the only true owner files:

- `xtask/src/family/analysis_core/mod.rs`
- `xtask/src/family/analysis_core/helper_surface.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/analysis_core/proof_fingerprint.rs`

### Compatibility-Only Surfaces

These may exist only as compatibility shims:

- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`

### Local Command and Artifact Surfaces

These remain local command consumers, not shared-core owners:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/mod.rs`
- `xtask/src/family/paths.rs`
- `xtask/src/lib.rs`

### Dependency Graph

```text
analysis_core seam
  helper_surface.rs
  decision_contract.rs
  proof_fingerprint.rs
          │
          ├──────────────► recommend.rs
          │                command behavior stays unchanged
          │
          ├──────────────► verify.rs
          │                contract verifier stays unchanged
          │
          ├──────────────► promotion_artifacts.rs
          │                artifact validation stays unchanged
          │
          ├──────────────► xtask/src/lib.rs tests
          │                proof wall must consume seam directly
          │
          └──────────────► family/mod.rs
                           module topology must admit the seam honestly

compatibility only
  helper_surface.rs shim
  decision_kernel.rs shim
          │
          └──────────────► allowed only as passthroughs
                           not semantic owners
```

### Invariants

All of these must remain true after M51:

1. `recommend` still reports `insufficient_real_corpus`.
2. `corpus-decision` still reports `decision_action = "stop"` and `decision_basis_code = "no_actionable_candidate"`.
3. `verify-decision-contract` still passes all five checks.
4. `analysis_core` remains the sole owner of helper-surface classification, decision derivation, and proof-fingerprint normalization.
5. Any retained shim file is obviously compatibility-only in both code and docs.
6. No new codepath starts treating the shims as a second semantic home.

## Implementation Plan

### Write Scope

Primary write scope:

- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`
- `xtask/src/lib.rs`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` only if current wording becomes false

Protected read-only reference scope:

- `xtask/src/family/analysis_core/*`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/verify.rs`
- `xtask/src/family/promotion_artifacts.rs`

### Step 1: Freeze the ownership contract

Owner: topology lane

Touch:

- `xtask/src/family/mod.rs`
- `xtask/src/family/helper_surface.rs`
- `xtask/src/family/decision_kernel.rs`

Work:

1. freeze the wording and topology rule that `analysis_core/*` is the sole owner surface
2. decide the final `mod.rs` presentation:
   - `analysis_core` stays first-class
   - shims either remain exported with compatibility-only framing or stop being elevated as peer truth
3. keep shim behavior identical if they remain
4. do not add a new facade, adapter, or wrapper layer

Definition of done:

- there is one frozen ownership story in code
- the shims are clearly compatibility-only if retained
- no behavior changed yet

### Step 2: Rewire the command-facing proof wall

Owner: proof lane

Touch:

- `xtask/src/lib.rs`

Work:

1. remove the test-module import dependency on `decision_kernel`
2. retarget the two basis-snapshot proof-wall call sites to `family::analysis_core::corpus_program_basis_snapshot(...)`
3. update any nearby test prose, helper names, or comments that still imply shim ownership
4. keep all expected stop-state outputs unchanged

Definition of done:

- the proof wall consumes the seam directly
- tests still assert the same decision basis and stop-state outcomes
- no new helper layer is introduced

### Step 3: Update command-facing docs to match the code truth

Owner: docs lane

Touch:

- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` only if needed

Work:

1. replace statements that say `helper_surface.rs` or `decision_kernel.rs` own current semantics
2. rewrite that ownership story to point at `analysis_core/*`
3. preserve historical milestone context only where it is clearly labeled historical
4. do not widen this into a broad roadmap prose cleanup

Definition of done:

- maintainers reading the docs learn the same ownership graph the code implements
- no copy reads like the shims are current owners

### Step 4: Re-prove the frozen stop-state contract

Owner: integrated branch

Run the proof floor:

```bash
cargo test -p xtask
./.agents/skills/next-milestone/scripts/collect_signals.sh
cargo xtask family recommend --format json
cargo xtask family corpus-decision --format json
cargo xtask family verify-decision-contract --format json
```

Expected semantic outputs after M51:

- `recommendation_status = "insufficient_real_corpus"`
- `decision_status = "not_recommended"`
- `decision_action = "stop"`
- `decision_basis_code = "no_actionable_candidate"`
- `required_next_action = "record_stop_without_new_milestone"`
- `overall_verdict = "pass"`

Escalate only if parity fails:

```bash
cargo test -p xtask -- --nocapture
```

Definition of done:

- the stop-state truth is unchanged
- the command-facing proof wall is green
- the grep exit gate passes

## Code Quality Guardrails

- Keep the change explicit. This is an adoption milestone, not an abstraction milestone.
- Do not add a third wrapper layer around `analysis_core`.
- Do not duplicate ownership constants or wording across helper modules if one direct import is enough.
- Prefer changing the two real proof-wall call sites over inventing adapter helpers.
- If the shims remain, keep them one-hop passthroughs. No new behavior, no branching, no cached state.
- Update nearby prose or ASCII diagrams if they still imply shim ownership. Stale diagrams are lies.
- Do not widen the docs blast radius beyond the named command-facing ownership surfaces.

## Test Review

The goal is full coverage of every behaviorally meaningful path in the adoption cleanup. This milestone should look boring when it lands. If it changes semantics, it failed.

### Code Path Coverage

```text
CODE PATH COVERAGE
===========================
[+] Module topology
    │
    ├── family/mod.rs
    │   ├── [REGRESSION] analysis_core remains first-class
    │   └── [REGRESSION] shims no longer read as peer owner modules
    │
    ├── family/helper_surface.rs
    │   └── [REGRESSION] shim remains pure passthrough + compatibility-only
    │
    └── family/decision_kernel.rs
        └── [REGRESSION] shim remains pure passthrough + compatibility-only

[+] Command-facing proof wall
    │
    └── xtask/src/lib.rs tests
        ├── [REGRESSION] test imports stop pulling in decision_kernel as owner truth
        ├── [REGRESSION] basis snapshot assertions call analysis_core directly
        └── [REGRESSION] stop-state assertions remain unchanged

[+] Direct command consumers
    │
    ├── recommend.rs
    │   └── [REGRESSION] helper-surface durable hold semantics unchanged
    ├── verify.rs
    │   └── [REGRESSION] five contract checks stay green
    └── promotion_artifacts.rs
        └── [REGRESSION] artifact validation still matches the frozen stop-state tuple

[+] Maintainer narrative
    │
    ├── semantic_family_capability_corpus_guide_v0.1.md
    │   └── [REGRESSION] ownership text points at analysis_core
    ├── recommendation_corpus_expansion_program_v0.1.md
    │   └── [REGRESSION] ownership text points at analysis_core
    └── ai_promotion_and_multilanguage_milestones_v0.1.md
        └── [CONDITIONAL] update only if current wording becomes false

[+] Proof floor
    │
    ├── cargo test -p xtask
    ├── collect_signals.sh
    ├── family recommend --format json
    ├── family corpus-decision --format json
    └── family verify-decision-contract --format json
        └── [REGRESSION] stop-state outputs and pass/fail status unchanged

─────────────────────────────────
PLANNED COVERAGE: 14 critical paths
  module-topology paths: 4
  proof-wall paths: 3
  direct-consumer paths: 3
  maintainer-narrative paths: 3
  proof-floor parity paths: 1 integrated wall
CRITICAL REGRESSION TESTS: 7
─────────────────────────────────
```

### Maintainer Flow Coverage

```text
MAINTAINER FLOW COVERAGE
===========================
[+] Command operator loop
    ├── cargo xtask family recommend --format json
    ├── cargo xtask family corpus-decision --format json
    └── cargo xtask family verify-decision-contract --format json

[+] Proof owner loop
    ├── xtask tests resolve basis snapshot through analysis_core
    ├── shim modules remain compatibility-only
    └── direct consumers remain unchanged

[+] Documentation loop
    ├── maintainer reads seam ownership docs
    ├── docs describe analysis_core as the owner
    └── no doc teaches the old split as present-day truth
```

### Required Assertion Surfaces

| File | Required updates or assertions |
| --- | --- |
| `xtask/src/family/mod.rs` | present `analysis_core` as the maintained seam surface and stop elevating shim ownership as peer truth |
| `xtask/src/family/helper_surface.rs` | remain a pure passthrough or be removed from first-class presentation; no behavior added |
| `xtask/src/family/decision_kernel.rs` | remain a pure passthrough or be removed from first-class presentation; no behavior added |
| `xtask/src/lib.rs` | replace shim-based basis-snapshot calls and imports with `analysis_core` ownership |
| `xtask/src/family/recommend.rs` | unchanged behavior, still passes proof via `cargo test -p xtask` |
| `xtask/src/family/verify.rs` | unchanged `verify-decision-contract` check behavior |
| `xtask/src/family/promotion_artifacts.rs` | unchanged artifact validation against the frozen stop-state tuple |
| `docs/semantic_family_capability_corpus_guide_v0.1.md` | update current-ownership prose |
| `docs/recommendation_corpus_expansion_program_v0.1.md` | update current-ownership prose |
| `docs/ai_promotion_and_multilanguage_milestones_v0.1.md` | update only if its current ownership language becomes false after code cleanup |

### Test Plan Artifact

The QA-oriented artifact for this plan lives at:

- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-213500.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-213500.md)

That file is the short `/qa` input. This plan is the exhaustive implementation version.

### Grep Exit Gate

M51 is done only when all of these are true:

1. command-facing docs no longer claim `xtask/src/family/helper_surface.rs` or `xtask/src/family/decision_kernel.rs` own current semantics
2. `xtask/src/lib.rs` no longer imports or calls `decision_kernel::corpus_program_basis_snapshot(...)`
3. any remaining mention of `helper_surface.rs` or `decision_kernel.rs` as live code surfaces is explicitly labeled compatibility-only or historical

Use one final targeted audit, not a permanent new code path:

```bash
rg -n "decision_kernel|helper_surface" xtask/src/lib.rs xtask/src/family docs/semantic_family_capability_corpus_guide_v0.1.md docs/recommendation_corpus_expansion_program_v0.1.md docs/ai_promotion_and_multilanguage_milestones_v0.1.md
```

## Failure Modes

| Codepath | Real failure | Test coverage required | Error handling today | User-visible impact | Critical gap if missed |
| --- | --- | --- | --- | --- | --- |
| proof-wall rewire | `xtask/src/lib.rs` still calls `decision_kernel` so the repo keeps teaching the old owner path | yes | compile/test only | maintainers see split ownership truth | yes |
| fake shim cleanup | shim export is removed or narrowed and a hidden internal consumer breaks | yes | compile failure | adoption slice looks bigger and noisier than intended | yes |
| topology lie | `family/mod.rs` still exports the old shape as peer owner truth | yes | none | future maintainers keep importing the wrong surface | yes |
| semantic drift | stop-state outputs change during cleanup | yes | verifier catches only after the fact | roadmap and operator decisions become untrustworthy | yes |
| docs lie | command-facing docs still claim the shims own semantics | yes | none | copy-paste reasoning is wrong even if code is green | yes |
| scope creep | path/write/reporting logic starts moving into `analysis_core` | yes | none | portability slice mutates into pseudo-extraction work | yes |
| milestone theater | docs-only or import-only cleanup lands without proof-floor parity | yes | none | branch claims progress without closing the real adoption gap | yes |

## Performance Review

No meaningful runtime performance work is justified here. This milestone is ownership cleanup plus proof.

Guardrails:

1. Do not add new scans or normalizers to enforce adoption. One final grep audit is enough.
2. Do not widen `cargo test -p xtask` fallout by moving modules unnecessarily.
3. Do not introduce a new facade just to avoid touching two proof-wall call sites.
4. Keep compile churn local to `xtask` and the three named docs.

## Worktree Parallelization Strategy

This plan has limited but real parallelization after the ownership contract is frozen.

The mistake would be starting docs or proof-wall work before the code truth is settled. Once the contract is frozen in `xtask/src/family/`, the remaining work splits cleanly enough to justify parallel lanes:

- one code lane for the proof wall in `xtask/src/lib.rs`
- one docs lane for the command-facing ownership narrative in `docs/`

The integrated parity run stays sequential and last.

### Dependency Table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Freeze ownership contract and shim presentation | `xtask/src/family/` | — |
| B. Rewire proof-wall imports and basis-snapshot call sites | `xtask/src/lib.rs` | A |
| C. Rewrite command-facing ownership docs | `docs/` | A |
| D. Re-run proof floor and grep exit gate | `xtask/`, `docs/`, `.agents/skills/next-milestone/scripts/` | B, C |

### Parallel Lanes

- `Lane A`: Step A
- `Lane B`: Step B (starts after A)
- `Lane C`: Step C (starts after A)
- `Lane D`: Step D (starts after B and C merge)

In plain English:

- freeze the code contract first
- then run proof-wall cleanup and docs cleanup in parallel worktrees
- then merge both lanes and run the acceptance wall once

### Execution Order

1. Land Step A first. This is the contract freeze and the only real blocker.
2. Launch Step B and Step C in parallel worktrees after Step A is merged or otherwise fixed as the shared base.
3. Merge the proof-wall lane and the docs lane back into one branch.
4. Run Step D on the integrated branch only.

### Conflict Flags

- `Lane B` and `Lane C` are safe only if Step A already froze whether the shims remain exported and how they are described.
- If Step C starts before Step A is settled, the docs lane will guess at wording and likely drift.
- If Step B discovers a hidden non-test shim consumer, stop the split and collapse back to one owner lane. That means the blast radius was understated.
- Do not split `xtask/src/lib.rs` and `xtask/src/family/` in parallel before Step A. They both participate in the ownership contract.

## NOT in Scope

These were considered and are explicitly deferred:

- new family promotion or corpus spend
- new decision basis codes
- generalized multi-wedge decision framework
- cross-crate extraction of `analysis_core`
- JSON schema or artifact path changes
- CLI command or flag changes
- second-language execution expansion
- repo-root roadmap or authority-plan rewrites outside this file
- broad docs gardening unrelated to current seam ownership truth

## Implementation Checklist

1. Freeze the M51 ownership contract in code comments and module presentation.
2. Remove the `decision_kernel` test import from `xtask/src/lib.rs`.
3. Retarget both basis-snapshot proof-wall call sites to `family::analysis_core::corpus_program_basis_snapshot(...)`.
4. Keep `recommend.rs`, `verify.rs`, and `promotion_artifacts.rs` behavior unchanged.
5. Decide whether `family/mod.rs` keeps shim exports, and if it does, mark them compatibility-only.
6. Keep `helper_surface.rs` as a pure passthrough only.
7. Keep `decision_kernel.rs` as a pure passthrough only.
8. Update command-facing docs that still claim shim ownership.
9. Update `ai_promotion_and_multilanguage_milestones_v0.1.md` only if its current wording becomes false.
10. Run `cargo test -p xtask`.
11. Run `collect_signals.sh`, `family recommend --format json`, `family corpus-decision --format json`, and `family verify-decision-contract --format json`.
12. Run the grep exit gate and classify any remaining shim mention as compatibility-only, historical, or a bug.

## Completion Summary

- Step 0: Scope Challenge, combined code + doc + proof slice accepted as the minimum honest cut
- Architecture Review: 1 seam, 2 shims, 1 proof wall, 1 integrated proof floor
- Code Quality Review: explicit-over-clever guardrails written, no new abstraction authorized
- Test Review: coverage diagram produced, 14 critical paths identified
- Performance Review: 0 runtime findings, 4 compile-churn guardrails
- NOT in scope: written
- What already exists: written
- Failure modes: 7 critical gaps flagged
- Parallelization: 4 steps, 2 post-freeze parallel lanes, 1 final sequential acceptance lane
- Lake Score: 6/6 major recommendations chose the complete option over the shortcut

## Recommended Next Action

Start with [xtask/src/family/mod.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/mod.rs), [xtask/src/family/helper_surface.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/helper_surface.rs), and [xtask/src/family/decision_kernel.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/decision_kernel.rs).

That is the contract freeze.

After that, split the work:

- proof wall in [xtask/src/lib.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs)
- docs in [docs/semantic_family_capability_corpus_guide_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/semantic_family_capability_corpus_guide_v0.1.md) and [docs/recommendation_corpus_expansion_program_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/recommendation_corpus_expansion_program_v0.1.md)

If the proof wall still calls `decision_kernel` or the module graph still presents the shims as peers, the repo has not actually adopted the seam. Once that code truth is frozen, the rest is straightforward and bounded.
