<!-- /autoplan refresh: unified from completion + test plan sources on 2026-05-10 -->
# M46 Completion And Landing Plan

Status: **authority plan candidate**  
Milestone family: **second-language-backend**  
Implementation readiness: **ready-now**  
Next artifact kind: **authority_plan**  
Autoplan ready: **yes**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Current primary head: **`a976a1f`**  
Authoritative integration head: **`ccefca8`**  
Authoritative execution root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration`**  
Last rewritten: **2026-05-10**

Primary sources:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-m46-completion-plan-20260510-201930.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-202402.md`

Supersedes:
- prior repo-root `PLAN.md` for M46 feature implementation
- the standalone M46 completion-plan draft above as the single-source authority document

Related artifact:
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`

## Executive Verdict

M46 feature work is already integrated at `ccefca8`. The remaining job is not more feature design. It is proof, closeout, and clean landing onto `feat/m40-plus`.

The preferred path is:

1. run the frozen M46 proof wall from the integration worktree
2. write closeout from observed results
3. fast-forward `feat/m40-plus` to the integrated head
4. rerun narrow validation on the landed branch

Use a bounded follow-up commit on top of `ccefca8` only if proof or landing exposes one last real M46 defect. Manual cherry-pick reconstruction onto `feat/m40-plus` is not allowed.

## Goal

Finish M46 honestly.

That means:

- prove the integrated helper-aware monotone-up TypeScript lane against the real code
- keep the expected mixed-root TypeScript status contract explicit and unchanged
- land the integrated M46 truth onto `feat/m40-plus`
- write closeout that records what actually happened, not what was expected to happen

## Problem

The repo currently has split truth:

- the authoritative integrated M46 code exists at `ccefca8` in the integration worktree
- the primary working branch `feat/m40-plus` still points at `a976a1f`
- acceptance and closeout are not yet finished at the repo-root authority level

If the repo starts the next wedge from `feat/m40-plus` before this is resolved, future work will inherit stale branch truth and fake confidence about what M46 actually shipped.

## Done Means

M46 is closed only when all of the following are true:

1. the frozen proof wall passes on `ccefca8` with the expected positive and negative outcomes
2. `closeout.md` records the exact observed command results, exact integrated SHA, and exact landed SHA
3. `feat/m40-plus` points at the landed M46 head
4. post-landing rerun on `feat/m40-plus` matches the integration truth for the narrow M46 validation surface
5. repo-root authority docs describe one coherent landing story with no branch-state ambiguity

## Scope

### In scope

- proof-wall execution on the integration head
- acceptance and closeout completion from observed truth
- deciding between fast-forward landing and one bounded continuation commit
- landing the integrated truth onto `feat/m40-plus`
- post-landing branch validation

### Out of scope

- any new post-M46 milestone selection
- wrapper TypeScript execution
- additional TypeScript family support
- broader docs rewriting outside M46 closeout truth
- reopening the M46 helper-aware implementation scope itself

## What Already Exists

| Sub-problem | Existing source of truth | Reuse verdict |
| --- | --- | --- |
| M46 scope authority | repo-root `PLAN.md` and `ORCH_PLAN.md` | reuse, now unified here |
| Integrated M46 code | integration worktree at `ccefca8` | authoritative code source |
| Merge and run history | `.runs/m46_helper_aware_monotone_up_typescript/merge-log.md` | reuse |
| Acceptance scaffold | `.runs/m46_helper_aware_monotone_up_typescript/acceptance.md` | reuse and finalize |
| Closeout scaffold | `.runs/m46_helper_aware_monotone_up_typescript/closeout.md` | reuse and finalize |
| Primary landing target | `feat/m40-plus` at `a976a1f` | still stale, must advance |

## Delta Audit

Observed M46 delta from `a976a1f` to `ccefca8`:

- core code: `spec-core/src/typescript_backend.rs`, `spec-core/src/validator.rs`, `spec-cli/src/commands.rs`
- CLI coverage: `spec-cli/tests/cli.rs`
- canonical example truth: `examples/ecommerce/units/money/round.unit.spec`, `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- packet proof truth: aligned monotone-up fixture helper and apply-tax unit specs plus refreshed passports
- public contract: `README.md`, `CHANGELOG.md`

This is not a trivial branch-pointer move, but it is still one coherent M46 delta. The plan must treat `ccefca8` as the starting authority and avoid reconstructing that delta by hand on the primary branch.

## Locked Decisions

### 1. Hold scope

This is a completion and landing plan, not a new feature plan. M46 scope is already chosen.

### 2. `ccefca8` is the authoritative starting point

Proof and landing work begins from the integration worktree head, not from `feat/m40-plus`.

### 3. Option A is preferred

If the proof wall passes unchanged, fast-forward `feat/m40-plus` directly to `ccefca8`.

### 4. Option B is bounded contingency only

If proof or landing exposes one real unfinished M46 defect, fix it on top of `ccefca8`, rerun the proof wall, then land that new head.

### 5. Cherry-pick reconstruction is forbidden

Do not manually replay selected M46 commits onto `feat/m40-plus`. That creates two competing truth surfaces.

### 6. One canonical green trust surface must be recorded

The closeout must highlight one clean product-facing success case:

`cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript`

### 7. The mixed-root TypeScript status remains intentionally non-green

The closeout must preserve the expected root TypeScript status contract instead of pretending M46 created full green parity.

## Step 0 - Scope Challenge

### Minimum complete change

The minimum honest change set is operational, not architectural:

1. verify the integrated head
2. decide Option A or Option B from observed proof
3. finalize acceptance and closeout
4. land onto `feat/m40-plus`
5. rerun narrow landed-branch validation

Anything beyond that is scope creep.

### Complexity check

This plan touches multiple truth surfaces, but only one code surface:

- repo-root authority files
- `.runs/m46_helper_aware_monotone_up_typescript/` artifacts
- git branch pointers
- the already-integrated code at `ccefca8`

That is the smallest complete plan. The overbuilt version would be reopening implementation or inventing a new milestone before landing the one that already exists.

### Completeness check

The bad shortcut is "integration proved it once, close enough." That saves almost nothing and leaves the repo in a split-truth state. The complete version is still cheap:

- prove the integrated head
- land it
- prove the landed head
- record both SHAs

Do the complete version.

## Architecture Contract

### Truth-surface diagram

```text
repo-root authority
  PLAN.md + ORCH_PLAN.md
          |
          v
run artifacts (.runs/m46...)
  acceptance.md / merge-log.md / closeout.md
          |
          v
integration code truth
  ws/spec-m46-integration @ ccefca8
          |
          v
proof wall execution
  cargo test / spec test / spec status
          |
          v
landing decision
  Option A: fast-forward feat/m40-plus
  Option B: bounded fix on top of ccefca8, then land
          |
          v
primary branch truth
  feat/m40-plus @ landed M46 head
          |
          v
post-landing validation
```

### Landing topology

| Option | What it means | When allowed | Verdict |
| --- | --- | --- | --- |
| A. Fast-forward landing | advance `feat/m40-plus` directly to `ccefca8` | proof wall passes unchanged | preferred |
| B. Bounded continuation | add one last M46 fix on top of `ccefca8`, then land that new head | proof or landing exposes a real remaining defect | allowed, contingency only |
| Rejected. Cherry-pick reconstruction | replay selected commits onto `feat/m40-plus` | never | forbidden |

## Ordered Execution Plan

### Step 1. Freeze the execution root

- execute all proof-wall commands from `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration`
- record `ccefca8` as the authoritative starting head in acceptance and closeout artifacts
- refuse to run the proof wall from `feat/m40-plus`

Exit gate:
- execution root is explicit in the run record
- integrated SHA is recorded before proof begins

### Step 2. Run the frozen M46 proof wall

Run the exact commands listed in the proof-wall section below. Record observed outcomes, not inferred outcomes.

Exit gate:
- every command has pass/fail output captured
- the expected non-green TypeScript status contract is either confirmed or falsified

### Step 3. Decide landing path from evidence

- choose Option A if Step 2 matches expected truth exactly
- choose Option B only if Step 2 or the branch move exposes one remaining bounded M46 defect
- if Option B is required, keep the fix on top of `ccefca8`, rerun Step 2, then continue

Exit gate:
- chosen landing path is written down with a one-line reason

### Step 4. Write acceptance and closeout from observed truth

- acceptance records exact command outcomes
- closeout records exact integrated SHA, exact landed SHA once known, the canonical green trust surface, and the expected non-green root status
- do not write final success language before branch landing and post-landing validation

Exit gate:
- acceptance is no longer "not started"
- closeout reflects observed truth only

### Step 5. Land onto `feat/m40-plus`

- if Option A, fast-forward `feat/m40-plus` to `ccefca8`
- if Option B, fast-forward `feat/m40-plus` to the new bounded continuation head
- do not cherry-pick

Exit gate:
- `feat/m40-plus` points at the landed M46 head

### Step 6. Rerun narrow validation on the landed branch

Rerun the narrow post-landing validation commands from `feat/m40-plus`:

- `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript`
- `cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json`

Exit gate:
- the landed branch matches the integrated truth for the canonical green surface and the mixed-root non-green surface

### Step 7. Finalize authority artifacts

- update closeout with the exact landed head
- mark M46 complete only after Step 6 passes
- keep future milestone selection explicitly out of this closeout

Exit gate:
- one coherent repo-root landing story
- no ambiguity about what code is landed and what proof was observed

## Required Proof Wall

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_plus_tax.test.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
```

## Expected Truth

Expected outcomes on the integration head:

- `cargo test -p spec-core -- --color never` passes
- `cargo test -p spec-cli --test cli -- --color never` passes
- aligned monotone-up packet proof passes in TypeScript
- unsupported near miss fails before Bun
- helper-aware `pricing/apply_tax` passes in TypeScript
- `discount_plus_tax.test.spec` fails before Bun because molecule tests remain Rust-only
- root TypeScript status remains expected non-green:
  - exit code `1`
  - `pricing/apply_tax` is `valid`
  - `money/round`, `pricing/apply_discount`, `pricing/calculate_total`, `pricing/checkout_quote`, and `pricing/discount_policy` are `untested`
  - no Rust proof inheritance appears in the TypeScript status view

## Test Review

### Test artifact

Primary QA-facing artifact:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-202402.md`

This plan does not replace that artifact. It depends on it and adds the missing branch-landing validation contract.

### Coverage diagram

```text
INTEGRATION HEAD COVERAGE
=========================
[+] Core regression
    └── [COVERED] cargo test -p spec-core

[+] CLI regression
    └── [COVERED] cargo test -p spec-cli --test cli

[+] Positive helper-aware packet proof
    └── [COVERED] aligned packet in TypeScript

[+] Negative bounded-lane packet proof
    └── [COVERED] unsupported near miss fails before Bun

[+] Canonical green trust surface
    └── [COVERED] pricing/apply_tax.unit.spec passes in TypeScript

[+] Molecule rejection surface
    └── [COVERED] discount_plus_tax.test.spec fails before Bun

[+] Mixed-root TypeScript status surface
    └── [COVERED] status exits 1 with expected non-green contract

LANDED BRANCH COVERAGE
======================
[+] Canonical green trust surface parity
    └── [GAP TO EXECUTE] rerun apply_tax TypeScript test from feat/m40-plus

[+] Mixed-root non-green parity
    └── [GAP TO EXECUTE] rerun TypeScript status JSON from feat/m40-plus
```

### Required post-landing validation

The integration proof wall is necessary but not sufficient. Post-landing rerun is mandatory because the primary branch is the surface future work will build from.

### Observed integration proof notes

Observed proof-wall truth on `ccefca8` already indicates:

- core tests passed
- CLI tests passed
- aligned TypeScript packet passed
- unsupported near-miss packet failed before Bun as intended
- `pricing/apply_tax.unit.spec` passed in TypeScript
- `discount_plus_tax.test.spec` failed before Bun as intended
- `spec status ... --target-language typescript --format json` exited `1` with the expected mixed-root result

One non-blocking engineering concern remains visible during those flows:

- `spec-cli/src/commands.rs` still emits dead-code warnings for `status_command`, `generate_command`, `build_command`, and `test_command`

That warning cleanup is not part of M46 completion unless it blocks landing.

## Error And Rescue Registry

| Failure | Why it happens | Rescue |
| --- | --- | --- |
| Proof wall fails on `ccefca8` | M46 is not actually done | stay on Option B, fix only the bounded M46 defect |
| Closeout drifts from actual results | closeout written from expectation | write closeout only after proof records exist |
| `feat/m40-plus` receives partial M46 truth | wrong base or manual cherry-picking | land only from integration head |
| Root TS status gets misread as a green gate | old misunderstanding returns | preserve explicit non-green contract in closeout |
| Landed branch diverges from integration truth | branch advanced without rerun | run mandatory post-landing validation |

## Failure Modes Registry

| Risk | Severity | Test coverage | Error handling | User-visible outcome |
| --- | --- | --- | --- | --- |
| M46 closes without landing on `feat/m40-plus` | Critical | no, unless branch rerun happens | process gate only | stale branch truth |
| Manual reconstruction onto primary branch | Critical | no | policy gate only | competing truth surfaces |
| Proof wall only run on integration head | High | partial | none if skipped | false confidence on primary branch |
| Closeout omits exact SHAs | High | no | documentation gate only | readers cannot verify what actually landed |
| Mixed-root red surface is reported without a canonical green surface | High | yes, if closeout records both | documentation gate only | milestone sounds more broken than it is |

Critical gap rule for this plan:

- if post-landing rerun is missing, M46 is not done
- if closeout does not record both SHAs, M46 is not done
- if the plan lands by cherry-pick reconstruction, M46 is not done

## Observability And Closeout Requirements

Closeout must include all of the following:

- authoritative integration head SHA
- landed primary-branch SHA
- exact proof-wall command list
- exact observed outcome for each command
- one canonical green trust surface:
  - `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript`
- one explicit expected non-green trust surface:
  - `cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json`
- statement that `.test.spec --target-language typescript` remains intentionally unsupported

## Worktree Parallelization Strategy

This plan has limited parallelism. Most critical steps share the same truth surfaces, `.runs/m46...` artifacts and the integration worktree, so the core landing path is mostly sequential.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| freeze execution root and run proof wall | `.worktrees/spec-m46/integration`, `.runs/m46_helper_aware_monotone_up_typescript/` | — |
| acceptance capture | `.runs/m46_helper_aware_monotone_up_typescript/` | freeze execution root and run proof wall |
| closeout drafting | `.runs/m46_helper_aware_monotone_up_typescript/`, repo-root authority docs | freeze execution root and run proof wall |
| branch landing | git refs for `feat/m40-plus`, integration worktree | acceptance capture |
| post-landing validation | primary branch checkout, `.runs/m46_helper_aware_monotone_up_typescript/` | branch landing |
| final closeout finalize | `.runs/m46_helper_aware_monotone_up_typescript/`, repo-root authority docs | post-landing validation |

### Parallel lanes

Lane A: freeze execution root -> run proof wall -> acceptance capture  
Lane B: closeout draft shell after proof starts, then fill only from observed outputs  
Lane C: branch landing -> post-landing validation -> final closeout finalize

### Execution order

1. Launch Lane A first. It owns the evidence that every other step depends on.
2. Lane B may prepare closeout structure in parallel after Lane A begins, but it cannot finalize any result text until Lane A completes.
3. Launch Lane C only after Lane A confirms the landing path and acceptance is written.

### Conflict flags

- Lane A and Lane B both touch `.runs/m46_helper_aware_monotone_up_typescript/`. Keep one parent owner for writes to avoid artifact drift.
- Lane A and Lane C both depend on the authoritative integrated head. Do not move `feat/m40-plus` before Lane A is complete.
- Lane B and Lane C both affect final closeout wording. Final closeout ownership stays with the parent after post-landing validation.

### Parallelization verdict

Safe concurrency is limited. Treat this as one primary sequential lane with one light parallel drafting lane, not as a multi-worker code implementation effort.

## NOT in scope

- choosing M47
- reopening M46 helper-aware implementation scope
- redefining the TypeScript status contract
- dead-code warning cleanup unless it blocks landing
- any new product work unrelated to M46 completion and landing

## Completion Summary

| Area | Verdict | Notes |
| --- | --- | --- |
| Problem choice | Strong | this is the real unfinished work |
| Scope | Strong | bounded to proof, closeout, landing, rerun |
| Reuse | Strong | existing integration head and artifacts already exist |
| Architecture | Strong | one authoritative code truth surface |
| Test plan | Strong with one mandatory gap | landed-branch parity rerun is still required |
| Performance | Strong | human-process integrity is the main risk, not runtime cost |
| Recommendation | Approve and execute | prefer Option A, keep Option B as bounded contingency |

## One-Line Summary

M46 is already built at `ccefca8`; the remaining honest work is to prove that head, land it onto `feat/m40-plus`, rerun the narrow branch validation, and write closeout that records one canonical green TypeScript trust surface plus the still-intentional mixed-root non-green status.
