<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/codex-m23-contract-autoplan-restore-20260429-154318.md -->
# M26 - Approval-Gated AI Family Promotion Loop

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/m26**  
Last rewritten: **2026-04-29**

## Plan Authority

This file is the authoritative M26 plan.

Upstream docs informed it:

- `docs/m26_implementation_plan_v0.1.md`
- `docs/m26_approval_gated_ai_family_promotion_loop_design_v0.1.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `docs/north_star_v0.2.md`
- `docs/high_level_technical_architecture_v0.2.md`
- `docs/roadmap_and_release_shape_v0.1.md`

If any upstream doc disagrees with this file, `PLAN.md` wins for M26 execution.

That matters because the upstream docs still carry one major contradiction:

- older draft shape: add a new `spec-orchestrator` workspace crate
- locked M26 shape: keep the workspace boundary unchanged and implement the hard kernel inside `xtask`

This plan resolves that contradiction explicitly. M26 does **not** add a new workspace crate.

## Problem Statement

M21 through M24 proved the narrow Rust wedge:

- runtime semantic review can distinguish aligned truth, drift, under-specification, and unsupported near misses
- packetized family promotion can land real promoted families under `smoke`, `prove`, and `certify`

What the repo still does not have is the right operator model.

Right now, family promotion still assumes a human can carry too much ceremony in their head:

- which family should be promoted next
- whether that choice is justified by repo truth
- how to scaffold a truthful packet
- how to tell a fixable failure from an honest blocker
- what changed across a promotion run

If that remains manual, broader Rust family coverage does not scale and multi-language planning is fake confidence.

M26 fixes that bottleneck. It locks a slower, safer loop:

1. AI reads deterministic repo truth and recommends the next family.
2. Human approves or rejects that candidate family.
3. AI performs the promotion loop under hard gates.
4. Human approves or rejects the final promoted output.

No hidden human rescue work is allowed between those two approvals.

## Milestone Outcome

When M26 is done, the repo can truthfully claim:

- AI-operated family promotion is real, not chat theater
- the only human approvals are target-family approval and final-output approval
- `cargo xtask family smoke`, `prove`, and `certify` remain the hard proof kernel
- recommendation, execution, and blocker surfaces are durable machine-readable artifacts
- the next bottleneck after M26 is throughput, not re-arguing packet ceremony

M26 does **not** claim:

- next-family ranking is globally optimal
- multi-language promotion is solved
- non-function families are in scope
- family authoring ceremony is fully minimized
- a standalone orchestration subsystem deserves to exist forever

## Scope

### In Scope

- lock the approval-gated operator contract for family promotion
- add a deterministic family-scoped repo-truth export surface
- lock machine-readable schemas for recommendation, execution, and blocker artifacts
- keep `xtask` as the hard proof kernel
- prove the loop on one real supported-but-unpromoted Rust family
- choose that family from repo truth, not taste
- make blocker termination honest and durable

### NOT In Scope

- multi-language backend work
- new target-language lowering
- non-function family promotion for `sum` or `data`
- broad recommendation optimization or scoring science
- human-facing UI, dashboard, or approval app work
- background autonomous promotion queues
- a new workspace crate purely for architectural neatness

## Step 0 - Scope Challenge

### What already exists

| Area | Current truth | M26 reuse decision |
|---|---|---|
| Runtime supported function routes | `spec-core/src/semantic_review.rs` routes `chain3 -> wrapper -> monotone_down -> monotone_up` | Reuse directly. M26 must not invent new runtime family theory. |
| Current promoted packets | `semantic-families/` already contains `function.wrapper.pipeline.chain3.v1`, `function.arithmetic_leaf.monotone_down_nonnegative.v1`, and `function.arithmetic_leaf.monotone_up.v1` | Reuse as the promoted baseline and routing anchor. |
| Hard proof primitives | `cargo xtask family new`, `smoke`, `prove`, and `certify` already exist in `xtask` | Keep. Do not replace. |
| Hard proof artifacts | `.semantic-family-artifacts/semantic-families/<family>/prove.latest.json`, `attempt-*.json`, and `certification.report.json` already exist | Keep them authoritative. M26 references them instead of duplicating them. |
| Canonical wrapper seed | `examples/ecommerce/units/pricing/calculate_total.unit.spec` already expresses the two-step wrapper semantic shape | Use it as semantic seed truth. |
| Existing wrapper wedge regressions | `spec-cli/tests/m14_regressions.rs` already exercises aligned, drift, under-specified, and unsupported-near-miss wrapper cases via `calculate_total` | Reuse those patterns instead of inventing a new wedge. |
| Existing internal wrapper packet corpus | `function.wrapper.pipeline.chain3.v1` already carries packet-local wrapper and leaf fixtures across all four buckets | Reuse as the fastest truthful seed for the dedicated wrapper family. |
| Workspace boundary | `Cargo.toml` still defines exactly `spec-core`, `spec-cli`, and `xtask` | Keep unchanged in M26. |

### Minimum change

The minimum honest diff is:

1. expose deterministic repo-truth inventory for recommendation input
2. lock durable orchestration artifact schemas
3. register and prove the dedicated wrapper family packet
4. run one real approval-gated loop using the existing hard gates

Anything broader is M27 or later.

### Complexity check

This is already a multi-module milestone, but it is still a bounded one:

- `xtask/src/family/**` takes the majority of the new deterministic work
- `spec-core` and `spec-cli` only need family-specific truth-surface and regression coverage
- no new published binary, service, or crate is introduced

That is engineered enough. A new workspace crate here would spend an innovation token for no payoff.

### Completeness check

The complete version is still cheap enough here. M26 should land with:

- full inventory contract
- locked artifact schemas
- family-specific scaffold truth
- prove/certify coverage
- blocker-path coverage
- explicit worktree parallelization

Do not ship a happy-path-only version and promise blocker honesty later.

### Distribution check

M26 introduces no new externally distributed artifact.

The ship surface is:

- one new `cargo xtask family inventory --format json` command
- one new promoted packet directory
- one new derived orchestration artifact tree under `.semantic-family-artifacts/family-promotion/`

That means there is no release-pipeline blocker for M26 beyond the normal workspace test surface.

## Resolved Premises

| Premise | Verdict | Why |
|---|---|---|
| The narrow Rust wedge is already proven | Accept | M21 through M24 already established this. M26 should not re-fight it. |
| The next bottleneck is throughput, not semantic credibility | Accept | Runtime routes, packet registry, and hard proof gates already exist. |
| Human approvals should be limited to target-family and final-output approval | Accept | This is the north-star operator model and forces honest machine-operable artifacts. |
| M26 should add a standalone `spec-orchestrator` workspace crate | Reject | Overbuilt. The deterministic kernel already lives in `xtask`, and the new crate adds surface without proving reuse pressure. |
| `function.wrapper.pipeline.v1` is the right first live proof target | Accept | It is already supported at runtime, already has a canonical seed, and broadens topology beyond the leaf families. |
| `cargo xtask family inventory --format json` is the right minimal repo-truth export | Accept | It exposes truth without embedding ranking policy. |

## Dream State Delta

```text
CURRENT
  runtime routes exist
  promoted packets exist
  proof kernel exists
  operator loop is still manual

        │
        ▼

M26
  AI reads deterministic repo truth
  human approves candidate family
  AI edits packet + tests under hard gates
  AI emits execution or blocker artifact
  human approves final output

        │
        ▼

12-MONTH IDEAL
  most meaningful Rust function families are promoted
  family selection pressure is measured from coverage truth
  second-language pilots plug into the same approval-gated model
  humans govern approvals, not ceremony
```

## Locked Architecture Decisions

### 1. Workspace boundary

Keep the public binary surface inside `xtask`.

Lock this:

- do **not** add a fourth workspace member in M26
- do **not** move `smoke`, `prove`, or `certify` out of `xtask`
- do **not** make `spec-core` or `spec-cli` depend on approval-state logic

### 2. Hard kernel vs operator policy

`xtask` owns:

- family registry truth
- scaffold generation
- smoke validation
- prove gate execution
- certify gate execution
- deterministic report emission
- pure repo-truth inventory export
- typed orchestration artifact schemas and validation tests

The AI operator owns:

- ranking families from inventory truth
- writing recommendation artifacts
- editing repo truth for the approved family
- retrying under hard gates
- writing promotion execution and blocker artifacts

That split is fixed for M26.

### 3. No hidden approval state

Inventory is a projection of repo truth only.

It must not contain:

- approval state
- LLM-authored reasoning prose
- inferred "best" choice

Approval state begins only after the AI writes the recommendation artifact.

## Architecture / Dependency View

```text
spec-core runtime semantic review
        │
        ├── explicit supported routes
        ├── unsupported reason codes
        └── canonical seed behavior in examples/tests
                │
                ▼
xtask repo-truth inventory export
                │
                ▼
AI recommendation layer
        │
        ├── writes recommendation.latest.json
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
                ├── green  -> promotion.execution.json -> human final approval
                └── blocked -> blocker.report.json
```

### Primary module ownership

| Layer | Responsibility in M26 | Must not happen |
|---|---|---|
| `spec-core` | remain the source of runtime supported-family truth and unsupported diagnostics | do not absorb approval or orchestration policy |
| `spec-cli` | remain the source of read-side truth-surface and wedge regression proof | do not become the operator loop |
| `xtask/src/family/**` | remain the deterministic family packet kernel, plus the new inventory export and schema validation | do not embed ranking heuristics or human approvals |
| `.semantic-family-artifacts/**` | hold proof artifacts and orchestration artifacts | do not become authored source |

## Locked Inventory Contract

M26 adds:

```bash
cargo xtask family inventory --format json
```

This is the minimal honest export because it answers "what does the repo already know?" without smuggling in approval policy.

Required fields:

| Field | Meaning |
|---|---|
| `schema_version` | `1` |
| `generated_at` | UTC timestamp |
| `promoted_families[]` | family ids already registered and packetized |
| `runtime_supported_routes[]` | supported function compatibility keys in routing order |
| `supported_unpromoted_families[]` | supported family ids not yet promoted |
| `supported_unpromoted_families[].family` | repo-supported family id |
| `supported_unpromoted_families[].canonical_seed_paths[]` | real source-unit paths that anchor the family |
| `supported_unpromoted_families[].existing_wedge_paths[]` | existing regression or corpus paths that already exercise the family |
| `supported_unpromoted_families[].supporting_packet_paths[]` | existing packet-local fixture paths that can seed a dedicated packet |
| `supported_unpromoted_families[].routing_predecessor` | immediate promoted or runtime predecessor |
| `supported_unpromoted_families[].routing_successors[]` | lower-precedence family ids and terminal unsupported |

Locked rules:

- inventory does **not** rank candidates
- inventory does **not** include approval state
- inventory cites repo truth only

### Inventory command behavior

`cargo xtask family inventory --format json` is a read-only projection command.

Lock this behavior:

- JSON is written to stdout, not to an artifact file
- the command does not mutate repo files or `.semantic-family-artifacts/`
- stdout bytes are UTF-8 JSON in stable schema field order with exactly one trailing newline
- arrays are deterministically ordered
  - `promoted_families[]` in registered routing order
  - `runtime_supported_routes[]` in runtime routing order
  - `supported_unpromoted_families[]` in runtime routing order
  - all path arrays sorted lexicographically
- captured inventory snapshots and `inventory_sha256` are computed from the verbatim stdout bytes,
  including the trailing newline
- command exits `0` only when the projection is internally coherent
- command exits nonzero if runtime-supported route truth and promoted-family registry truth cannot be projected into one coherent inventory

`inventory_path` in downstream artifacts therefore points to a captured inventory snapshot
written by the operator under `.semantic-family-artifacts/family-promotion/inventory/`.
That snapshot is a derived run artifact, not authored source and not an assumed
checked-in deliverable.

### Locked initial truth for the first M26 run

Unless repo truth changes before implementation starts, the first green inventory should show:

- `promoted_families = [`
  - `function.wrapper.pipeline.chain3.v1`
  - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - `function.arithmetic_leaf.monotone_up.v1`
  - `]`
- `supported_unpromoted_families = [function.wrapper.pipeline.v1]`

If the initial inventory does not show that shape, stop and fix the projection before doing recommendation work.

## Locked Artifact Contract

M26 adds three durable orchestration artifacts under:

```text
.semantic-family-artifacts/family-promotion/
```

### Recommendation packet

Path:

```text
.semantic-family-artifacts/family-promotion/recommendation.latest.json
```

Required fields:

| Field | Meaning |
|---|---|
| `schema_version` | `1` |
| `artifact_kind` | `family_recommendation` |
| `generated_at` | UTC timestamp |
| `inventory_path` | exact inventory artifact used |
| `inventory_sha256` | SHA-256 digest of the exact inventory snapshot bytes used |
| `target_language` | `rust` |
| `ranked_candidates[]` | ranked candidate list |
| `ranked_candidates[0].family` | the only family the human is allowed to approve from this artifact |
| `ranked_candidates[].evidence[]` | repo paths, not paraphrases |
| `ranked_candidates[].expected_leverage` | why this family matters now |
| `ranked_candidates[].expected_risks[]` | known risks before approval |

Locked rules:

- only `ranked_candidates[0]` is approval-eligible
- every evidence claim must cite a repo path
- this artifact is recommendation-only, not execution state
- backup candidates are informational only

### Recommendation packet semantics

Lock this:

- `ranked_candidates[0]` is the sole candidate the human may approve from that file
- if the human wants candidate `N > 0`, AI must write a fresh recommendation artifact with that family moved to index `0`
- evidence entries are repo paths only, not prose summaries
- recommendation generation is external operator behavior, but the schema is repo-owned and validated in `xtask` tests
- `inventory_sha256` must be computed from the exact captured inventory snapshot
  referenced by `inventory_path`
- Gate 1 is a pre-edit approval over repo truth before approved-family edits begin
- Gate 1 approval remains valid only while a fresh inventory snapshot of the unchanged
  pre-edit basis yields both:
  - the same `inventory_sha256`
  - the same `ranked_candidates[0].family`
- the required recheck happens immediately before the first approved-family edit
- after the first approved-family edit lands, Gate 1 is no longer compared against live
  post-edit inventory; from that point on, correctness is governed by the hard gates and
  the post-promotion inventory expectations
- if the pre-edit Gate 1 basis check changes, the prior approval is stale; AI must halt,
  write a fresh `recommendation.latest.json`, rerun artifact validation, and wait for a new
  human approval before wrapper-family edits continue
- a stale Gate 1 approval may not be reused across packet, runtime, CLI, or
  integration work

### Promotion execution report

Path:

```text
.semantic-family-artifacts/family-promotion/<family>/<run-id>/promotion.execution.json
```

Required fields:

| Field | Meaning |
|---|---|
| `schema_version` | `1` |
| `artifact_kind` | `promotion_execution` |
| `run_id` | stable run id for this attempt series |
| `family` | approved family id |
| `status` | `green` or `blocked` |
| `recommendation_path` | exact recommendation artifact used |
| `approvals.target_family.status` | `approved` |
| `approvals.final_output.status` | `pending`, `approved`, or `rejected` |
| `files_changed[]` | source files changed during promotion |
| `commands[]` | every hard-gate command that ran, with exit code |
| `referenced_proof_artifacts[]` | exact paths to `prove.latest.json`, `attempt-*.json`, and `certification.report.json` |
| `iterations` | number of AI retry loops |
| `gate_summary` | final smoke/prove/certify state |
| `notes[]` | short factual notes, not essay text |

Locked rules:

- this is the final human approval surface
- it must reference actual proof-artifact paths
- it may not claim green if `certification.report.json` is missing or failing
- it exists only for an already-approved target family

### Promotion execution semantics

Lock this:

- `status` is execution outcome only, not human final approval outcome
- `approvals.target_family.status` is always `approved` in this file
- `approvals.final_output.status` starts as `pending`; the human may later change it to `approved` or `rejected`
- `run_id` format is `{UTC-basic-timestamp}-{family}`, for example `20260429T154500Z-function.wrapper.pipeline.v1`
- `commands[]` entries must include at least:
  - `step`
  - `command`
  - `exit_code`
  - `started_at`
  - `finished_at`
  - `artifact_path` when that command produced or refreshed a proof artifact
- `files_changed[]` must be repo-relative paths, sorted lexicographically
- `referenced_proof_artifacts[]` must all exist on disk at report-write time

### Blocker report

Path:

```text
.semantic-family-artifacts/family-promotion/<family>/<run-id>/blocker.report.json
```

Required fields:

| Field | Meaning |
|---|---|
| `schema_version` | `1` |
| `artifact_kind` | `promotion_blocker` |
| `run_id` | same run id as the matching execution attempt |
| `family` | approved family id |
| `blocking_step` | `inventory`, `scaffold`, `smoke`, `prove`, or `certify` |
| `blocker_kind` | stable blocker vocabulary |
| `summary` | one-sentence factual blocker summary |
| `machine_evidence[]` | commands, exit codes, and artifact paths |
| `required_human_action` | exact missing decision or truth |
| `safe_next_actions[]` | what must remain unchanged while fixing the blocker |

Locked rules:

- blocker termination is a first-class honest outcome
- blocker classification must cite machine evidence
- blocker reports must name the exact human decision or missing truth that stopped the loop

### Blocker vocabulary

M26 locks the first blocker kinds to this set:

- `inventory_projection_mismatch`
- `inventory_no_supported_candidate`
- `scaffold_contract_mismatch`
- `smoke_contract_failure`
- `prove_suite_failure`
- `certify_suite_failure`
- `certify_routing_conflict`
- `proof_artifact_missing`
- `human_decision_required`

If a blocker does not fit one of those values, stop and extend the vocabulary explicitly in the plan rather than inventing an ad hoc string during implementation.

### Machine evidence contract

Each `machine_evidence[]` entry must include:

- `kind` as one of `command`, `artifact`, or `diff`
- `path` for artifact or diff evidence
- `command` and `exit_code` for command evidence
- `observed_at`
- `note` as a short factual string

## Locked First Live Proof Target

The first live M26 family proof is locked to:

- `function.wrapper.pipeline.v1`

Why this wins now:

- it already exists in runtime route order
- it already has a truthful semantic seed in `pricing/calculate_total`
- it already has aligned, drift, under-specified, and unsupported-near-miss wedge rewrites
- it broadens topology from arithmetic leaves to a two-step wrapper
- `function.wrapper.pipeline.chain3.v1` already reserves it in `must_not_shadow`

Fallback rule:

- if implementation truth proves `function.wrapper.pipeline.v1` is not promotion-ready, M26 must emit a blocker report
- it may not silently switch to another family without a fresh recommendation artifact and a fresh human approval

## Locked Family Contract

### Family id

- `function.wrapper.pipeline.v1`

### Summary

- `Straight-line two-call wrapper pipeline over supported semantic deps.`

### Locked routing metadata

- `precedence = 2`
- `must_not_shadow = [`
  - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - `function.arithmetic_leaf.monotone_up.v1`
  - `unsupported.function.v1`
  - `]`

### Locked suite slug

- `wrapper_pipeline_`

### Locked scaffold template

Add a dedicated template:

- `StarterTemplate::WrapperPipelineTwoStep`

Do **not** reuse `GenericPlaceholder`.

### Locked packet-local starter set

Each bucket must contain exactly these starter units:

- `fixtures/<bucket>/units/pricing/pricing_discount_leaf_<bucket>.unit.spec`
- `fixtures/<bucket>/units/pricing/pricing_tax_leaf_<bucket>.unit.spec`
- `fixtures/<bucket>/units/pricing/pricing_total_wrapper_<bucket>.unit.spec`

This packet is self-contained on purpose. The wrapper unit depends on packet-local supported leaf fixtures, not on mutable external example units.

### Canonical aligned truth

Semantic seed:

- `examples/ecommerce/units/pricing/calculate_total.unit.spec`

Packet-local aligned wrapper truth:

- unit id: `pricing/pricing_total_wrapper_aligned`
- deps:
  - `pricing/pricing_discount_leaf_aligned`
  - `pricing/pricing_tax_leaf_aligned`
- body shape:

```text
{
    let discounted = pricing_discount_leaf_aligned(subtotal, discount_rate);
    pricing_tax_leaf_aligned(discounted, tax_rate)
}
```

### Bucket contract

| Bucket | Locked behavior |
|---|---|
| `aligned` | discount leaf stays monotone-down nonnegative, tax leaf stays monotone-up, wrapper applies discount then tax |
| `drift` | wrapper reverses the order and taxes before discount, but still claims discount then tax |
| `under_specified` | wrapper body stays aligned, but semantic surface weakens to vague truth |
| `unsupported_near_miss` | wrapper stays semantically close but leaves the honest subset by threading `tax_rate.max(Decimal::ZERO)` into the second call |

## Smoke / Prove / Certify Contract

### Smoke

Required command:

```bash
cargo xtask family smoke function.wrapper.pipeline.v1
```

Smoke owns scaffold honesty only.

Smoke must verify:

- `family.toml` regenerates byte-for-byte
- all twelve locked starter unit specs exist
- the aligned wrapper starter contains:
  - `subtotal: Decimal`
  - `discount_rate: Decimal`
  - `tax_rate: Decimal`
  - both packet-local dep ids
  - the locked let-threaded wrapper body

### Prove

Required prove suites:

- `cargo test -p spec-core --lib wrapper_pipeline_classifier_ -- --color never`
- `cargo test -p spec-cli --test cli wrapper_pipeline_truth_surface_ -- --color never`
- `cargo test -p spec-cli --test m14_regressions wrapper_pipeline_corpus_ -- --color never`

Prove responsibilities:

- aligned wrapper fixtures route to `function.wrapper.pipeline.v1`
- reversed-pipeline drift projects `semantic_drift`
- vague intent projects `under_specified`
- unsupported near miss stays unsupported and additive-only
- truth-surface behavior stays honest across `spec test`, `spec status`, `spec build`, and `spec export`

### Certify

Required certify suite:

- `cargo test -p spec-cli --test m14_regressions wrapper_pipeline_regression_ -- --color never`

Required certify command:

```bash
cargo xtask family certify function.wrapper.pipeline.v1
```

Certify responsibilities:

- re-run prove and persist the attempt artifact
- enforce manifest-local routing truth for the new wrapper family
- enforce registry-global order:
  - chain3
  - wrapper pipeline
  - monotone-down leaf
  - monotone-up leaf
  - unsupported terminal
- confirm chain3 remains green and unshadowed
- confirm both arithmetic leaves remain green and unshadowed

## Exact AI Operator Command Loop

### Phase A - Inventory and Recommendation

Required machine step:

```bash
cargo xtask family inventory --format json
```

AI then writes:

```text
.semantic-family-artifacts/family-promotion/recommendation.latest.json
```

Human then approves or rejects `ranked_candidates[0].family`.

Before any approved-family edits continue, the operator must capture and retain the
inventory snapshot used by that recommendation under:

```text
.semantic-family-artifacts/family-promotion/inventory/<run-id>.json
```

The recommendation artifact must reference that snapshot through `inventory_path`
and `inventory_sha256`. That snapshot is the sole Gate 1 approval basis for the run.

The operator must also record the pre-edit basis commit that produced that snapshot.
That commit is the only commit on which Gate 1 inventory equality may be rechecked.

### Phase B - Promotion Loop After Approval

For approved family `<family>`, the AI loop is:

```bash
cargo fmt --all
cargo test -p xtask
cargo xtask family smoke <family>
cargo xtask family prove <family>
cargo xtask family certify <family>
```

Allowed fast inner-loop commands:

```bash
cargo test -p spec-core --lib wrapper_pipeline_
cargo test -p spec-cli --test cli wrapper_pipeline_
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_
```

Locked loop rule:

1. read the approved recommendation artifact
2. before the first approved-family edit, verify the unchanged pre-edit basis still matches
   the approved Gate 1 basis:
   - rerun inventory
   - capture the fresh snapshot
   - compare `inventory_sha256`
   - compare `ranked_candidates[0].family`
3. if either pre-edit Gate 1 basis check changed, stop and reopen Gate 1 with a fresh
   recommendation artifact
4. edit repo truth for the approved family
5. run targeted tests if useful
6. rerun `smoke`
7. rerun `prove`
8. rerun `certify`
9. if green, write `promotion.execution.json`
10. if blocked, write `blocker.report.json`

The human does not steer those retries.
The live post-edit repo state is not required to match the original Gate 1 inventory snapshot,
because approved-family edits are expected to change inventory-visible truth.

### Phase C - Final Approval

When the hard gates are green, AI writes:

```text
.semantic-family-artifacts/family-promotion/<family>/<run-id>/promotion.execution.json
```

Human then approves or rejects the final output from that report.

## Test / Coverage Diagram

```text
CODE PATH COVERAGE
===========================
[+] xtask inventory export
    ├── [GAP] promoted-family projection
    ├── [GAP] runtime-supported unpromoted-family projection
    ├── [GAP] canonical seed path emission
    └── [GAP] supporting packet-path emission

[+] xtask wrapper family scaffold
    ├── [GAP] harness registration + routing order
    ├── [GAP] starter template emits 12 locked unit specs
    ├── [GAP] smoke exact-match contract for family.toml
    └── [GAP] smoke content contract for aligned wrapper body

[+] spec-core semantic review
    ├── [EXISTS] runtime route for function.wrapper.pipeline.v1
    ├── [GAP] dedicated wrapper-packet classifier prove tests
    ├── [GAP] chain3-vs-wrapper routing-order regression
    └── [GAP] wrapper-family certify slug coverage

[+] spec-cli truth surface + corpus
    ├── [EXISTS] calculate_total wedge regressions
    ├── [GAP] dedicated wrapper-packet corpus tests
    ├── [GAP] truth-surface preserve/stale tests for wrapper packet
    └── [GAP] additive-only unsupported-near-miss read-side regression

[+] orchestration artifacts
    ├── [GAP] recommendation artifact schema round-trip
    ├── [GAP] execution report schema round-trip
    ├── [GAP] blocker report schema round-trip
    └── [GAP] execution report must reference real proof artifacts

OPERATOR FLOW COVERAGE
===========================
[+] Approval gate 1
    ├── [GAP] top-ranked candidate only
    └── [GAP] no silent target-family switch

[+] Promotion loop
    ├── [GAP] green path writes execution report
    ├── [GAP] certify failure writes blocker report
    └── [GAP] no hidden human steering between approvals

CRITICAL GAPS
===========================
1. inventory export does not exist yet
2. wrapper family packet is not yet promoted
3. orchestration artifact schemas are not yet locked in executable tests
```

## Error & Rescue Registry

| Failure | Why it matters | Rescue path |
|---|---|---|
| Inventory export omits a real supported-but-unpromoted family | Recommendation quality becomes fake because AI is ranking incomplete truth | Fix the inventory projection first. Do not proceed to recommendation. |
| Wrapper family overlaps chain3 incorrectly | Promotion may look green locally but break routing truth globally | Certify must fail Gate D and the blocker report must force explicit routing repair. |
| Scaffold still emits generic placeholders | `family new` and `family smoke` would validate the wrong family contract | Add `StarterTemplate::WrapperPipelineTwoStep` and smoke content contracts before packet curation. |
| Execution report references stale or missing proof artifacts | Final approval becomes untrustworthy | Treat missing proof-artifact references as execution-report validation failure. |
| Blocker report is assembled from hand-wavy stderr strings | AI will thrash instead of stopping honestly | Add stable blocker kinds and machine-evidence references in M26, not later. |

## Failure Modes

| Codepath | Real production failure | Test coverage required | Error handling required | User-visible outcome | Critical gap |
|---|---|---|---|---|---|
| `family inventory` | wrapper pipeline omitted from the supported-unpromoted list | golden JSON fixture test for wrapper candidate presence | exit nonzero if runtime route and inventory projection disagree structurally | wrong candidate recommendation | Yes |
| wrapper scaffold | starter files are created with generic placeholder semantics | xtask unit tests for starter file paths and smoke contents | `family smoke` fails with exact missing-content error | AI cannot start truthful packet curation | No |
| `family prove` | aligned wrapper routes correctly but drift or under-specified cases are mislabeled | dedicated `wrapper_pipeline_classifier_` plus corpus suites | existing prove artifact plus failing suite list | AI sees prove failure and retries | No |
| `family certify` | routing order allows wrapper to shadow leaves or conflict with chain3 | certify regression suite plus Gate D routing diagnostics | blocker report cites manifest or registry routing failure | honest blocker instead of silent bad promotion | No |
| execution report writer | report says green without a real `certification.report.json` reference | schema round-trip plus path-exists assertion | artifact validation fails | human cannot honestly approve | Yes |

## Performance / Operational Notes

- Keep the public binary surface at `cargo xtask ...`. No new crate, no new published artifact.
- Use targeted `wrapper_pipeline_` test prefixes before full gates to reduce thrash.
- `family inventory` must stay fast and deterministic. It should read repo truth, not generated caches.
- Promotion artifacts live under `.semantic-family-artifacts/`. They are derived outputs, not authored source.
- `xtask` can take a normal `spec-core` dependency in M26 if inventory reuses runtime route truth directly. Duplicating routing truth would be worse.

## Distribution / Ship Surface

M26 adds no new end-user distribution artifact.

The ship surface remains:

- existing workspace crates: `spec-core`, `spec-cli`, `xtask`
- one new `xtask` subcommand: `family inventory`
- one new promoted family packet: `function.wrapper.pipeline.v1`
- new derived promotion artifacts under `.semantic-family-artifacts/family-promotion/`

## Implementation Sequence

1. Keep the workspace boundary unchanged.
   `Cargo.toml` must still list only `spec-core`, `spec-cli`, and `xtask`.

2. Add `cargo xtask family inventory --format json`.
   Touch:
   - `xtask/src/lib.rs`
   - new `xtask/src/family/inventory.rs`
   - `xtask/Cargo.toml` if `spec-core` becomes a normal dependency

3. Add typed promotion artifact contracts and validation tests.
   Keep them in `xtask`. Do not create a new package just to serialize JSON.
   Use a dedicated module:
   - `xtask/src/family/promotion_artifacts.rs`

4. Register `function.wrapper.pipeline.v1` in `xtask/src/family/harness.rs`.
   Add:
   - `WRAPPER_PIPELINE_PRECEDENCE = 2`
   - `WRAPPER_PIPELINE_MUST_NOT_SHADOW = [...]`
   - `WRAPPER_PIPELINE_SUITE_SLUG = "wrapper_pipeline_"`
   - prove suite definitions
   - certify suite definitions
   - `StarterTemplate::WrapperPipelineTwoStep`
   - `FamilyHarness` entry
   - registry-order tests that place wrapper between chain3 and the leaves

5. Extend `xtask/src/family/scaffold.rs`.
   Add a truthful wrapper-family starter template that emits the twelve locked starter units and aligned smoke-content contract.

6. Extend `xtask/src/lib.rs` tests.
   Add lock tests for:
   - inventory JSON shape
   - inventory ordering and exit behavior
   - harness contract
   - registry routing order
   - starter scaffold file paths
   - smoke content contract
   - promotion artifact schema round-trips
   - blocker vocabulary validation
   - execution report path-exists validation for referenced proof artifacts

7. Add the committed packet at:

```text
semantic-families/function.wrapper.pipeline.v1/
```

Seed it by lifting the existing wrapper-family fixture corpus embedded in the chain3 packet, then tighten it to the dedicated wrapper-family contract above.

8. Extend runtime and read-side proof surfaces.
   Touch:
   - `spec-core/src/semantic_review.rs`
   - `spec-cli/tests/cli.rs`
   - `spec-cli/tests/m14_regressions.rs`

9. Run the real approval-gated loop end to end.
   Required sequence:

```bash
cargo xtask family inventory --format json
# AI writes recommendation.latest.json
# human approves function.wrapper.pipeline.v1
cargo fmt --all
cargo test -p xtask
cargo xtask family smoke function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1
cargo xtask family certify function.wrapper.pipeline.v1
# AI writes promotion.execution.json or blocker.report.json
```

10. Update repo-truth docs only after the loop is green.
    Touch:
    - `semantic-families/README.md`

## Worktree Parallelization Strategy

M26 has one hard serialization lane and then three bounded parallel lanes.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Lock inventory export + artifact schemas | `xtask/src/lib.rs`, `xtask/src/family/inventory.rs`, `xtask/src/family/report.rs`, `xtask/Cargo.toml` | — |
| Lock wrapper family contract + scaffold | `xtask/src/family/harness.rs`, `xtask/src/family/scaffold.rs`, `xtask/src/lib.rs` | Lock inventory export + artifact schemas |
| Curate committed wrapper packet | `semantic-families/function.wrapper.pipeline.v1/` | Lock wrapper family contract + scaffold |
| Add runtime classifier and route-order proof | `spec-core/src/semantic_review.rs` | Lock wrapper family contract + scaffold |
| Add CLI truth-surface/corpus/regression tests | `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs` | Lock wrapper family contract + scaffold |
| Final command loop + docs | repo-wide commands, `semantic-families/README.md` | packet curation, runtime tests, CLI tests |

### Parallel lanes

- Lane A: `Lock inventory export + artifact schemas` -> `Lock wrapper family contract + scaffold`
- Lane B: `Curate committed wrapper packet`
- Lane C: `Add runtime classifier and route-order proof`
- Lane D: `Add CLI truth-surface/corpus/regression tests`
- Lane E: `Final command loop + docs`

### Execution order

1. Run Lane A first and keep it sequential.
2. Treat Lane A output as the frozen wrapper-family contract for M26:
   - suite slug
   - packet file names and starter paths
   - per-bucket starter semantics
   - unsupported-near-miss boundary
3. Merge the Lane A result onto `feat/m26` and record that resulting commit as the
   `contract_freeze_commit`.
4. Create `ws/m26-packet`, `ws/m26-runtime`, and `ws/m26-cli` from that exact
   `contract_freeze_commit`, then launch Lanes B, C, and D in parallel worktrees.
5. Merge B, C, and D.
6. Run Lane E only after that merge.

### Run-state ownership

- The primary checkout at the live `feat/m26` path is the canonical run-state root for M26.
- All `.runs/m26/*` and `.semantic-family-artifacts/family-promotion/**` writes are owned by the
  parent agent and resolved against that primary checkout root, not against worker-local worktree
  relative paths.
- Worker worktrees may read frozen prompts or summaries, but they do not become independent
  sources of truth for approvals, sentinels, or promotion artifacts.
- Artifact validation in worker or integration contexts must target the canonical artifact path,
  preferably as an absolute path under the primary checkout root.

### Conflict flags

- Lane A is the serialization point because it fixes suite slug, packet file names, inventory fields, smoke contracts, and all 12 starter semantics.
- The parent must not launch B, C, or D from a stale pre-freeze commit. The recorded
  `contract_freeze_commit` is the only valid worker base.
- Lanes B, C, and D are parallel only after Lane A freezes the wrapper-family contract.
- Lane C may add classifier proof and route-order assertions, but it may not redefine wrapper-family semantics, starter paths, or the unsupported-near-miss boundary.
- Lanes B and D must consume the frozen Lane A contract literally. They do not invent or reinterpret family semantics independently.
- If Lane C discovers that runtime truth disagrees with the frozen Lane A contract, stop parallel lane-local reconciliation, return the mismatch to the serialized owner flow, and relaunch affected lanes only after the contract is updated explicitly.
- Docs stay out of parallel lanes. Do not claim AI-operated promotion until the end-to-end loop is green.

## Acceptance Gates

M26 is done only when all of the following are true:

- `Cargo.toml` still lists only `spec-core`, `spec-cli`, and `xtask`
- `cargo xtask family inventory --format json` exists and emits the locked family-scoped truth
- inventory output is deterministic, stdout-only, and side-effect-free
- inventory truth shows `function.wrapper.pipeline.v1` as supported but unpromoted before the run
- the only human approvals are target-family approval and final-output approval
- the recommendation artifact exists and cites repo-path evidence
- the recommendation artifact allows approval of only `ranked_candidates[0].family`
- `function.wrapper.pipeline.v1` is registered in `xtask/src/family/harness.rs`
- `StarterTemplate::WrapperPipelineTwoStep` exists and is used by the harness
- a committed packet exists at `semantic-families/function.wrapper.pipeline.v1/`
- `cargo xtask family smoke function.wrapper.pipeline.v1` passes
- `cargo xtask family prove function.wrapper.pipeline.v1` passes
- `cargo xtask family certify function.wrapper.pipeline.v1` passes
- `promotion.execution.json` references real `prove.latest.json` and `certification.report.json`
- `promotion.execution.json` uses the locked `run_id`, `commands[]`, and `files_changed[]` contracts
- `blocker.report.json` exists as a tested honest termination path even though the milestone exits through the green path
- `blocker.report.json` uses only locked `blocker_kind` values and locked `machine_evidence[]` shape
- chain3 remains green and unshadowed
- both arithmetic leaves remain green and unshadowed
- `semantic-families/README.md` is updated to match the new promoted-family truth

## Follow-ups Explicitly Deferred

- M27 ranking optimization and coverage accounting
- any second-language work
- non-function family promotion
- a standalone orchestration crate, if future reuse pressure genuinely proves it
- approval UI or dashboard work
- background autonomous promotion queues

## Unresolved Risks

- `family inventory` is new, and the quickest bad version is an incomplete one. Inventory must be tested as carefully as any gate command.
- `function.wrapper.pipeline.v1` is the right first target, but it may expose real routing pressure against chain3. If so, certify must force an explicit answer.
- current proof failures still lean on stderr strings in some paths. M26 may need sharper structured blocker mapping than the current reports expose.
- lifting wrapper fixtures out of the chain3 packet risks accidental drift if the dedicated packet and chain3 packet stop agreeing on shared helper semantics.

## Review Summary

- Step 0: Scope Challenge — accepted with one deliberate scope reduction: **no new workspace crate**
- Architecture Review: 3 major decisions locked
- Code Quality Review: 2 abstraction reductions locked
- Test Review: diagram produced, 11 required coverage targets identified
- Performance Review: 2 operational constraints locked
- NOT in scope: written
- What already exists: written
- Failure modes: 2 critical gaps flagged
- Outside voice: not run as a separate model pass in this rewrite
- Parallelization: 5 lanes, 3 parallel after 1 serialization lane
- Lake Score: complete option chosen everywhere that materially affected correctness

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Keep M26 as the workflow bridge between the Rust wedge and broader Rust coverage | Mechanical | Completeness | This is the next real bottleneck after M24 | Re-opening semantic-theory scope |
| 2 | CEO | Limit human approvals to candidate family and final output | Mechanical | Explicit over clever | Hidden mid-loop rescue work would fake the operator model | Manual steering between gates |
| 3 | CEO | Lock `function.wrapper.pipeline.v1` as the first live proof target | Taste | Pragmatic | It broadens topology using existing repo truth and wedge evidence | Another leaf-family proof |
| 4 | Eng | Reject a new `spec-orchestrator` workspace crate in M26 | Mechanical | Minimal diff | The current workspace and `xtask` boundary already fit the problem | Adding a fourth workspace member |
| 5 | Eng | Add `cargo xtask family inventory --format json` as the minimal repo-truth export | Mechanical | Explicit over clever | Inventory exposes truth without embedding ranking policy | Ranking inside `xtask` |
| 6 | Eng | Add a dedicated `StarterTemplate::WrapperPipelineTwoStep` | Mechanical | Completeness | Generic placeholder starters are not truthful enough for a promoted family | Reusing `GenericPlaceholder` |
| 7 | Eng | Reuse chain3 packet-local wrapper fixtures as the wrapper packet seed | Mechanical | DRY | The repo already has truthful wrapper-family bucket material | Rebuilding the packet from scratch |
| 8 | Eng | Keep `xtask` as the hard proof kernel and make orchestration artifacts reference its reports | Mechanical | Systems over heroes | One proof source keeps the operator loop auditable | Competing proof artifact systems |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/autoplan` | Scope & strategy | 1 | CLEAR | Kept M26 narrow as the workflow bridge, not a ranking or multi-language milestone. Locked wrapper pipeline as the first proof target and rejected a new crate. |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | No separate external-model pass was run in this rewrite. |
| Eng Review | `/autoplan` | Architecture & tests (required) | 1 | CLEAR | Locked the `xtask` boundary, the `family inventory` export, the wrapper-family packet contract, the command loop, the coverage map, and worktree parallelization. |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | No UI scope in M26. |

**VERDICT:** CEO + ENG CLEARED. `PLAN.md` is now the single implementation-ready M26 execution contract.
