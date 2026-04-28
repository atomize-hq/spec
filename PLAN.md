<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m21-autoplan-restore-20260427-213147.md -->
# M22 - Integrity-First Semantic Family Promotion Hardening

Status: **Fresh plan on `feat/m21`** (rewritten via `/autoplan` + `/plan-eng-review` on 2026-04-27).

M21 was a real milestone. `function.wrapper.pipeline.chain3.v1` now proves and certifies through
`cargo xtask family prove|certify`. The missing piece is narrower and more important: the repo
still does **not** have an honest, boring, repeatable maintainer workflow for family number two.

M22 fixes that exact gap. It does **not** expand semantic ambition. It hardens bootstrap truth,
orchestration truth, and docs truth so the next family can be added through one explicit contract
instead of maintainer memory and scattered chain3-only constants.

UI scope: **no**. This is backend-only harness and process work.

## Source Inputs

- Current branch: `feat/m21`
- Base branch: `main`
- Restore point:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m21-autoplan-restore-20260427-213147.md`
- Checkpoint:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/checkpoints/20260427-210258-m21-harness-review.md`
- Design doc:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m21-design-20260427-202732.md`
- Test plan artifact:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m21-eng-review-test-plan-20260427-213147.md`
- Verified repo behavior on 2026-04-27:
  - `cargo xtask family prove function.wrapper.pipeline.chain3.v1` passes.
  - `cargo xtask family certify function.wrapper.pipeline.chain3.v1` passes.
  - `cargo xtask family new function.wrapper.pipeline.chain4.v1` fails with the expected
    "family is not registered" error.
- Primary code seams:
  - `xtask/src/family/harness.rs`
  - `xtask/src/family/routing.rs`
  - `xtask/src/family/scaffold.rs`
  - `xtask/src/family/prove.rs`
  - `xtask/src/family/certify.rs`
  - `xtask/src/family/manifest.rs`
  - `xtask/src/lib.rs`
  - `semantic-families/README.md`

## Milestone Summary

```text
M22a  Make the bootstrap contract honest                                 required
M22b  Make orchestration truth explicit and reviewable                    required
M22c  Remove one-family assumptions from xtask helpers                    required
M22d  Keep chain3 green as the frozen regression backstop                 required
M22e  Prove generalization with synthetic multi-family tests              required
M22f  Update docs to say exactly what is and is not automated             required
```

**Lake to boil in M22**

- A maintainer should know exactly where to add the next family and exactly what stays manual.
- Adding a second family should require **one registry entry**, not edits in multiple hidden spots.
- `family.toml` should remain a packet contract, not an untrusted command script.
- Chain3 must stay the live proof while the workflow gets cleaned up.
- The repo must stop implying that the next-family path is already self-bootstrapping if it is not.

## User Outcome

After M22, a maintainer adding the next function family should have one boring workflow:

1. add one explicit family definition to the xtask registry
2. run `cargo xtask family new <family>`
3. fill the packet fixtures and manifest
4. wire the runtime classifier in `spec-core`
5. run `cargo xtask family prove <family>`
6. run `cargo xtask family certify <family>`

That is still registry-first. It is **not** manifest-magical. The point of M22 is that the repo
says that plainly and the code matches the claim.

## Step 0: Scope Challenge

### Current system state

| Surface | Already true | Still wrong | M22 response |
|---|---|---|---|
| `xtask/src/family/harness.rs` | Real chain3 harness exists | The next-family workflow hard-stops here and the error is truthful but repo claims are ahead of it | Keep this file as the explicit orchestration registry and document it honestly |
| `xtask/src/family/routing.rs` | Routing mismatch checks exist | Helpers still assume one registered family and one fixed `must_not_shadow` width | Generalize helpers to iterate over the registry, not chain3-only shapes |
| `xtask/src/family/scaffold.rs` | Scaffold path safety and bucket layout are real | `family new` is only reusable for already hard-coded families | Make reuse explicit: one registry entry unlocks scaffold, no extra hidden edits |
| `xtask/src/family/prove.rs` / `certify.rs` | Prove/certify reports and gate flow are real | Suite selection truth still lives in one chain3 carveout | Keep suite selection registry-owned, but centralize and test the contract |
| `semantic-families/README.md` / prior `PLAN.md` | Packet layout is documented | The docs can still be read as "add any family and run new/prove/certify" | Narrow wording to registry-first, reviewable bootstrap truth |

### What already exists

| Sub-problem | Existing code / flow | M22 reuse decision |
|---|---|---|
| Explicit family definition | `xtask/src/family/harness.rs` already contains `FamilyHarness` and `FAMILY_REGISTRY` | Reuse. Do not invent a second registry format. |
| Path safety and packet layout | `paths.rs`, `layout.rs`, scaffold bucket creation | Reuse directly. |
| Prove/certify report pipeline | `report.rs`, `prove.rs`, `certify.rs` | Reuse directly. Keep report schema stable unless a real gap appears. |
| Live semantic proof | chain3 packet, `spec-core` chain3 tests, `spec-cli` truth-surface and corpus tests | Reuse as the regression backstop. |
| Honest failure on unknown family | `require_family_harness()` | Reuse the behavior, but make the surrounding docs match it. |

### Minimum diff that still solves the problem

- Keep runtime semantic classification in `spec-core` unchanged.
- Keep public `spec` CLI unchanged.
- Keep `family.toml` as packet-local validation data, not executable orchestration.
- Refactor xtask family helpers so the registry is the obvious, sole orchestration truth.
- Add synthetic multi-family tests in `xtask` so the code no longer assumes "chain3 is the world".
- Update docs to describe the registry-first workflow honestly.

Anything beyond that is scope creep for M22.

### Complexity check

This is still a medium-sized change, roughly 7-9 files in one subsystem:

- `xtask/src/family/harness.rs`
- `xtask/src/family/routing.rs`
- `xtask/src/family/scaffold.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/lib.rs`
- `semantic-families/README.md`
- possibly `README.md` or `AGENTS.md` if workflow wording also lives there

Auto-decision: **scope reduced**. M22 does **not** ship a second real semantic family. It ships
the honest and generalizable harness contract first.

### Search check

- **[Layer 1]** Reuse the current Rust registry pattern. It is already in the repo and proven.
- **[Layer 3]** Do **not** let `family.toml` choose commands to execute. User-authored packet data
  controlling subprocesses would be a bad trust boundary.
- No new framework, background worker, or concurrency model is needed here.

### TODOS cross-reference

- Existing CLI harness cleanup TODOs remain orthogonal. M22 should not widen into general CLI test
  infrastructure cleanup.
- If, after M22, the Rust registry still feels too manual, that becomes an explicit future TODO:
  "make packet metadata drive more of the registry safely." It is not a reason to overbuild M22.

### Completeness check

The complete version is:

- honest registry-first docs
- single-source orchestration contract
- no one-family assumptions in helper code
- synthetic tests proving the registry shape scales beyond one entry
- chain3 prove/certify still green

Rejected shortcuts:

- docs-only honesty fix with no code hardening
- adding another real family before the harness contract is cleaned up
- moving command orchestration into `family.toml`
- generic multi-kind abstraction for function/data/sum in one pass

### Distribution check

No new binary or publish surface is introduced. Existing Cargo and CI remain enough.

`.semantic-family-artifacts/` stays the local and CI output surface for proof artifacts.

## Approved Scope

- Orchestration truth remains in Rust, in the xtask registry.
- `family.toml` remains packet validation data, not command-selection truth.
- `chain3` remains the only real promoted family in M22.
- Synthetic test families are allowed **inside xtask tests only** to prove helper generalization.
- Public `spec` CLI stays unchanged.
- No new report format, no new packet root layout, no new semantic kinds.

## Architecture Review

### Opinionated recommendation

Use the existing Rust registry as the durable source of orchestration truth. That is the smallest,
clearest, least clever fix. The repo already has this shape. The problem is not "no registry". The
problem is that the registry contract is implicit, chain3-scattered, and partially contradicted by
the docs.

### Dependency graph

```text
cargo xtask family <cmd> <family>
        │
        ├── FamilyId::parse()
        │
        ├── family_harness() / require_family_harness()
        │       │
        │       └── explicit Rust registry in harness.rs
        │              ├── scaffold defaults
        │              ├── routing contract
        │              ├── prove suite definitions
        │              └── certify suite definitions
        │
        ├── scaffold.rs
        │       └── candidate.md + family.toml + fixture buckets
        │
        ├── prove.rs
        │       ├── manifest.rs validation
        │       ├── layout.rs validation
        │       ├── registry-selected suites
        │       └── prove.latest.json
        │
        ├── certify.rs
        │       ├── prove execution reuse
        │       ├── routing.rs mismatch checks
        │       └── certification.report.json
        │
        └── semantic-families/README.md
                └── says the same thing the code actually does
```

### Concrete architecture changes

1. `xtask/src/family/harness.rs`
   - Keep `FamilyHarness` as the core registry shape.
   - Add helper APIs that return routing order, suite names, and scaffold defaults from the
     registry generically.
   - Remove any assumptions that the registry length is one.

2. `xtask/src/family/routing.rs`
   - Replace `debug_assert_eq!(harnesses.len(), 1)` and fixed-array helpers with dynamic helpers
     driven by registry order.
   - Keep the terminal `unsupported.function.v1` catch-all explicit.

3. `xtask/src/family/scaffold.rs`, `prove.rs`, `certify.rs`
   - Continue to require registration first.
   - Ensure the error text consistently points maintainers to the registry as the single edit site.
   - Do not duplicate per-family contract knowledge anywhere else.

4. `semantic-families/README.md`
   - Say plainly: "In M22, packet creation is registry-first. Add a Rust registry entry, then run
     xtask. The manifest does not bootstrap a family on its own."

### Error & Rescue Registry

| Failure | Why it happens | Rescue |
|---|---|---|
| Unknown family still requires hidden edits | Registry knowledge still leaks into other files | Add a test that a synthetic second family can be registered in one place and all helpers see it |
| Routing helper still encodes one-family width | Future family order bugs hide until runtime | Drive routing-order formatting from registry iteration, not fixed arrays |
| Docs overclaim automation again | A future maintainer copies the wrong narrative | Lock wording in `semantic-families/README.md` and test the error message path |
| `family.toml` starts controlling commands | Packet data becomes a code-execution surface | Keep suite commands compile-time Rust constants only |

## Code Quality Review

- **DRY**: the registry contract should live in one place. If routing order, scaffold defaults, and
  suite selection each re-derive chain3 knowledge differently, that is the exact bug M22 is
  supposed to remove.
- **Explicit over clever**: prefer a boring `Vec<&FamilyHarness>` or iterator-based helper over
  macro tricks or data-driven command templates.
- **Minimal diff**: do not introduce a second manifest format, codegen step, or custom DSL.
- **Diagram maintenance**: update nearby ASCII docs if code comments or README snippets imply the
  old self-bootstrapping story.

## Test Review

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] xtask/src/family/harness.rs
    │
    ├── family_harness()
    │   ├── [GAP] registered family lookup with 2+ entries
    │   └── [GAP] missing family still returns the honest registry-first error
    │
    └── registered_harnesses_in_routing_order()
        ├── [GAP] stable sort across 2+ families
        └── [GAP] duplicate/adjacent precedence expectations stay deterministic

[+] xtask/src/family/routing.rs
    │
    ├── locked_routing_order_with_terminal()
    │   └── [GAP] no one-family fixed-width assumption
    │
    ├── manifest_matches_locked_routing()
    │   ├── [GAP] second-family positive case
    │   └── [GAP] mismatch report contains correct family-specific expectation
    │
    └── manifest_routing_mismatch_message()
        └── [GAP] mismatch text stays useful for non-chain3 families

[+] xtask/src/family/scaffold.rs
    │
    ├── run()
    │   ├── [★★ TESTED] chain3 scaffold happy path already exists indirectly
    │   ├── [GAP] synthetic second registered family scaffold succeeds
    │   └── [★★ TESTED] unregistered family failure already exists and should be kept
    │
    └── manifest_template()
        └── [GAP] synthetic family uses its own routing values, not chain3 spillover

[+] xtask/src/family/prove.rs + certify.rs
    │
    ├── registry-selected suite execution
    │   ├── [★★★ TESTED] chain3 prove/certify still pass end-to-end
    │   └── [GAP] synthetic second family report/gate helpers do not assume chain3-only widths
    │
    └── routing gate messages
        └── [GAP] mismatch report remains correct for another registered family
```

### Operator-flow coverage

```text
OPERATOR FLOW COVERAGE
===========================
[+] Maintainer adds next family
    │
    ├── update Rust registry entry
    ├── run `cargo xtask family new <family>`
    ├── inspect generated packet
    └── [GAP] test the registry-first path with a synthetic family

[+] Maintainer forgets registration
    │
    └── [★★★ TESTED] gets explicit "family is not registered" error

[+] Maintainer changes routing metadata wrong
    │
    └── [GAP] certify mismatch output for a non-chain3 family remains actionable

[+] Maintainer reads docs first
    │
    └── [GAP] docs must say registry-first, not implied auto-bootstrap
```

### Required test split

- `xtask/src/lib.rs`
  - add synthetic second-family registry fixtures and unit tests
  - assert routing-order helpers no longer assume one family
  - assert scaffold/manifest templates use the selected family definition
- existing chain3 tests
  - keep the current chain3 harness contract tests
  - re-run `cargo xtask family prove function.wrapper.pipeline.chain3.v1`
  - re-run `cargo xtask family certify function.wrapper.pipeline.chain3.v1`
- docs smoke
  - at minimum, assert the unknown-family error message still points to the single registry edit

### Test plan artifact

Write the artifact at:
`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m21-eng-review-test-plan-20260427-213147.md`

### Verification loop

```text
1. cargo test -p xtask
2. cargo xtask family new function.wrapper.pipeline.chain4.v1
   expected: still fails before implementation, then succeeds once the M22 registry entry exists
3. cargo xtask family prove function.wrapper.pipeline.chain3.v1
4. cargo xtask family certify function.wrapper.pipeline.chain3.v1
5. rg -n "self-bootstrapping|repeatable|registry-first" semantic-families/README.md PLAN.md README.md
```

## Performance Review

- Do not add more Cargo subprocesses than M21 already runs. M22 is about selection truth, not more
  suites.
- Registry iteration cost is trivial. The only real performance footgun is accidentally widening
  prove/certify into workspace-scale discovery or duplicated suite execution. Do not do that.
- Keep report generation file-local and deterministic. No new caches are needed.

## Security and Trust Boundary Review

- `family.toml` stays validated input, not executable orchestration.
- Suite commands remain compile-time Rust constants, which keeps packet authors from steering what
  subprocesses run.
- Existing packet path safety and symlink rejection remain mandatory and unchanged.

## NOT in Scope

- Promoting a second real semantic family in `spec-core`
- Changing the public `spec` CLI
- Making `family.toml` self-bootstrapping or command-bearing
- Multi-kind family promotion (`data`, `sum`, or generic family abstraction)
- CI workflow redesign or artifact schema redesign

## Implementation Order

### M22a. Narrow the repo claim

- Rewrite `semantic-families/README.md` so the documented workflow is explicitly registry-first.
- Remove or update any phrasing in repo docs that implies packet creation works for arbitrary new
  families without registry work.

### M22b. Lock the registry contract

- Keep `FamilyHarness` as the one family-definition shape.
- Add helper functions in `harness.rs` for routing order and family-specific contract access.
- Make the registry the single edit site for family-specific xtask knowledge.

### M22c. Remove one-family assumptions

- Rewrite `routing.rs` helpers to iterate over registered families dynamically.
- Ensure scaffold/prove/certify consumers rely on the same registry access path.
- Remove fixed-width assumptions about `must_not_shadow` ordering where possible. Where not
  possible, make them explicit per family definition, not global chain3 assumptions.

### M22d. Add synthetic generalization tests

- In `xtask/src/lib.rs`, add test-only synthetic family definitions.
- Prove that lookup, ordering, scaffold templating, and routing mismatch reporting all behave
  correctly with 2+ registered families.
- Keep the live chain3 contract tests intact.

### M22e. Re-run the live proof

- `cargo test -p xtask`
- `cargo xtask family prove function.wrapper.pipeline.chain3.v1`
- `cargo xtask family certify function.wrapper.pipeline.chain3.v1`

## Failure Modes Registry

| Codepath | Realistic failure | Test covers it? | Error handling? | Operator outcome |
|---|---|---:|---:|---|
| `family_harness()` | next family missing from registry | yes, required | yes | clear, actionable failure |
| routing-order helper | second family renders wrong order or mismatch message | yes, required | partial until added | misleading certify failure if untested |
| scaffold template | synthetic family inherits chain3 routing values | yes, required | no | silently wrong generated packet |
| certify gate D | chain3 still passes but future family mismatch message is wrong | yes, required | partial | hard to debug certification failure |
| docs workflow | maintainer skips registry edit because docs overclaim | no automation, doc review only | no | wasted time and false confidence |

Critical gap rule: the scaffold-template spillover case is the one to fear. If a second family can
generate a packet with chain3 routing values silently, that is a correctness bug, not a docs bug.

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Narrow claim and registry contract | `semantic-families/`, `xtask/src/family/` | — |
| Remove one-family assumptions | `xtask/src/family/` | registry contract |
| Add synthetic generalization tests | `xtask/src/` | one-family assumption cleanup |
| Re-run live chain3 proof | `xtask/`, `spec-core/`, `spec-cli/` via commands | tests |

### Parallel lanes

- Lane A: narrow claim docs
- Lane B: registry contract + helper cleanup -> synthetic tests -> live proof

### Execution order

Launch Lane A after the registry contract is chosen. Lane B stays sequential because the same xtask
module owns the whole blast radius.

### Conflict flags

Lane A and Lane B both conceptually touch workflow wording. Keep docs edits small and rebase after
the xtask contract is settled.

## Green Gate

M22 is green only if all of these are true:

- docs say registry-first, not self-bootstrapping
- one-family assumptions are removed from helper code
- synthetic 2+ family xtask tests pass
- `cargo xtask family prove function.wrapper.pipeline.chain3.v1` passes
- `cargo xtask family certify function.wrapper.pipeline.chain3.v1` passes

## Red Gate

M22 is red if any of these are still true:

- a future family still requires edits outside one explicit registry contract
- routing helpers still assume registry length `== 1`
- docs still imply arbitrary new-family bootstrap is already automated
- chain3 proof regresses during hardening

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Do not ship a second real family in M22 | auto-decided | completeness + minimal diff | Hardening truth first keeps the blast radius small and the milestone honest | "prove generalization" by mixing in new semantic breadth |
| 2 | CEO | Keep orchestration truth in Rust registry | auto-decided | explicit over clever | The repo already has this pattern and it keeps command execution out of packet data | manifest-driven command orchestration |
| 3 | CEO | Keep `family.toml` as validation data only | auto-decided | boring by default | Safer trust boundary and smaller diff | packet-owned suite selection |
| 4 | Eng | Remove one-family helper assumptions | auto-decided | completeness | This is the concrete code smell behind the M22 premise | docs-only honesty fix |
| 5 | Eng | Prove generalization with synthetic xtask tests | auto-decided | pragmatic | Gives real evidence without shipping fake family breadth | shipping an incomplete second family |
| 6 | Eng | Keep chain3 as frozen live regression backstop | auto-decided | reversibility | Existing proof stays the canary while harness code changes | refactoring without end-to-end proof reruns |

## Completion Summary

- Step 0: Scope Challenge — scope reduced to harness hardening, docs honesty, and synthetic generalization proof
- Architecture Review: 4 issues found, all resolved in-plan
- Code Quality Review: 3 issues found, all resolved in-plan
- Test Review: coverage diagram written, 9 required gaps identified
- Performance Review: 2 issues found, both resolved in-plan
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 0 required for M22 plan approval
- Failure modes: 1 critical gap flagged
- Outside voice: skipped
- Parallelization: 2 lanes, 1 parallel / 3 sequential stages
- Lake Score: 6/6 recommendations chose the complete option inside the bounded blast radius

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | clear (manual) | integrity-first scope chosen, second real family deferred, docs claim narrowed |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | clear (manual) | registry-first contract, 9 test obligations, 1 critical gap, chain3 proof preserved |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | no UI scope |

**UNRESOLVED:** 0

**VERDICT:** CEO + ENG CLEARED (manual) — ready to implement M22 on `feat/m21`.
