# M29 - Scoped Second-Language TypeScript Pilot

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Last rewritten: **2026-05-02**  
Supersedes: **M28 - Shared-Core Boundary Extraction + Escape-Hatch Containment**  
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-182618.md`**  
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**

## Decision

M28 already paid for the shared backend-execution boundary.

M29 is the first pressure test of that claim. It is intentionally narrow:

> prove that one already-promoted family packet can complete a second-language
> scaffold -> smoke -> prove -> certify loop in TypeScript without reopening
> corpus policy, recommendation semantics, passport/status redesign, or a
> repo-wide multi-target rewrite.

The pilot is locked to:

- second target language: **TypeScript**
- family wedge: **`function.arithmetic_leaf.monotone_up.v1`**
- packet scope: **one packet, all four existing buckets**
- proof surface: **`cargo xtask family smoke|prove|certify ... --target-language typescript`**

This milestone answers one question only:

> did the M28 shared-core extraction hold once a non-Rust executable lane
> touched the same family semantics and certification workflow?

If yes, the repo earns a real expand/narrow decision later.

If no, M29 closes with one named leak. No vague optimism.

## Done Means

M29 is complete only when all of these are true:

1. `function.arithmetic_leaf.monotone_up.v1` has a checked-in TypeScript pilot target under the same packet and the same four buckets:
   - `aligned`
   - `drift`
   - `under_specified`
   - `unsupported_near_miss`
2. The shared spec surface can carry a TypeScript executable body for `kind:function` without breaking current Rust fixtures.
3. Semantic review can classify the TypeScript `monotone_up` pilot honestly:
   - aligned stays supported and aligned
   - drift stays supported and semantically drifted
   - under-specified stays supported and under-specified
   - control-flow near miss stays unsupported
4. `cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript` succeeds and preserves a stable TypeScript scaffold contract.
5. `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript` writes a target-aware prove artifact and passes the required suites.
6. `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript` writes a target-aware certification artifact and passes the final gate.
7. The existing Rust lane remains green and path-stable for:
   - packet layout
   - prove/certify artifacts
   - coverage/recommendation artifacts
   - current promoted-family classification tests
8. CI runs the TypeScript pilot lane automatically on this family.
9. M29 closes with an explicit next-step verdict:
   - `EXPAND`
   - `NARROW`
   - `STOP`

## Hard Non-Goals

M29 does **not** do any of the following:

- broaden to a second family
- broaden to a second second-language
- add repo-wide TypeScript support for `kind:data` or `kind:sum`
- redesign passports, `spec status`, `spec export`, or freshness accounting to be multi-target
- reopen M27 or M27.5 recommendation/corpus semantics
- change current Rust promotion ordering or routing semantics
- add npm publishing, package distribution, or a user-facing TypeScript product
- replace current Rust packet layout
- add a general `spec build/test --target-language typescript` CLI surface
- invent a generic multi-language plugin architecture
- treat TypeScript local proof as authority outside the packet certification lane

This is one honest pilot, not a portability manifesto.

## Current Repo Truth

### What M28 already proved

- `spec-core/src/backend_execution.rs` exists as the explicit shared backend-execution boundary.
- seam marker collection, digests, and read-side projection no longer need to be rediscovered from scattered Rust-only logic.
- M28 explicitly closed with `M29 decision: go`.

### What is still Rust-hardcoded today

The second-language seam is not theoretical. It is visible in shipped code:

- [`spec-core/src/types.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs)
  - `Body` is `body.rust` only
  - normalized function IR stores `body_rust` only
- [`spec-core/src/validator.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs)
  - function-body validation is Rust `syn` based
  - non-function body validation is Rust-specific
- [`spec-core/src/generator.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/generator.rs)
  - function lowering emits Rust only
- [`spec-core/src/semantic_review.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs)
  - supported-family classifiers parse Rust AST only
- [`xtask/src/lib.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs)
  - `family smoke|prove|certify` take only `<family>`
- [`xtask/src/family/layout.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/layout.rs)
  - packet buckets are hard-wired to `Cargo.toml`, `src/main.rs`, and `units/`
- [`xtask/src/family/scaffold.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/scaffold.rs)
  - scaffolding is Rust-only
- [`xtask/src/family/report.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/report.rs)
  - proof artifacts assume one artifact root per family, not per non-default target
- [`xtask/src/family/promotion_artifacts.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/promotion_artifacts.rs)
  - `TargetLanguage` is `Rust` only
  - recommendation validation rejects non-Rust target values
- [`xtask/src/family/paths.rs`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/family/paths.rs)
  - packet artifact roots have no non-default target partition
- [`.github/workflows/ci.yml`](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.github/workflows/ci.yml)
  - CI is Rust-only

That is the real M29 wedge.

## Step 0 - Scope Challenge

### Premise challenge

1. M28 already answered the shared-core prerequisite honestly enough to justify one second-language pilot.  
   Verdict: **accept**
2. The smallest honest pilot is one already-promoted arithmetic leaf, not a wrapper family.  
   Verdict: **accept**
3. TypeScript is the right second-language truth test for this repo.  
   Why:
   - the architecture docs already name Rust and TypeScript as the intended output pair
   - it immediately breaks the Cargo-only assumption
   - it does not require inventing a second compiled systems toolchain  
   Verdict: **accept**
4. The pilot should stay packet-local and proof-local instead of dragging passports, status, or export into a multi-target redesign.  
   Verdict: **accept**
5. The first proof should target `function.arithmetic_leaf.monotone_up.v1`, not `function.wrapper.pipeline.v1`.  
   Why:
   - smaller starter-case set
   - no composed-call topology at the same time as the language seam
   - still exercises the optional helper-dep shape through `money/round`  
   Verdict: **accept**

### What already exists

| Sub-problem | Existing code / truth | M29 decision |
|---|---|---|
| Shared promotion loop | `xtask/src/family/harness.rs`, `prove.rs`, `certify.rs`, `report.rs` already own the hard gates | Reuse. Extend with target-language routing, do not invent a second promotion workflow. |
| Promoted leaf packet to pilot | `semantic-families/function.arithmetic_leaf.monotone_up.v1/` already exists and is promoted | Reuse. Do not create a new family id. |
| Shared backend-execution containment | `spec-core/src/backend_execution.rs` already boxed seam-specific backend execution | Reuse as precedent. Do not reopen M28. |
| Supported-family evaluator shape | `spec-core/src/semantic_review.rs` already classifies `monotone_up` in Rust | Reuse structure. Add one TypeScript evaluator for the same family. |
| Packet artifact validation | `xtask/src/family/promotion_artifacts.rs` already validates prove/certify artifacts | Reuse. Make it target-aware without changing recommendation/corpus semantics. |
| Rust scaffold/layout contract | `xtask/src/family/scaffold.rs`, `layout.rs`, `smoke.rs` already enforce committed-packet truth | Preserve unchanged for Rust. Add a parallel TypeScript target root. |
| CI baseline | `.github/workflows/ci.yml` already proves Rust discipline | Extend with Node setup and one packet-local TypeScript pilot job. |

### Minimum honest change

The smallest complete M29 diff is:

1. add a TypeScript executable body surface for `kind:function`
2. add a TypeScript `monotone_up` semantic-review wedge
3. add TypeScript target roots, scaffold rules, artifact partitioning, and target-aware family commands for one packet
4. add TypeScript pilot fixtures for the four `monotone_up` buckets
5. run the TypeScript pilot in CI

Anything smaller is fake confidence.

Examples of fake confidence:

- changing artifact schemas without executing TypeScript code
- adding TypeScript fixtures without shared semantic-review support
- adding a TypeScript evaluator without the packet proof loop
- broadening to wrapper families before this leaf lane is green

### Closed implementation surface

Implementation scope is closed to these modules plus the pilot packet:

- `spec-core/src/types.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/semantic_review.rs`
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

Everything else is read-only for M29.

If the pilot requires widening into `spec status`, `spec export`, passport freshness, or non-function lowering to go green, stop and split a follow-on instead of broadening M29 silently.

### Complexity, completeness, and distribution check

- Complexity: this is above the tiny-diff threshold, but still acceptable because it is one family, one second-language, and one proof lane.
- Completeness: the shortcut version would be "run one TypeScript demo by hand." That is not a pilot. M29 chooses the complete bounded version: checked-in scaffold, automated proof loop, CI, and explicit closeout verdict.
- Distribution: M29 introduces no user-facing distributable artifact. The delivery surface is internal only:
  - checked-in TypeScript target scaffold
  - target-aware proof artifacts
  - CI coverage
  - closeout verdict

## Search And Boring-Tech Rule

- **[Layer 1]** Reuse the current family smoke/prove/certify workflow. Do not invent a second certification product.
- **[Layer 1]** Keep the Rust packet layout stable and add a parallel TypeScript target root under the same packet.
- **[Layer 1]** Keep recommendation and coverage artifacts Rust-only.
- **[Layer 1]** Use a real TypeScript parser in Rust for the evaluator. The parser is locked to `swc_ecma_parser`.
- **[Layer 1]** Use `decimal.js` for the pilot runtime numeric type. Do not improvise with JS `number`.
- **[EUREKA]** The seam is not "can we emit a second syntax." The seam is "can the same packet and proof model stay honest once Cargo stops being the execution substrate."

## Locked Pilot Contract

### Language and family

- second language: **TypeScript**
- pilot family: **`function.arithmetic_leaf.monotone_up.v1`**
- no second family
- no second runtime choice

### Command surface

The public contract becomes:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript

cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript

cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Rules:

- `rust` remains the implicit default when the flag is omitted
- the only valid flag values in M29 are `rust` and `typescript`
- `cargo xtask family new` stays Rust-default and unchanged
- there is **no** `cargo xtask family new --target-language typescript` public surface in M29

### Authored source contract

For `kind:function` only, `Body` becomes additive:

```yaml
body:
  rust: |
    { ... }
  typescript: |
    { ... }
```

Rules:

- `body.rust` remains backward-compatible and authoritative for the existing Rust lane
- `body.typescript` is allowed only on `kind:function`
- `body.typescript` is required only for the M29 pilot packet when the TypeScript family proof lane is invoked
- `body.typescript` uses the same authored convention as `body.rust`: block body only, no full function declaration
- `body.typescript` must parse as a TypeScript function body fragment
- `body.typescript` must reject `import` and `export` statements
- seam kinds do not gain TypeScript lowering in M29
- `.test.spec` and `local_tests` do not gain TypeScript execution in M29

### Normalized IR contract

To keep the diff explicit and boring, M29 does **not** introduce a generic language registry.

The narrow contract is:

- `Body` gains `typescript`
- normalized function IR gains `body_typescript`
- Rust-normalized fields stay named and stored exactly as they are today
- callers switch explicitly on `rust` vs `typescript`

No trait-heavy backend abstraction. No plugin model.

### Type mapping contract

M29 locks the TypeScript pilot runtime shape to one numeric library:

- package dependency: `decimal.js`
- generated TypeScript function modules always import `Decimal` from `decimal.js`
- `Decimal` spec contract types map directly to `Decimal`

M29 does **not** solve general type mapping. If the pilot needs more than this narrow mapping, stop and split the follow-on.

### Import contract

`imports[]` stays Rust-oriented in M29.

The TypeScript generator is allowed exactly two import classes:

1. `Decimal` from `decimal.js`
2. relative imports for `deps` within the same generated TypeScript module tree

If a TypeScript pilot body needs arbitrary external imports beyond those two classes, M29 stops. That is out of scope.

## Packet And Artifact Topology

### Committed packet tree

The existing Rust root stays unchanged:

```text
semantic-families/function.arithmetic_leaf.monotone_up.v1/
  candidate.md
  family.toml
  fixtures/
    aligned/
      Cargo.toml
      src/main.rs
      units/**
    drift/
      ...
    under_specified/
      ...
    unsupported_near_miss/
      ...
```

M29 adds a committed TypeScript scaffold root:

```text
semantic-families/function.arithmetic_leaf.monotone_up.v1/
  targets/
    typescript/
      fixtures/
        aligned/
          package.json
          package-lock.json
          tsconfig.json
          src/main.ts
          tests/pilot.test.ts
        drift/
          ...
        under_specified/
          ...
        unsupported_near_miss/
          ...
```

Shared authored unit specs stay under the existing packet Rust-style root:

- `semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/<bucket>/units/**`

### Generated output contract

Generated TypeScript source is **not checked in**.

Proof commands materialize it ephemerally under the TypeScript fixture root:

```text
targets/typescript/fixtures/<bucket>/src/generated/**
```

Rules:

- committed scaffold files are exact-match smoke inputs
- generated TypeScript source is execution-time output only
- `node_modules/` is execution-time only
- proof commands must not dirty the committed packet tree

### Proof-artifact contract

Rust proof artifact paths remain unchanged:

```text
.semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/prove.latest.json
.semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/attempt-<timestamp>.json
.semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/certification.report.json
```

TypeScript proof artifacts are an additive partition under the same family root:

```text
.semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/targets/typescript/prove.latest.json
.semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/targets/typescript/attempt-<timestamp>.json
.semantic-family-artifacts/semantic-families/function.arithmetic_leaf.monotone_up.v1/targets/typescript/certification.report.json
```

Rules:

- Rust paths stay byte-stable
- only non-default targets get a target partition
- recommendation and coverage artifacts remain Rust-only under `.semantic-family-artifacts/family-promotion/**`
- promotion execution artifacts may reference both Rust and TypeScript proof artifacts, but recommendation validation remains Rust-only in M29

## Architecture

### High-level dependency graph

```text
shared packet unit specs
        |
        +--> spec-core validation
        |      |
        |      +--> rust body path (unchanged)
        |      |
        |      +--> typescript body path (pilot only)
        |
        +--> spec-core lowering
        |      |
        |      +--> Rust generated source
        |      |
        |      +--> TypeScript generated source
        |
        +--> semantic review
        |      |
        |      +--> Rust monotone_up classifier
        |      |
        |      +--> TypeScript monotone_up classifier
        |
        +--> xtask family workflow
               |
               +--> layout validation
               +--> smoke scaffold exact-match
               +--> prove execution
               |      |
               |      +--> Rust suite gates
               |      +--> TypeScript temp execution lane
               |
               +--> certify artifacts + final gate
```

### TypeScript prove execution model

The TypeScript lane is not allowed to mutate the committed packet tree.

Execution model:

```text
committed packet
    |
    +--> validate Rust packet root
    +--> validate committed TS scaffold root
    +--> copy TS fixture root + shared units into temp execution dir
    +--> generate src/generated/** into temp dir
    +--> npm ci
    +--> npm run build
    +--> npm test
    +--> write target-partitioned proof artifacts
```

That preserves:

- committed packet truth
- deterministic smoke inputs
- no checked-in `node_modules`
- no checked-in generated TypeScript

### Key design constraints

- Rust stays the reference lane
- TypeScript is additive and pilot-scoped
- no Rust directory churn
- no Rust artifact-path churn
- no corpus/recommendation churn
- no passport/status churn
- no certify-semantic change for the Rust lane

## Implementation Plan

### Workstream 1 - Shared function-body surface and TypeScript lowering

Own:

- `spec-core/src/types.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/generator.rs`

Deliver:

1. Add `body.typescript` to `Body`.
2. Extend normalized function IR with `body_typescript`.
3. Preserve `body.rust` behavior unchanged for all existing Rust callers.
4. Add TypeScript body validation:
   - reject `body.typescript` on `kind:data`
   - reject `body.typescript` on `kind:sum`
   - reject empty TypeScript body strings
   - reject `import` / `export` statements
   - do **not** make global spec validation require `body.typescript` outside the pilot proof context
5. Add TypeScript lowering for `kind:function` only.
6. Generate TypeScript module files under `src/generated/**`.
7. Synthesize function signatures from the existing contract just as Rust already does.
8. Always import `Decimal` from `decimal.js` in pilot-generated TypeScript modules.
9. Derive dep imports from shared `deps` only.
10. Emit no TypeScript local-test lowering in M29.

Acceptance checks:

- Rust generator output remains byte-stable
- function units with only `body.rust` still validate and lower exactly as before
- pilot units with both bodies validate and lower for both targets

Stop condition:

- if TypeScript lowering requires a general multi-target import registry, stop M29 and split the follow-on

### Workstream 2 - TypeScript monotone_up semantic-review wedge

Own:

- `spec-core/src/semantic_review.rs`

Deliver:

1. Add explicit authored executable selection for semantic review:
   - Rust remains the default path
   - TypeScript is opt-in and explicit
2. Parse TypeScript pilot bodies with `swc_ecma_parser`.
3. Implement exactly one TypeScript supported-family evaluator:
   - `function.arithmetic_leaf.monotone_up.v1`
4. Keep the same four truthful outcomes as the Rust lane:
   - aligned
   - drift
   - under_specified
   - unsupported_near_miss
5. Keep all non-pilot TypeScript function bodies unsupported in M29.

Acceptance checks:

- the aligned pilot bucket routes to supported monotone_up
- drift reports semantic drift
- under_specified reports vague truth
- control-flow near miss stays unsupported

Stop condition:

- if honest classification requires generic TypeScript-family normalization beyond this one family, stop M29 and split the follow-on

### Workstream 3 - Target-aware family command plumbing

Own:

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

Deliver:

1. Add `--target-language rust|typescript` to `family smoke`, `family prove`, and `family certify`.
2. Extend `TargetLanguage` from `Rust` only to `Rust | TypeScript`.
3. Preserve Rust default behavior when the flag is omitted.
4. Keep recommendation and coverage artifacts Rust-only.
5. Add target-aware artifact-path helpers for proof artifacts.
6. Add TypeScript committed layout validation under `targets/typescript/fixtures/<bucket>/`.
7. Make the TypeScript proof lane enforce pilot completeness:
   - every shared function unit in the pilot packet must have `body.typescript`
8. Add internal TypeScript scaffold generation used by `family smoke --target-language typescript`.
9. Keep `family new` unchanged.
10. Make `family smoke --target-language typescript` compare committed TypeScript scaffold files against internally scaffolded files.
11. Make `family prove --target-language typescript` execute the TypeScript temp-lane build/test flow and record results in the target-partitioned prove artifact.
12. Make `family certify --target-language typescript` reuse the existing prove-then-certify shape and write target-partitioned certify artifacts.
13. Make certification reporting target-neutral:
    - add `target_language`
    - replace Rust-only runtime wording with target-neutral runtime/toolchain wording

Acceptance checks:

- `cargo xtask family smoke <family>` remains identical for Rust
- `cargo xtask family smoke <family> --target-language typescript` proves the committed TS scaffold exactly matches the internal scaffold contract
- TypeScript prove/certify artifacts never overwrite Rust artifacts

Stop condition:

- if the public command surface needs to grow beyond one explicit `--target-language` flag on the existing family commands, stop and split the follow-on

### Workstream 4 - Checked-in TypeScript pilot packet

Own:

- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`

Deliver:

1. Add `body.typescript` to the packet's shared bucket unit specs.
2. Add committed TypeScript scaffold roots for all four buckets under `targets/typescript/fixtures/`.
3. Check in exactly these TypeScript scaffold files per bucket:
   - `package.json`
   - `package-lock.json`
   - `tsconfig.json`
   - `src/main.ts`
   - `tests/pilot.test.ts`
4. Lock `package.json` scripts to:
   - `"build": "tsc -p tsconfig.json"`
   - `"test": "node --test"`
5. Keep bucket semantic intent identical to the Rust packet:
   - aligned: truthful monotone-up tax function
   - drift: authored intent says increase, executable body contradicts it
   - under_specified: executable body truthful, authored story too weak
   - unsupported_near_miss: semantic intent still monotone-up, executable body uses disallowed control flow
6. Keep the packet-local helper shape through `money/round`.
7. Do not check in generated TypeScript output.

Acceptance checks:

- each bucket has one committed TS scaffold root
- packet-local tests import generated code and assert the same semantic claims as the Rust bucket intent

Stop condition:

- if the packet needs a second TypeScript dependency besides `decimal.js` to express this family honestly, stop and name that seam explicitly

### Workstream 5 - CI and closeout

Own:

- `.github/workflows/ci.yml`
- `PLAN.md`

Deliver:

1. Add a dedicated CI job for the TypeScript pilot lane.
2. Install Node with `actions/setup-node`.
3. Run the exact TypeScript proof loop:
   - smoke
   - prove
   - certify
4. Keep all existing Rust CI steps.
5. Fail CI if either:
   - Rust regressions appear
   - TypeScript pilot smoke/prove/certify fails
6. Close M29 with one explicit verdict:
   - `EXPAND`
   - `NARROW`
   - `STOP`

Acceptance checks:

- CI proves both lanes before the milestone closes
- the closeout names exactly what stayed shared and exactly what leaked

## Test Diagram

### Code-path coverage

```text
CODE PATH COVERAGE
==================
[+] spec-core/src/types.rs
    |
    ├── [REQ] Rust-only function specs still deserialize exactly as before
    ├── [REQ] TypeScript body surface deserializes for function units
    └── [REQ] Non-function units reject TypeScript body surface

[+] spec-core/src/validator.rs
    |
    ├── [REQ] Missing required pilot body is rejected for the pilot packet
    ├── [REQ] Empty TypeScript body is rejected
    ├── [REQ] import/export statements are rejected in body.typescript
    ├── [REQ] Rust validation remains unchanged
    └── [REQ] TypeScript body on seam kinds is rejected

[+] spec-core/src/generator.rs
    |
    ├── [REQ] Rust output remains byte-stable
    ├── [REQ] TypeScript modules emit under src/generated/**
    ├── [REQ] Decimal import is present in pilot output
    ├── [REQ] dep-relative imports resolve correctly inside generated TS output
    └── [REQ] unsupported kinds fail loudly instead of silently emitting garbage

[+] spec-core/src/semantic_review.rs
    |
    ├── [REQ] monotone_up TypeScript aligned fixture routes to supported family
    ├── [REQ] monotone_up TypeScript drift fixture reports semantic drift
    ├── [REQ] monotone_up TypeScript under_specified fixture reports vague truth
    └── [REQ] monotone_up TypeScript control-flow near miss stays unsupported

[+] xtask/src/lib.rs
    |
    ├── [REQ] CLI accepts --target-language typescript on smoke/prove/certify
    ├── [REQ] omitted flag still means rust
    └── [REQ] unsupported target values are rejected

[+] xtask layout / scaffold / smoke
    |
    ├── [REQ] Rust packet layout still passes unchanged
    ├── [REQ] committed TS scaffold root is validated
    ├── [REQ] TS smoke exact-match compares scaffolded TS files only
    └── [REQ] family new remains unchanged

[+] xtask prove / certify / report
    |
    ├── [REQ] TypeScript prove artifact writes under target-partitioned path
    ├── [REQ] TypeScript certify artifact writes under target-partitioned path
    ├── [REQ] certification report includes target_language=typescript
    ├── [REQ] committed packet tree stays clean after TS prove/certify
    └── [REQ] Rust artifact paths remain unchanged
```

### Operator-flow coverage

```text
OPERATOR FLOW COVERAGE
======================
[+] Maintainer verifies scaffold truth
    ├── [REQ] family smoke (rust) stays green
    └── [REQ] family smoke --target-language typescript proves committed TS scaffold truth

[+] Maintainer proves packet
    ├── [REQ] shared packet units lower into temp TS generated output
    ├── [REQ] npm ci runs from a checked-in lockfile
    ├── [REQ] tsc build passes for aligned, drift, under_specified, and unsupported buckets
    ├── [REQ] node --test passes for packet-local TS tests
    └── [REQ] semantic-review suites report the honest aligned/drift/under_specified/unsupported outcomes

[+] Maintainer certifies packet
    ├── [REQ] prove runs first inside certify, just like the Rust lane
    ├── [REQ] certify writes attempt and certification artifacts under the TS partition
    └── [REQ] routing and registry gates remain unchanged for Rust

[+] CI reruns pilot
    ├── [REQ] Node installs reproducibly
    ├── [REQ] Rust lane remains green
    └── [REQ] TS lane failures stop the branch
```

### Required tests to add

1. `spec-core` tests for:
   - `Body` TypeScript deserialization
   - validator acceptance/rejection for `body.typescript`
   - target-specific lowering selection
2. `spec-core` semantic-review tests for TypeScript `monotone_up`:
   - aligned
   - drift
   - under_specified
   - unsupported_near_miss
3. `xtask` tests for:
   - CLI parsing of `--target-language`
   - target-aware layout validation
   - target-aware artifact-path normalization
   - target-aware smoke exact-match enforcement
   - target-aware proof-artifact writing
4. packet-local TypeScript tests under:
   - `semantic-families/function.arithmetic_leaf.monotone_up.v1/targets/typescript/fixtures/*/tests/pilot.test.ts`
5. CI coverage proving the full TypeScript smoke/prove/certify loop

## Failure Modes Registry

| Codepath | Realistic failure | Test coverage required | Error handling required | Operator impact |
|---|---|---|---|---|
| Body selection | TypeScript proof accidentally reads `body.rust` | generator-selection tests | hard error when requested target body is missing | fake green certification |
| TS lowering | generated dep imports point at the wrong relative path | packet-local TS build tests | emit generation failure before npm test | wasted debug time, misleading scaffold |
| TS semantic review | evaluator silently falls back to Rust parsing or generic unsupported | four-bucket classifier tests | explicit target-specific unsupported verdict or parse failure | wrong family truth |
| TS scaffold | committed root passes validation while missing `package-lock.json` or `tsconfig.json` | layout tests | invalid-input failure before prove | late CI failure |
| Temp execution | prove writes `node_modules` or generated TS into the committed packet tree | xtask temp-workspace tests | hard failure if committed tree gets dirty | noisy repo, false diffs |
| Proof artifact pathing | TypeScript prove overwrites Rust `prove.latest.json` | artifact-path tests | target-partitioned path guarantee | Rust lane corruption |
| CI runtime | TypeScript install depends on hidden mutable state | CI job proof | lockfile + `npm ci` only | flaky pilot, low confidence |
| Unsupported near miss | control-flow TypeScript body accidentally classifies as supported | unsupported fixture test | explicit unsupported reason code | false portability claim |

Critical-gap rule:

If any one of those failures is possible without:

- a test
- an explicit error
- and a target-partitioned artifact path when artifacts are involved

then M29 is not done.

## Performance And Cost Guardrails

- Do not change certify semantics just to avoid duplicate work. The Rust lane already reruns prove inside certify. M29 keeps that boring behavior.
- Do not scan the whole repo for TypeScript fixtures. Operate packet-locally under `function.arithmetic_leaf.monotone_up.v1` only.
- Do not introduce a global npm workspace. Each bucket stays self-contained.
- CI caching is optional. Correctness must not depend on cache hits.

## NOT in scope

- general multi-target passports
- target-aware `spec status`
- target-aware `spec export`
- TypeScript support for seam kinds
- TypeScript molecule-test support
- repo-wide `spec-cli` TypeScript UX
- second-family pilot
- second TypeScript runtime choice
- npm publishing
- packet ergonomics cleanup after the pilot

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Shared function-body surface + TypeScript lowering | `spec-core/` | — |
| TypeScript monotone_up semantic-review wedge | `spec-core/` | shared function-body surface |
| Target-aware family command plumbing | `xtask/` | — |
| TypeScript pilot packet | `semantic-families/function.arithmetic_leaf.monotone_up.v1/` | shared function-body surface, target-aware family command plumbing |
| CI pilot lane | `.github/workflows/` | target-aware family command plumbing, TypeScript pilot packet |

### Parallel lanes

- `Lane A`: shared function-body surface + TypeScript lowering, then semantic-review wedge  
  Touches: `spec-core/`  
  Sequence: lowering -> semantic review
- `Lane B`: target-aware family command plumbing  
  Touches: `xtask/`  
  Sequence: CLI -> layout/scaffold/smoke -> prove/certify/report
- `Lane C`: packet TypeScript scaffold and tests  
  Touches: `semantic-families/function.arithmetic_leaf.monotone_up.v1/`  
  Sequence: committed scaffold -> pilot tests
- `Lane D`: CI job  
  Touches: `.github/workflows/`  
  Sequence: after B and C

### Execution order

Launch `Lane A` and `Lane B` in parallel first.

`Lane C` waits for:

- `Lane A` to freeze the generated TypeScript module shape
- `Lane B` to freeze the TypeScript scaffold and artifact-path contract

`Lane D` waits for:

- `Lane B`
- `Lane C`

Final order:

1. `Lane A` + `Lane B` in parallel
2. merge `Lane A`
3. merge `Lane B`
4. run `Lane C`
5. run `Lane D`
6. run the final local proof loop

### Conflict flags

- `Lane A` and `Lane B` are safe to parallelize at the directory level, but they both define the TypeScript proof contract. Merge each before packet work starts.
- `Lane B` and `Lane C` are logically coupled even without shared directories. `Lane C` must not author committed scaffold files before `Lane B` freezes the exact scaffold contract.
- `Lane A` and `Lane C` can diverge on generated module import shape. Freeze `Lane A` first.

## Mandatory Proof Loop

Rust regression guard:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
```

TypeScript pilot proof:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

CI must run both lanes before the milestone closes.

## Closeout Questions

M29 is not complete until the closeout answers these directly:

1. What exactly stayed shared between Rust and TypeScript?
2. What exact assumptions were still Cargo-only or Rust-only?
3. Did the packet artifact contract remain honest across both lanes?
4. Is the next move:
   - another leaf family in TypeScript
   - one named portability-seam repair
   - or stop because the shared-core claim still collapses under second-language proof?

The milestone ends with exactly one verdict:

- **EXPAND**: the TypeScript pilot was green and the remaining debt is local
- **NARROW**: the pilot found one contained portability seam worth fixing next
- **STOP**: the shared-core claim still collapses under second-language proof

## Completion Summary

- Step 0: Scope Challenge — accepted as-is
- Architecture shape — locked to one family, one second-language, one proof lane
- Code quality direction — explicit over clever, no generic backend registry
- Test review — full code-path diagram included, all pilot gaps named
- Performance review — no global npm workspace, no certify-semantic churn
- NOT in scope — written
- What already exists — written
- Failure modes — written with critical-gap rule
- Parallelization — 4 lanes total, 2 launch in parallel, 2 follow sequentially
- Lake score — complete bounded pilot chosen over shortcut demo
