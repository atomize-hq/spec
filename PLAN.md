<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m18-autoplan-restore-20260426-112223.md -->
# M19 - Semantic Review Falsification Pack

Status: **Implemented on `feat/m18` for PR #21** (April 26, 2026).

This is the current implementation contract for M19. M18 landed real substrate progress, but it
did not clear its own gate. M19 is therefore not "more M18 work" and not backend-readiness
reopening. It is the falsification pack that answers one question cleanly:

Can the current `kind:function` semantic-review substrate travel beyond canonical pricing examples
without preserving stale proof or false-greening nearby wrappers?

UI scope: **no**. This is a backend-only semantic-review milestone for freshness honesty,
Family A / Family B unseen-proof travel, Family B argument-flow validation, and an explicit freeze
of the current unsupported-surface contract.

## Source Inputs

- Current plan file:
  `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- Post-M18 design direction:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m18-design-20260426-095101.md`
- Eng-review test plan artifact:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m18-eng-review-test-plan-20260426-112223.md`
- Prior checkpoint grounding the family-shape decision:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/checkpoints/20260425-200501-m18-generalization-gate.md`

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

An AI-heavy Rust maintainer edits a function spec, its declared deps, or its executable Rust body,
runs the normal `spec` loop, and gets one honest answer:

- supported-family proof stays current only when the semantic inputs that produced it are still
  current;
- unseen Family A and Family B examples classify as `aligned`, `semantic_drift`, or
  `under_specified` for structural reasons, not canonical ids;
- Family B wrappers do not align unless the declared argument flow is actually preserved;
- unsupported near misses remain additive-only in `spec test` and neutral on official read-side
  surfaces.

The outcome is not "more semantic families." The outcome is a falsifiable answer to whether the
families already admitted by M18 are trustworthy enough to build on.

## Step 0: Scope Challenge

### Current system state

| Surface | Already proved | Still missing | M19 implication |
|---|---|---|---|
| `spec-core/src/semantic_review.rs` family routing | Function review is no longer exact-id only; Family A has alternate-id proof and Family B exists as a bounded wrapper family | Family B unseen travel is still canonical-name-adjacent and its flow proof is too weak | Reuse the router. Tighten only the admitted Family B claim. |
| `spec-core/src/passport.rs` freshness projection | Preserve vs refresh already follows the right top-level product rule | Function authored freshness still hashes only contract data for non-seam units, so semantic edits can survive as fake-current proof | Fix the digest and preserve-mode projection, not the whole passport model. |
| `spec-core/src/export.rs` / `spec-cli/src/commands.rs` read-side projection | Status and export already project stored proof instead of minting it | They can only be honest if freshness and supported-key compatibility are honest first | Reuse the projection path. Do not add a second read-side truth path. |
| `spec-cli/tests/m14_regressions.rs` | Canonical ecommerce wedges already prove aligned / drift / under-specified / unsupported behavior for the current anchors | There is no dedicated unseen M19 corpus and no argument-flow adversarial matrix | Extend the existing wedge harness instead of inventing a new end-to-end test framework. |
| Unsupported near-miss behavior | `spec test` can persist `unsupported.function.v1` while status/export drop it back to neutral | The product story is awkward and needs an explicit freeze | Freeze the contract and prove it. Do not redesign it here. |

### What already exists

| Sub-problem | Existing code surface | M19 reuse / correction |
|---|---|---|
| Supported-surface routing | `supported_surface_for_spec`, `supported_function_surface` in `spec-core/src/semantic_review.rs` | Reuse the family router. Do not bolt on a parallel M19 classifier path. |
| Supported function evaluation | `evaluate_supported_function_semantic_review` and family-specific body classifiers in `spec-core/src/semantic_review.rs` | Reuse Family A and Family B evaluation shape. Tighten Family B slot/flow semantics instead of broadening the family. |
| Preserve vs refresh truth loop | `project_semantic_review_with_context`, `project_passport_truth_with_context`, `write_passports` | Keep `spec test` as refresh and build/status/export as preserve. Fix the stale-proof bug inside that loop. |
| Read-side health demotion | `semantic_health_effect`, `apply_semantic_review_to_health`, `compute_health_status` | Reuse current precedence so stale base health still outranks semantic demotion. |
| Canonical wedge harness | `spec-cli/tests/m14_regressions.rs` canonical Family A / B tests | Reuse the exact harness and add unseen corpus + adversarial-flow cases beside it. |
| Command-matrix projection | `spec test`, `spec build`, `spec status`, `spec export` plus existing unsupported-near-miss coverage | Reuse the current commands. Add freshness and unsupported neutrality assertions, not new commands. |

### Minimum diff that still solves the problem

- Touch only the semantic-review, passport freshness, export/status projection, and existing CLI
  regression seams already under pressure.
- Add one dedicated unseen fixture pack under `spec-cli/tests/fixtures/m19/`.
- Keep the current unsupported-surface contract and document it honestly instead of redesigning it.
- Add no new semantic family, no new CLI command, no new artifact type, and no backend-readiness
  planning.

### Complexity check

- Expected primary module blast radius:
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/fixtures/m19/`
- This is a healthy lake. Same subsystem, no new services, no new schema family, no new infra.
- If implementation starts reaching for new semantic families, graph-wide coherence rules, or new
  command surfaces, stop. That is scope drift, not milestone necessity.

### TODOS cross-reference

- The existing `TODOS.md` entry about the Cargo-heavy CLI harness matters here, but it does not
  block M19. The right move is to keep most classifier coverage in `spec-core` unit tests and use
  CLI wedges only for read-side projection and end-to-end proof.
- No new TODO is required to ship this plan. Deferred work already has a home:
  unsupported-surface redesign, broader function understanding, and harness cleanup all stay out of
  M19 execution scope.

### Completeness check

- The complete version is: unseen corpus + freshness correction + Family B flow hardening +
  unsupported command matrix + canonical regression pass + honest docs paragraph.
- The shortcut is: more canonical-name tests and a small classifier patch. Reject that. It saves
  almost nothing and leaves the real trust question unanswered.

### Distribution check

- M19 introduces no new artifact type.
- Existing branch/test/release flow is enough.
- The deliverable is proof honesty in the existing CLI loop, not new packaging or CI machinery.

## Approved Scope

M19 includes exactly these capability corrections and proof obligations:

- unseen Family A corpus
- unseen Family B corpus
- function semantic freshness correction
- Family B argument-flow validation
- frozen unsupported-surface contract for this milestone

M19 may adjust implementation details only insofar as they are necessary to make those obligations
true and testable.

## Architecture Review

The repo already has the right top-level trust loop. The bug is inside the semantic inputs and the
Family B honest subset, not in the existence of refresh vs preserve itself.

### Ownership split

| Layer | Owns | Must not own |
|---|---|---|
| Family router | deciding whether a function belongs to Family A, Family B, or unsupported | inventing proof freshness |
| Supported-family classifier | aligned / drift / under-specified inside one admitted family | product-surface projection rules |
| Freshness snapshot | "did the semantic inputs that produced this proof change?" | family eligibility decisions |
| Preserve / refresh projection | whether stored semantic truth is kept, refreshed, or dropped on each command path | classifier semantics |
| Status / export surfaces | surfacing current truth honestly to users and downstream tooling | minting new proof |

### Trust-loop diagram

```text
authored function spec
  │
  ├── semantic_review.rs
  │     ├── supported_function_surface()
  │     └── evaluate_supported_function_semantic_review()
  │
  ├── passport.rs
  │     ├── compute_authored_truth_digest()
  │     ├── resolve_passport_freshness()
  │     └── project_passport_truth_with_context()
  │
  └── commands / export
        ├── spec test   -> refresh supported semantic truth
        ├── spec build  -> preserve only
        ├── spec status -> preserve only
        └── spec export -> preserve only
```

### Code seams under pressure

| Seam | Current behavior | M19 change |
|---|---|---|
| `compute_authored_truth_digest()` in `spec-core/src/passport.rs` | For non-seam units it serializes only `contract`, which means function `intent`, `deps`, and `body.rust` edits do not affect authored freshness | Expand the function authored-truth surface so preserve-mode can mark semantic proof stale honestly |
| `classify_family_b_nested_call()` / `classify_family_b_let_then_return()` in `spec-core/src/semantic_review.rs` | They prove shape and alias threading, but not full parameter-slot correctness or uniqueness | Require exact dep order, slot-0 threading, and expected top-level parameter mapping |
| `project_semantic_review_with_context()` | Preserve-mode keeps only matching supported keys and drops unsupported surfaces | Keep this rule. Add regression proof that freshness/input changes prevent fake-current supported proof |
| `write_passports()` in `spec-cli/src/commands.rs` | Non-test flows preserve stored proof and project live freshness | Keep this contract. The fix is in the freshness snapshot and projection inputs, not a new write mode |
| `enrich_passports_for_export()` and status JSON projection | Export/status reproject preserve-mode truth for read-side consumers | Add regressions that stale supported proof drops or demotes correctly and unsupported near misses stay neutral |

### Frozen Unsupported-Surface Contract

M19 freezes the current unsupported contract:

- `spec test` may record unsupported semantic-review metadata as additive proof detail.
- Official read-side surfaces remain neutral for unsupported cases.
- `spec status --format json` and `spec export` must not demote unsupported near misses because
  they look similar to a supported family.
- M19 does not decide whether unsupported metadata should become first-class everywhere or disappear
  entirely. That is explicitly deferred.

This is awkward, but it is stable enough for this milestone. Redesigning it now would hide the
actual trust-risk behind surface cleanup.

### Function Freshness Contract

Supported function semantic truth is current only if the semantic inputs that produced it are still
current. For M19 that means the freshness anchor for function units must account for:

- `intent`
- declared `deps`
- executable `body.rust`
- the authored contract and invariant fields used by family routing

Concrete reason this is in scope now: today `compute_authored_truth_digest()` for non-seam units
serializes only the function contract. That means an `intent` rewrite, dep swap, or body rewrite
can survive preserve-mode as if the semantic proof were still current. Wild. M19 is not green if
that remains possible on status or export.

### Family B Argument-Flow Contract

Family B stays the bounded two-step wrapper family. M19 tightens the claim from "the wrapper has
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

### Appendix: Family B Adversarial Verdict Map

This appendix removes the last meaningful implementation ambiguity in the Family B falsification
pack. Every case below must fail `aligned`, and the expected non-`aligned` verdict is part of the
contract.

| Adversarial case | Expected verdict | Why |
|---|---|---|
| inner-call args swapped | `semantic_drift` | The wrapper still claims a supported pipeline, but the executable dep flow contradicts that claim. |
| outer-call rate arg swapped | `semantic_drift` | The second dep receives the wrong top-level parameter in a required slot, so the executable meaning contradicts the authored wrapper intent. |
| wrong threaded alias returned | `semantic_drift` | The wrapper shape is still a two-step pipeline, but it threads the wrong intermediate value. |
| duplicated param passed where distinct params are required | `semantic_drift` | The wrapper claims distinct semantic roles but collapses them into one executable path. |
| dropped parameter or unused parameter path | `under_specified` | The declared wrapper contract is broader than the executable proof surface, so the honest result is that the authored claim is too weak or too loose. |
| literal substituted for a required arg | unsupported | This exits the admitted honest subset rather than proving a contradictory supported-family implementation. |
| arithmetic expression substituted for a required arg | unsupported | Derived computation in a required arg slot is outside the bounded wrapper family M19 admits. |
| method chain substituted for a required arg | unsupported | Non-trivial expression shaping in a required arg slot leaves the approved Family B subset. |

Rule of thumb:

- Wrong but still clearly attempting the admitted wrapper semantics => `semantic_drift`
- Claim too vague relative to the executable path => `under_specified`
- No longer inside the admitted wrapper subset at all => unsupported

### Error & Rescue Registry

| Step | Failure | Detection | Rescue |
|---|---|---|---|
| Semantic edit after passing proof | status/export still show a current supported review | freshness regression tests on preserve-mode status/export | expand function freshness digest and reproject truth through the existing preserve path |
| Family B classifier hardening | wrapper with swapped or duplicated args still returns `aligned` | unit tests in `semantic_review.rs` plus unseen CLI wedge | tighten slot/alias mapping rules, do not widen the family |
| Unsupported near-miss freeze | status/export begin surfacing `unsupported.function.v1` or demoting health | command-matrix CLI regressions | keep unsupported refresh metadata additive-only and read-side neutral |
| Unseen travel proof | alternate-id cases only pass when using canonical `pricing/*` deps | unseen fixture corpus with alternate ids and alternate-id leaf deps | keep family recognition structural, not name-based |
| Test harness cost | every new classifier case becomes a full Cargo-backed CLI test | test review split between unit and CLI layers | keep classifier matrix local to `spec-core`, reserve CLI for trust surfaces |

## Code Quality Review

The main quality risk is duplicated truth logic drifting apart. M19 should stay boring here.

- One freshness rule. `passport`, `status`, and `export` should all depend on the same
  `project_passport_truth_with_context()` path.
- One Family B honest subset. Do not create "CLI-only" or "fixture-only" semantics that differ
  from `semantic_review.rs`.
- One unsupported contract. `spec test` additive, read-side neutral. Say it once and prove it.
- Keep the diff explicit. A 20-line obvious slot-mapping check beats a cute abstraction that hides
  why a wrapper is unsupported.

## Test Review

100% relevant coverage is the goal here. This milestone is mostly proof and trust-surface work, so
the test plan has to map codepaths to the exact surfaces users rely on.

### Code path coverage to add

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/passport.rs
    │
    ├── compute_authored_truth_digest()
    │   ├── [GAP] function digest changes on intent-only edit
    │   ├── [GAP] function digest changes on dep-only edit
    │   ├── [GAP] function digest changes on body-only edit
    │   └── [GAP] routing-relevant authored cues still affect freshness
    │
    └── resolve_passport_freshness() / preserve projection
        ├── [GAP] preserved supported review becomes stale after semantic edit
        └── [GAP] stale base health still outranks semantic demotion

[+] spec-core/src/semantic_review.rs
    │
    ├── supported_function_surface()
    │   ├── [★★★ TESTED] Family A alternate-id aligned
    │   └── [GAP] Family B alternate-id aligned without canonical dep ids
    │
    ├── classify_family_b_nested_call()
    │   ├── [GAP] inner-call args swapped
    │   ├── [GAP] outer-call arg swapped
    │   ├── [GAP] duplicated or dropped param
    │   └── [GAP] literal / expression substituted for required param
    │
    └── classify_family_b_let_then_return()
        ├── [GAP] wrong alias threaded to dep_b slot 0
        └── [GAP] dep order reversed but still shape-compatible

[+] spec-cli read-side surfaces
    │
    ├── spec test
    │   └── [★★ TESTED] unsupported near miss may persist additive unsupported metadata
    │
    ├── spec build / spec status / spec export
    │   ├── [GAP] unsupported near miss stays neutral after refresh
    │   ├── [GAP] stale supported proof does not remain current after semantic edit
    │   └── [GAP] canonical Family A / B keys still project after fresh proof
```

### Operator-flow coverage

```text
OPERATOR FLOW COVERAGE
===========================
[+] Maintainer edits function intent after passing proof
    ├── [GAP] run build without test -> passport/status/export must no longer look current
    └── [GAP] run status --format json -> stale reason must surface honestly

[+] Maintainer edits Family B wrapper body incorrectly
    ├── [GAP] spec test -> semantic_drift or unsupported, never aligned
    └── [GAP] subsequent status/export -> same trust story projected read-side

[+] Maintainer authors unseen alternate-id function
    ├── [GAP] Family A aligned / drift / under-specified proof pack
    └── [GAP] Family B aligned / drift / under-specified proof pack

[+] Maintainer authors unsupported near miss
    ├── [GAP] spec test writes additive unsupported metadata
    └── [GAP] status/export remain neutral
```

### Required test split

| Layer | What belongs there | Why |
|---|---|---|
| `spec-core/src/semantic_review.rs` unit tests | Family B slot/flow adversarial matrix, Family A/B alternate-id evaluator cases | Fastest place to prove classifier truth without spawning Cargo for every case |
| `spec-core/src/passport.rs` and `spec-core/src/export.rs` unit tests | freshness digest regressions and preserve-mode projection behavior | These are the real stale-proof bug surfaces |
| `spec-cli/tests/m14_regressions.rs` | end-to-end canonical + unseen wedges, unsupported command matrix, status/export truth projection | Proves the user-visible loop |
| `spec-cli/tests/cli.rs` | command-surface regressions that do not fit the wedge helpers cleanly | Keeps CLI contract honest without making every proof case a CLI test |

### Test plan artifact

The QA-facing artifact for this review lives at:

`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m18-eng-review-test-plan-20260426-112223.md`

### Verification loop

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

The dedicated M19 fixture root must also pass the exact fixture gate listed in the Green Gate
section. M19 is not green unless both the fixture gate and the full final gate pass.

## Performance Review

There is no database or network performance story here. The real performance risk is test harness
cost and duplicated projection work.

| Concern | Risk | M19 guardrail |
|---|---|---|
| Cargo-heavy CLI regression expansion | Full end-to-end cases get slow and noisy fast | Keep classifier matrix in `spec-core` unit tests; reserve CLI for trust-surface projection and wedge coverage |
| Duplicate freshness / projection logic | One surface gets fixed and another stays fake-green | Route all read-side checks through the existing passport projection path |
| Fixture-pack bloat | Unseen corpus becomes hard to reason about and expensive to maintain | Keep one small M19 fixture root with explicit aligned / drift / under-specified / unsupported variants per family |

## NOT in Scope

| Deferred item | Why it is deferred |
|---|---|
| Unsupported-surface redesign | Real issue, wrong milestone. M19 needs a frozen rule, not a new model. |
| New semantic families | M19 must answer whether the current families are trustworthy before widening surface area. |
| Backend-readiness reopening | False confidence multiplier if freshness and Family B proof are still weak. |
| Branching or looping wrapper semantics | That is a broader Family B claim than M18/M19 ever approved. |
| Graph-wide semantic coherence | Different problem, different blast radius. |
| New CLI commands or artifact types | Existing loop is enough; honesty is the bottleneck, not feature count. |
| Large docs sweep | Only the contract paragraph and milestone truth need updates now. |

## Implementation Order

1. Fix function authored-truth freshness so `intent`, `deps`, `body.rust`, and routing-relevant
   authored cues invalidate supported proof in preserve mode.
2. Tighten Family B argument-flow validation and land the adversarial-flow unit tests.
3. Add the dedicated M19 unseen fixture corpus for both families.
4. Run combined `spec-core` tests for the freshness and Family B changes before adding CLI
   truth-surface assertions.
5. Add CLI command-matrix regressions for unsupported neutrality and stale supported-proof
   projection.
6. Re-run canonical ecommerce proofs to ensure M19 does not regress the M18 wins.
7. Update the one-paragraph product contract for supported vs unsupported function review.
8. Re-run the fixture gate and full final gate before any backend-readiness discussion reopens.

Freshness correction should land before the new unseen corpus is treated as evidence. Otherwise the
new fixture pack can accidentally bless preserved stale truth.

## Failure Modes Registry

| Codepath | Realistic production failure | Test cover required | Error handling exists? | User-visible outcome if missed |
|---|---|---|---|---|
| Function freshness snapshot | Maintainer edits `intent` and status still reports supported proof as current | yes, unit + CLI preserve-mode regression | partially, but wrong today for function semantic edits | silent false-green |
| Family B nested call classifier | Swapped or duplicated params still classify as `aligned` | yes, unit adversarial matrix | no, classifier truth is the defense | silent false-green |
| Family B alternate-id travel | Only canonical dep ids pass, so "generalization" is just a whitelist with extra steps | yes, unseen corpus | no | misleading product claim |
| Unsupported near miss | status/export start surfacing unsupported review or demoting health | yes, CLI command-matrix regression | yes, preserve path currently drops unsupported surfaces | read-side contract drift |
| Canonical Family A/B recheck | M19 fix breaks existing canonical keys after fresh `spec test` | yes, canonical regression commands | yes, tests catch it | visible regression on examples |
| Test harness layering | every new proof case uses Cargo-backed CLI flow and becomes expensive to maintain | yes, unit-vs-CLI split enforced in test plan | process guidance only | slow noisy review loop |

Any row that would otherwise be silent false-green is a critical gap. That is the whole game.

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Worker A. Freshness contract correction | `spec-core/src/passport.rs`, `spec-core/src/export.rs` | — |
| Worker B. Family B verdict split and flow hardening | `spec-core/src/semantic_review.rs` | — |
| Worker C. M19 unseen fixture corpus | `spec-cli/tests/fixtures/m19/semantic_falsification_pack/` | — |
| Worker D. CLI regression harness, command matrix, docs | `spec-cli/tests/m14_regressions.rs`, `spec-cli/tests/cli.rs`, minimal docs | A, B, C |

### Parallel lanes

- `Worker A`: owns `spec-core/src/passport.rs`, any needed `spec-core/src/export.rs` code changes
  and tests, and `spec-core`-level freshness coverage only. `compute_contract_hash()` must remain
  legacy contract-only. `compute_authored_truth_digest()` broadens function freshness for `intent`,
  `deps`, `body.rust`, and routing-relevant authored cues. Worker A proves stale freshness
  primarily in `spec-core` tests and does not touch CLI harness files.
- `Worker B`: owns `spec-core/src/semantic_review.rs` and Family B unit tests. Preserve the current
  syntax envelope: direct nested call or `let` + return only. Implement the Family B verdict split
  so admitted wrappers can become `semantic_drift`, `under_specified`, or unsupported. Hard-code
  the adversarial verdict map in tests. Cover non-stacking wrappers only in `spec-core` unit tests;
  the M19 fixture root is not expected to prove non-stacking.
- `Worker C`: owns only `spec-cli/tests/fixtures/m19/semantic_falsification_pack/`. Track source
  fixture files only: no generated Rust, passports, or evidence artifacts. Include Family A down
  variants `billing/apply_membership_discount`, `_drift`, `_under_specified`, and
  `_unsupported_near_miss`; Family A up variants `billing/apply_regional_fee`, `_drift`,
  `_under_specified`, and `_unsupported_near_miss`; and Family B variants
  `billing/checkout_net_total`, `_drift`, `_under_specified`, and `_unsupported_near_miss`. The
  aligned Family B wrapper must depend on alternate-id Family A leaves, not canonical `pricing/*`.
- `Worker D`: owns all `spec-cli/tests/cli.rs`, all `spec-cli/tests/m14_regressions.rs`, and the
  minimal docs refresh. It must prove stale supported function proof after semantic edits surfaces
  correctly on read-side commands; unsupported near misses may persist additive metadata after
  `spec test`; unsupported near misses remain neutral on `spec build`, `spec generate`,
  `spec status`, and `spec export`; unsupported-near-miss neutrality holds for one Family A down
  case, one Family A up case, and one Family B case; and drift / under-specified M19 fixture units
  are exercised in `spec-cli/tests/m14_regressions.rs`. `spec generate` coverage is required in
  Worker D's CLI tests, not necessarily in the top-level shell gate.

Family B adversarial verdict map:

| Adversarial case | Required verdict |
|---|---|
| inner-call args swapped | `semantic_drift` |
| outer-call rate arg swapped | `semantic_drift` |
| wrong threaded alias returned | `semantic_drift` |
| duplicated param passed where distinct params are required | `semantic_drift` |
| dropped parameter or unused parameter path | `under_specified` |
| literal substituted for a required arg | unsupported |
| arithmetic expression substituted for a required arg | unsupported |
| method chain substituted for a required arg | unsupported |

### Execution order

Launch Workers A, B, and C in parallel worktrees. Stabilize A. Stabilize B. Review C fixture root.
Run combined `spec-core` tests for A + B. Only then run Worker D, because the CLI assertions and
contract wording need the final freshness behavior, Family B verdict behavior, and fixture corpus.

### Conflict flags

- Worker A and Worker B are cleanly separable by module.
- Worker C is independent until Worker D starts consuming the fixtures.
- Worker D owns every CLI harness change. Do not let Worker A or Worker C edit
  `spec-cli/tests/cli.rs` or `spec-cli/tests/m14_regressions.rs`.

## Green Gate

M19 is green only if all of these are true:

1. Family A unseen corpus proves `aligned`, `semantic_drift`, `under_specified`, and
   unsupported-near-miss neutrality with non-canonical unit ids for both monotone-down and
   monotone-up role sets.
2. Family B unseen corpus proves `aligned`, `semantic_drift`, `under_specified`, and
   unsupported-near-miss neutrality with non-canonical unit ids.
3. At least one aligned Family B case uses alternate-id Family A leaf deps, not only canonical
   `pricing/apply_discount` and `pricing/apply_tax`.
4. Every Family B adversarial-flow case resolves to the exact non-`aligned` verdict in the verdict
   map above.
5. Supported function semantic truth becomes stale or drops after `intent`, `deps`, `body.rust`, or
   routing-relevant contract changes.
6. Unsupported near misses stay neutral in `spec build`, `spec generate`,
   `spec status --format json`, and `spec export` after a refresh writes additive unsupported
   metadata.
7. Canonical ecommerce examples still project the intended Family A and Family B keys after fresh
   `spec test` proof.
8. The supported vs unsupported story can be stated in one crisp product paragraph without claiming
   generic function support or promising unsupported-surface redesign.

### Fixture Gate

These exact fixture commands must exist and pass with their expected outcomes. Because the fixture
root intentionally includes drift and under-specified units, `spec status --format json` is expected
to exit non-zero; passing means the JSON reports the expected failing/incomplete units while
unsupported near misses stay read-side neutral.

```text
cargo run -p spec-cli -- test spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/apply_membership_discount.unit.spec --crate-root spec-cli/tests/fixtures/m19/semantic_falsification_pack
cargo run -p spec-cli -- test spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/apply_regional_fee.unit.spec --crate-root spec-cli/tests/fixtures/m19/semantic_falsification_pack
cargo run -p spec-cli -- test spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/checkout_net_total.unit.spec --crate-root spec-cli/tests/fixtures/m19/semantic_falsification_pack
cargo run -p spec-cli -- test spec-cli/tests/fixtures/m19/semantic_falsification_pack/units/billing/checkout_net_total_unsupported_near_miss.unit.spec --crate-root spec-cli/tests/fixtures/m19/semantic_falsification_pack
cargo run -p spec-cli -- build spec-cli/tests/fixtures/m19/semantic_falsification_pack/units --crate-root spec-cli/tests/fixtures/m19/semantic_falsification_pack
cargo run -p spec-cli -- status spec-cli/tests/fixtures/m19/semantic_falsification_pack --format json
cargo run -p spec-cli -- export spec-cli/tests/fixtures/m19/semantic_falsification_pack
```

The unsupported command matrix is an ordered sequence against the same fixture root: run
`spec test` on the unsupported-near-miss fixture unit, then `spec build` on that fixture root, then
`spec status --format json` on that fixture root, then `spec export` on that fixture root, and
verify read-side neutrality after the sequence.

The fixture files alone are not proof. Worker D must either execute or assert every required unseen
drift, under-specified, and unsupported-near-miss behavior in `spec-cli/tests/m14_regressions.rs`.

### Full Final Gate

M19 is not green unless both the fixture gate and this full final gate pass:

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
| 3 | Correct function freshness before trusting preserve-mode proof | mechanical | Current supported reviews can look current after semantic edits because function authored freshness is too narrow. | rely on existing passport preservation |
| 4 | Tighten Family B argument flow instead of widening the family | mechanical | Nesting shape alone can miss wrong parameter flow. | call wrapper shape sufficient |
| 5 | Freeze unsupported behavior for M19 | taste | The contract is awkward but redesign is out of scope. | broaden unsupported-surface cleanup |
| 6 | Keep most classifier coverage in `spec-core` unit tests | mechanical | The CLI harness is the wrong place to spend every test dollar. | make every proof case a Cargo-backed wedge |
| 7 | Keep backend-readiness closed | taste | Backend work would multiply false confidence if semantic proof is stale or weak. | reopen backend-readiness after M18 |

## Completion Summary

| Item | Status |
|---|---|
| Step 0 scope challenge | written |
| Architecture review | written |
| Code quality review | written |
| Test review | written, with code-path and operator-flow diagrams |
| Performance review | written |
| Approved scope and NOT in scope | written |
| What already exists | written |
| Failure modes | written |
| Worktree parallelization | written, 4 lanes with 3 launchable in parallel |
| Test plan artifact | written |
| Lake score | 7/7 major recommendations chose the complete option |
| Current status | implemented on `feat/m18` for PR #21; backend-readiness remains closed |
