# M26 Orchestration Plan

## Summary

- Execute from the current branch `feat/m26`, because that is the live checked-out baseline in this workspace.
- Keep the critical path local to the parent agent for the two serialized `xtask` phases, both human approval gates, and final integration. Use subagents only for the three disjoint post-approval lanes.
- Use dedicated worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m26/{contract,packet,runtime,cli,int}` with workstream branches `ws/m26-contract`, `ws/m26-packet`, `ws/m26-runtime`, `ws/m26-cli`, and `ws/m26-int`.
- Use GPT-5.4 with `reasoning_effort=high` for all workers. Cap concurrency at 3 workers. The parent agent remains the only integrator.
- Keep orchestration state in one canonical location owned by the parent agent:
  - `PRIMARY_ROOT=/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
  - `M26_RUN_ROOT=$PRIMARY_ROOT/.runs/m26`
  - `M26_ARTIFACT_ROOT=$PRIMARY_ROOT/.semantic-family-artifacts/family-promotion`
  - queue: `$M26_RUN_ROOT/tasks.json`
  - session log: `$M26_RUN_ROOT/session-log.md`
  - Gate 1 basis record: `$M26_RUN_ROOT/gate1-basis.json`
  - Lane A freeze record: `$M26_RUN_ROOT/contract-freeze.json`
  - per-task sentinels: `$PRIMARY_ROOT/.runs/<TASK_ID>/`
- Treat `$M26_RUN_ROOT/*` and `$M26_ARTIFACT_ROOT/*` as run artifacts and derived output surfaces, not authored source and not assumed git-tracked deliverables.
- Worker worktrees do not become independent sources of truth for approvals or artifacts. They return code changes and narrow summaries only. The parent writes all orchestration artifacts back to `PRIMARY_ROOT`.

## Hard Guards

- Workspace boundary is locked for the entire run:
  - no new workspace member
  - no `spec-orchestrator`
  - no fourth crate in `Cargo.toml`
- `cargo xtask family inventory --format json` is a pure projection only.
  - It must not rank candidates.
  - It must not embed recommendation policy.
  - It must not contain approval logic.
- Inventory capture is byte-stable:
  - snapshot bytes are the verbatim UTF-8 stdout bytes from `cargo xtask family inventory --format json`
  - the captured snapshot includes the command's single trailing newline
  - `inventory_sha256` hashes those exact bytes, not reparsed or pretty-printed JSON
- Approval state begins only after the AI writes `recommendation.latest.json` and captures the exact inventory snapshot used for that recommendation under:
  - `$M26_ARTIFACT_ROOT/inventory/<run-id>.json`
- The Gate 1 recommendation artifact must encode:
  - `inventory_path`
  - `inventory_sha256`
- Gate 1 is a pre-edit approval over repo truth before approved-family edits begin.
- Gate 1 approval remains valid only while a fresh inventory snapshot of the unchanged pre-edit basis yields both:
  - the same `inventory_sha256`
  - the same `ranked_candidates[0].family`
- If either pre-edit Gate 1 basis check changes:
  - halt immediately
  - write a fresh `recommendation.latest.json`
  - require new human approval
  - do not begin wrapper-family edits under the old approval
- After the first approved-family edit lands, Gate 1 is no longer compared against live post-edit inventory.
  - from that point on, correctness is governed by hard gates plus the post-promotion inventory expectations
- All AI-written promotion artifacts must be validated via one deterministic, path-aware command:
  - `cargo xtask family validate-artifact <path>`
- The parent agent uses that command as the runtime validation gate for:
  - `$M26_ARTIFACT_ROOT/recommendation.latest.json` before Gate 1 approval use
  - `$M26_ARTIFACT_ROOT/function.wrapper.pipeline.v1/<run-id>/blocker.report.json` on blocked termination
  - `$M26_ARTIFACT_ROOT/function.wrapper.pipeline.v1/<run-id>/promotion.execution.json` before Gate 2 approval use
- `cargo test -p xtask artifact_schema_ -- --color never` remains development coverage for schema and unit behavior, not the runtime approval gate.

## Workstream Plan

### WS-CONTRACT (`ws/m26-contract`) — parent agent only, sequential

1. `task/m26-a1-inventory-artifacts`
- Own `xtask/src/lib.rs`, `xtask/src/family/mod.rs`, new `xtask/src/family/inventory.rs`, new `xtask/src/family/promotion_artifacts.rs`, and `xtask/Cargo.toml` only if `spec-core` must become a normal dependency.
- Add `cargo xtask family inventory --format json`.
- Add typed schemas and validation support for:
  - `$M26_ARTIFACT_ROOT/recommendation.latest.json`
  - `$M26_ARTIFACT_ROOT/function.wrapper.pipeline.v1/<run-id>/promotion.execution.json`
  - `$M26_ARTIFACT_ROOT/function.wrapper.pipeline.v1/<run-id>/blocker.report.json`
- Add the stable path-aware parent validation path:
  - `cargo xtask family validate-artifact <path>`
- Keep `cargo test -p xtask artifact_schema_ -- --color never` as development coverage for schema and unit behavior.
- Add lock tests for inventory shape, ordering, exit behavior, byte-stable snapshot hashing, blocker vocabulary, schema round-trips, recommendation `inventory_path` and `inventory_sha256`, and proof-artifact path validation.
- Verify with:
  - `cargo test -p xtask`
  - `cargo test -p xtask artifact_schema_ -- --color never`
  - `cargo xtask family inventory --format json`

Acceptance for `task/m26-a1-inventory-artifacts`:
- inventory emits repo truth only:
  - promoted families
  - runtime supported routes
  - supported-but-unpromoted families
  - canonical seed paths
  - supporting packet paths
  - routing predecessor and successors
- inventory remains stdout-only, deterministic, and side-effect-free
- recommendation ranking remains outside `xtask` and exists only in AI-written `recommendation.latest.json`
- `cargo xtask family validate-artifact <path>` exists and exits nonzero on schema or content failure for any supported promotion artifact path

2. Approval Gate 1
- Write the exact inventory snapshot used for this run under:
  - `$M26_ARTIFACT_ROOT/inventory/<run-id>.json`
- Write `$M26_ARTIFACT_ROOT/recommendation.latest.json` as a run artifact.
- Write `$M26_RUN_ROOT/gate1-basis.json` with:
  - `run_id`
  - `basis_commit`
  - `inventory_path`
  - `inventory_sha256`
  - `approved_family`
- Ensure the recommendation artifact references the retained inventory snapshot through:
  - `inventory_path`
  - `inventory_sha256`
- Run:
  - `cargo xtask family validate-artifact $M26_ARTIFACT_ROOT/recommendation.latest.json`
- Stop for human approval of `ranked_candidates[0].family`.
- Preserve the approved `recommendation.latest.json`, `gate1-basis.json`, and retained inventory snapshot as the Gate 1 audit basis.
- Immediately before the first approved-family edit, rerun inventory on the unchanged pre-edit basis commit and confirm it still yields:
  - the same `inventory_sha256`
  - the same `ranked_candidates[0].family`
- If either check fails, halt and require a new Gate 1 approval before any wrapper-family edits proceed.

3. `task/m26-a2-wrapper-harness-scaffold`
- Own `xtask/src/family/harness.rs`, `xtask/src/family/scaffold.rs`, and related `xtask/src/lib.rs` tests.
- Register `function.wrapper.pipeline.v1` with locked `precedence = 2`, `must_not_shadow`, `suite_slug = "wrapper_pipeline_"`, and prove and certify suite ownership.
- Add `StarterTemplate::WrapperPipelineTwoStep`.
- Lock the 12 starter paths and full per-bucket starter semantics.
- Lock the wrapper-family contract before any parallel lane starts:
  - suite slug
  - packet file names and starter paths
  - per-bucket starter semantics
  - unsupported-near-miss boundary
- Verify with:
  - `cargo test -p xtask`
  - `cargo xtask family smoke function.wrapper.pipeline.v1`
- Merge Lane A onto `feat/m26`, then write `$M26_RUN_ROOT/contract-freeze.json` with:
  - `contract_freeze_commit`
  - `approved_family`
  - `suite_slug`
  - `frozen_contract_summary`

Acceptance for `task/m26-a2-wrapper-harness-scaffold`:
- `StarterTemplate::WrapperPipelineTwoStep` emits the locked semantics for all 12 starter units:
  - aligned: discount then tax
  - drift: reversed pipeline
  - under_specified: aligned body with weakened authored semantic surface
  - unsupported_near_miss: semantically close but outside the honest subset via the locked non-parameter threaded tax arg
- smoke contracts prove scaffold honesty for paths and content, not placeholders or generic half-truths

### Parallel workers after WS-CONTRACT is green, Gate 1 approval is granted, and the wrapper-family contract is frozen

All worker branches must fork from the exact `contract_freeze_commit` recorded in `$M26_RUN_ROOT/contract-freeze.json`. No worker may branch from an earlier `feat/m26` HEAD.

4. `task/m26-b-packet` on `ws/m26-packet` — worker 1
- Own only `semantic-families/function.wrapper.pipeline.v1/**`.
- Consume the frozen WS-CONTRACT wrapper-family contract literally.
- Seed from `function.wrapper.pipeline.chain3.v1`, but lift only bucket shells, packet-local leaf specs, aligned wrapper spec, manifest and candidate skeleton, and rewrite the three non-aligned wrapper cases.
- Acceptance:
  - exactly three family units per bucket
  - no `checkout_chain3_*` units in the dedicated wrapper packet
  - packet remains self-contained
  - manifest routing reflects the dedicated wrapper family, not chain3
  - candidate text reflects the dedicated wrapper family, not lazily reused chain3 language

5. `task/m26-c-runtime` on `ws/m26-runtime` — worker 2
- Own only `spec-core/src/semantic_review.rs`.
- Add `wrapper_pipeline_classifier_*` tests plus any route-order assertions that truly belong in semantic-review unit tests.
- Preserve the existing route order `chain3 -> wrapper -> monotone_down -> monotone_up`.
- Do not own or add `wrapper_pipeline_regression_*`.
- Do not change packet files or CLI tests.
- Do not redefine wrapper-family semantics, starter paths, or the unsupported-near-miss boundary. This lane proves the frozen contract, it does not rewrite it.

6. `task/m26-d-cli` on `ws/m26-cli` — worker 3
- Own only `spec-cli/tests/cli.rs` and `spec-cli/tests/m14_regressions.rs`.
- Own:
  - `wrapper_pipeline_truth_surface_*`
  - `wrapper_pipeline_corpus_*`
  - `wrapper_pipeline_regression_*`
- Consume the frozen WS-CONTRACT wrapper-family contract literally.
- Explicit required unsupported-near-miss coverage:
  - wrapper unsupported-near-miss stays unsupported
  - status remains health-neutral where `PLAN.md` requires it
  - export, passport, and read-side surfaces preserve the seeded unsupported review truth
  - additive-only unsupported behavior remains honest across `spec test`, `spec status`, `spec build`, and `spec export`
- Use the existing family-B and M21 naming patterns.
- Do not change `xtask`, `spec-core`, or packet files.

### WS-INT (`ws/m26-int`) — parent agent only

7. `task/m26-e-integrate`
- Merge the packet, runtime, and CLI workstreams into the integration worktree.
- Resolve only straightforward merge mechanics in integration-owned surfaces.
- If packet shape, route order, starter contract, or unsupported-near-miss boundary disagree across lanes, integration does not resolve creatively.
  - Bounce the conflict back to the owning lane, or
  - apply the already-locked `PLAN.md` contract literally
- Before hard gates:
  - verify the preserved Gate 1 basis still reproduces from the recorded `basis_commit`, not from the current merged tree
  - verify the current integration branch descends from the recorded `contract_freeze_commit`
  - do not require the current merged repo state to match the original Gate 1 inventory snapshot, because approved-family edits are expected to change inventory-visible truth
- After merging lanes but before hard gates:
  - preserve the approved `recommendation.latest.json`
  - preserve `gate1-basis.json`
  - preserve `contract-freeze.json`
  - regenerate execution and blocker artifact payloads from the current merged repo state only
  - do not carry forward stale pre-merge lane-local execution or blocker artifact contents
- Run the full loop:
  - `cargo fmt --all`
  - `cargo test -p xtask`
  - `cargo test -p xtask artifact_schema_ -- --color never`
  - targeted fast tests:
    - `cargo test -p spec-core --lib wrapper_pipeline_`
    - `cargo test -p spec-cli --test cli wrapper_pipeline_`
    - `cargo test -p spec-cli --test m14_regressions wrapper_pipeline_`
  - hard gates:
    - `cargo xtask family smoke function.wrapper.pipeline.v1`
    - `cargo xtask family prove function.wrapper.pipeline.v1`
    - `cargo xtask family certify function.wrapper.pipeline.v1`
- If gates stay red:
  - write `$M26_ARTIFACT_ROOT/function.wrapper.pipeline.v1/<run-id>/blocker.report.json` as a run artifact
  - run:
    - `cargo xtask family validate-artifact $M26_ARTIFACT_ROOT/function.wrapper.pipeline.v1/<run-id>/blocker.report.json`
  - stop
- If gates go green:
  - rerun `cargo xtask family inventory --format json`
  - verify `function.wrapper.pipeline.v1` no longer appears in `supported_unpromoted_families[]`
  - verify it now appears in `promoted_families[]`
  - update `semantic-families/README.md`
  - write `$M26_ARTIFACT_ROOT/function.wrapper.pipeline.v1/<run-id>/promotion.execution.json` as a run artifact

8. Approval Gate 2
- Run:
  - `cargo xtask family validate-artifact $M26_ARTIFACT_ROOT/function.wrapper.pipeline.v1/<run-id>/promotion.execution.json`
- Confirm the execution report references real proof-artifact paths before proceeding.
- Gate 2 cannot proceed if `promotion.execution.json` fails path-aware validation or lacks real proof-artifact paths.
- Stop after writing the validated execution report.
- Human approves or rejects final output from that report.
- No hidden retries after final-output rejection without an explicit new run.

## Context-Control Rules

- Parent agent keeps only four live artifacts in working context:
  - `PLAN.md`
  - `$M26_RUN_ROOT/tasks.json`
  - the acceptance checklist
  - the latest integration diff summary
- Each worker prompt contains only:
  - its owned file set
  - the exact relevant `PLAN.md` excerpt
  - required commands
  - forbidden touch surfaces
  - the recorded `contract_freeze_commit`
- Each worker must return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- Workers do not write `$M26_RUN_ROOT/*` or `$M26_ARTIFACT_ROOT/*`.
- The parent agent reviews summaries plus narrow diffs only. It does not ingest full worker transcripts into the main context.
- Close each worker immediately after merge.
- Use completion sentinels or long waits, not tight polling.

## Tests And Acceptance

- Inventory
  - `cargo xtask family inventory --format json` is stdout-only, deterministic, side-effect-free, and shows `function.wrapper.pipeline.v1` as the supported-unpromoted family before promotion.
  - inventory contains no ranking, approval, or recommendation policy.
  - the retained Gate 1 inventory snapshot is the approval basis referenced by `inventory_path` and `inventory_sha256`.
  - Gate 1 basis equality is checked only on the unchanged pre-edit basis, not on the post-promotion merged tree.
  - after green promotion, rerunning inventory shows `function.wrapper.pipeline.v1` promoted and no longer supported-unpromoted.
- `xtask` contract
  - wrapper harness registration, routing order, suite slug ownership, starter paths, per-bucket starter semantics, and the unsupported-near-miss boundary are all covered by `xtask` tests.
  - `cargo xtask family validate-artifact <path>` is the stable runtime approval-artifact gate.
  - `cargo test -p xtask artifact_schema_ -- --color never` remains development coverage only.
- Packet
  - committed packet exists at `semantic-families/function.wrapper.pipeline.v1/`
  - exactly three dedicated wrapper-family units exist per bucket
  - no `checkout_chain3_*` units remain in the dedicated wrapper packet
  - packet is self-contained
  - manifest routing and candidate text are wrapper-family-specific
- Runtime and CLI
  - `spec-core/src/semantic_review.rs` covers `wrapper_pipeline_classifier_*` and runtime route-order assertions that belong there.
  - `spec-cli/tests/cli.rs` and `spec-cli/tests/m14_regressions.rs` cover `wrapper_pipeline_truth_surface_*`, `wrapper_pipeline_corpus_*`, and `wrapper_pipeline_regression_*`.
  - unsupported-near-miss remains additive-only and read-side-honest where the plan requires it.
  - chain3 and both arithmetic leaves remain green and unshadowed.
- Operator flow
  - recommendation artifact exists, validates through the path-aware validator, and is approved before wrapper-family edits continue.
  - Gate 1 approval is reopened automatically if the unchanged pre-edit basis changes `inventory_sha256` or `ranked_candidates[0].family`.
  - `gate1-basis.json` records the only commit on which Gate 1 inventory equality may be rechecked.
  - `contract-freeze.json` records the exact commit from which packet, runtime, and CLI workers are forked.
  - green path writes a path-valid `promotion.execution.json` with real proof-artifact paths.
  - blocked path writes a path-valid `blocker.report.json` with locked machine-evidence shape.
  - approved `recommendation.latest.json` and its retained Gate 1 inventory snapshot are preserved as audit trail unless pre-edit inventory truth changes enough to force reopening Gate 1.
- Workspace boundary
  - `Cargo.toml` still lists only `spec-core`, `spec-cli`, and `xtask`.

## Assumptions

- Worktree naming follows the repo’s existing `spec-m24/*` pattern.
- `spec-core` may become a normal `xtask` dependency if inventory reuses runtime route truth directly.
- The human approval steps in M26 are mandatory and are the only intentional pauses in the execution flow.
- `.semantic-family-artifacts/*` remains a derived run-artifact surface throughout M26, not a checked-in deliverable requirement.
