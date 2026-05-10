<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260510-111915.md -->
# M45 - Make TypeScript Real For One Bounded Monotone-Up Lane

Status: **authority plan**  
Milestone family: **second-language-backend**  
Implementation readiness: **ready-now**  
Next artifact kind: **authority_plan**  
Autoplan ready: **yes**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Last rewritten: **2026-05-10**  
Primary sources:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260510-111915.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-111915.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-m45-plan-20260510-111915.md`
Supersedes: **M44 - Freeze The Shared-Core Portability Contract**

## Executive Verdict

M45 should make TypeScript real in `spec`, but only for one bounded lane:

- `kind:function`
- semantic family `function.arithmetic_leaf.monotone_up.v1`
- zero-dependency units only
- generated `.ts` modules only
- Bun build and Bun execution only
- atom tests only

Everything else stays explicitly out of scope.

This milestone is not "TypeScript support." It is one honest execution lake inside the main `spec` loop.

## Product Truth Gap

The repo can already:

- author `body.typescript`
- read authored TypeScript in semantic review
- run bounded family-packet proof in `xtask` for selected families

But the first-class `spec` workflow is still Rust-only:

- `spec generate` ignores authored TypeScript bodies
- `spec build` only knows Cargo
- `spec test` only knows Cargo plus Rust-shaped `local_tests.expect`
- `.test.spec` is still Rust-only by design

That means the repo can talk about TypeScript without delivering a TypeScript backend. M45 fixes that by making one bounded path executable, testable, and visible in the same truth surfaces users already trust.

## Repo Truth Basis

### Live code surfaces

- `spec-core/src/generator.rs` has a regression proving authored TypeScript is ignored today.
- `spec-core/src/types.rs` already carries `ResolvedSpec.body_typescript`.
- `spec-core/src/pipeline.rs` only exposes Cargo build/test execution.
- `spec-cli/src/commands.rs` routes `spec build` and `spec test` through Cargo only.
- `spec-core/src/validator.rs` rejects `body.typescript` in `.test.spec` and should keep doing that in M45.
- `xtask/src/family/prove.rs` already gates `--target-language typescript` to `function.arithmetic_leaf.monotone_up.v1` and `function.wrapper.pipeline.v1`.

### Branch truth

- M44 already paid the shared-core portability debt.
- The recommendation surface is now in stop-state, so another family-selection milestone would be churn.
- The next honest product move is first-class second-language execution in `spec`, not more metadata about possible second-language support.

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing owner | M45 action |
|---|---|---|
| authored TS source preservation | `spec-core/src/types.rs`, `spec-core/src/validator.rs` | preserve and make executable for one bounded lane |
| Rust generation path | `spec-core/src/generator.rs` | preserve unchanged as the default lane |
| Rust build/test pipeline | `spec-core/src/pipeline.rs`, `spec-cli/src/commands.rs` | preserve unchanged as the default lane |
| bounded TS packet proof | `xtask/src/family/prove.rs` | reuse as evidence that monotone-up is the right first execution family |
| semantic family truth | `spec-core/src/semantic_review.rs` | use as the bounded eligibility gate |
| molecule-test rejection | `spec-core/src/validator.rs` | preserve for M45 |

### Minimum complete change

M45 is complete only if all of this lands together:

1. `spec generate`, `spec build`, `spec test`, and `spec status` accept a target-language switch.
2. The TypeScript lane emits real generated output for eligible monotone-up units.
3. The TypeScript lane owns one explicit build contract and one explicit atom-test execution contract.
4. The TypeScript lane owns one explicit, bounded translation contract for `local_tests.expect`.
5. TypeScript proof is stored distinctly from Rust proof.
6. Unsupported units fail before Bun runs.

If any one of those is missing, the repo is back to gestures instead of backend support.

### Complexity check

This remains a lake only if M45 refuses these expansions:

- wrapper-family execution
- generic `kind:function` parity
- dependency topology work
- molecule tests
- seam kinds
- package-manager or runtime selection abstraction
- schema redesign beyond additive proof storage

If the implementation needs any of those to succeed, the milestone is scoped wrong.

### Completeness check

The complete version is still cheap enough to do now:

- real generation
- real build
- real test execution
- real proof separation
- real negative-path regressions

The shortcut version would be "build only" or "TS metadata plus packet proof only." That would save very little work and keep the product-truth bug alive. Not acceptable.

### Distribution check

No new end-user artifact is introduced here. The distribution surface is the existing CLI. The only runtime prerequisite added by M45 is Bun, and that prerequisite must be documented in `README.md` and surfaced clearly in CLI failure output.

## Locked Decisions

### 1. The first lane is monotone-up only

M45 supports exactly `function.arithmetic_leaf.monotone_up.v1` inside `spec`.

It does not widen first-class backend execution to `function.wrapper.pipeline.v1` even though `xtask family prove --target-language typescript` already allows wrapper packets. Packet proof can stay broader than first-class execution for one milestone.

### 2. The first lane is zero-dependency only

Eligible M45 TypeScript units must have:

- `kind:function`
- compatibility key `function.arithmetic_leaf.monotone_up.v1`
- `deps: []`

No dependency imports. No helper topology. No "just one dep" loophole. If a unit depends on another unit, it is outside the first lane.

### 3. Bun is the only TypeScript execution contract

M45 uses Bun only:

- `bun build` for the compile gate
- `bun` execution for the generated atom-test harness

No `package.json`. No `tsconfig.json`. No `npm`. No `pnpm`. No `tsc`. No runtime auto-detection layer.

### 4. JavaScript `number` is rejected

The numeric contract for M45 is one generated fixed-point helper backed by `bigint`.

Why:

- the current monotone-up fixtures and `apply_tax` semantics depend on exact decimal-scale equality
- `number` would create fake greens immediately
- one small generated helper is cheaper than pretending decimal drift is acceptable

### 5. The local-test floor is explicit and AST-bounded

TypeScript atom-test translation in M45 supports only one shape:

```text
<unit_fn>(Decimal::new(i, s), Decimal::new(i, s), ...) == Decimal::new(i, s)
```

That means:

- root expression is `==`
- left side is a call to the current unit function
- every call argument is `Decimal::new(int, scale)`
- right side is `Decimal::new(int, scale)`

Anything else fails before generation with a stable error. No Rust fallback. No best effort.

### 6. TypeScript proof never overwrites Rust proof

M45 adds one additive proof surface:

- `target_proofs.rust`
- `target_proofs.typescript`

Each target-proof entry carries the same truth shape the repo already understands:

- `evidence`
- `freshness_anchor`
- `freshness`

Legacy top-level Rust-facing mirrors remain in place for compatibility during M45. They continue to represent Rust, not a cross-target merge.

## Architecture Contract

### Current to target flow

```text
spec generate/build/test/status --target-language typescript
                    |
                    v
         eligibility gate in CLI + validator
                    |
                    +--> reject if:
                    |    - wrong family
                    |    - deps present
                    |    - target is .test.spec
                    |    - expect grammar unsupported
                    |
                    v
          bounded TS backend in spec-core
                    |
                    +--> generated unit .ts modules
                    +--> __spec_ts/runtime.ts
                    +--> __spec_ts/build_entry.ts
                    +--> __spec_ts/local_tests.ts
                    |
                    +--> bun build __spec_ts/build_entry.ts
                    +--> bun __spec_ts/local_tests.ts
                    |
                    v
         target_proofs.typescript proof refresh
                    |
                    v
        spec status --target-language typescript
```

### Ownership table

| Module | Owns after M45 | Must not own |
|---|---|---|
| `spec-core/src/types.rs` | shared `TargetLanguage` enum and target identifiers | CLI parsing logic |
| `spec-core/src/typescript_backend.rs` | bounded TS generation, helper/runtime source, harness emission, expect translation | CLI policy, generic family routing |
| `spec-core/src/generator.rs` | dispatch into Rust or TS generation entrypoints | Bun execution |
| `spec-core/src/pipeline.rs` | Cargo runners plus Bun runners | target eligibility policy |
| `spec-core/src/validator.rs` | bounded-lane semantic eligibility checks and molecule rejection | file emission |
| `spec-core/src/passport.rs` | additive `target_proofs` storage and projection | CLI rendering policy |
| `spec-cli/src/commands.rs` | flag parsing, lane routing, target-aware status selection | TS code generation details |

### Non-negotiable invariants

- Rust remains the default target for every existing workflow.
- `.test.spec` remains unsupported for TypeScript in M45.
- Units with `deps` are unsupported in the TS lane.
- TypeScript proof never overwrites Rust proof.
- Unsupported units fail before Bun runs.
- The bounded TS lane never silently widens to wrapper or generic function families.
- Generated helper/runtime files are emitted once per output root, not once per unit.

## CLI Contract

### `spec generate`

- `spec generate <path> --target-language rust` keeps current behavior.
- `spec generate <path> --target-language typescript` emits:
  - one `.ts` module per eligible unit
  - `__spec_ts/runtime.ts`
  - `__spec_ts/build_entry.ts`
  - `__spec_ts/local_tests.ts`

### `spec build`

- Rust path stays Cargo-backed.
- TS path runs `bun build <output_root>/__spec_ts/build_entry.ts`.
- TS build is a compile gate only. It does not mint proof.

### `spec test`

- Rust path stays Cargo-backed.
- TS path runs a TS build pass, then `bun <output_root>/__spec_ts/local_tests.ts`.
- TS proof is refreshed only by `spec test --target-language typescript`.

### `spec status`

- Add `--target-language rust|typescript`, default `rust`.
- TS status uses `target_proofs.typescript` freshness and evidence.
- If no TS proof exists, TS status is `untested`. It must never silently inherit Rust proof.

### `spec validate`

- No new target-language behavior in M45.
- Validation remains shared, except for the additional bounded-lane eligibility checks that run only when the CLI is about to execute a TS target path.

### `spec export`

- No new target flag in M45.
- Exported passports carry additive `target_proofs`.
- Existing Rust-facing top-level fields remain present for compatibility.

## Numeric And Test Translation Contract

### Fixed-point runtime

Emit exactly one generated runtime helper at:

- `<output_root>/__spec_ts/runtime.ts`

That helper owns only:

- decimal construction from `(int, scale)`
- normalization
- addition
- multiplication
- equality

No generalized decimal library surface. No reusable public runtime story. Just enough for this lane.

### Atom-test translation floor

Translate the parsed Rust `expect` AST, not raw strings.

Accepted M45 grammar:

```text
<unit_fn>(Decimal::new(i, s), Decimal::new(i, s), ...) == Decimal::new(i, s)
```

Rejected in M45:

- method calls
- helper calls
- non-`==` comparisons
- boolean combinators
- non-`Decimal::new` literals
- tests that reference any unit other than the current one

This is intentionally narrow. A bounded translator is a lake. A general Rust-expression translator is an ocean.

## File-By-File Implementation Contract

| File | Change | Why |
|---|---|---|
| `spec-core/src/types.rs` | add `TargetLanguage` enum | one shared target identifier across crates |
| `spec-core/src/lib.rs` | export the TS backend module | keep call sites clean |
| `spec-core/src/typescript_backend.rs` | new bounded TS backend module | isolate TS generation logic from Rust generation |
| `spec-core/src/generator.rs` | dispatch by target and retire the TS-ignore regression | make authored TS executable |
| `spec-core/src/pipeline.rs` | add Bun build/test runners | keep subprocess behavior centralized |
| `spec-core/src/validator.rs` | add bounded TS eligibility checks | fail unsupported shapes before execution |
| `spec-cli/src/commands.rs` | add target-language flags and target-aware proof/status routing | keep CLI behavior explicit |
| `spec-core/src/passport.rs` | add additive `target_proofs` projection and Rust-compat mirrors | separate TS proof from Rust proof |
| `spec-core/src/export.rs` | pass through additive target proof data | keep machine-readable truth honest |
| `spec-cli/tests/cli.rs` | add end-to-end bounded-lane and negative-path regressions | lock behavior at the product surface |
| `README.md` | document Bun prerequisite and bounded lane | keep DX honest |
| `CHANGELOG.md` | record first-class bounded TS execution | mark user-visible truth change |

## Implementation Sequence

### Step 1. Freeze target-language primitives

Add the shared `TargetLanguage` enum and CLI flag plumbing.

Done means:

- every target-aware command parses the same enum
- Rust remains the default without behavior drift
- TypeScript selection is explicit at the command boundary

### Step 2. Freeze bounded TS eligibility

Add validator and CLI guards for:

- family must equal `function.arithmetic_leaf.monotone_up.v1`
- `deps` must be empty
- `.test.spec` is rejected
- `local_tests.expect` must match the bounded AST grammar

Done means unsupported cases fail before generation or Bun.

### Step 3. Add bounded TS generation

Create `spec-core/src/typescript_backend.rs` and emit:

- unit modules
- `__spec_ts/runtime.ts`
- `__spec_ts/build_entry.ts`
- `__spec_ts/local_tests.ts`

Done means the old regression proving TS is ignored is replaced by one proving TS source is emitted and wired into the generated tree.

### Step 4. Add Bun pipeline support

Extend `spec-core/src/pipeline.rs` with Bun-backed build/test helpers and thread them through `spec-cli/src/commands.rs`.

Done means:

- build status lines name the target language
- Bun stderr is surfaced verbatim
- TS build/test use one runner invocation per tree, not per unit

### Step 5. Add target-aware proof honesty

Extend passports with additive `target_proofs` and keep top-level Rust mirrors.

Done means:

- `spec test --target-language typescript` writes only `target_proofs.typescript`
- Rust proof remains untouched
- `spec status --target-language typescript` reads the TS proof path explicitly
- `spec export` carries both proof surfaces honestly

### Step 6. Lock regressions and fixtures

Add aligned, drift, unsupported-near-miss, example-unit, and molecule-negative tests.

Done means the full proof wall below is green.

### Step 7. Refresh docs

Update `README.md` and `CHANGELOG.md` only after the proof wall is green.

Done means the written product boundary exactly matches the landed code.

## Architecture Review

The right architecture is one new backend module plus one new runner branch, not a backend abstraction framework. This is not the moment to invent generic multi-language orchestration.

The lane boundary must be visible in both structure and naming. Use a bounded filename like `typescript_backend.rs`, but keep the module documentation explicit that it is M45 monotone-up-only logic, not a promise of generic TS parity.

No new crate split is justified. No new config system is justified. No "language plugin" concept is justified.

## Code Quality Constraints

- one new backend module, not a backend abstraction layer
- one shared `TargetLanguage` enum, not stringly typed target checks in multiple crates
- no duplicate `run_typescript_*` logic that forks the Rust path structurally when a shared helper would do
- one generated helper/runtime per output root
- no widening from leaf-only to dependency-aware semantics in M45
- keep existing Rust behavior byte-for-byte where possible
- keep the additive proof schema as small as possible to avoid a parallel export redesign

## Test Review

### Required proof wall

```bash
cargo test
cargo run -p spec-cli -- generate semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- build semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/drift/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_plus_tax.test.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
cargo run -p spec-cli -- export examples/ecommerce/units --format json
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

### Expected results

- aligned monotone-up fixtures pass generation, build, and test
- drift fixtures execute successfully while semantic-review drift truth remains drift
- unsupported-near-miss fixtures fail before Bun runs with a stable bounded-lane reason
- `apply_tax.unit.spec` passes if it stays inside the lane
- `discount_plus_tax.test.spec` fails fast with a stable molecule-unsupported message
- `status --target-language typescript` reports TS proof honestly instead of mirroring Rust
- `export` includes additive TS proof data without deleting Rust proof

### Code path coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] target-language CLI plumbing
    ├── rust default path preserved
    ├── typescript explicit path added
    └── status reads target-specific proof

[+] bounded TS validator gate
    ├── family == monotone_up
    ├── deps.is_empty()
    ├── reject .test.spec
    └── reject unsupported expect AST

[+] TS generation lane
    ├── unit module emission
    ├── runtime helper emission
    ├── build entry emission
    └── local test harness emission

[+] Bun execution lane
    ├── build_entry compile passes
    ├── local_tests harness passes
    ├── Bun stderr surfaces on failure
    └── runner invoked once per tree

[+] target-aware proof lane
    ├── rust proof untouched by TS run
    ├── ts proof refreshed by TS run
    ├── ts status reads ts proof only
    └── export carries both proofs

[!] negative-path coverage
    ├── unsupported family rejected
    ├── deps present rejected
    ├── unsupported expect shape rejected
    └── molecule target rejected
```

### Required regressions

1. Replace the current TS-ignore generator regression with a regression proving authored TS is emitted for the bounded lane.
2. Add validator regressions for:
   - wrong family under `--target-language typescript`
   - non-empty `deps`
   - unsupported `expect` AST
   - `.test.spec --target-language typescript`
3. Add pipeline regressions proving Bun, not Cargo, is used in the TS lane.
4. Add CLI regressions for aligned, drift, and unsupported-near-miss fixture roots.
5. Add passport/export/status regressions proving TS proof is additive and separate from Rust proof.
6. Add one example regression for `examples/ecommerce/units/pricing/apply_tax.unit.spec`.

### Regression rule

Any path that can create a silent green TypeScript claim gets a regression test. No exceptions.

## Performance Review

- Bun must run once per generated tree, not once per unit or per local test.
- helper/runtime files must be emitted once per output root
- TS status projection must read existing passport data, not re-run generation
- M45 must not add package-install or dependency-resolution work to the happy path

This is not a performance-sensitive runtime feature. The real performance trap is accidental N-times work in generation or subprocess execution.

## Failure Modes Registry

| Codepath | Failure mode | Test required | Error handling required | User-visible effect if broken |
|---|---|---:|---:|---|
| TS eligibility gate | non-monotone-up unit executes anyway | Y | Y | fake backend support |
| zero-dependency contract | unit with deps slips through | Y | Y | hidden topology scope creep |
| expect translator | unsupported Rust-shaped expect silently drops coverage | Y | Y | false green tests |
| numeric runtime | JS `number` semantics drift from decimal truth | Y | Y | wrong tax math |
| Bun runner | stderr swallowed or mislabeled as Cargo | Y | Y | operator debugs the wrong system |
| target proof storage | TS run overwrites Rust proof | Y | Y | corrupted repo truth |
| TS status projection | status mirrors Rust proof when TS proof is missing | Y | Y | false green read-side surface |
| molecule rejection | `.test.spec` tries to execute in TS lane | Y | Y | accidental scope expansion |

Critical gap rule:

- Any row above without a regression is a release blocker for M45.

## Developer Experience And Docs Contract

Required:

- `README.md`
  - document Bun as the only TS prerequisite
  - document the monotone-up-only boundary
  - document the zero-dependency requirement
  - document atom-test grammar limits
- `CHANGELOG.md`
  - record first-class bounded TS execution in `spec`

CLI output must also be honest:

- target language named in build/test status lines
- Bun missing message includes remediation
- unsupported-lane messages name the exact bounded requirements

Do not document:

- generic TypeScript support
- wrapper parity
- molecule parity
- future runtime configurability

## Worktree Parallelization Strategy

This milestone has one serial contract gate, then two real implementation lanes, then one lock-and-prove lane.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Step 1. Freeze target-language primitives | `spec-core/src/`, `spec-cli/src/` | — |
| Step 2. Freeze bounded TS eligibility | `spec-core/src/`, `spec-cli/src/` | Step 1 |
| Step 3. Add bounded TS generation | `spec-core/src/` | Steps 1, 2 |
| Step 4. Add Bun execution and proof routing | `spec-core/src/`, `spec-cli/src/` | Steps 1, 2 |
| Step 5. Add regressions, fixtures, docs | `spec-cli/tests/`, `semantic-families/`, `examples/`, docs | Steps 3, 4 |

### Parallel lanes

- Lane 0: Step 1 -> Step 2
  - Owns: target-language enum, flag plumbing, bounded eligibility rules
  - Reason: these are shared contracts every downstream lane depends on

- Lane A: Step 3
  - Owns: `spec-core/src/typescript_backend.rs`, `spec-core/src/generator.rs`, `spec-core/src/lib.rs`
  - Goal: emit the bounded TS output tree

- Lane B: Step 4
  - Owns: `spec-core/src/pipeline.rs`, `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs`
  - Goal: execute the TS tree and project proof honestly

- Lane C: Step 5
  - Owns: `spec-cli/tests/cli.rs`, fixture trees, `README.md`, `CHANGELOG.md`
  - Goal: lock the product surface after A and B are stable

- Lane D: final integration
  - Owns: merge coordination, proof wall, parity cleanup
  - Goal: make the whole milestone green together

### Execution order

1. Launch Lane 0 first and freeze the shared target-language contract.
2. Once Lane 0 is merged or stable, launch Lanes A and B in parallel worktrees.
3. Launch Lane C only after A and B have stable file names, error strings, and proof-shape expectations.
4. Merge A and B, then run Lane D on the integrated branch.
5. Run the entire proof wall at the end. Docs do not land ahead of proof.

### Conflict flags

- Lanes A and B both touch `spec-core/src/`, so `TargetLanguage`, output-root conventions, and helper filenames must freeze first.
- Lane B is the highest merge-risk lane because it spans CLI, pipeline, and proof projection.
- Lane C must not snapshot error strings before Step 2 is stable.
- Do not let docs merge before the proof wall is green. This milestone is very easy to over-claim.

## Not In Scope

- `function.wrapper.pipeline.v1` execution in `spec`
- any non-monotone-up function family
- any function unit with `deps`
- `.test.spec` TypeScript execution
- `kind:data` or `kind:sum` TypeScript execution
- generic decimal/runtime reuse beyond the monotone-up helper
- target-language support in `spec validate`
- package-manager detection or JS runtime selection
- schema redesign outside the additive `target_proofs` field

## Acceptance Criteria

M45 is complete only if all of the following are true:

1. `spec generate/build/test/status` accept `--target-language`.
2. Rust remains the default path with no behavior regression.
3. `spec generate --target-language typescript` emits a real TS tree for eligible monotone-up units.
4. The TS tree includes exactly one generated runtime helper and one generated harness per output root.
5. `spec build --target-language typescript` uses Bun successfully on aligned fixtures.
6. `spec test --target-language typescript` refreshes TS proof without touching Rust proof.
7. Units outside the bounded lane fail before Bun runs.
8. `.test.spec --target-language typescript` fails with a stable unsupported message.
9. `spec status --target-language typescript` reads TS proof explicitly and never mirrors Rust by accident.
10. `spec export` carries additive target-proof data.
11. The proof wall passes.
12. The docs describe the bounded lane exactly, with no broader claim.

## Definition Of Done

M45 is done when a maintainer can answer all of these questions by pointing at code, not caveats:

- Can `spec` execute TypeScript at all?
- Exactly which units qualify?
- Exactly which atom-test shapes qualify?
- Which runtime/toolchain is required?
- Where is TS proof stored?
- How do I see TS status without confusing it with Rust status?
- Why do wrapper families, dependency-bearing units, and molecule tests still fail?

If any answer still starts with "well, sort of" then M45 is not done.

## Completion Summary

This is the smallest honest second-language backend milestone.

It does not try to make TypeScript a peer of Rust everywhere. It does not reopen family strategy. It does not build JS infrastructure for its own sake.

It makes one bounded monotone-up lane real, testable, and visible in the product truth surfaces. Then it stops.
