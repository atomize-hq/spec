<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260502-214857.md -->
# M29 - Scoped Second-Language TypeScript Pilot

Status: **implementation contract, recovery rewrite**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-02**
Supersedes: **M28 - Shared-Core Boundary Extraction + Escape-Hatch Containment**
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-213513.md`**
Execution authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**
Blocked checkpoint: **`d10679a`**

## Decision

This milestone stays **M29**.

It does not become `M29A`. The goal did not change. The bounded pilot did not change.
What changed is that the first foundation merge violated the written contract before
packet work could begin.

M29 now resumes through one explicit recovery loop:

1. re-freeze the foundation contract from `741a83e`
2. relaunch the two foundation lanes
3. merge them into a fresh integration base
4. re-open packet freeze
5. continue the original M29 sequence

That is still one milestone.

## Problem Statement

M29 is blocked before `Lane C`, `Lane D`, final proof, push, or CI observation.

The block is concrete:

1. `xtask/src/family/paths.rs` introduced a second packet root,
   `semantic-families-typescript/`, which conflicts with the locked packet root
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`.
2. `xtask/src/family/scaffold.rs` started authoring `body.typescript`, but the
   integrated `spec-core` foundation still deserialized and validated the
   TypeScript pilot through `body.rust`.

This is not packet drift.

This is a foundation-contract mismatch between `spec-core` and `xtask`.

## Done Means

M29 is complete only when all of these are true:

1. The packet stays rooted at
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`.
2. `kind:function` can author both `body.rust` and `body.typescript` additively,
   and Rust-default behavior remains unchanged for every existing caller.
3. The TypeScript lane reads, validates, lowers, and semantically reviews
   `body.typescript`, not `body.rust`.
4. `cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
   succeeds against committed packet truth.
5. `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
   writes target-partitioned proof artifacts and passes the required suites.
6. `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
   writes target-partitioned certification artifacts and passes the final gate.
7. The existing Rust lane remains green and path-stable for packet layout,
   recommendation surfaces, prove/certify artifacts, and promoted-family tests.
8. CI runs the TypeScript pilot automatically on the M29 packet.
9. M29 closes with one explicit verdict:
   - `EXPAND`
   - `NARROW`
   - `STOP`

## NOT in Scope

The following work was considered and is explicitly deferred:

- Repo-wide `spec build/test --target-language typescript`
  Reason: M29 is a packet-local proof lane, not a productized multi-target CLI.
- TypeScript support for `kind:data` or `kind:sum`
  Reason: that spends a second innovation token before the first one is proven.
- Passport, `spec status`, or `spec export` multi-target redesign
  Reason: the pilot must stay proof-local.
- Recommendation or coverage semantic changes
  Reason: M27 and M27.5 stay closed.
- A global npm workspace
  Reason: each bucket must stay self-contained to keep blast radius small.
- npm publishing or a user-facing TypeScript package
  Reason: M29 proves internal truthfulness only.
- Wrapper-family or second-family pilot expansion
  Reason: one leaf family is the smallest honest test.
- A hidden `spec_version` sentinel for target selection
  Reason: the authored body surface should be explicit, not encoded in version text.

## What Already Exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Shared promotion loop | `xtask/src/family/harness.rs`, `prove.rs`, `certify.rs`, `report.rs` | Reuse. Extend existing family commands, do not invent a second workflow. |
| Promoted pilot packet | `semantic-families/function.arithmetic_leaf.monotone_up.v1/` | Reuse. Keep the same packet root. |
| Shared backend-execution boundary | `spec-core/src/backend_execution.rs` | Reuse. M29 tests whether it actually held. |
| Rust semantic evaluator shape | `spec-core/src/semantic_review.rs` | Reuse structure. Add one TypeScript wedge for the same family. |
| Rust scaffold and layout discipline | `xtask/src/family/layout.rs`, `scaffold.rs`, `smoke.rs` | Reuse. Add a parallel TypeScript target root under the same packet. |
| CI baseline | `.github/workflows/ci.yml` | Reuse. Extend with one packet-local TypeScript lane. |

## Step 0 - Scope Challenge

### Premises

1. M28 already paid for the shared-core extraction.
   Verdict: **accept**
2. The current block is repairable inside the closed M29 file surface.
   Verdict: **accept**
3. The correct recovery is to repair the foundation, not to let `Lane C` absorb the mismatch.
   Verdict: **accept**
4. The milestone name stays `M29`.
   Verdict: **accept**

### Minimum change that still counts

The minimum honest M29 recovery is:

1. make `spec-core` understand `body.typescript` directly
2. make `xtask` keep all TypeScript packet truth under the existing packet root
3. freeze that repaired foundation before any packet edits resume
4. finish the original packet, CI, and proof sequence

Anything smaller is fake confidence.

### Complexity check

This plan touches more than 8 files, which is usually a smell.

It is still justified because the blast radius is the smallest complete one:

- `spec-core` target body contract
- `xtask` target-aware family plumbing
- one promoted packet
- one CI lane

No new subsystems are introduced.

### Search-before-building decisions

- **[Layer 1]** Keep the existing `family smoke/prove/certify` command surface.
- **[Layer 1]** Keep Rust as the omitted-flag default.
- **[Layer 1]** Keep the TypeScript runtime self-contained per bucket.
- **[EUREKA]** The real leak is not syntax emission. It is whether one packet and one proof model stay coherent once Cargo is no longer the only execution substrate.

### Completeness check

The shortcut would be "repair foundation drift and run a manual TypeScript demo."

That is not enough.

The complete version is still cheap enough to do now:

- repaired foundation
- checked-in packet truth
- automated smoke/prove/certify
- CI branch gate
- explicit closeout verdict

### Distribution check

M29 introduces no new user-facing artifact type.

Internal delivery surfaces only:

- committed TypeScript packet scaffold
- target-partitioned proof artifacts
- CI job
- closeout verdict

## Closed Implementation Surface

Only these surfaces may change for M29:

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

Everything else is read-only.

If honest completion requires widening into `spec status`, `spec export`, passport
freshness, or non-function lowering, stop M29 and split the follow-on.

## Locked Recovery Decisions

These are now fixed.

1. **Packet root**
   TypeScript packet truth stays under
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`.
   `semantic-families-typescript/` is forbidden.

2. **Authored body surface**
   `kind:function` uses additive bodies:

   ```yaml
   body:
     rust: |
       { ... }
     typescript: |
       { ... }
   ```

   `body.typescript` is allowed only on `kind:function`.

3. **Target selection**
   Rust-default paths continue to consume `body.rust`.
   TypeScript pilot paths consume `body.typescript`.
   No `spec_version` sentinel is allowed to stand in for target selection.

4. **Integration recovery**
   `d10679a` is preserved as the blocked checkpoint in run-state history.
   Active integration restarts from `741a83e`.

5. **Lane names**
   `Lane A`, `Lane B`, `Lane C`, and `Lane D` keep their current names and ownership model.
   Recovery happens inside the same orchestration topology, not under a renamed milestone.

## Architecture

### System shape

```text
                 +-----------------------------+
                 |  Authored packet truth      |
                 |  semantic-families/...      |
                 +--------------+--------------+
                                |
                                v
                  +-------------+-------------+
                  | shared spec surface       |
                  | body.rust + body.typescript|
                  +------+------+-------------+
                         |      |
             rust lane   |      | typescript lane
                         |      |
                         v      v
                 +-------+------+--------+
                 | spec-core              |
                 | validate / lower /     |
                 | semantic review        |
                 +-------+------+--------+
                         |      |
                         v      v
                 +-------+------+--------+
                 | xtask family commands  |
                 | smoke / prove / certify|
                 +-------+------+--------+
                         |      |
                         |      +--> target-partitioned artifacts
                         |
                         +--> committed packet truth checks
```

### Recovery execution flow

```text
741a83e
  |
  +--> Lane A: spec-core body contract repair
  |
  +--> Lane B: xtask packet-root and target plumbing repair
          |
          v
   parent merge into fresh ws/m29-int
          |
          +--> freeze packet contract
                  |
                  v
              Lane C: packet runtime + fixture truth
                  |
                  v
           parent merge + freeze ci contract
                  |
                  v
              Lane D: ci pilot lane
                  |
                  v
         final local proof + push + ci observation
```

### Body-selection contract

```text
LoadedSpec.body
   |
   +--> rust caller
   |      |
   |      +--> validate body.rust
   |      +--> lower rust module
   |      +--> run existing rust semantic family logic
   |
   +--> typescript caller
          |
          +--> require body.typescript for the pilot packet
          +--> validate TypeScript block rules
          +--> lower TypeScript module
          +--> run monotone_up TypeScript semantic family logic
```

## Command Contract

The public command surface remains:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript

cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript

cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Rules:

- omitted flag means `rust`
- valid flag values are only `rust` and `typescript`
- `family new` stays Rust-default and unchanged
- M29 does not add `spec build/test --target-language typescript`

## Packet Contract

The pilot packet stays at:

```text
semantic-families/function.arithmetic_leaf.monotone_up.v1/
  candidate.md
  family.toml
  fixtures/
    aligned/
      Cargo.toml
      units/
    drift/
      Cargo.toml
      units/
    under_specified/
      Cargo.toml
      units/
    unsupported_near_miss/
      Cargo.toml
      units/
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

Rules:

- Rust fixtures stay where they are today.
- TypeScript runtime files live only under `targets/typescript/fixtures/<bucket>/`.
- Shared authored unit truth stays under the existing `fixtures/<bucket>/units/**`.
- Proof commands may materialize ephemeral generated output under temp directories, but they must not dirty the committed packet tree.

## Runtime Contract

M29 locks the TypeScript runtime shape to:

- Node package manager: `npm`
- numeric library: `decimal.js`
- install mode: `npm ci`
- generated TypeScript function modules always import `Decimal` from `decimal.js`
- dep imports in generated TypeScript output derive only from shared `deps`

If the pilot needs arbitrary external imports, a global npm workspace, or a general
import registry, stop M29 and split the follow-on.

## Workstreams

### Workstream 0 - Parent re-freeze

Owner: parent only

Deliverables:

- reset active integration seed to `741a83e`
- preserve `d10679a` in run-state history as the blocked checkpoint
- refresh `foundation-freeze.json`
- restate forbidden surfaces and lane ownership

Done when:

- the relaunched foundation branches fork from `741a83e`
- the old blocked merge is preserved only as evidence, not as the active packet base

### Workstream 1 - Lane A, shared body contract in `spec-core`

Touches:

- `spec-core/src/types.rs`
- `spec-core/src/validator.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/semantic_review.rs`

Required changes:

1. `Body` gains additive `typescript`.
2. The normalized function IR carries `body_typescript`.
3. Rust-default validation remains unchanged.
4. TypeScript pilot validation reads `body.typescript`.
5. `body.typescript` rejects `import` and `export`.
6. TypeScript lowering stays limited to the pilot family.
7. TypeScript semantic review stays limited to `function.arithmetic_leaf.monotone_up.v1`.

Acceptance commands:

```bash
cargo test -p spec-core --lib body_typescript_ -- --color never
cargo test -p spec-core --lib validator_typescript_ -- --color never
cargo test -p spec-core --lib generator_typescript_ -- --color never
cargo test -p spec-core --lib monotone_up_typescript_ -- --color never
cargo test -p spec-core --lib semantic_review_typescript_ -- --color never
cargo test -p spec-core --lib -- --color never
```

Stop conditions:

- if Rust-default validation changes for non-pilot callers
- if TypeScript support requires seam kinds
- if target selection still depends on hidden metadata instead of explicit body selection

### Workstream 2 - Lane B, target-aware family plumbing in `xtask`

Touches:

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

Required changes:

1. Add `--target-language rust|typescript` to `family smoke/prove/certify`.
2. Keep omitted flag behavior as Rust.
3. Keep packet roots under `semantic-families/`.
4. Partition TypeScript proof artifacts under target-aware artifact paths.
5. Validate committed TypeScript runtime layout under `targets/typescript/fixtures/<bucket>/`.
6. Make scaffold truth compare against the locked packet root, not a parallel tree.
7. Ensure `smoke --target-language typescript` proves committed scaffold truth.
8. Ensure `prove` and `certify` write TypeScript target markers into artifacts and reports.

Acceptance commands:

```bash
cargo test -p xtask target_language_ -- --color never
cargo test -p xtask typescript_layout_ -- --color never
cargo test -p xtask scaffold_typescript_ -- --color never
cargo test -p xtask smoke_typescript_ -- --color never
cargo test -p xtask prove_typescript_ -- --color never
cargo test -p xtask certify_typescript_ -- --color never
cargo test -p xtask artifact_path_ -- --color never
cargo test -p xtask report_target_language_ -- --color never
cargo test -p xtask -- --color never
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
```

Stop conditions:

- if any codepath writes packet truth outside `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`
- if the command surface needs more than one explicit `--target-language` flag
- if artifact partitioning breaks Rust path stability

### Workstream 3 - Parent merge and packet contract freeze

Owner: parent only

Required outcomes:

1. merge repaired `Lane A` and `Lane B` into a fresh `ws/m29-int`
2. rerun all foundation acceptance commands
3. write `packet-contract-freeze.json`
4. unblock `task/m29-c1-freeze-packet-contract`

Packet freeze must record:

- exact integration SHA
- frozen packet root summary
- frozen `body.typescript` contract summary
- frozen generated-module import shape summary
- exact acceptance commands for `Lane C`

### Workstream 4 - Lane C, committed packet truth

Touches:

- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`

Required changes:

1. Add `body.typescript` to the packet's shared bucket unit specs.
2. Add committed TypeScript runtime roots for all four buckets under
   `targets/typescript/fixtures/`.
3. Check in `package.json`, `package-lock.json`, `tsconfig.json`, `src/main.ts`,
   and `tests/pilot.test.ts` per bucket.
4. Keep packet-local tests aligned with the semantic intent of each bucket:
   aligned, drift, under_specified, unsupported_near_miss.

Acceptance commands:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
rg --files semantic-families/function.arithmetic_leaf.monotone_up.v1/targets/typescript/fixtures
rg -n "typescript:" semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures
```

Stop conditions:

- if `Lane C` needs to edit `spec-core` or `xtask`
- if committed packet truth still expects a parallel `semantic-families-typescript/` root
- if bucket runtime files require mutable local state instead of checked-in lockfiles

### Workstream 5 - Parent merge and CI freeze

Owner: parent only

Required outcomes:

1. merge `Lane C`
2. rerun packet-local acceptance
3. write `ci-freeze.json`
4. freeze exact workflow commands for `Lane D`

### Workstream 6 - Lane D, CI pilot lane

Touches:

- `.github/workflows/ci.yml`

Required changes:

1. Add Node setup for the TypeScript pilot lane.
2. Run packet-local TypeScript smoke/prove/certify on the promoted packet.
3. Preserve the existing Rust lane unchanged.
4. Use checked-in lockfiles and `npm ci` only.

Acceptance commands:

```bash
cargo test -p spec-core --lib body_typescript_ -- --color never
cargo test -p xtask target_language_ -- --color never
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Stop conditions:

- if CI requires hidden mutable caches or local-only setup
- if the TypeScript pilot cannot run as an automatic branch gate

### Workstream 7 - Final proof and closeout

Owner: parent only

Required commands:

```bash
cargo test -p spec-core --lib -- --color never
cargo test -p xtask -- --color never
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

Closeout must answer:

1. what stayed truly shared
2. what still leaked target-specific assumptions
3. whether the verdict is `EXPAND`, `NARROW`, or `STOP`

## Test Diagram

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/types.rs
    |
    ├── [REQ] Body accepts additive body.typescript
    ├── [REQ] Rust callers remain body.rust-only by default
    └── [REQ] normalized function IR carries body_typescript

[+] spec-core/src/validator.rs
    |
    ├── [REQ] TypeScript path reads body.typescript, not body.rust
    ├── [REQ] block syntax required for body.typescript
    ├── [REQ] import/export statements rejected
    ├── [REQ] control-flow near miss stays unsupported
    └── [REQ] Rust validation behavior unchanged

[+] spec-core/src/generator.rs
    |
    ├── [REQ] TypeScript lowering emits Decimal import
    ├── [REQ] dep-relative imports resolve correctly
    ├── [REQ] pilot family lowers under TypeScript path
    └── [REQ] Rust lane rejects accidental TypeScript-only units

[+] spec-core/src/semantic_review.rs
    |
    ├── [REQ] aligned fixture stays supported and aligned
    ├── [REQ] drift fixture reports semantic drift
    ├── [REQ] under_specified fixture reports under-specification
    ├── [REQ] control-flow near miss stays unsupported
    └── [REQ] citations point at the TypeScript pilot body

[+] xtask target-language routing
    |
    ├── [REQ] omitted flag means rust
    ├── [REQ] explicit typescript accepted only for pilot family
    └── [REQ] non-pilot family rejects typescript cleanly

[+] xtask layout / scaffold / smoke
    |
    ├── [REQ] packet root stays under semantic-families/
    ├── [REQ] Rust layout still passes unchanged
    ├── [REQ] TypeScript layout requires package.json + lockfile + tsconfig + src/main.ts
    ├── [REQ] scaffold writes starter specs with body.typescript inside packet root
    └── [REQ] smoke exact-match gate fails on root drift

[+] xtask prove / certify / report
    |
    ├── [REQ] prove writes target-partitioned report
    ├── [REQ] certify writes target-partitioned attempt + certification artifacts
    ├── [REQ] report records target_language=typescript
    └── [REQ] Rust artifact paths unchanged

[+] packet-local TypeScript runtime
    |
    ├── [REQ] npm ci runs from checked-in lockfile
    ├── [REQ] npm run build passes for each bucket
    ├── [REQ] npm test passes for each bucket
    └── [REQ] prove/certify do not dirty committed packet truth

[+] CI operator flow
    |
    ├── [REQ] branch CI runs the TypeScript pilot automatically
    ├── [REQ] Rust lane still runs unchanged
    └── [REQ] failure output points at the packet lane, not a generic CI crash
```

### Regression rule

These regressions are mandatory to lock:

1. A regression test must prove the TypeScript packet root never leaves
   `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`.
2. A regression test must prove the TypeScript validator reads `body.typescript`,
   not `body.rust`.
3. A regression test must prove Rust-default callers remain unchanged when
   `body.typescript` is absent.

## Failure Modes Registry

| Codepath | Realistic failure | Test coverage required | Error handling required | Operator impact |
|---|---|---|---|---|
| Body selection | TypeScript lane silently reads `body.rust` | spec-core validator and generator regression tests | explicit validation failure or correct field selection | fake green proof |
| Packet root | scaffold or smoke uses `semantic-families-typescript/` | xtask root-path regression tests | invalid-input failure before packet freeze | packet truth split across two trees |
| TS lowering | generated dep imports use wrong relative path | packet-local build tests | generation failure before npm test | misleading scaffold debugging |
| Semantic review | aligned TypeScript fixture classified from Rust body | semantic review fixture tests | evaluator citation must point at TS body | false certification |
| Packet runtime | missing `package-lock.json` passes local smoke | layout and smoke tests | fail fast before prove | late CI failure |
| Artifact partition | TS proof overwrites Rust artifacts | xtask artifact-path tests | target-aware path partitioning | stale or corrupted proof history |
| CI lane | job depends on hidden mutable npm state | CI job proof with `npm ci` only | deterministic workflow scripts | flaky pilot confidence |

Critical gap rule:

If any new M29 failure mode has no test, no explicit error handling, and would fail
silently, the milestone is blocked.

## Performance And Cost Guardrails

- No global npm workspace.
- No repo-wide TypeScript package install.
- No extra certify-semantic churn beyond the pilot packet.
- Keep Rust as the fast-path default for all existing commands.
- Keep TypeScript proof artifacts partitioned so Rust reads do not pay new cost.

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Parent re-freeze | `.runs/`, planning docs, worktree metadata | — |
| Lane A, spec-core body contract | `spec-core/src/` | Parent re-freeze |
| Lane B, xtask family plumbing | `xtask/src/`, `xtask/src/family/` | Parent re-freeze |
| Parent packet freeze | integration worktree, `.runs/` | Lane A, Lane B |
| Lane C, packet truth | `semantic-families/function.arithmetic_leaf.monotone_up.v1/` | Parent packet freeze |
| Parent ci freeze | integration worktree, `.runs/` | Lane C |
| Lane D, CI lane | `.github/workflows/` | Parent ci freeze |
| Final proof and closeout | integration worktree, `.runs/` | Lane D |

### Parallel lanes

- `Lane A`: shared body contract in `spec-core/`
- `Lane B`: target-aware family plumbing in `xtask/`
- `Lane C`: packet truth in `semantic-families/function.arithmetic_leaf.monotone_up.v1/`
- `Lane D`: CI lane in `.github/workflows/`

Execution:

- Launch `Lane A` + `Lane B` in parallel.
- Merge both into fresh integration.
- Freeze packet contract.
- Launch `Lane C` sequentially after the freeze.
- Merge `Lane C`.
- Freeze CI contract.
- Launch `Lane D`.
- Merge `Lane D`.
- Run final proof loop and closeout.

### Conflict flags

- `Lane A` and `Lane B` do not share module directories, so they can run in parallel.
- `Lane A` and `Lane B` are contract-coupled on body selection. The parent must re-freeze before `Lane C`.
- `Lane C` must not launch from the blocked integration SHA `d10679a`.
- `Lane D` must not launch before `Lane C` freezes the final packet layout and command contract.

## Exact Parent Acceptance Checklist

Parent may advance only when all are true:

1. `foundation-freeze.json` records `741a83e` as the active relaunch base.
2. `d10679a` is preserved only as blocked evidence.
3. `Lane A` and `Lane B` acceptance commands pass on fresh integration.
4. `packet-contract-freeze.json` names the existing packet root and additive `body.typescript` contract explicitly.
5. `Lane C` acceptance commands pass with no foundation-file edits.
6. `ci-freeze.json` records the exact CI commands for the TypeScript pilot.
7. Final rust + typescript smoke/prove/certify commands all pass.
8. Push and CI observation are captured in run-state artifacts.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | Scope | Keep milestone name `M29` | mechanical | explicit over clever | the milestone goal did not change | `M29A` rename |
| 2 | Architecture | Keep packet root under `semantic-families/...` | mechanical | DRY | one packet root preserves truthful packet ownership | parallel TypeScript packet tree |
| 3 | Architecture | Make `body.typescript` an explicit additive field | mechanical | explicit over clever | authored truth should live in the schema, not hidden metadata | `spec_version` target selector |
| 4 | Execution | Preserve `d10679a` as blocked evidence but restart active integration from `741a83e` | taste, resolved | pragmatic | keeps forensics without contaminating the active packet base | continuing from blocked merge |
| 5 | Parallelization | Run `Lane A` and `Lane B` in parallel, everything else sequentially | mechanical | bias toward action | this is the only safe parallel split with disjoint module ownership | launching `Lane C` early |
