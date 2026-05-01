<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260501-192409.md -->
# M27.8R - Fixture-Owned Harness Truth Repair

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Nearest source-truth branches: **`ws/m27_8-int`, `ws/m27_8-lane-a`, `ws/m27_8-lane-b`**  
Last rewritten: **2026-05-01**

## Summary

The old M27.8 corpus-expansion framing is done.

The integrated run already proved the product truth that matters:

- coverage totals: `28 / 15 / 0 / 13`
- recommendation status: `ranked`
- first ranked candidate: arithmetic cluster, `ready`
- second ranked candidate: `money/round` cluster, `hold` for `unknown_overlap_family`

The failure is narrower. The final copied-workspace lock in `cargo test -p xtask -- --color never`
replayed a different seeded world and observed
`coverage.function_coverage.promoted_family_units == 10`.

M27.8R is therefore a harness-truth repair milestone.

The job is:

1. recover the already-proven lane-A authored source truth onto `feat/corpus-expansion`
2. preserve the ranked command-path assertions from lane-B
3. repair the seeded workspace so it copies the promoted packet root the failing command path actually consumes
4. rerun the exact proof loop until the final `xtask` lock reproduces the same truth the integrated run already proved

If the repaired harness still diverges, stop and re-plan from captured seeded-workspace evidence. No second guesswork pass.

## Plan Authority

This file supersedes the earlier M27.8 corpus-expansion plan.

Primary sources:

- [spensermcconnell-feat-corpus-expansion-design-20260501-191122.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260501-191122.md)
- `.runs/m27_8/acceptance.md`
- `.runs/m27_8/merge-log.md`
- `.runs/m27_8/contract-freeze.json`
- `xtask/src/lib.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/inventory.rs`
- `xtask/src/family/recommend.rs`
- `semantic-families/corpus/rust-function.toml`

Durable truth lives in the blocked run artifacts:

- `.runs/m27_8/acceptance.md`
- `.runs/m27_8/merge-log.md`
- `.runs/m27_8/contract-freeze.json`

The `ws/m27_8-*` branches are recovery sources for authored files. They are not the only authority.

## Resolved Causality Model

The plan now chooses one causality model and retires the stale one.

Accepted model:

1. the failing lock is `xtask/src/lib.rs::recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()`
2. that test seeds a temp workspace with `seed_locked_recommendation_workspace()`
3. the test calls `recommend::run_with_writer(temp_dir.path(), "json", ...)`
4. `recommend::run_with_writer()` calls `coverage::collect_latest(temp_dir.path())`
5. `coverage::collect_latest()` calls `inventory::collect_inventory(temp_dir.path())`
6. `inventory::collect_inventory()` marks promoted families from promoted packet roots that physically exist in the seeded workspace
7. the current seed list copies:
   - `semantic-families/function.wrapper.pipeline.chain3.v1`
   - `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1`
   - `semantic-families/function.arithmetic_leaf.monotone_up.v1`
8. the current seed list does **not** copy `semantic-families/function.wrapper.pipeline.v1`
9. that omission is the strongest concrete explanation for the exact promoted-family drop from `15` to `10`

Retired model:

- the earlier design doc hypothesis that `examples/crosslib-app/spec.toml` is the primary missing seeded input is no longer the working theory for M27.8R
- this plan does **not** widen into speculative config-copy expansion unless the packet-root repair fails and seeded evidence proves a second missing dependency

That matters. The plan is no longer "patch whatever seems plausibly missing." The plan is "repair the precise seeded input path the failing test actually reads."

## Milestone Outcome

When M27.8R lands, the repo can truthfully claim:

- the already-proven M27.8 lane-A source truth has been recovered onto `feat/corpus-expansion`
- the seeded command-path harness now copies the promoted packet root required to model the same promoted-family inventory the integrated run used
- the ranked command-path lock reproduces the same coverage and recommendation truth already observed in the blocked integration run
- the repo did not widen scope into recommendation policy, coverage policy, schema changes, or broader harness redesign

M27.8R does **not** claim:

- a new corpus-expansion milestone
- new recommendation logic
- new coverage logic
- new artifact schemas
- a repo-wide replacement of copied-workspace harnesses
- M28 shared-core extraction

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Exact authored `apply_tax` source truth | `.runs/m27_8/contract-freeze.json`, merge commit `ab11249`, branch `ws/m27_8-lane-a` | Reuse literally. Do not re-author from memory. |
| Ranked command-path assertion shape | merge commit `7ae58ae`, branch `ws/m27_8-lane-b` | Reuse as the starting point. Repair the harness around it. |
| Integrated acceptance truth | `.runs/m27_8/acceptance.md` | Reuse as the contract oracle. |
| Seeded workspace helper seam | `xtask/src/lib.rs::seed_locked_recommendation_workspace()` | Reuse and repair. Do not introduce a new harness framework. |
| Promoted-family inventory model | `xtask/src/family/inventory.rs` | Reuse as-is. Feed it the missing promoted packet root. |
| Existing proof-loop order | `.runs/m27_8/contract-freeze.json.required_build_order` | Reuse exactly. |

### Minimum honest change

The smallest complete tracked diff is still three source files:

1. `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
2. `examples/crosslib-app/units/.gitignore`
3. `xtask/src/lib.rs`

Their roles are now explicit:

- files 1 and 2 are recovery of already-proven lane-A source truth
- file 3 preserves the ranked lane-B lock shape and repairs the seeded workspace inputs it depends on

Anything larger is scope creep unless the diagnostic stop gate proves a second missing seeded input.

Anything smaller leaves the repo in the same bad state where integrated proof and final lock disagree about reality.

### Alternatives deferred

| Alternative | Why deferred |
|---|---|
| Real checkout or worktree instead of copied-workspace harness | Bigger test-model change than this follow-up needs. Only justify it if the packet-root repair fails and seeded evidence shows the copied-world approach is still incomplete. |
| Checked-in manifest of every copied seeded path | Reasonable future hardening, but too much ceremony for this repair. |
| Artifact-hash-only lock without reconstructing a seeded world | Too weak for this milestone because the failure is specifically about the reconstructed world disagreeing with integrated truth. |
| Expanding the seed list from suspicion, starting with `spec.toml` | Rejected as first move. Evidence now points first at promoted packet presence, not general config drift. |

### Complexity, completeness, and distribution

- Complexity check: exactly three tracked source files, zero new Rust modules, zero new services, zero new harness frameworks
- Completeness check: do the full repair now, meaning recovery + packet-root fix + exact proof loop + explicit diagnostic stop gate
- Distribution check: no new artifact type, CI surface, or install surface is introduced
- `TODOS.md` cross-reference: no existing TODO blocks this plan directly, and this follow-up should not create a vague TODO in place of captured evidence

## Scope Contract

### In Scope

- recover the exact lane-A authored files from `ws/m27_8-lane-a` or merge commit `ab11249`
- preserve the ranked lock shape from `ws/m27_8-lane-b` or merge commit `7ae58ae`
- repair `seed_locked_recommendation_workspace()` so it copies `semantic-families/function.wrapper.pipeline.v1`
- add one short code comment above the seeded copy list noting that promoted packet roots are part of command-path inventory truth
- rerun the exact proof loop from `.runs/m27_8/contract-freeze.json`
- if the final lock still diverges, capture seeded temp-workspace evidence from inside the test and stop on the first unexplained mismatch

### NOT In Scope

- editing `semantic-families/corpus/rust-function.toml`
- changing `xtask/src/family/coverage.rs`
- changing `xtask/src/family/recommend.rs`
- changing `xtask/src/family/promotion_artifacts.rs`
- changing `xtask/src/family/inventory.rs`
- changing recommendation or coverage schemas
- rewriting `.runs/m27_8/*` historical artifacts
- replacing copied-workspace harnesses repo-wide
- M28 work

## Exact File Contract

### Tracked source files

These are the only tracked source files this follow-up should change:

1. `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
2. `examples/crosslib-app/units/.gitignore`
3. `xtask/src/lib.rs`

### File-by-file contract

| File | Required state | Proof of correctness | Forbidden drift |
|---|---|---|---|
| `examples/crosslib-app/units/pricing/apply_tax.unit.spec` | Exact recovery of the frozen lane-A authored truth | matches `ab11249` and `.runs/m27_8/contract-freeze.json.locked_apply_tax_shape`; `spec test` passes | re-authoring a logically similar but textually different unit |
| `examples/crosslib-app/units/.gitignore` | exact whitelist line `!pricing/apply_tax.spec.passport.json` restored | matches `ab11249` and contract freeze | broadening or reformatting unrelated ignore policy |
| `xtask/src/lib.rs` | ranked command-path lock preserved, seeded copy list repaired with `semantic-families/function.wrapper.pipeline.v1`, short inventory comment added | final `cargo test -p xtask -- --color never` reproduces frozen truth | new fixture framework, new abstraction layer, unrelated test churn, policy changes outside the failing lock path |

### Non-touch source surfaces

- `semantic-families/corpus/rust-function.toml`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/inventory.rs`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `.runs/m27_8/*`

### Expected derived artifact churn

- `examples/crosslib-app/units/pricing/apply_tax.spec.passport.json` (new or refreshed)
- `examples/crosslib-app/units/pricing/apply_discount.spec.passport.json`
- `examples/shared-spec/units/money/round.spec.passport.json`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

## Architecture Contract

Use the blocked run as the oracle and repair the existing harness seam in place.

Do not add a second fixture layer. Do not generalize this into a new test framework. This repo already has the seam it needs.

### Architecture ASCII diagram

```text
CURRENT BRANCH
==============
feat/corpus-expansion
    │
    ├── missing lane-A source truth
    │   ├── apply_tax.unit.spec absent
    │   └── apply_tax passport whitelist absent
    │
    └── stale command-path harness
        ├── stale no_strong_candidate test name + baseline
        └── seeded workspace omits wrapper.pipeline.v1 packet root

RECOVERY SOURCES
================
ws/m27_8-lane-a / ab11249
    └── exact authored apply_tax truth + whitelist line

ws/m27_8-lane-b / 7ae58ae
    └── ranked command-path lock shape

FAILING RUNTIME PATH
====================
xtask test
    -> recommendation_command_path_...()
    -> seed_locked_recommendation_workspace(temp_dir)
    -> recommend::run_with_writer(temp_dir)
    -> coverage::collect_latest(temp_dir)
    -> inventory::collect_inventory(temp_dir)
    -> promoted packet roots present in temp_dir decide promoted-family count

REPAIR
======
xtask/src/lib.rs
    └── seed_locked_recommendation_workspace()
        ├── copy corpus unit trees
        ├── copy wrapper.pipeline.chain3.v1
        ├── copy arithmetic promoted packets
        ├── copy wrapper.pipeline.v1                  <-- add
        └── keep ranked lock assertions unchanged

PROOF LOOP
==========
shared-spec build
    -> exact apply_tax proof
    -> crosslib build
    -> crosslib crate tests
    -> coverage artifact equality
    -> recommendation artifact equality
    -> artifact validation
    -> xtask command-path lock
    -> if still red: capture seeded temp-workspace inventory/coverage/recommendation, then stop
```

## Implementation Steps

1. Recover `examples/crosslib-app/units/pricing/apply_tax.unit.spec` exactly from `ab11249` or `ws/m27_8-lane-a`.
2. Recover the `!pricing/apply_tax.spec.passport.json` whitelist line exactly from `ab11249` or `ws/m27_8-lane-a`.
3. Start from the ranked command-path test shape from `7ae58ae` or `ws/m27_8-lane-b`.
4. In `seed_locked_recommendation_workspace()`, add this copied input:
   - `semantic-families/function.wrapper.pipeline.v1`
5. Add one short comment above the seed list noting that promoted packet roots are part of command-path inventory truth for the copied workspace.
6. Keep the ranked assertions bound to this exact frozen truth:
   - source ids: `examples_ecommerce`, `m19_semantic_falsification_pack`, `m20_unsupported_truth_pack`, `examples_shared_spec`, `examples_crosslib_app`
   - source counts: `6 / 12 / 9 / 1 / 2`
   - function coverage: `28 / 15 / 0 / 13`
   - recommendation status: `ranked`
   - arithmetic cluster first and `ready`
   - `money/round` cluster second and `hold`
7. Run the locked proof loop below in order.
8. If the final lock still reports promoted-family count other than `15`, capture seeded temp-workspace evidence from inside the test before any second edit.

## Test And Proof Contract

### Code-path coverage

```text
CODE PATH COVERAGE
==================
[+] examples/crosslib-app/units/pricing/apply_tax.unit.spec
    │
    ├── [PLAN] recover exact frozen authored shape from ab11249 / contract-freeze.json
    └── [PLAN] prove via `cargo run -p spec-cli -- test .../apply_tax.unit.spec`

[+] xtask/src/lib.rs :: seed_locked_recommendation_workspace()
    │
    ├── [PLAN] copy semantic-families/function.wrapper.pipeline.v1
    ├── [PLAN] document that promoted packet roots are inventory inputs
    └── [PLAN] reuse existing copy helper, no new harness abstraction

[+] xtask command-path recommendation lock
    │
    ├── [PLAN] stdout bytes == written artifact bytes on run 1
    ├── [PLAN] stdout bytes == written artifact bytes on run 2
    ├── [PLAN] run 1 bytes == run 2 bytes
    ├── [PLAN] coverage source ids == locked five-source order
    ├── [PLAN] coverage source counts == 6 / 12 / 9 / 1 / 2
    ├── [PLAN] function coverage == 28 / 15 / 0 / 13
    ├── [PLAN] recommendation status == ranked
    ├── [PLAN] first candidate == arithmetic ready
    └── [PLAN] second candidate == money/round hold

[+] Diagnostic stop gate
    │
    ├── [PLAN] if promoted-family count still != 15, capture inventory from temp_dir.path()
    ├── [PLAN] capture coverage.latest.json from temp_dir.path()
    ├── [PLAN] capture recommendation.latest.json from temp_dir.path()
    └── [PLAN] stop before any second speculative fix

INTEGRATED PROOF COVERAGE
=========================
[+] shared-spec build
    └── [PLAN] must pass before crosslib proof

[+] crosslib exact-unit proof
    └── [PLAN] apply_tax exact-unit test must pass

[+] crosslib build + crate tests
    └── [PLAN] shared dep wiring must compile and test clean

[+] artifact validation
    ├── [PLAN] coverage.latest.json validates
    └── [PLAN] recommendation.latest.json validates

─────────────────────────────────
COVERAGE: 11/11 planned paths covered
  Code paths: 8/8
  Integrated proof paths: 3/3
QUALITY: ★★★: 11  ★★: 0  ★: 0
GAPS: 0 remaining if the plan is executed literally
─────────────────────────────────
```

### Locked proof loop

Run these commands in order. The order must match `.runs/m27_8/contract-freeze.json.required_build_order`.

```bash
git status --short

cargo run -p spec-cli -- build examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- build examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo test --manifest-path examples/crosslib-app/Cargo.toml

cargo xtask family coverage --format json > /tmp/m27_8r-coverage.stdout.json
cmp -s /tmp/m27_8r-coverage.stdout.json .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json || { diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m27_8r-coverage.stdout.json || true; exit 1; }

cargo xtask family recommend --format json > /tmp/m27_8r-recommend.stdout.json
cmp -s /tmp/m27_8r-recommend.stdout.json .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json || { diff -u .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json /tmp/m27_8r-recommend.stdout.json || true; exit 1; }

cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json

cargo test -p xtask -- --color never
```

### Diagnostic stop gate

If the final `cargo test -p xtask -- --color never` still fails on promoted-family count, do **not** start hunting for a second missing path from the repo root.

That would be misleading because:

- the failing test seeds a temp workspace with `temp_dir.path()`
- `xtask` CLI commands always read the current working directory
- `cargo xtask family inventory --format json` from repo root is repo truth, not seeded temp-workspace truth

If the lock still fails, capture evidence from inside the test against `temp_dir.path()` before any second fix:

1. write `inventory::render_snapshot_bytes(temp_dir.path())` to `/tmp/m27_8r-seeded-inventory.json`
2. write `fs::read(temp_dir.path().join(FAMILY_COVERAGE_LATEST_PATH))` to `/tmp/m27_8r-seeded-coverage.json`
3. write `fs::read(temp_dir.path().join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH))` to `/tmp/m27_8r-seeded-recommendation.json`
4. if more spelunking is needed, also print or persist the temp workspace path itself before the assertion fires
5. stop and re-plan from those seeded outputs

Those diagnostic edits are investigative only. Revert them before final landing unless the captured evidence proves a new permanent assertion or helper is required.

## Failure Modes

| Codepath | Realistic failure | Test covers it? | Error handling exists? | User-visible or silent? | Critical gap? |
|---|---:|---:|---:|---|---:|
| `apply_tax.unit.spec` recovery | wrong body copied back onto branch | Yes | N/A | visible in exact-unit proof | No |
| `.gitignore` recovery | passport whitelist not restored exactly | Yes | N/A | visible in diff and proof rerun | No |
| seed helper copy list | promoted packet root still omitted | Yes | No | visible in final xtask failure, not silent | No |
| ranked lock assertions | counts updated without reproducing integrated proof | Yes | No | visible in xtask failure | No |
| diagnostic stop gate | maintainer captures repo inventory instead of seeded inventory | Yes, if gate is followed literally | N/A | would be misleading, not silent | No |
| proof-loop rerun | earlier steps pass but final lock still seeds a divergent world | Yes | No | visible in final xtask failure | No |

No failure mode in this plan is both silent and untested. Good.

## Worktree Parallelization Strategy

Parallelization exists in theory, but it is not the recommended execution mode for a single maintainer on this repair.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Recover lane-A source truth | `examples/crosslib-app/units/` | — |
| Recover lane-B lock shape and repair seeded copy list | `xtask/src/` | — |
| Proof loop and artifact refresh | `examples/shared-spec/`, `examples/crosslib-app/`, `.semantic-family-artifacts/`, `xtask/` | lane-A recovery + lane-B harness repair |

### Parallel lanes

- Lane A: recover `apply_tax.unit.spec` and `.gitignore` from `ab11249`
- Lane B: restore ranked lock semantics from `7ae58ae` and add `semantic-families/function.wrapper.pipeline.v1` to the seed list
- Lane C: run the proof loop and refresh derived artifacts after A and B are merged

### Execution order

If two people are assigned, launch Lane A and Lane B in parallel worktrees, merge both, then run Lane C.

### Conflict flags

- Merge-conflict risk is low because Lane A touches `examples/crosslib-app/units/` and Lane B touches `xtask/src/`
- Conceptual drift risk is higher than merge risk because both lanes must stay bound to the same frozen truth in `.runs/m27_8/*`

### Recommended mode

Recommended for this milestone: one branch, sequential execution.

Reason:

- the total tracked scope is three files
- lane-A recovery is essentially exact source restoration
- lane-B repair is one-file harness work
- the only expensive step is the proof loop, which depends on both anyway

## Test Plan Artifact

Primary QA surface for this milestone is command and artifact truth, not pages.

- `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  - prove the recovered cross-library unit is valid and executable
- `cargo xtask family coverage --format json`
  - verify the coverage artifact reproduces the frozen integrated truth `28 / 15 / 0 / 13`
- `cargo xtask family recommend --format json`
  - verify the recommendation artifact stays `ranked` with arithmetic first and `money/round` held second
- `cargo test -p xtask -- --color never`
  - verify the copied-workspace command-path lock reproduces the same truth as the integrated proof loop

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | Scope | Replace the old corpus-expansion framing with harness-truth repair | Mechanical | Explicit over clever | The integrated run already proved product truth; the block is now harness truth | Pretending the milestone is still exploratory corpus work |
| 2 | Authority | Treat `.runs/m27_8/*` artifacts plus `ws/m27_8-*` merges as source truth | Mechanical | Pragmatic | The current branch is missing proven authored files, and the blocked artifacts freeze the accepted outputs | Re-authoring or recomputing truth from memory |
| 3 | Architecture | Reuse the existing seeded helper instead of inventing a new fixture framework | Mechanical | Boring by default | One explicit seam already exists in `xtask/src/lib.rs` | New fixture registry or generalized harness layer |
| 4 | Causality | Repair the missing promoted packet root first | Mechanical | Explicit over clever | The failing code path reads promoted packet presence through inventory; the missing wrapper packet explains the exact `15 -> 10` drop | Speculative config-copy expansion as the first move |
| 5 | Diagnostics | Capture evidence from `temp_dir.path()` if the lock still fails | Mechanical | Choose completeness | Repo-root inventory is the wrong world; seeded temp-workspace evidence is the only honest next input | Running repo-root commands and calling them seeded truth |
| 6 | Execution | Recommend one-branch sequential execution even though A and B are parallelizable on paper | Mechanical | Pragmatic | This is too small to justify worktree coordination overhead for one maintainer | Splitting a three-file repair by default |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | advisory only | The plan is now correctly framed as harness-truth repair, not a second corpus-expansion pass |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR | The document now names one causality model, aligns the proof-loop order with the frozen contract, and fixes the diagnostic gate so it captures seeded temp-workspace evidence instead of repo-root truth |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | No UI scope |

**UNRESOLVED:** 0

**VERDICT:** ENG CLEARED — `PLAN.md` is now a single implementation contract with explicit authority, exact scope, deterministic proof steps, and a non-ambiguous seeded-workspace diagnostic gate.
