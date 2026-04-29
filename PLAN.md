<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/codex-m23-contract-autoplan-restore-20260429-154318.md -->
# M26 - Approval-Gated AI Family Promotion Loop

Status: **implementation contract**  
Base branch: **main**  
Working branch: **codex/m23-contract**  
Last rewritten: **2026-04-29**

Source of truth for this plan:

- repo code and docs in this checkout
- [docs/m26_implementation_plan_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/m26_implementation_plan_v0.1.md)
- [docs/m26_approval_gated_ai_family_promotion_loop_design_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/m26_approval_gated_ai_family_promotion_loop_design_v0.1.md)
- [docs/ai_promotion_and_multilanguage_milestones_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/ai_promotion_and_multilanguage_milestones_v0.1.md)
- [docs/north_star_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/north_star_v0.2.md)
- [docs/high_level_technical_architecture_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/high_level_technical_architecture_v0.2.md)
- [docs/roadmap_and_release_shape_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/roadmap_and_release_shape_v0.1.md)
- direct repo truth in:
  - [xtask/src/lib.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs)
  - [xtask/src/family/harness.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/harness.rs)
  - [xtask/src/family/scaffold.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/scaffold.rs)
  - [xtask/src/family/prove.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/prove.rs)
  - [xtask/src/family/certify.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/certify.rs)
  - [xtask/src/family/report.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/report.rs)
  - [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs)
  - [spec-cli/tests/m14_regressions.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/m14_regressions.rs)
  - [semantic-families/README.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md)

M21 through M24 already proved the narrow Rust wedge. The repo can now distinguish aligned truth,
drift, under-specification, and unsupported near misses for real promoted function families.

M26 is not another semantics-theory milestone. M26 is the operator-model milestone that makes
those proofs scale.

The causal chain is fixed:

1. prove the intent-drift thesis in one narrow Rust wedge
2. make family promotion operable by AI under hard proof gates
3. use that promotion machinery to cover most of Rust family-by-family
4. only then factor toward multiple languages

If step 2 stays manual, steps 3 and 4 are fake roadmap confidence.

## Problem Statement

After M24, the repo has two strong but separate truths:

- runtime semantic-review truth in `spec-core` plus read-side truth in `spec-cli`
- deterministic packet promotion truth in `xtask` plus committed packets in `semantic-families/`

What is still missing is the durable machine-operable bridge between them.

Right now, family promotion still assumes a human can hold too much ceremony in their head:

- which family should be promoted next
- why that choice is justified by repo truth
- how to scaffold a truthful packet
- how to distinguish a fixable gate failure from an honest blocker
- what changed across the promotion run

That is the wrong operator model for the repo thesis in [docs/north_star_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/north_star_v0.2.md). The right model is slower, safer, and explicit:

1. AI reads deterministic repo truth and recommends the next family.
2. Human approves or rejects that candidate family.
3. AI performs the promotion loop under hard gates.
4. Human approves or rejects the final promoted output.

No hidden human rescue work is allowed between those two approvals.

## Milestone Outcome

When M26 is done, the repo can truthfully claim:

- AI-operated family promotion is real, not chat theater
- the only human approvals are candidate-family approval and final-output approval
- `cargo xtask family smoke`, `prove`, and `certify` remain the hard proof kernel
- recommendation, execution, and blocker artifacts are durable machine-readable files
- the first broadening step after the Rust wedge is now throughput, not re-arguing semantics

M26 does **not** claim:

- next-family ranking is globally optimal
- multi-language promotion is solved
- non-function families are in scope
- packet ceremony is fully minimized
- a new standalone orchestration subsystem deserves to exist forever

## Scope

### In scope

- lock the approval-gated operator contract for family promotion
- add a deterministic family-scoped repo-truth export surface
- lock machine-readable schemas for recommendation, execution, and blocker artifacts
- keep `xtask` as the hard proof kernel
- prove the loop on one real supported-but-unpromoted Rust family
- choose that family from repo truth, not taste
- make blocker termination honest and durable

### NOT in scope

- multi-language backend work
- new target-language lowering
- non-function family promotion for `sum` or `data`
- broad recommendation optimization or scoring science
- human-facing UI, dashboard, or approval app work
- background autonomous promotion queues
- a new workspace crate purely for architectural neatness

## Premise Challenge

| Premise | Verdict | Why |
|---|---|---|
| The narrow Rust wedge is already proven | Accept | M21 through M24, plus current runtime routes and packetized families, already established this. M26 should not re-fight it. |
| The next bottleneck is workflow throughput, not semantic credibility | Accept | Runtime routes, wedge regressions, and packet gates already exist. The limiting factor is how many families can be promoted safely. |
| Human approvals should be limited to target-family and final-output approval | Accept | This matches the north-star operator model and forces honest machine-operable artifacts. |
| M26 should add a standalone `spec-orchestrator` workspace crate | Reject | That is one abstraction too early. `xtask` already identifies itself as "Workspace orchestration for semantic family packets" in `xtask/Cargo.toml`. Adding a fourth workspace member before reuse pressure is proven is unnecessary surface area. |
| `function.wrapper.pipeline.v1` is the right first live proof target | Accept | It already exists in runtime routing order, has a canonical seed at `pricing/calculate_total`, has wedge regressions, and broadens topology from leaves to two-step wrappers. |
| `cargo xtask family inventory --format json` is the right minimal repo-truth export | Accept | It is family-scoped, deterministic, and policy-free. It exposes truth. It does not pretend to rank or approve. |

## Why This Milestone Now

The repo already proved the thing that had to come first: semantic review can catch meaningful
intent drift in a narrow real Rust subset.

That changes the honest next question.

The next question is not "can the classifier work?" The next question is "can this family
promotion workflow be operated by AI without turning into hidden human ceremony?"

That is why M26 exists now:

- broad Rust coverage needs throughput
- throughput needs deterministic repo truth plus hard gates
- multi-language work before that loop exists would be premature architecture tourism

## Dream State Delta

```text
CURRENT
  runtime routes exist
  promoted families exist
  proof kernel exists
  operator still manual

        │
        ▼

M26
  AI reads repo truth
  human approves target family
  AI edits + runs smoke/prove/certify
  AI emits execution or blocker artifact
  human approves final output

        │
        ▼

12-MONTH IDEAL
  most meaningful Rust function families promoted
  selection pressure measured from coverage truth
  second-language pilots plug into the same gate model
  humans govern approvals, not ceremony
```

## What Already Exists

| Area | Current truth | M26 reuse decision |
|---|---|---|
| Runtime function routing | [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs) routes `WrapperPipelineChain3`, `WrapperPipeline`, `ArithmeticLeafMonotoneDownNonnegative`, then `ArithmeticLeafMonotoneUp` in explicit order | Reuse directly. M26 must not invent new runtime family theory. |
| Canonical wrapper seed | [examples/ecommerce/units/pricing/calculate_total.unit.spec](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/examples/ecommerce/units/pricing/calculate_total.unit.spec) is already the truthful two-step wrapper wedge | Use as the semantic source for the first live proof. |
| Existing wrapper wedge regressions | [spec-cli/tests/m14_regressions.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/m14_regressions.rs) already has aligned, reversed-pipeline drift, under-specified, and unsupported-near-miss rewrites for `calculate_total` | Reuse those exact wedge patterns instead of inventing new ones. |
| Existing internal wrapper packet corpus | `function.wrapper.pipeline.chain3.v1` already carries packet-local `pricing_total_wrapper_*`, `pricing_discount_leaf_*`, and `pricing_tax_leaf_*` fixtures across all four buckets | Reuse this internal corpus as the fastest truthful packet seed for the dedicated wrapper family. |
| Current promoted packets | `semantic-families/` currently promotes chain3 plus the two arithmetic leaves | Use as the promoted-family baseline and routing order anchor. |
| Proof kernel | `cargo xtask family new`, `smoke`, `prove`, `certify` already exist and already write machine-readable prove/certify artifacts | Keep. Do not replace. |
| Proof artifacts | `prove.latest.json`, `attempt-*.json`, and `certification.report.json` are already written under `.semantic-family-artifacts/semantic-families/<family>/` | Keep them authoritative. M26 artifacts must reference them, not duplicate them. |
| Routing truth | [semantic-families/README.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md) already documents terminal `unsupported.function.v1` behavior and current promoted-family order | Reuse for doc truth. Update only after M26 is green. |
| Workspace boundary | [Cargo.toml](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/Cargo.toml) currently has exactly three workspace members: `spec-core`, `spec-cli`, and `xtask` | Keep this boundary in M26. |

## Alternatives Pressure Test

| Option | Shape | Verdict | Why |
|---|---|---|---|
| A | Add new `spec-orchestrator` workspace crate | Reject | Overbuilt for M26. It adds a new package, new workspace wiring, and new API seams before reuse pressure exists. |
| B | Keep one public binary, extend `xtask` with deterministic inventory export and typed promotion artifact contracts, let the AI operator live outside the repo | Choose | This preserves the proof kernel, minimizes diff, and keeps policy outside deterministic code. |
| C | Keep the current manual/chat loop and only write docs | Reject | This does not create promotion machinery. It just renames the current bottleneck. |

## Locked Architecture Decisions

### 1. Orchestration Boundary

`spec-orchestrator` is **not** the right M26 boundary.

The right M26 boundary is:

- keep the public binary surface inside `xtask`
- keep deterministic proof primitives in `xtask/src/family/**`
- add one pure repo-truth export at `xtask/src/family/inventory.rs`
- add typed promotion artifact structs and validation tests inside `xtask`
- keep the actual AI operator outside the repo, using locked artifacts plus shell commands

This matters because the AI operator is a workflow role, not a library boundary yet.

Lock this explicitly:

- do **not** add a fourth workspace member in M26
- do **not** make `spec-core` or `spec-cli` depend on approval-state logic
- do **not** move `smoke`, `prove`, or `certify` out of `xtask`

### 2. Hard Kernel vs AI Policy

`xtask` owns:

- family registry and routing truth
- scaffold generation
- smoke validation
- prove gate execution
- certify gate execution
- deterministic report emission
- pure repo-truth inventory export

The AI operator owns:

- ranking candidates from inventory truth
- writing recommendation artifacts
- making repo edits for the approved family
- deciding when to rerun targeted tests versus full gates
- writing promotion execution and blocker artifacts

That split is the M26 contract.

### 3. Minimal Repo-Truth Export

M26 adds this command:

```bash
cargo xtask family inventory --format json
```

It is the minimal honest export because it answers "what does the repo already know?" without
smuggling in ranking policy.

Required JSON fields:

| Field | Meaning |
|---|---|
| `schema_version` | Locked to `1` in M26 |
| `generated_at` | UTC timestamp |
| `promoted_families[]` | Family ids currently registered and packetized |
| `runtime_supported_routes[]` | Supported runtime compatibility keys in routing order |
| `supported_unpromoted_families[]` | Repo-supported family ids that are not yet promoted |
| `supported_unpromoted_families[].canonical_seed_paths[]` | Real source units that anchor the family |
| `supported_unpromoted_families[].existing_wedge_paths[]` | Existing regression or corpus paths that already exercise the family |
| `supported_unpromoted_families[].supporting_packet_paths[]` | Existing packet-local fixture paths that can be lifted into a dedicated packet |
| `supported_unpromoted_families[].routing_predecessor` | Immediate promoted or runtime predecessor |
| `supported_unpromoted_families[].routing_successors[]` | Lower-precedence family ids and terminal unsupported catch-all |

Locked rules:

- inventory does **not** rank candidates
- inventory does **not** contain approval state
- inventory does **not** contain LLM-written prose
- inventory cites repo truth only

### 4. First Live Proof Target

The first live M26 proof target is locked to:

- `function.wrapper.pipeline.v1`

Why this wins now:

- it already exists in runtime route order
- it already has a truthful seed in `pricing/calculate_total`
- it already has aligned, drift, under-specified, and unsupported-near-miss wedge rewrites
- it broadens topology from arithmetic leaves to two-step wrappers
- it does not require new evaluator-scope invention
- chain3 already declares it in `must_not_shadow`, so promoting it stabilizes an existing boundary

Fallback rule:

- if M26 discovers `function.wrapper.pipeline.v1` is not promotion-ready, the loop must emit a
  blocker report
- it may not silently switch to another family without a fresh recommendation artifact and a fresh
  human approval

## Locked Artifact Contract

M26 adds three durable orchestration artifacts under:

```text
.semantic-family-artifacts/family-promotion/
```

### Recommendation Packet

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
| `inventory_path` | Path to the exact inventory export used |
| `target_language` | `rust` |
| `ranked_candidates[]` | Ranked candidate list |
| `ranked_candidates[0].family` | The only family the human is allowed to approve from this artifact |
| `ranked_candidates[].evidence[]` | Repo paths, not paraphrases |
| `ranked_candidates[].expected_leverage` | Why this family matters now |
| `ranked_candidates[].expected_risks[]` | Known risks before approval |

Locked rules:

- only `ranked_candidates[0]` is approval-eligible
- every evidence claim must cite a repo path
- this artifact is recommendation-only, not execution state

### Promotion Execution Report

Path:

```text
.semantic-family-artifacts/family-promotion/<family>/<run-id>/promotion.execution.json
```

Required fields:

| Field | Meaning |
|---|---|
| `schema_version` | `1` |
| `artifact_kind` | `promotion_execution` |
| `run_id` | Stable run id for this attempt series |
| `family` | Approved family id |
| `status` | `green` or `blocked` |
| `recommendation_path` | Exact recommendation artifact used |
| `approvals.target_family.status` | `approved` or `rejected` |
| `approvals.final_output.status` | `pending`, `approved`, or `rejected` |
| `files_changed[]` | Source files changed during promotion |
| `commands[]` | Every hard-gate command that ran, with exit code |
| `referenced_proof_artifacts[]` | Exact paths to `prove.latest.json`, `attempt-*.json`, and `certification.report.json` as applicable |
| `iterations` | Number of AI retry loops |
| `gate_summary` | Final smoke/prove/certify state |
| `notes[]` | Short factual notes, not essay text |

Locked rules:

- this is the final human approval surface
- it must reference actual proof-artifact paths
- it must distinguish green completion from blocked termination
- it may not claim green if `certification.report.json` is missing or failing

### Blocker Report

Path:

```text
.semantic-family-artifacts/family-promotion/<family>/<run-id>/blocker.report.json
```

Required fields:

| Field | Meaning |
|---|---|
| `schema_version` | `1` |
| `artifact_kind` | `promotion_blocker` |
| `run_id` | Same run id as the matching execution attempt |
| `family` | Approved family id |
| `blocking_step` | `inventory`, `scaffold`, `smoke`, `prove`, or `certify` |
| `blocker_kind` | Stable blocker vocabulary |
| `summary` | One-sentence factual blocker summary |
| `machine_evidence[]` | Commands, exit codes, and artifact paths |
| `required_human_action` | The exact missing decision or truth |
| `safe_next_actions[]` | What must remain unchanged while fixing the blocker |

Locked rules:

- blocker termination is a first-class honest outcome
- blocker classification must cite machine evidence
- blocker reports must name the exact human decision or missing truth that stopped the loop

## Locked Family Contract

M26 does not leave the first live family packet to implementer taste.

### Family Id

- `function.wrapper.pipeline.v1`

### Summary

- `Straight-line two-call wrapper pipeline over supported semantic deps.`

### Locked Routing Metadata

- `precedence = 2`
- `must_not_shadow = [`
  - `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - `function.arithmetic_leaf.monotone_up.v1`
  - `unsupported.function.v1`
  - `]`

Why:

- runtime route order is `chain3 -> wrapper -> monotone_down -> monotone_up`
- chain3 already reserves precedence `1`
- the wrapper family must sit between chain3 and the leaves
- the registry contract requires lower-precedence successors plus terminal unsupported to stay explicit

### Locked Suite Slug

- `wrapper_pipeline_`

Every family-owned prove/certify suite for this family must use that slug.

### Locked Scaffold Template

Add a dedicated template:

- `StarterTemplate::WrapperPipelineTwoStep`

Do **not** reuse `GenericPlaceholder`. The current chain3 scaffold deliberately emits unsupported
placeholder starters. That is wrong for a dedicated promoted wrapper family.

### Locked Packet-Local Starter Set

Each bucket must contain exactly these starter units:

- `fixtures/<bucket>/units/pricing/pricing_discount_leaf_<bucket>.unit.spec`
- `fixtures/<bucket>/units/pricing/pricing_tax_leaf_<bucket>.unit.spec`
- `fixtures/<bucket>/units/pricing/pricing_total_wrapper_<bucket>.unit.spec`

This family packet is self-contained on purpose. The wrapper unit depends on packet-local supported
leaf fixtures, not on mutable external example units.

### Canonical Aligned Truth

Semantic seed:

- [examples/ecommerce/units/pricing/calculate_total.unit.spec](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/examples/ecommerce/units/pricing/calculate_total.unit.spec)

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

This is the same semantic claim as `pricing/calculate_total`, but lifted into a self-contained
family packet.

## Packet Contract By Bucket

### Aligned

- packet-local discount leaf remains monotone-down nonnegative
- packet-local tax leaf remains monotone-up
- wrapper body applies discount first, then tax

### Drift

- wrapper body reverses the order and taxes before discount
- semantic claim still says discount then tax
- this must project semantic drift, not unsupported

### Under Specified

- wrapper body stays aligned
- authored semantic surface weakens to vague truth
- this must project under-specified, not drift

### Unsupported Near Miss

- wrapper body stays semantically close but leaves the honest subset by changing the threaded tax
  argument to a non-parameter expression
- locked shape:

```text
{
    pricing_tax_leaf_unsupported_near_miss(
        pricing_discount_leaf_unsupported_near_miss(subtotal, discount_rate),
        tax_rate.max(Decimal::ZERO)
    )
}
```

- this must remain additive-only and health-neutral on read-side surfaces

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
- aligned wrapper starter contains:
  - `subtotal: Decimal`
  - `discount_rate: Decimal`
  - `tax_rate: Decimal`
  - both packet-local dep ids
  - the locked let-threaded wrapper body

Smoke does **not** require whole-packet byte equality against the committed packet.

### Prove

Required prove suites:

- `cargo test -p spec-core --lib wrapper_pipeline_classifier_ -- --color never`
- `cargo test -p spec-cli --test cli wrapper_pipeline_truth_surface_ -- --color never`
- `cargo test -p spec-cli --test m14_regressions wrapper_pipeline_corpus_ -- --color never`

Prove responsibilities:

- classifier routes aligned wrapper fixtures to `function.wrapper.pipeline.v1`
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
2. edit repo truth for the approved family
3. run targeted tests if useful
4. rerun `smoke`
5. rerun `prove`
6. rerun `certify`
7. if green, write `promotion.execution.json`
8. if blocked, write `blocker.report.json`

The human does not steer those retries.

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
    ├── [GAP] runtime-supported unpromoted family projection
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
    ├── [GAP] chain3-vs-wrapper routing order regression
    └── [GAP] wrapper-family certify slug coverage

[+] spec-cli truth surface + corpus
    ├── [EXISTS] calculate_total wedge regressions
    ├── [GAP] dedicated wrapper-packet corpus tests
    ├── [GAP] truth-surface preserve/stale tests for wrapper packet
    └── [GAP] read-side additive-only unsupported-near-miss regression

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
3. artifact schemas are not yet locked in executable tests
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
| `family inventory` | Wrapper pipeline omitted from supported-unpromoted list | Golden JSON fixture test for wrapper candidate presence | Exit nonzero if runtime route and inventory projection disagree structurally | Wrong candidate recommendation | Yes |
| wrapper scaffold | Starter files are created with generic placeholder semantics | xtask unit tests for starter file paths and smoke contents | `family smoke` fails with exact missing-content error | AI cannot start truthful packet curation | No |
| `family prove` | Wrapper classifier passes aligned fixture but mislabels drift or under-specified cases | dedicated `wrapper_pipeline_classifier_` plus corpus suites | existing prove artifact plus failing suite list | AI sees prove failure and retries | No |
| `family certify` | Routing order allows wrapper to shadow leaves or conflict with chain3 | certify regression suite plus Gate D routing diagnostics | blocker report cites manifest or registry routing failure | Honest blocker instead of silent bad promotion | No |
| execution report writer | Report says green without real `certification.report.json` reference | schema round-trip plus path-exists assertion | artifact validation fails | Human cannot honestly approve | Yes |

## Performance / Operational Notes

- Keep the public binary surface at `cargo xtask ...`. No new crate, no new published artifact.
- Use targeted `wrapper_pipeline_` test prefixes before full gates to reduce thrash.
- `family inventory` must stay fast and deterministic. It should read repo truth, not generated
  caches.
- Promotion artifacts live under `.semantic-family-artifacts/`. They are derived outputs, not
  authored source.
- `xtask` needs a normal `spec-core` dependency in M26 if runtime-supported routes are reused at
  runtime for inventory export. That is acceptable. Duplicating routing truth is not.

## Distribution / Ship Surface

M26 adds no new end-user distribution artifact.

The ship surface remains:

- existing workspace crates: `spec-core`, `spec-cli`, `xtask`
- one new `xtask` subcommand: `family inventory`
- one new promoted family packet: `function.wrapper.pipeline.v1`
- new derived promotion artifacts under `.semantic-family-artifacts/family-promotion/`

Code without distribution is still a trap, but this milestone is repo-process work. The usable
surface is the checked-in command and packet truth, not a new external binary.

## Implementation Sequence

1. Keep the workspace boundary unchanged.
   Do not add `spec-orchestrator`. `Cargo.toml` must still list only `spec-core`, `spec-cli`, and
   `xtask`.

2. Add `cargo xtask family inventory --format json`.
   Touch:
   - [xtask/src/lib.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs)
   - new `xtask/src/family/inventory.rs`
   - `xtask/Cargo.toml` if `spec-core` becomes a normal dependency

3. Add typed promotion artifact contracts and validation tests.
   Keep them in `xtask`. Do not create a new package just to serialize JSON.

4. Register `function.wrapper.pipeline.v1` in [xtask/src/family/harness.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/harness.rs).
   Add:
   - `WRAPPER_PIPELINE_PRECEDENCE = 2`
   - `WRAPPER_PIPELINE_MUST_NOT_SHADOW = [...]`
   - `WRAPPER_PIPELINE_SUITE_SLUG = "wrapper_pipeline_"`
   - prove suite definitions
   - certify suite definitions
   - `StarterTemplate::WrapperPipelineTwoStep`
   - `FamilyHarness` entry
   - registry-order tests that place wrapper between chain3 and the leaves

5. Extend [xtask/src/family/scaffold.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/scaffold.rs).
   Add a truthful wrapper-family starter template that emits the twelve locked starter units and
   aligned smoke-content contract.

6. Extend [xtask/src/lib.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs).
   Add lock tests for:
   - inventory JSON shape
   - harness contract
   - registry routing order
   - starter scaffold file paths
   - smoke content contract
   - promotion artifact schema round-trips

7. Add the committed packet at:

```text
semantic-families/function.wrapper.pipeline.v1/
```

Seed it by lifting the existing wrapper-family fixture corpus already embedded in the chain3
packet, then tighten it to the dedicated wrapper-family contract above.

8. Extend runtime and read-side proof surfaces.
   Touch:
   - [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs)
   - [spec-cli/tests/cli.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/cli.rs)
   - [spec-cli/tests/m14_regressions.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/m14_regressions.rs)

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
    - [semantic-families/README.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/semantic-families/README.md)

## Worktree Parallelization Strategy

M26 has one hard serialization lane and then three safe parallel lanes.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Lock inventory export + artifact schemas | `xtask/src/lib.rs`, `xtask/src/family/inventory.rs`, `xtask/Cargo.toml` | — |
| Lock wrapper family contract + scaffold | `xtask/src/family/harness.rs`, `xtask/src/family/scaffold.rs`, `xtask/src/lib.rs` | Lock inventory export + artifact schemas |
| Curate committed wrapper packet | `semantic-families/function.wrapper.pipeline.v1/` | Lock wrapper family contract + scaffold |
| Add runtime prove/certify tests | `spec-core/src/semantic_review.rs` | Lock wrapper family contract + scaffold |
| Add CLI truth-surface/corpus/regression tests | `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs` | Lock wrapper family contract + scaffold |
| Final command loop + docs | repo-wide commands, `semantic-families/README.md` | packet curation, runtime tests, CLI tests |

### Parallel lanes

- Lane A: `Lock inventory export + artifact schemas` → `Lock wrapper family contract + scaffold`
- Lane B: `Curate committed wrapper packet`
- Lane C: `Add runtime prove/certify tests`
- Lane D: `Add CLI truth-surface/corpus/regression tests`
- Lane E: `Final command loop + docs`

### Execution order

1. Run Lane A first and keep it sequential.
2. Once Lane A is stable, launch Lanes B, C, and D in parallel worktrees.
3. Merge B, C, and D.
4. Run Lane E only after that merge.

### Conflict flags

- Lane A is the serialization point because it fixes suite slug, packet file names, inventory
  fields, and smoke contracts.
- Lanes B, C, and D are safe in parallel because they touch disjoint primary module roots.
- If Lane C changes the wrapper-family unsupported-near-miss boundary, Lane B and Lane D must
  reconcile to that exact shape before Lane E runs.
- Docs stay out of parallel lanes. Do not claim AI-operated promotion until the end-to-end loop is
  green.

## Acceptance Gates

M26 is done only when all of the following are true:

- `Cargo.toml` still lists only `spec-core`, `spec-cli`, and `xtask`
- `cargo xtask family inventory --format json` exists and emits the locked family-scoped truth
- inventory truth shows `function.wrapper.pipeline.v1` as supported but unpromoted before the run
- the only human approvals are target-family approval and final-output approval
- the recommendation artifact exists and cites repo-path evidence
- `function.wrapper.pipeline.v1` is registered in `xtask/src/family/harness.rs`
- `StarterTemplate::WrapperPipelineTwoStep` exists and is used by the harness
- a committed packet exists at `semantic-families/function.wrapper.pipeline.v1/`
- `cargo xtask family smoke function.wrapper.pipeline.v1` passes
- `cargo xtask family prove function.wrapper.pipeline.v1` passes
- `cargo xtask family certify function.wrapper.pipeline.v1` passes
- `promotion.execution.json` references real `prove.latest.json` and `certification.report.json`
- `blocker.report.json` exists as a tested honest termination path even though the milestone exits
  through the green path
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

- `family inventory` is new, and the quickest bad version is an incomplete one. Inventory must be
  tested as carefully as any gate command.
- `function.wrapper.pipeline.v1` is the right first target, but it may expose real routing
  pressure against chain3. If so, certify must force an explicit answer.
- current proof failures still lean on stderr strings in some paths. M26 may need sharper
  structured blocker mapping than the current reports expose.
- lifting wrapper fixtures out of the chain3 packet risks accidental drift if the dedicated packet
  and chain3 packet stop agreeing on shared helper semantics. Keep the dedicated wrapper packet
  authoritative for `function.wrapper.pipeline.v1`.

## Review Summary

- Step 0: Scope Challenge — accepted with one deliberate reduction: **no new workspace crate**
- Architecture Review: 3 major decisions locked
- Code Quality Review: 2 abstraction reductions locked
- Test Review: diagram produced, 11 required coverage targets identified
- Performance Review: 2 operational constraints locked
- NOT in scope: written
- What already exists: written
- Failure modes: 2 critical gaps flagged
- Outside voice: skipped, no separate external model pass was run for this rewrite
- Parallelization: 5 lanes, 3 parallel after 1 serialization lane
- Lake Score: 6/6 recommendations chose the more complete option over the shortcut

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Keep M26 as the workflow bridge between the Rust wedge and broad Rust coverage | Mechanical | Completeness | This is the next real bottleneck after M24, not a side quest | Re-opening semantic-theory scope |
| 2 | CEO | Limit human approvals to candidate family and final output | Mechanical | Explicit over clever | Hidden mid-loop rescue work would fake the operator model | Manual steering between gates |
| 3 | CEO | Lock `function.wrapper.pipeline.v1` as the first live proof target | Taste | Pragmatic | It broadens topology with existing repo truth and existing wedge evidence | Another leaf-family proof |
| 4 | Eng | Reject a new `spec-orchestrator` workspace crate in M26 | Taste | Minimal diff | The current workspace and `xtask` boundary already fit the problem | Adding a fourth workspace member |
| 5 | Eng | Add `cargo xtask family inventory --format json` as the minimal repo-truth export | Mechanical | Explicit over clever | Inventory exposes truth without embedding ranking policy | Ranking inside `xtask` |
| 6 | Eng | Add a dedicated `StarterTemplate::WrapperPipelineTwoStep` | Mechanical | Completeness | Generic placeholder starters are not truthful enough for a promoted family | Reusing `GenericPlaceholder` |
| 7 | Eng | Reuse chain3 packet-local wrapper fixtures as the wrapper packet seed | Mechanical | DRY | The repo already has truthful wrapper-family bucket material | Rebuilding the packet from scratch |
| 8 | Eng | Keep `xtask` as the hard proof kernel and make promotion artifacts reference its reports | Mechanical | Systems over heroes | One proof source keeps the operator loop auditable | Competing proof artifacts |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/autoplan` | Scope & strategy | 1 | CLEAR | Kept M26 narrow as the workflow bridge, not a ranking or multi-language milestone. Locked wrapper pipeline as the first proof target and rejected a new crate. |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | No separate external-model review was run in this pass. The plan was rewritten from direct repo inspection instead. |
| Eng Review | `/autoplan` | Architecture & tests (required) | 1 | CLEAR | Locked the `xtask` boundary, the `family inventory` export, the wrapper-family packet contract, the command loop, the coverage map, and worktree parallelization. |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | No UI scope in M26. |

**VERDICT:** CEO + ENG CLEARED — `PLAN.md` is now an implementation-ready M26 execution
contract. The main change from the draft is explicit: do **not** add a standalone
`spec-orchestrator` crate in M26.
