<!-- restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m21-m22-plan-solidify-restore-20260427-214444.md -->
# M22 - Integrity-First Semantic Family Promotion Hardening

Status: **solidified implementation plan on `feat/m21`**  
Last rewritten: **2026-04-27**

M21 proved one real family promotion path. `function.wrapper.pipeline.chain3.v1` can be scaffolded,
proved, and certified through the current xtask flow. What M21 did not prove is the stronger repo
claim that maintainers now have a clean, boring, repeatable path for family number two.

M22 fixes that narrower but more important gap.

The milestone is not "add more semantics." The milestone is "make the repo tell the truth about
how family promotion works, and make the xtask harness general enough that the next family only
needs one explicit registry entry instead of scattered chain3 assumptions."

## Milestone Outcome

After M22, the intended maintainer workflow is:

1. add one explicit family definition to the xtask family registry
2. run `cargo xtask family new <family>`
3. fill packet fixtures and packet metadata
4. wire runtime classification in `spec-core`
5. run `cargo xtask family prove <family>`
6. run `cargo xtask family certify <family>`

That is still registry-first. It is not manifest-magical. M22 succeeds only if the code and docs
both say that plainly.

## Core Decision

Keep orchestration truth in Rust.

- `xtask/src/family/harness.rs` remains the sole source of family-specific xtask orchestration
  data.
- `family.toml` remains packet-local validation and truth-surface metadata, not command-selection
  truth.
- `chain3` remains the only real promoted family shipped in M22.
- Generalization is proven with synthetic multi-family xtask tests, not by sneaking in a second
  real semantic family.

This is the smallest complete fix. It keeps the trust boundary boring and the blast radius
contained.

## Verified Starting Point

The following facts were verified on 2026-04-27 on `feat/m21`:

- `cargo xtask family prove function.wrapper.pipeline.chain3.v1` passes.
- `cargo xtask family certify function.wrapper.pipeline.chain3.v1` passes.
- `cargo xtask family new function.wrapper.pipeline.chain4.v1` fails with the expected
  "family is not registered" error.
- `xtask/src/family/harness.rs` contains `const FAMILY_REGISTRY: [FamilyHarness; 1]`.
- `xtask/src/family/routing.rs` still hard-codes one-family assumptions:
  - `debug_assert_eq!(harnesses.len(), 1, "locked routing helper assumes one family")`
  - `debug_assert_eq!(harnesses[0].routing.must_not_shadow.len(), 3, ...)`
  - `locked_routing_order_with_terminal()` returns a fixed `[&'static str; 5]`

Those facts define the plan boundary. M22 is about removing those hidden "chain3 is the world"
assumptions without changing the public `spec` CLI or pretending a second real family already
exists.

## Problem Statement

M21 shipped a real harness, but the repo still has three truth mismatches:

1. **Bootstrap truth mismatch**  
   The docs can still be read as if `family new/prove/certify` are generic next-family workflows.
   They are not. They require Rust-side registration first.

2. **Orchestration truth mismatch**  
   The registry exists, but helper logic in `routing.rs` and nearby consumers still assumes one
   registered family and one fixed `must_not_shadow` width.

3. **Reviewability mismatch**  
   Future maintainers can still trip over "works for chain3, breaks for family two" behavior
   because the generalization boundary is not proven in tests.

M22 closes those three mismatches and nothing more.

## What Already Exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Explicit family definition | `xtask/src/family/harness.rs` with `FamilyHarness` and `FAMILY_REGISTRY` | Reuse directly. Do not invent a second registry format. |
| Packet path safety and layout | `xtask/src/family/paths.rs`, `layout.rs`, scaffold bucket creation | Reuse directly. |
| Prove/certify reporting | `xtask/src/family/prove.rs`, `certify.rs`, `report.rs` | Reuse directly. Keep report format stable unless a real defect forces change. |
| Live promoted family | `function.wrapper.pipeline.chain3.v1` packet plus current chain3 prove/certify path | Preserve as the frozen regression backstop. |
| Honest failure for unknown families | `require_family_harness()` | Preserve behavior and make surrounding docs match it. |

## Non-Negotiable Invariants

These are hard constraints, not suggestions:

1. `family.toml` does not select commands to run.
2. Suite definitions stay compile-time Rust constants.
3. Public `spec` CLI behavior stays unchanged.
4. M22 does not add a second real semantic family in `spec-core`.
5. M22 does not add a new registry file format, codegen step, or generic multi-kind abstraction.
6. `chain3` prove/certify stays green through the entire milestone.

## Affected Modules

| Module | Role in M22 | Required change |
|---|---|---|
| `xtask/src/family/harness.rs` | Registry source of truth | Expose the full family contract through one obvious API surface and make registry-driven helpers testable with synthetic registries. |
| `xtask/src/family/routing.rs` | Locked routing checks and mismatch messages | Remove fixed-width and one-family assumptions. |
| `xtask/src/family/scaffold.rs` | Packet bootstrap and manifest template generation | Ensure all family-specific values come from the selected harness, never chain3 spillover. |
| `xtask/src/family/prove.rs` | Family prove workflow | Keep suite selection registry-owned and free of hidden chain3 carveouts outside the harness. |
| `xtask/src/family/certify.rs` | Family certify workflow | Keep routing mismatch checks accurate for any registered family. |
| `xtask/src/lib.rs` | xtask test coverage | Add synthetic multi-family tests that prove generalization without adding a second production family. |
| `semantic-families/README.md` | Maintainer-facing workflow contract | Rewrite to say exactly what is manual and what is automated. |
| `README.md` and `AGENTS.md` if needed | Secondary workflow wording | Update only if they repeat the old implication. |

## Architecture Contract

### Durable source of truth

`FamilyHarness` remains the single family-definition shape for xtask orchestration:

- scaffold defaults
- locked manifest routing
- prove suite membership
- certify suite membership

No other module may reconstruct family-specific behavior from chain3-specific constants once M22
is done.

### Required internal shape

The helper logic must become testable against synthetic registries without forcing a second real
production family into `FAMILY_REGISTRY`.

That means:

- production wrappers may still read `FAMILY_REGISTRY`
- pure helper logic used for ordering, mismatch rendering, and template population must be callable
  against synthetic two-family inputs in xtask tests
- test coverage must prove behavior scales past one family even while production registry size
  remains one

### Data-flow diagram

```text
cargo xtask family <cmd> <family>
        │
        ├── parse FamilyId
        │
        ├── resolve harness from xtask registry
        │       │
        │       ├── scaffold contract
        │       ├── locked routing contract
        │       ├── prove suite contract
        │       └── certify suite contract
        │
        ├── scaffold.rs
        │       └── generates candidate.md + family.toml + starter fixtures
        │
        ├── prove.rs
        │       ├── validates packet
        │       ├── runs registry-selected suites
        │       └── writes prove.latest.json
        │
        ├── certify.rs
        │       ├── reuses prove execution
        │       ├── checks manifest routing against locked registry routing
        │       └── writes certification.report.json
        │
        └── semantic-families/README.md
                └── describes the same registry-first workflow the code enforces
```

## Scope Challenge

### Minimum diff that still solves the problem

The minimum complete M22 is:

- honest registry-first docs
- one obvious harness API surface
- no one-family assumptions in routing and related helpers
- synthetic multi-family xtask tests
- live chain3 prove/certify regression reruns

Anything beyond that is scope creep.

### Explicitly rejected shortcuts

- docs-only fix with no harness hardening
- shipping a second real family to "prove" generalization
- moving orchestration into `family.toml`
- adding a generic family framework for `function`, `data`, and `sum` in one pass

### Complexity call

This touches roughly 7-9 files in one subsystem. That is acceptable because the blast radius is
contained to xtask family promotion and docs. Do not widen it into `spec-core` semantic expansion.

## Implementation Plan

### Workstream 1. Narrow the repo claim

**Files**

- `semantic-families/README.md`
- `README.md` only if it repeats the same implication
- `AGENTS.md` only if it repeats the same implication

**Changes**

- Rewrite family-promotion workflow text to say:
  - registration happens first in `xtask/src/family/harness.rs`
  - `family new` bootstraps packets only for registered families
  - `family.toml` is packet metadata and validation truth, not orchestration truth
- Remove any wording that suggests arbitrary new-family bootstrap already works end-to-end.

**Acceptance**

- A maintainer reading docs first would not expect `cargo xtask family new <new-family>` to work
  before registry entry creation.
- No doc path contradicts the runtime error from `require_family_harness()`.

### Workstream 2. Lock the harness contract

**Files**

- `xtask/src/family/harness.rs`

**Changes**

- Keep `FamilyHarness` as the only family-definition struct.
- Add or refactor helper accessors so scaffold, routing, prove, and certify all obtain
  family-specific data from the same harness path.
- Extract internal pure helper logic as needed so xtask tests can exercise synthetic registries
  without mutating the production registry.
- Preserve the unknown-family error message and keep it explicitly pointed at
  `xtask/src/family/harness.rs`.

**Acceptance**

- There is one obvious place to add the next family for xtask orchestration.
- No command consumer duplicates per-family contract knowledge outside the harness contract.
- The production registry may still have one entry, but the helper logic is no longer structurally
  tied to one entry.

### Workstream 3. Remove one-family routing assumptions

**Files**

- `xtask/src/family/routing.rs`
- `xtask/src/family/harness.rs` if helper extraction lives there

**Changes**

- Remove `debug_assert_eq!(harnesses.len(), 1, ...)`.
- Remove `debug_assert_eq!(harnesses[0].routing.must_not_shadow.len(), 3, ...)`.
- Replace fixed-width array logic with registry-driven iteration.
- Keep the terminal `unsupported.function.v1` catch-all explicit in the rendered locked order.
- Ensure mismatch messages remain family-specific and actionable.

**Acceptance**

- Routing helpers behave correctly with a synthetic registry of at least two families.
- No helper assumes chain3's `must_not_shadow` width globally.
- Mismatch text identifies the target family and the expected locked routing contract clearly.

### Workstream 4. Align scaffold, prove, and certify to the same registry contract

**Files**

- `xtask/src/family/scaffold.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`

**Changes**

- Ensure scaffold templates derive precedence and `must_not_shadow` from the selected harness only.
- Ensure prove and certify suite selection stays registry-owned and does not rely on out-of-band
  chain3 assumptions.
- Keep certify routing mismatch checks driven by the same locked routing contract as scaffold and
  helper ordering.

**Acceptance**

- A synthetic non-chain3 harness used in tests produces its own routing values in
  `manifest_template()`.
- Prove/certify consumers do not contain hidden duplicate family contract data outside the harness.

### Workstream 5. Add synthetic multi-family xtask tests

**Files**

- `xtask/src/lib.rs`

**Changes**

- Add test-only synthetic harness definitions or synthetic registry inputs.
- Cover the following cases:
  - family lookup works for 2+ families
  - routing order is stable across 2+ families plus the terminal catch-all
  - manifest mismatch messages stay accurate for a non-chain3 family
  - scaffold manifest generation uses the selected family's routing values
  - unknown-family error still points to `xtask/src/family/harness.rs`
- Keep existing live chain3 contract tests intact.

**Acceptance**

- xtask test coverage proves generalization without adding a second production family.
- The plan no longer relies on "trust me, family two will work."

### Workstream 6. Re-run the live chain3 proof

**Files**

- no new source files required

**Commands**

```bash
cargo test -p xtask
cargo xtask family prove function.wrapper.pipeline.chain3.v1
cargo xtask family certify function.wrapper.pipeline.chain3.v1
rg -n "registered for|registry-first|self-bootstrapping" semantic-families/README.md README.md AGENTS.md PLAN.md
```

**Acceptance**

- xtask tests pass
- chain3 prove passes
- chain3 certify passes
- docs and plan wording are aligned with the implemented registry-first behavior

## Code Quality Constraints

These are implementation rules for M22:

- Prefer explicit iterator- or slice-based helpers over macros or generic registry DSLs.
- Keep the diff minimal. No new abstraction is justified unless it removes actual duplicated
  family-contract knowledge.
- Any nearby ASCII comment or workflow diagram touched by this change must be updated in the same
  commit.
- If a helper only exists to paper over chain3-only assumptions, delete or flatten it.

## Test Plan

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/harness.rs
    │
    ├── family_harness()
    │   ├── [REQUIRED] lookup works with 2+ synthetic family definitions
    │   └── [REQUIRED] unknown family error still points to harness.rs
    │
    └── registry-driven helper accessors
        ├── [REQUIRED] no production-only singleton assumption
        └── [REQUIRED] helpers accept synthetic multi-family inputs in tests

[+] xtask/src/family/routing.rs
    │
    ├── locked_routing_order_with_terminal()
    │   └── [REQUIRED] dynamic ordering across 2+ families plus terminal catch-all
    │
    ├── manifest_matches_locked_routing()
    │   ├── [REQUIRED] positive case for non-chain3 synthetic family
    │   └── [REQUIRED] mismatch when precedence or must_not_shadow differ
    │
    └── manifest_routing_mismatch_message()
        └── [REQUIRED] family-specific actionable mismatch text

[+] xtask/src/family/scaffold.rs
    │
    └── manifest_template()
        ├── [REQUIRED] synthetic family gets its own precedence
        └── [REQUIRED] synthetic family gets its own must_not_shadow values

[+] xtask/src/family/prove.rs + certify.rs
    │
    ├── registry-selected suite execution
    │   └── [REQUIRED] no duplicate chain3-only contract logic outside harness
    │
    └── chain3 live regression
        ├── [REQUIRED] prove still passes end-to-end
        └── [REQUIRED] certify still passes end-to-end
```

### Operator-flow coverage

```text
OPERATOR FLOW COVERAGE
===========================
[+] Maintainer reads docs first
    └── [REQUIRED] docs say registration comes before family new/prove/certify

[+] Maintainer forgets registration
    └── [REQUIRED] gets explicit registry-first error

[+] Maintainer adds a new family later
    └── [REQUIRED] xtask helper logic is already proven against synthetic 2+ family registries

[+] Maintainer breaks routing metadata
    └── [REQUIRED] certify mismatch output stays actionable for a non-chain3 family
```

### Test artifact

The existing eng-review test-plan artifact remains the QA handoff anchor for this branch:

`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m21-eng-review-test-plan-20260427-213147.md`

No new test artifact is required unless the implementation plan meaningfully changes.

## Failure Modes Registry

| Codepath | Realistic failure | Test required? | Error handling required? | Operator outcome if broken |
|---|---|---:|---:|---|
| `family_harness()` | future family missing from registry | yes | yes | clear, actionable failure |
| registry helper extraction | helper remains implicitly singleton-scoped | yes | no | family two breaks despite "generalized" code |
| `locked_routing_order_with_terminal()` | order or width still encodes chain3-only assumptions | yes | partial | misleading certify mismatch output |
| `manifest_template()` | synthetic family inherits chain3 routing values | yes | no | silently wrong packet bootstrap |
| `manifest_routing_mismatch_message()` | reports wrong expected routing for non-chain3 family | yes | partial | hard-to-debug certification failure |
| docs workflow | docs still imply self-bootstrap | doc review only | no | wasted maintainer time and false confidence |

**Critical gap to prevent:** scaffold spillover. If a future family can generate a packet with
chain3 routing values silently, that is a correctness failure, not a documentation defect.

## Performance Review

- Registry iteration is trivial. Do not optimize it.
- Do not widen prove/certify into workspace-scale discovery or duplicate suite execution.
- Do not add caches. M22 is about correctness and explicitness, not speed.

## Security and Trust Boundary Review

- `family.toml` remains validated input, not executable orchestration.
- Suite commands remain compile-time Rust constants.
- Existing packet path safety and symlink rejection remain unchanged.
- No user-authored packet data may decide which subprocesses xtask executes.

## Worktree Parallelization Strategy

M22 has one real code lane and one small docs lane. The code lane remains the critical path.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Docs honesty pass | `semantic-families/`, `README.md`, `AGENTS.md` | harness contract wording frozen |
| Harness contract cleanup | `xtask/src/family/` | — |
| Routing generalization | `xtask/src/family/` | harness contract cleanup |
| Consumer alignment | `xtask/src/family/` | routing generalization |
| Synthetic multi-family tests | `xtask/src/` | consumer alignment |
| Live chain3 proof reruns | `xtask/` via commands | synthetic multi-family tests |

### Parallel lanes

- Lane A: harness contract cleanup -> routing generalization -> consumer alignment -> synthetic
  multi-family tests -> live chain3 proof reruns
- Lane B: docs honesty pass

### Execution order

1. Start Lane A first. It determines the final contract wording.
2. Start Lane B after the harness contract is settled enough that docs language will not thrash.
3. Merge Lane B back before final verification so the grep-based wording checks run against final
   docs.

### Conflict flags

- Lane A and Lane B both touch workflow language. Keep Lane B small and rebase it after Lane A if
  necessary.
- There is no safe multi-code-lane split inside `xtask/src/family/`; the same module cluster owns
  the entire blast radius.

## NOT in Scope

- Promoting a second real semantic family in `spec-core`
- Changing the public `spec` CLI
- Making `family.toml` self-bootstrapping or command-bearing
- Multi-kind family promotion for `data` or `sum`
- CI workflow redesign
- certification artifact schema redesign
- generic registry abstraction beyond what this xtask family harness needs

## Green Gate

M22 is green only if all of the following are true:

- docs say registry-first, not self-bootstrapping
- `xtask/src/family/routing.rs` no longer assumes one family or one fixed width
- scaffold/prove/certify all derive family-specific values from the harness contract
- synthetic multi-family xtask tests pass
- `cargo xtask family prove function.wrapper.pipeline.chain3.v1` passes
- `cargo xtask family certify function.wrapper.pipeline.chain3.v1` passes

## Red Gate

M22 is red if any of the following remain true:

- a future family still requires hidden edits outside one explicit xtask registry contract
- routing helpers still encode registry length `== 1`
- scaffold templates can spill chain3 routing data into another family shape
- docs still imply arbitrary new-family bootstrap is already automated
- chain3 prove/certify regresses during hardening

## Review Status

This plan has already been pressure-tested for strategy and engineering shape. The purpose of this
rewrite is to convert that review output into one implementation contract.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | clear (manual) | integrity-first scope chosen, second real family deferred, docs claim narrowed |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | clear (manual) | registry-first contract, multi-family proof requirements, chain3 regression preserved |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | no UI scope |

**UNRESOLVED:** 0

**VERDICT:** CEO + ENG CLEARED (manual) — ready to implement M22 on `feat/m21`.
