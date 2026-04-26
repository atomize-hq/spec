# M18 — Semantic Review Generalization Gate

Status: **Draft, CEO-reviewed and eng-solidified** (April 25, 2026). This plan replaces the old
stacked M15.5/M16/M17 planning artifact with one current implementation contract for the next
milestone: prove that semantic review has a reusable substrate across more than one bounded family,
without slipping back into exact unit-id routing or fake-generic function understanding.

UI scope: **no**. This is a backend-only semantic-review milestone for family routing,
compatibility-key migration, proof surfaces, test evidence, and product-honest docs.

## Source Inputs

- Checkpoint:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/checkpoints/20260425-200501-m18-generalization-gate.md`
- Design artifact:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m17-design-20260425-105241.md`
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

**Lake to boil in M18**

- Prove that semantic review for `kind:function` is no longer just a longer whitelist of named
  pricing examples.
- Keep one shared truth loop:
  `spec test` refreshes truth, `spec build/generate/status/export` only project stored truth.
- Prove travel across two bounded families:
  - Family A: arithmetic leaf transforms
  - Family B: bounded wrapper/pipeline functions
- Keep unsupported or overly broad functions honest:
  additive-only neutral if they never qualify, `under_specified` if they qualify but leave the
  admitted honest subset.

**User job**

- An AI-heavy Rust maintainer adds or edits a new pricing leaf or tiny wrapper function, runs the
  normal `spec` loop, and gets one honest answer:
  supported and aligned, supported and drifted, supported but under-specified, or unsupported and
  neutral.
- That maintainer does **not** need to memorize a hidden whitelist of exact ids to know whether
  semantic review is real for the function they are touching.

## Step 0: Scope Challenge

### Current system state

| Surface | Already proved | Still missing | M18 implication |
|---|---|---|---|
| `pricing/discount_policy` (`kind: sum`) | One honest supported seam with explicit helper/example filtering and trust-surface projection | Generalized function-family routing | Keep the preserve/drop contract and verdict projection. Do not re-solve sum semantics. |
| `pricing/checkout_quote` (`kind: data`) | The compatibility-key keep/drop contract travels beyond `sum` | Generic function-family travel | Reuse the same truth loop. Do not widen data-seam semantics in M18. |
| `pricing/apply_discount` + `pricing/apply_tax` (`kind: function`) | Two explicit supported function ids can project semantic truth honestly | Family-based support that works on unseen examples | M18 should replace id-based routing, not bolt more ids onto it. |
| `pricing/calculate_total` (`kind: function`) | Additive-only neutrality still works for unsupported orchestration | Honest bounded composition semantics | This becomes the canonical Family B candidate if it fits the final bounded wrapper contract. |

### What already exists

| Sub-problem | Existing code surface | M18 reuse / correction |
|---|---|---|
| Truth-surface projection | `spec-core/src/semantic_review.rs`, `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs` | Reuse one shared refresh vs preserve pipeline. Do not create a second persistence path for family-based function review. |
| Canonical arithmetic functions | `examples/ecommerce/units/pricing/apply_discount.unit.spec`, `apply_tax.unit.spec` | Reuse as the seen Family A anchors, but migrate them from exact-id keys to family keys. |
| Canonical wrapper candidate | `examples/ecommerce/units/pricing/calculate_total.unit.spec` | Reuse as the seen Family B anchor if it fits the final bounded pipeline contract honestly. |
| Existing data + sum semantic seams | `pricing/discount_policy`, `pricing/checkout_quote` | Reuse as regression cross-checks so M18 does not break already-landed supported surfaces. |
| Unsupported-surface neutrality tests | `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs` | Reuse and extend so near-miss functions do not silently start demoting health in M18. |
| Product-honest docs | `README.md`, `AGENTS.md`, `examples/ecommerce/README.md` | Reuse but rewrite the support story from exact ids to bounded families. |

### Minimum diff that still solves the problem

- Introduce family routing for `kind:function` only.
- Keep `sum` and `data` support exactly as they are in M17-era code.
- Replace exact supported-function compatibility keys with family-scoped keys.
- Admit exactly two bounded function families in M18:
  - Family A: arithmetic leaf transforms
  - Family B: two-step pipeline wrapper functions
- Reuse the existing proof surfaces. M18 adds **no** new CLI command and **no** new artifact type.
- Add unseen example fixtures and canonical regressions for both families.

### Complexity check

- Expected blast radius should stay bounded to:
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - docs describing semantic-review support
- If M18 starts adding generic graph reasoning, a second classifier subsystem, or new authored
  schema fields, stop and split the work. That is ocean behavior.

### Search check

- **[Layer 1]** Reuse the existing compatibility-key preserve/drop contract. It already solves the
  dangerous part: stored truth must be kept or dropped deterministically.
- **[Layer 1]** Reuse the existing normalized function path instead of inventing a parallel parser.
- **[Layer 3]** Generalization should mean “family-scoped support from authored + executable
  structure,” not “support one more named function forever.”

### TODO cross-reference

- Keep the Cargo-heavy CLI harness cleanup in `TODOS.md` out of M18 scope.
- Keep generic `kind:function` understanding, cross-unit semantic coherence, and second-backend
  work out of M18 scope.
- If Family B proves too narrow and the real next hole is “branching wrappers” or “predicate
  families,” capture that as follow-on work after M18, not inside it.

### Completeness check

- The complete move is family routing + family keys + honest fallback + unseen-example proof packs
  + doc refresh together.
- The shortcut is “add three more exact ids and call it generalized.” Reject that. It saves almost
  nothing and leaves the product story fake-green.

### Distribution check

- M18 introduces no new artifact type.
- Existing CLI packaging and release machinery remain sufficient.
- The deliverable is implementation + regression evidence + doc honesty, not CI or release work.

## Architecture Review

M18 should widen `kind:function` support without changing the core trust model. The substrate stays
shared. The family-local semantics vary. The fallback stays honest.

### Ownership split

| Layer | Owns | Must not own |
|---|---|---|
| Supported family router | which bounded family, if any, a function belongs to | generic `kind:function` support |
| Authored packet builder | contract, invariants, deps, and stable authored cues used by family routing | free-text NLP or exact-id whitelists |
| Executable packet builder | normalized signature plus trimmed body shape | whole-program reasoning |
| Family classifier | aligned / semantic_drift / under_specified inside one admitted family subset | persistence, health precedence, or schema projection |
| Truth-surface projection | when to refresh, keep, or drop stored review | inventing truth during non-proof flows |
| Doc layer | what users are told is supported | language that implies broader support than the code actually has |

### Supported-family routing

`kind:function` routing should become deterministic and family-based:

```text
function unit
  │
  ├── existing non-function supported seams?
  │      └── no, continue
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

M17-era exact keys:

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
- Preserve-mode drops stored exact-id function reviews once the surface is now family-keyed and the
  keys no longer match

This is a good thing. Old proof should not silently survive a meaningfully different support model.

### Family A contract: arithmetic leaf transforms

Family A is the first proving family, not the whole milestone.

**Admitted roles**

1. `monotone_down_nonnegative`
2. `monotone_up`

**Authored eligibility**

- `kind: function`
- returns `Decimal`
- exactly two `Decimal` inputs
- zero or one helper dep
- no branching requirement is inferred from authored fields alone
- role is derived from invariants, not id:
  - `monotone_down_nonnegative` requires explicit invariants equivalent to:
    - `output <= input0`
    - `output >= 0`
  - `monotone_up` requires an explicit invariant equivalent to:
    - `output >= input0`

**Executable honest subset**

- zero or one local binding, then return
- no branching
- no loops
- no match
- no dependency chain beyond the optional helper dep

**Accepted aligned shapes**

`monotone_down_nonnegative`

- `round((base - base * rate).max(Decimal::ZERO))`
- `let discounted = base - base * rate; round(discounted.max(Decimal::ZERO))`

`monotone_up`

- `round(base + base * rate)`
- `let taxed = base + base * rate; round(taxed)`

**Recognized drift**

- sign inversion
- missing clamp on `monotone_down_nonnegative`
- missing round when round is part of the admitted authored/executable shape
- additive body for a subtractive authored role, or vice versa

**Under-specified inside Family A**

- authored invariants admit the family role, but the body uses branching
- body shape performs extra arithmetic steps outside the admitted subset
- body shape introduces additional deps or helper calls

**Still neutral**

- function does not satisfy the authored eligibility packet at all
- function returns `Decimal` but invariants are too weak to admit either role

### Family B contract: bounded wrapper pipeline functions

Family B is the second proving family. It is how M18 earns the word “generalization.”

**Admitted role**

- `wrapper.pipeline`

**Authored eligibility**

- `kind: function`
- exactly two declared deps
- deps are already supported semantic surfaces or explicitly admitted by the current proof run
- contract is top-level function truth, not seam-local lowering
- invariants stay local and boring, no graph-wide claims

**Executable honest subset**

- either:
  - one intermediate local from dep A, then return dep B using that local
  - or a direct nested dep B(dep A(...), ...)
- no branching
- no loops
- no extra arithmetic around the dep chain
- each declared dep used once
- dep order preserved
- final call consumes the intermediate output of the first call

**Canonical seen example**

- `pricing/calculate_total`

**Recognized drift**

- dep order reversed
- one declared dep omitted
- first dep result not threaded into second dep
- final return bypasses a declared dep
- extra arithmetic wrapped around the pipeline

**Under-specified inside Family B**

- admitted authored packet, but body uses extra locals, branches, or duplicate dep usage beyond the
  honest subset
- body shape makes it impossible to say whether the wrapper preserves the declared dependency
  pipeline

**Still neutral**

- function has multiple deps but does not satisfy the admitted two-step pipeline shape
- function delegates into unsupported callees or generic orchestration outside the admitted subset

### Honest fallback contract

This boundary must be explicit:

- **Unsupported and neutral**
  - unit never qualifies for any admitted family packet
  - stored review, if any, stays additive-only and non-demoting
- **Supported but under-specified**
  - unit qualifies for a family packet
  - executable body or authored truth leaves that family’s admitted honest subset

That is the core honesty rule. Similar-looking code does **not** get demoted unless the router has
first admitted it into a supported family.

### Full system architecture

```text
Loaded function unit
  │
  ├── build authored packet
  │      ├── inputs / returns
  │      ├── invariants
  │      ├── deps
  │      └── stable authored cues
  │
  ├── build executable packet
  │      ├── normalized signature
  │      └── normalized body shape
  │
  ├── supported family router
  │      ├── Family A role?
  │      ├── Family B role?
  │      └── unsupported.function.v1
  │
  ├── family classifier
  │      ├── aligned
  │      ├── semantic_drift
  │      └── under_specified
  │
  └── truth-surface projection
         ├── spec test    -> Refresh -> write semantic_review
         ├── spec build   -> Preserve -> keep/drop only
         ├── spec generate-> Preserve -> keep/drop only
         ├── spec status  -> Preserve -> project only
         └── spec export  -> Preserve -> project only
```

### Semantic-review state machine

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

### Production failure scenarios

| Integration point | Realistic failure | Plan response |
|---|---|---|
| family router | exact-id shortcut remains in one branch and silently masks family routing bugs | add direct resolver regressions that assert unseen examples route by family and no longer need named ids |
| key migration | old stored exact-id function review survives preserve-mode and falsely looks current | preserve-mode mismatch must drop stored review until `spec test` refreshes under the new family key |
| Family B classifier | pipeline wrapper gets treated as aligned even when dep order is reversed | add dedicated drift fixtures that reverse dep order and assert `semantic_drift` |
| fallback boundary | near-miss function starts demoting because router admits too broadly | add neutrality regressions for unsupported but structurally similar functions |
| docs | README/AGENTS still imply “supported function ids” or “all functions” | update all support-story docs in same milestone and gate ship on wording audit |

### Rollback posture

- No feature flag is needed. This is CLI/tooling behavior, not a live service rollout.
- Safe rollback is a git revert of the M18 code plus re-running the canonical proof loop to refresh
  passports/evidence under the reverted classifier.
- The compatibility-key drop behavior is the safety valve for mixed artifacts. If M18 keys are
  present but code is reverted, preserve-mode should drop incompatible stored reviews rather than
  projecting stale truth.

## Code Quality Review

The main quality risk is accidental duplication: one shared substrate on paper, but three subtly
different logic paths in code.

### Concrete code-quality rules

- Keep one semantic-review entrypoint in `spec-core/src/semantic_review.rs`.
- Add one explicit family descriptor layer, not three separate classifier modules that each
  duplicate routing and verdict projection.
- Reuse the normalized function representation. Do not parse authored YAML or raw Rust in a second
  disconnected way.
- Keep routing deterministic and explicit. No keyword scanning in `intent.why`.
- Keep `unsupported.function.v1` additive-only and non-demoting for all out-of-family functions.
- Update doc language only in:
  - `PLAN.md`
  - `README.md`
  - `AGENTS.md`
  - `examples/ecommerce/README.md`
- Add inline ASCII diagrams near:
  - family routing in `spec-core/src/semantic_review.rs`
  - keep/drop migration behavior in `spec-core/src/passport.rs`

### Well-designed patterns to preserve

- `project_semantic_review` already centralizes the dangerous keep/drop logic. M18 should extend
  that contract, not bypass it.
- The repo already treats `spec test` as the only proof-writing flow. Keep that line sharp.

### Anti-patterns to avoid

- growing an ever-longer exact-id support list
- family-specific persistence codepaths
- intent-string heuristics masquerading as support detection

## Test Review

100% new-path coverage is the goal. M18 only earns the word “generalization” if unseen examples are
the main proof, not a footnote.

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

─────────────────────────────────
COVERAGE TARGET: every new M18 path lands at ★★★
REQUIRED NEW EVIDENCE:
  Family A -> unseen aligned + drift + under_specified
  Family B -> unseen aligned + drift + under_specified
─────────────────────────────────
```

### Unseen-example bar

This is the heart of M18. The bar is:

- **Family A**
  - at least one unseen aligned example
  - at least one unseen drift example
  - at least one unseen under-specified example
- **Family B**
  - at least one unseen aligned example
  - at least one unseen drift example
  - at least one unseen under-specified example

Canonical seen examples (`apply_discount`, `apply_tax`, `calculate_total`) are still required, but
they are no longer sufficient evidence by themselves.

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
  - export/status project stored truth only
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
- Do **not** hide the whole milestone in test-only fake units. The canonical ecommerce story must
  still exercise both families.

### Regression rule

These regressions are mandatory:

- exact-id function reviews must drop on preserve-mode mismatch after M18 family routing lands
- unsupported near-miss functions must remain additive-only neutral
- Family B must fail on reversed dependency order
- Family A must fail when the admitted role and executable sign/clamp behavior disagree

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

- Family routing must stay local to one function body and one authored packet.
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
| family router | hidden exact-id branch still decides support | yes | yes | fake-green “generalization” | yes | **yes** |
| key migration | old exact-id review survives preserve-mode | yes | yes | stale semantic truth looks current | yes | **yes** |
| Family A classifier | wrong sign, missing clamp, or wrong rounding still aligns | yes | yes | pricing drift not caught | yes | **yes** |
| Family B classifier | wrapper skips or reorders deps and still aligns | yes | yes | orchestration drift not caught | yes | **yes** |
| fallback boundary | near-miss function demotes because router is too broad | yes | yes | unsupported code suddenly looks broken | yes | **yes** |
| docs | README/AGENTS still describe exact ids or generic function support | yes | n/a | maintainer overtrusts product claims | n/a | **yes** |

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

## Parallelization / Lanes

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| 1. Lock family routing + family keys | `spec-core/semantic_review`, `passport`, `export`, `commands` | - |
| 2. Implement Family A classifier + unseen fixtures | `semantic_review`, `spec-cli/tests`, ecommerce pricing examples | 1 |
| 3. Implement Family B classifier + unseen fixtures | `semantic_review`, `spec-cli/tests`, ecommerce pricing examples | 1 |
| 4. Add preserve/drop migration regressions | `passport`, `export`, `commands`, `spec-cli/tests` | 1 |
| 5. Refresh docs and workflow text | `PLAN.md`, `README.md`, `AGENTS.md`, `examples/ecommerce/README.md` | 1 |
| 6. Re-prove canonical truth loop and unseen packs | `spec-cli/tests`, evidence artifacts | 2, 3, 4 |

### Parallel lanes

- **Gate 0, sequential:** Step 1 must land first. Family routing and key vocabulary are the shared
  contract.
- **Lane A:** Step 2
  - Family A role routing, classifier, and unseen proof pack
- **Lane B:** Step 3
  - Family B pipeline routing, classifier, and unseen proof pack
- **Lane C:** Step 4
  - preserve/drop migration, unsupported neutrality, and status/export regressions
- **Lane D:** Step 5
  - docs and workflow honesty updates
- **Lane E:** Step 6
  - final canonical and unseen proof run after A + B + C merge

### Execution order

1. Lock Step 1.
2. Launch Lanes A, B, C, and D in parallel worktrees.
3. Merge A + B + C + D.
4. Run Lane E last for full trust-loop verification.

### Conflict flags

- `spec-core/src/semantic_review.rs` is the main conflict magnet. One owner should coordinate
  family routing and classifier composition.
- `spec-cli/tests/cli.rs` and `spec-cli/tests/m14_regressions.rs` are the second conflict magnets.
  Keep migration regressions and unseen-proof packs batched.
- Docs can run in parallel, but they must not merge before the family vocabulary is locked.

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

1. `pricing/apply_discount` and `pricing/apply_tax` are refreshed under Family A keys, not exact-id
   keys.
2. `pricing/calculate_total` either:
   - honestly fits `function.wrapper.pipeline.v1` and projects aligned / drift / under_specified,
     or
   - stays unsupported and forces the milestone red. No silent fallback.
3. Family A and Family B each pass the unseen-example bar:
   aligned + drift + under_specified on unseen examples.
4. Unsupported near-miss functions remain additive-only neutral and non-demoting.
5. `spec build`, `spec generate`, `spec status`, and `spec export` never mint replacement family
   truth.
6. README/AGENTS/examples docs describe bounded family support honestly.

**Green gate**

- If all six conditions hold, backend-readiness can reopen as the next planning question.

**Red gate**

- Stay on semantic-core correction if any of these are true:
  - unseen examples still require bespoke exact-id exceptions
  - Family B needs graph reasoning or branching semantics to make the canonical case green
  - router broadening causes unsupported near-miss functions to demote
  - docs cannot describe the supported surface in one crisp paragraph without caveats

## Completion Summary

| Item | Status |
|---|---|
| Scope challenge | written |
| What already exists | written |
| Architecture review | written |
| Code quality review | written |
| Test review | diagram + unseen-example bar + wedge loop written |
| Performance review | written |
| Failure modes | written |
| NOT in scope | written |
| Parallelization | written |
| Post-M18 gate | written |
| Current status | ready for implementation planning against M18 |

## Dream State Delta

- **Before M18**
  - semantic review for functions is still effectively a bounded exact-id story
  - `calculate_total` remains neutral, so the function support story does not yet travel into
    bounded composition
  - docs are forced to describe support as a narrow named set

- **After M18**
  - function semantic review is routed by explicit bounded families, not exact ids
  - arithmetic leaves and tiny pipeline wrappers can be evaluated on unseen examples
  - unsupported functions still stay honest and neutral
  - the product story becomes:
    “semantic review supports bounded function families with explicit fallback,” which is both more
    powerful and more honest than the old exact-id story

## M18 Review-Locked Decisions

- M18 is a generalization gate, not a “support one more pricing function” milestone.
- Family A is arithmetic leaf transforms. Family B is bounded two-step pipeline wrappers.
- `kind:function` support must route by family eligibility and compatibility keys, not exact ids.
- Exact-id M17 function keys are intentionally replaced and dropped on preserve mismatch.
- Unsupported but similar functions stay additive-only neutral unless the router first admits them
  into a supported family packet.
- `pricing/calculate_total` is allowed into M18 only if it fits the Family B contract honestly.
  It does not define the family by itself.
- Backend-readiness stays closed until M18 passes the unseen-example bar across both families.

## Decision Audit Trail (M18)

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | Scope | Replace exact-id function routing with family routing | mechanical | explicit over clever | Generalization must travel to unseen examples, not more ids | add more named supported functions |
| 2 | Scope | Keep exactly two admitted families in M18 | taste | boil the lake, not the ocean | Enough to prove travel, not enough to drift into generic semantics | one family only, or three families at once |
| 3 | Architecture | Use family-scoped compatibility keys | mechanical | DRY + reversibility | Keeps preserve/drop deterministic during migration | retain exact-id function keys forever |
| 4 | Family B | Admit only two-step pipeline wrappers | mechanical | engineered enough | Smallest composed-function contract that pressures the thesis | branching wrappers, arbitrary orchestration |
| 5 | Fallback | Neutral if not admitted, under_specified if admitted-but-ambiguous | mechanical | honesty over coverage theater | Prevents fake-green and fake-red alike | demote all similar functions |
| 6 | Proof bar | Require unseen aligned / drift / under_specified examples per family | mechanical | completeness | M18 is not credible without unseen travel | canonical examples only |
| 7 | Gate | Keep backend-readiness closed until both families pass | taste | focus as subtraction | Avoids calling the semantic core “done” too early | backend-readiness immediately after Family A |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | COMPLETE | M18 reframed as shared substrate + two-family proof; one-family generalization rejected |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | COMPLETE | Family contracts, migration keys, fallback boundary, unseen-example bar, test matrix, and rollout gate locked |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | No UI scope |

**UNRESOLVED:** 0

**VERDICT:** CEO + ENG WRITTEN — ready for implementation on `feat/m17`, with backend-readiness
explicitly blocked until M18 passes.
