<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/codex-m23-contract-autoplan-restore-20260429-114932.md -->
# M24 - Direct Sibling Leaf Promotion (`function.arithmetic_leaf.monotone_up.v1`)

Status: **implementation plan**  
Base branch: **main**  
Working branch: **codex/m23-contract**  
Last rewritten: **2026-04-29**

Source of truth for this plan:

- repo code and docs in this checkout
- [PLAN.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md)
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-codex-m23-contract-design-20260429-112959.md`

M23 proved the packet workflow can promote one real arithmetic leaf family:
`function.arithmetic_leaf.monotone_down_nonnegative.v1`.

M24 proves the next narrow thing and nothing more: the same workflow promotes its direct sibling
`function.arithmetic_leaf.monotone_up.v1`, whose semantics point in the opposite direction.

This is not a harness redesign. It is not a packet-ceremony cleanup. It is not a maintainer-UX
milestone. It is one more real family packet, promoted through the existing registry-first
workflow.

## Problem Statement

The runtime semantic reviewer already recognizes `pricing/apply_tax` as
`function.arithmetic_leaf.monotone_up.v1`, and the repo already contains wedge-style aligned,
drift, under-specified, and negative-shape coverage for that behavior in
`spec-cli/tests/m14_regressions.rs` and `spec-core/src/semantic_review.rs`.

What is still missing is the promoted-family contract:

1. `xtask/src/family/harness.rs` does not register a `FamilyHarness` for
   `function.arithmetic_leaf.monotone_up.v1`.
2. `xtask/src/family/scaffold.rs` has no monotone-up starter template.
3. there is no committed packet at
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/`.
4. there are no family-owned `smoke`, `prove`, or `certify` suites locked to a
   `monotone_up_` slug.
5. repo truth still says `monotone_up` is unregistered in `semantic-families/README.md`.

Until those five gaps close, the family exists only as runtime truth, not as a maintainer-owned,
packetized, certifiable family.

## Milestone Outcome

When M24 is done, the repo can truthfully claim:

- promoted `kind:function` families include one wrapper family and two sibling arithmetic leaf
  families
- the family-promotion workflow is not specific to wrapper semantics
- the family-promotion workflow is not specific to monotone-down clamp behavior
- `cargo xtask family new/smoke/prove/certify function.arithmetic_leaf.monotone_up.v1` works
  through the same registry-first path as the two already-promoted families

M24 does **not** claim:

- future family promotion is now generic
- packet authoring ceremony is settled
- non-author maintainer usability is proven
- non-function families are now in scope
- the runtime semantic-review architecture needs a broader redesign

## Scope

### In scope

- register `function.arithmetic_leaf.monotone_up.v1` in `xtask/src/family/harness.rs`
- add a monotone-up starter template in `xtask/src/family/scaffold.rs`
- add xtask lock tests in `xtask/src/lib.rs` for registration, routing, scaffold truth, smoke
  truth, and suite ownership
- add a committed packet at `semantic-families/function.arithmetic_leaf.monotone_up.v1/`
- add prove/certify coverage in `spec-core/src/semantic_review.rs`
- add truth-surface, corpus, and read-side regression coverage in
  `spec-cli/tests/m14_regressions.rs`
- update repo-truth docs that still say `monotone_up` is unregistered

### NOT in scope

- any UI or frontend work
- design-system or interaction changes
- new runtime semantic-family vocabulary
- a generic template abstraction for future arithmetic leaves beyond what M24 directly needs
- packet-ceremony reduction
- non-author maintainer dry run
- simultaneous promotion of another family
- broad `xtask` refactors unrelated to `monotone_up`

## Why This Milestone Now

M23 retired the obvious objection, that the workflow only works for wrapper families.

The next honest question is smaller and sharper: was the leaf promotion path a one-off that only
works for monotone-down clamp semantics, or can it promote the sibling monotone-up family with the
same registry-first workflow?

That is the whole game here. Do the sibling proof cleanly. Do not spend the milestone flattering
ourselves with ergonomics cleanup.

## Premises

1. `pricing/apply_tax` is already the canonical truthful seed for this family.
2. The M23 harness shape is the default answer unless `monotone_up` exposes a concrete mismatch.
3. The sibling proof matters more than any local cleanup opportunity found during implementation.
4. Routing and anti-shadow metadata must stay explicit in both the Rust harness and `family.toml`.
5. Packet-local helper units are acceptable ceremony in M24 because truthful self-contained
   packets matter more than reducing authoring friction.

## What Already Exists

| Area | Current truth | Reuse decision |
|---|---|---|
| Runtime family vocabulary | `spec-core/src/semantic_review.rs` already emits `function.arithmetic_leaf.monotone_up.v1` for `pricing/apply_tax` | Reuse existing runtime truth. Do not invent new semantics. |
| Canonical seed | `examples/ecommerce/units/pricing/apply_tax.unit.spec` | Lift directly into packet-aligned form. |
| Existing wedge coverage | `spec-cli/tests/m14_regressions.rs` already has `rewrite_apply_tax_as_drift`, `rewrite_apply_tax_as_under_specified`, and clamp-style drift coverage | Reuse the wedge style and add the one missing unsupported-near-miss helper for promoted-family coverage. |
| Existing promoted sibling leaf | `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/**` | Mirror structure, commands, and ownership rules. |
| Existing promoted wrapper | `semantic-families/function.wrapper.pipeline.chain3.v1/**` | Keep as the routing and certify regression canary. |
| Existing harness workflow | `xtask/src/family/{harness,scaffold,prove,certify,smoke}.rs` | Extend. No new command family. |
| Existing registry order contract | `xtask/src/family/harness.rs` sorts registered families by explicit precedence and then appends `unsupported.function.v1` | M24 must fit into that order, not bypass it. |
| Existing doc truth | `semantic-families/README.md` explicitly says `monotone_up` is unregistered | Update when the packet is real and green. |

## Minimal Change Strategy

This plan stays boring on purpose.

- No new crates.
- No new top-level commands.
- No new generic orchestration layer.
- No new packet format.
- No new family vocabulary.

The minimum honest diff is:

1. extend the registry and scaffold for one more concrete family
2. add one committed packet
3. add family-owned tests for smoke, prove, certify, and read-side regression
4. update README truth

Anything broader is scope creep.

## Architecture / Dependency View

```text
examples/ecommerce/units/pricing/apply_tax.unit.spec
        │
        ├── already classifies in runtime review:
        │     spec-core/src/semantic_review.rs
        │
        ├── must be registered in family workflow:
        │     xtask/src/family/harness.rs
        │
        ├── must be scaffoldable through:
        │     xtask/src/family/scaffold.rs
        │
        ├── must be represented as committed packet:
        │     semantic-families/function.arithmetic_leaf.monotone_up.v1/**
        │
        ├── must be provable through:
        │     xtask/src/family/prove.rs
        │     xtask/src/family/report.rs
        │
        ├── must be certifiable through:
        │     xtask/src/family/certify.rs
        │
        ├── must preserve read-side truth via:
        │     spec-cli/tests/m14_regressions.rs
        │
        └── must not regress sibling or wrapper routing:
              spec-core/src/semantic_review.rs
              spec-cli/tests/m14_regressions.rs
              semantic-families/README.md
```

### Primary module boundaries

| Module root | Responsibility in M24 | Must not happen |
|---|---|---|
| `xtask/src/family/harness.rs` | register the family, lock routing metadata, lock suite ownership, lock scaffold contract | do not generalize the registry model |
| `xtask/src/family/scaffold.rs` | emit truthful monotone-up starter units | do not turn this into a generic DSL |
| `xtask/src/lib.rs` | lock invariants with xtask-focused tests | do not hide behavior behind helper indirection that weakens the tests |
| `semantic-families/function.arithmetic_leaf.monotone_up.v1/**` | hold committed packet truth | do not depend on external units |
| `spec-core/src/semantic_review.rs` | preserve runtime classification and routing truth | do not widen supported-family semantics |
| `spec-cli/tests/m14_regressions.rs` | preserve status/export/passport/read-side truth | do not borrow unrelated test prefixes |
| `semantic-families/README.md` | tell the truth about what is promoted and how smoke works | do not claim registration before green commands exist |

## Locked Family Contract

M24 does not leave the family shape to implementer taste. These values are fixed by the plan.

### Family id

- `function.arithmetic_leaf.monotone_up.v1`

### Summary

- `Straight-line arithmetic leaf with zero-or-one helper dep and monotone-up semantics.`

### Locked routing metadata

- `precedence = 4`
- `must_not_shadow = ["unsupported.function.v1"]`

Reasoning:

- `chain3` already owns precedence `1`
- `monotone_down_nonnegative` already owns precedence `3`
- runtime truth already knows about `monotone_up`
- the remaining honest successor after `monotone_up` is the terminal unsupported catch-all

### Locked suite slug

- `monotone_up_`

Every family-owned prove/certify suite name and every expected test name for this family must
include that slug.

### Locked scaffold template choice

Add a new explicit starter template variant in `xtask/src/family/harness.rs` and
`xtask/src/family/scaffold.rs`:

- `StarterTemplate::ArithmeticLeafMonotoneUp`

This is intentionally concrete. Do not collapse it into a generic arithmetic-leaf template in M24.

### Canonical aligned authored truth

- unit id: `pricing/apply_tax_aligned`
- seed file: `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- inputs: `subtotal`, `rate`
- invariant: `output >= subtotal`
- helper dep shape: optional `money/round`
- executable body shape:

```text
{
    let taxed = subtotal + subtotal * rate;
    round(taxed)
}
```

## Scaffold Truth vs Committed Packet Truth

This boundary must stay explicit.

### Scaffold-owned truth

`cargo xtask family new` and `cargo xtask family smoke` own only these surfaces:

- `family.toml`
- `fixtures/<bucket>/Cargo.toml`
- `fixtures/<bucket>/src/main.rs`
- the four locked starter pricing units:
  - `fixtures/aligned/units/pricing/apply_tax_aligned.unit.spec`
  - `fixtures/drift/units/pricing/apply_tax_drift.unit.spec`
  - `fixtures/under_specified/units/pricing/apply_tax_under_specified.unit.spec`
  - `fixtures/unsupported_near_miss/units/pricing/apply_tax_control_flow_unsupported_near_miss.unit.spec`

Smoke does **not** require whole-packet byte equality.

### Committed-packet truth

The committed packet adds maintainer-authored surfaces beyond scaffold output:

- `candidate.md`
- one packet-local helper per bucket:
  - `fixtures/aligned/units/money/round_aligned.unit.spec`
  - `fixtures/drift/units/money/round_drift.unit.spec`
  - `fixtures/under_specified/units/money/round_under_specified.unit.spec`
  - `fixtures/unsupported_near_miss/units/money/round_unsupported_near_miss.unit.spec`
- any bucket-local `Cargo.toml` dependency additions required by those helpers
- aligned/drift/under-specified/unsupported rationale in `candidate.md`

That distinction is important because a non-empty diff after deleting and regenerating the
committed packet is still honest. Only scaffold-owned surfaces must match scaffold output.

## Packet Contract By Bucket

### Aligned

- lift `examples/ecommerce/units/pricing/apply_tax.unit.spec` into
  `fixtures/aligned/units/pricing/apply_tax_aligned.unit.spec`
- preserve the authored tax story and `output >= subtotal`
- preserve the optional `money/round` helper dep
- add a packet-local `money/round_aligned.unit.spec`

### Drift

- reuse the existing `rewrite_apply_tax_as_drift(...)` shape already present in
  `spec-cli/tests/m14_regressions.rs`
- authored story still says tax increases the subtotal
- executable body instead subtracts and clamps
- expected semantic verdict remains `semantic_drift`
- add a packet-local `money/round_drift.unit.spec`

### Under Specified

- reuse the existing `rewrite_apply_tax_as_under_specified(...)` shape
- keep the aligned executable body
- weaken `intent.why` to `todo`
- expected semantic verdict remains `under_specified`
- add a packet-local `money/round_under_specified.unit.spec`

### Unsupported Near Miss

- add a new control-flow near miss for the tax family instead of reusing clamp drift
- locked body shape:

```text
{
    let taxed = subtotal + subtotal * rate;
    if rate == Decimal::ZERO {
        subtotal
    } else {
        round(taxed)
    }
}
```

- keep the authored monotone-up contract
- keep the optional `money/round` helper dep
- add a packet-local `money/round_unsupported_near_miss.unit.spec`
- expected semantic review stays `unsupported.function.v1` with
  `UnsupportedControlFlow`

This requires one new helper in `spec-cli/tests/m14_regressions.rs`:

- `rewrite_apply_tax_as_unsupported_near_miss(...)`

That helper is part of the planned diff. Do not fake this bucket by reusing clamp drift.

## Smoke / Prove / Certify Contract

### Smoke

Required command:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
```

Smoke must verify scaffold-owned surfaces only:

- `family.toml` regenerates byte-for-byte
- the four locked pricing starter units reappear in the correct bucket paths
- the aligned starter reads like the tax family, not the discount family
- the aligned starter contains all of:
  - `subtotal: Decimal`
  - `rate: Decimal`
  - `- output >= subtotal`
  - `deps:\n  - money/round`
  - `let taxed = subtotal + subtotal * rate;`
  - `round(taxed)`

Smoke must **not** require:

- `candidate.md` equality
- packet-local helper equality
- whole-packet byte equality

### Prove

Required command:

```bash
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
```

Locked prove suites:

- `spec-core:monotone_up_classifier_`
- `spec-cli:monotone_up_truth_surface_`
- `spec-cli:monotone_up_corpus_`

Locked prove expectations:

- `semantic_review::tests::monotone_up_classifier_aligned_fixture_routes_to_promoted_leaf`
- `semantic_review::tests::monotone_up_classifier_drift_fixture_reports_semantic_drift`
- `semantic_review::tests::monotone_up_classifier_under_specified_fixture_reports_vague_truth`
- `semantic_review::tests::monotone_up_classifier_unsupported_near_miss_stays_unsupported`
- `monotone_up_truth_surface_command_matrix_preserves_until_spec_test_refresh`
- `monotone_up_truth_surface_stale_status_and_export_preserve_last_proven_review`
- `monotone_up_corpus_aligned_fixture_projects_valid_state`
- `monotone_up_corpus_drift_fixture_projects_failing_state`
- `monotone_up_corpus_under_specified_fixture_projects_incomplete_state`
- `monotone_up_corpus_unsupported_near_miss_stays_additive_only_and_neutral`

Gate mapping:

- Gate A: runtime classifier truth
- Gate B: packet corpus truth
- Gate C: status/export/passport truth-surface preservation

### Certify

Required command:

```bash
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
```

Locked certify suites:

- `spec-core:monotone_up_regression_`
- `spec-cli:monotone_up_regression_`

Locked certify expectations:

- `semantic_review::tests::monotone_up_regression_chain3_is_not_shadowed`
- `semantic_review::tests::monotone_up_regression_monotone_down_nonnegative_is_not_shadowed`
- `semantic_review::tests::monotone_up_regression_runtime_order_matches_locked_precedence`
- `monotone_up_regression_read_side_surfaces_are_not_shadowed`
- `monotone_up_regression_unsupported_near_miss_stays_additive_only_and_neutral`

Gate D owns only regression and routing coherence. If Gate D fails, fix the harness/runtime
disagreement explicitly. Do not weaken the tests.

## Test / Coverage Diagram

The plan targets 100% coverage for the new codepaths. There are no intentional gaps.

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/harness.rs
    │
    ├── register monotone_up family
    │   └── [PLANNED TEST] xtask lock test proves registry membership
    │
    ├── lock precedence=4 and must_not_shadow=["unsupported.function.v1"]
    │   ├── [PLANNED TEST] xtask routing-order lock test
    │   └── [PLANNED TEST] spec-core runtime-order regression
    │
    ├── lock suite slug monotone_up_
    │   └── [PLANNED TEST] xtask suite-ownership rejection test
    │
    └── lock smoke contract strings
        └── [PLANNED TEST] xtask smoke acceptance test

[+] xtask/src/family/scaffold.rs
    │
    ├── StarterTemplate::ArithmeticLeafMonotoneUp
    │   └── [PLANNED TEST] family new emits truthful tax starter
    │
    └── smoke rejects template leakage from monotone_down
        └── [PLANNED TEST] aligned starter content contract

[+] semantic-families/function.arithmetic_leaf.monotone_up.v1/**
    │
    ├── aligned packet truth
    ├── drift packet truth
    ├── under-specified packet truth
    └── unsupported-near-miss packet truth
        └── [PLANNED TEST] prove/corpus suites execute committed packet fixtures

[+] spec-core/src/semantic_review.rs
    │
    ├── aligned routes to promoted leaf
    ├── drift reports semantic_drift
    ├── under_specified reports vague truth
    ├── unsupported near miss stays unsupported
    ├── chain3 not shadowed
    ├── monotone_down_nonnegative not shadowed
    └── runtime order matches locked precedence

[+] spec-cli/tests/m14_regressions.rs
    │
    ├── truth-surface preservation until spec test refresh
    ├── stale status/export preserve last proven review
    ├── aligned corpus projects valid
    ├── drift corpus projects failing
    ├── under_specified corpus projects incomplete
    ├── unsupported near miss stays additive-only and health-neutral
    └── read-side regression surfaces are not shadowed

─────────────────────────────────
COVERAGE TARGET: 100%
GAPS ACCEPTED: 0
E2E/EVAL REQUIRED: 0
─────────────────────────────────
```

## Failure Modes

| Codepath | Real failure | Detection | Maintainer-visible effect | Critical? |
|---|---|---|---|---|
| Harness routing | wrong precedence or `must_not_shadow` lets `monotone_up` shadow or get shadowed | xtask routing locks + `monotone_up_regression_*` | green packet with dishonest routing | Yes |
| Scaffold template | starter output still reads like discount semantics | smoke contract + xtask scaffold locks | maintainer trusts a fake starter packet | Yes |
| Packet helper topology | aligned packet forgets `money/round` helper or externalizes it | corpus aligned fixture + packet review | family no longer proves the intended zero-or-one helper shape | Medium |
| Unsupported boundary | control-flow near miss accidentally routes as supported | classifier prove suite + corpus unsupported suite | family boundary becomes fuzzy and future packets learn the wrong rule | Yes |
| Suite ownership | prove/certify suites do not use `monotone_up_` slug and borrow unrelated tests | xtask suite-ownership tests | false-green certification | Yes |
| Truth-surface preservation | stale status/export stop preserving last proven review | truth-surface suite | promoted family diverges from repo read-side contract | Yes |

## Performance / Operational Notes

There is no production runtime performance risk here. This milestone changes maintainer workflow and
test surfaces, not end-user codepaths.

The only meaningful performance concern is test/runtime sprawl:

- keep all new tests under the `monotone_up_` prefix
- keep prove/certify commands targeted to those prefixes
- do not add repo-wide integration loops when existing targeted suite shapes already exist

That keeps the blast radius small and the command cost proportional to one promoted family.

## Distribution / Ship Surface

No new distributable artifact is introduced.

The maintainer interface remains:

- `cargo xtask family new <family>`
- `cargo xtask family smoke <family>`
- `cargo xtask family prove <family>`
- `cargo xtask family certify <family>`

The only ship-surface update required is repo truth:

- `semantic-families/README.md` must stop calling `monotone_up` unregistered once all commands are
  green

## Implementation Sequence

1. Extend `xtask/src/family/harness.rs`.
   Add:
   - `MONOTONE_UP_PRECEDENCE = 4`
   - `MONOTONE_UP_MUST_NOT_SHADOW = ["unsupported.function.v1"]`
   - `MONOTONE_UP_SUITE_SLUG = "monotone_up_"`
   - prove suite definitions
   - certify suite definitions
   - `StarterTemplate::ArithmeticLeafMonotoneUp`
   - `FamilyHarness` entry
   - registry inclusion in `FAMILY_REGISTRY`

2. Extend `xtask/src/family/scaffold.rs`.
   Add the monotone-up starter template with the locked aligned/drift/under-specified/unsupported
   pricing starter files.

3. Extend `xtask/src/lib.rs`.
   Add lock tests for:
   - harness contract
   - registry routing order
   - starter scaffold file paths
   - smoke content contract
   - suite ownership and slug enforcement

4. Generate the starter packet.

```bash
cargo xtask family new function.arithmetic_leaf.monotone_up.v1
```

5. Curate the committed packet at
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/`.
   Add:
   - `candidate.md`
   - packet-local `money/round_*` helper units in all four buckets
   - any bucket-local dependency adjustments needed by those helpers

6. Extend `spec-core/src/semantic_review.rs`.
   Add the prove/certify monotone-up tests with the exact locked names above.

7. Extend `spec-cli/tests/m14_regressions.rs`.
   Add:
   - `rewrite_apply_tax_as_unsupported_near_miss(...)`
   - monotone-up truth-surface tests
   - monotone-up corpus tests
   - monotone-up read-side regression tests

8. Update `semantic-families/README.md`.
   Change repo truth from "unregistered" to "registered and promoted" only after commands are
   green.

9. Run the command loop and keep fixing only M24-related failures.

```bash
cargo fmt --all
cargo test -p xtask
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
```

10. If a prove/certify failure points at routing order, resolve the runtime/harness disagreement
    explicitly before touching docs.

## Worktree Parallelization Strategy

M24 has one real choke point and then three clean parallel lanes.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Lock family contract | `xtask/src/family/harness.rs`, `xtask/src/family/scaffold.rs` | — |
| Add xtask lock tests | `xtask/src/lib.rs` | Lock family contract |
| Curate committed packet | `semantic-families/function.arithmetic_leaf.monotone_up.v1/` | Lock family contract |
| Add runtime prove/certify tests | `spec-core/src/semantic_review.rs` | Lock family contract |
| Add CLI truth-surface/corpus/regression tests | `spec-cli/tests/m14_regressions.rs` | Lock family contract |
| Final command loop | repo-wide command execution | xtask lock tests, packet curation, runtime tests, CLI tests |
| Repo-truth docs update | `semantic-families/README.md` | Final command loop |

### Parallel lanes

- Lane A: `Lock family contract` → `Add xtask lock tests`
- Lane B: `Curate committed packet`
- Lane C: `Add runtime prove/certify tests`
- Lane D: `Add CLI truth-surface/corpus/regression tests`
- Lane E: `Final command loop` → `Repo-truth docs update`

### Execution order

1. Run Lane A first and keep it sequential.
2. Once Lane A is stable, launch Lanes B, C, and D in parallel worktrees.
3. Merge B, C, and D.
4. Run Lane E after the merge, not before.

### Conflict flags

- Lane A is a hard serialization point because it defines suite slug, starter paths, and routing
  metadata consumed by every other lane.
- Lanes B, C, and D are safe in parallel because they touch disjoint primary module roots.
- If Lane C changes the semantic boundary for the unsupported near miss, Lane B and Lane D must
  reconcile to that result before Lane E runs.
- Docs stay out of the parallel lanes. Do not claim promotion until the command loop is green.

## Acceptance Gates

M24 is done only when all of the following are true:

- `function.arithmetic_leaf.monotone_up.v1` is registered in `xtask/src/family/harness.rs`
- `StarterTemplate::ArithmeticLeafMonotoneUp` exists and is used by the harness
- a committed packet exists at `semantic-families/function.arithmetic_leaf.monotone_up.v1/`
- `cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1` passes
- `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1` passes
- `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1` passes
- `chain3` remains green and unshadowed
- `monotone_down_nonnegative` remains green and unshadowed
- `semantic-families/README.md` no longer describes `monotone_up` as unregistered
- no hidden family-specific edits outside:
  - harness registration
  - scaffold template support
  - packet-local authoring
  - family-owned tests
  - repo-truth docs

## Follow-ups Explicitly Deferred

- non-author maintainer dry run
- packet-ceremony reduction
- promotion of any third leaf family or another wrapper
- generic arithmetic-leaf templating
- broader runtime semantic-review cleanup

These belong in later milestones only after M24 lands cleanly.

## Unresolved Risks

- The locked unsupported-near-miss shape is intentionally concrete, but runtime classifier work may
  reveal a sharper monotone-up boundary. If that happens, adjust the packet and tests, not the
  milestone scope.
- `precedence = 4` is the honest current choice, but certify is the source of truth. If certify
  exposes a runtime-order mismatch, make runtime and harness agree explicitly.
- Packet-local helpers add ceremony. That is acceptable in M24 because truthful self-contained
  packets matter more than reducing authoring friction.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | Scope remains intentionally narrow: one sibling leaf promotion, no ceremony cleanup, no maintainer UX expansion |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | No separate codex review log recorded; this rewrite used direct repo inspection instead of a second external pass |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 0 | — | Plan rewritten to eng-review rigor: exact file roots, exact command gates, exact coverage targets, explicit failure modes, and worktree parallelization |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | No UI scope in M24 |

**VERDICT:** PLAN IS IMPLEMENTATION-READY. Formal review logs are still not persisted in the
dashboard, but the plan now reads as one cohesive M24 execution contract instead of a layered
review artifact.
