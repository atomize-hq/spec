# M50: Canonical Seam Family Migration Implementation Plan

Status: **implementation plan**  
Milestone: **M50**  
Milestone family: **semantic-review-contract-completion**  
Implementation readiness: **ready for bounded execution**  
Plan scope: **rename the canonical seam examples to family-aligned names, retire legacy seam aliases, and re-prove all canonical read-side and teaching surfaces**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Last rewritten: **2026-05-11**

Supersedes:
- the prior repo-root M49 plan previously maintained at this path
- the design draft at [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-131248.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-131248.md)

Primary source artifacts:
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-131248.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260511-131248.md)
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260511-131248.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260511-131248.md)
- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-134424.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-134424.md)
- [ORCH_PLAN.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md)
- [TODOS.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/TODOS.md)

Primary repo surfaces:
- [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs)
- [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs)
- [spec-core/src/passport.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/passport.rs)
- [spec-core/src/generator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/generator.rs)
- [spec-core/src/molecule_evidence.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/molecule_evidence.rs)
- [spec-core/src/escape_hatch.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/escape_hatch.rs)
- [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs)
- [spec-cli/tests/cli.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/cli.rs)
- [spec-cli/tests/m14_regressions.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/tests/m14_regressions.rs)
- [examples/ecommerce/units/pricing](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/examples/ecommerce/units/pricing)
- [examples/ecommerce/src/raw_baseline/pricing](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/examples/ecommerce/src/raw_baseline/pricing)

## Executive Summary

M49 finished the reusable seam-family substrate. Good. The repo can now classify the supported canonical seam shapes by family instead of by one literal example id.

But the repo still ships a split contract:

- refresh emits the family-aligned keys
- preserve still accepts the legacy seam keys
- the canonical ecommerce seams, molecule fixtures, raw baseline modules, plan-spec references, CLI regression fixtures, and teaching docs still center the old example names

That was fine as a temporary migration window. It is not fine as steady state.

M50 closes the window. Rename the canonical sum seam from `pricing/discount_policy` to `pricing/discount_strategy`. Rename the canonical data seam from `pricing/checkout_quote` to `pricing/pricing_quote`. Rename every maintained contract surface that teaches, loads, proves, or exports those canonical examples. Then remove the legacy seam-key preserve window and re-prove the same bounded semantic truth.

No new family support. No new schema. No TypeScript scope expansion. No architecture side quest. Just make the shipped contract read like one coherent product instead of two overlapping eras.

## Decision This Plan Makes

This plan authorizes exactly one bounded milestone:

1. Rename the canonical sum seam example from `pricing/discount_policy` to `pricing/discount_strategy`.
2. Rename the canonical data seam example from `pricing/checkout_quote` to `pricing/pricing_quote`.
3. Rename example-owned companion surfaces that encode those old names:
   - source file paths
   - `.spec.passport.json` artifact paths
   - `.test.evidence.json` artifact paths
   - molecule ids and file names where the canonical seam name is embedded
   - raw baseline module names
   - example `main.rs` narrative string
   - local plan-spec acceptance references
   - maintained CLI/export fixture ids and paths
4. Remove legacy seam-key preservation for:
   - `sum.discount_policy.v1`
   - `data.checkout_quote.v1`
5. Keep the supported seam-family vocabulary unchanged:
   - `sum.discount_strategy.v1`
   - `data.pricing_quote.v1`
6. Re-prove semantic review, export, passport/status/export projection, CLI regression behavior, example plan validation, and maintained docs/commands against the renamed canonical surfaces.

This plan does not authorize:

- new supported seam families
- new supported function families
- generic seam synonym support
- export schema changes
- CLI JSON redesign
- TypeScript product-scope expansion
- shared-core or cross-library follow-on work
- a repo-wide string-eradication campaign outside the canonical-contract blast radius

## Live Validated Basis

Validated from the current tree on `feat/m40-plus` at commit `e7b35a6`.

Observed truth:

- `spec-core/src/semantic_review.rs` already routes supported seams through the canonical family keys:
  - `sum.discount_strategy.v1`
  - `data.pricing_quote.v1`
- that same file still preserves the legacy seam keys on read-side projection paths
- the canonical ecommerce source tree still uses the old example ids and file names:
  - `examples/ecommerce/units/pricing/discount_policy.unit.spec`
  - `examples/ecommerce/units/pricing/checkout_quote.unit.spec`
  - `examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`
- the example tree also contains a second molecule path, `checkout_flow.*`, that still references `pricing/checkout_quote` and therefore must move with the data-seam rename even though its filename stays the same
- the raw baseline modules still use the old canonical names:
  - `examples/ecommerce/src/raw_baseline/pricing/discount_policy.rs`
  - `examples/ecommerce/src/raw_baseline/pricing/checkout_quote.rs`
- `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec` still points at the old local unit ids
- maintained proof surfaces still hard-code old canonical ids, old canonical file paths, or old legacy compatibility keys in:
  - `spec-core/src/export.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/generator.rs`
  - `spec-core/src/molecule_evidence.rs`
  - `spec-core/src/escape_hatch.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`
  - `spec-cli/tests/fixtures/plan-validate-valid-mixed.json`
  - `spec-cli/tests/fixtures/plan-export-valid-mixed.json`
- maintained teaching surfaces still teach the old names in:
  - `README.md`
  - `examples/ecommerce/README.md`
  - `examples/ecommerce/src/main.rs`
  - `AGENTS.md`

That is the real problem. The repo already knows the new family vocabulary. It just has not finished moving the canonical example contract onto that vocabulary.

## Step 0: Scope Challenge

### What Already Exists

| Sub-problem | Existing owner | Reuse verdict |
| --- | --- | --- |
| canonical seam-family truth | `spec-core/src/semantic_review.rs` | reuse, remove the legacy preserve window instead of widening semantics |
| read-side projection | `spec-core/src/export.rs`, `spec-cli/src/commands.rs`, `spec-cli/tests/cli.rs` | reuse, retarget fixture ids and current-state expectations |
| canonical example seams | `examples/ecommerce/units/pricing/*.spec` | reuse, rename instead of inventing new examples |
| canonical proof coverage definitions | `spec-core/src/passport.rs` | reuse, retarget the canonical seam ids and molecule ids |
| raw baseline comparison | `examples/ecommerce/src/raw_baseline/pricing/*.rs` | reuse, rename to keep generated-vs-raw teaching coherent |
| molecule freshness and escape-hatch truth | `spec-core/src/molecule_evidence.rs`, `spec-core/src/escape_hatch.rs` | reuse, update hard-coded canonical ids and file paths |
| generated module-path expectations | `spec-core/src/generator.rs` | reuse, update canonical module-path assertions where the example ids are encoded |
| deep CLI regression pack | `spec-cli/tests/m14_regressions.rs` | reuse, retarget the canonical wedge fixtures instead of creating a second wedge |
| example plan-spec truth | `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec` and JSON fixtures | reuse, rename the local acceptance ids |
| teaching and maintainer workflow docs | `README.md`, `examples/ecommerce/README.md`, `examples/ecommerce/src/main.rs`, `AGENTS.md` | reuse, update the canonical commands and narrative |

### Affected Surface Triage

This is the ambiguity-killer for implementation. Not every old-name literal in the repo means the same thing.

#### Category A: Required M50 updates

These are contract surfaces. Old names must not survive here when M50 lands:

- canonical ecommerce source specs and tracked artifacts
- canonical example molecule ids, `covers`, imports, and evidence
- canonical raw baseline modules and their `mod.rs`
- example `main.rs`
- example plan-spec acceptance ids
- semantic-review preserve/refresh logic
- passport/export/status/read-side proof fixtures that model the canonical current state
- CLI regression fixtures that model the canonical current state
- maintained docs and maintainer commands

#### Category B: Conditional updates

These may contain old-name literals today, but they are only in scope if they encode the canonical example as present-day truth or if renamed files make them fail:

- synthetic helper tests in `spec-core` that use old ids only as placeholder values
- portability/backend-execution tests that do not load the on-disk canonical example tree
- generic type-name, graph, or normalizer tests whose point is formatting or structure, not the canonical example contract

Rule: update them only if they either fail because paths moved or they claim to represent the canonical example as current truth.

#### Category C: Intentionally retained historical literals

These are allowed only in targeted assertions whose point is legacy behavior itself:

- tests proving a legacy key is rejected after M50
- comments or fixture names that explicitly explain historical compatibility behavior

Rule: any retained old-name literal must be clearly tied to a legacy-specific assertion. Old names are not allowed to remain as the unmarked default fixture language for current-state tests.

### Minimum Complete Slice

This is the minimum honest implementation:

1. freeze the rename map and alias-removal contract in `semantic_review.rs`
2. rename the canonical example source tree and tracked example artifacts
3. retarget every maintained proof wall that treats those examples as a contract surface
4. update the docs and maintainer commands so the repo teaches only one vocabulary
5. regenerate and re-prove the canonical example artifacts

Anything smaller is fake done. It would leave the product core saying `pricing_quote` while the README, fixtures, or passports still say `checkout_quote`.

### Complexity Check

This milestone touches more than eight files. Normally that is a smell.

Here it is justified because all extra files are in the blast radius of one narrow contract decision: what the canonical seam examples are called and what semantic-review keys they preserve. We are not adding new architecture, new crates, or new feature scope.

Recommendation: **accept the expanded blast radius, but forbid behavioral widening beyond rename completion and alias retirement.**

### Completeness Check

Choose the complete version:

- rename the canonical examples everywhere they are a maintained contract surface
- retire the old seam keys fully
- refresh the tracked example proof artifacts from source specs
- update the docs and commands that teach the canonical workflow

Do not ship a halfway version. With CC, the delta between partial cleanup and complete cleanup is minutes. The cost of leaving split vocabulary behind is weeks of future archaeology.

### TODOS Cross-Reference

Relevant existing TODOs:

- `Canonical example as compatibility surface` is directly aligned with this milestone and is exactly why the example tree must move with the product contract
- `Remove deprecated cover-derived molecule imports fallback` is unrelated and remains deferred
- `Wrapper TypeScript execution in spec` and the rest of the post-M46 TypeScript backlog remain explicitly out of scope

Default expectation: **M50 should not create new TODOs.**  
Exception: if a Category B surface is intentionally deferred because it is truly historical and non-blocking, capture that explicitly in `TODOS.md` instead of silently leaving ambiguity behind.

### Locked Plan Decisions

These are frozen. Do not reopen them mid-implementation:

1. Canonical seam ids become:
   - `pricing/discount_strategy`
   - `pricing/pricing_quote`
2. Canonical source files become:
   - `examples/ecommerce/units/pricing/discount_strategy.unit.spec`
   - `examples/ecommerce/units/pricing/pricing_quote.unit.spec`
3. Canonical raw baseline modules become:
   - `examples/ecommerce/src/raw_baseline/pricing/discount_strategy.rs`
   - `examples/ecommerce/src/raw_baseline/pricing/pricing_quote.rs`
4. The mixed-kind molecule that embeds the old sum seam name is renamed:
   - `pricing/discount_policy_checkout_flow` -> `pricing/discount_strategy_checkout_flow`
   - `discount_policy_checkout_flow.test.spec` -> `discount_strategy_checkout_flow.test.spec`
5. `checkout_flow.test.spec` keeps its filename, but its `covers`, imports, and tracked evidence must move from `pricing/checkout_quote` to `pricing/pricing_quote`.
6. The supported semantic families do not widen. Only the canonical naming surface changes.
7. `Refresh` and `Preserve` both speak only the canonical seam-family keys after M50 lands.
8. Generated artifacts are refreshed from source specs. They are never hand-edited.
9. M50 is not a global search-and-replace milestone. Category B surfaces move only if they are true contract surfaces or break because of the rename.

### Abort and Re-scope Triggers

Stop and re-scope if any of these become necessary:

1. export JSON needs a schema-version bump
2. CLI command semantics need to change beyond rename expectations
3. the rename requires a new generic alias framework instead of straight removal
4. TypeScript backend behavior needs product-scope changes rather than fixture retargeting
5. the rename breaks a non-example public contract outside the canonical teaching and proof surfaces named in this plan

## Architecture and Ownership

### Canonical Rename Contract

| Old surface | New surface | Notes |
| --- | --- | --- |
| `pricing/discount_policy` | `pricing/discount_strategy` | canonical `kind: sum` seam id |
| `pricing/checkout_quote` | `pricing/pricing_quote` | canonical `kind: data` seam id |
| `discount_policy.unit.spec` | `discount_strategy.unit.spec` | canonical sum source spec |
| `checkout_quote.unit.spec` | `pricing_quote.unit.spec` | canonical data source spec |
| `discount_policy.spec.passport.json` | `discount_strategy.spec.passport.json` | tracked canonical sum artifact |
| `checkout_quote.spec.passport.json` | `pricing_quote.spec.passport.json` | tracked canonical data artifact |
| `pricing/discount_policy_checkout_flow` | `pricing/discount_strategy_checkout_flow` | mixed-kind molecule id |
| `discount_policy_checkout_flow.test.spec` | `discount_strategy_checkout_flow.test.spec` | mixed-kind molecule file |
| `discount_policy_checkout_flow.test.evidence.json` | `discount_strategy_checkout_flow.test.evidence.json` | tracked molecule evidence |
| `discount_policy.rs` | `discount_strategy.rs` | raw baseline sum module |
| `checkout_quote.rs` | `pricing_quote.rs` | raw baseline data module |
| `sum.discount_policy.v1` | removed | legacy seam key retired |
| `data.checkout_quote.v1` | removed | legacy seam key retired |

### Required Surface Inventory

#### Authority and source surfaces

- `spec-core/src/semantic_review.rs`
- `examples/ecommerce/units/pricing/discount_policy.unit.spec`
- `examples/ecommerce/units/pricing/checkout_quote.unit.spec`
- `examples/ecommerce/units/pricing/discount_policy.spec.passport.json`
- `examples/ecommerce/units/pricing/checkout_quote.spec.passport.json`
- `examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`
- `examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.evidence.json`
- `examples/ecommerce/units/pricing/checkout_flow.test.spec`
- `examples/ecommerce/units/pricing/checkout_flow.test.evidence.json`
- `examples/ecommerce/src/raw_baseline/pricing/discount_policy.rs`
- `examples/ecommerce/src/raw_baseline/pricing/checkout_quote.rs`
- `examples/ecommerce/src/raw_baseline/pricing/mod.rs`
- `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec`

#### Required proof-wall updates

- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/molecule_evidence.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/fixtures/plan-validate-valid-mixed.json`
- `spec-cli/tests/fixtures/plan-export-valid-mixed.json`

#### Required teaching-surface updates

- `README.md`
- `examples/ecommerce/README.md`
- `examples/ecommerce/src/main.rs`
- `AGENTS.md`

### Dependency Graph

```text
canonical example source specs and tracked artifacts
  │
  ├── discount_strategy.unit.spec
  ├── pricing_quote.unit.spec
  ├── discount_strategy_checkout_flow.test.spec
  ├── checkout_flow.test.spec
  ├── *.spec.passport.json
  ├── *.test.evidence.json
  └── checkout-tax-refactor.plan.spec
          │
          ▼
spec loader / generator / passport / molecule evidence / escape-hatch truth
          │
          ▼
semantic_review.rs
  ├── canonical family routing stays the same
  └── legacy seam-key preserve window is removed
          │
          ├── export.rs
          ├── commands.rs
          ├── cli.rs
          ├── m14_regressions.rs
          ├── generator.rs
          ├── passport.rs
          ├── molecule_evidence.rs
          ├── escape_hatch.rs
          └── README / example README / example main / AGENTS
```

### Invariants

All of these must remain true after M50:

1. `semantic_review.rs` still classifies the same supported seam shapes as before.
2. Wrapper-family support still accepts the renamed canonical seams as supported dependency surfaces.
3. Unsupported renamed-vocabulary near misses remain unsupported.
4. Example plan validation still points at truthful local unit ids.
5. Passport freshness, escape-hatch gates, and molecule freshness still behave the same, just against renamed canonical ids and paths.
6. Docs, commands, and tracked example artifacts all point at the same canonical vocabulary.

### Grep Exit Gate

M50 is done only when both of these are true:

1. Category A paths contain no remaining literals of:
   - `pricing/discount_policy`
   - `pricing/checkout_quote`
   - `sum.discount_policy.v1`
   - `data.checkout_quote.v1`
2. Any retained old-name literal outside Category A is clearly intentional and legacy-specific.

Use one final targeted audit, not a permanent new code path:

```bash
rg -n "pricing/discount_policy|pricing/checkout_quote|sum.discount_policy.v1|data.checkout_quote.v1" \
  README.md AGENTS.md examples/ecommerce spec-core spec-cli/tests spec-cli/src/commands.rs spec-core/src
```

## Implementation Sequence

### Step 1: Freeze the authority contract

Owner: authority lane

Touch:

- `spec-core/src/semantic_review.rs`

Work:

1. confirm the canonical family keys stay exactly:
   - `sum.discount_strategy.v1`
   - `data.pricing_quote.v1`
2. remove legacy seam-key preservation for:
   - `sum.discount_policy.v1`
   - `data.checkout_quote.v1`
3. make the current-state rename contract explicit in local helper fixtures or tests so downstream proof walls consume one frozen vocabulary
4. keep any retained old-name literal only inside targeted legacy-rejection assertions

Definition of done:

- preserve no longer accepts the legacy seam keys
- refresh still emits the same canonical family keys as M49
- downstream lanes have one frozen rename map and one frozen key policy

### Step 2: Rename the canonical example tree

Owner: authority lane

Touch:

- canonical source specs
- canonical tracked passport artifacts
- canonical molecule specs and evidence
- raw baseline modules
- example `mod.rs`
- example plan-spec acceptance ids

Work:

1. rename the two canonical `.unit.spec` files and their `id:` values
2. rename the two tracked canonical passport files to match
3. rename `discount_policy_checkout_flow.test.spec` and its id/evidence file to `discount_strategy_checkout_flow`
4. update `checkout_flow.test.spec` and `checkout_flow.test.evidence.json` so their data-seam references move from `pricing/checkout_quote` to `pricing/pricing_quote`
5. rename the raw baseline pricing modules and update `examples/ecommerce/src/raw_baseline/pricing/mod.rs`
6. update `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec`

Definition of done:

- the example tree loads and reads as one coherent vocabulary
- no tracked example artifact points at a path or id that no longer exists
- both molecule surfaces are truthful:
  - one renamed because its own canonical id changed
  - one updated in place because one of its covered seam ids changed

### Step 3: Retarget proof walls

Owner: downstream proof lanes

Touch:

- `spec-core/src/export.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/molecule_evidence.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/fixtures/plan-validate-valid-mixed.json`
- `spec-cli/tests/fixtures/plan-export-valid-mixed.json`

Work:

1. retarget canonical fixture ids, canonical file paths, and tracked artifact paths
2. rewrite current-state tests that still seed old compatibility keys as if they were the normal contract
3. keep old compatibility keys only inside tests whose point is legacy rejection or historical migration behavior
4. update canonical proof-coverage definitions, generated module-path assertions, molecule freshness lookups, escape-hatch gate fixtures, and CLI status/export fixtures

Definition of done:

- proof walls fail only for real semantic regressions, not stale naming
- no maintained current-state test models the canonical example with the old names
- legacy-key references that remain are clearly intentional and narrow

### Step 4: Update teaching surfaces

Owner: docs lane

Touch:

- `README.md`
- `examples/ecommerce/README.md`
- `examples/ecommerce/src/main.rs`
- `AGENTS.md`

Work:

1. update canonical example file inventories
2. update maintainer commands
3. update narrative text describing the canonical wedge
4. update any nearby ASCII diagrams or workflow snippets that mention the old canonical seam names

Definition of done:

- a maintainer following the docs lands on the renamed canonical files on the first try
- example narrative, proof commands, and raw-baseline references all match the code on disk

### Step 5: Regenerate and re-prove

Owner: integrated branch

Run the source-of-truth loop:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/discount_strategy.unit.spec --format json
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/pricing_quote.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/pricing_quote.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/checkout_flow.test.spec
cargo run -p spec-cli -- plan validate examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec --format json
cargo run -p spec-cli -- status examples/ecommerce --format json
```

Then run the targeted Rust proof walls:

```bash
cargo test -p spec-core semantic_review
cargo test -p spec-core export
cargo test -p spec-core passport
cargo test -p spec-core generator
cargo test -p spec-core molecule_evidence
cargo test -p spec-core escape_hatch
cargo test -p spec-cli cli
cargo test -p spec-cli m14_regressions
```

Escalate to broader suites only if fallout leaks outside the targeted walls:

```bash
cargo test -p spec-core
cargo test -p spec-cli
cargo test --manifest-path examples/ecommerce/Cargo.toml
```

Definition of done:

- canonical renamed examples still classify to the same supported seam families
- tracked artifacts refresh cleanly from source specs
- read-side surfaces export and report only the canonical current-state keys
- the grep exit gate passes

## Code Quality Guardrails

- Keep the change explicit. This is a rename-and-proof milestone, not a framework milestone.
- Centralize the current-state canonical names once per surface. Do not scatter new alias tables across helper fixtures.
- Delete legacy seam-key preservation instead of adding another migration shim.
- Prefer updating existing fixture builders over cloning nearly identical helpers with new names.
- Update any nearby ASCII diagrams that mention the canonical seam names. Stale diagrams are lies.
- Touch source specs first, then regenerate derived artifacts. Never hand-edit generated truth.
- If a Category B surface is left unchanged, make that a deliberate decision, not an accident from an incomplete grep pass.

## Test Review

The goal is full coverage of every renamed or behaviorally changed path. This milestone mixes semantic-contract cleanup with canonical example renaming. That combination is exactly where fake-green regressions happen if coverage is casual.

### Code Path Coverage

```text
CODE PATH COVERAGE
===========================
[+] Canonical example source tree
    │
    ├── discount_policy.unit.spec
    │   └── [REGRESSION] rename file + id to discount_strategy
    ├── checkout_quote.unit.spec
    │   └── [REGRESSION] rename file + id to pricing_quote
    ├── discount_policy.spec.passport.json
    │   └── [REGRESSION] rename tracked artifact path
    ├── checkout_quote.spec.passport.json
    │   └── [REGRESSION] rename tracked artifact path
    ├── discount_policy_checkout_flow.test.spec
    │   ├── [REGRESSION] rename file + test id to discount_strategy_checkout_flow
    │   └── [REGRESSION] update covers/imports/module refs
    ├── checkout_flow.test.spec
    │   └── [REGRESSION] update data-seam covers/imports without renaming file
    └── checkout-tax-refactor.plan.spec
        └── [REGRESSION] update local acceptance ids

[+] Semantic review core
    │
    └── project_semantic_review_with_context(...)
        ├── [ADD] preserve rejects legacy seam-family keys
        ├── [REGRESSION] refresh still emits canonical family keys
        └── [REGRESSION] same supported seam shapes still classify as supported

[+] Proof helpers
    │
    ├── passport.rs
    │   └── [REGRESSION] canonical proof coverage definitions follow renamed seam ids
    ├── generator.rs
    │   └── [REGRESSION] generated module-path assertions use renamed canonical modules
    ├── molecule_evidence.rs
    │   └── [REGRESSION] freshness lookups follow renamed canonical ids
    └── escape_hatch.rs
        └── [REGRESSION] gate recomputation follows renamed canonical ids and file paths

[+] Read-side truth walls
    │
    ├── export.rs
    │   ├── [ADD] canonical passports/export rows use renamed ids
    │   └── [ADD] legacy seam-family keys no longer survive projection
    ├── commands.rs
    │   └── [REGRESSION] status-health fixtures use canonical current-state keys
    ├── cli.rs
    │   ├── [ADD] status/export/test expectations use renamed ids and files
    │   └── [REGRESSION] refresh still rewrites proof truthfully
    └── m14_regressions.rs
        ├── [REGRESSION] canonical wedge fixtures move to renamed files
        └── [REGRESSION] semantic review still reports the same verdicts and states

[+] Teaching surfaces
    │
    ├── README.md
    ├── examples/ecommerce/README.md
    ├── examples/ecommerce/src/main.rs
    └── AGENTS.md
        └── [REGRESSION] example commands and inventories resolve on first copy-paste

─────────────────────────────────
PLANNED COVERAGE: 23 critical paths
  source/artifact rename paths: 8
  semantic-review contract paths: 3
  proof-helper paths: 4
  read-side truth-wall paths: 5
  teaching/workflow paths: 3
CRITICAL REGRESSION TESTS: 10
─────────────────────────────────
```

### Maintainer Flow Coverage

```text
MAINTAINER FLOW COVERAGE
===========================
[+] Canonical author loop
    ├── validate renamed sum seam
    ├── validate renamed data seam
    ├── build units into src/generated
    ├── test renamed sum seam
    ├── test renamed data seam
    ├── test renamed mixed-kind molecule
    ├── test existing checkout_flow molecule with renamed data seam refs
    └── status examples/ecommerce

[+] Read-side proof loop
    ├── export canonical ids after refresh
    ├── status canonical ids after artifact refresh
    ├── plan validate acceptance ids after rename
    └── grep current-state surfaces for stale old-name literals

[+] Docs loop
    ├── repo README commands resolve
    ├── example README commands resolve
    ├── example binary message points at real files
    └── AGENTS workflow points at renamed files
```

### Required Assertion Surfaces

| File | Required updates or assertions |
| --- | --- |
| `spec-core/src/semantic_review.rs` | prove canonical family keys still classify the renamed canonical examples; prove legacy seam-family keys are rejected on preserve |
| `spec-core/src/export.rs` | update canonical fixture ids and paths; prove exported passports use renamed ids and canonical seam keys only |
| `spec-core/src/passport.rs` | retarget canonical proof-coverage definitions and tracked molecule ids |
| `spec-core/src/generator.rs` | update module-path assertions from `checkout_quote` to `pricing_quote` where the canonical example is referenced |
| `spec-core/src/molecule_evidence.rs` | update freshness lookup fixtures for the renamed data seam |
| `spec-core/src/escape_hatch.rs` | update canonical fixture paths, ids, and gate recomputation expectations |
| `spec-cli/src/commands.rs` | rewrite current-state status-health fixtures away from legacy compatibility keys |
| `spec-cli/tests/cli.rs` | update status/export/test expectations, passport paths, molecule evidence paths, and command-surface assertions |
| `spec-cli/tests/m14_regressions.rs` | move canonical wedge fixture expectations to renamed files, ids, and module paths without changing semantic outcomes |
| `spec-cli/tests/fixtures/plan-validate-valid-mixed.json` | update local unit ids and molecule ids |
| `spec-cli/tests/fixtures/plan-export-valid-mixed.json` | update local unit ids and molecule ids |

### Test Plan Artifact

The QA-oriented artifact for this plan lives at:

- [/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-134424.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-eng-review-test-plan-20260511-134424.md)

That file is the short version for `/qa` and `/qa-only`. This plan is the exhaustive implementation version.

## Failure Modes

| Codepath | Real production or maintainer failure | Test coverage required | Error handling today | User-visible impact | Critical gap if missed |
| --- | --- | --- | --- | --- | --- |
| example rename | file path changes but `id:` does not | yes | none | maintainer commands and fixtures drift instantly | yes |
| tracked passport rename | source spec moves but tracked passport filename does not | yes | none | status/export surface stale or missing proof | yes |
| molecule rename | sum seam rename lands but `covers`, imports, or evidence file names stay old | yes | none | molecule proof freshness lies | yes |
| in-place molecule update | `checkout_flow.*` keeps old data-seam refs because its filename did not change | yes | none | build/tests look partially green but canonical data contract is still split | yes |
| alias retirement | preserve silently still accepts legacy seam keys | yes | none | repo claims cleanup but old contract is still live | yes |
| export projection | renamed canonical ids not reflected in exported passports | yes | read-side projection only | external consumers see split-brain vocabulary | yes |
| passport coverage defs | proof coverage still keys off `pricing/discount_policy` | yes | none | canonical seam can report incomplete coverage for the wrong reason | yes |
| generator/module refs | raw baseline or generated imports still point at old module names | yes | compile failure surfaces late | example build breaks after apparently successful rename | yes |
| escape-hatch gate | gate recomputation still keys off old file names or ids | yes | live recompute only | valid seam can look incomplete or stale for the wrong reason | yes |
| docs loop | README, example README, example main, or AGENTS still point at deleted files | yes | none | maintainer hits dead commands on first copy-paste | yes |

## Performance Review

No meaningful runtime performance work is justified here. This milestone is dominated by renames, fixtures, and proof.

Guardrails:

1. Do not add permanent new scans just to enforce the rename. One final grep audit is enough.
2. Do not duplicate canonical rename constants across helper fixtures if one existing builder can own them.

## NOT in Scope

These were considered and are explicitly deferred:

- new seam-family support
- function-family work
- generic alias or synonym support
- TypeScript feature expansion
- export schema changes
- CLI UX redesign
- corpus or promotion work
- roadmap or architecture doc rewrites outside the canonical teaching surfaces named above
- Category B historical or synthetic tests that do not model the canonical current state and do not break because of renamed paths

## Worktree Parallelization Strategy

This plan has a real parallelization opportunity, but only after the authority contract is frozen.

### Dependency Table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Freeze rename map and retire legacy seam aliases | `spec-core/`, `examples/ecommerce/units/`, `examples/ecommerce/src/raw_baseline/`, `examples/ecommerce/plans/` | — |
| B. Core proof-wall rewiring | `spec-core/` | A |
| C. CLI and regression rewiring | `spec-cli/` | A |
| D. Teaching-surface updates | repo root docs, `examples/ecommerce/` docs | A |

### Parallel Lanes

- `Lane A`: contract freeze -> example tree rename -> tracked artifact rename (sequential, shared `examples/ecommerce/`)
- `Lane B`: export -> passport -> generator -> molecule_evidence -> escape_hatch -> any needed proof-helper cleanup (sequential, shared `spec-core/`)
- `Lane C`: commands -> cli -> m14_regressions -> JSON fixtures (sequential, shared `spec-cli/`)
- `Lane D`: README -> examples README -> example main -> AGENTS (sequential, shared docs)

### Execution Order

1. Launch `Lane A` first.
2. Do not start downstream lanes until all of these are frozen:
   - canonical seam ids
   - canonical file rename map
   - canonical tracked artifact rename map
   - legacy seam-key removal decision
3. Launch `Lane B`, `Lane C`, and `Lane D` in parallel worktrees after the freeze gate passes.
4. Merge `Lane B` and `Lane C` first and run the targeted proof suites.
5. Merge `Lane D`.
6. Run the full authoritative proof loop once on the integrated branch.

### Conflict Flags

- `Lane B` must not reopen `semantic_review.rs`. If it needs to, collapse back to sequential execution.
- `Lane C` must not decide canonical names through test fixes. If a CLI assertion forces a rename decision, bounce it back to `Lane A`.
- `Lane D` should not edit source specs, proof files, or generated artifacts. If docs uncover a source mismatch, hand it back to the owning lane.
- `Lane B` and `Lane C` both depend on the renamed example artifact paths. If those paths are still moving, parallelization is premature.

## Implementation Checklist

1. Remove legacy seam-key preservation from `semantic_review.rs`.
2. Freeze the canonical rename map in the authority lane.
3. Rename the canonical sum seam source spec and tracked passport artifact.
4. Rename the canonical data seam source spec and tracked passport artifact.
5. Rename the mixed-kind canonical molecule id, file, and tracked evidence artifact.
6. Update `checkout_flow.*` to the renamed data-seam id without renaming the file.
7. Rename raw baseline pricing modules and update `mod.rs`.
8. Update the example plan-spec acceptance ids.
9. Retarget export, passport, generator, molecule-evidence, and escape-hatch proof fixtures.
10. Retarget commands, CLI, M14 regression, and plan-fixture suites.
11. Update README, example README, example `main.rs`, and AGENTS commands and inventories.
12. Rebuild example generated output and refresh tracked artifacts from source specs.
13. Run targeted proof suites, then broader suites only if fallout demands it.
14. Run the final grep exit gate and classify any remaining old-name literal as intentional or a bug.

## Completion Summary

- Step 0: Scope Challenge, expanded blast radius accepted because it is one bounded rename contract, not scope creep
- Architecture Review: 1 contract change, 3 surface classes, 4 execution lanes
- Code Quality Review: explicit-over-clever guardrails written, no new abstraction layer authorized
- Test Review: coverage diagram produced, 23 critical paths identified
- Performance Review: 0 runtime performance findings, 2 guardrails
- NOT in scope: written
- What already exists: written
- Failure modes: 10 critical gaps flagged
- Parallelization: 4 lanes total, 3 downstream lanes parallel after authority freeze
- Lake Score: 7/7 major recommendations chose the complete option over the shortcut

## Recommended Next Action

Execute `Lane A` first. Freeze the canonical rename map, remove the legacy seam-key preserve window, rename the example source tree, and settle the tracked artifact rename map before any downstream lane touches proof walls or docs.

If `Lane A` lands cleanly, the rest of M50 is straightforward proof-surface rewiring instead of archaeology. That is the whole game.
