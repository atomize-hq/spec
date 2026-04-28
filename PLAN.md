<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m22-autoplan-restore-20260428-074139.md -->
# M23 - First Real Leaf-Family Promotion (`function.arithmetic_leaf.monotone_down_nonnegative.v1`)

Status: **fresh implementation plan on `feat/m22`**  
Base branch: **main**  
Last rewritten: **2026-04-28**

M22 made the family-promotion harness honest.

M23 uses that hardened harness for the first real non-chain3 promotion. The goal is not to invent
new runtime semantics. The goal is to prove that a second, materially different `kind:function`
family can move through the full packet, prove, and certify path without hidden chain3 assumptions.

This plan is explicit about the milestone boundary:

- **M23 is a real family-promotion milestone.**
- **M23 is not primarily an infrastructure milestone.**
- harness/manifest changes are in scope only where they are required to make the promoted family
  truthful, scaffoldable, and certifiable

If the family were already promotable without those changes, they would be out of scope.

The selected family is:

- `function.arithmetic_leaf.monotone_down_nonnegative.v1`

This is the canonical `pricing/apply_discount` shape already named in `AGENTS.md`, already
recognized by the runtime semantic reviewer, and already exercised by wedge-style CLI regression
tests. That makes it the strongest next move: reuse what is already true, then make the promotion
workflow prove it end-to-end.

## Milestone Outcome

The outcome is simple:

- M23 promotes `function.arithmetic_leaf.monotone_down_nonnegative.v1` as the first real promoted
  non-chain3 family.

The enabling work is also simple:

- remove the wrapper-only scaffold/manifest assumptions that would otherwise make that promotion
  fake.

Stated differently, M23 is **not** "do harness work for its own sake." M23 is "land a real
leaf-family promotion, and do whatever minimum harness work is necessary so the result is honest."

After M23, the intended maintainer workflow is proven on two real promoted families:

1. add one explicit family definition to the xtask registry
2. run `cargo xtask family new function.arithmetic_leaf.monotone_down_nonnegative.v1`
3. fill packet fixtures and packet metadata
4. keep or adjust runtime classifier wiring only if the packet exposes a real gap
5. run `cargo xtask family prove function.arithmetic_leaf.monotone_down_nonnegative.v1`
6. run `cargo xtask family certify function.arithmetic_leaf.monotone_down_nonnegative.v1`

That is the whole game.

If M23 lands cleanly, the repo can honestly claim that family promotion is repeatable across
meaningfully different `kind:function` shapes, not just chain-style wrappers.

More precisely, M23 should let the repo claim:

- the promotion harness is no longer wrapper-only
- a second real promoted function family can be added without hidden chain3-only edits
- the workflow is legible enough that another maintainer can follow it from docs and artifacts

It should **not** claim that family promotion is solved generally across all kinds or all future
family shapes. That stronger claim belongs to later work.

## User Job To Be Done

The external job this milestone serves is not "make the repo feel cleaner."

It is:

- a maintainer can promote a supported semantic family without hidden code edits
- a maintainer can understand why prove/certify passed or failed
- the repo can freeze that family as a durable contract surface instead of a loose runtime behavior

If M23 does not make family promotion more legible and less maintainer-dependent, it is not enough.

## Core Decision

Promote one already-supported leaf family, not a near-neighbor wrapper.

- Do **not** spend M23 re-hardening M22 unless a new defect is discovered.
- Do **not** spend M23 inventing new semantic vocabulary in `spec-core`.
- Do **not** promote both arithmetic leaf families in one milestone.
- Do **not** generalize to `kind:data` or `kind:sum` yet.

The smallest complete move is:

- one real promoted arithmetic leaf family
- packet corpus under `semantic-families/`
- family-specific xtask scaffold/manifest support, only as enabling work
- prove/certify suites locked to that family
- live certify artifacts for the new family

The point of M23 is the promoted family. The xtask changes matter only because the current xtask
layer would otherwise force a wrapper-shaped lie.

## Selected Family

### Why `function.arithmetic_leaf.monotone_down_nonnegative.v1`

This family is the best M23 target because:

1. it is already part of the shipped function-family vocabulary in `AGENTS.md` and `README.md`
2. the runtime classifier already recognizes it through `pricing/apply_discount`
3. the repo already has aligned, drift, and under-specified wedge coverage for it
4. it is materially different from chain3, so success proves real generalization

### Why not `function.wrapper.pipeline.chain4.v1`

`chain4` would be faster, but it is weaker proof. It stays too close to the chain3 mental model and
lets the repo overclaim repeatability without proving the harness can carry a genuinely different
packet shape.

### Why not `function.arithmetic_leaf.monotone_up.v1` first

`monotone_up` is a good M24 candidate, but `monotone_down_nonnegative` is slightly stronger as the
first leaf-family promotion because its nonnegative bound makes the family contract tighter. It is
more likely to expose whether packet shape, invariants, and unsupported-near-miss corpus modeling
are actually family-specific, not just generic arithmetic boilerplate.

## Alternatives Table

| Option | New risk retired | Time / cost | If it works, what changes |
|---|---|---|---|
| `chain4` first | Proves wrapper family can extend one more step | Lowest | Good local green signal, weak generalization proof |
| `monotone_down_nonnegative` first | Proves packet + harness can handle a true leaf shape | Medium | First honest proof that promotion is not wrapper-only |
| `monotone_up` first | Proves a second leaf family with weaker invariant structure | Medium | Similar proof, slightly less stress on packet invariants |
| non-function kind first | Proves broader architecture faster | Highest | Bigger future payoff, much higher scope and ambiguity now |

`monotone_down_nonnegative` still wins because it retires the most important next risk without
turning M23 into an architecture referendum.

## Verified Starting Point

The following facts are true in the current tree:

- `spec-core/src/semantic_review.rs` already ships
  `function.arithmetic_leaf.monotone_down_nonnegative.v1` and
  `function.arithmetic_leaf.monotone_up.v1` as supported function-family compatibility keys.
- `spec-cli/tests/m14_regressions.rs` already has wedge tests for `pricing/apply_discount`:
  aligned, drift, under-specified, and clamp-drift.
- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_discount_control_flow.unit.spec`
  already gives the repo an unsupported leaf-family near-miss starting point.
- `xtask/src/family/harness.rs` still only registers chain3.
- `xtask/src/family/scaffold.rs` still emits a chain3-shaped `family.toml` template:
  `dep_count = 3`, wrapper-like argument threading fields, and `requires_supported_function_deps = true`.

That last point is the M23 forcing function. The harness is honest now, but the scaffold contract is
not yet family-shaped enough to bootstrap a real leaf family.

## Problem Statement

M22 proved registry-first promotion mechanics. M23 must close the remaining gap between "the
runtime already knows this family" and "this family is a real promoted packet with prove/certify
artifacts."

Three concrete problems remain:

1. **Packet-shape gap**  
   `scaffold.rs` can now route family-specific values from the harness, but it still hard-codes a
   chain3-style manifest shape. A real leaf family needs family-specific scaffold metadata.

2. **Promotion-surface gap**  
   The runtime already classifies `pricing/apply_discount` as
   `function.arithmetic_leaf.monotone_down_nonnegative.v1`, but there is no family packet or xtask
   prove/certify surface that freezes this as a promoted contract.

3. **Evidence-shape gap**  
   The repo already has leaf-family wedge tests and an unsupported pack, but they are not organized
   into the packet-shaped aligned/drift/under-specified/unsupported-near-miss corpus that the
   promotion harness expects.

M23 closes those three gaps and nothing more.

## Premise Challenge

The plan assumes all of the following. If any are false, M23 must narrow its claims even if the
code lands green.

1. The next blocked maintainer need is promoting a second real function family, not a CLI or
   non-function capability gap.
2. `pricing/apply_discount` is already close enough to the intended family truth that promotion
   should mostly freeze existing behavior, not rediscover the family from scratch.
3. One wrapper family plus one leaf family is enough to claim the harness is less chain3-specific,
   but not enough to claim promotion is solved generally.
4. Manual packet curation is still acceptable at this stage if it produces a trustworthy contract
   and reveals where future automation would matter.

## Graduation Criteria

M23 does **not** graduate the repo to "family promotion is repeatable everywhere."

It graduates the repo only to:

- `kind:function` promotion is now proven on one wrapper and one leaf family
- the xtask harness can express materially different packet shapes
- a new maintainer has a plausible chance of following the workflow from docs and artifacts

The stronger repeatability claim stays blocked until at least one of these happens later:

- a third promoted family retires a different structural risk
- a non-author maintainer runs the promotion path successfully
- another `kind:` family demonstrates the harness is not function-special forever

## What Already Exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Runtime family vocabulary | `spec-core/src/semantic_review.rs` supported function routing + compatibility keys | Reuse directly. Do not invent a new family semantics model. |
| Canonical aligned leaf unit | `examples/ecommerce/units/pricing/apply_discount.unit.spec` | Reuse as the aligned corpus seed. |
| Leaf-family drift / under-specified wedges | `spec-cli/tests/m14_regressions.rs` rewrite helpers and assertions | Reuse as corpus design source and regression backstop. |
| Unsupported near-miss leaf example | `spec-cli/tests/fixtures/m20/unsupported_truth_pack/units/pricing/apply_discount_control_flow.unit.spec` | Reuse as the starting unsupported bucket truth. |
| Honest registry-first promotion mechanics | `xtask/src/family/*` from M22 | Reuse directly. Extend only where leaf-family packet shape requires it. |
| Live promoted family baseline | `function.wrapper.pipeline.chain3.v1` packet + prove/certify artifacts | Preserve as the regression backstop. |

## NOT In Scope

- Promoting `function.arithmetic_leaf.monotone_up.v1` in the same milestone
- Promoting `function.wrapper.pipeline.v1` in the same milestone
- Generic multi-kind family promotion for `data` or `sum`
- Public `spec` CLI changes unrelated to family promotion
- Reworking semantic-review scoring or adding new verdict fields
- Reopening M22 routing/reporting hardening without a new reproduced defect

## Non-Negotiable Invariants

1. `family.toml` stays packet-local metadata and validation truth. It does not choose commands.
2. `xtask/src/family/harness.rs` stays the single orchestration source of truth.
3. M23 promotes exactly one new real function family.
4. `chain3` prove/certify stays green throughout the milestone.
5. Runtime semantic-review meaning for `pricing/apply_discount` stays stable unless the packet
   proves an actual bug.
6. The selected leaf family's packet corpus must include aligned, drift, under-specified, and
   unsupported-near-miss buckets.

## Architecture Contract

### Durable source of truth

`FamilyHarness` must grow from "routing plus suites" into the full xtask family-definition
contract for promoted function families:

- scaffold namespace and starter stems
- locked routing metadata
- locked packet-shape metadata
- prove suite membership
- certify suite membership

No other module may reconstruct leaf-family packet shape out of ad hoc constants once M23 is done.

### Family-specific scaffold contract

The harness must be able to express at least these packet-shape differences:

- helper-dep topology
- control-flow constraint
- return-style constraint
- whether supported-function deps are required
- argument-threading rule
- family-specific starter fixture stems

For `function.arithmetic_leaf.monotone_down_nonnegative.v1`, that means a leaf-shaped packet, not a
wrapper packet with renamed fields.

### Canonical leaf-family contract for M23

This is the design decision that matters most for M23.

The current tree does **not** support a truthful "leaf family means `dep_count = 0`" story:

- `examples/ecommerce/units/pricing/apply_discount.unit.spec` already uses one helper dep:
  `money/round`
- `spec-core/src/semantic_review.rs` already classifies this family with `[]` or `[dep]`
- `strip_outer_helper_if_present(...)` already models the optional helper-wrapper shape in the
  runtime classifier

So M23 must freeze the leaf-family contract as:

- zero or one helper dep, not exactly zero deps
- no loops
- no branching
- straight-line arithmetic body
- optional outer helper call around the arithmetic core
- no required supported-function deps

That is the packet shape the harness and manifest validator must describe. Anything else is fake
green.

### Locked M23 manifest schema choice

M23 locks the manifest change instead of leaving it to implementer taste.

`family.toml` moves from schema version `1` to schema version `2` for **both** chain3 and the new
leaf family. The exact shape change is:

- remove `shape.dep_count`
- add `shape.dep_min`
- add `shape.dep_max`

Exact semantics:

- `dep_min` and `dep_max` are inclusive
- `dep_min <= dep_max` is required
- exact-cardinality families encode as `dep_min == dep_max`
- range families encode as `dep_min < dep_max`

Exact family values:

- `function.wrapper.pipeline.chain3.v1`
  - `shape.dep_min = 3`
  - `shape.dep_max = 3`
- `function.arithmetic_leaf.monotone_down_nonnegative.v1`
  - `shape.dep_min = 0`
  - `shape.dep_max = 1`

This is the only dep-topology encoding M23 should implement. Do **not** add parallel encodings
like `dep_count` plus `helper_dep_optional`; that would preserve ambiguity instead of removing it.

### Locked M23 harness shape

M23 also locks the exact harness additions.

`FamilyHarness` must grow by:

- `shape`, a first-class harness-owned packet-shape definition used by both scaffold and manifest
  validation
- `suite_slug`, a family-owned stable slug used to validate prove/certify suite ownership
- bucket-local starter case definitions, replacing the current global `starter_case_stems`

The starter cases for the promoted family are locked as:

- `fixtures/aligned/units/pricing/apply_discount_aligned.unit.spec`
- `fixtures/drift/units/pricing/apply_discount_drift.unit.spec`
- `fixtures/under_specified/units/pricing/apply_discount_under_specified.unit.spec`
- `fixtures/unsupported_near_miss/units/pricing/apply_discount_control_flow_unsupported_near_miss.unit.spec`

That is the exact starter packet M23 should scaffold.

### Runtime reuse contract

M23 is allowed to reuse the existing runtime classifier wholesale.

The default expectation is:

- no new semantic family added to `spec-core`
- no public truth-surface schema change
- only targeted runtime edits if the new packet corpus exposes a real mismatch between the claimed
  family contract and the shipped classifier behavior

### Dream State Delta

```text
CURRENT
  one real promoted wrapper family
  leaf families exist only as runtime behavior + wedge tests
  scaffold still assumes chain3-style manifest shape

THIS PLAN (M23)
  one real promoted wrapper family
  one real promoted leaf family
  harness can emit leaf-shaped packet metadata truthfully
  docs and artifacts explain promotion with less maintainer guesswork

12-MONTH IDEAL
  multiple promoted families across more than one structural axis
  promotion workflow runnable by non-authors
  packet authoring lighter, with less manual curation
  stronger evidence that the contract is worth the ceremony
```

## Promotion Data Flow

```text
runtime family already exists
        │
        ├── spec-core/src/semantic_review.rs
        │       └── classifies apply_discount as monotone_down_nonnegative
        │
        ├── xtask/src/family/harness.rs
        │       └── add real leaf-family harness entry
        │
        ├── xtask/src/family/scaffold.rs
        │       └── emit leaf-shaped packet template from harness metadata
        │
        ├── semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/
        │       ├── family.toml
        │       ├── candidate.md
        │       └── fixtures/
        │            ├── aligned/
        │            ├── drift/
        │            ├── under_specified/
        │            └── unsupported_near_miss/
        │
        ├── cargo xtask family prove <family>
        │       └── prove.latest.json
        │
        └── cargo xtask family certify <family>
                └── certification.report.json
```

## Affected Modules

| Module | Role in M23 | Required change |
|---|---|---|
| `xtask/src/family/harness.rs` | Registry contract | Add the new leaf-family harness entry and the family-specific packet-shape metadata it needs. |
| `xtask/src/family/scaffold.rs` | Packet bootstrap | Replace chain3-shaped manifest assumptions with harness-driven family packet shape. |
| `xtask/src/family/manifest.rs` | Packet validation | Validate the new leaf-family packet shape truthfully and reject chain3-only assumptions where needed. |
| `xtask/src/family/prove.rs` | Prove workflow | Bind the new family to locked prove suites and packet corpus. |
| `xtask/src/family/certify.rs` | Certify workflow | Certify the new family on top of M22 routing/reporting behavior without regressions. |
| `xtask/src/lib.rs` | xtask tests | Add real-family and synthetic coverage for leaf-family scaffold/prove/certify behavior. |
| `semantic-families/` | New packet | Add the first real leaf-family packet. |
| `spec-cli/tests/m14_regressions.rs` | Runtime regression backstop | Reuse and, if needed, refactor the existing apply_discount wedge coverage into the promoted-family prove story. |
| `spec-core/src/semantic_review.rs` | Runtime classifier | Touch only if the packet proves a mismatch. |

## Workstreams

### Workstream 1. Add the minimum harness/manifest support required to make the leaf-family promotion real

**Files**

- `xtask/src/family/harness.rs`
- `xtask/src/family/scaffold.rs`
- `xtask/src/family/manifest.rs`

**Changes**

- Add a first-class packet-shape definition to `FamilyHarness`, not just more routed constants.
- Bump `family.toml` to schema version `2`.
- Replace `shape.dep_count` with `shape.dep_min` and `shape.dep_max`.
- Move manifest truth out of the current M21-global assumptions in `xtask/src/family/manifest.rs`
  and into harness-defined family shape validation.
- Replace chain3-shaped manifest defaults in `xtask/src/family/scaffold.rs` with harness-driven
  rendering.
- Encode the monotone-down leaf family as:
  - `shape.dep_min = 0`
  - `shape.dep_max = 1`
  - arithmetic-leaf control-flow constraints
  - no required supported-function deps
  - the exact bucket-local starter fixture set locked above
- Migrate chain3 to the same schema with:
  - `shape.dep_min = 3`
  - `shape.dep_max = 3`
- Replace the current misleading starter unit template with one that teaches the real family shape:
  `subtotal`, `rate`, optional helper dep, and nonnegative clamp semantics.
- Tighten manifest validation so a leaf-family packet cannot accidentally serialize as a wrapper
  family with leaf routing.

This workstream exists to unblock the promoted family. It is not a separate "infrastructure
milestone" hidden inside M23.

**Acceptance**

- `cargo xtask family new function.arithmetic_leaf.monotone_down_nonnegative.v1` generates a
  leaf-shaped `family.toml`, not a chain3-shaped one.
- `cargo xtask family new function.wrapper.pipeline.chain3.v1` generates schema-version-2
  metadata with exact `dep_min = dep_max = 3`.
- The new harness entry is sufficient to express the packet without hidden scaffold edits.
- The generated starter unit spec is recognizably aligned with `pricing/apply_discount`, not a
  generic placeholder that teaches the wrong family.

### Workstream 2. Add the real leaf-family packet

**Files**

- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/family.toml`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/candidate.md`
- `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/**`

**Changes**

- Create the packet from `family new` after the harness entry exists.
- Seed the aligned bucket from the canonical ecommerce `pricing/apply_discount` truth.
- Build the drift and under-specified buckets from the existing wedge transformations already used
  in `spec-cli/tests/m14_regressions.rs`.
- Seed the unsupported-near-miss bucket from the existing unsupported leaf-family fixture shape in
  the M20 unsupported truth pack, adapted into packet-local form.

**Acceptance**

- The packet has all four required buckets.
- Each bucket is meaningful for the selected family, not generic filler.
- A maintainer can inspect `candidate.md` and see exactly why each bucket exists.

### Workstream 3. Bind prove/certify to the leaf family

**Files**

- `xtask/src/family/harness.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/lib.rs`

**Changes**

- Add locked prove and certify suite definitions for the new family.
- Reuse existing wedge/runtime tests where they are already truthful.
- Add or refactor only the minimum family-specific test selectors needed to make prove/certify
  artifacts a stable contract.
- Lock the prove/certify linkage mechanism as follows:
  - each `FamilyHarness` entry owns a stable `suite_slug`
  - every prove suite name for that family must include that `suite_slug`
  - every certify suite name for that family must include that `suite_slug`
  - every `expected_tests` entry in those suites must also include that `suite_slug`
  - `family prove` and `family certify` must validate this ownership rule before running suites
    and fail as invalid input if any suite or attested test is not family-owned
- For M23, the exact slug is `monotone_down_nonnegative_`.
- For chain3, the exact slug remains `m21_chain3_`.
- Make prove/certify consume packet semantics, not just packet layout plus manually named suites.
  The family packet must have a visible relationship to the proving suites that certify it through
  the single owning harness entry.
- Keep M22 artifact truth and routing coherence behavior unchanged.

**Acceptance**

- `cargo xtask family prove function.arithmetic_leaf.monotone_down_nonnegative.v1` passes.
- `cargo xtask family certify function.arithmetic_leaf.monotone_down_nonnegative.v1` passes.
- prove/certify artifacts are written under `.semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/`.
- The prove story is resilient against a packet that is layout-valid but semantically hollow.
- prove/certify reject a suite-definition mismatch before suite execution if the suite or attested
  test names are not owned by the family's locked `suite_slug`.

### Workstream 4. Preserve runtime semantics unless the packet proves a real bug

**Files**

- `spec-core/src/semantic_review.rs` only if required
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/cli.rs` only if required

**Changes**

- Treat the current runtime classifier as the default truth to reuse.
- If the new packet corpus proves a mismatch, fix the runtime classifier narrowly and add a
  regression test tied to the promoted family packet.
- Do not opportunistically refactor supported-function routing order or semantic-review schema.

**Acceptance**

- `pricing/apply_discount` still refreshes to
  `function.arithmetic_leaf.monotone_down_nonnegative.v1`.
- Any runtime change is justified by a failing promoted-family proof, not by cleanup instinct.

### Workstream 5. Keep chain3 as the frozen regression backstop

**Files**

- existing chain3 packet
- `xtask/src/lib.rs`

**Changes**

- Re-run chain3 prove/certify after the new family lands.
- Keep the chain3 packet and artifacts as the "known-good first family" backstop.
- Add tests only where the new leaf-family promotion could accidentally shadow or disturb chain3.

**Acceptance**

- `cargo xtask family prove function.wrapper.pipeline.chain3.v1` still passes.
- `cargo xtask family certify function.wrapper.pipeline.chain3.v1` still passes.

### Workstream 6. Document the new repo claim

**Files**

- `semantic-families/README.md`
- `README.md`
- `AGENTS.md` only if needed

**Changes**

- Update the maintainer story from "one real promoted family plus synthetic generalization" to
  "two real promoted function families, one wrapper and one leaf."
- Keep the registry-first wording explicit.
- Name the new promoted family and the exact workflow it proves.

**Acceptance**

- A maintainer reading the docs can tell the difference between:
  - shipped runtime-supported family vocabulary
  - actually promoted packet families
  - the next most likely follow-up family

## Existing Code Leverage Map

```text
apply_discount runtime classification
    └── reuse from spec-core/src/semantic_review.rs

apply_discount wedge truth surfaces
    └── reuse from spec-cli/tests/m14_regressions.rs

unsupported near miss leaf shape
    └── reuse from spec-cli/tests/fixtures/m20/unsupported_truth_pack/

promotion mechanics, routing coherence, truthful artifacts
    └── reuse from xtask/src/family/* shipped in M22
```

## Architecture ASCII Diagram

```text
                         M23 LEAF PROMOTION CONTRACT

examples/ecommerce/units/pricing/apply_discount.unit.spec
        │
        │ canonical aligned authored truth, optional helper dep
        ▼
spec-core/src/semantic_review.rs
        │
        │ shipped runtime classifier
        │   - zero-or-one helper dep
        │   - straight-line arithmetic core
        │   - no control flow
        ▼
xtask/src/family/harness.rs
        │
        │ family definition becomes the xtask source of truth
        ├──────────────┐
        ▼              ▼
xtask/src/family/scaffold.rs   xtask/src/family/manifest.rs
        │                      │
        │ emit packet          │ validate packet against harness-defined shape
        └──────────────┬───────┘
                       ▼
semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/
        │
        ├── candidate.md
        ├── family.toml
        └── fixtures/
            ├── aligned
            ├── drift
            ├── under_specified
            └── unsupported_near_miss
                       │
                       ▼
xtask/src/family/prove.rs
        │
        │ layout + packet-linked suites + artifact truth
        ▼
.semantic-family-artifacts/.../prove.latest.json
        │
        ▼
xtask/src/family/certify.rs
        │
        │ routing coherence + regression suites + artifact truth
        ▼
.semantic-family-artifacts/.../certification.report.json
```

## Error & Rescue Registry

| Failure point | What goes wrong | Detection surface | Rescue |
|---|---|---|---|
| Harness shape under-specified | Leaf family still encoded as chain3-ish scalar defaults | `cargo test -p xtask`, scaffold/manifest tests | Introduce explicit helper-dep topology and family-shaped validation in `harness.rs` and `manifest.rs` |
| Scaffold lies about the family | `family new` generates wrapper-like `family.toml` or starter unit spec | `cargo xtask family new ...` plus xtask golden assertions | Fix harness-driven rendering in `scaffold.rs`; do not paper over with manual packet edits |
| Runtime/packet mismatch | Packet claims a shape classifier does not actually accept | promoted-family prove/certify failure, targeted `spec-cli` / `spec-core` regressions | Change runtime narrowly only if packet truth proves it |
| Hollow packet goes green | Layout passes but packet does not truly ground the family | packet-linked prove/certify suite review | Bind suites to packet semantics and corpus buckets, not just folder presence |
| Unsupported bucket is fake | near-miss case drifts into generic unsupported filler | packet review, `unsupported_reason_codes`, read-side assertions | Seed from real `apply_discount_control_flow` shape and preserve additive-only behavior |
| Maintainer path still depends on author memory | Another maintainer cannot reproduce the flow from docs/artifacts | clean-room smoke test during milestone validation | Fix docs, starter spec, and prove/certify failure messaging before claiming repeatability |

## Failure Modes Registry

| Risk | Severity | Why it matters | Planned guardrail |
|---|---|---|---|
| Wrapper assumptions still baked into manifest validation | High | Makes M23 fake by turning a new family into renamed chain3 | Replace global M21 constants with harness-defined shape rules |
| Exact `dep_count = 0` contract | High | Contradicts canonical `apply_discount` and shipped classifier behavior | Freeze zero-or-one helper dep contract explicitly |
| Packet-local unsupported coverage not grounded | Medium | Makes certification look stronger than the read-side truth it proves | Require packet-local unsupported near miss with additive-only assertions |
| Packet/suite linkage too loose | Medium | Prove/certify can stay green while semantic corpus weakens | Lock stable family-specific suite selectors and packet-linked expectations |
| Misleading starter unit scaffold | Medium | Teaches future maintainers the wrong family shape | Replace placeholder starter body and signature with family-shaped starter truth |
| Overclaiming repeatability | Medium | Repo docs become untrustworthy again | Restrict claim to one wrapper plus one leaf family until later milestones retire more risk |

## Test Coverage Diagram

```text
M23 codepath / claim
    │
    ├── Harness entry exists and routes correctly
    │     └── xtask tests in xtask/src/lib.rs
    │
    ├── family new emits truthful leaf packet
    │     ├── xtask scaffold tests
    │     └── manual smoke: cargo xtask family new function.arithmetic_leaf.monotone_down_nonnegative.v1
    │
    ├── aligned leaf family stays supported
    │     └── spec-cli/tests/m14_regressions.rs
    │         canonical_apply_discount_semantic_review_wedge_projects_aligned_state
    │
    ├── drift and under-specified wedges demote honestly
    │     └── spec-cli/tests/m14_regressions.rs
    │         drift_apply_discount_wedge_projects_failing_state
    │         under_specified_apply_discount_wedge_projects_incomplete_state
    │         clamp_drift_apply_discount_wedge_projects_failing_state
    │
    ├── unsupported near miss stays additive-only
    │     └── new/updated packet-local regression tied to apply_discount_control_flow shape
    │
    ├── prove artifact truth remains honest
    │     └── xtask prove tests + cargo xtask family prove <leaf>
    │
    ├── certify routing/report truth remains honest
    │     └── xtask certify tests + cargo xtask family certify <leaf>
    │
    └── chain3 remains the backstop
          └── cargo xtask family prove/certify function.wrapper.pipeline.chain3.v1
```

Test plan artifact:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m22-test-plan-20260428-075526.md`

## CEO Review

### CODEX SAYS (CEO — strategy challenge)

The strategy is right if the claim stays narrow. M23 should prove "promotion is not wrapper-only,"
not "promotion is generally solved." The useful product outcome is maintainer legibility, not just
one more green artifact.

### CLAUDE SUBAGENT (CEO — strategic independence)

The outside voice pushed on the same fault line: the repo was close to overclaiming repeatability.
It asked for clearer maintainer-facing success criteria, stronger graduation language, and explicit
distinction between engineering confidence and broad product proof.

### CEO DUAL VOICES — CONSENSUS TABLE

| Dimension | Claude | Codex | Consensus |
|---|---|---|---|
| Premises valid? | Yes | Yes | CONFIRMED |
| Right problem to solve? | Yes | Yes | CONFIRMED |
| Scope calibration correct? | Yes, if narrow | Yes, if narrow | CONFIRMED |
| Alternatives sufficiently explored? | Yes, after rewrite | Yes, after rewrite | CONFIRMED |
| Competitive / market risks covered? | Yes, after explicit section | Yes, after explicit section | CONFIRMED |
| 6-month trajectory sound? | Yes, with staged claims | Yes, with staged claims | CONFIRMED |

### CEO Completion Summary

| Item | Status | Note |
|---|---|---|
| Premise challenge | Complete | Explicit premises and failure conditions are now named |
| Alternatives | Complete | `chain4`, `monotone_up`, and non-function promotion are all compared |
| Claim discipline | Complete | Plan now limits M23 to wrapper + leaf proof, not general repeatability |
| Maintainer job to be done | Complete | Success is legibility and repeatability for maintainers, not ceremony |
| Open CEO concern | Narrow | Broader market proof still waits on later adoption evidence |

## Eng Review

### CODEX SAYS (eng — architecture challenge)

The main engineering issue was structural, not cosmetic. `xtask/src/family/manifest.rs` still
validates M21 wrapper assumptions globally, and `xtask/src/family/scaffold.rs` still teaches a
chain3 packet. That does **not** change the milestone. The milestone is still the first real
non-chain3 family promotion. It just means the promotion is not honest until that contract moves
into a first-class harness shape and matches the actual `apply_discount` family topology.

### CLAUDE SUBAGENT (eng — independent review)

The outside eng voice agreed on the same core issues: Workstream 1 was under-scoped, packet/suite
linkage was too weak, the starter scaffold was misleading, and the plan needed an executable
maintainer-legibility gate rather than a vague aspiration.

### ENG DUAL VOICES — CONSENSUS TABLE

| Dimension | Claude | Codex | Consensus |
|---|---|---|---|
| Architecture sound? | Yes, after harness-shape redesign | Yes, after harness-shape redesign | CONFIRMED |
| Test coverage sufficient? | Yes, after packet-linked proof additions | Yes, after packet-linked proof additions | CONFIRMED |
| Performance risks addressed? | Yes | Yes | CONFIRMED |
| Security threats covered? | Yes, low-change local surface | Yes, low-change local surface | CONFIRMED |
| Error paths handled? | Yes, after rescue/failure registries | Yes, after rescue/failure registries | CONFIRMED |
| Deployment risk manageable? | Yes | Yes | CONFIRMED |

### Eng Completion Summary

| Item | Status | Note |
|---|---|---|
| Scope challenge against real code | Complete | Hard-coded wrapper assumptions in `manifest.rs` and `scaffold.rs` are now explicit in plan scope |
| Architecture diagram | Complete | Added M23 dependency graph tied to existing code |
| Test diagram | Complete | Packet claims are mapped to concrete xtask and CLI coverage |
| Failure modes | Complete | High-risk false-green paths are enumerated with guardrails |
| Open eng concern | Narrow | Maintainer clean-room run still needs execution during implementation, not just planning |

## Design Review

Skipped, no UI scope. This milestone changes xtask, packet artifacts, tests, and documentation
only. No user-facing interaction surface is being designed in M23.

## Cross-Phase Themes

- The real M23 problem is claim honesty. Both CEO and Eng review converged on reducing fake
  generalization.
- The most dangerous false green was "leaf family equals dep_count 0." That is now explicitly
  rejected in favor of the shipped optional-helper topology.
- The remaining design ambiguity was removed by locking one schema path: `dep_min`/`dep_max` in
  manifest schema version `2`, plus harness-owned `suite_slug` validation for prove/certify.
- Maintainer legibility is the through-line. Starter scaffold truth, packet explanation quality,
  and prove/certify failure clarity all serve that job.
- The repo can safely reuse shipped runtime behavior, but only while packet truth is allowed to
  challenge it. Reuse without packet authority would just recreate M22's original honesty problem.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Promote `function.arithmetic_leaf.monotone_down_nonnegative.v1` first | scope | smallest real proof | It retires the wrapper-only risk faster than `chain4` without opening multi-kind scope | `chain4`, `monotone_up`, non-function first |
| 2 | CEO | Keep M23 to one real promoted family | scope | avoid fake completeness | Two families in one milestone would blur whether failures come from promotion mechanics or family semantics | dual-family M23 |
| 3 | Eng | Keep M23 framed as a real promotion milestone, with harness work scoped as enabling work | architecture | preserve milestone honesty | `manifest.rs` and `scaffold.rs` are structurally wrapper-biased today, but fixing them is in service of the promoted family, not the point of the milestone | generic harness hardening milestone |
| 4 | Eng | Replace `dep_count` with `dep_min` / `dep_max` in manifest schema version `2` | architecture | remove schema ambiguity | One exact encoding covers both chain3 exact cardinality and leaf-family range cardinality | keep `dep_count`, add parallel optional-helper flags |
| 5 | Eng | Freeze leaf contract as `dep_min = 0`, `dep_max = 1` | architecture | match shipped truth | Canonical `apply_discount` and runtime classifier already allow an optional helper dep | exact `dep_count = 0` |
| 6 | Eng | Require harness-owned `suite_slug` validation for prove/certify suite ownership | test | no hollow green artifacts | Packet proof must be owned by the same harness entry that defines the family packet | prove layout only |
| 7 | CEO | Add maintainer-legibility success criteria | product | optimize for real user | The maintainer is the user of this workflow, not the artifact JSON | green artifacts alone |
| 8 | Cross-phase | Defer `monotone_up` to M24 | sequencing | one structural risk at a time | M23 already retires the wrapper-only risk; a second leaf family is follow-up, not prerequisite | promote both leaf families now |

## Competitive / Substitute Risk

The repo is not competing against another packet registry.

It is competing against:

- doing nothing and trusting code review plus tests
- lighter-weight semantic tooling with less ceremony
- future automation that infers family truth instead of asking maintainers to author packets

M23 only makes sense if it improves one of these:

- trust
- legibility
- reuse
- promotion speed for maintainers who are not the original author

If it improves none of them, it is local optimization.

## Regret Guardrail

If this milestone ships cleanly, the repo may claim:

- "the harness can now promote a real wrapper family and a real leaf family"
- "the packet scaffold is less chain3-specific"

It may **not** claim:

- family promotion is solved generally
- non-function promotion is now straightforward
- packet curation cost is already low enough for broad adoption
- the current runtime classifier is universally the right abstraction

## Operational Acceptance

These are maintainer-facing success metrics, not just engineering green lights:

- after the harness entry exists, `family new` should require no hidden scaffold edits
- the new packet should be explainable from `candidate.md` plus fixtures alone
- prove/certify failures should point at the packet truth or suite truth, not require source diving
- a non-author maintainer should be able to follow the documented path without private context

These do not need formal benchmarking in M23, but the milestone should visibly optimize for them.

## Complexity Call

This will likely touch 8-10 files across `xtask`, `semantic-families`, and targeted test surfaces.

That is near the smell threshold, but still justified because:

- the blast radius is tightly contained to family promotion
- the runtime family already exists
- the biggest new work is packet truth, not framework invention

Anything beyond one promoted leaf family is scope creep.

## Commands

Primary validation loop:

```bash
cargo test -p xtask
cargo xtask family new function.arithmetic_leaf.monotone_down_nonnegative.v1
cargo xtask family prove function.arithmetic_leaf.monotone_down_nonnegative.v1
cargo xtask family certify function.arithmetic_leaf.monotone_down_nonnegative.v1
cargo xtask family prove function.wrapper.pipeline.chain3.v1
cargo xtask family certify function.wrapper.pipeline.chain3.v1
```

Maintainer-legibility smoke loop:

```bash
rm -rf semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1
cargo xtask family new function.arithmetic_leaf.monotone_down_nonnegative.v1
git diff -- semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1
```

Targeted runtime backstop commands if classifier changes are required:

```bash
cargo test -p spec-core --lib apply_discount
cargo test -p spec-cli --test m14_regressions apply_discount
```

## Success Criteria

M23 is green only if all of the following are true:

- `function.arithmetic_leaf.monotone_down_nonnegative.v1` has a real packet under `semantic-families/`
- the xtask scaffold emits a truthful leaf-family manifest shape from harness metadata
- the new family prove command passes
- the new family certify command passes
- chain3 prove/certify still pass
- docs now honestly describe two real promoted function families
- the maintainer smoke loop does not require hidden edits beyond the planned harness and packet work
- the implementation uses exactly one dep-topology schema, `dep_min` / `dep_max`, and exactly one
  suite-ownership rule, harness-owned `suite_slug` validation

M23 is red if any of the following remain true:

- adding the leaf family still requires hidden edits outside the harness contract and obvious packet work
- the generated `family.toml` still looks like chain3 with a different name
- unsupported-near-miss coverage is missing or fake
- chain3 regresses while promoting the leaf family

## Follow-Up Boundary

If M23 lands cleanly, the likely next milestones are:

- M24: promote `function.arithmetic_leaf.monotone_up.v1`
- later: decide whether `function.wrapper.pipeline.v1` or a non-function kind gets the next real packet

That is later work. M23 should stay boring.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR | Narrow-claim discipline, maintainer JTBD, and graduation criteria were added |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR | Workstream 1 was clarified as enabling work for a real non-chain3 promotion, and the remaining implementation choices were locked to `dep_min` / `dep_max` plus harness-owned `suite_slug` validation |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | No UI scope in M23 |

- **CROSS-MODEL:** CEO and Eng outside voices converged on the same main correction, do not overclaim repeatability and do not fake the leaf-family contract.
- **UNRESOLVED:** 0 planning decisions unresolved. Implementation risk remains in executing the clean-room maintainer smoke run during delivery.
- **VERDICT:** CEO + ENG CLEARED — ready to implement.
