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

I4 does not invent anything new.

Its only job is to make that frozen contract hard to regress accidentally.

Today the repo already has meaningful coverage in `spec-cli/tests/cli.rs`, but
that coverage is still too soft in three ways:

1. it locks only selected fragments of the frozen JSON contract instead of the
   whole command-wall shape
2. the full I3.5 acceptance wall is not represented as one deliberate,
   maintainable golden suite
3. single-file `status` is part of the frozen wall in I3.5 proof artifacts, but
   it is not promoted to an equally explicit checked-in contract fixture

That is the whole I4 problem.

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
   - Contract fields, enum values, scope-authority signals, benchmark visibility,
     and error codes may not be normalized away.

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
   - I4 only touches docs if a test-driven ambiguity proves the shipped wording
     is incomplete.

10. **Minimal diff wins.**
   - Prefer a tighter fixture harness over new abstractions.
   - Prefer dedicated contract helpers over spreading more ad hoc assertions
     through an already-large `cli.rs`.

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

The remaining gap is not “we have no tests.”

The gap is “the frozen command wall is not yet promoted into one exact,
maintained golden contract suite.”

## Step 0: Scope Challenge

### Premise correction

The problem is not “do more Rust V1 work.”

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
| benchmark fixture corpus | `spec-cli/tests/fixtures/benchmarks/*.json` | extend and reorganize; do not replace with a second fixture system |
| integration test harness | `spec-cli/tests/cli.rs` | tighten and isolate contract-wall assertions |
| repo-root unsupported export contract | `spec-cli/src/commands.rs` | preserve; add stronger golden coverage |
| repo-root `inventory_only` status signal | `spec-cli/src/commands.rs` | preserve; add stronger golden coverage |
| snapshot coverage | `spec-cli/tests/cli.rs` | leave intact; not part of the new I4 acceptance wall |

### Minimum complete slice

The minimum honest I4 slice is:

1. define the exact I4 command-wall fixture roster from the frozen I3.5 wall
2. add stable normalization helpers for nondeterministic fields
3. add full-command golden fixtures for each frozen status/export surface
4. wire dedicated regression tests that compare normalized full outputs, not
   just selective fragments
5. explicitly add the missing single-file `status` golden wall
6. preserve targeted assertions for critical semantic invariants that full-file
   comparison alone does not explain clearly
7. document the acceptance command wall and fixture regeneration workflow in the
   plan and test comments so later contributors know what is intentional

Anything smaller is fake done.

Examples:

- adding a single-file status test without full-fixture promotion is fake done
- comparing only `benchmarks[]` while leaving `roots[]`, `units[]`, and
  loader-error surfaces free to drift is fake done
- copying proof outputs into fixtures without normalization rules is fake done
- adding new product semantics to make tests easier is fake done

### Complexity check

This milestone should stay below the “new subsystem” threshold.

Expected write scope:

- one integration test surface
- one benchmark fixture tree
- optionally one small shared test helper area
- zero or one tiny product-code stabilization edits only if a nondeterministic
  output detail blocks truthful fixture locking

If implementation starts touching broad `spec-core` semantics, benchmark
projection logic, or docs beyond narrow clarifications, the milestone has
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
these two truths:

- the fixture harness requires a reusable test-support extraction that is too
  large for I4
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

## Architecture Review

### System design

I4 should be a thin test-hardening layer on top of the shipped command wall.

No new runtime architecture.
No new benchmark subsystem.
No second source of truth.

The desired shape is:

```text
I3.5 final-main proof outputs
        |
        v
fixture seed + normalization policy
        |
        v
checked-in golden JSON fixtures
        |
        v
dedicated CLI contract-wall tests
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
spec-cli/tests/cli.rs or split contract test module         |   |
        |                                                   |   |
        +---- normalization helpers ------------------------+---+
        |
        +---- checked-in fixtures under spec-cli/tests/fixtures/benchmarks/
        |
        +---- seeded from .runs/i3_5_authority_alignment/validation/final-main/
```

### Architecture recommendation

Keep the runtime code boring.

If a helper is needed, it belongs in test code unless an unstable output field
cannot be normalized externally. The implementation should assume the current
command semantics are correct and should treat product-code edits as exceptions
that need a written reason.

### Production failure scenarios

| Surface | Realistic failure | Does I4 need to catch it? | Planned guard |
| --- | --- | --- | --- |
| repo-root `status` | `scope_authority` disappears or changes wording | yes | full normalized repo-root status fixture + explicit assertion |
| repo-root `export` | command starts dumping aggregate bundle again | yes | full normalized unsupported-scope fixture + explicit no-`benchmarks`/`units` assertion |
| namespace `status` | partial scope starts minting positive credit | yes | full normalized namespace fixture + explicit `counts_as_supported_positive == false` assertions |
| single-file `status` | command shape drifts without fixture coverage | yes | new full normalized single-file status fixture |
| benchmark-root `status` | roots/units payload shape drifts while `benchmarks[]` still passes | yes | full normalized benchmark-root status fixture |
| benchmark-root `export` | top-level export shape drifts while `benchmarks[]` still passes | yes | full normalized benchmark-root export fixture |

## Code Quality Review

### DRY and module structure

The current benchmark contract tests are good, but they are spread across
one-off assertions inside a very large integration file.

That is fine for I3 landing energy. It is not the best steady-state shape for a
frozen command wall.

I4 should aggressively remove repetition in three places:

1. normalization of unstable fields
2. fixture loading for full command outputs
3. benchmark-fixture repo setup for the command-wall tests

### Opinionated recommendation

Do not invent a large new test framework.

Add one small contract helper layer, just enough to make these tests obvious in
30 seconds:

- `read_contract_fixture(...)`
- `normalize_status_contract_json(...)`
- `normalize_export_contract_json(...)`
- `assert_contract_matches_fixture(...)`

That is explicit, DRY enough, and still a minimal diff.

### ASCII diagrams in code comments

If I4 extracts contract helpers or a dedicated contract-test block, add one
short ASCII diagram comment near the helper entrypoint so future contributors
know the fixture flow:

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
    ├── [PARTIAL TODAY] loader error + no positive-credit assertions locked
    └── [GAP] full normalized stdout contract not locked

[+] single-file status
    |
    ├── [GAP] no dedicated frozen fixture coverage in the benchmark contract suite
    └── [GAP] partial benchmark semantics not promoted from I3.5 proof output

[+] repo-root status
    |
    ├── [PARTIAL TODAY] schema_version + scope_authority contract locked
    ├── [PARTIAL TODAY] benchmarks[] fixture locked
    └── [GAP] full normalized stdout contract not locked

[+] repo-root export
    |
    ├── [GOOD TODAY] machine-readable unsupported-scope response fixture exists
    └── [GAP] treat as part of one unified command-wall suite, not a standalone special case

─────────────────────────────────
COVERAGE: 1/6 fully locked
  fully locked surfaces: repo-root export
  partially locked surfaces: 4
  missing surfaces: 1
GAPS: 5 command-wall promotions needed
─────────────────────────────────
```

### Test artifact plan

I4 should add or promote these checked-in fixtures:

- `spec-cli/tests/fixtures/benchmarks/status-repo-root-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-ecommerce-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-ecommerce-pricing-partial-full.json`
- `spec-cli/tests/fixtures/benchmarks/status-apply-discount-partial-full.json`
- `spec-cli/tests/fixtures/benchmarks/export-ecommerce-full.json`
- `spec-cli/tests/fixtures/benchmarks/export-repo-root-unsupported-scope.json`
  - existing fixture may be kept and renamed only if that reduces ambiguity

Normalization rules for those fixtures:

- replace absolute filesystem paths with stable sentinels
- replace unit and molecule `evidence_at` timestamps with a stable placeholder
- replace `exported_at` with a stable placeholder
- replace `provenance.git_commit_sha` with a stable placeholder
- replace these digest literals with stable placeholders while preserving field
  presence and absence semantics:
  - `freshness.authored_truth_digest`
  - benchmark `label_digest`
  - benchmark `projection_digest`
- keep relative proof-ref paths, benchmark ids, classifications, statuses,
  loader-error codes, and scope labels exact
- do not strip `scope_authority`, `status`, `errors[].code`, benchmark ids,
  path-scope labels, gate statuses, or positive-credit flags

### Test types

| Surface | Test type | Why |
| --- | --- | --- |
| benchmark-root status/export | integration fixture | command contract, not unit logic |
| namespace status | integration fixture | partial-scope contract must stay honest |
| single-file status | integration fixture | frozen diagnostic contract needs explicit regression wall |
| repo-root status/export | integration fixture | public machine surface with strict semantics |
| snapshot command | existing regression coverage only | not part of the I4 frozen wall |

### Failure-mode matrix

| Command surface | Test covers failure? | Error handling exists? | User gets clear signal? | Critical gap if missing? |
| --- | --- | --- | --- | --- |
| repo-root export unsupported scope | yes, must | yes | yes | yes |
| repo-root inventory-only status | yes, must | yes | yes | yes |
| namespace partial no-credit behavior | yes, must | yes | yes | yes |
| single-file partial status semantics | currently no | yes in product code | yes | yes |
| benchmark-root full contract shape | currently partial | yes | yes | yes |

Any surface above without a full golden test is an I4 blocker.

## Performance Review

I4 should not materially affect runtime performance.

This is almost entirely test code and checked-in JSON.

The only performance smell to avoid is rebuilding large copied fixtures more
than necessary inside tests. Reuse the existing temporary benchmark-repo copy
strategy unless it becomes a measurable test bottleneck. Do not prematurely
optimize with a custom caching layer.

## Implementation Plan

### Phase 1: Freeze the fixture roster

1. Enumerate the exact I3.5 command-wall commands that belong to I4.
2. Map each command to one maintained checked-in fixture file.
3. Decide the normalization policy field-by-field and write it down in test
   comments next to the helpers.
4. Keep `.runs/i3_5_authority_alignment/validation/final-main/*.stdout` as the
   seed truth, not as the asserted runtime source.

### Phase 2: Build the normalization helpers

1. Add small test-only helpers to normalize:
   - absolute paths
   - exported timestamps
   - commit SHAs
   - any non-contract digests that would churn with proof refreshes
2. Keep separate normalization entrypoints for status JSON and export JSON if
   that avoids clever conditionals.
3. Avoid mutating away contract fields that later reviewers will need to see in
   fixture diffs.

### Phase 3: Promote the command-wall fixtures

1. Check in full normalized benchmark-root status fixture.
2. Check in full normalized namespace status fixture.
3. Check in full normalized single-file status fixture.
4. Check in full normalized repo-root status fixture.
5. Check in full normalized benchmark-root export fixture.
6. Keep or rename the repo-root unsupported export fixture so its purpose is
   unmistakable.

### Phase 4: Tighten the integration tests

1. Add one dedicated test per frozen command-wall surface.
2. Compare normalized full JSON output to the exact fixture.
3. Preserve a few explicit invariant assertions where a full-file diff would be
   opaque:
   - repo-root `scope_authority == "inventory_only"`
   - repo-root export `errors[0].code == "SPEC_UNSUPPORTED_SCOPE"`
   - namespace and single-file partial cases never count as supported positive
   - partial status omits full-scope-only projection surfaces
4. Keep snapshot tests and invalid-registry tests intact as adjacent coverage,
   but do not let them substitute for the I4 wall.

### Phase 5: Prove the regression wall

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
  - or a narrowly extracted adjacent contract test module if that is cleaner
- `spec-cli/tests/fixtures/benchmarks/*.json`

Conditional write set, only if required:

- `spec-cli/src/commands.rs`
  - only for stability fixes that preserve the frozen contract

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

This milestone has two real workstreams with one merge point.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. fixture promotion | `spec-cli/tests/fixtures/benchmarks/` | — |
| B. test helper + assertions | `spec-cli/tests/` | A for final fixture names and normalization policy |
| C. optional stability fix | `spec-cli/src/`, `spec-core/src/` | B only if tests expose real nondeterminism |

### Parallel lanes

Lane A: fixture promotion
- seed normalized full-command fixtures from `.runs/i3_5_authority_alignment/validation/final-main/`
- finalize file names and placeholder policy

Lane B: test harness tightening
- build normalization helpers
- add one test per command-wall surface
- wire tests to the finalized fixtures

Lane C: optional runtime stabilization
- only launch if Lane B proves a truthful contract cannot be asserted without a
  tiny product-code stabilization change

### Execution order

1. Launch Lane A and the first half of Lane B in parallel.
2. Merge on fixture names and normalization policy.
3. Finish Lane B against the finalized fixture corpus.
4. Launch Lane C only if B finds a real nondeterministic output blocker.
5. Run the full proof wall after A+B, and after C if C exists.

### Conflict flags

- Lanes A and B both touch `spec-cli/tests/`, so they must coordinate on helper
  names and final fixture paths.
- Lane C touches product code and should stay isolated unless tests force it.
- If Lane B stays inside `cli.rs`, keep Lane A focused on fixture files to avoid
  edit collisions.

## Acceptance Checklist

- [ ] full normalized fixture exists for benchmark-root `status`
- [ ] full normalized fixture exists for namespace `status`
- [ ] full normalized fixture exists for single-file `status`
- [ ] full normalized fixture exists for repo-root `status`
- [ ] full normalized fixture exists for benchmark-root `export`
- [ ] repo-root unsupported export fixture remains deliberate and explicit
- [ ] one dedicated regression test exists per frozen command-wall surface
- [ ] normalization helpers are documented and narrow
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
