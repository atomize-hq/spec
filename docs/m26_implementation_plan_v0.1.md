# M26 - Approval-Gated AI Family Promotion Loop

Status: **implementation plan**  
Base branch: **main**  
Working branch: **codex/m23-contract**  
Last rewritten: **2026-04-29**

Source of truth for this plan:

- repo code and docs in this checkout
- [PLAN.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md)
- [docs/ai_promotion_and_multilanguage_milestones_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/ai_promotion_and_multilanguage_milestones_v0.1.md)
- [docs/m26_approval_gated_ai_family_promotion_loop_design_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/m26_approval_gated_ai_family_promotion_loop_design_v0.1.md)
- [docs/north_star_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/north_star_v0.2.md)
- [docs/high_level_technical_architecture_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/high_level_technical_architecture_v0.2.md)
- [docs/roadmap_and_release_shape_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/roadmap_and_release_shape_v0.1.md)

M21 through M24 proved the narrow Rust wedge first: the runtime semantic reviewer can distinguish
aligned truth, drift, under-specification, and unsupported near misses for real promoted function
families.

M26 does not try to re-prove that thesis. M26 turns that proof into the right operator model.

The causal chain is fixed:

1. prove the intent-drift thesis in one narrow Rust wedge
2. make family promotion operable by AI under hard proof gates
3. use that promotion machinery to broaden per-language semantic review coverage
4. only then factor toward multi-language support

If step 2 remains manual, steps 3 and 4 do not scale.

## Problem Statement

After M24, the repo has two kinds of truth:

- runtime semantic-review truth in `spec-core` and read-side surfaces in `spec-cli`
- packetized promotion truth and proof gates in `xtask` plus `semantic-families/`

What is still missing is the operator loop that joins those surfaces without falling back to hidden
human ceremony.

Today, promotion still assumes a human can hold too much in their head:

- which family to promote next
- whether that choice is justified by repo truth
- how to scaffold and curate the packet
- how to interpret `smoke`, `prove`, and `certify` failures
- when a failure is fixable versus when it is an honest blocker

That is not the north-star operator model. The north-star model is slower, safer,
verify-as-it-builds AI work under explicit gates.

M26 therefore locks the operator model:

1. AI recommends the next family candidate from repo truth.
2. Human approves or rejects that candidate.
3. AI performs the full promotion loop under hard gates.
4. Human approves or rejects the final promoted output.

Human approval is limited to the candidate family and the final output. The human does not rescue
the loop between those points.

## Milestone Outcome

When M26 is done, the repo can truthfully claim:

- family promotion is operable by AI rather than only by a repo expert
- the hard gates remain `cargo xtask family smoke`, `prove`, and `certify`
- recommendation, execution, and blocker surfaces are durable machine-readable artifacts rather
  than chat-only reasoning
- a human intervenes only at the two planned approval boundaries
- the promotion loop now exists at the right abstraction layer to unlock broader Rust family
  coverage first and multi-language work later

M26 does **not** claim:

- the recommendation engine is globally optimal
- multi-language promotion is already solved
- non-function families are now in scope
- family authoring ceremony is fully minimized
- AI can invent new semantic families without repo-grounded seeds

## Scope

### In scope

- lock the approval-gated operator model for family promotion
- add durable machine-readable artifacts for recommendation, execution, and blockers
- define a deterministic repo-truth input surface for the AI recommender
- keep `xtask` as the hard proof primitive layer
- place recommendation ranking, retry logic, approval checkpoints, and blocker synthesis in a
  higher-level orchestration layer
- complete one real Rust family promotion through the approval-gated AI loop
- choose the first live proof target from current repo truth, not from speculation

### NOT in scope

- multi-language backend work
- new target-language lowering
- non-function family promotion for `sum` or `data`
- global coverage optimization across all future families
- maintainer UI or dashboard work
- generic long-horizon agent workflow infrastructure beyond what M26 directly needs
- replacing `xtask` proof commands with a new orchestration command family

## Why This Milestone Now

The narrow Rust proof came first on purpose.

The repo thesis was never "AI should guess family semantics." The thesis was:

- semantic review should detect intent drift honestly
- AI should operate inside validation, build, test, and evidence gates

M21 through M24 proved the first half in one language and one bounded function subset.

That changes the next bottleneck. The blocker is no longer semantic-review credibility. The blocker
is throughput.

Without an approval-gated AI promotion loop:

- broad Rust family coverage stays manual
- the set of promoted families grows too slowly to matter
- multi-language work arrives before the Rust workflow has learned how to scale

So M26 is the bridge milestone. It turns a manually proved Rust wedge into an AI-operated family
promotion loop, which is the prerequisite for both wider Rust coverage and later multi-language
portability.

## Premises

1. Human approvals are limited to candidate selection and final output approval.
2. AI must do the recommendation, editing, iteration, and failure handling work between those
   approvals.
3. `xtask` remains the deterministic proof primitive because it already owns scaffold, smoke,
   prove, certify, and artifact writing for family packets.
4. Recommendation logic should consume repo truth; it should not live inside `xtask` as opaque
   policy.
5. The first live M26 family proof should use a family class the repo already supports at runtime
   but has not yet promoted through the packet workflow.
6. M26 proves the operator model, not global selection optimality. M27 can optimize ranking later.

## What Already Exists

| Area | Current truth | M26 reuse decision |
|---|---|---|
| North-star operator thesis | AI should work in a slower, safer, verify-as-it-builds loop | Reuse directly. M26 makes it concrete for family promotion. |
| Current promoted packet registry | `semantic-families/` currently promotes `function.wrapper.pipeline.chain3.v1`, `function.arithmetic_leaf.monotone_down_nonnegative.v1`, and `function.arithmetic_leaf.monotone_up.v1` | Reuse as the baseline packet and artifact contract. |
| Current runtime supported function routes | `spec-core/src/semantic_review.rs` routes `WrapperPipelineChain3`, `WrapperPipeline`, `ArithmeticLeafMonotoneDownNonnegative`, and `ArithmeticLeafMonotoneUp` in explicit order | Use this as recommendation input. Do not invent new runtime families in M26. |
| Current proof primitives | `cargo xtask family new`, `smoke`, `prove`, `certify` | Keep as the hard proof loop. Do not replace them. |
| Current proof artifacts | `prove.latest.json`, `attempt-*.json`, and `certification.report.json` under `.semantic-family-artifacts/semantic-families/<family>/` | Reuse as gate truth and reference them from M26 orchestration artifacts. |
| Current blocker vocabulary | unsupported-function reason codes and certify routing diagnostics already exist | Reuse as the first machine evidence for blocker classification. |
| Supported-but-unpromoted candidate | `function.wrapper.pipeline.v1` exists in runtime semantic review, is referenced in routing metadata, and has a canonical seed at `examples/ecommerce/units/pricing/calculate_total.unit.spec` | This is the recommended first live M26 proof family unless implementation truth disproves it. |
| Existing wedge evidence for that candidate | `spec-cli/tests/m14_regressions.rs` already has `calculate_total` drift, under-specified, and unsupported-near-miss rewrites | Reuse as the narrowest truthful first promotion case. |

## Minimal Change Strategy

M26 stays narrow on purpose.

- Do not move semantic-family proof logic out of `xtask`.
- Do not put recommendation ranking or approval state into `spec-core`.
- Do not invent a multi-language kernel yet.
- Do not prove the AI loop on a brand-new semantic family.

The minimum honest diff is:

1. expose deterministic repo-truth inventory for recommendation input
2. add durable M26 orchestration artifacts
3. define the approval-gated command loop around existing `xtask` proof gates
4. prove the loop on one already-supported but unpromoted Rust family

Anything broader is M27 or later.

## Architecture / Dependency View

```text
spec-core runtime semantic review
        │
        ├── explicit supported routes
        ├── unsupported reason codes
        └── canonical seed behavior in examples/tests
                │
                ▼
deterministic repo-truth inventory surface
                │
                ▼
AI recommendation layer
        │
        ├── writes recommendation packet
        └── waits for human target approval
                │
                ▼
AI promotion loop
        │
        ├── edits harness / packet / tests / docs for approved family
        ├── runs cargo xtask family smoke
        ├── runs cargo xtask family prove
        ├── runs cargo xtask family certify
        ├── reads prove/certify artifacts
        └── repeats until green or blocked
                │
                ├── green  -> promotion execution report -> human final approval
                └── blocked -> blocker report
```

### Primary module boundaries

| Layer | Responsibility in M26 | Must not happen |
|---|---|---|
| `spec-core` | remain source of runtime family truth, supported routing order, and unsupported diagnostics | do not absorb AI recommendation or approval state |
| `spec-cli` | remain source of read-side truth-surface and corpus regressions | do not become the orchestration layer |
| `xtask/src/family/**` | keep deterministic family packet primitives: scaffold, smoke, prove, certify, report emission, and pure repo-truth export | do not embed ranking heuristics, human approvals, or LLM-specific policy |
| new workspace crate `spec-orchestrator/src/**` | recommend candidate families, checkpoint approvals, run the edit-and-proof loop, interpret failures, and write M26 orchestration artifacts | do not reimplement `smoke`, `prove`, or `certify` logic |
| `.semantic-family-artifacts/**` | hold machine-readable proof and orchestration artifacts | do not become authored source |

## Locked Operator Contract

M26 does not leave the operator model to implementer taste.

### Human approvals

- approval point 1: approve or reject the AI recommendation packet
- approval point 2: approve or reject the final promotion execution report

No third approval is allowed for ordinary promotion work.

### AI responsibilities

- inspect repo truth
- rank candidate families
- emit a recommendation packet
- after approval, perform repo edits for the approved family
- run proof commands
- inspect machine-readable artifacts and command failures
- retry within hard gates
- emit either a promotion execution report or a blocker report

### Hard boundary

- `xtask` owns deterministic proof primitives and pure repo projections
- `spec-orchestrator` owns recommendation, approval gating, loop control, and blocker synthesis

That boundary is fixed for M26.

## Locked Artifact Contract

M26 adds three orchestration artifacts. These are provisional schemas, but their role and paths are
locked by the plan.

### Artifact locations

- recommendation packet:
  `.semantic-family-artifacts/family-promotion/recommendation.latest.json`
- promotion execution report:
  `.semantic-family-artifacts/family-promotion/<family>/<run-id>/promotion.execution.json`
- blocker report:
  `.semantic-family-artifacts/family-promotion/<family>/<run-id>/blocker.report.json`

The execution and blocker artifacts must reference the existing `xtask` proof artifacts rather than
copying their payloads.

### Recommendation packet

Purpose: give the human a repo-grounded candidate choice before any promotion work starts.

Provisional schema:

```json
{
  "schema_version": 1,
  "artifact_kind": "family_recommendation",
  "generated_at": "2026-04-29T00:00:00Z",
  "target_language": "rust",
  "current_promoted_families": [
    "function.wrapper.pipeline.chain3.v1",
    "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    "function.arithmetic_leaf.monotone_up.v1"
  ],
  "ranked_candidates": [
    {
      "family": "function.wrapper.pipeline.v1",
      "status": "recommended",
      "why_now": "Supported in runtime semantic review but not yet promoted through the packet workflow.",
      "evidence": [
        {
          "kind": "runtime_supported_route",
          "path": "spec-core/src/semantic_review.rs"
        },
        {
          "kind": "canonical_seed",
          "path": "examples/ecommerce/units/pricing/calculate_total.unit.spec"
        },
        {
          "kind": "existing_wedge_regressions",
          "path": "spec-cli/tests/m14_regressions.rs"
        }
      ],
      "expected_leverage": {
        "language": "rust",
        "surface": "kind:function",
        "topology": "two-dep wrapper pipeline"
      },
      "expected_risks": [
        "routing overlap with function.wrapper.pipeline.chain3.v1",
        "packet curation may reveal family boundary drift"
      ]
    }
  ]
}
```

Locked rules:

- recommendation is evidence-backed, not gut feel
- `ranked_candidates[0]` is the only approval target
- the packet must cite repo paths for every substantive claim

### Promotion execution report

Purpose: record the whole AI-operated run, the proof commands, the edits made, and the final gate
state for human approval.

Provisional schema:

```json
{
  "schema_version": 1,
  "artifact_kind": "promotion_execution",
  "run_id": "20260429T120000Z-function.wrapper.pipeline.v1",
  "family": "function.wrapper.pipeline.v1",
  "status": "green",
  "recommendation_path": ".semantic-family-artifacts/family-promotion/recommendation.latest.json",
  "approvals": {
    "target_family": {
      "status": "approved",
      "approved_at": "2026-04-29T12:05:00Z"
    },
    "final_output": {
      "status": "pending"
    }
  },
  "repo_state": {
    "git_commit_sha": "HEAD-at-start",
    "rust_toolchain": "rustc --version"
  },
  "files_changed": [
    "xtask/src/family/harness.rs",
    "semantic-families/function.wrapper.pipeline.v1/family.toml"
  ],
  "commands": [
    {
      "step": "smoke",
      "command": "cargo xtask family smoke function.wrapper.pipeline.v1",
      "exit_code": 0
    },
    {
      "step": "prove",
      "command": "cargo xtask family prove function.wrapper.pipeline.v1",
      "exit_code": 0,
      "artifact_path": ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/prove.latest.json"
    },
    {
      "step": "certify",
      "command": "cargo xtask family certify function.wrapper.pipeline.v1",
      "exit_code": 0,
      "artifact_path": ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/certification.report.json"
    }
  ],
  "iterations": 3,
  "gate_summary": {
    "smoke": "pass",
    "prove": "pass",
    "certify": "pass"
  },
  "notes": [
    "AI resolved one packet-layout failure without extra human steering."
  ]
}
```

Locked rules:

- this is the final human approval surface
- it must reference every hard-gate command that ran
- it must list changed source surfaces
- it must distinguish green completion from blocked termination

### Blocker report

Purpose: terminate honestly when AI cannot complete promotion inside the hard gates without hidden
human rescue work.

Provisional schema:

```json
{
  "schema_version": 1,
  "artifact_kind": "promotion_blocker",
  "run_id": "20260429T120000Z-function.wrapper.pipeline.v1",
  "family": "function.wrapper.pipeline.v1",
  "blocking_step": "certify",
  "blocker_kind": "routing_incoherence",
  "summary": "Gate D failed because runtime routing and manifest routing disagree.",
  "machine_evidence": [
    {
      "command": "cargo xtask family certify function.wrapper.pipeline.v1",
      "exit_code": 1,
      "artifact_path": ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/attempt-2026-04-29T12:40:00Z.json"
    }
  ],
  "required_human_action": "Decide whether function.wrapper.pipeline.v1 should precede or follow chain3 in the promoted registry.",
  "safe_next_actions": [
    "Keep prove/certify gates unchanged.",
    "Resolve runtime/harness routing disagreement explicitly."
  ],
  "terminal": true
}
```

Locked rules:

- a blocker is a first-class honest outcome in M26
- blocker classification must cite machine evidence, not conversational opinion
- the report must tell the human exactly what decision or missing truth prevented completion

## Locked `xtask` vs Orchestration Boundary

M26 must answer where logic lives.

### `xtask` should own

- `family new`, `family smoke`, `family prove`, `family certify`
- packet layout validation
- manifest validation
- routing diagnostics
- existing proof artifact emission
- a new pure repo-truth export surface if needed, for example:
  `cargo xtask family inventory --format json`

Why:

- these are deterministic repo-local computations
- they should be testable without AI
- they already define the hard proof contract

### `spec-orchestrator` should own

- ranking candidate families from repo truth
- writing `recommendation.latest.json`
- waiting at approval boundaries
- making repo edits across harness, packet, tests, and docs
- deciding when to rerun fast inner-loop commands versus full proof gates
- synthesizing `promotion.execution.json` and `blocker.report.json`

Why:

- this layer is stateful and workflow-oriented
- it must encode approval checkpoints
- it should be able to evolve without destabilizing the deterministic proof kernel

### Locked `spec-orchestrator` module split

The first M26 plan should not leave the orchestration crate structure implicit.

- `spec-orchestrator/src/recommend.rs`
  reads repo-truth inventory and emits ranked recommendation packets
- `spec-orchestrator/src/approvals.rs`
  owns the two approval checkpoints and their persisted state transitions
- `spec-orchestrator/src/run.rs`
  owns command ordering, retry state, and loop termination rules
- `spec-orchestrator/src/blockers.rs`
  maps proof failures into stable blocker kinds with machine evidence
- `spec-orchestrator/src/report.rs`
  writes `promotion.execution.json` and final approval bundles

The crate may start as a binary-only workspace member in M26, but the ownership
of these responsibilities is locked to this crate rather than left diffuse
across shell scripts or chat-only glue.

### Hard non-goals for `xtask`

- no hidden LLM prompt logic
- no approval checkpoint persistence
- no ranking heuristics that choose the next family autonomously

## First Live Proof-Family Selection

M26 needs one real family promotion to prove the operator loop. That first target must be chosen
by repo truth, not by taste.

### Selection criteria

The first live M26 family should satisfy all of these:

1. it is already recognized as a supported runtime semantic-review family or route
2. it has a canonical real seed unit in repo truth
3. it already has aligned, drift, under-specified, or unsupported-near-miss wedge material that
   can be reused truthfully
4. it does not require new evaluator-scope invention
5. it broadens Rust coverage meaningfully rather than duplicating a direct sibling proof
6. it keeps M26 focused on workflow proof, not semantic-theory expansion

### Recommended first live target

The recommended first live M26 proof family is:

- `function.wrapper.pipeline.v1`

Reasoning, grounded in current repo truth:

- `spec-core/src/semantic_review.rs` already contains the runtime compatibility key
  `function.wrapper.pipeline.v1` and a `WrapperPipeline` route.
- the canonical seed already exists at
  `examples/ecommerce/units/pricing/calculate_total.unit.spec`.
- `spec-cli/tests/m14_regressions.rs` already contains `calculate_total` drift,
  under-specified, and unsupported-near-miss rewrite helpers.
- `spec-cli/tests/cli.rs`, `spec-core/src/passport.rs`, and `spec-core/src/export.rs` already
  exercise `pricing/calculate_total` as `function.wrapper.pipeline.v1`.
- `function.wrapper.pipeline.chain3.v1` already declares `function.wrapper.pipeline.v1` in its
  routing boundary, so promoting the two-dep wrapper family stabilizes an existing shadow line
  rather than inventing a new one.

This is a stronger first M26 target than another arithmetic leaf because it broadens function
topology coverage from single-leaf arithmetic to the reusable two-dep wrapper shape that the repo
already understands.

If implementation truth later proves that `function.wrapper.pipeline.v1` is not promotion-ready,
the fallback rule is not "pick something convenient." The fallback rule is "choose the next
repo-supported but unpromoted family that still satisfies the criteria above."

## Exact AI Operator Command Loop

M26 should lock one exact command loop for the AI operator.

### Phase A - inventory and recommendation

Required machine step:

```bash
cargo xtask family inventory --format json
```

AI then writes:

- `.semantic-family-artifacts/family-promotion/recommendation.latest.json`

Human then approves or rejects `ranked_candidates[0].family`.

### Phase B - promotion loop after approval

For approved family `<family>`, the AI loop is:

```bash
cargo fmt --all
cargo test -p xtask
cargo xtask family smoke <family>
cargo xtask family prove <family>
cargo xtask family certify <family>
```

Fast inner-loop commands are allowed between hard-gate runs, but they do not replace the hard
gates. The AI may also run targeted suites while iterating, for example:

```bash
cargo test -p spec-core --lib <family-slug>
cargo test -p spec-cli --test m14_regressions <family-slug>
```

Loop rule:

1. edit repo truth for the approved family
2. run the fast inner loop if useful
3. rerun `smoke`
4. rerun `prove`
5. rerun `certify`
6. if green, write the execution report
7. if blocked, write the blocker report

The human does not steer any of those retries.

### Phase C - final approval

When all hard gates are green, AI writes:

- `.semantic-family-artifacts/family-promotion/<family>/<run-id>/promotion.execution.json`

Human then approves or rejects the final output using that report.

## Implementation Sequence

1. Lock the artifact and operator contract in code-facing docs and tests.
   Add the M26 artifact shapes, approval states, and command-loop expectations as executable
   invariants where possible.

2. Add deterministic repo-truth inventory export.
   This belongs in `xtask` because it is a pure projection of current registry, runtime supported
   routes, promoted packets, and seed evidence paths.

3. Add the `spec-orchestrator` workspace crate.
   Extend the workspace to include one new orchestration crate whose only job is
   approval-gated AI family promotion. Do not smear this logic across `xtask`,
   `spec-cli`, or ad hoc repo scripts.

4. Add the recommendation writer.
   It consumes the inventory export and emits `recommendation.latest.json` with ranked candidates
   and repo-path evidence.

5. Add the orchestration runner.
   It owns approvals, retry state, command execution ordering, and final artifact writing. It must
   call into `xtask`, not duplicate `xtask`.

6. Add blocker classification.
   Map `smoke`, `prove`, `certify`, routing, manifest, and unsupported-diagnostic failures into a
   stable `blocker_kind` vocabulary.

7. Prove the loop on `function.wrapper.pipeline.v1`.
   Use `pricing/calculate_total` as the canonical seed and the existing `calculate_total` wedge
   rewrites as the initial packet truth.

8. Run the full approval-gated loop end to end.
   Require the actual two approvals, machine-written artifacts, and green hard gates before
   claiming success.

9. Document only repo truth that is green.
   Do not claim AI-operated promotion has landed until the first real family promotion and its
   artifacts exist.

## Acceptance Gates

M26 is done only when all of the following are true:

- the operator model is explicit and enforced: human approves target family and final output only
- the recommendation packet exists and cites repo-path evidence for the chosen family
- the `spec-orchestrator` crate can run the promotion loop without hidden human steering
- `xtask` remains the hard proof kernel for `smoke`, `prove`, and `certify`
- one real Rust family promotion completes through that loop
- the first live proof target is selected from repo truth, not ad hoc judgment
- the final execution report references the actual `prove.latest.json` and
  `certification.report.json` artifacts
- if the loop cannot finish honestly, it emits a blocker report instead of silently requiring
  manual rescue
- `xtask`, `spec-core`, and `spec-cli` do not gain reverse dependencies on `spec-orchestrator`
- the M26 implementation leaves the system better positioned for broad Rust family expansion and
  later multi-language work

## Follow-ups Explicitly Deferred

- M27 coverage accounting and ranking optimization
- language-agnostic family-kernel extraction
- second-language promotion work
- non-function family promotion
- maintainer UI for approvals and artifact browsing
- continuous autonomous background promotion queues

## Unresolved Risks

- The current repo has strong proof primitives but no shipped inventory export yet. If that export
  is underspecified, the recommendation layer will stay too chat-dependent.
- `function.wrapper.pipeline.v1` is the best current first target from repo truth, but promoting it
  may expose real routing pressure against `chain3` that must be resolved explicitly.
- Existing `xtask` reports are machine-readable, but some failure causes still arrive as stderr
  strings. M26 may need sharper structured diagnostics to avoid AI thrash.
- Packet curation may still hide too much maintainer judgment in `candidate.md` unless the blocker
  vocabulary is crisp.
- If orchestration artifacts drift away from `xtask` proof artifacts, the system could develop two
  competing truths. M26 must keep the proof kernel authoritative.
