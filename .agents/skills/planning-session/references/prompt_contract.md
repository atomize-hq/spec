# Prompt Contract

Use these prompts exactly as written below.

Allowed substitutions:
- `<PRIOR_IMPLEMENTATION_LAST_MESSAGE>`
- `<NEXT_MILESTONE_NUMBER>` where the wrapper computes `previous milestone + 1`
- `<NEXT_MILESTONE_MEMO>`
- `<DESIGN_DOC_PATH>`

Do not add wrapper commentary or extra instructions inside the stage prompts.

## Stage 1: Next Milestone

```text
Previous session last message:

<PRIOR_IMPLEMENTATION_LAST_MESSAGE>

Now we need to determine what the [$next-milestone](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md) should be for M<NEXT_MILESTONE_NUMBER>
```

## Stage 2: Design Doc

```text
[$autoplan](/Users/spensermcconnell/gstack/.agents/skills/gstack-autoplan/SKILL.md) a new design document from the recommended next milestone from the below message for M<NEXT_MILESTONE_NUMBER>:

<NEXT_MILESTONE_MEMO>
```

## Stage 3: Fresh PLAN

```text
[$autoplan](/Users/spensermcconnell/gstack/.agents/skills/gstack-autoplan/SKILL.md)  a NEW/FRESH @PLAN.md based on this design docs:

<DESIGN_DOC_PATH>

   -- the plan should be  unified/solidified so it reads as a single cohesive/implementable plan with zero ambiguity, and all of the normal generated structure and rigor frrom the [$plan-eng-review](/Users/spensermcconnell/gstack/.agents/skills/gstack-plan-eng-review/SKILL.md) -- and it should include the parallelization section
```

## Stage 4: PLAN Solidification Pass

```text
[$autoplan](/Users/spensermcconnell/gstack/.agents/skills/gstack-autoplan/SKILL.md) PLAN.md needs to go through one more pass to unify/solidify the details in  PLAN.md so it reads as a single cohesive/implementable plan with zero ambiguity, and all of the normal generated structure and rigor frrom the [$plan-eng-review](/Users/spensermcconnell/gstack/.agents/skills/gstack-plan-eng-review/SKILL.md) -- and it should include the parallelization section

Related Documents:
<DESIGN_DOC_PATH>
```

## Stage 5: ORCH Plan

```text
I need you to use the below example of an Orchestration Plan, the example below is the correct level of structure, rigor, explicit orchestration and completeness, and use it as a guide to generate NEW/FRESH ORCH_PLAN.md with the same level of detail and parallel subagent workstream optimization to ensure it successfully kicksoff and walks the entire PLAN.md session to its completion.

To create the new ORCH_PLAN.md you need to spin up a fresh GPT-5.4 subagent  on high to draft the orchestration plan, you will review it for completness, and if it is not sufficient, you will send it back to the same subagent with details on what needs to be changed/corrected -- you are responsible for providing the subagent with the seed context that will enable it to be successfull



# M26 Orchestration Plan

## Summary

- Execute from the current branch `codex/m23-contract`, because that is the live checked-out baseline in this workspace.
- Keep the critical path local to the parent agent for the two serialized `xtask` phases, both human approval gates, and final integration. Use subagents only for the three disjoint post-approval lanes.
- Use dedicated worktrees under `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m26/{contract,packet,runtime,cli,int}` with workstream branches `ws/m26-contract`, `ws/m26-packet`, `ws/m26-runtime`, `ws/m26-cli`, and `ws/m26-int`.
- Use GPT-5.4 with `reasoning_effort=high` for all workers. Cap concurrency at 3 workers. The parent agent remains the only integrator.
- Keep orchestration state in one local source of truth:
  - queue: `.runs/m26/tasks.json`
  - session log: `.runs/m26/session-log.md`
  - per-task sentinels: `.runs/<TASK_ID>/`
- Treat `.runs/m26/*` and `.semantic-family-artifacts/*` as run artifacts and derived output surfaces, not authored source and not assumed git-tracked deliverables.

## Hard Guards

- Workspace boundary is locked for the entire run:
  - no new workspace member
  - no `spec-orchestrator`
  - no fourth crate in `Cargo.toml`
- `cargo xtask family inventory --format json` is a pure projection only.
  - It must not rank candidates.
  - It must not embed recommendation policy.
  - It must not contain approval logic.
- Approval state begins only after the AI writes `recommendation.latest.json`.
- If inventory truth changes such that `function.wrapper.pipeline.v1` is no longer the top honest candidate:
  - halt immediately
  - write a fresh `recommendation.latest.json`
  - require new human approval
  - do not begin or continue wrapper-family edits under the old approval
- All AI-written promotion artifacts must be validated via one deterministic, path-aware command:
  - `cargo xtask family validate-artifact <path>`
- The parent agent uses that command as the runtime validation gate for:
  - `.semantic-family-artifacts/family-promotion/recommendation.latest.json` before Gate 1 approval use
  - `.semantic-family-artifacts/family-promotion/<family>/<run-id>/blocker.report.json` on blocked termination
  - `.semantic-family-artifacts/family-promotion/<family>/<run-id>/promotion.execution.json` before Gate 2 approval use
- `cargo test -p xtask artifact_schema_ -- --color never` remains development coverage for schema/unit behavior, not the runtime approval gate.

## Workstream Plan

### WS-CONTRACT (`ws/m26-contract`) — parent agent only, sequential

1. `task/m26-a1-inventory-artifacts`
- Own `xtask/src/lib.rs`, `xtask/src/family/mod.rs`, new `xtask/src/family/inventory.rs`, new `xtask/src/family/promotion_artifacts.rs`, and `xtask/Cargo.toml` only if `spec-core` must become a normal dependency.
- Add `cargo xtask family inventory --format json`.
- Add typed schemas and validation support for:
  - `.semantic-family-artifacts/family-promotion/recommendation.latest.json`
  - `.semantic-family-artifacts/family-promotion/<family>/<run-id>/promotion.execution.json`
  - `.semantic-family-artifacts/family-promotion/<family>/<run-id>/blocker.report.json`
- Add the stable path-aware parent validation path:
  - `cargo xtask family validate-artifact <path>`
- Keep `cargo test -p xtask artifact_schema_ -- --color never` as development coverage for schema/unit behavior.
- Add lock tests for inventory shape, ordering, exit behavior, blocker vocabulary, schema round-trips, and proof-artifact path validation.
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
  - routing predecessor/successors
- inventory remains stdout-only, deterministic, and side-effect-free
- recommendation ranking remains outside `xtask` and exists only in AI-written `recommendation.latest.json`
- `cargo xtask family validate-artifact <path>` exists and exits nonzero on schema or content failure for any supported promotion artifact path

2. Approval Gate 1
- Write the inventory snapshot used for this run under `.semantic-family-artifacts/family-promotion/` as a run artifact.
- Write `.semantic-family-artifacts/family-promotion/recommendation.latest.json` as a run artifact.
- Run:
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/recommendation.latest.json`
- Stop for human approval of `ranked_candidates[0].family`.
- Preserve the approved `recommendation.latest.json` as the Gate 1 audit artifact.
- Do not regenerate `recommendation.latest.json` after approval unless inventory truth changed enough to invalidate the old approval, in which case halt and require a new Gate 1 approval before any wrapper-family edits proceed.

3. `task/m26-a2-wrapper-harness-scaffold`
- Own `xtask/src/family/harness.rs`, `xtask/src/family/scaffold.rs`, and related `xtask/src/lib.rs` tests.
- Register `function.wrapper.pipeline.v1` with locked `precedence = 2`, `must_not_shadow`, `suite_slug = "wrapper_pipeline_"`, and prove/certify suite ownership.
- Add `StarterTemplate::WrapperPipelineTwoStep`.
- Lock the 12 starter paths and full per-bucket starter semantics.
- Verify with:
  - `cargo test -p xtask`
  - `cargo xtask family smoke function.wrapper.pipeline.v1`

Acceptance for `task/m26-a2-wrapper-harness-scaffold`:
- `StarterTemplate::WrapperPipelineTwoStep` emits the locked semantics for all 12 starter units:
  - aligned: discount then tax
  - drift: reversed pipeline
  - under_specified: aligned body with weakened authored semantic surface
  - unsupported_near_miss: semantically close but outside the honest subset via the locked non-parameter threaded tax arg
- smoke contracts prove scaffold honesty for paths and content, not placeholders or generic half-truths

### Parallel workers after WS-CONTRACT is green and Gate 1 approval is granted

4. `task/m26-b-packet` on `ws/m26-packet` — worker 1
- Own only `semantic-families/function.wrapper.pipeline.v1/**`.
- Seed from `function.wrapper.pipeline.chain3.v1`, but lift only bucket shells, packet-local leaf specs, aligned wrapper spec, manifest/candidate skeleton, and rewrite the three non-aligned wrapper cases.
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

6. `task/m26-d-cli` on `ws/m26-cli` — worker 3
- Own only `spec-cli/tests/cli.rs` and `spec-cli/tests/m14_regressions.rs`.
- Own:
  - `wrapper_pipeline_truth_surface_*`
  - `wrapper_pipeline_corpus_*`
  - `wrapper_pipeline_regression_*`
- Explicit required unsupported-near-miss coverage:
  - wrapper unsupported-near-miss stays unsupported
  - status remains health-neutral where `PLAN.md` requires it
  - export/passport/read-side surfaces preserve the seeded unsupported review truth
  - additive-only unsupported behavior remains honest across `spec test`, `spec status`, `spec build`, and `spec export`
- Use the existing family-B and M21 naming patterns.
- Do not change `xtask`, `spec-core`, or packet files.

### WS-INT (`ws/m26-int`) — parent agent only

7. `task/m26-e-integrate`
- Merge the packet, runtime, and CLI workstreams into the integration worktree.
- Resolve only straightforward merge mechanics in integration-owned surfaces.
- If packet shape, route order, or starter contract disagree across lanes, integration does not resolve creatively.
  - Bounce the conflict back to the owning lane, or
  - apply the already-locked `PLAN.md` contract literally
- After merging lanes but before hard gates:
  - preserve the approved `recommendation.latest.json`
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
  - write `.semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/<run-id>/blocker.report.json` as a run artifact
  - run:
    - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/<run-id>/blocker.report.json`
  - stop
- If gates go green:
  - rerun `cargo xtask family inventory --format json`
  - verify `function.wrapper.pipeline.v1` no longer appears in `supported_unpromoted_families[]`
  - verify it now appears in `promoted_families[]`
  - update `semantic-families/README.md`
  - write `.semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/<run-id>/promotion.execution.json` as a run artifact

8. Approval Gate 2
- Run:
  - `cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/<run-id>/promotion.execution.json`
- Confirm the execution report references real proof-artifact paths before proceeding.
- Gate 2 cannot proceed if `promotion.execution.json` fails path-aware validation or lacks real proof-artifact paths.
- Stop after writing the validated execution report.
- Human approves or rejects final output from that report.
- No hidden retries after final-output rejection without an explicit new run.

## Context-Control Rules

- Parent agent keeps only four live artifacts in working context:
  - `PLAN.md`
  - `.runs/m26/tasks.json`
  - the acceptance checklist
  - the latest integration diff summary
- Each worker prompt contains only:
  - its owned file set
  - the exact relevant `PLAN.md` excerpt
  - required commands
  - forbidden touch surfaces
- Each worker must return only:
  - changed files
  - commands run and exit codes
  - blockers or unresolved assumptions
- The parent agent reviews summaries plus narrow diffs only. It does not ingest full worker transcripts into the main context.
- Close each worker immediately after merge.
- Use completion sentinels or long waits, not tight polling.

## Tests And Acceptance

- Inventory
  - `cargo xtask family inventory --format json` is stdout-only, deterministic, side-effect-free, and shows `function.wrapper.pipeline.v1` as the supported-unpromoted family before promotion.
  - inventory contains no ranking, approval, or recommendation policy.
  - after green promotion, rerunning inventory shows `function.wrapper.pipeline.v1` promoted and no longer supported-unpromoted.
- `xtask` contract
  - wrapper harness registration, routing order, suite slug ownership, starter paths, and per-bucket starter semantics are all covered by `xtask` tests.
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
  - green path writes a path-valid `promotion.execution.json` with real proof-artifact paths.
  - blocked path writes a path-valid `blocker.report.json` with locked machine-evidence shape.
  - approved `recommendation.latest.json` is preserved as audit trail unless inventory truth changes enough to force reopening Gate 1.
- Workspace boundary
  - `Cargo.toml` still lists only `spec-core`, `spec-cli`, and `xtask`.

## Assumptions

- Worktree naming follows the repo’s existing `spec-m24/*` pattern.
- `spec-core` may become a normal `xtask` dependency if inventory reuses runtime route truth directly.
- The human approval steps in M26 are mandatory and are the only intentional pauses in the execution flow.
- `.semantic-family-artifacts/*` remains a derived run-artifact surface throughout M26, not a checked-in deliverable requirement.
```
