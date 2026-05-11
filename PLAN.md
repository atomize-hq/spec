<!-- /autoplan refresh: unified authority plan rewritten for single-threaded execution clarity on 2026-05-10 -->
# M46 Completion And Landing Plan

Status: **authority plan**  
Milestone family: **second-language-backend**  
Implementation readiness: **ready-now**  
Autoplan ready: **yes**  
Base branch: **main**  
Working branch: **feat/m40-plus**  
Current primary head: **`a976a1f`**  
Authoritative integration head: **`ccefca8`**  
Authoritative execution root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration`**  
Last rewritten: **2026-05-10**

Primary source artifacts:
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-m46-completion-plan-20260510-201930.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-202402.md`

Related repo artifacts:
- `ORCH_PLAN.md`
- `.runs/m46_helper_aware_monotone_up_typescript/merge-log.md`
- `.runs/m46_helper_aware_monotone_up_typescript/acceptance.md`
- `.runs/m46_helper_aware_monotone_up_typescript/closeout.md`

## Executive Verdict

M46 code is already integrated at `ccefca8`. The remaining work is operational, not architectural:

1. rerun the frozen M46 proof wall from the authoritative integration worktree
2. write acceptance and closeout from observed outputs
3. land the integrated truth onto `feat/m40-plus`
4. rerun the narrow proof on the landed branch

Default landing path is **Option A: fast-forward `feat/m40-plus` directly to `ccefca8`**.

Use **Option B: one bounded continuation commit on top of `ccefca8`** only if the fresh rerun or the branch move exposes one real unfinished M46 defect.

Cherry-pick reconstruction onto `feat/m40-plus` is forbidden.

## Goal

Finish M46 honestly.

That means:

- prove the helper-aware monotone-up TypeScript lane against the real integrated code
- preserve the intentionally non-green mixed-root TypeScript status contract
- land the integrated M46 truth onto `feat/m40-plus`
- write closeout from observed results, not expectation and not memory

## Current State

| Surface | Current truth | Why it matters |
| --- | --- | --- |
| Integrated code | `ccefca8` in `ws/spec-m46-integration` | this is the authoritative M46 implementation |
| Primary branch | `feat/m40-plus` at `a976a1f` | future work will branch from here, so this must be advanced |
| Acceptance artifact | scaffold exists | still needs observed results recorded |
| Closeout artifact | scaffold exists | still needs exact SHAs and command outcomes recorded |
| Repo-root authority plan | this file | must tell one coherent landing story |

## Done Means

M46 is done only when all five conditions are true:

1. the frozen proof wall is rerun from the authoritative execution root and matches expected truth
2. `acceptance.md` records the exact command outcomes from that rerun
3. `closeout.md` records the exact integrated SHA and the exact landed SHA
4. `feat/m40-plus` points at the landed M46 head
5. the landed branch rerun matches the integration truth for the narrow M46 validation surface

If any one of those is missing, M46 is not done.

## Step 0 - Scope Challenge

### Minimum complete change

The minimum honest change set is:

1. rerun proof from `ccefca8`
2. choose Option A or Option B from evidence
3. finalize acceptance and closeout
4. land onto `feat/m40-plus`
5. rerun narrow post-landing validation

Anything beyond that is scope creep.

### Complexity check

This plan touches multiple truth surfaces, but only one code surface:

- the integration worktree at `ccefca8`
- `.runs/m46_helper_aware_monotone_up_typescript/` artifacts
- the primary branch ref for `feat/m40-plus`
- this repo-root authority document

That is engineered enough. Reopening implementation design, adding new TypeScript families, or inventing a new milestone here would be overbuilt.

### Completeness check

The bad shortcut is "integration already looked good once, so just move the branch."

That saves almost nothing and leaves the repo exposed to the exact failure this milestone is trying to avoid: split truth between the integrated worktree and the branch everyone will actually use.

The complete version is still cheap:

- rerun the full proof wall once from the authoritative execution root
- land from the integrated head only
- rerun narrow landed-branch proof
- record both SHAs in closeout

Do the complete version.

### Distribution check

This plan does **not** introduce a new distributable artifact. Distribution is already solved at the repo level and is unaffected by M46 closeout. No new CI/CD or publish work is required for this landing.

## Scope

### In scope

- rerunning the frozen proof wall on `ccefca8`
- finalizing acceptance and closeout from observed outputs
- deciding between Option A and Option B from fresh evidence
- landing the integrated truth onto `feat/m40-plus`
- rerunning narrow post-landing validation

### Out of scope

- wrapper TypeScript execution
- cross-library TypeScript helper imports
- generic multi-dependency TypeScript execution
- any new post-M46 milestone selection
- dead-code warning cleanup unless it blocks landing
- broad documentation rewrites outside M46 acceptance and closeout truth

## What Already Exists

| Sub-problem | Existing source of truth | Reuse verdict |
| --- | --- | --- |
| M46 code | integration worktree at `ccefca8` | authoritative, reuse as-is |
| M46 delta history | `.runs/m46_helper_aware_monotone_up_typescript/merge-log.md` | authoritative context |
| Acceptance scaffold | `.runs/m46_helper_aware_monotone_up_typescript/acceptance.md` | finalize, do not recreate |
| Closeout scaffold | `.runs/m46_helper_aware_monotone_up_typescript/closeout.md` | finalize, do not recreate |
| Test surface definition | `spensermcconnell-feat-m40-plus-test-plan-20260510-202402.md` | authoritative QA input |
| Repo-root orchestration context | `ORCH_PLAN.md` | supporting context only |

## Delta Audit

Observed M46 delta from `a976a1f` to `ccefca8`:

- core code: `spec-core/src/typescript_backend.rs`, `spec-core/src/validator.rs`, `spec-cli/src/commands.rs`
- CLI coverage: `spec-cli/tests/cli.rs`
- canonical example truth: `examples/ecommerce/units/money/round.unit.spec`, `examples/ecommerce/units/pricing/apply_tax.unit.spec`
- packet proof truth: aligned monotone-up fixture helper and apply-tax unit specs plus refreshed passports
- public contract: `README.md`, `CHANGELOG.md`

Engineering implication:

- this is not a trivial branch-pointer move
- but it is still one coherent M46 delta already assembled on one head
- rebuilding that delta manually on `feat/m40-plus` would create two competing truth surfaces

## Locked Decisions

### 1. Hold scope

This is a completion-and-landing plan, not a new feature plan.

### 2. `ccefca8` is the authoritative starting point

All proof and landing work begins from the integration worktree head, not from `feat/m40-plus`.

### 3. Fresh rerun is required

Prior observed proof on `ccefca8` is useful supporting evidence, but it does **not** replace the fresh rerun required for closure.

### 4. Option A is the default landing path

If the fresh proof wall matches expected truth, fast-forward `feat/m40-plus` directly to `ccefca8`.

### 5. Option B is bounded contingency only

If the fresh rerun or the branch move exposes one real unfinished M46 defect, fix it on top of `ccefca8`, rerun the proof wall, then land that new head.

### 6. A bounded continuation fix has a hard boundary

An Option B fix is allowed only if all of the following are true:

- it stays inside the existing M46 delta surface
- it does not add new family support or widen the milestone contract
- it does not require new architecture or new milestone planning
- it can be proven with the same frozen proof wall plus the landed-branch rerun

If a needed fix breaks those boundaries, stop and re-scope instead of smuggling new work into M46.

### 7. Cherry-pick reconstruction is forbidden

Do not manually replay selected M46 commits onto `feat/m40-plus`.

### 8. Closeout must record one green trust surface and one intentional red surface

The closeout must record:

- one canonical green surface:
  - `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript`
- one intentional non-green surface:
  - `cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json`

## Architecture Contract

### Truth-surface diagram

```text
repo-root authority
  PLAN.md
     |
     v
run artifacts
  acceptance.md / closeout.md / merge-log.md
     |
     v
integration code truth
  ws/spec-m46-integration @ ccefca8
     |
     v
fresh proof wall rerun
  cargo test / spec test / spec status
     |
     v
landing decision
  Option A: fast-forward feat/m40-plus -> ccefca8
  Option B: bounded fix on ccefca8 -> rerun -> land new head
     |
     v
primary branch truth
  feat/m40-plus @ landed M46 head
     |
     v
post-landing narrow rerun
  canonical green + mixed-root non-green parity
```

### Landing topology

| Path | What it means | When allowed | Verdict |
| --- | --- | --- | --- |
| Option A | fast-forward `feat/m40-plus` directly to `ccefca8` | fresh rerun matches expected truth exactly | preferred |
| Option B | one bounded M46 continuation commit on top of `ccefca8`, then land that head | fresh rerun or branch move exposes one real bounded defect | contingency only |
| Rejected path | manual cherry-pick reconstruction onto `feat/m40-plus` | never | forbidden |

## Ordered Execution Plan

### Step 1. Freeze the execution root

Run all proof-wall commands from:

`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration`

Actions:

- verify HEAD is `ccefca8`
- record `ccefca8` as the authoritative starting SHA in `acceptance.md` and `closeout.md`
- refuse to run the proof wall from `feat/m40-plus`

Exit gate:

- execution root is explicit in the run record
- authoritative integrated SHA is recorded before proof begins

### Step 2. Rerun the frozen M46 proof wall

Run the exact command set in the next section. Record observed stdout/stderr and exit status. Do not paraphrase while the run is still in progress.

If a command fails unexpectedly:

- rerun once only if the failure is clearly environmental
- otherwise treat it as real evidence and move to the landing decision step with failure recorded

Exit gate:

- every command has one recorded observed outcome
- the rerun either confirms the expected truth or identifies the exact mismatch

### Step 3. Decide landing path from evidence

Decision rule:

- choose **Option A** if every proof-wall result matches expected truth
- choose **Option B** only if proof or landing exposes one bounded M46 defect
- stop and re-scope if the needed fix breaks the Option B boundary

The chosen path must be written down in both `acceptance.md` and `closeout.md` with a one-line reason.

Exit gate:

- landing path is explicit
- scope remains bounded

### Step 4. Finalize acceptance from observed truth

`acceptance.md` must record:

- authoritative execution root
- authoritative integrated SHA
- exact proof-wall command list
- observed outcome for each command
- explicit statement that the mixed-root TypeScript status remains intentionally non-green

Exit gate:

- acceptance is no longer a scaffold
- no command outcome is implicit or hand-waved

### Step 5. Draft closeout from observed truth

`closeout.md` must record:

- exact integrated SHA
- provisional landing path
- canonical green trust surface
- intentional non-green trust surface
- current limitation that `.test.spec --target-language typescript` remains unsupported

At this step, the closeout may still leave the landed SHA blank until the branch move completes. Everything else should be ready.

Exit gate:

- closeout is truthful and mostly complete
- only the landed SHA and post-landing confirmation remain open

### Step 6. Land onto `feat/m40-plus`

Branch move rules:

- if Option A, fast-forward `feat/m40-plus` directly to `ccefca8`
- if Option B, land the new bounded continuation head instead
- do not cherry-pick
- do not rebuild the delta on the primary branch

Exit gate:

- `feat/m40-plus` points at the landed M46 head
- landed SHA is known exactly

### Step 7. Rerun narrow validation on the landed branch

From `feat/m40-plus`, rerun only the narrow parity surface:

- `cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript`
- `cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json`

This rerun is mandatory. Integration-only proof is not enough because future work will branch from `feat/m40-plus`, not from the integration worktree.

Exit gate:

- landed branch matches integration truth for the canonical green surface
- landed branch matches integration truth for the mixed-root intentional non-green surface

### Step 8. Finalize closeout and repo-root authority

Finish `closeout.md` with:

- exact landed SHA
- post-landing parity result
- explicit statement that M46 is closed on `feat/m40-plus`

This file remains the repo-root authority plan and should not need another rewrite after closeout unless facts change.

Exit gate:

- one coherent landing story
- no ambiguity about what landed, what was proven, and what remains intentionally unsupported

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

| Surface | Expected result | Why it matters |
| --- | --- | --- |
| `cargo test -p spec-core` | pass | core regression safety |
| `cargo test -p spec-cli --test cli` | pass | CLI regression safety |
| aligned monotone-up packet | pass | positive TypeScript packet proof |
| unsupported near miss packet | fail before Bun | bounded-lane rejection still works |
| `pricing/apply_tax.unit.spec` in TypeScript | pass | canonical green helper-aware trust surface |
| `discount_plus_tax.test.spec` in TypeScript | fail before Bun | molecule TypeScript execution is still intentionally unsupported |
| `spec status examples/ecommerce --target-language typescript --format json` | exit `1` | mixed-root non-green contract remains truthful |

Additional required details for the final `spec status` result:

- `pricing/apply_tax` is `valid`
- `money/round`, `pricing/apply_discount`, `pricing/calculate_total`, `pricing/checkout_quote`, and `pricing/discount_policy` are `untested`
- no Rust proof inheritance appears in the TypeScript status view

## Test And Validation Strategy

### QA artifact

Primary QA-facing artifact:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-test-plan-20260510-202402.md`

This plan depends on that artifact. It adds one critical requirement the QA artifact alone does not fully enforce: **post-landing parity rerun on `feat/m40-plus`**.

### Coverage diagram

```text
INTEGRATION HEAD PROOF
======================
[+] Core regression
    └── [REQUIRED] cargo test -p spec-core

[+] CLI regression
    └── [REQUIRED] cargo test -p spec-cli --test cli

[+] Positive packet proof
    └── [REQUIRED] aligned monotone-up fixture passes in TypeScript

[+] Negative packet proof
    └── [REQUIRED] unsupported near miss fails before Bun

[+] Canonical green trust surface
    └── [REQUIRED] pricing/apply_tax.unit.spec passes in TypeScript

[+] Intentional molecule rejection
    └── [REQUIRED] discount_plus_tax.test.spec fails before Bun

[+] Mixed-root status truth surface
    └── [REQUIRED] status exits 1 with expected non-green contract

LANDED BRANCH PARITY
====================
[+] Canonical green parity
    └── [REQUIRED] rerun apply_tax TypeScript proof from feat/m40-plus

[+] Mixed-root non-green parity
    └── [REQUIRED] rerun status JSON from feat/m40-plus
```

### Test gaps

There is only one meaningful remaining test gap in the current draft flow:

- **missing mandatory landed-branch parity rerun**

This plan closes that gap explicitly in Step 7. No additional broad test expansion is required for M46 closeout.

### Prior observed proof note

Prior observed proof on `ccefca8` already indicated the expected results above, including the intentionally failing molecule TypeScript surface and the intentionally non-green mixed-root status.

That prior observation makes Option A likely.

It does **not** remove the requirement to rerun the proof wall fresh before landing.

## Acceptance And Closeout Contract

### `acceptance.md` must contain

- execution timestamp
- execution root
- authoritative integrated SHA
- exact command list
- exact outcome for each command
- explicit landing-path decision

### `closeout.md` must contain

- authoritative integrated SHA
- exact landed SHA
- canonical green trust surface
- intentional non-green trust surface
- note that `.test.spec --target-language typescript` remains unsupported
- statement that future milestone selection is out of scope for this closeout

### Writing rule

Do not write "M46 shipped" language until Step 7 passes on `feat/m40-plus`.

## Error And Rescue Registry

| Failure | Why it happens | Required rescue |
| --- | --- | --- |
| proof wall fails on `ccefca8` | M46 is not actually done | stay on Option B only if the fix is bounded, otherwise stop and re-scope |
| rerun is skipped because prior proof looked good | process shortcut | rerun anyway, no exceptions |
| closeout drifts from actual results | artifact written from expectation | write only from recorded outputs |
| `feat/m40-plus` receives partial M46 truth | wrong base or cherry-pick reconstruction | land only from integration head or bounded continuation head |
| landed branch diverges from integration truth | branch moved without rerun | Step 7 is mandatory |

## Failure Modes Registry

| Risk | Severity | Test coverage | Error handling | Outcome if missed |
| --- | --- | --- | --- | --- |
| M46 closes without landing on `feat/m40-plus` | Critical | no, unless Step 7 happens | process gate only | future work starts from stale branch truth |
| manual reconstruction onto `feat/m40-plus` | Critical | no | policy gate only | competing truth surfaces |
| proof wall runs only on integration head | Critical | partial | none if skipped | false confidence about the branch that matters |
| closeout omits one or both SHAs | High | no | documentation gate only | later readers cannot verify what landed |
| mixed-root red surface is reported without the canonical green surface | High | yes, if closeout records both | documentation gate only | milestone sounds more broken than it is |
| dead-code warnings are mistaken for a landing blocker | Low | yes, warnings are visible | human judgment | unnecessary scope expansion |

Critical-gap rule for this plan:

- if post-landing rerun is missing, M46 is not done
- if closeout does not record both SHAs, M46 is not done
- if landing occurs by cherry-pick reconstruction, M46 is not done

## Non-Blocking Engineering Notes

Current proof flows still emit dead-code warnings for `status_command`, `generate_command`, `build_command`, and `test_command` in `spec-cli/src/commands.rs`.

That cleanup is **not** part of M46 closeout unless it turns into a real landing blocker.

## Worktree Parallelization Strategy

This plan has **limited safe parallelism**. The critical path is mostly sequential because most steps depend on the same truth surfaces: the integration worktree, the `.runs/m46...` artifacts, and the final branch ref.

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| freeze execution root and rerun proof wall | `.worktrees/spec-m46/integration`, `.runs/m46_helper_aware_monotone_up_typescript/` | — |
| acceptance capture | `.runs/m46_helper_aware_monotone_up_typescript/` | freeze execution root and rerun proof wall |
| closeout drafting | `.runs/m46_helper_aware_monotone_up_typescript/`, repo-root authority docs | freeze execution root and rerun proof wall |
| branch landing | git refs for `feat/m40-plus`, integration worktree | acceptance capture |
| post-landing validation | primary branch checkout, `.runs/m46_helper_aware_monotone_up_typescript/` | branch landing |
| final closeout finalize | `.runs/m46_helper_aware_monotone_up_typescript/`, repo-root authority docs | post-landing validation |

### Parallel lanes

Lane A: freeze execution root -> rerun proof wall -> acceptance capture  
Lane B: draft closeout shell after proof starts, then fill only from observed outputs  
Lane C: branch landing -> post-landing validation -> final closeout finalize

### Execution order

1. Launch Lane A first. It owns the evidence that everything else depends on.
2. Lane B may prepare structure in parallel after Lane A begins, but it may not finalize result text until Lane A completes.
3. Launch Lane C only after Lane A confirms the landing path and acceptance is written.

### Conflict flags

- Lane A and Lane B both touch `.runs/m46_helper_aware_monotone_up_typescript/`. Keep one writer responsible for final artifact content.
- Lane A and Lane C both depend on the authoritative integrated head. Do not move `feat/m40-plus` before Lane A completes.
- Lane B and Lane C both affect final closeout wording. Final closeout ownership stays with the parent after post-landing validation.

### Parallelization verdict

Treat this as **one primary sequential lane with one light drafting lane**.

This is not a good candidate for broad multi-worktree implementation fan-out.

## NOT in scope

- selecting or naming the next milestone after M46
- widening TypeScript support beyond the helper-aware monotone-up lane
- changing the mixed-root TypeScript status contract
- cleaning dead-code warnings unless they block landing
- doing new feature or architecture work while closing M46

## Completion Checklist

- [ ] execution root fixed to `ws/spec-m46-integration`
- [ ] authoritative integrated SHA recorded as `ccefca8`
- [ ] frozen proof wall rerun from integration worktree
- [ ] acceptance updated from observed outputs
- [ ] landing path explicitly chosen as Option A or Option B
- [ ] `feat/m40-plus` advanced to the landed M46 head
- [ ] narrow landed-branch rerun completed
- [ ] closeout updated with integrated SHA and landed SHA
- [ ] closeout records canonical green surface and intentional non-green surface
- [ ] repo-root authority story is coherent and final

## One-Line Summary

M46 is already built at `ccefca8`; the remaining honest work is to rerun the frozen proof wall from the integration worktree, finalize acceptance and closeout from observed results, land that truth onto `feat/m40-plus`, rerun narrow branch parity, and record both the canonical green TypeScript surface and the still-intentional mixed-root non-green status.
