<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260513-084347.md -->
# M55: Bounded Cross-Library TypeScript Helper Imports Plan

Status: **implementation plan**
Milestone: **M55**
Milestone family: **bounded-typescript-execution**
Implementation readiness: **ready for bounded execution**
Plan scope: **extend the existing Bun-backed TypeScript lane to allow bounded cross-library helper imports, while keeping direct wrapper and chain3 root deps local-only**
Base branch: **main**
Working branch: **feat/m40-plus**
Validated at commit: **`dd4008f`**
Last rewritten: **2026-05-13**

Supersedes:

- the prior M54 plan at this path
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260513-082845.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260513-082845.md`

Primary source artifacts:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260513-082845.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260513-082845.md`
- `TODOS.md`
- `README.md`
- `CHANGELOG.md`

Primary repo surfaces:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`
- `examples/crosslib-app/spec.toml`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/shared-spec/units/money/round.unit.spec`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

## Executive Summary

M55 closes one specific product mismatch.

The repo already supports cross-library units in the Rust path. `examples/crosslib-app` is real. `[libraries]` config already works. Cross-library validate, generate, export, and Rust test flows are already first-class.

The Bun-backed TypeScript lane is still artificially narrower. It supports:

- helper-aware monotone-up roots from M46
- same-tree wrapper roots from M52
- same-tree chain3 roots from M54

But it still rejects any `shared::...` helper dep before Bun runs. That makes the product story weird. The user can author a real cross-library function unit like `examples/crosslib-app/units/pricing/apply_tax.unit.spec`, the Rust path treats it as normal, and the TypeScript path says "local only" even when the only thing crossing libraries is a helper leaf already within the bounded lane.

M55 fixes that exact seam. Not generic TypeScript portability. Not generic cross-library execution. One bounded move:

> The Bun-backed TypeScript lane supports cross-library helper imports for already-supported bounded function families, while direct wrapper and chain3 root deps remain local-only.

If implementation starts sounding like "cross-library TypeScript support," the milestone drifted.

## Problem Statement

Today the TypeScript lane is family-shaped but still topology-fragile. The root-family admission logic already exists, and the generated same-tree closure logic already exists, but helper resolution assumes the direct helper dep must be local. That makes the maintained cross-library example truthfully portable in Rust and artificially non-portable in TypeScript.

This matters because the user journey is currently arbitrary:

1. author a real cross-library helper in a sibling spec library
2. prove it in Rust
3. switch to `--target-language typescript`
4. hit a pre-Bun rejection that is not about family support, only about helper locality

That feels fake because it is fake. M55 removes that fake limitation without widening the root contract.

## Step 0: Scope Challenge

### What already exists for each sub-problem

| Sub-problem | Existing surface | M55 action |
|---|---|---|
| Cross-library library loading | `examples/crosslib-app/spec.toml`, M9 library loading, many `shared::...` CLI tests in `spec-cli/tests/cli.rs` | reuse |
| TypeScript target root-family validation | `spec-core/src/validator.rs` | extend carefully |
| TypeScript bounded closure emission | `spec-core/src/typescript_backend.rs` | extend helper resolution only |
| Same-tree wrapper and chain3 TypeScript proofs | `spec-cli/tests/cli.rs` | preserve |
| Current cross-library TypeScript rejection wall | `TYPESCRIPT_CROSS_LIBRARY_HELPER_UNSUPPORTED_MESSAGE` and local-only dep parsing in `spec-core/src/validator.rs` | replace only for helper slots |
| Real maintained cross-library example | `examples/crosslib-app/` | reuse as the canonical green path |

### Minimum change set

The minimum honest implementation is:

1. allow cross-library resolution only for helper dep positions already permitted by the bounded lane
2. keep all direct root deps for wrapper and chain3 local-only
3. render correct relative TypeScript imports for sibling-library helper units
4. prove the real maintained monotone-up green path in `examples/crosslib-app`
5. prove wrapper and chain3 recursive helper reuse inside the bounded closure
6. add the full negative wall
7. update docs only after proof is green

Anything beyond that is scope creep.

### Complexity check

This plan touches more than 8 files if docs and fixtures are included, but the logic change is still bounded because it stays inside two existing code seams:

- validation contract in `spec-core/src/validator.rs`
- generation contract in `spec-core/src/typescript_backend.rs`

No new crates, services, commands, runtimes, or schema surfaces are allowed.

### Search check

No new framework or infrastructure pattern is being introduced. This is a contract-extension milestone inside the existing Rust + Bun lane. Search is not the bottleneck. The bottleneck is keeping the contract narrow and proving it with a real example.

### TODOS cross-reference

This plan executes the existing deferred item in `TODOS.md`:

- `Cross-library TypeScript helper imports`

It must not silently absorb these still-deferred items:

- `Generic multi-dependency TypeScript execution`
- direct cross-library wrapper deps
- direct cross-library chain3 deps

### Completeness check

The complete version is still bounded. The shortcut would be proving only a synthetic fixture and calling it done. That would save minutes and cost clarity. The complete version for M55 is:

- one real green path in `examples/crosslib-app`
- explicit negative proofs for every widening case
- exact docs language

That is the lake. Boil it.

### Distribution check

No new artifact type is introduced. Distribution remains the existing `spec` CLI via current cargo install and GitHub release paths.

## Current State

Observed on `feat/m40-plus` at `dd4008f`:

- `spec-core/src/validator.rs` still hard-rejects cross-library helper deps for the TypeScript target with M52 wording
- `spec-core/src/typescript_backend.rs` still parses helper deps as local-only
- `spec-cli/tests/cli.rs` already proves same-tree green paths for monotone-up, wrapper, and chain3
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec` already depends on `shared::money/round`
- `examples/shared-spec/units/money/round.unit.spec` already exists as the sibling helper truth

Known governance truth from prior work still stands:

- `recommendation_status = insufficient_real_corpus`
- `decision_action = stop`
- `required_next_action = record_stop_without_new_milestone`

M55 changes backend execution truth only. It does not reopen family-analysis governance.

## Exact Product Contract

### In Scope

- cross-library helper deps for `function.arithmetic_leaf.monotone_up.v1`
- cross-library helper deps reached from already-supported wrapper or chain3 closure members, if those helper deps occupy helper positions already allowed today
- correct TypeScript import rendering for sibling-library helper units
- actionable bounded errors for unresolved alias, missing helper unit, wrong helper family, and missing helper `body.typescript`
- at least one real green path through `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- preservation of additive, target-specific proof in passports and status output

### Not In Scope

- direct cross-library wrapper deps
- direct cross-library chain3 deps
- generic cross-library TypeScript execution
- generic multi-dependency TypeScript execution
- molecule TypeScript execution
- seam-kind TypeScript execution
- new schema fields
- new CLI flags or runtime channels
- family-analysis recommendation changes

## Locked Decisions

These are contract decisions, not suggestions:

1. Cross-library support is keyed by helper-slot position, not by "dep contains `::`".
2. Direct root deps for `function.wrapper.pipeline.v1` remain local-only.
3. Direct root deps for `function.wrapper.pipeline.chain3.v1` remain local-only.
4. Cross-library resolution is allowed only for helper leaves already permitted by the current bounded lane.
5. Helper units must still classify as `function.helper.identity_passthrough.v1`.
6. Helper units must still author non-empty `body.typescript`.
7. Generated TypeScript includes each required unit once and excludes unrelated loaded units.
8. Rust and TypeScript proof remain additive and target-specific.
9. Molecule tests remain Rust-only.
10. Public docs must say "cross-library helper imports in the bounded TypeScript lane," nothing broader.
11. The canonical user-facing proof path is `examples/crosslib-app/units/pricing/apply_tax.unit.spec`, not a temporary injected fixture.
12. Wrapper and chain3 recursive cross-library helper support is in scope for M55 and must be proven, but it does not require a second maintained example.
13. Focused fixtures may support bounded recursive proofs, but they do not replace the canonical real-example proof.

## Abort And Re-scope Triggers

Stop implementation and rewrite the plan if any of these become true:

1. supporting helper imports requires a generic graph executor instead of the bounded closure collector
2. direct wrapper or chain3 root deps must become cross-library for the green path to work
3. import rendering requires a second TypeScript-only library-resolution system instead of reusing loaded unit truth
4. passport or export schemas need new fields
5. molecule TypeScript execution becomes necessary to prove the feature
6. docs can only be made truthful by using broader wording than the actual proof wall
7. the only way to make the canonical example pass is to keep injecting temporary `body.typescript` during tests instead of committing truthful authored spec bodies

## Architecture

### Admission Flow

```text
CURRENT
  spec test --target-language typescript
    |
    v
  validator.rs
    |
    +-- monotone_up root
    |     `-- helper dep must be local
    |
    +-- wrapper root
    |     `-- direct deps must be local
    |
    `-- chain3 root
          `-- direct deps must be local

TARGET M55
  spec test --target-language typescript
    |
    v
  validator.rs
    |
    +-- monotone_up root
    |     `-- helper dep may be local or cross-library
    |           if alias resolves, unit loads, family matches, TS body exists
    |
    +-- wrapper root
    |     `-- direct deps stay local-only
    |           but nested helper leaves may be cross-library
    |
    `-- chain3 root
          `-- direct deps stay local-only
                but nested helper leaves may be cross-library
```

### Resolution And Emission Flow

```text
CLI: spec test --target-language typescript
  |
  +-- validator.rs
  |     +-- classify root family
  |     +-- validate root direct-dep tuple
  |     `-- resolve helper dep
  |            +-- local helper, existing path
  |            `-- shared::helper, new bounded path
  |
  +-- typescript_backend.rs
  |     +-- collect bounded closure
  |     +-- resolve helper module owner
  |     `-- render relative import path across sibling library boundary
  |
  `-- Bun
        +-- build generated tree
        `-- run local tests
```

### Import Rendering Rule

The backend must not special-case imports by string concatenation. It should resolve the loaded helper unit first, derive the emitted module path from the resolved unit id, then compute the relative import path from the importing unit module to the helper module. This keeps the TypeScript backend aligned with loaded-unit truth instead of making a second resolver.

## Write Scope

Expected write scope:

- `spec-core/src/validator.rs`
- `spec-core/src/typescript_backend.rs`
- `spec-cli/tests/cli.rs`
- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/shared-spec/units/money/round.unit.spec`
- `examples/crosslib-app/README.md`
- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Support-proof fixture write scope, only for bounded recursive wrapper and chain3 coverage:

- a small cross-library aligned fixture under `semantic-families/.../fixtures/`

This fixture support is additive only. It does not replace the canonical real-example proof path.

## Detailed Implementation Plan

### Phase 1: Validator Contract

Primary file: `spec-core/src/validator.rs`

1. Replace the helper-specific TypeScript rejection rule so helper deps may be local or cross-library when the helper slot is otherwise legal for the bounded lane.
2. Preserve local-only rejection for direct wrapper deps.
3. Preserve local-only rejection for direct chain3 deps.
4. Split failure classes so the error tells the user exactly what failed:
   - alias unresolved
   - helper unit missing
   - helper family wrong
   - helper `body.typescript` missing
5. Update the M52/M54 wording in TypeScript error constants where needed so the user-facing contract says M55 and says the narrow thing.
6. Land validator unit coverage for every positive and negative branch in the same phase.

Required implementation constraints:

- no generic "allow cross-library dep" switch
- no broadening of direct root dep validation
- no change to molecule target validation
- no change to non-TypeScript validation semantics

Phase 1 is done when:

- `validate_typescript_execution_target_spec_with_specs` accepts a legal shared helper dep in a helper slot
- direct wrapper and chain3 shared root deps still fail before any backend work
- each failure class has its own stable assertion in validator tests
- no remaining user-facing error string implies broad cross-library TypeScript support

### Phase 2: TypeScript Backend Closure And Import Rendering

Primary file: `spec-core/src/typescript_backend.rs`

1. Extend helper dep parsing to accept library-qualified helper ids in helper positions only.
2. Resolve helper specs from the loaded unit set, not from a TypeScript-only path guess.
3. Continue collecting only the bounded closure.
4. Render sibling-library helper imports with the correct relative path.
5. Keep output deterministic and de-duplicated.
6. Prove recursive wrapper and chain3 helper closure behavior here if the CLI layer would otherwise need large fixture orchestration just to exercise the backend contract.

Required implementation constraints:

- no inclusion of unrelated loaded units
- no nested chain3 closure support beyond the current bounded contract
- no second import resolver stack

Phase 2 is done when:

- backend tests prove correct relative imports across the sibling-library boundary
- backend tests prove helper units are emitted once and unrelated units stay out
- bounded recursive wrapper and chain3 helper closure behavior is covered either here or by Phase 3 CLI proofs, with no gap left ambiguous

### Phase 3: Proof Wall

Primary file: `spec-cli/tests/cli.rs`

Add or update these proofs:

1. real cross-library green path for `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
2. target-specific status row remains truthful after TypeScript proof
3. wrapper root still executes when any cross-library helper appears only inside an already-legal nested helper slot
4. chain3 root still executes when any cross-library helper appears only inside an already-legal nested helper slot
5. unresolved library alias fails before Bun
6. missing shared helper fails before Bun
7. wrong helper family fails before Bun
8. missing helper `body.typescript` fails before Bun
9. direct cross-library wrapper dep still fails before Bun
10. direct cross-library chain3 dep still fails before Bun

The negative wall is mandatory. Do not update docs before these are green.

Phase 3 is done when:

- the maintained cross-library example passes through Bun without test-only spec mutation
- wrapper and chain3 recursive helper reuse is proven somewhere concrete, not left as an implied backend property
- every widened negative case still dies before Bun starts build or test execution

### Phase 4: Authored Spec Truth

Primary files:

- `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
- `examples/shared-spec/units/money/round.unit.spec`

M55 assumes the real example path is chosen. Ensure the helper and root units carry truthful, minimal `body.typescript` that mirrors their existing semantic contract. Do not author decorative TypeScript just to make the example green. Keep the bodies as short as possible while remaining semantically honest.

If the only way to keep `examples/crosslib-app` green is to bloat the authored spec bodies or diverge from the Rust contract, stop and re-scope instead of hiding behind test-only mutation helpers.

### Phase 5: Docs And Backlog

Primary files:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`

Only after proof is green:

1. document the exact new claim
2. document the exact non-goals next to it
3. promote one canonical command for the real green path
4. keep deferred items explicit in `TODOS.md`

Phase 5 is done when:

- README, cross-library example docs, and CHANGELOG all use the same narrow M55 sentence
- the canonical command matches the maintained example that just passed
- `TODOS.md` still clearly defers direct cross-library roots and generic multi-dependency execution

## Code Quality Guardrails

- Reuse existing dep parsing and loaded-spec lookup shapes where possible. Do not grow a second parallel contract.
- Prefer one small helper for "resolve bounded helper dep" over repeated cross-library branching.
- Keep new constants explicit. Avoid clever generic messages that blur failure classes.
- If a comment or module header still says "bounded M52 TypeScript lane" in a way that is now materially inaccurate, update it as part of the same change.

## Test Review

100% coverage of new behavior is required. The implementation is small enough that there is no excuse for a partial wall.

### Code Path Coverage

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/validator.rs
    |
    ├── monotone-up root with local helper
    │   └── [PRESERVE] Existing supported path stays green
    |
    ├── monotone-up root with shared helper
    │   └── [ADD] Green path if alias resolves, unit exists, family matches, TS body exists
    |
    ├── wrapper root with direct shared dep
    │   └── [ADD] Reject before Bun
    |
    ├── chain3 root with direct shared dep
    │   └── [ADD] Reject before Bun
    |
    └── helper failure branches
        ├── [ADD] unresolved alias
        ├── [ADD] missing helper unit
        ├── [ADD] wrong helper family
        └── [ADD] missing helper body.typescript

[+] spec-core/src/typescript_backend.rs
    |
    ├── helper dep resolution
    │   ├── [ADD] local helper still resolves
    │   └── [ADD] shared helper resolves to sibling library unit
    |
    ├── import rendering
    │   └── [ADD] relative import path is correct across sibling library boundary
    |
    └── bounded closure collection
        └── [ADD] include helper once, exclude unrelated units
```

### User Flow Coverage

```text
USER FLOW COVERAGE
===========================
[+] Real cross-library example
    |
    ├── [ADD] spec test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
    ├── [ADD] passport stores additive TypeScript proof
    └── [ADD] spec status --target-language typescript reports valid

[+] Recursive bounded closure reuse
    |
    ├── [ADD] wrapper root stays green when its nested helper leaf resolves cross-library
    └── [ADD] chain3 root stays green when its nested helper leaf resolves cross-library

[+] Failure UX
    |
    ├── [ADD] alias error points at [libraries] config
    ├── [ADD] missing helper error names the qualified unit id
    ├── [ADD] wrong-family error names the resolved compatibility key
    └── [ADD] missing TypeScript body error identifies the helper unit
```

### Required Tests

#### `spec-core/src/validator.rs`

- accept cross-library helper dep in a legal helper slot
- reject unresolved library alias
- reject missing loaded helper unit
- reject wrong helper family
- reject missing helper `body.typescript`
- reject direct cross-library wrapper dep
- reject direct cross-library chain3 dep

#### `spec-core/src/typescript_backend.rs`

- render correct sibling-library import path
- collect bounded cross-library helper closure without unrelated unit leakage
- keep emission deterministic when helper ids are library-qualified

#### `spec-cli/tests/cli.rs`

- real example green path through Bun
- TypeScript status row uses the TypeScript proof
- bounded wrapper recursive helper green path
- bounded chain3 recursive helper green path
- negative cases fail before Bun:
  - unresolved alias
  - missing shared helper
  - wrong helper family
  - missing helper `body.typescript`
  - direct cross-library wrapper dep
  - direct cross-library chain3 dep

### Exact Commands

Focused unit tests:

```bash
cargo test -p spec-core typescript_target
cargo test -p spec-core typescript_tree
```

Focused CLI integration:

```bash
cargo test -p spec-cli typescript
```

Canonical green-path proof:

```bash
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript
```

Target-specific status proof:

```bash
cargo run -p spec-cli -- status examples/crosslib-app --target-language typescript --format json
```

Recursive bounded-closure proof:

- if the repo already has a truthful maintained wrapper or chain3 example that exercises a shared helper leaf, use it
- otherwise add the smallest focused fixture that proves wrapper and chain3 recursive helper reuse without changing the public M55 product story

## Failure Modes

| Failure mode | Test covers it | Error handling exists | User sees clear error | Severity | Required action |
|---|---|---|---|---|---|
| Direct cross-library wrapper dep accidentally passes | yes, add CLI + validator negative | yes | yes | High | preserve local-only direct-dep wall |
| Direct cross-library chain3 dep accidentally passes | yes, add CLI + validator negative | yes | yes | High | preserve local-only direct-dep wall |
| Shared helper alias is unresolved | yes, add CLI + validator negative | yes | must be explicit | Medium | point at `[libraries]` alias |
| Shared helper unit is absent from loaded sibling library | yes, add CLI + validator negative | yes | must name the unit id | Medium | name the missing qualified unit |
| Shared helper resolves to wrong family | yes, add CLI + validator negative | yes | must name the resolved family | Medium | print compatibility key |
| Shared helper lacks `body.typescript` | yes, add CLI + validator negative | yes | must identify the helper unit | High | fail before Bun |
| Import rendering points at wrong relative path | yes, add backend + CLI green-path proof | partial today | not necessarily | High | treat backend import tests as mandatory |
| Unrelated loaded units leak into generated tree | yes, add backend tree assertion | partial today | no | Medium | keep bounded closure collector strict |

Critical gap rule:

- Any failure mode that has no test, no explicit error handling, and would fail silently blocks completion.

M55 should ship with zero critical gaps.

## Performance And Operational Review

Performance is not the main risk here. Complexity drift is.

Still, hold these lines:

- do not add generic graph traversal where bounded helper recursion already works
- do not scan more loaded units than needed for the bounded closure
- do not recompute import ancestry with a second custom resolver if module paths already encode what is needed

Operationally, the biggest risk is debugging generated import paths after validation passed. That is why backend import rendering tests are first-class, not "nice to have."

## What Already Exists

| Existing code or flow | Reuse or rebuild |
|---|---|
| M9 cross-library library loading and alias semantics | reuse |
| M46 helper-aware monotone-up TypeScript lane | reuse and extend |
| M52 wrapper TypeScript lane | reuse and preserve direct local deps |
| M54 chain3 TypeScript lane | reuse and preserve direct local deps |
| `examples/crosslib-app` as maintained product example | reuse as the canonical user-facing green path |
| existing CLI TypeScript proof wall in `spec-cli/tests/cli.rs` | reuse and expand |

## NOT In Scope

- proving cross-library direct wrapper roots
- proving cross-library direct chain3 roots
- supporting arbitrary multi-dependency TypeScript targets
- broadening target-language validation to new spec kinds
- adding TypeScript molecule execution
- redesigning export or passport schema
- reopening semantic-family promotion or corpus strategy

## Worktree Parallelization Strategy

This plan does have parallelization opportunities.

### Dependency Table

| Step | Modules touched | Depends on |
|---|---|---|
| 1. Validator contract and unit tests | `spec-core/src/` validation surfaces | — |
| 2. Backend helper resolution and import rendering | `spec-core/src/` backend surfaces | 1 contract freeze |
| 3. CLI proof wall plus maintained example authoring | `spec-cli/tests/`, `examples/crosslib-app/`, `examples/shared-spec/` | 1 contract freeze |
| 4. Docs and backlog updates | repo docs surfaces | 2 + 3 green proof |

### Parallel Lanes

Lane A: Step 1 -> Step 2  
Sequential. One owner. Shared `spec-core/src/` write surface, same contract seam.

Lane B: Step 3  
Can start after Step 1 freezes the contract wording and negative cases. This lane owns `spec-cli/tests/` plus the maintained example specs and can scaffold the proof wall in parallel with Lane A Step 2, but it cannot claim green until Lane A finishes.

Lane C: Step 4  
Strictly last. Docs that land before proof will lie.

### Execution Order

1. Launch Lane A first.
2. Once Step 1 is stable, launch Lane B in a second worktree while Lane A continues through backend import rendering.
3. Merge Lane A backend completion and Lane B proof wall together.
4. Run the full focused command set.
5. Launch Lane C only after the canonical real green path, the recursive bounded-closure proofs, and the negative wall are all passing.

### Conflict Flags

- Lane A Step 1 and Step 2 both touch `spec-core/src/`. Keep them sequential in one worktree or under one owner.
- Lane B owns `spec-cli/tests/cli.rs`. Do not split that file across workers. It is already huge and easy to conflict.
- Lane B also owns the maintained example spec bodies. Lane C must not edit docs until those files and commands are stable.
- If the CLI tests need helper utilities added in `spec-cli/tests/cli.rs`, keep one owner for that file. It is a large file and conflict-prone.

## Documentation And DX Deliverables

The DX outcome is simple:

- one canonical cross-library TypeScript command that works
- exact explanation of what works
- equally exact explanation of what still does not

Required doc updates after proof:

1. README note for bounded cross-library helper imports
2. `examples/crosslib-app/README.md` points at the canonical command
3. CHANGELOG entry with the narrow product sentence
4. TODOS keep direct cross-library root deps and generic multi-dep execution deferred

Target TTHW for this wedge:

- current: ~15 minutes because there is no maintained passing example
- target after M55: ~5 minutes with one copy-paste command

## Acceptance Criteria

M55 is done when all of these are true:

1. `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec --target-language typescript` passes.
2. The TypeScript lane accepts cross-library helper deps only in legal helper positions.
3. Wrapper and chain3 bounded recursive helper closure support is explicitly proven.
4. Direct cross-library wrapper deps still reject before Bun.
5. Direct cross-library chain3 deps still reject before Bun.
6. Unresolved alias, missing helper unit, wrong helper family, and missing helper `body.typescript` all fail with actionable bounded errors before Bun runs.
7. Generated TypeScript imports the sibling helper module correctly.
8. Status and passport proof remain additive and target-specific.
9. README, `examples/crosslib-app/README.md`, and CHANGELOG describe exactly the landed boundary and no broader claim.

## Completion Summary

- Step 0: Scope Challenge — accepted as a bounded execution wedge
- Architecture — validator contract plus backend import rendering are the only real logic seams
- Code Quality — reuse existing loaded-unit truth, no second resolver, no generic dep switch
- Test Review — full coverage required for canonical green path, recursive bounded-closure proofs, and negative wall
- Performance Review — no meaningful runtime risk, but real complexity drift risk
- NOT in scope — written
- What already exists — written
- Failure modes — explicit, zero critical gaps allowed
- Parallelization — 3 lanes, with Lane A sequential and Lane B parallel after contract freeze
- Distribution — unchanged existing CLI distribution only

## Immediate Next Action

Implement Phase 1 first and keep it in one owner's worktree. As soon as the validator contract and unit tests are frozen, start the CLI/example proof lane in parallel while the same owner finishes backend import rendering.
