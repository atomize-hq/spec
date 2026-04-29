# M24 - Direct Sibling Leaf Promotion (`function.arithmetic_leaf.monotone_up.v1`)

Status: **fresh implementation plan for M24**  
Base branch: **main**  
Last rewritten: **2026-04-29**

Source of truth for this plan:

- repo code and docs in this checkout
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-codex-m23-contract-design-20260429-112959.md`

M23 proved the family-promotion workflow on one real arithmetic leaf family:
`function.arithmetic_leaf.monotone_down_nonnegative.v1`.

M24 should prove the same workflow on its direct sibling:
`function.arithmetic_leaf.monotone_up.v1`.

This is intentionally narrow. M24 is not an ergonomics rewrite, not a multi-family burst, and not
a generic harness redesign. It is the second real leaf-family promotion, using the already-shipped
M23 harness shape unless `monotone_up` reveals a concrete defect.

## Problem Statement

The runtime semantic reviewer already recognizes `pricing/apply_tax` as
`function.arithmetic_leaf.monotone_up.v1`, and the repo already has wedge-style aligned/drift/
under-specified coverage for that canonical unit.

What is still missing is the promoted-family contract:

1. there is no registered `FamilyHarness` entry for `function.arithmetic_leaf.monotone_up.v1`
2. there is no packet at `semantic-families/function.arithmetic_leaf.monotone_up.v1/`
3. there are no locked `family smoke`, `family prove`, or `family certify` surfaces for this id
4. the repo therefore still relies on runtime truth alone instead of a maintainer-owned packet and
   certification path

M24 closes that gap and nothing else.

## Milestone Outcome

When M24 is complete, the repo can truthfully claim:

- promoted `kind:function` families now include one wrapper family and two sibling arithmetic leaf
  families
- the promotion workflow is not specific to chain-style wrappers
- the promotion workflow is not specific to monotone-down clamp semantics

M24 should not claim:

- that family promotion is now solved generically for all future function families
- that packet authoring ceremony is settled
- that a non-author maintainer dry run is complete
- that non-function family promotion is now in scope

## Scope

### In scope

- register `function.arithmetic_leaf.monotone_up.v1` in `xtask/src/family/harness.rs`
- add a real packet under `semantic-families/function.arithmetic_leaf.monotone_up.v1/`
- make `cargo xtask family new/smoke/prove/certify function.arithmetic_leaf.monotone_up.v1`
  work end-to-end
- add explicit regression protection so `chain3` and
  `function.arithmetic_leaf.monotone_down_nonnegative.v1` do not regress or get shadowed
- update repo-truth docs that still say `monotone_up` is unregistered

### NOT in scope

- any UI or frontend work
- design system or interaction changes
- new semantic family vocabulary in `spec-core`
- packet-ceremony reduction beyond what is required for truthful M24 landing
- non-author maintainer validation
- simultaneous promotion of another family
- broad refactors to `xtask` unrelated to `monotone_up`

## Premises

1. `pricing/apply_tax` is already the canonical truthful seed for this family.
2. The M23 harness shape is the default answer unless `monotone_up` exposes a concrete mismatch.
3. The sibling proof matters more than any local cleanup opportunity exposed during implementation.
4. Routing and anti-shadow metadata must stay explicit in the packet and in the Rust harness.
5. Packet-local fixtures should remain self-contained, even when they reuse the same helper shape
   as the canonical example.

## CEO Review

### Why this milestone now

M23 retired the “wrapper-only harness” objection.

The next real risk is narrower: was M23 a one-off leaf proof specific to nonnegative clamp
semantics, or can the exact workflow promote the sibling arithmetic leaf family whose direction is
the opposite?

M24 should answer that directly instead of widening into ceremony cleanup.

### Alternatives rejected

- `chain4` first: weaker proof because it stays too close to chain3
- ceremony cleanup first: optimizes a workflow that still only has one leaf-family proof
- non-function promotion first: too much scope, wrong sequencing
- promote both monotone leaf families together: removes the clean sibling-proof read

### Decision

Promote exactly one new family:

- `function.arithmetic_leaf.monotone_up.v1`

## Design Review Note

There is no UI scope in the provided design doc or in the repo surfaces touched by M24.

Because of that, the design phase produces no separate visual/design artifact. M24 remains a pure
engineering milestone with a narrow packet and certification contract.

## What Already Exists

| Area | Current truth | Why it matters for M24 |
|---|---|---|
| Runtime family vocabulary | `spec-core/src/semantic_review.rs` already ships `function.arithmetic_leaf.monotone_up.v1` as a supported function family | M24 should freeze existing truth, not invent new runtime semantics |
| Canonical aligned unit | `examples/ecommerce/units/pricing/apply_tax.unit.spec` | Seed for aligned packet truth |
| Existing wedge coverage | `spec-cli/tests/m14_regressions.rs` already has aligned, drift, under-specified, and clamp-drift `apply_tax` wedges | Reuse as prove/certify inputs instead of inventing new ad hoc coverage |
| Existing promoted sibling leaf packet | `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/**` | The closest implementation model; M24 should mirror its workflow shape |
| Existing promoted wrapper packet | `semantic-families/function.wrapper.pipeline.chain3.v1/**` | Non-regression baseline for routing and certification |
| Registered family harnesses | `xtask/src/family/harness.rs` currently registers `chain3` and `monotone_down_nonnegative` only | Explains why `monotone_up` is still blocked today |
| Maintainer workflow | `cargo xtask family new`, `smoke`, `prove`, `certify` exist and are already hardened by M23 | M24 should reuse them with the minimum new family-specific wiring |

## Architecture / Dependency View

```text
examples/ecommerce/units/pricing/apply_tax.unit.spec
        │
        ├── semantic meaning already classified in:
        │     spec-core/src/semantic_review.rs
        │
        ├── family registration must be added in:
        │     xtask/src/family/harness.rs
        │
        ├── packet scaffold uses:
        │     xtask/src/family/scaffold.rs
        │     xtask/src/family/manifest.rs
        │
        ├── committed packet lives at:
        │     semantic-families/function.arithmetic_leaf.monotone_up.v1/
        │
        ├── prove wiring runs through:
        │     xtask/src/family/prove.rs
        │
        ├── certify wiring runs through:
        │     xtask/src/family/certify.rs
        │
        └── regression proof lives in:
              spec-core/src/semantic_review.rs
              spec-cli/tests/m14_regressions.rs
              xtask/src/lib.rs
```

### Modules expected to change for M24

- `xtask/src/family/harness.rs`
  Adds the registered family definition, suite slug, routing metadata, scaffold template choice,
  and prove/certify suite ownership.
- `xtask/src/family/scaffold.rs`
  Extends starter-template support so `family new` can emit truthful `monotone_up` starter units
  instead of cloning the monotone-down wording.
- `xtask/src/lib.rs`
  Adds or updates harness-lock tests, scaffold/smoke tests, and suite-ownership tests for the new
  family.
- `spec-core/src/semantic_review.rs`
  Adds promoted-family classifier/regression tests. Runtime semantics should change only if the new
  packet reveals a real mismatch.
- `spec-cli/tests/m14_regressions.rs`
  Adds truth-surface, corpus, and regression tests for the promoted `monotone_up` packet.
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`
  New packet source of truth.
- `semantic-families/README.md`
  Must stop listing `monotone_up` as unregistered once the packet lands.

## Locked M24 Family Contract

M24 should explicitly lock the new family shape instead of leaving the sibling details to
implementer taste.

### Family id

- `function.arithmetic_leaf.monotone_up.v1`

### Summary

- `Straight-line arithmetic leaf with zero-or-one helper dep and monotone-up semantics.`

### Canonical aligned authored truth

- unit id: `pricing/apply_tax_aligned`
- semantic seed: `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- inputs: `subtotal`, `rate`
- invariant: `output >= subtotal`
- helper dep shape: optional `money/round`
- executable body shape: straight-line arithmetic, no loops, no branching

### Locked routing metadata

M24 should freeze the family as the last registered supported function family before the terminal
unsupported catch-all.

- `precedence = 4`
- `must_not_shadow = ["unsupported.function.v1"]`

Reasoning:

- `chain3` already locks `must_not_shadow` against both leaf siblings
- `monotone_down_nonnegative` already locks `must_not_shadow = ["function.arithmetic_leaf.monotone_up.v1"]`
- `monotone_up` is the remaining tail family in the current shipped runtime order, so its only
  honest explicit successor is the terminal unsupported catch-all

### Locked suite slug

- `monotone_up_`

Every prove/certify suite name and every expected test name for this family must include that slug.

### Locked starter corpus paths

- `fixtures/aligned/units/pricing/apply_tax_aligned.unit.spec`
- `fixtures/drift/units/pricing/apply_tax_drift.unit.spec`
- `fixtures/under_specified/units/pricing/apply_tax_under_specified.unit.spec`
- `fixtures/unsupported_near_miss/units/pricing/apply_tax_control_flow_unsupported_near_miss.unit.spec`

Each bucket should also carry a packet-local helper:

- `fixtures/<bucket>/units/money/round_<bucket>.unit.spec`

The helper stays intentionally boring. It exists only to preserve the truthful zero-or-one helper
dep contract already used by the canonical `apply_tax` example.

## Corpus Design

### Aligned

Lift `examples/ecommerce/units/pricing/apply_tax.unit.spec` into packet-local form with the same
semantic claim: add tax to a subtotal using a decimal rate, preserve `output >= subtotal`, and
route through the optional `money/round` helper.

### Drift

Reuse the existing M14 drift shape already modeled by `rewrite_apply_tax_as_drift(...)`:

- authored story still says tax increases the subtotal
- executable body instead subtracts and clamps
- expected verdict remains `semantic_drift`

### Under Specified

Reuse the existing M14 vague-truth wedge:

- aligned executable body
- `intent.why = "todo"`
- expected verdict remains `under_specified`

### Unsupported Near Miss

Add an explicit control-flow near miss for the tax family instead of reusing clamp drift.

Locked choice:

- keep the authored monotone-up contract
- keep the optional `money/round` helper dep
- introduce a branch-based fast path in the leaf body

Example shape:

```text
let taxed = subtotal + subtotal * rate;
if rate == Decimal::ZERO {
    subtotal
} else {
    round(taxed)
}
```

Why this choice:

- it is a true near miss for the family, not generic filler
- it exercises unsupported control flow directly
- it keeps the proof focused on the family boundary rather than on helper topology

## Prove / Smoke / Certify Contract

### Smoke

Required command:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
```

Smoke should verify scaffold-owned surfaces only:

- `family.toml` regenerates byte-for-byte
- the four locked starter cases reappear in the correct bucket paths
- the aligned starter spec reads like the tax family, not the discount family
- the aligned starter spec includes `subtotal`, `rate`, `output >= subtotal`, optional
  `money/round`, and a straight-line tax body

### Prove

Required command:

```bash
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
```

Locked suite families for prove:

- `spec-core:monotone_up_classifier_`
- `spec-cli:monotone_up_truth_surface_`
- `spec-cli:monotone_up_corpus_`

Expected prove coverage:

- classifier aligned fixture routes to promoted leaf
- classifier drift fixture reports semantic drift
- classifier under-specified fixture reports vague truth
- classifier unsupported near miss stays unsupported
- truth surface command matrix preserves review until `spec test` refresh
- stale status/export preserve last proven review
- corpus aligned projects `valid`
- corpus drift projects `failing`
- corpus under-specified projects `incomplete`
- corpus unsupported near miss stays additive-only and health-neutral

### Certify

Required command:

```bash
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
```

Locked suite families for certify:

- `spec-core:monotone_up_regression_`
- `spec-cli:monotone_up_regression_`

Expected certify coverage:

- `chain3` still routes to `function.wrapper.pipeline.chain3.v1`
- `apply_discount` still routes to `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `apply_tax` still routes to `function.arithmetic_leaf.monotone_up.v1`
- unsupported near misses for the promoted family stay additive-only and do not contaminate status
  or export surfaces

## Test / Coverage Matrix

```text
family smoke
  proves scaffold truth only
  no curated packet diff requirement

family prove
  Gate A: classifier truth
  Gate B: packet corpus truth
  Gate C: status/export/passport truth-surface preservation

family certify
  Gate A/B/C from prove stay green
  Gate D: regression and routing coherence stay green
```

Explicit M24 regression expectations:

- `chain3` remains green before and after `monotone_up` registration
- `monotone_down_nonnegative` remains green before and after `monotone_up` registration
- `monotone_up` does not require hidden family-specific edits outside the harness contract,
  scaffold template support, packet-local authoring, and test registration
- stale-review preservation still behaves identically for the promoted family

## Failure Modes

### Routing / shadowing mistakes

- wrong `precedence` or `must_not_shadow` can silently make the new packet lie about runtime order
- wrong routing can also weaken existing `chain3` or `monotone_down_nonnegative` guarantees

### Template leakage from M23

- if scaffold output still reads like discount semantics, the packet starts fake
- if scaffold output forgets the helper dep, the packet no longer proves the intended dep-topology
  range

### Unsupported-family boundary erosion

- if the new unsupported near miss lands as supported drift or aligned, the family boundary became
  fuzzy and the packet is not trustworthy

### False-green suite ownership

- if suite/test names are not locked under `monotone_up_`, `prove` and `certify` can accidentally
  borrow unrelated tests

### Read-side truth regressions

- if status/export no longer preserve the last proven review on stale authored truth, the promoted
  family diverges from the repo’s existing semantic-review contract

## Non-Regression Concerns

### `chain3`

`chain3` is still the wrapper-family baseline and the strongest regression canary for routing and
registry coherence. M24 must not:

- mutate its packet-local manifest truth
- break its prove/certify suite ownership
- change its locked routing order behavior

### `monotone_down_nonnegative`

This is the closest sibling and the most likely accidental casualty. M24 must not:

- reuse the discount packet paths or wording for the tax family
- let `apply_tax` shadow `apply_discount`
- weaken the existing packet-local helper-dep shape
- regress the additive-only unsupported-near-miss behavior already locked in M23

## Implementation Sequence

1. Add the new `FamilyHarness` entry and locked constants in `xtask/src/family/harness.rs`.
2. Extend `xtask/src/family/scaffold.rs` with a truthful `monotone_up` starter template.
3. Add xtask lock tests in `xtask/src/lib.rs` for registration, routing, scaffold, smoke, and
   suite ownership.
4. Generate the starter packet via `cargo xtask family new function.arithmetic_leaf.monotone_up.v1`.
5. Curate the packet into committed form:
   `candidate.md`, `family.toml`, packet-local helper units, and the four bucket fixtures.
6. Add prove/certify test surfaces in `spec-core/src/semantic_review.rs` and
   `spec-cli/tests/m14_regressions.rs`.
7. Update `semantic-families/README.md` so repo truth no longer calls `monotone_up` unregistered.
8. Run `family smoke`, `family prove`, and `family certify` for the new family.
9. Re-run the sibling and wrapper regressions if any routing or suite-ownership failure appeared
   during M24 work.

## Parallelization Guidance

M24 is not fully sequential, but it is also not a free-for-all. The harness contract is the choke
point. Until that is locked, parallel work just creates fake motion and merge conflict risk.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Lock family contract | `xtask/src/family/harness.rs`, `xtask/src/family/scaffold.rs` | — |
| Add xtask lock tests | `xtask/src/lib.rs` | Lock family contract |
| Curate committed packet | `semantic-families/function.arithmetic_leaf.monotone_up.v1/` | Lock family contract |
| Add runtime classifier / regression coverage | `spec-core/src/semantic_review.rs` | Lock family contract |
| Add CLI truth-surface / corpus coverage | `spec-cli/tests/m14_regressions.rs` | Lock family contract |
| Final smoke / prove / certify loop | repo-wide command execution | xtask lock tests, packet curation, runtime coverage, CLI coverage |
| Repo-truth docs update | `semantic-families/README.md`, plan-adjacent docs if needed | Final smoke / prove / certify loop |

### Parallel lanes

- Lane A: `Lock family contract` → `Add xtask lock tests`
- Lane B: `Curate committed packet`
- Lane C: `Add runtime classifier / regression coverage`
- Lane D: `Add CLI truth-surface / corpus coverage`
- Lane E: `Final smoke / prove / certify loop` → `Repo-truth docs update`

### Execution order

1. Run Lane A first. Nothing else should start until the family id, routing metadata, suite slug,
   starter corpus, and scaffold truth are locked.
2. Launch Lanes B, C, and D in parallel once Lane A is merged or otherwise stable.
3. Run Lane E only after B, C, and D are all complete.

### Conflict flags

- Lanes B, C, and D are safe to parallelize because they touch disjoint primary module roots.
- Lane A must stay sequential because it defines the packet paths, suite slug, and routing
  metadata consumed by every other lane.
- If runtime coverage in Lane C reveals a routing mismatch, stop Lane E and resolve the runtime vs.
  harness disagreement before trusting any green certify result.

## Acceptance Gates

M24 is done only when all of the following are true:

- `function.arithmetic_leaf.monotone_up.v1` is registered in `xtask/src/family/harness.rs`
- a committed packet exists at `semantic-families/function.arithmetic_leaf.monotone_up.v1/`
- `cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1` passes
- `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1` passes
- `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1` passes
- `chain3` remains green and unshadowed
- `monotone_down_nonnegative` remains green and unshadowed
- repo docs no longer describe `monotone_up` as unregistered

## Unresolved Risks

- The new unsupported near-miss shape is locked in this plan, but the runtime classifier may reveal
  a sharper sibling-specific unsupported boundary during implementation. If so, change the packet,
  not the milestone scope.
- `precedence = 4` is the most honest current choice from shipped routing order, but any runtime
  mismatch discovered in certification must be resolved by making the harness and runtime agree
  explicitly, not by weakening tests.
- Packet-local helper units add maintenance overhead. That is acceptable in M24 because truthful
  self-contained packets matter more than lighter ceremony.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | Narrow sibling-proof milestone selected via `/office-hours`; no separate CEO review log recorded yet |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | No separate codex plan review log recorded yet |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 0 | — | This plan is written to eng-review rigor, but no formal eng review log has been recorded yet |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | No UI scope in M24 |

**VERDICT:** NO FORMAL REVIEW LOGS YET — plan is freshly rewritten for M24, but `/autoplan` and `/plan-eng-review` logs have not been persisted in the repo review dashboard yet.
