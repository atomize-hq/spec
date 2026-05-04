<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260503-085418.md -->
# M29R - Additive Body Contract Recovery

Status: **authoritative implementation plan**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-03**
Supersedes: **M29 - Scoped Second-Language TypeScript Pilot**
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260503-084150.md`**
Execution authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md` must be rewritten to match this plan before implementation starts**
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**
Blocked checkpoint history: **`d10679a`**
Recovery seed: **`741a83e`**

## Decision

This milestone is now **M29R**.

The product goal did not change. The shared authoring contract did.

M29 assumed the repo could truthfully author a single packet that carried
Rust and TypeScript body truth for the monotone-up family. The current merged
state proves that claim is false on the authoritative shared path. The packet
already contains additive `body.typescript`, but the shared schema and shared
parsed model still behave as if only `body.rust` exists.

M29R is the authority reset:

1. make additive `body.typescript` real in the shared unit schema and shared
   parsed model
2. route the monotone-up TypeScript pilot through that explicit field, not
   hidden metadata
3. re-anchor the read-side proof surfaces on the repaired contract
4. replay packet and CI work only after the foundation is coherent again

## Problem Statement

The blocker is not a packet-formatting bug. It is a shared-core truth bug.

Hard evidence from the current branch:

1. The promoted monotone-up packet under
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/**` now carries
   additive `body.typescript`.
2. `spec-core/src/schema/unit.spec.json` still allows only `body.rust` for
   `kind:function`.
3. `spec-core/src/types.rs` still defines `Body` as Rust-only and
   `ResolvedSpec::from_spec()` still copies only `body.rust`.
4. `spec-core/src/semantic_review.rs::build_authored_function_packet()` still
   rebuilds authored function truth without any TypeScript body field.
5. `spec-cli/tests/m14_regressions.rs` copies the promoted packet fixtures
   directly, so the shared loader path now fails with
   `Schema validation failed: unknown field at /body: typescript`.

There are two hidden blast-radius details that must be called out explicitly:

1. `spec-core/src/types.rs::Body` is reused by both `.unit.spec` and
   `.test.spec`. Widening `Body` carelessly would also widen molecule-test
   authoring. M29R must keep `test.spec` Rust-only.
2. Changing the `Body` struct will trigger compile-fix fallout in test helpers
   and fixture constructors across `spec-core` and `spec-cli`. Those edits are
   mechanical, but they are real. Pretending the change is isolated to three
   files is how this plan would become fake again.

## Done Means

M29R is complete only when all of these are true:

1. `spec-core/src/schema/unit.spec.json` accepts additive
   `body.typescript: string` for `kind:function` and still rejects it for
   `kind:data`, `kind:sum`, and `.test.spec`.
2. `spec-core/src/types.rs::Body` becomes:

   ```rust
   pub struct Body {
       pub rust: String,
       pub typescript: Option<String>,
   }
   ```

   with serde defaults so authored absence stays distinct from authored presence.
3. `ResolvedSpec` carries `body_typescript: Option<String>` explicitly. Rust
   generation still consumes `body_rust` only.
4. No hidden `spec_version` sentinel is required to infer whether a TypeScript
   body exists.
5. `validate_function_semantic()` accepts additive TypeScript on function units,
   while `validate_data_semantic()` and `validate_sum_semantic()` explicitly
   reject top-level `body.typescript`.
6. `test.spec.json` remains Rust-only and `.test.spec` authoring still rejects
   additive `body.typescript`.
7. The monotone-up semantic-review route can read explicit authored TypeScript
   truth directly when the TypeScript pilot asks for it.
8. `cargo test -p spec-cli --test m14_regressions monotone_up_corpus_ -- --color never`
   passes against copied promoted fixtures that include additive
   `body.typescript`.
9. `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1` passes
   from merged state without regressing the Rust-default lane.
10. `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
    and `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
    still pass.
11. CI proves both the ordinary workspace lane and the monotone-up pilot lane
    on the exact pushed SHA.
12. Closeout can truthfully say one packet carries explicit additive truth for
    both targets and both targets read that truth through the same shared
    authoring boundary.

## NOT in Scope

The following work was considered and is explicitly deferred:

- Repo-wide `spec build --target-language typescript`
  Reason: M29R repairs packet-local truth, not repo-wide multi-target CLI.
- Repo-wide `spec test --target-language typescript`
  Reason: same as above.
- TypeScript support for `kind:data`
  Reason: seam kinds stay outside this recovery loop.
- TypeScript support for `kind:sum`
  Reason: same blast-radius control.
- TypeScript support for `.test.spec`
  Reason: molecule tests remain authored as Rust blocks only.
- Passport redesign
  Reason: proof storage is not the current break.
- `spec status` multi-target redesign
  Reason: read-side surface redesign is outside this milestone.
- `spec export` multi-target redesign
  Reason: same blast-radius control.
- Second family rollout
  Reason: one family is the smallest honest proof.
- Second second-language rollout
  Reason: one target is enough to prove the repaired contract.
- npm publishing or external TypeScript distribution
  Reason: this is an internal milestone, not a user-facing artifact release.
- New packet roots such as `semantic-families-typescript/`
  Reason: one packet root must stay authoritative.

## What Already Exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Shared authoring validation entry | `spec-core/src/validator.rs`, `spec-core/src/schema/unit.spec.json` | Reuse. Widen the unit schema truthfully. Keep `test.spec.json` closed. |
| Shared parsed spec model | `spec-core/src/types.rs` | Reuse, but promote `body.typescript` into the real `Body` and `ResolvedSpec` surfaces. |
| Rust generation path | `spec-core/src/generator.rs` | Reuse. Preserve Rust-default output. Do not let TypeScript widen Rust codegen. |
| Semantic family evaluation | `spec-core/src/semantic_review.rs` | Reuse. Re-anchor the TypeScript pilot on explicit authored truth. |
| Corpus truth surface and regression coverage | `spec-cli/tests/m14_regressions.rs` | Reuse. Update fixture/expectation truth so additive packets are legal. |
| Family harness and locked prove/certify selectors | `xtask/src/family/harness.rs`, `xtask/src/lib.rs`, `xtask/src/family/{prove,certify,paths,scaffold,report}.rs` | Reuse. Keep the command surface and selector names. Repair only the contract assumptions underneath. |
| Promoted packet root | `semantic-families/function.arithmetic_leaf.monotone_up.v1/**` | Reuse. Do not introduce a second packet tree. |
| CI baseline | `.github/workflows/ci.yml` | Reuse. Extend, do not replace, the existing workspace gate. |

## Step 0 - Scope Challenge

### Premises

1. The additive authored body contract must be real in the shared loader path,
   not only in a pilot-only side lane.
   Verdict: **accept**
2. Hidden version metadata is the wrong mechanism for target selection here.
   Verdict: **accept**
3. The smallest honest repair is an authority reset in `spec-core`, not a
   prove-suite band-aid.
   Verdict: **accept**
4. The milestone rename to `M29R` is warranted because the implementation
   contract changed materially.
   Verdict: **accept**

### Minimum Change That Still Counts

The minimum honest M29R diff is:

1. widen the shared unit schema and shared `Body` model
2. thread `body.typescript` into `ResolvedSpec` and the monotone-up semantic
   route without widening unrelated execution IR
3. keep `.test.spec` and seam kinds closed to TypeScript bodies
4. align the corpus, prove, certify, and CI surfaces with the repaired contract
5. absorb only mechanical compile-fix fallout caused by the `Body` struct
   change

Anything smaller creates another fake green state.

### Complexity Check

This plan touches more than 8 files. Normally that is a smell.

Here it is still the smallest honest surface because the bug crosses four
layers that must agree:

- authored schema truth
- shared parsed model
- read-side family evaluation and proof selectors
- copied-fixture and CI surfaces that exercise the promoted packet

No new subsystem is introduced. That matters.

### TODOS Cross-Reference

`TODOS.md` contains no open item that blocks M29R directly.

The only relevant standing theme is prior deferral discipline: do not let
multi-target distribution, read-side redesign, or second-family expansion sneak
back in under a "small follow-up" label.

### Completeness Check

The obvious shortcut is to special-case the failing corpus path and keep the
hidden sentinel model alive.

That saves almost no time with AI help and leaves the same contradiction in
place. Reject it.

M29R takes the complete option:

- shared contract repair
- seam/test-schema containment
- regression-proofed read/write surfaces
- explicit pilot verification in both local proof and CI

### Distribution Check

No new user-facing artifact type is introduced.

Internal delivery surfaces only:

- shared schema/model repair
- packet truth replay
- prove/certify artifacts
- CI verification
- milestone closeout

## Closed Implementation Surface

### Primary implementation files

- `spec-core/src/schema/unit.spec.json`
- `spec-core/src/schema/test.spec.json`
- `spec-core/src/types.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/lib.rs` only if error or export plumbing requires it
- `spec-cli/tests/m14_regressions.rs`
- `xtask/src/lib.rs`
- `xtask/src/family/harness.rs`
- `xtask/src/family/layout.rs`
- `xtask/src/family/scaffold.rs`
- `xtask/src/family/smoke.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/family/report.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/paths.rs`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`
- `.github/workflows/ci.yml`
- `PLAN.md`
- `ORCH_PLAN.md`

### Allowed mechanical compile-fix spillover

Because `Body` is a shared Rust struct, adding `typescript: Option<String>` will
require non-semantic compile fixes anywhere the repo constructs `Body { rust: ...
}` literals in tests or fixture helpers. These spillover edits are allowed only
when all of the following are true:

1. the file is only being updated to compile against the widened `Body`
2. the change is mechanical, typically `typescript: None`
3. no runtime behavior or product scope changes in that file

Likely spillover sites include test/helper code in:

- `spec-core/src/{export,escape_hatch,generator,graph,molecule_evidence,normalizer,passport,plan,semantic_review,validator}.rs`
- `spec-cli/src/commands.rs`

Anything beyond that mechanical spillover requires stopping and rewriting the
plan first.

## Architecture

### Current Broken State

```text
promoted packet fixtures
author body.typescript
        |
        v
shared unit schema rejects field
shared Body model drops field
        |
        +--> copied corpus fixtures fail to load
        +--> semantic pilot truth must be rediscovered elsewhere
        +--> Rust prove and TypeScript pilot disagree on what is authored
```

### Target State

```text
authored unit truth
body.rust + body.typescript?
        |
        v
shared unit schema + shared Body model + ResolvedSpec
        |
        +--> validator sees explicit authored truth
        +--> semantic review sees explicit authored truth
        +--> Rust default lane consumes body.rust
        +--> monotone-up TypeScript pilot consumes body.typescript
        |
        v
copied corpus fixtures, prove/certify, packet, and CI all evaluate the same packet honestly
```

### Dependency Graph

```text
spec-core/src/schema/unit.spec.json
            |
            v
spec-core/src/types.rs::Body + ResolvedSpec::from_spec()
            |
            +--> spec-core/src/validator.rs
            +--> spec-core/src/generator.rs
            +--> spec-core/src/semantic_review.rs
            |
            v
spec-cli/tests/m14_regressions.rs
            |
            v
xtask/src/family/{harness,prove,certify,paths,report,scaffold,...}
            |
            v
.github/workflows/ci.yml
```

### Locked Architecture Rules

1. `body.typescript` is represented as `Option<String>`, not `String`, and not
   inferred from `spec_version`.
2. `Body` may be widened, but `.test.spec` remains Rust-only through
   `spec-core/src/schema/test.spec.json`.
3. `ResolvedSpec` may carry `body_typescript`, but `spec-core/src/generator.rs`
   must remain Rust-only in M29R.
4. The monotone-up TypeScript pilot may branch on explicit authored truth and
   explicit CLI target choice, not on hidden version text.
5. The packet root remains
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`.
6. Any code path that silently drops authored `body.typescript` is a bug.

### File-Level Change Map

| Surface | Exact change | Must stay true after change |
|---|---|---|
| `spec-core/src/schema/unit.spec.json` | Widen `function_body` with optional `typescript` string; keep seam bodies closed | `kind:data`, `kind:sum`, and unknown fields still fail |
| `spec-core/src/schema/test.spec.json` | No TypeScript widening; keep Rust-only body schema | `.test.spec` remains Rust-only |
| `spec-core/src/types.rs` | Add `Body.typescript: Option<String>` and `ResolvedSpec.body_typescript: Option<String>`; thread through `ResolvedSpec::from_spec()` | Rust-default consumers still see unchanged `body_rust` |
| `spec-core/src/validator.rs` | Allow function `body.typescript`; explicitly reject it on data/sum seams; keep molecule tests Rust-only through test schema | no cross-kind widening |
| `spec-core/src/generator.rs` | Ignore `body_typescript`; preserve Rust codegen contract | Rust generation output remains stable |
| `spec-core/src/semantic_review.rs` | Thread explicit TypeScript authored truth into monotone-up evaluation where target-language pilot requires it | no sentinel-based target inference |
| `spec-cli/tests/m14_regressions.rs` | Update copied monotone-up fixture expectations so additive packet truth loads and projects correctly | Rust truth-surface and corpus selectors remain locked |
| `xtask/src/family/harness.rs` and family command plumbing | Keep selector names and command matrix stable; ensure prove/certify assumptions match repaired contract | no new packet root, no CLI sprawl |
| `.github/workflows/ci.yml` | Run the targeted monotone-up proof surface alongside workspace health | exact pushed SHA proves both lanes |

### Diagram Maintenance Targets

If implementation adds or changes nearby ASCII diagrams, update them in the
same commit. At minimum review these files for stale diagrams or stale comments:

- `spec-core/src/types.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/semantic_review.rs`
- `xtask/src/family/harness.rs`
- `xtask/src/lib.rs`

## Implementation Plan

### Phase 1 - Shared Contract Repair

Goal: make additive `body.typescript` legal and preservable in the ordinary
shared authoring path without widening other authoring surfaces.

Work:

1. Widen `function_body` in `spec-core/src/schema/unit.spec.json` to allow
   optional `typescript`.
2. Leave `spec-core/src/schema/test.spec.json` unchanged so `.test.spec`
   authoring stays Rust-only.
3. Extend `spec-core/src/types.rs::Body` to:
   `rust: String, typescript: Option<String>`.
4. Extend `ResolvedSpec` with `body_typescript: Option<String>` and thread it
   through `ResolvedSpec::from_spec()`.
5. Apply only mechanical compile-fix updates to helper/test `Body` literals
   outside the primary surface.

Exit criteria:

- shared unit schema accepts additive `body.typescript`
- `.test.spec` schema remains Rust-only
- parsed shared `Body` preserves TypeScript truth
- `ResolvedSpec` carries explicit TypeScript authored truth

### Phase 2 - Re-anchor Shared Consumers

Goal: eliminate authority drift inside `spec-core`.

Work:

1. Update `validate_function_semantic()` so additive TypeScript is allowed for
   function units.
2. Update `validate_data_escape_hatches()` and `validate_sum_escape_hatches()`
   so top-level `body.typescript` is explicitly rejected for seam kinds.
3. Keep `validate_body_rust_block()` and Rust generation rules unchanged for the
   default lane.
4. Update the monotone-up semantic-review path so explicit authored TypeScript
   truth is available without using `spec_version` as a target sentinel.
5. Keep the TypeScript-specific logic bounded to the monotone-up pilot surface.

Exit criteria:

- Rust default lane remains unchanged
- seam kinds still reject top-level TypeScript bodies
- TypeScript pilot logic no longer depends on hidden version semantics
- new read path is explicit and inspectable

### Phase 3 - Re-anchor Prove, Certify, and Corpus Surfaces

Goal: make the authoritative proof paths exercise the repaired contract.

Work:

1. Update `spec-cli/tests/m14_regressions.rs` so copied monotone-up fixtures are
   legal when they carry additive `body.typescript`, and still prove the Rust
   lane remains stable.
2. Keep the existing monotone-up selector buckets:
   `monotone_up_truth_surface_`, `monotone_up_corpus_`,
   `monotone_up_regression_`.
3. Update `xtask/src/family/harness.rs` and adjacent family surfaces only where
   prove/certify expectations assumed the old contract.
4. Keep `xtask/src/family/paths.rs` and `scaffold.rs` locked to the existing
   packet root. No parallel TypeScript packet tree.

Exit criteria:

- monotone-up corpus tests pass with additive packet fixtures
- prove/certify route through the same shared truth the corpus tests use
- no packet-root drift

### Phase 4 - Packet Replay

Goal: re-freeze the packet only after the foundation is honest.

Work:

1. Refresh only the required monotone-up packet fixtures under
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`.
2. Preserve the four required buckets:
   `aligned`, `drift`, `under_specified`, `unsupported_near_miss`.
3. Keep packet truth explicit and additive. No side-channel target encoding.

Exit criteria:

- packet fixtures and family metadata agree with the repaired contract
- local prove and certify consume the refreshed packet without ad hoc shims

### Phase 5 - CI and Closeout

Goal: prove the exact pushed SHA is green for both ordinary workspace health and
the monotone-up pilot lane.

Work:

1. Extend `.github/workflows/ci.yml` so CI explicitly runs the monotone-up proof
   surface in addition to workspace health.
2. Keep the new CI additions targeted. Do not duplicate full-workspace work
   unnecessarily.
3. Close the milestone with one explicit verdict and a truthful note on what
   stayed shared versus what remained pilot-local.

Exit criteria:

- CI runs on the exact pushed SHA
- Rust and TypeScript pilot lanes both pass
- closeout can make a truthful claim about the repaired contract

## Code Quality Review

1. **Explicit over clever**
   The target-language split must be visible in data structures and routing.
   No "magic" inference path.
2. **Minimal diff**
   Add `body_typescript` only where read-side consumers actually need it. Do not
   widen unrelated IR or CLI surfaces.
3. **DRY**
   There must be exactly one shared representation of TypeScript authored truth:
   `Body.typescript` flowing into `ResolvedSpec.body_typescript`.
4. **Contain the blast radius**
   Mechanical `Body` compile-fix edits are allowed. Semantic behavior changes
   outside the primary surface are not.
5. **Rust-default safety**
   Any change that perturbs existing Rust-only callers without opting into the
   TypeScript pilot is a regression.
6. **No dead compatibility baggage**
   If any sentinel or compatibility helper survives temporarily, it must be
   inert, named as temporary, and removed before M29R closeout.

## Test Review

100% planned coverage is required for every surface M29R touches. No "we will
add the regression later."

### Required Coverage Diagram

```text
CODE PATH COVERAGE
===========================
[+] Shared schema and model
    |
    ├── unit schema accepts function body.typescript
    │   └── [REQUIRED] unit schema validation regression
    ├── test schema still rejects body.typescript
    │   └── [REQUIRED] negative test.spec schema regression
    ├── seam kinds still reject top-level body.typescript
    │   └── [REQUIRED] data + sum semantic regressions
    ├── Body model preserves rust + typescript together
    │   └── [REQUIRED] deserialize / round-trip regression
    └── unknown body fields still fail
        └── [REQUIRED] negative schema regression

[+] Shared consumer routing
    |
    ├── ResolvedSpec carries body_typescript explicitly
    │   └── [REQUIRED] normalization / model regression
    ├── validator accepts additive function body without widening seams
    │   └── [REQUIRED] validator regression
    ├── generator still emits Rust-default code from body.rust only
    │   └── [REQUIRED] Rust generation regression
    ├── semantic review reads explicit body.typescript for pilot routing
    │   └── [REQUIRED] monotone-up semantic-review regression
    └── no hidden spec_version target selection required
        └── [REQUIRED] regression proving explicit-field route works without sentinel

[+] Corpus and read-side proof
    |
    ├── aligned fixture projects valid state
    ├── drift fixture projects failing state
    ├── under_specified fixture projects incomplete state
    ├── unsupported_near_miss stays additive-only and neutral
    └── copied fixtures with body.typescript no longer fail schema load
        └── [REQUIRED] extend m14_regressions coverage

[+] Prove / certify command matrix
    |
    ├── cargo xtask family prove ... (Rust default)
    ├── cargo xtask family prove ... --target-language typescript
    ├── cargo xtask family certify ... (Rust default)
    └── cargo xtask family certify ... --target-language typescript
        └── [REQUIRED] harness and selector coverage stay explicit

[+] CI flow
    |
    ├── workspace health lane still passes
    ├── monotone-up pilot proof lane runs on push / PR
    └── failing pilot lane fails CI loudly
        └── [REQUIRED] workflow update + dry-run verification
```

### Required Test/Additive Change Matrix

| Surface | Test home | Required assertion |
|---|---|---|
| unit schema widening | `spec-core/src/validator.rs` schema tests | function units accept `body.typescript`; unknown body keys still fail |
| seam containment | `spec-core/src/validator.rs` semantic tests | `kind:data` and `kind:sum` reject top-level `body.typescript` |
| test-spec containment | `spec-core/src/validator.rs` or loader tests | `.test.spec` rejects `body.typescript` at schema-validation time |
| model threading | `spec-core/src/types.rs` tests | `ResolvedSpec::from_spec()` preserves `body_typescript` |
| Rust-default generation | `spec-core/src/generator.rs` tests | adding `body_typescript` does not change generated Rust |
| monotone-up authored truth route | `spec-core/src/semantic_review.rs` under `monotone_up_classifier_` / `monotone_up_regression_` | explicit authored TypeScript path works without sentinel inference |
| copied packet fixture load | `spec-cli/tests/m14_regressions.rs` under `monotone_up_corpus_` | copied additive fixture loads and projects expected health |
| command matrix stability | `xtask/src/lib.rs` and/or family harness tests | existing prove/certify selectors remain locked |
| CI pilot lane | workflow verification | exact monotone-up proof commands run in CI |

### Required Commands

These commands are the verification floor for M29R:

```bash
cargo test -p spec-core --lib monotone_up_classifier_ -- --color never
cargo test -p spec-core --lib monotone_up_regression_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_truth_surface_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_regression_ -- --color never
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo test --workspace
```

### Regression Rule

Any fix for the current `unknown field at /body: typescript` break must land
with a regression test in the same change set. No exceptions.

## Performance Review

This milestone is correctness-heavy, not throughput-heavy, but there are still
three real performance risks:

1. Do not turn CI into duplicated full-workspace work plus duplicated prove
   work. Keep the added lane targeted to monotone-up proof surfaces.
2. Do not create extra packet trees or duplicated fixture scans that multiply
   filesystem churn for no gain.
3. Do not introduce a second parsed representation of authored TypeScript truth.
   That would buy short-term convenience and permanent maintenance cost.

The right performance posture is boring:

- one packet root
- one shared parse model
- targeted prove/certify suites
- explicit CI additions

## Failure Modes Registry

| Failure mode | Surface | Test required | Error handling expectation | User-visible effect if missed | Critical gap |
|---|---|---|---|---|---|
| Unit schema accepts `body.typescript` but `ResolvedSpec` still drops it | `spec-core/src/schema/unit.spec.json`, `spec-core/src/types.rs` | deserialize / model regression | fail fast in tests | silent truth loss in TypeScript lane | **yes** |
| `Body` widening accidentally makes `.test.spec` accept TypeScript | `spec-core/src/schema/test.spec.json`, loader/validator path | negative test-spec regression | schema rejection | new hidden authoring surface | **yes** |
| Seam kinds silently accept top-level TypeScript bodies | `spec-core/src/validator.rs` | data/sum negative regressions | semantic rejection | contract drift across kinds | **yes** |
| Semantic review still routes through hidden metadata | `spec-core/src/semantic_review.rs` | monotone-up semantic-review regression | explicit assertion that authored field is used | fake green TypeScript classification | **yes** |
| Generator accidentally changes Rust-default behavior | `spec-core/src/generator.rs` | Rust generation regression | ordinary Rust lane tests fail | existing Rust users see unexplained drift | **yes** |
| Corpus fixtures stay truthful but copied fixture load still fails | `spec-cli/tests/m14_regressions.rs` | copied fixture load regression | test failure | milestone appears blocked after foundation repair | no |
| CI continues to run only workspace health and misses pilot proof | `.github/workflows/ci.yml` | workflow verification | CI must fail loudly on pilot break | false confidence on pushed SHA | **yes** |
| Packet root drifts to a TypeScript-only tree | `xtask/src/family/paths.rs`, scaffold surfaces | path-layout regression | hard fail | split-brain packet ownership | **yes** |

## Worktree Parallelization Strategy

Sequential implementation is not required here. There are two legitimate
parallel workstreams once the contract freeze is landed.

### Dependency Table

| Step | Modules touched | Depends on |
|---|---|---|
| 1. Freeze M29R contract and rewrite `ORCH_PLAN.md` | `PLAN.md`, `ORCH_PLAN.md` | — |
| 2. Shared schema/model repair | `spec-core/src/schema/`, `spec-core/src/types.rs`, mechanical `Body` compile-fix sites | 1 |
| 3. Shared consumer re-anchor | `spec-core/src/validator.rs`, `spec-core/src/generator.rs`, `spec-core/src/semantic_review.rs` | 2 |
| 4. Read-side proof and harness alignment | `spec-cli/tests/`, `xtask/src/family/`, `xtask/src/lib.rs` | 2 |
| 5. Packet replay | `semantic-families/function.arithmetic_leaf.monotone_up.v1/` | 3, 4 |
| 6. CI lane update and final verification | `.github/workflows/`, proof commands, final merged surfaces | 5 |

### Parallel Lanes

- **Lane A:** Step 2 -> Step 3
  Sequential, shared `spec-core/` ownership.
- **Lane B:** Step 4
  Independent from Lane A after Step 2 lands, owns `spec-cli/tests/` plus
  `xtask/`.
- **Lane C:** Step 5
  Waits for Lane A and Lane B to merge.
- **Lane D:** Step 6
  Waits for Lane C.

### Execution Order

1. Rewrite `ORCH_PLAN.md` and freeze M29R authority.
2. Land Step 2 first. This is the contract freeze.
3. Launch Lane A and Lane B in parallel from the same post-Step-2 SHA.
4. Merge Lane A and Lane B.
5. Run Lane C.
6. Run Lane D.

### Conflict Flags

- Steps 2 and 3 both touch `spec-core/`, so they stay in the same lane.
- Step 2 may cause mechanical compile-fix spillover in test/helper files. Those
  edits belong to Lane A if they arise from the `Body` shape change.
- Step 4 touches `spec-cli/tests/` and `xtask/`. Keep it out of Lane A to avoid
  merge churn in `spec-core/`.
- Step 6 may need a small follow-up in `xtask/` if CI commands drift. Treat that
  as a late, parent-owned merge risk.

## Explicit Verification Sequence

Run this sequence from merged state before calling M29R done:

```bash
cargo test -p spec-core --lib monotone_up_classifier_ -- --color never
cargo test -p spec-core --lib monotone_up_regression_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_truth_surface_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_corpus_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_regression_ -- --color never
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo test --workspace
```

## Closeout Contract

M29R closes with exactly one verdict:

- `EXPAND` if the repaired shared contract holds and both targets prove cleanly
- `NARROW` if the shared contract repair works but the pilot surface must stay
  narrower than currently planned
- `STOP` if the repo still cannot preserve one honest shared authoring contract
  for Rust and TypeScript without wider architecture work

The closeout must say plainly:

1. what changed in the shared contract
2. whether `.test.spec` and seam kinds stayed closed to TypeScript bodies
3. whether Rust-default behavior stayed stable
4. whether any temporary compatibility baggage survived
5. whether CI proved the exact pushed SHA

Anything softer is not a real closeout.
