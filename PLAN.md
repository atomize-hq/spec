<!-- /autoplan restore point: /home/azureuser/.gstack/projects/atomize-hq-spec/codex-i4-prep-autoplan-restore-20260520-014047.md -->
# I4: Rust V1 Command-Wall Fixture and Contract-Test Hardening Plan

Status: **authoritative implementation plan**
Iteration: **I4**
Milestone family: **Rust V1 contract-wall regression hardening**
Implementation readiness: **ready for implementation**
Plan scope: **freeze the shipped I3.5 machine contract behind deliberate golden fixtures and regression tests, without reopening benchmark mechanics, repo-root semantics, or Rust V1 support scope**
Base branch: **main**
Working branch: **codex/i4-prep**
Validated at commit: **`ede7fa7`**
Last rewritten: **2026-05-20**

Supersedes:

- the prior `I3: Rust V1 Contract Stack Mechanics Landing Plan` previously maintained at this path
- the prior `I3.5: Post-I3 Authority Alignment and Repo-Root Contract Freeze Plan` snapshot as the active work plan

Locked authority inputs:

- contract-stack index: `docs/rust_v1_contract_stack.md`
- I3.5 authority snapshot: `.runs/i3_5_authority_alignment/authority-plan.snapshot.md`
- I3.5 freeze record: `.runs/i3_5_authority_alignment/phase2-freeze.json`
- I3.5 final merged proof wall: `.runs/i3_5_authority_alignment/validation/final-main/**`
- alignment design: `/home/azureuser/.gstack/projects/atomize-hq-spec/azureuser-main-design-20260519-145148.md`
- `M65`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-200036.md`
- `M66`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-213928.md`
- `M67`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-220646.md`
- `M68`: `/home/azureuser/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m60-plus-design-20260517-225503.md`

Historical context, not authority:

- `README.md`
- `CHANGELOG.md`
- `TODOS.md`
- `ORCH_PLAN.md`
- `.runs/i3_rust_v1_contract_stack_mechanics/**`

Primary repo surfaces:

- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/benchmarks/*.json`
- `spec-cli/src/commands.rs`
- `spec-core/src/export.rs`
- `docs/rust_v1_contract_stack.md`
- `.runs/i3_5_authority_alignment/validation/final-main/*.stdout`

## Executive Summary

I3 landed benchmark mechanics.

I3.5 froze what the public command wall means:

- benchmark-root `status` is the proof wall
- benchmark-root `export` is the export wall
- namespace and single-file `status` are partial diagnostic surfaces
- repo-root `status . --format json` is supported, but only as `inventory_only`
- repo-root `export .` is unsupported and must fail with `SPEC_UNSUPPORTED_SCOPE`

I4 does not add a new behavior surface.

I4 turns that frozen wall into a deliberate regression boundary. Today the repo
already has meaningful CLI coverage, but it still locks too much by fragments
and implication:

1. selected JSON fragments are locked, but the full command outputs are not
2. the final I3.5 proof wall is archived under `.runs/`, not promoted into one
   maintained in-repo contract suite
3. single-file `status` is part of the frozen wall, but not yet represented as
   a first-class checked-in fixture

That is the whole I4 problem. If implementation grows beyond that, it is scope
creep.

## Frozen Decisions

These decisions are inherited from `M65` through `M68`, the I3 landing, and the
I3.5 freeze. I4 implements them. It does not reopen them.

1. **I4 does not widen Rust V1 support.**
   - `M66` still owns support rows and interactions.
   - `BENCH-SERVICE` stays reserved and unimplemented.

2. **I4 does not redesign benchmark mechanics.**
   - `M67` still owns benchmark roles.
   - `M68` still owns benchmark artifact shapes, path-scope rules, and
     anti-laundering behavior.

3. **The frozen I3.5 command wall is the only contract I4 hardens.**
   - `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
   - `cargo run -p spec-cli -- export examples/ecommerce/units`
   - `cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json`
   - `cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json`
   - `cargo run -p spec-cli -- status . --format json`
   - `cargo run -p spec-cli -- export .`

4. **I4 is a test-and-fixture milestone first.**
   - Product-code edits are allowed only when needed to stabilize an already
     frozen machine contract.
   - New semantics are out of scope.

5. **Golden fixtures must encode command truth, not incidental noise.**
   - Absolute paths, wall-clock timestamps, and commit SHAs may be normalized.
   - Contract fields, enum values, scope-authority signals, benchmark
     classifications, and error codes may not be normalized away.

6. **The authoritative source for full-command baselines is the I3.5 final-main proof wall.**
   - `.runs/i3_5_authority_alignment/validation/final-main/*.stdout` is the
     seed truth for I4 fixture authoring.
   - I4 promotes that truth into maintained checked-in fixtures.

7. **Repo-root export stays unsupported.**
   - I4 does not add aggregate export behavior.
   - I4 only locks the existing unsupported-scope response harder.

8. **Namespace and single-file status stay partial.**
   - I4 must prove they never mint whole-benchmark positive credit.
   - I4 must prove they omit full-scope-only surfaces like projection digests
     and readability verdicts.

9. **This is a CLI contract hardening milestone, not a docs milestone.**
   - Docs already teach the frozen wall after I3.5.
   - I4 touches docs only if a test-driven ambiguity proves the shipped wording
     incomplete.

10. **Minimal diff wins.**
    - Prefer a tighter fixture harness over new abstractions.
    - Prefer test-local helpers over product refactors.

## Current Validated Basis

Observed repo truth on `main` at `ede7fa7`:

- I3 benchmark mechanics are merged
- I3.5 repo-root contract freeze is merged
- `spec-cli/src/commands.rs` emits `scope_authority: "inventory_only"` at repo
  root status scope
- `spec-cli/src/commands.rs` emits `status: "unsupported_scope"` and
  `errors[0].code == "SPEC_UNSUPPORTED_SCOPE"` for repo-root export
- `spec-cli/tests/cli.rs` already has benchmark-contract tests for:
  - repo-root `status`
  - benchmark-root `status`
  - namespace partial `status`
  - benchmark-root `export`
  - single-file partial `export`
  - repo-root unsupported `export`
  - benchmark snapshot behavior
- `.runs/i3_5_authority_alignment/validation/final-main/` already contains the
  exact merged proof outputs for:
  - benchmark-root `status`
  - namespace `status`
  - single-file `status`
  - repo-root `status`
  - benchmark-root `export`
  - repo-root `export`

The gap is not "we have no tests."

The gap is "the frozen command wall is not yet promoted into one exact,
maintained golden contract suite."

## Step 0: Scope Challenge

### Premise correction

The problem is not "do more Rust V1 work."

The problem is narrower:

```text
the repo already ships the I3.5 command wall,
but the regression suite still locks it by fragments and implication
instead of by one deliberate golden contract boundary
```

If I4 expands beyond that sentence, it is overbuilt.

### What already exists

| Sub-problem | Existing owner | I4 action |
| --- | --- | --- |
| benchmark mechanics and path-scope semantics | `spec-cli/src/commands.rs`, `spec-core/src/export.rs`, `spec-core/src/benchmark.rs` | reuse; do not redesign |
| authoritative final command outputs | `.runs/i3_5_authority_alignment/validation/final-main/*.stdout` | reuse as fixture seed truth |
| benchmark fixture corpus | `spec-cli/tests/fixtures/benchmarks/*.json` | extend in place; do not replace with a second fixture system |
| integration test harness | `spec-cli/tests/cli.rs` | tighten and isolate contract-wall assertions |
| repo-root unsupported export contract | `spec-cli/src/commands.rs` | preserve; add stronger golden coverage |
| repo-root `inventory_only` status signal | `spec-cli/src/commands.rs` | preserve; add stronger golden coverage |
| snapshot coverage | `spec-cli/tests/cli.rs` | leave intact; not part of the new I4 acceptance wall |

### Minimum complete slice

The minimum honest I4 slice is:

1. define the exact I4 command roster from the frozen I3.5 wall
2. map each frozen command to one checked-in fixture and one named regression test
3. add stable normalization helpers for nondeterministic fields
4. compare normalized full command outputs, not just selected fragments
5. explicitly add the missing single-file `status` golden wall
6. preserve targeted invariant assertions where a full-file diff alone is too opaque
7. document fixture authoring and regeneration so the next contributor can refresh
   the suite without guessing

Anything smaller is fake done.

Examples:

- adding a single-file status test without full-fixture promotion is fake done
- comparing only `benchmarks[]` while leaving `roots[]`, `units[]`, and
  loader-error surfaces free to drift is fake done
- copying proof outputs into fixtures without normalization rules is fake done
- adding new product semantics to make tests easier is fake done

### Complexity check

This milestone should stay below the "new subsystem" threshold.

Expected write scope:

- one integration test surface, `spec-cli/tests/cli.rs`
- one fixture tree, `spec-cli/tests/fixtures/benchmarks/`
- zero or one tiny product-code stabilization edit only if a nondeterministic
  output detail blocks truthful fixture locking

If implementation starts touching broad `spec-core` semantics, benchmark
projection logic, or docs beyond a narrow clarification, the milestone has
escaped its lane.

### Search check

Search unavailable for external advice, so this plan stays anchored in live repo
truth and existing Rust CLI test architecture.

In-repo first-principles conclusions:

- **[Layer 1]** reuse the I3.5 final-main stdout artifacts as the starting
  golden source
- **[Layer 1]** reuse the existing CLI integration harness instead of creating a
  new test binary or external snapshot framework
- **[Layer 3]** normalize only unstable transport noise, not actual contract
  fields
- **[EUREKA]** the real risk is not missing assertions, it is asserting only the
  easy fragments and leaving the true command wall implicit

### TODOS cross-reference

`TODOS.md` already carries `M69` and broader Rust follow-ons.

I4 should not add new product TODOs unless implementation discovers one of
these truths:

- the fixture harness wants a reusable extraction that is too large for I4
- the command wall still contains an ambiguity that cannot be frozen without a
  follow-on product decision

If neither happens, I4 should close without new TODO debt.

### Completeness and distribution check

This milestone introduces no new user-visible artifact type.

Completeness here means:

- every frozen I3.5 status/export command has a deliberate regression surface
- the regression surface is maintained in-repo, not only archived in `.runs/`
- the suite fails on contract drift, not just on obvious benchmark-array breaks

That is the whole game.

### Frozen command roster

This is the authoritative I4 roster. Each row must end with one checked-in
fixture and one dedicated assertion path.

| Surface | Command | Contract role | Fixture path | Dedicated test obligation | Required explicit invariant |
| --- | --- | --- | --- | --- | --- |
| benchmark-root `status` | `cargo run -p spec-cli -- status examples/ecommerce/units --format json` | proof wall | `spec-cli/tests/fixtures/benchmarks/status-ecommerce-full.json` | compare full normalized JSON | benchmark-root remains full-scope positive surface |
| benchmark-root `export` | `cargo run -p spec-cli -- export examples/ecommerce/units` | export wall | `spec-cli/tests/fixtures/benchmarks/export-ecommerce-full.json` | compare full normalized JSON | exported bundle shape remains benchmark-root scoped |
| namespace `status` | `cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json` | partial diagnostic surface | `spec-cli/tests/fixtures/benchmarks/status-ecommerce-pricing-partial-full.json` | compare full normalized JSON | partial scope never counts as supported positive |
| single-file `status` | `cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json` | partial diagnostic surface | `spec-cli/tests/fixtures/benchmarks/status-apply-discount-partial-full.json` | compare full normalized JSON | partial scope omits full-scope-only benchmark projection surfaces |
| repo-root `status` | `cargo run -p spec-cli -- status . --format json` | inventory-only supported surface | `spec-cli/tests/fixtures/benchmarks/status-repo-root-full.json` | compare full normalized JSON | `scope_authority == "inventory_only"` |
| repo-root `export` | `cargo run -p spec-cli -- export .` | unsupported surface | `spec-cli/tests/fixtures/benchmarks/export-repo-root-unsupported-scope.json` | compare full normalized JSON | `errors[0].code == "SPEC_UNSUPPORTED_SCOPE"` |

## Architecture Review

### System design

I4 is a thin test-hardening layer on top of the shipped command wall.

No new runtime architecture.
No new benchmark subsystem.
No second source of truth.

The desired shape is:

```text
I3.5 final-main proof outputs
        |
        v
fixture authoring pass
        |
        +--> normalization policy (documented once)
        |
        v
checked-in golden JSON fixtures
        |
        v
dedicated command-wall tests in spec-cli/tests/cli.rs
        |
        v
cargo test -p spec-cli blocks accidental command drift
```

### Dependency graph

```text
spec-cli/src/commands.rs
        |
        +---- status JSON surface -------------------------------+
        |                                                       |
        +---- export JSON surface ---------------------------+   |
                                                            |   |
spec-cli/tests/cli.rs                                       |   |
        |                                                   |   |
        +---- normalize_status_contract_json() -------------+---+
        +---- normalize_export_contract_json() -------------+---+
        +---- assert_contract_matches_fixture() ------------+---+
        |
        +---- checked-in fixtures under spec-cli/tests/fixtures/benchmarks/
        |
        +---- seeded from .runs/i3_5_authority_alignment/validation/final-main/
```

### Architecture recommendation

Keep helpers inside `spec-cli/tests/cli.rs` in one clearly labeled I4 contract
section.

That is the minimal-diff choice. Do not create a new test framework, a new
integration binary, or a general-purpose snapshot abstraction for one milestone.

Product code stays untouched unless all three conditions are true:

1. the observed output differs from the frozen I3.5 proof wall
2. the difference is nondeterministic transport noise, not semantic drift
3. the noise cannot be normalized truthfully in test code

If any product-code edit is needed, the commit message and test comment must say
which frozen contract it preserves.

### Production failure scenarios

| Surface | Realistic failure | Planned guard | Critical if missing? |
| --- | --- | --- | --- |
| repo-root `status` | `scope_authority` disappears or changes wording | full normalized fixture plus explicit equality assertion | yes |
| repo-root `export` | command starts returning an aggregate bundle instead of an unsupported error | full normalized fixture plus explicit error-code assertion and no-success-shape check | yes |
| namespace `status` | partial scope starts minting positive credit | full normalized fixture plus explicit non-credit assertions | yes |
| single-file `status` | command shape drifts silently because only benchmark-root paths are covered | new full normalized fixture and dedicated test | yes |
| benchmark-root `status` | `roots[]` or `units[]` drift while `benchmarks[]` still passes | full normalized benchmark-root fixture | yes |
| benchmark-root `export` | top-level export shape drifts while benchmark arrays still pass | full normalized export fixture | yes |

## Code Quality Review

### DRY targets

The existing benchmark contract tests are useful, but too much knowledge is
duplicated across one-off assertions. I4 should remove repetition in exactly
three places:

1. normalization of unstable fields
2. fixture loading and JSON comparison
3. benchmark fixture repo setup for the command-wall tests

### Opinionated helper surface

Use one small contract helper layer, no more:

- `read_contract_fixture(path: &str) -> serde_json::Value`
- `normalize_status_contract_json(value: &mut serde_json::Value)`
- `normalize_export_contract_json(value: &mut serde_json::Value)`
- `assert_contract_matches_fixture(actual: &serde_json::Value, fixture_path: &str)`

This is explicit, DRY enough, and still a minimal diff.

Do not add generic trait plumbing, macro-driven assertion wrappers, or a shared
test-support crate for I4. That is spending an innovation token to compare JSON.
Wild.

### Normalization contract

These are the only fields I4 may rewrite before fixture comparison:

- absolute filesystem paths
- unit and molecule `evidence_at` timestamps
- `exported_at`
- `provenance.git_commit_sha`
- digest literals whose value is expected to churn across truthful proof refreshes:
  - `freshness.authored_truth_digest`
  - benchmark `label_digest`
  - benchmark `projection_digest`

These fields must remain exact:

- `schema_version`
- `status`
- `scope_authority`
- `errors[].code`
- benchmark ids and benchmark classifications
- path-scope labels
- positive-credit signals
- relative proof-ref paths
- benchmark presence versus absence

### Fixture authoring workflow

Fixture authoring is part of the plan. The implementer should not guess.

1. capture the raw I3.5 proof outputs from
   `.runs/i3_5_authority_alignment/validation/final-main/*.stdout`
2. map each file to the frozen command roster above
3. convert the raw JSON into its checked-in fixture path
4. normalize only the allowed transport-noise fields
5. keep the original raw proof output untouched under `.runs/`
6. use the same normalization rules in the regression tests so authoring and
   verification match exactly

### ASCII diagram comment

Add one short ASCII comment near the helper entrypoint in `spec-cli/tests/cli.rs`:

```text
raw command JSON
   -> normalize unstable fields
   -> compare against golden fixture
   -> assert critical invariants not obvious from diff
```

That comment belongs in test code, not product code.

## Test Review

100% coverage is the point of this milestone.

The changed behavior is not user interaction. It is command-contract stability.
So every frozen status/export surface needs a deliberate test.

### Command-wall coverage diagram

```text
COMMAND-WALL COVERAGE
===============================
[+] benchmark-root status
    |
    ├── [PARTIAL TODAY] benchmarks[] fixture locked
    └── [GAP] full normalized stdout contract not locked

[+] benchmark-root export
    |
    ├── [PARTIAL TODAY] benchmarks[] fixture locked
    └── [GAP] full normalized stdout contract not locked

[+] namespace status
    |
    ├── [PARTIAL TODAY] partial benchmarks[] fixture locked
    ├── [PARTIAL TODAY] loader-error and no-positive-credit assertions exist
    └── [GAP] full normalized stdout contract not locked

[+] single-file status
    |
    ├── [GAP] no dedicated frozen fixture coverage in the command-wall suite
    └── [GAP] partial benchmark semantics not promoted from I3.5 proof output

[+] repo-root status
    |
    ├── [PARTIAL TODAY] schema_version and scope_authority contract locked
    ├── [PARTIAL TODAY] benchmarks[] fixture locked
    └── [GAP] full normalized stdout contract not locked

[+] repo-root export
    |
    ├── [GOOD TODAY] unsupported-scope fixture exists
    └── [GAP] not yet treated as part of one unified command-wall suite

─────────────────────────────────
COVERAGE: 1/6 fully locked
  fully locked surfaces: repo-root export
  partially locked surfaces: 4
  missing surfaces: 1
GAPS: 5 command-wall promotions needed
─────────────────────────────────
```

### Required fixture set

I4 must end with exactly this maintained fixture set:

- `spec-cli/tests/fixtures/benchmarks/status-repo-root-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-ecommerce-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-ecommerce-pricing-partial-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-apply-discount-partial-full.json`
- `spec-cli/tests/fixtures/benchmarks/export-ecommerce-full.json`
- `spec-cli/tests/fixtures/benchmarks/export-repo-root-unsupported-scope.json`

The existing repo-root unsupported export fixture may keep its current filename
if that avoids churn, but the test names and plan language must make its role
unmistakable.

### Required test set

Add or tighten one dedicated regression test per frozen surface:

| Surface | Required test behavior |
| --- | --- |
| benchmark-root `status` | run command, normalize full JSON, compare to `status-ecommerce-full.json`, assert full benchmark-root semantics still present |
| benchmark-root `export` | run command, normalize full JSON, compare to `export-ecommerce-full.json`, assert export wall remains benchmark-root scoped |
| namespace `status` | run command, normalize full JSON, compare to `status-ecommerce-pricing-partial-full.json`, assert no positive credit |
| single-file `status` | run command, normalize full JSON, compare to `status-apply-discount-partial-full.json`, assert omission of full-scope-only projection surfaces |
| repo-root `status` | run command, normalize full JSON, compare to `status-repo-root-full.json`, assert `scope_authority == "inventory_only"` |
| repo-root `export` | run command, normalize full JSON, compare to `export-repo-root-unsupported-scope.json`, assert `SPEC_UNSUPPORTED_SCOPE` |

### Failure-mode matrix

| Command surface | Test covers failure? | Error handling exists? | User gets clear signal? | Critical gap if missing? |
| --- | --- | --- | --- | --- |
| repo-root export unsupported scope | yes, must | yes | yes | yes |
| repo-root inventory-only status | yes, must | yes | yes | yes |
| namespace partial no-credit behavior | yes, must | yes | yes | yes |
| single-file partial status semantics | currently no | yes in product code | yes | yes |
| benchmark-root full contract shape | currently partial | yes | yes | yes |

Any row above without a full golden test is an I4 blocker.

## Performance Review

I4 should not materially affect runtime performance.

This is almost entirely test code and checked-in JSON. The only performance smell
to avoid is doing extra benchmark fixture repo setup or reparsing in ways that
slow `cargo test -p spec-cli` for no benefit.

Recommendations:

- reuse the existing temporary benchmark-repo copy strategy
- parse and normalize once per command assertion path
- do not add a caching layer, a separate fixture compiler, or other "clever"
  speedups for a suite this small

## Implementation Plan

### Phase 1: Freeze the roster and policy

1. confirm the six-command roster in this plan against the I3.5 proof wall
2. map each command to one fixture path and one test name
3. document the allowed normalization fields in the helper comment block
4. lock the rule that `.runs/.../final-main/*.stdout` is seed truth, not the
   runtime assertion source

Done when:

- the command roster table above needs no further interpretation
- the fixture names are final
- the normalization policy is written before fixtures are copied

### Phase 2: Build the contract helpers

1. add the small test-local helper surface in `spec-cli/tests/cli.rs`
2. keep separate entrypoints for status JSON and export JSON normalization
3. add the short ASCII fixture-flow comment above the helper block
4. reuse existing benchmark repo setup instead of inventing new scaffolding

Done when:

- one test can load a fixture and compare a normalized full command output
- no helper strips real contract fields

### Phase 3: Promote the fixtures

1. create or refresh the five missing full-command fixtures
2. keep the repo-root unsupported export fixture deliberate and named clearly
3. verify each fixture is derived from final-main proof output, not hand-authored
   from memory

Done when:

- all six fixture paths exist
- each fixture reflects the normalization contract exactly once

### Phase 4: Tighten the regression tests

1. add one dedicated command-wall test per frozen surface
2. compare normalized full JSON to the exact fixture
3. preserve explicit invariant assertions where a full-file diff would be hard to
   read:
   - repo-root `scope_authority == "inventory_only"`
   - repo-root export `errors[0].code == "SPEC_UNSUPPORTED_SCOPE"`
   - namespace and single-file partial cases never count as supported positive
   - partial status omits full-scope-only projection surfaces
4. keep snapshot and invalid-registry tests intact as adjacent coverage, but do
   not let them substitute for the I4 wall

Done when:

- every row in the frozen command roster has a named test
- no surface relies on fragment-only assertions anymore

### Phase 5: Prove the wall

Run exactly:

```bash
cargo test -p spec-cli
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export .
```

Acceptance requires:

- the test suite passes
- all full-command fixtures match after normalization
- repo-root export still fails with `SPEC_UNSUPPORTED_SCOPE`
- repo-root status still emits `scope_authority: "inventory_only"`
- namespace and single-file status still remain partial and non-crediting

## Concrete File Plan

Expected write set:

- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/benchmarks/*.json`

Conditional write set, only if forced by the blocker rule:

- `spec-cli/src/commands.rs`
  - allowed only for stability fixes that preserve the frozen contract
- `spec-core/src/export.rs`
  - allowed only if the same frozen contract cannot be expressed without a tiny
    stabilization fix here instead of in CLI glue

Read-only authority inputs:

- `.runs/i3_5_authority_alignment/validation/final-main/*.stdout`
- `docs/rust_v1_contract_stack.md`
- `.runs/i3_5_authority_alignment/phase2-freeze.json`

## NOT in Scope

- new Rust V1 support rows or interaction claims
- `BENCH-SERVICE` implementation or service workload authoring
- benchmark roster changes
- benchmark snapshot command redesign
- repo-root aggregate export support
- schema-version bump
- doc rewrite beyond narrow clarification forced by test evidence
- new benchmark scoring, history, or review workflows
- large test-framework extraction across unrelated CLI surfaces

## What Already Exists

- full I3.5 proof outputs already exist under
  `.runs/i3_5_authority_alignment/validation/final-main/`
- repo-root unsupported export fixture already exists in
  `spec-cli/tests/fixtures/benchmarks/export-repo-root-unsupported-scope.json`
- benchmark-root and namespace benchmark-array fixtures already exist
- benchmark-repo copy harness already exists in `copy_benchmark_repo_fixture()`
- product code for `inventory_only` and `SPEC_UNSUPPORTED_SCOPE` is already live

I4 reuses all of that. It should not rebuild any of it in parallel.

## Parallelization Strategy

This milestone has two real workstreams and one conditional escape hatch.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. fixture promotion | `spec-cli/tests/fixtures/benchmarks/` | — |
| B1. helper scaffolding | `spec-cli/tests/` | — |
| B2. final test wiring | `spec-cli/tests/` | A and B1 |
| C. optional stability fix | `spec-cli/src/`, `spec-core/src/` | B2 only if tests expose real nondeterminism |

### Parallel lanes

Lane A: fixture promotion
- copy and normalize the final-main proof outputs into the six maintained fixture paths
- finalize placeholder values and fixture naming

Lane B: test harness tightening
- B1 can start immediately with helper scaffolding, test naming, and the shared
  assertion flow
- B2 starts after Lane A finalizes fixture names and placeholder policy

Lane C: optional runtime stabilization
- only launch if Lane B proves a truthful contract cannot be asserted without a
  tiny product-code stabilization change

### Execution order

1. Launch Lane A and Lane B1 in parallel worktrees.
2. Merge on final fixture names and normalization policy.
3. Finish Lane B2 against the finalized fixture corpus.
4. Launch Lane C only if B2 finds a real nondeterministic output blocker.
5. Run the full proof wall after A+B, and again after C if C exists.

### Conflict flags

- Lane A touches `spec-cli/tests/fixtures/benchmarks/`; Lane B touches
  `spec-cli/tests/cli.rs`. That is a safe parallel split until the merge point.
- B1 and B2 stay in the same lane because both touch `spec-cli/tests/`.
- Lane C touches product code and should stay isolated unless tests force it.
- If Lane C is activated, do not keep editing fixture names in parallel. Freeze
  the test side first.

## Verification Commands

Authoritative verification commands for implementation closeout:

```bash
cargo test -p spec-cli
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
cargo run -p spec-cli -- status . --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- export .
```

## Acceptance Checklist

- [ ] full normalized fixture exists for benchmark-root `status`
- [ ] full normalized fixture exists for namespace `status`
- [ ] full normalized fixture exists for single-file `status`
- [ ] full normalized fixture exists for repo-root `status`
- [ ] full normalized fixture exists for benchmark-root `export`
- [ ] repo-root unsupported export fixture remains deliberate and explicit
- [ ] one dedicated regression test exists per frozen command-wall surface
- [ ] normalization helpers are documented, narrow, and test-local
- [ ] partial scopes never count as supported positive credit
- [ ] repo-root status still emits `scope_authority: "inventory_only"`
- [ ] repo-root export still fails with `SPEC_UNSUPPORTED_SCOPE`
- [ ] `cargo test -p spec-cli` passes

## Completion Summary

- Step 0: Scope Challenge — scope accepted as the full I3.5 command-wall
  hardening slice, not a new mechanics milestone
- Architecture Review: 1 core architecture direction, keep runtime code boring
- Code Quality Review: 3 DRY targets, 0 justification for a new subsystem
- Test Review: command-wall diagram produced, 5 contract-promotion gaps
  identified
- Performance Review: no runtime-risk work expected
- NOT in scope: written
- What already exists: written
- Failure modes: 5 critical contract gaps flagged
- Parallelization: 3 lanes, 2 parallel / 1 conditional sequential
- Lake Score: 8/8 recommendations chose the complete option
