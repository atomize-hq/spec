<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m17-autoplan-restore-20260425-210154.md -->
# M18 - Semantic Review Generalization Gate

Status: **Review-locked implementation contract** (April 25, 2026).

This plan replaces the prior stacked M15.5/M16/M17 planning artifacts with one current execution
contract for the next milestone: prove that semantic review for `kind:function` has a reusable
substrate across more than one bounded family, without slipping back into exact unit-id routing or
pretend-generic function understanding.

UI scope: **no**. This is a backend-only semantic-review milestone for family routing,
compatibility-key migration, truth-surface projection, regression evidence, and product-honest
docs.

## Source Inputs

- Checkpoint:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/checkpoints/20260425-200501-m18-generalization-gate.md`
- Design artifact:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m17-design-20260425-105241.md`
- Current implementation seams:
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`
- Current supported semantic-review surfaces already present in repo:
  - `pricing/discount_policy` (`kind: sum`)
  - `pricing/checkout_quote` (`kind: data`)
  - `pricing/apply_discount` (`kind: function`)
  - `pricing/apply_tax` (`kind: function`)
  - `pricing/calculate_total` currently unsupported, additive-only, non-demoting

## Milestone Summary

```text
M18a  Replace exact-id function routing with bounded family routing                     required
M18b  Introduce family-scoped compatibility keys and deterministic keep/drop migration required
M18c  Generalize arithmetic leaf review to unseen examples                             required
M18d  Add a second family: bounded pipeline wrapper functions                          required
M18e  Lock honest fallback: under_specified vs additive-only neutrality                required
M18f  Add unseen-example proof packs for both families                                required
M18g  Refresh docs and agent workflow text to describe family support honestly         required
M18h  Define the post-M18 green/red gate for backend-readiness                         required
```

## User Outcome

An AI-heavy Rust maintainer adds or edits a new pricing leaf or tiny wrapper function, runs the
normal `spec` loop, and gets one honest answer:

- supported and aligned
- supported and drifted
- supported but under-specified
- unsupported and neutral

That maintainer should not need to memorize a hidden whitelist of exact ids to know whether
semantic review is real for the function they are touching.

## Entry Criteria

- M17-era sum and data support stays intact.
- The current proof rule stays intact:
  `spec test` refreshes semantic truth, `spec build` / `spec generate` / `spec status` /
  `spec export` only project stored truth.
- The implementation remains local to `kind:function`.
- M18 adds no new CLI command and no new artifact type.

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing code surface | Reuse / correction in M18 |
|---|---|---|
| Semantic routing and verdict projection | `spec-core/src/semantic_review.rs` | Reuse one semantic-review entrypoint. Replace exact-id function routing with family routing inside this file instead of adding a second path. |
| Preserve vs refresh truth behavior | `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs` | Reuse the current keep/drop contract. Family-based function support must flow through the same projector. |
| Canonical arithmetic function examples | `examples/ecommerce/units/pricing/apply_discount.unit.spec`, `apply_tax.unit.spec` | Reuse as Family A seen anchors, but migrate from exact-id keys to family keys. |
| Canonical wrapper candidate | `examples/ecommerce/units/pricing/calculate_total.unit.spec` | Reuse as the Family B seen anchor only if it honestly fits the bounded wrapper contract. |
| Command-matrix regressions | `spec-cli/tests/cli.rs` | Reuse and extend preserve/refresh tests so family keys follow the same CLI truth rules. |
| End-to-end semantic wedges | `spec-cli/tests/m14_regressions.rs` | Reuse as the canonical proof harness. Extend from exact-id function wedges to family-based wedges. |
| Existing supported seam coverage | `pricing/discount_policy`, `pricing/checkout_quote` | Keep as regression cross-checks so M18 does not break already-landed supported surfaces. |
| Product-honest docs | `README.md`, `AGENTS.md`, `examples/ecommerce/README.md` | Rewrite the support story from exact ids to bounded families. |

### Minimum diff that still solves the problem

- Introduce family routing for `kind:function` only.
- Keep `sum` and `data` support behavior unchanged.
- Replace exact supported-function compatibility keys with family-scoped keys.
- Admit exactly two bounded families in M18:
  - Family A: arithmetic leaf transforms
  - Family B: two-step pipeline wrapper functions
- Reuse the current proof surfaces and current artifact model.
- Add unseen-example fixtures and canonical regressions for both families.

### Complexity check

The expected blast radius stays bounded to:

- `spec-core/src/semantic_review.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- support-story docs

If M18 starts adding graph-wide reasoning, a second classifier subsystem, or new authored schema
fields, stop and split the work. That is ocean behavior.

### Search check

- **[Layer 1]** Reuse the current compatibility-key preserve/drop contract.
- **[Layer 1]** Reuse the existing normalized function representation instead of inventing a
  second parser path.
- **[Layer 3]** Generalization should mean family-scoped support from authored plus executable
  structure, not support one more blessed function forever.

### TODO cross-reference

- Keep the Cargo-heavy CLI harness cleanup in `TODOS.md` out of M18 scope.
- Keep generic `kind:function` understanding, cross-unit semantic coherence, and second-backend
  work out of M18 scope.
- If Family B proves too narrow and the real next hole is branching wrappers or predicate families,
  capture that as follow-on work after M18 rather than widening M18.

### Completeness check

The complete move is family routing, family keys, honest fallback, unseen-example proof packs, and
doc refresh together.

The shortcut is "add three more exact ids and call it generalized." Reject that. It saves almost
nothing and leaves the product story fake-green.

### Distribution check

M18 introduces no new artifact type. Existing CLI packaging and release machinery remain
sufficient. The deliverable is implementation, regression evidence, and doc honesty.

## Architecture Review

M18 widens `kind:function` support without changing the core trust model. The substrate stays
shared. The family-local semantics vary. The fallback stays honest.

### Ownership split

| Layer | Owns | Must not own |
|---|---|---|
| Supported family router | which bounded family, if any, a function belongs to | generic `kind:function` support |
| Authored packet builder | contract, invariants, deps, and stable authored cues used by family routing | free-text NLP or exact-id whitelists |
| Executable packet builder | normalized signature plus trimmed body shape | whole-program reasoning |
| Family classifier | `aligned` / `semantic_drift` / `under_specified` inside one admitted family subset | persistence, health precedence, or schema projection |
| Truth-surface projection | when to refresh, keep, or drop stored review | inventing truth during non-proof flows |
| Doc layer | what users are told is supported | language that implies broader support than the code actually has |

### Module plan

| Module | Current role | M18 change |
|---|---|---|
| `spec-core/src/semantic_review.rs` | exact-id function routing, supported-surface evaluation, keep/drop compatibility matching | Replace exact-id function support with family router plus family descriptors and bounded classifiers. Keep one entrypoint. |
| `spec-core/src/passport.rs` | passport rebuild and proof-state projection | Preserve current projector shape. Update tests so old exact-id function reviews drop when the current family key differs. |
| `spec-core/src/export.rs` | export bundle assembly and preserve-mode projection | Preserve behavior. Update tests so export never invents family-based truth and drops mismatched old keys. |
| `spec-cli/src/commands.rs` | command matrix, status/export projection, test refresh | Preserve command semantics. Refresh canonical family reviews only during `spec test`. |
| `spec-cli/tests/cli.rs` | command-matrix truth assertions | Update exact-id supported-function fixtures to family-keyed fixtures. Add migration and neutrality regressions. |
| `spec-cli/tests/m14_regressions.rs` | canonical wedge proofs | Replace exact-id function wedge assumptions with family proofs, including unseen aligned, drift, and under-specified cases for both families. |

### Dependency graph

```text
                    spec-core/src/semantic_review.rs
                                   │
                  ┌────────────────┼────────────────┐
                  │                │                │
                  │                │                │
         family router      packet builders   family classifiers
                  │                │                │
                  └────────────────┴────────────────┘
                                   │
                         compatibility key + verdict
                                   │
                    spec-core/src/passport.rs (projector)
                                   │
                     ┌─────────────┴─────────────┐
                     │                           │
     spec-core/src/export.rs          spec-cli/src/commands.rs
                     │                           │
                     └─────────────┬─────────────┘
                                   │
                   spec-cli/tests/cli.rs and m14_regressions.rs
```

### Supported-family routing

`kind:function` routing becomes deterministic and family-based:

```text
function unit
  │
  ├── Family A eligibility?
  │      ├── arithmetic_leaf.monotone_down_nonnegative.v1
  │      ├── arithmetic_leaf.monotone_up.v1
  │      └── no
  │
  ├── Family B eligibility?
  │      ├── wrapper.pipeline.v1
  │      └── no
  │
  └── unsupported.function.v1
```

### Compatibility-key migration

Current exact-id keys:

- `function.apply_discount.v1`
- `function.apply_tax.v1`

M18 family keys:

- `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `function.arithmetic_leaf.monotone_up.v1`
- `function.wrapper.pipeline.v1`

Migration rule:

- `pricing/apply_discount` refreshes from its exact-id key to
  `function.arithmetic_leaf.monotone_down_nonnegative.v1`
- `pricing/apply_tax` refreshes from its exact-id key to
  `function.arithmetic_leaf.monotone_up.v1`
- `pricing/calculate_total`, if admitted, refreshes to `function.wrapper.pipeline.v1`
- Preserve-mode drops stored exact-id function reviews once the surface is now family-keyed and
  the compatibility key no longer matches

Old proof should not silently survive a meaningfully different support model.

### Family A contract: arithmetic leaf transforms

**Admitted roles**

1. `monotone_down_nonnegative`
2. `monotone_up`

**Authored eligibility**

- `kind: function`
- returns `Decimal`
- exactly two `Decimal` inputs
- zero or one helper dep
- role is derived from invariants, not id:
  - `monotone_down_nonnegative` requires invariants equivalent to:
    - `output <= input0`
    - `output >= 0`
  - `monotone_up` requires an invariant equivalent to:
    - `output >= input0`

**Exact authored invariant normalization**

The authored-packet builder uses a closed, non-algebraic normalization rule before routing:

1. trim whitespace
2. strip one layer of redundant outer parentheses
3. rewrite the declared first and second input identifiers to `input0` and `input1`
4. rewrite the return slot to `output`
5. canonicalize `Decimal::ZERO` to `0`

After that normalization, M18 admits only these exact atomic invariant strings for Family A:

- `output <= input0`
- `output >= 0`
- `output >= input0`

Nothing else is treated as equivalent in M18. No algebraic simplification, no inequality flipping,
no synonym table. If the authored invariant does not normalize to one of the strings above, the
unit stays neutral.

**Helper dep rule**

For Family A, "zero or one helper dep" means exactly this:

- zero deps, or
- one declared dep used exactly once as the outermost call that wraps the final arithmetic result

That helper dep is structural only in M18. The router does not infer helper semantics from dep
name or `intent.why`. Any second dep, any repeated helper use, or any helper call that is not the
outermost wrapper pushes the unit out of the admitted Family A subset.

**Executable honest subset**

- zero or one local binding, then return
- no branching
- no loops
- no `match`
- no dependency chain beyond the optional helper dep

**Accepted aligned shapes**

`monotone_down_nonnegative`

- `round((base - base * rate).max(Decimal::ZERO))`
- `let discounted = base - base * rate; round(discounted.max(Decimal::ZERO))`

`monotone_up`

- `round(base + base * rate)`
- `let taxed = base + base * rate; round(taxed)`

**Exact executable matcher**

The executable matcher for Family A is also closed:

- allowed statements:
  - direct return expression, or
  - exactly one `let` binding followed by the return expression
- allowed arithmetic core:
  - `input0 - input0 * input1` for `monotone_down_nonnegative`
  - `input0 + input0 * input1` for `monotone_up`
- allowed clamp:
  - `.max(0)` only for `monotone_down_nonnegative`
- allowed helper wrapping:
  - zero helper call, or one outermost helper call around the final expression

The matcher does not admit reordered arithmetic, extra locals, extra arithmetic operators, method
chains other than the one clamp, or nested helper calls. Those cases are either
`semantic_drift` when they contradict the admitted role directly, or `under_specified` when they
leave the admitted Family A subset without proving contradiction.

**Recognized drift**

- sign inversion
- missing clamp on `monotone_down_nonnegative`
- missing round when round is part of the admitted authored or executable shape
- additive body for a subtractive authored role, or vice versa

**Under-specified inside Family A**

- authored invariants admit the family role, but the body uses branching
- body shape performs extra arithmetic steps outside the admitted subset
- body shape introduces additional deps or helper calls

**Still neutral**

- function does not satisfy the authored eligibility packet at all
- function returns `Decimal` but invariants are too weak to admit either role

### Family B contract: bounded wrapper pipeline functions

**Admitted role**

- `wrapper.pipeline`

**Authored eligibility**

- `kind: function`
- exactly two declared deps
- deps are already supported semantic surfaces or explicitly admitted by the current proof run
- contract is top-level function truth, not seam-local lowering
- invariants stay local and boring, with no graph-wide claims

**Exact dep-admission rule**

"Explicitly admitted by the current proof run" means:

- the dep id resolves in the currently loaded spec set for the same invocation
- the router computes support from current in-memory authored plus executable data, not from stored
  passports
- the dep must currently classify as one of:
  - Family A supported function
  - supported `sum`
  - supported `data`

Family B does not stack on another Family B wrapper in M18. That keeps the milestone local and
prevents hidden graph reasoning.

**Executable honest subset**

- either:
  - one intermediate local from dep A, then return dep B using that local
  - or a direct nested `dep_b(dep_a(...), ...)`
- no branching
- no loops
- no extra arithmetic around the dep chain
- each declared dep used once
- dep order preserved
- final call consumes the intermediate output of the first call

**Exact executable matcher**

M18 admits only these two wrapper skeletons:

1. `let tmp = dep_a(...); dep_b(tmp, ...)`
2. `dep_b(dep_a(...), ...)`

Within those skeletons, all of these must hold:

- `dep_a` is the first declared dep and `dep_b` is the second declared dep
- each dep is called exactly once
- the threaded result from `dep_a` must occupy argument slot `0` of `dep_b`
- every non-threaded call argument must be a bare top-level wrapper parameter identifier
- no literals, method chains, arithmetic expressions, closures, matches, or nested calls are
  allowed in call arguments
- in the `let tmp` form, `tmp` is used exactly once, only as the first argument to `dep_b`

If the body reverses dep order or bypasses a dep, it is `semantic_drift`.
If the body keeps the broad pipeline shape but adds extra locals, duplicate dep use, or
non-identifier argument expressions, it is `under_specified`.

**Canonical seen example**

- `pricing/calculate_total`

**Recognized drift**

- dep order reversed
- one declared dep omitted
- first dep result not threaded into second dep
- final return bypasses a declared dep
- extra arithmetic wrapped around the pipeline

**Under-specified inside Family B**

- admitted authored packet, but body uses extra locals, branches, or duplicate dep usage beyond
  the honest subset
- body shape makes it impossible to say whether the wrapper preserves the declared dependency
  pipeline

**Still neutral**

- function has multiple deps but does not satisfy the admitted two-step pipeline shape
- function delegates into unsupported callees or generic orchestration outside the admitted subset

### Truth-surface contract

This boundary is the milestone.

- **Unsupported and neutral**
  - the unit never qualifies for any admitted family packet
  - stored review, if any, stays additive-only and non-demoting
- **Supported but under-specified**
  - the unit qualifies for a family packet
  - executable body or authored truth leaves that family's admitted honest subset

Similar-looking code does not get demoted unless the router first admits it into a supported
family.

### Truth state machine

```text
None
  │
  ├── Refresh on supported family surface
  │      └── Review{compatibility_key, verdict}
  │
  ├── Refresh on unsupported surface
  │      └── AdditiveUnsupportedReview
  │
  └── Preserve
         └── None

Review{key=A}
  │
  ├── Preserve on same key
  │      └── keep
  │
  ├── Preserve on different key
  │      └── drop to None
  │
  ├── Refresh on supported new family key
  │      └── recompute with new key
  │
  └── Refresh on unsupported surface
         └── unsupported additive metadata only
```

## Code Quality Review

The main quality risk is accidental duplication: one shared substrate on paper, but three subtly
different logic paths in code.

### Code-quality rules

- Keep one semantic-review entrypoint in `spec-core/src/semantic_review.rs`.
- Add one explicit family descriptor layer, not several disconnected classifier modules.
- Reuse the normalized function representation. Do not parse authored YAML or raw Rust twice.
- Keep routing deterministic and explicit. No keyword scanning in `intent.why`.
- Keep `unsupported.function.v1` additive-only and non-demoting for out-of-family functions.
- Limit doc changes to:
  - `PLAN.md`
  - `README.md`
  - `AGENTS.md`
  - `examples/ecommerce/README.md`
- Add inline ASCII diagrams near:
  - family routing in `spec-core/src/semantic_review.rs`
  - keep/drop migration behavior in `spec-core/src/passport.rs`

### Existing patterns to preserve

- `project_semantic_review` already centralizes the dangerous keep/drop logic. M18 extends that
  contract instead of bypassing it.
- The repo already treats `spec test` as the only proof-writing flow. Keep that line sharp.

### Anti-patterns to avoid

- growing an ever-longer exact-id support list
- family-specific persistence codepaths
- string heuristics masquerading as semantic support detection

## Test Review

100% new-path coverage is the goal. M18 only earns the word "generalization" if unseen examples
are the main proof, not a footnote.

### New codepaths

```text
FAMILY ROUTING
  - exact-id function support replaced by family routing
  - exact-id function keys migrate to family keys
  - unsupported near-miss functions remain neutral

FAMILY A
  - monotone_down_nonnegative routing
  - monotone_up routing
  - aligned / drift / under_specified classification

FAMILY B
  - wrapper.pipeline routing
  - aligned / drift / under_specified classification

TRUTH SURFACES
  - spec test refreshes family-based keys
  - build/generate/status/export keep or drop only
  - stale base health still outranks semantic demotion

DOC HONESTY
  - support story says bounded families, not exact ids and not generic function support
```

### Coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] Existing shared invariants
    │
    ├── [★★★ TESTED] preserve/drop truth loop already exists
    ├── [★★★ TESTED] stale base health outranks semantic demotion
    └── [★★★ TESTED] unsupported surfaces can stay additive-only

[+] spec-core/src/semantic_review.rs
    │
    ├── [GAP] exact-id function keys migrate to family keys
    ├── [GAP] Family A role routing from authored packet
    ├── [GAP] Family B routing from deps + body packet
    ├── [GAP] unsupported near-miss remains neutral
    ├── [GAP] Family A aligned unseen example
    ├── [GAP] Family A drift unseen example
    ├── [GAP] Family A under_specified unseen example
    ├── [GAP] Family B aligned unseen example
    ├── [GAP] Family B drift unseen example
    └── [GAP] Family B under_specified unseen example

[+] spec-core/src/passport.rs / spec-core/src/export.rs / spec-cli/src/commands.rs
    │
    ├── [GAP] preserve drops old exact-id reviews on family-key mismatch
    ├── [GAP] spec test refreshes apply_discount to Family A key
    ├── [GAP] spec test refreshes apply_tax to Family A key
    ├── [GAP] spec test refreshes calculate_total to Family B key
    └── [GAP] status/export never invent family-based truth

[+] User-facing truth loops
    │
    ├── [GAP] canonical ecommerce loop stays honest
    ├── [GAP] unseen Family A proof pack stays honest
    ├── [GAP] unseen Family B proof pack stays honest
    └── [GAP] docs match the real support boundary

---------------------------------
COVERAGE TARGET: every new M18 path lands at ★★★
REQUIRED NEW EVIDENCE:
  Family A -> unseen aligned + drift + under_specified
  Family B -> unseen aligned + drift + under_specified
---------------------------------
```

### Required test matrix

- Unit tests in `spec-core/src/semantic_review.rs`
  - exact-id function key migration to family keys
  - Family A authored-role routing
  - Family B pipeline routing
  - unsupported near-miss remains neutral
  - Family A aligned / drift / under_specified fixtures
  - Family B aligned / drift / under_specified fixtures
- Projection tests in `spec-core/src/passport.rs` and `spec-core/src/export.rs`
  - preserve drops old exact-id function review when current family key differs
  - preserve keeps current family review when keys match
  - export and status project stored truth only
- CLI regressions in `spec-cli/tests/cli.rs`
  - `spec test` refreshes family-based reviews for canonical functions
  - `spec build`, `spec generate`, `spec status`, and `spec export` do not mint replacement truth
  - unsupported near-miss functions remain neutral through the full command matrix
  - stale base health still wins when a family-supported function also has semantic review
- Canonical and unseen proof packs in `spec-cli/tests/m14_regressions.rs`
  - Family A canonical seen examples stay aligned
  - Family B canonical `calculate_total` aligns if it fits the admitted wrapper contract
  - unseen aligned / drift / under_specified wedges for both families
  - existing `discount_policy`, `checkout_quote`, and molecule coverage still compose

### Fixture strategy

- Keep the canonical seen examples in `examples/ecommerce/units/pricing/`.
- Add unseen M18 fixture packs under `spec-cli/tests/fixtures/m18/`.
- Do not hide the whole milestone in test-only fake units. The canonical ecommerce story must
  still exercise both families.

### Regression rule

These regressions are mandatory:

- exact-id function reviews must drop on preserve-mode mismatch after M18 family routing lands
- unsupported near-miss functions must remain additive-only neutral
- Family B must fail on reversed dependency order
- Family A must fail when the admitted role and executable sign or clamp behavior disagree

### Canonical M18 wedge loop

```text
cargo test -p spec-core semantic_review -- --nocapture
cargo test -p spec-core passport -- --nocapture
cargo test -p spec-cli --test cli -- --nocapture
cargo test -p spec-cli --test m14_regressions -- --nocapture
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_discount.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/calculate_total.unit.spec
cargo run -p spec-cli -- status examples/ecommerce --format json
```

## Performance Review

M18 should stay boring on runtime cost.

- Family routing stays local to one function body and one authored packet.
- No graph traversal.
- No transitive semantic tracing through arbitrary dep trees.
- Refresh remains proof-flow only.
- Any body outside the admitted tiny subsets should fall out quickly as unsupported or
  under-specified.

If the implementation needs AST walks large enough to warrant caching or graph analysis, the scope
has already drifted too far.

## Failure Modes

| Codepath | Failure mode | Test? | Error handling? | User sees? | Logged? | Critical gap? |
|---|---|---:|---:|---|---:|---:|
| family router | hidden exact-id branch still decides support | yes | yes | fake-green "generalization" | yes | **yes** |
| key migration | old exact-id review survives preserve-mode | yes | yes | stale semantic truth looks current | yes | **yes** |
| Family A classifier | wrong sign, missing clamp, or wrong rounding still aligns | yes | yes | pricing drift not caught | yes | **yes** |
| Family B classifier | wrapper skips or reorders deps and still aligns | yes | yes | orchestration drift not caught | yes | **yes** |
| fallback boundary | near-miss function demotes because router is too broad | yes | yes | unsupported code suddenly looks broken | yes | **yes** |
| docs | README or AGENTS still describe exact ids or generic function support | yes | n/a | maintainer overtrusts product claims | n/a | **yes** |

Any row with missing test coverage is a ship blocker. M18 is a trust milestone.

## What NOT in M18 Scope

- generic support for arbitrary `kind:function` units
- branching or looping wrapper semantics
- cross-unit or whole-graph semantic coherence
- predicate families, boolean families, or third-family experiments
- second-backend work
- new CLI commands or new artifact types
- CLI harness cleanup from `TODOS.md`
- changes to `sum` or `data` semantic families beyond compatibility regressions

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| 1. Lock family routing + family keys | `spec-core/src`, `spec-cli/src` | - |
| 2. Implement Family A classifier + unseen fixtures | `spec-core/src`, `spec-cli/tests`, `examples/ecommerce/units/pricing` | 1 |
| 3. Implement Family B classifier + unseen fixtures | `spec-core/src`, `spec-cli/tests`, `examples/ecommerce/units/pricing` | 1 |
| 4. Add preserve/drop migration regressions | `spec-core/src`, `spec-cli/src`, `spec-cli/tests` | 1 |
| 5. Refresh docs and workflow text | repo docs only | 1 |
| 6. Re-prove canonical truth loop and unseen packs | `spec-cli/tests`, evidence artifacts, ecommerce example outputs | 2, 3, 4 |

### Parallel lanes

- **Gate 0, sequential:** Step 1 must land first. Family routing and key vocabulary are the
  shared contract.
- **Lane A:** Step 2
  - Family A role routing, classifier, and unseen proof pack
- **Lane B:** Step 3
  - Family B pipeline routing, classifier, and unseen proof pack
- **Lane C:** Step 4
  - preserve/drop migration, unsupported neutrality, and status/export regressions
- **Lane D:** Step 5
  - docs and workflow honesty updates
- **Lane E:** Step 6
  - final canonical and unseen proof run after A, B, and C merge

### Execution order

1. Lock Step 1.
2. Launch Lanes A, B, C, and D in parallel worktrees.
3. Merge A, B, C, and D.
4. Run Lane E last for full trust-loop verification.

### Conflict flags

- `spec-core/src/semantic_review.rs` is the main conflict magnet. One owner should coordinate the
  family router and shared classifier composition.
- `spec-cli/tests/cli.rs` and `spec-cli/tests/m14_regressions.rs` are the second conflict magnets.
  Keep migration regressions and unseen proof packs batched by lane.
- `examples/ecommerce/units/pricing/` is shared between Family A and Family B proof work. If both
  lanes need to touch canonical examples, agree on ownership before parallel execution.
- Docs can run in parallel, but should not merge before the family vocabulary is locked.

## Implementation Order

```text
1. Replace exact-id function routing with family routing and new family keys
2. Define Family A authored/executable packets and classifier
3. Define Family B authored/executable packets and classifier
4. Add preserve/drop migration behavior for old exact-id function reviews
5. Add unseen aligned / drift / under_specified proof packs for both families
6. Re-run canonical ecommerce truth loops and projection regressions
7. Rewrite support-story docs to match the new family boundary
8. Evaluate the post-M18 green/red gate
```

## Success Criteria / Post-M18 Gate

M18 is green only if all of these are true:

1. `pricing/apply_discount` and `pricing/apply_tax` refresh under Family A keys, not exact-id
   keys.
2. `pricing/calculate_total` either:
   - honestly fits `function.wrapper.pipeline.v1` and projects `aligned`, `semantic_drift`, and
     `under_specified`, or
   - stays unsupported and forces the milestone red. No silent fallback.
3. Family A and Family B each pass the unseen-example bar:
   `aligned` + `semantic_drift` + `under_specified` on unseen examples.
4. Unsupported near-miss functions remain additive-only neutral and non-demoting.
5. `spec build`, `spec generate`, `spec status`, and `spec export` never mint replacement family
   truth.
6. `README.md`, `AGENTS.md`, and example docs describe bounded family support honestly.

**Green gate**

- If all six conditions hold, backend-readiness can reopen as the next planning question.

**Red gate**

- Stay on semantic-core correction if any of these are true:
  - unseen examples still require bespoke exact-id exceptions
  - Family B needs graph reasoning or branching semantics to make the canonical case green
  - router broadening causes unsupported near-miss functions to demote
  - docs cannot describe the supported surface in one crisp paragraph without caveats

## Dream State Delta

- **Before M18**
  - function semantic review is still effectively a bounded exact-id story
  - `calculate_total` remains neutral, so the support story does not yet travel into bounded
    composition
  - docs are forced to describe support as a narrow named set

- **After M18**
  - function semantic review is routed by explicit bounded families, not exact ids
  - arithmetic leaves and tiny pipeline wrappers can be evaluated on unseen examples
  - unsupported functions still stay honest and neutral
  - the product story becomes:
    "semantic review supports bounded function families with explicit fallback"

## M18 Review-Locked Decisions

- M18 is a generalization gate, not a "support one more pricing function" milestone.
- Family A is arithmetic leaf transforms. Family B is bounded two-step pipeline wrappers.
- `kind:function` support routes by family eligibility and compatibility keys, not exact ids.
- Exact-id M17 function keys are intentionally replaced and dropped on preserve mismatch.
- Unsupported but similar functions stay additive-only neutral unless the router first admits them
  into a supported family packet.
- `pricing/calculate_total` is allowed into M18 only if it fits the Family B contract honestly.
  It does not define the family by itself.
- Backend-readiness stays closed until M18 passes the unseen-example bar across both families.

## Completion Summary

| Item | Status |
|---|---|
| Scope challenge | written |
| What already exists | written |
| Architecture review | written |
| Code quality review | written |
| Test review | diagram + matrix + wedge loop written |
| Performance review | written |
| Failure modes | written |
| NOT in scope | written |
| Parallelization | written |
| Post-M18 gate | written |
| Current status | ready for implementation planning against M18 |

## Decision Audit Trail (M18)

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | Scope | Replace exact-id function routing with family routing | mechanical | explicit over clever | Generalization must travel to unseen examples, not more ids | add more named supported functions |
| 2 | Scope | Keep exactly two admitted families in M18 | taste | boil the lake, not the ocean | Enough to prove travel without drifting into generic semantics | one family only, or three families at once |
| 3 | Architecture | Use family-scoped compatibility keys | mechanical | DRY + reversibility | Keeps preserve/drop deterministic during migration | retain exact-id function keys forever |
| 4 | Family B | Admit only two-step pipeline wrappers | mechanical | engineered enough | Smallest composed-function contract that pressures the thesis | branching wrappers, arbitrary orchestration |
| 5 | Fallback | Neutral if not admitted, under_specified if admitted-but-ambiguous | mechanical | honesty over coverage theater | Prevents fake-green and fake-red alike | demote all similar functions |
| 6 | Proof bar | Require unseen aligned / drift / under_specified examples per family | mechanical | completeness | M18 is not credible without unseen travel | canonical examples only |
| 7 | Gate | Keep backend-readiness closed until both families pass | taste | focus as subtraction | Avoids calling the semantic core done too early | backend-readiness immediately after Family A |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | COMPLETE | M18 reframed as shared substrate plus two-family proof; one-family generalization rejected |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | COMPLETE | Family contracts, migration keys, fallback boundary, unseen-example bar, test matrix, dependency graph, and parallel lanes locked |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | No UI scope |

**UNRESOLVED:** 0

**VERDICT:** CEO + ENG WRITTEN. `feat/m17` is ready for implementation against M18, with
backend-readiness explicitly blocked until the M18 gate passes.
