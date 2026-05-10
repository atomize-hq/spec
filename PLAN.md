<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260509-211536.md -->
# M43 - Promote `function.helper.identity_passthrough.v1`

Status: **authority plan**  
Milestone family: **rust-family-promotion**  
Implementation readiness: **ready-now**  
Next artifact kind: **authority_plan**  
Autoplan ready: **yes**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Last rewritten: **2026-05-09**  
Source design doc: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260509-211536.md`**  
Source test plan: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260509-212037.md`**  
Supersedes: **M42 Decision-Contract Verifier Stop-State Parity**

## Executive Verdict

M43 is the narrow Rust-family promotion wedge the branch has now earned.

The repo already supports `function.helper.identity_passthrough.v1` at runtime. The missing capability is not semantic-review understanding. The missing capability is that maintainers still cannot promote, smoke, prove, or certify that family as first-class packet truth.

This plan promotes exactly that family. No corpus-policy reopening. No shared-core portability follow-on. No second-language backend expansion.

## Problem Statement

The helper route for `money/round` has crossed the substrate threshold but not the promotion threshold.

Current repo truth:

- `spec-core/src/semantic_review.rs` routes helper-shaped unary Decimal functions to `function.helper.identity_passthrough.v1`.
- `cargo xtask family inventory --format json` reports that family as runtime-supported and supported-unpromoted.
- `cargo xtask family coverage --format json` reports `supported_unpromoted_family_units = 3`.
- There is no registered `FamilyHarness` entry for `function.helper.identity_passthrough.v1`.
- There is no `semantic-families/function.helper.identity_passthrough.v1/` packet.

That means the repo can describe the helper family honestly, but it cannot ship or maintain it through the same packetized proof workflow used for the promoted Rust families.

## Repo Truth Basis

### What already exists

| Sub-problem | Existing owner | M43 decision |
|---|---|---|
| Runtime helper-family routing | `spec-core/src/semantic_review.rs` | reuse directly, do not widen the classifier |
| Supported-unpromoted inventory truth | `xtask/src/family/inventory.rs` | reuse as the before/after promotion oracle |
| Coverage accounting | `xtask/src/family/coverage.rs` | reuse as the before/after coverage oracle |
| Existing promotion workflow | `xtask/src/family/harness.rs`, `scaffold.rs`, `smoke.rs`, `prove.rs`, `certify.rs` | reuse directly |
| Canonical helper seeds | `examples/ecommerce/units/money/round.unit.spec`, `examples/shared-spec/units/money/round.unit.spec` | use as the narrow wedge inputs |
| Helper regression truth | helper-family classifier tests in `spec-core/src/semantic_review.rs` | reuse as the semantic proof floor |

### Live signals that justify this wedge

- `cargo xtask family inventory --format json` reports `function.helper.identity_passthrough.v1` as the only supported-unpromoted runtime family.
- `cargo xtask family coverage --format json` reports the three helper units inside `supported_unpromoted_family_units`.
- The latest checkpoint says semantic review is core product, family-analysis is servant work, and the next bounded Rust wedge should reuse the M26-style promotion loop instead of more corpus churn.
- The current `recommend` / `corpus-decision` stop-state means there is no new unsupported-family discovery task to do first.

## Scope Challenge

### 0A. Premise challenge

1. The right problem is not “make recommendation smarter again.” The right problem is “convert already-supported helper truth into promoted packet truth.”
2. Doing nothing leaves one real runtime capability stuck outside the repo’s maintained proof workflow.
3. A broader helper-substrate milestone would solve a larger but less immediate problem and would delay the cleaner lake that is already visible.

### 0B. Existing code leverage

This milestone should reuse almost everything:

- `spec-core` already owns the classifier and regression tests.
- `xtask` already owns packet registry, scaffold, smoke, prove, and certify.
- existing promoted families already show the packet directory shape, manifest contract, prove/certify artifact flow, and smoke invariants.

The only missing layer is the helper-family-specific harness plus packet assets.

### 0C. Dream state mapping

```text
CURRENT STATE
  Runtime helper route exists.
  Inventory says one family is supported but unpromoted.
  Maintainers cannot prove/certify it as packet truth.

THIS PLAN
  Adds one helper-family harness and one helper-family packet.
  Runs the normal smoke/prove/certify workflow.
  Removes the helper family from supported-unpromoted inventory.

12-MONTH IDEAL
  Every runtime-supported Rust family is either intentionally unpromoted for a named reason
  or has a packetized proof path. No orphaned supported families.
```

### 0C-bis. Implementation alternatives

#### Approach A: Keep helper support substrate-only

Summary: leave the helper route in `spec-core` and do no packet work.  
Effort: S  
Risk: High  
Pros:
- no code churn in `xtask`
- no packet maintenance cost
Cons:
- strands real product truth outside the promotion workflow
- keeps inventory and coverage in a knowingly incomplete state
Reuses:
- all existing code, but solves nothing

#### Approach B: Promote the helper family through the existing Rust-family workflow

Summary: add one harness, one scaffold template, one packet, and the proof wiring needed to promote `function.helper.identity_passthrough.v1`.  
Effort: M  
Risk: Medium  
Pros:
- directly ships the missing product truth
- reuses all current M26 workflow machinery
- removes the only supported-unpromoted family from inventory
Cons:
- requires careful fixture semantics because the family allows both passthrough and round-like aligned behavior
Reuses:
- `harness.rs`, `scaffold.rs`, `smoke.rs`, `prove.rs`, `certify.rs`, helper classifier tests

#### Approach C: Broaden helper substrate before promotion

Summary: widen the helper-family semantic subset first, then promote a larger helper packet later.  
Effort: L  
Risk: High  
Pros:
- may cover more future helper shapes
Cons:
- larger blast radius
- delays the bounded ship-ready wedge
- reopens substrate work before shipping existing truth
Reuses:
- classifier code, but requires new semantic-design work

**RECOMMENDATION:** Choose Approach B because it ships the full lake already visible in repo truth without widening into a new research milestone.

### 0D. Mode-specific analysis

This is a feature enhancement on existing product-core machinery, so the correct review mode is **SELECTIVE EXPANSION** with a strong bias against any expansion that escapes the helper-family lake.

Possible expansions scanned and rejected for M43:

- add second-language helper-family proof
- generalize helper-family routing beyond the current honest subset
- rewrite inventory/recommendation policy to foreground supported-unpromoted families
- fold in shared-core portability cleanup

All are real follow-ons. None belong in M43.

### 0E. Temporal interrogation

```text
HOUR 1 (foundations):
  Register the family cleanly. Decide the helper packet contract before touching fixtures.

HOUR 2-3 (core logic):
  Add scaffold support and committed packet fixtures. Lock what counts as aligned, drift,
  under-specified, and unsupported-near-miss for this family.

HOUR 4-5 (integration):
  Thread the new family through smoke, prove, certify, inventory, and coverage expectations.

HOUR 6+ (polish/tests):
  Re-run the full xtask proof wall, confirm inventory/coverage deltas, and verify the family
  is no longer reported as supported-unpromoted.
```

## Accepted Scope

M43 is complete only if all of this lands together:

1. Add `function.helper.identity_passthrough.v1` to the family harness registry.
2. Add helper-family scaffold support so `cargo xtask family new function.helper.identity_passthrough.v1` can generate shape-honest starter packet content.
3. Commit a self-contained helper-family packet under `semantic-families/function.helper.identity_passthrough.v1/`.
4. Add or refresh smoke/prove/certify coverage for the new family.
5. Update regression tests so inventory and coverage truth reflects the promoted helper family.
6. Prove the before/after inventory and coverage state with live commands.

## Not In Scope

- widening helper-family semantic support beyond the current honest subset
- second-language packet proof for the helper family
- changing `recommend` or `corpus-decision` policy to prioritize supported-unpromoted families
- shared-core portability work
- generic multi-family registry refactors
- docs/changelog cleanup outside files forced by the promotion surface

## Architecture Contract

### Current to target

```text
CURRENT
  spec-core helper classifier -> runtime-supported helper family
  inventory/coverage -> supported_unpromoted helper truth
  xtask family workflow -> no registered helper family

TARGET
  spec-core helper classifier -> same runtime-supported helper family
  semantic-families/function.helper.identity_passthrough.v1 -> committed packet truth
  xtask family workflow -> helper family fully smoke/prove/certify capable
  inventory/coverage -> helper family counts as promoted, not supported-unpromoted
```

### Dependency graph

```text
spec-core/src/semantic_review.rs
        |
        v
xtask/src/family/harness.rs
        |
        +--> xtask/src/family/scaffold.rs
        +--> xtask/src/family/smoke.rs
        +--> xtask/src/family/prove.rs
        +--> xtask/src/family/certify.rs
        |
        v
semantic-families/function.helper.identity_passthrough.v1/**
        |
        v
inventory / coverage / certification artifacts
```

### File surface

Required production surfaces:

- `xtask/src/family/harness.rs`
- `xtask/src/family/scaffold.rs`
- `semantic-families/function.helper.identity_passthrough.v1/**`

Likely test / projection surfaces:

- `xtask/src/lib.rs`
- `spec-core/src/semantic_review.rs` tests only if current helper-family coverage is insufficient for prove/certify truth

Conditionally touched only if compile- or proof-forced:

- `xtask/src/family/inventory.rs`
- `xtask/src/family/coverage.rs`
- `semantic-families/README.md`

Forbidden surfaces unless the plan is rewritten:

- `xtask/src/family/recommend.rs`
- `xtask/src/family/analysis_core/decision_contract.rs`
- `xtask/src/family/helper_surface.rs`
- repo-root `ORCH_PLAN.md`

## Packet Contract

The helper family must stay aligned with the current classifier contract:

- function name: `round`
- deps: none
- invariants: none
- exactly one Decimal input
- Decimal return
- no control flow
- body shape:
  - direct passthrough is supported
  - round-like unary helper body is supported
- locked routing:
  - `precedence = 5`
  - `must_not_shadow = ["unsupported.function.v1"]`

The packet buckets must prove:

- **aligned**:
  - round-like intent + round-like body
  - passthrough intent + direct passthrough body
- **drift**: passthrough intent + round-like body
- **under_specified**: vague intent with otherwise-supported body
- **unsupported_near_miss**: control-flow branch around an otherwise helper-shaped body

This keeps the packet honest to both the shared-spec example and the classifier tests already shipping in `spec-core`.

Helper-family naming constraint:

- committed proving fixtures cannot rely on filenames like `round_aligned.unit.spec` for supported cases, because the current classifier hard-requires `fn_name == "round"`
- the locked scaffold starter can still live at `fixtures/<bucket>/units/money/round.unit.spec` and remain non-proving by authored content, matching the existing `family new` contract
- if M43 needs a second aligned proving case, add it as an extra committed packet fixture under a second namespace while keeping scaffold generation single-namespace; do not widen scaffold namespace plumbing in this milestone

## Implementation Plan

Execution order matters more than code volume in M43. Freeze the helper-family contract first,
then let packet authoring and regression work happen in parallel, then merge back into one proof
wall. Do not mix contract definition, scaffold widening, and projection rewrites in the same first
edit.

### Step dependency table

| Step | Goal | Primary surfaces | Depends on | Exit signal |
|---|---|---|---|---|
| 1 | lock the promoted helper-family contract | `xtask/src/family/harness.rs` | — | the registry exposes the family with the final routing and suite ownership contract |
| 2 | make `family new` emit truthful helper starter content | `xtask/src/family/scaffold.rs`, `xtask/src/lib.rs` | 1 | starter generation is green and still valid-but-non-proving |
| 3 | commit the packet that the harness and smoke/prove/certify will enforce | `semantic-families/function.helper.identity_passthrough.v1/**` | 1 | the packet contains all four buckets and an explicit story for both aligned lanes |
| 4 | refresh regression and read-side truth | `spec-core/src/semantic_review.rs`, `xtask/src/lib.rs`, `xtask/src/family/inventory.rs`, `xtask/src/family/coverage.rs` | 1, 2, 3 | direct passthrough is proven and projections flip from supported-unpromoted to promoted |
| 5 | run the full proof wall and capture acceptance evidence | local command wall | 2, 3, 4 | smoke, prove, certify, inventory, and coverage all match the expected end state |

### Step 1. Lock the helper-family contract in `harness.rs`

Goal: make the runtime-supported helper family first-class in the registry without widening what
the family means.

Required edits:

- add the family routing entry to `FAMILY_REGISTRY`
- lock `precedence = 5`
- lock `must_not_shadow = ["unsupported.function.v1"]`
- define the suite slug, summary text, starter-case definitions, smoke contract, prove suites, and certify ownership

Done when:

- helper family discovery succeeds through the same registry lookup path as the promoted arithmetic families
- no TypeScript or second-language prove/certify ownership is introduced
- the harness contract matches the classifier contract already defined in `spec-core`

### Step 2. Add helper-family scaffold support in `scaffold.rs`

Goal: make `cargo xtask family new function.helper.identity_passthrough.v1` generate a starter
packet that is truthful, stable, and intentionally non-proving.

Required edits:

- add one helper-family scaffold template for `candidate.md`, `family.toml`, and the four fixture buckets
- keep locked starter units at `fixtures/<bucket>/units/money/round.unit.spec`
- preserve the current starter contract instead of inventing a second scaffold mode

Done when:

- every scaffolded starter validates
- every scaffolded starter remains outside the supported subset by default
- the scaffold does not introduce per-case namespace plumbing just to express the second aligned lane

### Step 3. Commit the helper-family packet

Goal: commit the exact packet truth that smoke, prove, and certify will defend.

Required files:

- `semantic-families/function.helper.identity_passthrough.v1/family.toml`
- `semantic-families/function.helper.identity_passthrough.v1/candidate.md`
- `semantic-families/function.helper.identity_passthrough.v1/fixtures/aligned/**`
- `semantic-families/function.helper.identity_passthrough.v1/fixtures/drift/**`
- `semantic-families/function.helper.identity_passthrough.v1/fixtures/under_specified/**`
- `semantic-families/function.helper.identity_passthrough.v1/fixtures/unsupported_near_miss/**`

Packet rules:

- the packet must be self-contained and must not depend on units outside the packet
- the aligned bucket must preserve the existing round-like `money/round` wedge
- the plan must also prove the direct-passthrough aligned lane that the classifier already accepts
- if the second aligned lane is easiest to express as an extra committed packet fixture plus a `spec-core` regression, do that
- do not widen scaffold infrastructure first just to make the second aligned lane look symmetrical

### Step 4. Refresh regression and read-side truth

Goal: make the codebase prove the promotion everywhere it matters, not just in packet files.

Required edits:

- add the missing direct-passthrough aligned regression in `spec-core/src/semantic_review.rs`
- refresh `xtask` tests so they prove registry presence, scaffold generation, smoke, prove, and certify
- update inventory and coverage expectations so the helper family leaves supported-unpromoted truth and enters promoted-family truth
- repoint any helper-family inventory metadata that still describes the family as a transitional supported-unpromoted wedge

Done when:

- `spec-core` proves both supported aligned lanes plus drift, under-specified, and unsupported-near-miss behavior
- `xtask` proves scaffold starters stay valid-but-non-proving
- inventory no longer reports the helper family as supported-unpromoted
- coverage moves the three helper units out of `supported_unpromoted_family_units`

### Step 5. Run the live proof wall

Run the narrow tests first, then the full promotion loop, then the read-side truth commands:

```bash
cargo test -p spec-core helper_identity_passthrough -- --color never
cargo test -p xtask inventory -- --color never
cargo test -p xtask coverage -- --color never
cargo test -p xtask -- --color never
cargo xtask family smoke function.helper.identity_passthrough.v1
cargo xtask family prove function.helper.identity_passthrough.v1
cargo xtask family certify function.helper.identity_passthrough.v1
cargo xtask family inventory --format json
cargo xtask family coverage --format json
```

Acceptance evidence:

- `supported_unpromoted_families[]` no longer contains `function.helper.identity_passthrough.v1`
- `supported_unpromoted_family_units = 0`
- helper-family packet artifacts are present and truthful
- `cargo xtask family certify function.helper.identity_passthrough.v1` passes without manual patch-up

## Error And Rescue Registry

| Method / codepath | What can go wrong | Rescue class | Rescue action | User sees |
|---|---|---|---|---|
| `family new function.helper.identity_passthrough.v1` | harness missing | `NotImplemented` | fail loudly with harness registration message | maintainer gets actionable error |
| `family smoke ...` | scaffold output mismatches committed packet | smoke failure | fail loudly with exact-match / content diff | maintainer sees smoke failure |
| `family prove ...` | helper classifier or suite expectations drift | prove failure | fail loudly with gate and suite detail | maintainer sees failed gate |
| `family certify ...` | certify gates fail after prove | certification failure | fail loudly and preserve artifacts | maintainer sees certification failure |
| inventory / coverage projections | helper family remains supported-unpromoted | regression test failure | fail test / live assertion | maintainer sees projection mismatch |

No silent failure paths are acceptable in M43.

## Test Diagram

```text
HELPER FAMILY PROMOTION COVERAGE
================================
[+] spec-core helper classifier truth
    ├── aligned round-like helper routes to supported helper family
    ├── aligned passthrough helper routes to supported helper family
    ├── vague intent reports under-specified
    ├── passthrough intent + round-like body reports drift
    └── control-flow near miss stays unsupported

[+] xtask harness / scaffold
    ├── helper family registered for family new/smoke/prove/certify
    ├── scaffold emits shape-honest valid-but-non-proving starter files
    └── smoke verifies committed packet contracts

[+] packet proof loop
    ├── prove passes all helper-family suites
    ├── certify passes all helper-family gates
    └── artifacts serialize correctly

[+] read-side family-analysis truth
    ├── inventory no longer lists helper family as supported-unpromoted
    └── coverage no longer counts helper units in supported_unpromoted_family_units
```

## Failure Modes Registry

| Codepath | Failure mode | Rescued? | Test? | User sees? | Logged? |
|---|---|---:|---:|---|---:|
| helper harness registration | family id omitted from registry | N | Y | explicit `NotImplemented` failure | Y |
| helper scaffold | emitted starter does not match committed packet contract | N | Y | explicit smoke failure | Y |
| helper prove suites | helper classifier truth and packet fixtures disagree | N | Y | explicit prove failure | Y |
| helper certify gates | one certify suite regresses | N | Y | explicit certification failure | Y |
| inventory / coverage projections | helper family still counted as supported-unpromoted after packet lands | N | Y | explicit test / command mismatch | Y |

Critical-gap rule:

- any row with `Test = N` is unacceptable for M43
- any silent failure in the promotion workflow is unacceptable for M43

## Worktree Parallelization Strategy

This plan has one global gate and then three honest parallel workstreams.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Step 1. Lock helper-family contract | `xtask/src/family/` | — |
| Step 2. Add scaffold support | `xtask/src/family/`, `xtask/src/` | Step 1 |
| Step 3. Commit packet assets | `semantic-families/function.helper.identity_passthrough.v1/` | Step 1 |
| Step 4. Add direct-passthrough regression | `spec-core/src/` | Step 1 |
| Step 5. Refresh projections and run proof wall | `xtask/src/`, `xtask/src/family/`, local proof commands | Steps 2, 3, 4 |

### Parallel lanes

- Lane 0: Step 1
  Lock the family contract first. No other lane should guess the final routing, suite, or naming rules.
- Lane A: Step 2
  Scaffold support in `xtask/src/family/` and `xtask/src/` stays sequential because it shares starter-generation logic and test helpers.
- Lane B: Step 3
  Packet authoring under `semantic-families/function.helper.identity_passthrough.v1/` can run independently once Step 1 freezes the contract.
- Lane C: Step 4
  The `spec-core` direct-passthrough regression can run independently once Step 1 freezes the contract.
- Lane D: Step 5
  Projection updates, integration tests, and the live proof loop run after A, B, and C merge.

### Execution order

- Launch Lane 0 first.
- After Step 1 lands, launch Lanes A, B, and C in parallel worktrees.
- Merge Lanes B and C as soon as they are green.
- Merge Lane A once scaffold snapshots and starter-contract tests are green.
- Run Lane D last on top of the merged result to refresh projections and prove the end state.

### Conflict flags

- Lanes A and D both touch `xtask/src/` and `xtask/src/family/`, so D must wait.
- Lanes A and B do not share modules, but they do share the packet contract. Step 1 is the contract freeze that keeps them from drifting.
- Lane C is low-conflict, but it must not invent a different interpretation of the helper subset than the packet and harness use.

## Review Notes

### UI scope

No UI scope. `plan-design-review` should be skipped for M43.

### DX scope

DX scope exists because this milestone changes maintainer-facing commands and packet ergonomics. The main DX rule is clarity:

- the new family must use the same workflow shape maintainers already know
- command failures must stay explicit and local
- no hidden second path for helper promotion

## Completion Criteria

M43 is complete only if all of the following are true:

1. `function.helper.identity_passthrough.v1` is registered in the harness registry.
2. A committed helper-family packet exists under `semantic-families/`.
3. `family smoke`, `family prove`, and `family certify` all pass for the helper family.
4. Inventory and coverage no longer report the helper family as supported-unpromoted.
5. The full `cargo test -p xtask` wall stays green.
6. The implementation did not widen into recommendation policy, shared-core portability, or second-language work.

## Completion Summary

If M43 lands cleanly, the repo stops treating helper support as an orphaned runtime fact.

Maintainers get the same `family new` / `family smoke` / `family prove` / `family certify`
workflow for `function.helper.identity_passthrough.v1` that the other promoted Rust families
already have. Inventory and coverage stop reporting the helper family as stranded supported truth.

## CEO Review

Verdict: hold scope, but tighten the honest subset so the plan does not quietly under-prove the
family it claims to promote.

What changed in review:

- kept the milestone focused on promotion, not corpus policy or substrate widening
- rejected a scaffold-plumbing expansion just to model a second aligned namespace
- required explicit proof for the direct-passthrough aligned lane because the classifier already treats it as supported truth

CEO CONSENSUS TABLE:

| Topic | Host review | Outside voice | Subagent | Result |
|---|---|---|---|---|
| Scope | promote helper family only | unavailable | unavailable | confirm |
| Corpus policy | no change in M43 | unavailable | unavailable | confirm |
| Second-language work | defer | unavailable | unavailable | confirm |
| Helper subset | prove both aligned lanes | unavailable | unavailable | confirm |

## Eng Review

Verdict: the plan is execution-ready after one correction.

Critical engineering finding:

- the helper classifier hard-requires `fn_name == "round"` in [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs:1388), so committed proving fixtures cannot use suffix-bearing callable names the way the arithmetic packets do
- scaffold starters are already expected to remain valid-but-non-proving in [xtask/src/lib.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/xtask/src/lib.rs:5381), so M43 should preserve that contract instead of trying to make `family new` emit already-proving helper fixtures
- the current helper regression set proves round-like alignment, drift, under-specification, and unsupported control flow, but it does not prove the supported direct-passthrough aligned lane at [spec-core/src/semantic_review.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/semantic_review.rs:6079)

Engineering decision:

- keep scaffold generation single-namespace and starter-only
- prove the second aligned lane with a hand-authored extra packet fixture and a new `spec-core` regression
- repoint helper inventory metadata away from the old supported-unpromoted wedge once the packet exists

ENG CONSENSUS TABLE:

| Topic | Host review | Outside voice | Subagent | Result |
|---|---|---|---|---|
| Registry/harness change | required | unavailable | unavailable | confirm |
| Scaffold model | keep existing starter contract | unavailable | unavailable | confirm |
| Helper aligned coverage | add direct-passthrough proof | unavailable | unavailable | confirm |
| Inventory/coverage delta | must flip to promoted truth | unavailable | unavailable | confirm |

## DX Review

Verdict: DX scope exists, but it is narrow and maintainer-facing.

No dedicated `plan-devex-review` skill was available in this environment, so this pass used host
review only.

DX scorecard:

| Dimension | Score | Note |
|---|---:|---|
| Discoverability | 8/10 | existing `xtask family` verbs already frame the workflow |
| Setup friction | 8/10 | no new toolchain expected |
| Naming clarity | 7/10 | helper packet naming must explicitly document the `round` filename constraint |
| Error clarity | 9/10 | smoke/prove/certify already fail loudly |
| Workflow consistency | 9/10 | promotion path stays identical to the other Rust families |
| Testability | 9/10 | proof loop is explicit |
| TTHW | 8/10 | familiar maintainer loop, one new packet |
| Escape hatches | 8/10 | scope boundaries are explicit in this plan |

DX implementation checklist:

- document the `round` callable-name constraint in `candidate.md`
- keep scaffold failure messages local and explicit if helper starter files drift
- avoid adding a second hidden promotion path for helper fixtures

## Cross-Phase Themes

**Theme: keep the helper subset explicit**. CEO, eng, and DX review all converged on the same
point: M43 is good only if it promotes the exact helper truth the runtime already supports, no
more and no less.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|---|---|---|---:|---|---|
| CEO Review | `/autoplan` | Scope & strategy | 1 | clean | tightened proof contract, kept scope narrow |
| Codex Review | outside voice | Independent 2nd opinion | 0 | unavailable | Claude auth unavailable, no outside run |
| Eng Review | `/autoplan` | Architecture & tests | 1 | clean | fixed helper filename/scaffold assumption, added missing aligned lane proof |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | no UI scope |
| DX Review | `/autoplan` host fallback | Maintainer workflow | 1 | clean | keep one promotion path, preserve valid-but-non-proving starter contract |

**VERDICT:** REVIEWED, READY FOR APPROVAL GATE.

<!-- AUTONOMOUS DECISION LOG -->
## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | 0 | Bootstrap a fresh M43 design doc inside `/autoplan` instead of offering prerequisite branches | user direction | P6 | the user explicitly corrected the workflow and the design doc was missing | pause for `/office-hours` or skip design doc creation |
| 2 | 1 | Hold scope to helper-family promotion only | auto-decided | P2 | this is the smallest complete lake with live demand evidence | corpus-policy reopening, shared-core follow-on, TypeScript expansion |
| 3 | 2 | Skip design review | auto-decided | P1 | M43 has no UI scope | running a fake UI review |
| 4 | 3 | Preserve scaffold starters as valid-but-non-proving | auto-decided | P4 | existing `xtask` tests already enforce that contract | making `family new` emit proving helper fixtures |
| 5 | 3 | Add explicit direct-passthrough aligned proof | auto-decided | P1 | the classifier already accepts that lane, and the current tests do not prove it | shipping a promoted packet that under-covers supported truth |
| 6 | 3.5 | Keep scaffold single-namespace and express any second aligned lane in committed packet proof | taste decision | P2 | it avoids widening framework plumbing for one bounded family | expanding scaffold namespace modeling in M43 |
| 7 | 3.5 | Repoint helper inventory metadata to classifier-plus-packet truth after promotion | auto-decided | P4 | the old helper-surface wedge is only honest while the family is supported-unpromoted | leaving inventory metadata anchored to transitional surfaces |
