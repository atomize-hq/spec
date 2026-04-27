<!-- M20 solidify restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/main-m20-plan-solidify-restore-20260426-231434.md -->
# M20 - Unsupported Function Truth Surface

Status: **approved for implementation on `main` after M19 landed**.

UI scope: **no**. This is a backend semantic-review milestone for passports, status JSON, export
JSON, docs, and regression fixtures.

M19 proved the positive path: bounded function families can be recognized honestly when freshness,
argument flow, and unsupported neutrality are enforced. M20 closes the other half of the contract.
Unsupported `kind:function` shapes should no longer look like vague under-specified supported
families. They should be explicit, reasoned, actionable, machine-readable, freshness-aware, and
health-neutral.

M20 does **not** add a new supported function family.

## Source Inputs

- Current plan file: `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- Prior M20 autoplan restore: `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/main-autoplan-restore-20260426-220932.md`
- Current M20 solidify restore: `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/main-m20-plan-solidify-restore-20260426-231434.md`
- Shipped M19 commit on `main`: `0b4fb20 feat: add M19 semantic review falsification pack (#21)`
- Relevant current code:
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - `spec-cli/tests/fixtures/m19/semantic_falsification_pack/`

## Review Mode

- `/autoplan` mode: `SELECTIVE_EXPANSION`
- `/plan-eng-review` structure: applied
- Design phase: skipped, no UI scope
- Outside voices from the first M20 pass: Codex only, incorporated below
- Primary correction from this pass: unify the plan around `support_status`, not a breaking
  `SemanticVerdict::Unsupported`

## Milestone Summary

```text
M20a  Add a first-class function semantic support status                      required
M20b  Add a function-only unsupported diagnostic carrier                      required
M20c  Preserve fresh unsupported function proof on read-side surfaces         required
M20d  Drop stale unsupported function proof under preserve-mode projection    required
M20e  Keep unsupported health-neutral and preserve M19 supported families     required
M20f  Refresh fixture, docs, status, export, and consumer contract coverage   required
```

## User Outcome

An AI-heavy Rust maintainer authors a function outside the admitted Family A / Family B subset and
gets a useful "no":

- `spec test` refreshes passport semantic truth and records that the function is unsupported;
- the review says `support_status: "unsupported"` instead of requiring consumers to infer
  unsupported from `verdict == "under_specified"` plus `evaluator_scope == "unsupported_surface"`;
- reason codes explain why the function missed the admitted subset;
- rewrite hints tell the author what shape to try next;
- `spec build`, `spec generate`, `spec status`, and `spec export` preserve fresh unsupported proof
  without minting new proof;
- stale unsupported proof disappears from projected semantic review instead of pretending to be
  current;
- status health remains `valid` unless a separate validation, build, test, freshness, or gate issue
  says otherwise.

This gives maintainers and agents an honest negative path before the project widens the positive
path with more supported families.

## Step 0: Scope Challenge

### Current System State

| Surface | Current behavior after M19 | M20 problem | M20 decision |
|---|---|---|---|
| Supported function families | Family A down, Family A up, and Family B wrapper/pipeline are bounded and tested | Vocabulary is intentionally tiny | Keep it frozen |
| Unsupported function refresh | `spec test` can write `unsupported.function.v1` with `verdict: under_specified` and `evaluator_scope: unsupported_surface` | Unsupported is still overloaded onto the supported-family verdict model | Add `support_status: unsupported` and function-only diagnostics |
| Preserve-mode projection | Unsupported reviews are dropped in preserve-mode today | Read-side commands lose useful current unsupported explanations | Preserve fresh function-unsupported reviews |
| Freshness | M19 tightened semantic freshness for supported function proof | Unsupported proof needs the same stale/drop discipline | Drop stale unsupported review when authored truth changes |
| CLI health | `semantic_health_effect()` already keeps unsupported surfaces neutral because `unsupported_surface` is not a supported evaluator scope | New fields must not accidentally demote health | Keep unsupported mapped to `KeepBase` |
| Consumer contract | Docs still teach unsupported as "not evaluated" or implicit | Agents need a stable branch key | Document `support_status` as the branch key |

### What Already Exists

| Sub-problem | Existing code surface | M20 reuse |
|---|---|---|
| Semantic review model | `SemanticVerdict`, `SemanticReasonCode`, `EvaluatorScope`, `SemanticReview` in `spec-core/src/semantic_review.rs` | Add compatible fields to `SemanticReview`; do not replace the whole model |
| Unsupported review builder | `unsupported_surface_review(unit_kind)` | Split function unsupported into a richer function-only builder; keep data/sum generic |
| Family router | `supported_surface_for_spec()` and `supported_function_surface()` | Reuse the "unsupported" branch as the entrypoint for diagnostics |
| Preserve/refresh loop | `project_semantic_review_with_context()`, `project_passport_truth_with_context()`, `write_passports()` | Keep one projection loop; make it freshness-aware for unsupported functions |
| Health mapping | `semantic_health_effect()` and `apply_semantic_review_to_health()` | Preserve `KeepBase` for unsupported reviews |
| Export/status projection | `build_export_bundle()`, `enrich_passports_for_export()`, status rows in `spec-cli/src/commands.rs` | Surface the same semantic review shape everywhere |
| Regression harness | `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs`, M19 fixture pack | Reuse current CLI fixtures and add M20 unsupported assertions |

### Minimum Diff

- Add a small semantic support-status enum and function-only unsupported diagnostics.
- Preserve the existing `SemanticVerdict` enum in M20.
- Keep `unsupported.function.v1` as the compatibility key unless implementation proves a version
  bump is unavoidable.
- Rework preserve-mode projection in the existing passport/export/status path.
- Add targeted fixture and golden JSON coverage.
- Do not add a new command, service, artifact type, schema family, LLM eval, or supported function
  family.

### Complexity Check

Expected module blast radius:

```text
spec-core/src/semantic_review.rs      model + unsupported diagnostic builder
spec-core/src/passport.rs             freshness-aware semantic projection
spec-core/src/export.rs               exported passport projection tests
spec-cli/src/commands.rs              status health + JSON/text visibility tests
spec-cli/tests/cli.rs                 command matrix + golden fixture updates
spec-cli/tests/m14_regressions.rs     M19 supported-family non-regression
spec-cli/tests/fixtures/m20/          unsupported truth pack
README.md / AGENTS.md / CLAUDE.md     consumer contract wording
```

This is more than eight files, but it is one subsystem. That is acceptable because the milestone is
a contract change across all read surfaces. Scope drift starts when implementation touches new
semantic families, graph architecture, new CLI commands, or cross-kind unsupported ontology.

### TODOS Cross-Reference

- The open CLI-harness TODO about Cargo-heavy `spec test` coverage matters here. Keep most M20
  classifier proof in `spec-core` unit tests. Use CLI tests only for read-side projection and
  command-matrix proof.
- The M19 unused-variable fixture TODO is fixture hygiene, not an M20 blocker.
- No new TODO is required before implementation. Deferred items are listed in `## Not In Scope`.

### Completeness Check

Complete version:

- explicit branch key for unsupported function review;
- stable unsupported reason taxonomy;
- deterministic rewrite hints;
- fresh preserve-mode visibility;
- stale preserve-mode dropping;
- passport, status, export, docs, fixtures, and non-regression tests all updated together.

Shortcut to reject:

- better summary text only;
- more `UnsupportedSurface` assertions only;
- a breaking `SemanticVerdict::Unsupported` enum change without a migration path.

### Distribution Check

M20 introduces no binary, package, container, hosted service, or release channel. The deliverable is
the existing CLI's machine-readable contract. Existing Cargo and GitHub release machinery remains
enough.

## Approved Scope

M20 includes:

1. Function-only `support_status` in semantic review output.
2. Function-only unsupported diagnostic carrier with stable reason taxonomy and rewrite hints.
3. Preserve-mode projection that keeps fresh unsupported function review.
4. Preserve-mode projection that drops stale unsupported function review.
5. Health neutrality for unsupported function review.
6. M19 supported-family non-regression.
7. Fixture, golden JSON, README, AGENTS, and CLAUDE contract updates.

## Not In Scope

- New supported function family, including "Family C".
- Branching or looping wrappers becoming supported.
- LLM-based semantic review.
- Cross-kind unsupported redesign for `kind:data` or `kind:sum`.
- New CLI commands.
- New artifact family.
- Broad status/export UX polish outside this unsupported function truth contract.
- Removal of old compatibility keys in M20.

## Architecture Review

M20 changes the semantic contract, not the product architecture. The implementation stays inside the
existing refresh/preserve truth loop.

### Dependency Graph

```text
authored .unit.spec
  │
  ├── spec-core/src/semantic_review.rs
  │     ├── supported_function_surface()
  │     ├── evaluate_supported_function_semantic_review()
  │     ├── UnsupportedFunctionDiagnostic              [new internal carrier]
  │     ├── unsupported_function_review()              [new function-only builder]
  │     └── unsupported_surface_review()               [keep generic data/sum path]
  │
  ├── spec-core/src/passport.rs
  │     ├── resolve_passport_freshness()
  │     ├── project_passport_truth_with_context()
  │     └── project semantic review with freshness     [changed]
  │
  ├── read-side surfaces
  │     ├── spec-cli/src/commands.rs                   [status JSON/text + health]
  │     └── spec-core/src/export.rs                    [export bundle]
  │
  └── tests / docs
        ├── spec-core unit tests
        ├── spec-cli command matrix
        ├── m14/M19 non-regression
        └── README / AGENTS / CLAUDE contract text
```

### Data Model Contract

Add a compatibility-preserving support-status field:

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SemanticSupportStatus {
    Supported,
    Unsupported,
}

pub struct SemanticReview {
    pub verdict: SemanticVerdict,
    pub compatibility_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_status: Option<SemanticSupportStatus>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unsupported_reason_codes: Vec<UnsupportedFunctionReasonCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rewrite_hints: Vec<String>,
    ...
}
```

Contract rules:

- new supported Family A / Family B reviews emit `support_status: supported`;
- new unsupported function reviews emit `support_status: unsupported`;
- `support_status` is the public serialized field name. Do not rename it during implementation.
- behavior branches must call `SemanticReview::effective_support_status()`;
- explicit `support_status` wins when present;
- legacy reviews without `support_status` are interpreted by
  `SemanticReview::effective_support_status()`: old `evaluator_scope: unsupported_surface` or
  `unsupported.*.v1` keys are unsupported, old supported evaluator scopes are supported;
- generic unsupported data/sum reviews do not receive the richer function-only diagnostics in M20;
- existing `verdict` values remain unchanged in M20, so unsupported functions can keep
  `verdict: under_specified` while consumers branch on `support_status`;
- `unsupported.function.v1` remains the compatibility key unless the implementation uncovers a
  concrete semantic break that requires `v2`.

The plan intentionally chooses `support_status` over `SemanticVerdict::Unsupported`. That avoids an
immediate strict-deserializer break while still making unsupported first-class. The compatibility
helper is a method, not a loose call-site convention, so status/export/passport code cannot drift.

### Unsupported Reason Taxonomy

M20 should keep this small. More buckets are not more truth.

| Reason | Meaning | Rewrite hint |
|---|---|---|
| `unsupported_control_flow` | Function body uses branching, looping, match, early return, or other control flow outside the admitted subset | Rewrite as a straight-line arithmetic leaf or wrapper pipeline, or wait for a future family |
| `unsupported_required_argument_expression` | Wrapper required dep argument is computed with a literal, method call, nested expression, or non-admitted expression | Thread required arguments directly through the wrapper in declared order |
| `unsupported_dep_topology` | Function deps do not match one of the admitted leaf/wrapper topologies | Declare the exact leaf deps or split the behavior into admitted units |
| `unsupported_wrapper_body_shape` | Wrapper body is not a let-then-return or direct pipeline shape admitted by Family B | Use explicit `let` bindings followed by the final dep call |
| `unsupported_arithmetic_shape` | Leaf arithmetic is outside monotone up/down nonnegative forms | Rewrite into the current arithmetic family or defer until a future family exists |
| `unsupported_function_surface` | Fallback when no more specific function reason can be determined | Treat as unsupported by this engine version and inspect body/deps manually |

Implementation may merge a reason only if two buckets cannot be distinguished deterministically from
the AST without guesswork. It must not add open-ended prose-only reasons.

Classifier priority is fixed for deterministic output:

1. `unsupported_control_flow`
2. `unsupported_dep_topology`
3. `unsupported_required_argument_expression`
4. `unsupported_wrapper_body_shape`
5. `unsupported_arithmetic_shape`
6. `unsupported_function_surface`

When multiple buckets apply, emit the first matching primary reason as the lead reason and include
only additional reasons that are deterministic from the same AST walk. Tests should assert the lead
reason order so future classifier edits do not reshuffle public output accidentally.

### Projection Rules

```text
spec test
  │
  ├── supported function surface
  │     └── refresh supported review as M19 does today
  │
  └── unsupported function surface
        └── refresh unsupported function review
              support_status: unsupported
              verdict: under_specified
              evaluator_scope: unsupported_surface
              compatibility_key: unsupported.function.v1
              unsupported_reason_codes: [...]
              rewrite_hints: [...]

spec build / generate / status / export
  │
  ├── authored truth fresh
  │     └── preserve stored unsupported function review
  │
  └── authored truth stale / unknown
        └── drop projected semantic_review for unsupported function proof
            base health still carries stale/unknown reason independently
```

Projection must be freshness-aware at the passport/export/status layer. Do not hide freshness logic
inside the family classifier.

### Code Seams Under Pressure

| Seam | Current behavior | M20 change |
|---|---|---|
| `unsupported_surface_review(unit_kind)` | Generic unsupported review for any unit kind | Keep generic path, add `unsupported_function_review(diagnostic)` for functions |
| `project_semantic_review_with_context()` | Preserve drops unsupported review | Preserve fresh unsupported function review; continue dropping unsupported non-function review |
| `project_passport_truth_with_context()` | Computes freshness and semantic projection independently | Use freshness to decide whether unsupported function proof can be projected |
| `semantic_health_effect()` | Unsupported evaluator scopes keep base health | Keep this invariant and add tests for `support_status: unsupported` |
| Status/export JSON | Can only expose what projection leaves on the passport | Mirror the same `SemanticReview` shape from passports |
| JSON fixtures | Still encode old implicit unsupported shape | Refresh to include `support_status`, reasons, and hints where current |

### Security and Trust Model

No new auth, network, filesystem, or secret surface exists. The threat is trust corruption:

- agent mistakes unsupported for supported under-specification;
- stale unsupported proof is presented as current;
- status/export silently erase a current unsupported explanation;
- consumers branch on legacy overloaded fields forever.

The mitigation is an explicit branch key plus freshness-aware projection.

## Code Quality Review

### Engineering Rules

- **DRY**: one refresh/preserve projection path. No unsupported-only command branch.
- **Explicit over clever**: `support_status` is the branch key. Do not keep teaching consumers to
  infer from `verdict` plus `evaluator_scope`.
- **Minimal diff**: no new semantic family and no new command.
- **Boring by default**: use serde-default additive fields, existing enums, existing fixtures, and
  existing Cargo test harness.
- **Engineered enough**: reason taxonomy is small and stable, but not a single generic catch-all.

### Over-Engineering Guardrail

Do not build a broad unsupported ontology. M20 needs function-specific diagnostic truth for the
current admitted families, not a theory of all future unsupported code.

### Under-Engineering Guardrail

Do not ship only better strings. The plan is not complete unless machine consumers can branch on
`support_status` and tests prove fresh-vs-stale unsupported projection.

## Test Review

### Framework Detection

Runtime: Rust workspace with Cargo.

Primary commands:

- `cargo test -p spec-core ...`
- `cargo test -p spec-cli --test cli ...`
- `cargo test -p spec-cli --test m14_regressions ...`
- `cargo run -p spec-cli -- ...`

### Code Path Coverage Diagram

```text
CODE PATH COVERAGE
==================
[+] spec-core/src/semantic_review.rs
    │
    ├── SemanticReview additive fields
    │   ├── [GAP] new supported reviews serialize support_status=supported
    │   ├── [GAP] legacy supported JSON without support_status has effective supported status
    │   ├── [GAP] legacy unsupported JSON without support_status has effective unsupported status
    │   └── [GAP] unsupported function reviews serialize support_status=unsupported
    │
    ├── unsupported_function_review()
    │   ├── [GAP] control flow -> unsupported_control_flow + rewrite hint
    │   ├── [GAP] computed required arg -> unsupported_required_argument_expression + hint
    │   ├── [GAP] dep topology miss -> unsupported_dep_topology + hint
    │   ├── [GAP] wrapper body miss -> unsupported_wrapper_body_shape + hint
    │   ├── [GAP] arithmetic shape miss -> unsupported_arithmetic_shape + hint
    │   └── [GAP] fallback -> unsupported_function_surface + hint
    │
    └── semantic_health_effect()
        └── [GAP] support_status=unsupported remains KeepBase

[+] spec-core/src/passport.rs
    │
    ├── project_passport_truth_with_context()
    │   ├── [GAP] fresh unsupported function review preserved
    │   ├── [GAP] stale unsupported function review dropped
    │   ├── [GAP] supported -> unsupported transition after spec test refresh
    │   └── [GAP] unsupported -> supported transition after spec test refresh
    │
    └── build_passport_preserving_proof_state_with_context()
        └── [GAP] build/generate preserve fresh unsupported proof without refreshing it

[+] spec-core/src/export.rs
    │
    ├── [GAP] export includes fresh unsupported function details
    ├── [GAP] export drops stale unsupported function details
    └── [GAP] export does not invent unsupported data/sum diagnostics

[+] spec-cli/src/commands.rs + spec-cli/tests/cli.rs
    │
    ├── [GAP] status JSON includes fresh unsupported function details
    ├── [GAP] status text includes readable rewrite hint
    ├── [GAP] unsupported function status remains valid when base health is valid
    └── [GAP] unsupported function does not hide stale base health

[+] spec-cli/tests/m14_regressions.rs
    │
    ├── [GAP] Family A down M19 behavior unchanged
    ├── [GAP] Family A up M19 behavior unchanged
    └── [GAP] Family B wrapper/pipeline M19 behavior unchanged
```

### User and Agent Flow Coverage Diagram

```text
USER / AGENT FLOW COVERAGE
==========================
[+] Maintainer runs spec test on unsupported function
    ├── [GAP] passport records support_status=unsupported
    ├── [GAP] reason code is specific, not only unsupported_surface
    └── [GAP] summary/rewrite hint tells author what to try next

[+] Maintainer runs spec status --format json after fresh proof
    ├── [GAP] unit remains valid if no other issue exists
    └── [GAP] semantic_review is visible and machine-readable

[+] Maintainer edits unsupported function body after proof
    ├── [GAP] status reports stale authored truth
    └── [GAP] stale unsupported semantic_review is not projected as current

[+] Downstream consumer reads export bundle
    ├── [GAP] branches on support_status instead of overloaded legacy shape
    └── [GAP] sees the same reason/hint taxonomy as status JSON
```

Coverage target: **0 remaining gaps** before implementation is considered green.

### Required Tests

| Area | Required tests |
|---|---|
| Model compatibility | serde round-trip for old review JSON without `support_status`; explicit field wins over legacy inference; effective status for old supported and old unsupported shapes; new unsupported review explicit |
| Reason taxonomy | one unit test per unsupported reason bucket and fallback, with lead-reason priority asserted |
| Rewrite hints | deterministic hint per reason bucket, serialized as public `rewrite_hints` |
| Health neutrality | unsupported review keeps base health in `semantic_health_effect()` and status health |
| Fresh preserve | passport, status, and export preserve current unsupported function review |
| Stale drop | passport, status, and export drop stale unsupported function review |
| Transition rules | supported -> unsupported and unsupported -> supported refresh after `spec test` |
| Cross-kind boundary | data/sum unsupported behavior unchanged in M20 |
| M19 non-regression | Family A down/up and Family B wrapper/pipeline behavior unchanged |

### Fixture Gate

Create:

- `spec-cli/tests/fixtures/m20/unsupported_truth_pack/`

Minimum fixture units:

```text
units/pricing/apply_discount.unit.spec                             supported Family A down
units/pricing/apply_discount_control_flow.unit.spec                unsupported control flow
units/pricing/apply_tax.unit.spec                                  supported Family A up
units/pricing/apply_tax_arithmetic_shape.unit.spec                 unsupported arithmetic shape
units/pricing/checkout_total.unit.spec                             supported Family B wrapper
units/pricing/calculate_total.unit.spec                            unsupported required arg expression
units/pricing/checkout_total_bad_dep_topology.unit.spec            unsupported dep topology
units/pricing/checkout_total_bad_body_shape.unit.spec              unsupported wrapper body shape
```

Required command matrix:

```bash
cargo run -p spec-cli -- test spec-cli/tests/fixtures/m20/unsupported_truth_pack/units --output spec-cli/tests/fixtures/m20/unsupported_truth_pack/src/generated --crate-root spec-cli/tests/fixtures/m20/unsupported_truth_pack
cargo run -p spec-cli -- status spec-cli/tests/fixtures/m20/unsupported_truth_pack --format json
cargo run -p spec-cli -- export spec-cli/tests/fixtures/m20/unsupported_truth_pack
```

Assertions:

- unsupported function review has `support_status: "unsupported"`;
- `verdict` remains compatibility-safe in M20;
- public serialized fields are exactly `support_status`, `unsupported_reason_codes`, and
  `rewrite_hints`;
- `unsupported_reason_codes` and `rewrite_hints` are present for unsupported functions;
- lead unsupported reason follows the classifier priority order;
- health remains neutral;
- stale unsupported proof is not projected as current;
- supported Family A / B fixtures retain M19 behavior.

### Full Final Gate

```bash
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

### Test Plan Artifact

Primary QA artifact:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-eng-review-test-plan-20260426-m20.md`

If this file is missing at implementation time, create it from the `Test Review`, `Fixture Gate`,
and `Full Final Gate` sections above before running `/qa`.

## Error & Rescue Registry

| Codepath | What can go wrong | Rescue action | User sees |
|---|---|---|---|
| Unsupported refresh | Review remains only `under_specified` + generic unsupported key | Add `support_status`, reason taxonomy, and rewrite hints | Explicit unsupported explanation |
| Preserve projection | Fresh unsupported proof is dropped by build/status/export | Preserve current function-unsupported review | Same explanation on read-side commands |
| Freshness projection | Stale unsupported proof survives semantic edit | Drop semantic review when authored truth is stale or unknown | Stale health reason, no fake-current unsupported review |
| Health mapping | Unsupported starts demoting valid units | Keep unsupported reviews mapped to `KeepBase` | Unit remains valid unless another issue exists |
| Consumer compatibility | Strict consumers break on enum change | Preserve `SemanticVerdict`; add fields additively | Stable JSON with explicit branch key |
| Reason taxonomy | Reasons become vague or unstable | Freeze small enum and golden fixtures | Deterministic machine contract |

## Failure Modes Registry

| Codepath | Failure mode | Test required | Error handling | User impact if missed |
|---|---|---|---|---|
| `unsupported_function_review()` | Emits only generic fallback for all misses | Unit tests per reason bucket | Deterministic fallback | Author gets useless guidance |
| `SemanticReview` serde | Old passports without `support_status` fail to deserialize or misclassify old unsupported review | Legacy JSON round-trip | Optional field plus `effective_support_status()` | Existing projects break or old unsupported proof is misread |
| `project_passport_truth_with_context()` | Fresh unsupported review dropped | Passport preserve test | Preserve when authored truth fresh | Status/export lose explanation |
| `project_passport_truth_with_context()` | Stale unsupported review preserved | Passport stale test | Drop when authored truth stale/unknown | Agent trusts old proof |
| `semantic_health_effect()` | Unsupported demotes valid health | Unit + status tests | `KeepBase` for unsupported | False non-green status |
| `export.rs` | Export and status disagree on shape | Export/status fixture tests | Shared projected passport path | Consumers see inconsistent contract |
| M19 supported families | New unsupported logic widens or narrows Family A/B | m14/M19 regression tests | No family-router expansion | False positives or false negatives |
| Docs | README/AGENTS still teach old inference | Docs review in final gate | Update consumer guidance | Agents keep branching wrong |

Critical gap definition: any row with no test and no deterministic rescue is a blocker.

## Performance Review

There is no database, network, cache, or memory concern. The performance risk is test-suite drag.

Rules:

- reason taxonomy and serde behavior belong in `spec-core` unit tests;
- CLI command matrix proves projection and health only;
- do not add one Cargo-backed CLI test per reason bucket if a unit test proves the classifier;
- reuse M19 fixtures where possible, but create a dedicated M20 fixture root for the public command
  contract.

## Observability and Debuggability Review

M20 needs machine output, not a dashboard.

Status/export/passport output must make these obvious:

- `support_status` is the consumer branch key;
- unsupported reason codes are stable;
- rewrite hints are deterministic;
- base health neutrality is intentional;
- stale unsupported proof is absent because it is stale, not because the CLI forgot it.

Human-readable status text may include a concise hint, but JSON is the source of truth.

## Deployment and Rollout Review

No feature flag or migration tool is required.

Safe rollout order:

1. Add additive model fields and serde defaults.
2. Add unsupported diagnostic builder and unit tests.
3. Make passport projection freshness-aware for unsupported functions.
4. Update status/export projection tests.
5. Add M20 fixture root and command matrix.
6. Update README, AGENTS, CLAUDE, and checked-in JSON fixtures.
7. Run the full final gate.

Rollback is a normal git revert. Because M20 avoids a new serialized verdict enum variant, rollback
risk is mostly fixture/docs churn rather than downstream parser failure.

## Consumer Migration Contract

Consumers should move from:

```text
verdict == "under_specified" && evaluator_scope == "unsupported_surface"
```

to:

```text
semantic_review.support_status == "unsupported"
```

Compatibility rules:

- old passports without `support_status` deserialize safely and compute effective status from the
  legacy evaluator scope / compatibility key;
- M20 writes `support_status` in new output;
- strict consumers should ignore unknown additive fields if possible;
- docs must state that `verdict` is still about semantic result inside the current compatibility
  model, while `support_status` answers whether semantic governance is available for this function.

## Implementation Slices

### Slice 1: Additive Semantic Model

- Add `SemanticSupportStatus`.
- Add function-only unsupported reason enum or equivalent stable serialized taxonomy.
- Add public serialized fields named exactly `support_status`, `unsupported_reason_codes`, and
  `rewrite_hints` to `SemanticReview`.
- Add `SemanticReview::effective_support_status()`, with explicit field precedence over legacy
  inference, and round-trip tests.

Exit criteria:

- legacy semantic review JSON deserializes;
- explicit `support_status` wins when present;
- legacy supported and unsupported reviews compute the correct effective support status;
- unsupported review JSON serializes the new branch key.

### Slice 2: Unsupported Function Diagnostic Builder

- Add `UnsupportedFunctionDiagnostic`.
- Classify deterministic unsupported reason buckets.
- Generate rewrite hints from reason buckets.
- Keep generic `unsupported_surface_review(unit_kind)` behavior for data/sum.
- Implement the fixed lead-reason priority order from `Unsupported Reason Taxonomy`.

Exit criteria:

- unit tests prove every M20 reason bucket;
- tests prove deterministic lead-reason ordering when multiple reasons apply;
- no supported Family A / B classifier expansion.

### Slice 3: Freshness-Aware Projection

- Change passport projection so unsupported function review is preserved only when authored truth is
  fresh.
- Keep `spec test` as the only refresh path.
- Ensure status/export consume the same projected truth.

Exit criteria:

- fresh unsupported review appears on passport/status/export;
- stale unsupported review is dropped;
- base stale health still appears.

### Slice 4: CLI, Fixtures, Docs

- Add M20 fixture pack.
- Add command-matrix tests.
- Refresh golden JSON fixtures.
- Update README, AGENTS, CLAUDE, and example docs.

Exit criteria:

- full final gate passes;
- consumer contract is documented in every agent-facing instruction surface.

## Worktree Parallelization Strategy

M20 has parallelization opportunity, but only after the additive model contract is merged. The model
fields are a shared dependency for every other lane.

### Dependency Table

| Step | Modules touched | Depends on |
|---|---|---|
| Additive semantic model | `spec-core/src/` | - |
| Unsupported diagnostic builder | `spec-core/src/` | Additive semantic model |
| Freshness-aware projection | `spec-core/src/`, `spec-cli/src/` | Additive semantic model |
| CLI fixture and command matrix | `spec-cli/tests/`, `spec-cli/tests/fixtures/` | Additive semantic model, diagnostic builder, projection |
| Docs and consumer contract | repo root docs, examples docs | Additive semantic model |
| M19 non-regression verification | `spec-cli/tests/`, examples | Diagnostic builder, projection |

### Parallel Lanes

```text
Lane A: Additive semantic model
        then Unsupported diagnostic builder
        (sequential, shared spec-core/src/semantic_review.rs)

Lane B: Freshness-aware projection
        (can start after Lane A model fields, touches passport/export/commands)

Lane C: Docs and consumer contract
        (can start after Lane A model names are stable)

Lane D: CLI fixture and command matrix
        then M19 non-regression verification
        (waits for A + B because assertions need final JSON shape)
```

### Execution Order

1. Launch Lane A first. Merge the additive model fields before parallel work starts.
2. Launch Lane B and Lane C in parallel worktrees after Lane A model names are stable.
3. Merge Lane B before Lane D.
4. Launch Lane D after projection behavior is final.
5. Run the full final gate in one integration worktree after all lanes merge.

### Conflict Flags

- Lane A and Lane B both touch `spec-core/src/`, so do not run them concurrently before the model
  fields are merged.
- Lane B and Lane D both touch behavior visible in CLI tests. Lane D should wait or it will churn
  expected JSON.
- Lane C can run in parallel with Lane B if docs reference stable field names only.

## Success Criteria

- `semantic_review.support_status` exists and is the documented unsupported branch key.
- Unsupported function reviews carry stable reason codes and rewrite hints.
- `SemanticVerdict` remains compatibility-safe in M20.
- `spec test` refreshes unsupported function truth.
- `spec build`, `spec generate`, `spec status`, and `spec export` preserve fresh unsupported
  function truth without minting it.
- Stale unsupported function truth is not projected as current.
- Unsupported function review remains health-neutral.
- Data/sum unsupported behavior is unchanged.
- M19 supported-family behavior stays green.
- README, AGENTS, CLAUDE, fixtures, status JSON, and export JSON all teach the same contract.

## Cross-Phase Themes

- **Truth before breadth**: M20 should improve the negative path before adding Family C.
- **One projection path**: refresh/preserve semantics must stay centralized.
- **Explicit beats overloaded**: `support_status` removes inference from `verdict` and
  `evaluator_scope`.
- **Compatibility matters**: additive fields beat enum churn for this milestone.
- **Tests are the contract**: fixture and golden JSON coverage are not garnish here.

## Outside Voice Integration

First M20 pass used Codex outside voice only.

Findings incorporated:

- M20 must change operator behavior, not just labels.
- A direct serialized `SemanticVerdict::Unsupported` change is too risky for M20.
- The unsupported path is generic across unit kinds today, so M20 must be function-only.
- Rewrite guidance needs a diagnostic carrier, not just new summary strings.
- Consumer migration must be explicit.

No unresolved user challenge remains. The resulting plan keeps the user's M20 direction and narrows
the implementation to a compatibility-preserving truth-surface change.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | M20 targets unsupported-function truth, not a new family | Mechanical | Completeness | Positive-path trust is proven; negative-path trust is the next gap | Add Family C |
| 2 | CEO | Use selective expansion | Mechanical | Boil lake | Fix the exact trust surface under pressure | Reopen semantic roadmap |
| 3 | Eng | Add `support_status` instead of `SemanticVerdict::Unsupported` | Mechanical | Compatibility | Additive field avoids strict enum parser break | Breaking enum variant in M20 |
| 4 | Eng | Keep `unsupported.function.v1` unless implementation proves a version bump | Taste | Pragmatic | New branch key can carry the contract without key churn | Immediate compatibility-key bump |
| 5 | Eng | Preserve fresh unsupported function review on read-side surfaces | Mechanical | Completeness | Current read-side drop hides useful current truth | Keep dropping all unsupported review |
| 6 | Eng | Drop stale unsupported function review | Mechanical | Truthfulness | Unsupported proof must not survive semantic edits as current | Preserve stale explanation |
| 7 | Eng | Keep taxonomy small and function-only | Mechanical | Explicit over clever | Stable buckets beat an ontology nobody can maintain | Cross-kind unsupported redesign |
| 8 | Eng | Unit-heavy tests, CLI only for projection | Mechanical | Minimal diff | Prevent Cargo-backed fixture explosion | One CLI fixture per classifier detail |

## Completion Summary

```text
+====================================================================+
|              M20 PLAN SOLIDIFICATION SUMMARY                       |
+====================================================================+
| Scope                 | unsupported function truth surface          |
| UI scope              | no                                          |
| Primary architecture  | additive support_status + diagnostics       |
| Breaking enum change  | rejected for M20                            |
| Projection rule       | preserve fresh, drop stale                   |
| Health rule           | unsupported remains neutral                  |
| Test posture          | spec-core unit-heavy, CLI projection gate    |
| Not in scope          | written                                     |
| What already exists   | written                                     |
| Error/rescue registry | written                                     |
| Failure modes         | written                                     |
| Test diagram          | written                                     |
| Parallelization       | 4 lanes, gated by additive model             |
| Final gate            | written                                     |
+====================================================================+
```

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|---|---|---|---:|---|---|
| CEO Review | `/autoplan` | Scope and strategy | 1 | clear | Truth-before-breadth, no Family C in M20 |
| Codex Review | `/autoplan` | Independent plan challenge | 1 | incorporated | Avoid breaking verdict enum; add consumer migration |
| Eng Review | `/plan-eng-review` | Architecture and tests | 1 | clear | support_status, freshness-aware projection, full test gate |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | No UI scope |

**VERDICT:** CEO + ENG CLEARED. M20 is ready to implement after the additive model contract is
accepted.
