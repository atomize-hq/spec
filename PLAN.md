# M19 - Semantic Review Falsification Pack

Status: **Explicit M19 plan** (April 26, 2026).

This plan supersedes the prior M18 plan. M18 landed meaningful semantic-review substrate progress,
but the milestone remains red against its own gate. M19 is therefore not another M18 execution
loop and not backend-readiness reopening. M19 is the falsification pack that
decides whether the current `kind:function` semantic-review substrate travels beyond canonical
pricing examples without preserving stale truth or false-greening nearby wrappers.

UI scope: **no**. This is a backend semantic-review correctness milestone for unseen corpus proof,
function semantic freshness, Family B argument-flow validation, and product-honest freezing of the
current unsupported-surface contract.

## Source Inputs

- Current stale plan replaced by this file:
  `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- M19 design direction:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m18-design-20260426-095101.md`
- M19 test plan:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m18-test-plan-20260426-110032.md`

## Prior State: What M18 Proved and Did Not Prove

M18 moved the project in the right direction:

- `kind:function` semantic review now has family routing instead of pure exact-unit-id routing.
- Family A arithmetic leaves have an alternate-id proof.
- Family B wrapper classification exists and the canonical `pricing/calculate_total` path is no
  longer just unsupported by default.
- Preserve vs refresh behavior still mostly follows the core product rule:
  `spec test` refreshes semantic truth; read-side flows project stored truth and should not mint it.

M18 still does not pass the bar it wrote for itself:

- The gate required unseen `aligned`, `semantic_drift`, and `under_specified` examples for each
  family. Family B does not yet have that adversarial unseen proof pack.
- Function semantic freshness is currently wrong for semantic input changes. Edits to `intent`,
  `deps`, or `body.rust` can leave preserved supported semantic reviews looking current when they
  should become stale.
- Family B can false-green on wrong argument flow because the current classifier proves wrapper
  nesting shape more strongly than semantic parameter flow.
- Unsupported-surface behavior is awkward: unsupported review metadata may exist after refresh, but
  official read-side surfaces remain neutral. M19 freezes that contract instead of redesigning it.

## Milestone Summary

```text
M19a  Add a dedicated unseen corpus for Family A and Family B                  required
M19b  Correct function semantic freshness for intent/deps/body changes        required
M19c  Tighten Family B argument-flow validation                               required
M19d  Freeze unsupported-surface behavior for this milestone                  required
M19e  Prove the command matrix against the frozen truth-surface contract      required
M19f  Re-run the semantic-core green/red gate without reopening backend work  required
```

## User Outcome

A maintainer edits a function spec, its dependencies, or its executable Rust body, runs the normal
`spec` loop, and can trust the result:

- supported family examples stay current only when their authored and executable meaning has not
  changed since proof;
- unseen Family A and Family B examples classify as `aligned`, `semantic_drift`, or
  `under_specified` for structural reasons, not canonical names;
- Family B wrappers do not align unless the declared argument flow is actually preserved;
- unsupported near misses stay neutral on official read-side health surfaces.

The outcome is not "more semantic families." The outcome is a falsifiable answer to whether the
families already admitted by M18 are trustworthy enough to continue building on.

## Approved Scope

M19 includes exactly these capability corrections and proof obligations:

- unseen Family A corpus
- unseen Family B corpus
- function semantic freshness correction
- Family B argument-flow validation
- frozen unsupported-surface contract for this milestone

M19 may adjust implementation details only insofar as they are necessary to make those obligations
true and testable.

## NOT in Scope

- unsupported-surface redesign or broader contract cleanup
- new semantic families
- backend-readiness reopening
- arbitrary `kind:function` understanding
- branching or looping wrapper semantics
- graph-wide semantic coherence
- new CLI commands or new artifact types
- large docs refresh beyond explaining the M19 contract honestly

## Code Seams

| Seam | M19 pressure | Required outcome |
|---|---|---|
| `spec-core/src/semantic_review.rs` | Family A/B classifiers, Family B argument flow, unsupported fallback | Classifiers reject false-green wrappers and keep unsupported near misses out of supported-family demotion. |
| `spec-core/src/passport.rs` | Freshness projection and proof preservation | Supported function reviews become stale or drop when semantic inputs change instead of looking current. |
| `spec-core/src/export.rs` | Read-side truth projection | Export projects only current, compatible supported truth and preserves frozen unsupported neutrality. |
| `spec-cli/src/commands.rs` | Command matrix behavior | `spec test` remains the refresh path; build/status/export do not mint replacement semantic truth. |
| `spec-cli/tests/cli.rs` | CLI regressions | Command-level tests prove freshness and frozen unsupported read-side behavior. |
| `spec-cli/tests/m14_regressions.rs` | End-to-end semantic wedges | Canonical and unseen proof packs fail or pass through the same user-visible loop. |
| `spec-cli/tests/fixtures/m19/` | New unseen corpus | Family A and Family B examples use non-canonical ids and adversarial variants. |

If an implementation path requires new architecture outside these seams, stop and re-plan. That is
evidence M19 has drifted into redesign.

## Frozen Unsupported-Surface Contract

M19 freezes the current unsupported contract:

- `spec test` may record unsupported semantic-review metadata as additive proof detail.
- Official read-side surfaces remain neutral for unsupported cases.
- `spec status --format json` and `spec export` must not demote unsupported near misses because
  they look similar to a supported family.
- M19 does not decide whether unsupported metadata should become first-class everywhere or disappear
  entirely. That is explicitly deferred.

This contract is awkward but stable enough for M19. Redesigning it would hide the actual milestone
risks behind product-surface cleanup.

## Function Freshness Contract

Supported function semantic truth is current only if the semantic inputs that produced it are still
current. At minimum, the freshness anchor must account for:

- `intent`
- declared `deps`
- executable `body.rust`
- any authored contract or invariant fields used by the family router

M19 is not green if a supported function review can survive one of those changes in preserve mode
and still appear current on status or export. Stale base health must still outrank semantic
demotion, but freshness must not fake a current semantic proof.

## Family B Argument-Flow Contract

Family B remains the bounded two-step wrapper family. M19 tightens the claim from "the wrapper has
the right nesting shape" to "the wrapper preserves the declared semantic argument flow."

The aligned subset must prove all of these:

- the first declared dep is called before the second declared dep;
- the result of the first dep is threaded into argument slot `0` of the second dep;
- every non-threaded argument is the intended top-level wrapper parameter for that dep slot;
- each required wrapper parameter has exactly the expected use in the dep chain;
- no duplicated, swapped, dropped, or substituted parameter path can classify as `aligned`.

Nearby wrappers that cannot satisfy this flow contract must become `semantic_drift`,
`under_specified`, or unsupported according to the existing family boundary. They must not
false-green as `aligned`.

## Test Matrix

### Family A Unseen Corpus

| Case | Expected proof |
|---|---|
| alternate-id monotone-down aligned | `function.arithmetic_leaf.monotone_down_nonnegative.v1` + `aligned` |
| alternate-id monotone-down drift | supported Family A + `semantic_drift` |
| alternate-id monotone-down under-specified | supported Family A + `under_specified` |
| alternate-id monotone-down unsupported near miss | unsupported and neutral on read-side surfaces |
| alternate-id monotone-up aligned | `function.arithmetic_leaf.monotone_up.v1` + `aligned` |
| alternate-id monotone-up drift | supported Family A + `semantic_drift` |
| alternate-id monotone-up under-specified | supported Family A + `under_specified` |
| alternate-id monotone-up unsupported near miss | unsupported and neutral on read-side surfaces |

### Family B Unseen Corpus

| Case | Expected proof |
|---|---|
| alternate-id wrapper aligned | `function.wrapper.pipeline.v1` + `aligned` |
| alternate-id wrapper drift | supported Family B + `semantic_drift` |
| alternate-id wrapper under-specified | supported Family B + `under_specified` |
| alternate-id wrapper unsupported near miss | unsupported and neutral on read-side surfaces |
| alternate-id wrapper with alternate-id leaf deps | aligned without canonical `pricing/*` dep scaffolding |
| non-stacking wrapper around another wrapper | unsupported and neutral |

### Family B Adversarial Flow

| Case | Must not return |
|---|---|
| inner-call args swapped | `aligned` |
| outer-call rate arg swapped | `aligned` |
| wrong threaded alias returned | `aligned` |
| duplicated param passed where distinct params are required | `aligned` |
| dropped parameter or unused parameter path | `aligned` |
| literal, arithmetic expression, or method chain substituted for a required arg | `aligned` |

### Freshness Regressions

| Change after passing proof | Required status |
|---|---|
| intent-only edit | stale or dropped proof; never current supported review |
| dep-only edit | stale or dropped proof; never current supported review |
| body-only edit | stale or dropped proof; never current supported review |
| invariant/contract edit used by routing | stale or dropped proof; never current supported review |
| preserve-mode status/export after any semantic edit | no fake-current supported semantic truth |

### Unsupported Command Matrix

| Flow | Required behavior |
|---|---|
| `spec test` on unsupported near miss | may record additive unsupported metadata |
| `spec build` after unsupported refresh | does not promote unsupported truth into health demotion |
| `spec status --format json` | unsupported near miss remains neutral/read-side non-demoting |
| `spec export` | unsupported near miss remains neutral/read-side non-demoting |

## Implementation Order

1. Lock the M19 fixture shape and add unseen corpus cases for both families.
2. Fix function semantic freshness so intent, deps, body, and routing-relevant contract changes
   invalidate current supported proof.
3. Tighten Family B argument-flow validation and add adversarial false-green regressions.
4. Add command-matrix tests for the frozen unsupported-surface contract.
5. Re-run canonical ecommerce cases to ensure M19 corrections do not regress M18 substrate wins.
6. Re-run the M19 green/red gate and record the outcome before any backend-readiness planning.

Freshness correction should land before broad fixture expansion is treated as evidence. Otherwise
new tests can accidentally bless preserved stale truth.

## Verification Loop

Targeted commands should include, at minimum:

```text
cargo test -p spec-core semantic_review -- --nocapture
cargo test -p spec-core passport -- --nocapture
cargo test -p spec-core export -- --nocapture
cargo test -p spec-cli --test cli -- --nocapture
cargo test -p spec-cli --test m14_regressions -- --nocapture
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total.unit.spec
cargo run -p spec-cli -- status examples/ecommerce --format json
```

If M19 introduces a dedicated fixture root under `spec-cli/tests/fixtures/m19/`, add exact
single-unit and root-level commands for that corpus before declaring the gate green.

## Green Gate

M19 is green only if all of these are true:

1. Family A unseen corpus proves `aligned`, `semantic_drift`, and `under_specified` with
   non-canonical unit ids.
2. Family B unseen corpus proves `aligned`, `semantic_drift`, and `under_specified` with
   non-canonical unit ids.
3. At least one aligned Family B case uses alternate-id leaf deps, not only canonical
   `pricing/apply_discount` and `pricing/apply_tax`.
4. Every Family B adversarial-flow case fails to classify as `aligned`.
5. Supported function semantic truth becomes stale or drops after `intent`, `deps`, `body.rust`, or
   routing-relevant contract changes.
6. Unsupported near misses stay neutral in `spec status --format json` and `spec export`.
7. Canonical ecommerce examples still project the intended Family A and Family B keys after fresh
   `spec test` proof.
8. The supported vs unsupported story can be stated in one crisp product paragraph without claiming
   generic function support or promising unsupported-surface redesign.

## Red Gate

M19 stays red if any of these are true:

- supported function semantic truth survives semantic input edits as current proof;
- Family B still false-greens on swapped, duplicated, dropped, or mis-threaded arguments;
- unseen Family B examples require canonical pricing names or canonical dep ids to pass;
- unsupported near misses demote official read-side health surfaces;
- the work expands into new semantic families or unsupported-surface redesign to hide gaps in the
  current families;
- backend-readiness is reopened before this gate is green.

## Decision Audit Trail

| # | Decision | Classification | Rationale | Rejected |
|---|---|---|---|---|
| 1 | Treat M18 as substrate progress but gate-red | mechanical | The written M18 gate required unseen proof that is still incomplete. | declare M18 green ceremonially |
| 2 | Make M19 a falsification pack | taste | The next risk is proof quality, not more surface area. | add Family C |
| 3 | Correct semantic freshness before trusting preserve-mode proof | mechanical | Current supported reviews can look current after semantic edits. | rely on existing passport preservation |
| 4 | Tighten Family B argument flow | mechanical | Nesting shape alone can miss wrong parameter flow. | call wrapper shape sufficient |
| 5 | Freeze unsupported behavior | taste | The contract is awkward but redesign is out of scope. | broaden unsupported-surface cleanup |
| 6 | Keep backend-readiness closed | taste | Backend work would multiply false confidence if semantic proof is stale or weak. | reopen backend-readiness after M18 |

## Completion Summary

| Item | Status |
|---|---|
| M18 prior-state audit | written |
| M19 milestone summary | written |
| User outcome | written |
| Approved scope and NOT in scope | written |
| Code seams | written |
| Freshness contract | written |
| Family B argument-flow contract | written |
| Test matrix | written |
| Implementation order | written |
| Green/red gate | written |
| Current status | ready for M19 execution only; backend-readiness remains closed |
