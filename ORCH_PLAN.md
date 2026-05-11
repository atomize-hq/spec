# M46 Completion And Landing Orchestration Plan

## 1. Title + Metadata

Status: **authoritative orchestration plan for completing and landing the current M46 PLAN.md session**  
Supersedes: **the prior implementation-oriented `ORCH_PLAN.md`**  
Authority source: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Repo root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Current landing branch at kickoff: **`feat/m40-plus`**  
Current landing-branch HEAD at kickoff: **`991012f3f2112507d3ed9943eb96dacfc8bfa9be`**  
Authoritative integrated M46 head at kickoff: **`ccefca8`**  
Authoritative execution root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration`**  
Authoritative execution branch name: **`ws/spec-m46-integration`**  
Run artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/`**  
Primary execution mode: **one parent-owned sequential lane**  
Permitted parallelism: **one light support lane on the happy path, one bounded repair lane only if Option B activates**  
Default landing path: **Option A: fast-forward `feat/m40-plus` directly to `ccefca8` after fresh proof parity is confirmed**  
Forbidden landing path: **any cherry-pick reconstruction onto `feat/m40-plus`**  
Last rewritten: **2026-05-10**

## 2. Summary

This document is an execution contract, not a code implementation plan. M46 code is already integrated. The remaining work is operational: rerun the exact proof wall from the authoritative integration worktree, interpret the observed truth, finalize `acceptance.md` and `closeout.md` from those observations, land the integrated truth onto `feat/m40-plus`, and rerun the narrow landed-branch parity surface.

The critical path stays with the parent agent. The authoritative execution lane is `ws/spec-m46-integration` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration`. The landing lane is the repo root on `feat/m40-plus`. The only support lane permitted on the happy path is a light closeout-drafting lane. A bounded repair lane may exist only if Option B activates. Support lanes do not own branch movement, proof interpretation, path choice, or final acceptance.

The session succeeds only if the parent can prove all of the following from observed results:

- the fresh integration-root proof wall matches the expected M46 truth
- `acceptance.md` records the exact command outcomes
- `closeout.md` records the exact integrated SHA and exact landed SHA
- `feat/m40-plus` points at the landed M46 head
- the mandatory post-landing narrow rerun matches the integration truth

## 3. Hard Guards

- `PLAN.md` is the sole scope authority for this session.
- This plan must not drift into an implementation campaign. On the happy path, no product source changes are expected.
- The parent must treat `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration` at `ccefca8` as the only authoritative pre-landing execution surface.
- The parent must treat `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` on `feat/m40-plus` as the only authoritative landing and post-landing parity surface.
- The exact proof wall from `PLAN.md` is frozen and must be run verbatim.
- The exact post-landing narrow rerun from `PLAN.md` is frozen and must be run verbatim.
- `acceptance.md` and `closeout.md` under `.runs/m46_helper_aware_monotone_up_typescript/` are run artifacts. They must be written from observed results, not memory, expectation, or paraphrase.
- The closeout must preserve two explicit trust surfaces:
  - the canonical green TypeScript surface
  - the intentional non-green TypeScript status surface
- Option A is the default path.
- Option B is bounded contingency only. It may activate only if the rerun or branch move exposes one real unfinished M46 defect.
- Option B may not broaden scope, reopen milestone design, or introduce parallel code fan-out.
- Cherry-pick reconstruction onto `feat/m40-plus` is forbidden.
- If the defect is not bounded, the parent must stop the session and write `blocked.json` rather than force a landing.
- Workers do not own:
  - final proof interpretation
  - branch movement
  - path decision
  - final acceptance
  - final run-state truth
- Workers must be closed immediately after their output is merged, consumed, or rejected.

## 4. Execution Topology

### 4.1 Lane map

| Lane ID | Branch | Worktree path | Owner | Authority level | Purpose |
| --- | --- | --- | --- | --- | --- |
| `lane/m46-parent-integration-exec` | `ws/spec-m46-integration` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration` | Parent | **authoritative execution** | Verify `ccefca8`, run the full proof wall, interpret rerun truth |
| `lane/m46-parent-landing` | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | Parent | **authoritative landing** | Record kickoff branch state, land the approved head, run mandatory post-landing parity |
| `lane/m46-worker-closeout-draft` | `ws/spec-m46-closeout-draft` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/closeout-draft` | Worker | support-only | Draft closeout prose or artifact-audit notes from parent-supplied results |
| `lane/m46-worker-option-b-repair` | `ws/spec-m46-option-b-repair` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/option-b-repair` | Worker | support-only, disabled by default | Produce one bounded continuation repair on top of `ccefca8` if Option B activates |

### 4.2 Topology rules

- `lane/m46-parent-integration-exec` is the only lane allowed to determine whether the proof wall matches expected truth.
- `lane/m46-parent-landing` is the only lane allowed to move `feat/m40-plus` and the only lane allowed to decide that post-landing parity is satisfied.
- `lane/m46-worker-closeout-draft` may read parent-captured results and draft text, but it is not authoritative and may not mutate canonical branch state.
- `lane/m46-worker-option-b-repair` does not exist unless the parent explicitly activates Option B in run-state artifacts.
- The happy path is one sequential parent lane with one optional light drafting lane.
- There is no broad worker fan-out across code modules because M46 code is already integrated.

### 4.3 Expected worktree namespace

The session should use these exact worktree branches and paths:

| Branch | Path | Required state |
| --- | --- | --- |
| `ws/spec-m46-integration` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration` | checked out at authoritative integrated head `ccefca8` |
| `ws/spec-m46-closeout-draft` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/closeout-draft` | optional, support-only, created only if used |
| `ws/spec-m46-option-b-repair` | `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/option-b-repair` | optional, disabled unless Option B activates |
| `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | kickoff landing lane, starts at `991012f3f2112507d3ed9943eb96dacfc8bfa9be` |

## 5. Canonical Run-State And Artifact Surfaces

All canonical run-state authority for this session lives under:

`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/`

### 5.1 Canonical artifact set

| Path | Role | Owner |
| --- | --- | --- |
| `baseline.json` | kickoff baseline truth | Parent |
| `authority-freeze.json` | frozen execution assumptions and writable scope | Parent |
| `contract-freeze.json` | frozen proof contract and expected truth | Parent |
| `in-scope-files.txt` | exact writable surfaces for the session | Parent |
| `out-of-scope-files.txt` | explicit forbidden-touch surfaces | Parent |
| `queue.json` | current runnable queue and active lane state | Parent |
| `tasks.json` | durable task ledger with task IDs and statuses | Parent |
| `run-state.json` | session summary with SHAs, path choice, and gate state | Parent |
| `session-log.md` | chronological execution log | Parent |
| `merge-log.md` | landing path narrative and branch movement record | Parent |
| `acceptance.md` | observed command outcomes and acceptance ledger | Parent |
| `closeout.md` | final closeout record | Parent |
| `blocked.json` | required only if the session must stop instead of landing | Parent |
| `validation/` | raw command captures and verification evidence | Parent-owned tree, support lanes may write only explicitly assigned non-authoritative draft captures |

### 5.2 Required `baseline.json` contents

`baseline.json` must record:

- `run_id`
- `kickoff_timestamp`
- `repo_root`
- `landing_branch`
- `landing_branch_start_sha`
- `authoritative_integrated_branch`
- `authoritative_integrated_sha`
- `authoritative_execution_root`
- `option_a_default`
- `forbidden_path` set to cherry-pick reconstruction
- current `git status --short` for the repo root
- current `git status --short` for the integration worktree
- any pre-existing dirtiness in `.runs/m46_helper_aware_monotone_up_typescript/`

### 5.3 Required `authority-freeze.json` contents

`authority-freeze.json` must record:

- path to `PLAN.md`
- path to this `ORCH_PLAN.md`
- parent-authoritative lanes
- support-only lanes
- exact branch and worktree topology
- statement that M46 code is already integrated
- statement that the happy path expects no product-source edits
- statement that `acceptance.md` and `closeout.md` are run artifacts
- default path set to Option A
- Option B activation rule
- explicit statement that cherry-pick reconstruction is forbidden

### 5.4 Required `contract-freeze.json` contents

`contract-freeze.json` must record:

- the exact frozen proof-wall command list
- the exact frozen post-landing narrow rerun command list
- the exact expected truth for each proof-wall command
- the exact expected TypeScript status assertions
- the exact canonical green surface command
- the exact intentional non-green surface command
- the exact rule that `.test.spec --target-language typescript` remains unsupported for this milestone closeout
- the exact rule that no Rust proof inheritance may appear in the TypeScript status view
- the exact acceptance invariants from `PLAN.md`

### 5.5 Required `merge-log.md` contents

`merge-log.md` must record:

- kickoff `feat/m40-plus` SHA
- authoritative integrated SHA
- selected path: Option A or Option B
- exact landing command executed
- before and after SHAs for `feat/m40-plus`
- if Option B activates, the exact defect trigger and repair head SHA
- if a branch move is blocked, the exact reason
- final landed SHA
- final parity verdict

### 5.6 Required `acceptance.md` contents

`acceptance.md` must record:

- authoritative execution root
- authoritative integrated SHA
- the exact proof-wall command list
- per-command observed exit status
- per-command observed truth summary
- expected-versus-observed verdict for each command
- explicit statement that the mixed-root TypeScript status result is intentionally non-green
- explicit statement that `pricing/apply_tax` is the canonical green surface
- explicit path decision with one-line reason
- post-landing parity results for the narrow rerun
- final acceptance or non-acceptance verdict

### 5.7 Required `closeout.md` contents

`closeout.md` must record:

- exact integrated SHA
- exact landed SHA
- landing path actually used
- concise operator-level summary of what was completed
- canonical green surface command and observed result
- intentional non-green surface command and observed result
- exact statement that `.test.spec --target-language typescript` remains unsupported in this closeout
- statement that future milestone selection is out of scope
- any residual follow-up items that are explicitly outside M46
- final statement that M46 is or is not closed on `feat/m40-plus`

### 5.8 Allowed `tasks.json` statuses

Each `tasks.json` entry must include at least:

- `id`
- `title`
- `lane`
- `owner`
- `status`
- `depends_on`
- `owned_surfaces`
- `required_commands`
- `writes`
- `started_at`
- `completed_at`
- `notes`

Allowed `status` values are:

- `pending`
- `ready`
- `in_progress`
- `submitted`
- `blocked`
- `merged`
- `done`
- `cancelled`

### 5.9 Minimal required `validation/` tree

The session must maintain a minimal validation capture tree under:

`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m46_helper_aware_monotone_up_typescript/validation/`

Required structure:

```text
validation/
  baseline/
    repo-root-branch.txt
    repo-root-head.txt
    repo-root-status-short.txt
    integration-head.txt
    integration-status-short.txt
  proof-wall/
    01-spec-core.txt
    02-spec-cli-cli-test.txt
    03-aligned-packet-typescript.txt
    04-unsupported-near-miss-typescript.txt
    05-apply-tax-unit-typescript.txt
    06-discount-plus-tax-molecule-typescript.txt
    07-status-ecommerce-typescript.json.txt
    proof-wall-summary.md
  landing/
    landing-branch-head-before.txt
    landing-command.txt
    landing-branch-head-after.txt
  post-landing/
    01-apply-tax-unit-typescript.txt
    02-status-ecommerce-typescript.json.txt
    parity-summary.md
  closeout/
    closeout-checklist.md
  repair/
    option-b-trigger.md
    focused-repro.txt
    repair-head.txt
    repair-proof-wall-summary.md
```

### 5.10 Validation capture rules

- Every capture file must include:
  - command
  - working directory
  - timestamp
  - exit code
  - raw stdout
  - raw stderr, if any
- `validation/proof-wall/07-status-ecommerce-typescript.json.txt` must preserve the raw JSON status output exactly as emitted.
- `validation/landing/landing-command.txt` must preserve the exact branch-movement command used.
- `validation/post-landing/02-status-ecommerce-typescript.json.txt` must preserve the raw landed-branch JSON status output exactly as emitted.
- `validation/repair/` must exist only if Option B activates.
- Support lanes may contribute only explicitly assigned draft captures under `validation/closeout/` or `validation/repair/`. Canonical proof and landing captures remain parent-owned.

## 6. Workstream Plan

### 6.1 Task order

| Order | Task ID | Lane | Owner | Default state |
| --- | --- | --- | --- | --- |
| 1 | `task/m46-a1-kickoff-freeze` | `lane/m46-parent-landing` and `lane/m46-parent-integration-exec` | Parent | required |
| 2 | `task/m46-a2-proof-wall` | `lane/m46-parent-integration-exec` | Parent | required |
| 3 | `task/m46-a3-path-decision` | `lane/m46-parent-integration-exec` | Parent | required |
| 4 | `task/m46-b-closeout-draft` | `lane/m46-worker-closeout-draft` | Worker | optional, happy path only |
| 5 | `task/m46-c-option-b-repair` | `lane/m46-worker-option-b-repair` | Worker | disabled unless Option B activates |
| 6 | `task/m46-d1-land-and-verify` | `lane/m46-parent-landing` | Parent | required |
| 7 | `task/m46-d2-closeout-finalize` | `lane/m46-parent-landing` | Parent | required |

### 6.2 `task/m46-a1-kickoff-freeze`

Lane: `lane/m46-parent-landing` and `lane/m46-parent-integration-exec`  
Owner: Parent

Owned surfaces:

- `baseline.json`
- `authority-freeze.json`
- `contract-freeze.json`
- `in-scope-files.txt`
- `out-of-scope-files.txt`
- `queue.json`
- `tasks.json`
- `run-state.json`
- `session-log.md`
- `validation/baseline/*`

Required commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec branch --show-current
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec status --short
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration status --short
```

Acceptance / exit gate:

- repo root confirmed on `feat/m40-plus`
- repo-root HEAD confirmed as `991012f3f2112507d3ed9943eb96dacfc8bfa9be`
- integration worktree HEAD confirmed as `ccefca8`
- authoritative execution root recorded exactly
- proof wall frozen before any proof command is run
- writable and non-writable surfaces explicitly frozen

What gets written to run-state artifacts:

- `baseline.json` kickoff truth
- `authority-freeze.json` topology and guard freeze
- `contract-freeze.json` proof contract freeze
- `in-scope-files.txt` listing writable run artifacts only on the happy path
- `out-of-scope-files.txt` listing product source and unrelated repo surfaces as forbidden unless Option B activates
- `queue.json` setting `task/m46-a2-proof-wall` as next
- `tasks.json` marking `task/m46-a1-kickoff-freeze` done

### 6.3 `task/m46-a2-proof-wall`

Lane: `lane/m46-parent-integration-exec`  
Owner: Parent

Owned surfaces:

- `acceptance.md`
- `run-state.json`
- `session-log.md`
- `queue.json`
- `tasks.json`
- `validation/proof-wall/*`

Required commands:

```bash
cargo test -p spec-core -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/aligned/units --target-language typescript
cargo run -p spec-cli -- test semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_plus_tax.test.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
```

Acceptance / exit gate:

- all seven commands executed from `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration`
- each command captured under `validation/proof-wall/`
- expected-versus-observed comparison recorded
- status JSON assertions checked explicitly
- no landing action occurs yet

What gets written to run-state artifacts:

- `acceptance.md` updated with exact proof-wall command table and observed results
- `run-state.json` updated with `proof_wall_complete`
- `queue.json` advanced to `task/m46-a3-path-decision`
- `tasks.json` marking `task/m46-a2-proof-wall` done
- `validation/proof-wall/proof-wall-summary.md` summarizing expected versus observed truth

### 6.4 `task/m46-a3-path-decision`

Lane: `lane/m46-parent-integration-exec`  
Owner: Parent

Owned surfaces:

- `run-state.json`
- `queue.json`
- `tasks.json`
- `acceptance.md`
- `closeout.md`
- `merge-log.md`
- `blocked.json` if the session cannot proceed

Required commands:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec merge-base --is-ancestor 991012f3f2112507d3ed9943eb96dacfc8bfa9be ccefca8
```

Acceptance / exit gate:

- Option A selected if and only if the proof wall matches expected truth and the fast-forward path remains valid
- Option B selected only if one bounded unfinished M46 defect is explicitly identified
- blocked-stop selected if the defect is not bounded or if more than one substantive defect appears

What gets written to run-state artifacts:

- `run-state.json` updated with `path_choice`
- `acceptance.md` updated with path-decision reason
- `closeout.md` initialized with integrated SHA and provisional path
- `merge-log.md` initialized with kickoff and path-decision entries
- `queue.json` advanced either to `task/m46-b-closeout-draft`, `task/m46-c-option-b-repair`, or blocked state
- `blocked.json` written if the session must stop
- `tasks.json` marking `task/m46-a3-path-decision` done or blocked

### 6.5 `task/m46-b-closeout-draft`

Lane: `lane/m46-worker-closeout-draft`  
Owner: Worker  
Activation: Optional, only after `task/m46-a3-path-decision` selects Option A or after the repaired head has already passed the proof wall on Option B

Owned surfaces:

- `validation/closeout/closeout-checklist.md`
- optional non-authoritative draft notes in the closeout-draft worktree
- no canonical branch refs
- no canonical proof captures
- no canonical run-state files

Required commands:

- none required beyond any parent-assigned read-only inspection commands
- if the worker runs commands, they must be recorded and returned to the parent

Acceptance / exit gate:

- worker returns a concise closeout draft pack containing:
  - proposed acceptance wording
  - proposed closeout wording
  - artifact completeness checklist
  - unresolved assumptions, if any
- worker does not mutate canonical landing or execution truth
- worker is closed immediately after the parent consumes or rejects the draft

What gets written to run-state artifacts:

- parent may copy useful checklist notes into `validation/closeout/closeout-checklist.md`
- `tasks.json` updated by the parent to `done` or `cancelled`
- no canonical artifact becomes authoritative until the parent writes it

### 6.6 `task/m46-c-option-b-repair`

Lane: `lane/m46-worker-option-b-repair`  
Owner: Worker  
Activation: Disabled by default. Created only if the parent explicitly activates Option B.

Owned surfaces:

- exactly the bounded defect surfaces named by the parent in the Option B activation record
- `validation/repair/option-b-trigger.md`
- `validation/repair/focused-repro.txt`
- `validation/repair/repair-head.txt`
- no branch movement on `feat/m40-plus`
- no changes outside the parent-approved bounded defect surface

Required commands:

- one parent-specified focused repro command that demonstrates the defect
- any parent-specified bounded verification command
- `git rev-parse HEAD` in the repair worktree when the proposed repair head is ready

Acceptance / exit gate:

- worker produces exactly one bounded continuation repair on top of `ccefca8`
- worker returns:
  - changed files
  - commands run
  - blockers
  - unresolved assumptions
- worker does not broaden scope
- if the worker cannot keep the repair bounded, it must stop and report blocked status instead of improvising

What gets written to run-state artifacts:

- `validation/repair/option-b-trigger.md` written by the parent at activation time
- `validation/repair/focused-repro.txt` capture
- `validation/repair/repair-head.txt` with proposed repair SHA
- `queue.json` updated by the parent to re-enter `task/m46-a2-proof-wall` against the repair head
- `tasks.json` marking `task/m46-c-option-b-repair` submitted, blocked, or done
- `blocked.json` written if the repair cannot remain bounded

### 6.7 `task/m46-d1-land-and-verify`

Lane: `lane/m46-parent-landing`  
Owner: Parent

Owned surfaces:

- branch ref `feat/m40-plus`
- `merge-log.md`
- `acceptance.md`
- `closeout.md`
- `run-state.json`
- `queue.json`
- `tasks.json`
- `validation/landing/*`
- `validation/post-landing/*`

Required commands for Option A:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec checkout feat/m40-plus
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec merge --ff-only ccefca8
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
```

Required commands for Option B:

```bash
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec checkout feat/m40-plus
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec merge --ff-only <approved_repair_sha>
git -C /Users/spensermcconnell/__Active_Code/atomize-hq/spec rev-parse HEAD
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec --target-language typescript
cargo run -p spec-cli -- status examples/ecommerce --target-language typescript --format json
```

Acceptance / exit gate:

- `feat/m40-plus` moved by fast-forward only
- exact landed SHA recorded
- post-landing `apply_tax.unit.spec` TypeScript rerun matches integration truth
- post-landing TypeScript status JSON rerun matches integration truth
- no parity drift is tolerated

What gets written to run-state artifacts:

- `merge-log.md` with exact landing command and before/after SHAs
- `acceptance.md` with post-landing parity results
- `closeout.md` with landed SHA and parity result
- `run-state.json` with `landed_sha` and `post_landing_parity_complete`
- `validation/landing/*` captures
- `validation/post-landing/*` captures
- `tasks.json` marking `task/m46-d1-land-and-verify` done or blocked

### 6.8 `task/m46-d2-closeout-finalize`

Lane: `lane/m46-parent-landing`  
Owner: Parent

Owned surfaces:

- `acceptance.md`
- `closeout.md`
- `merge-log.md`
- `queue.json`
- `tasks.json`
- `run-state.json`
- `session-log.md`
- `blocked.json` if final acceptance fails

Required commands:

- no new product-validation commands required beyond any final SHA or status reads needed to complete the record
- if any final verification command is run, it must be captured in `validation/closeout/`

Acceptance / exit gate:

- all gates have passed
- `acceptance.md` contains the exact command outcomes and final verdict
- `closeout.md` contains exact integrated and landed SHAs plus the two trust surfaces
- all canonical artifacts agree on the final truth
- if any final inconsistency remains, the session stops in blocked state rather than declaring completion

What gets written to run-state artifacts:

- final `acceptance.md`
- final `closeout.md`
- final `merge-log.md`
- `run-state.json` with `final_acceptance_state`
- `queue.json` closed out
- `tasks.json` marking session-complete tasks done
- `blocked.json` if final acceptance cannot be truthfully declared

## 7. Gate Definitions

### Gate 1: Kickoff integrity

Pass only if:

- repo root is `feat/m40-plus`
- kickoff SHA is `991012f3f2112507d3ed9943eb96dacfc8bfa9be`
- authoritative integration SHA is `ccefca8`
- authoritative execution root is recorded exactly
- proof and landing topology are frozen in run-state artifacts

### Gate 2: Proof-wall completeness

Pass only if:

- all seven proof-wall commands have been run from `ws/spec-m46-integration`
- each command has a capture file under `validation/proof-wall/`
- each command has an expected-versus-observed entry in `acceptance.md`

### Gate 3: Truth match and path choice

Pass only if:

- Option A or Option B is explicitly selected
- the reason for the choice is recorded
- blocked-stop is chosen instead of improvisation if the defect is not bounded

### Gate 4: Landing eligibility

Pass only if:

- the approved landed head is identified exactly
- the branch move is a fast-forward
- cherry-pick reconstruction is not used

### Gate 5: Post-landing parity

Pass only if:

- the landed-branch `apply_tax.unit.spec` TypeScript rerun matches the integration truth
- the landed-branch TypeScript status JSON rerun matches the integration truth
- parity is recorded in both `acceptance.md` and `closeout.md`

### Gate 6: Closeout completeness

Pass only if:

- `acceptance.md` contains the full proof-wall ledger
- `closeout.md` contains exact integrated and landed SHAs
- the canonical green surface is named and observed
- the intentional non-green surface is named and observed
- all canonical run-state artifacts agree on the final result

## 8. Context-Control Rules

- The parent keeps a deliberately small active context set:
  - `PLAN.md`
  - this `ORCH_PLAN.md`
  - exact kickoff SHA and integrated SHA
  - exact proof-wall command list
  - exact post-landing rerun command list
  - `.runs/m46_helper_aware_monotone_up_typescript/` canonical artifacts
- The parent does not hold broad implementation context unless Option B activates and a bounded defect requires it.
- Workers receive only a minimal task packet containing:
  - relevant `PLAN.md` excerpt
  - relevant `ORCH_PLAN.md` excerpt
  - task ID
  - owned paths
  - required commands
  - expected outputs
  - forbidden-touch surfaces
- Workers must not be asked to infer missing milestone scope or reinterpret expected truth.
- Workers must return only:
  - changed files or draft outputs
  - commands run
  - blockers
  - unresolved assumptions
- Workers must not return broad repo advice, substitute test plans, or parallelize new work on their own.
- The closeout drafting worker must be closed immediately after the parent consumes or rejects the draft.
- The Option B repair worker must be closed immediately after:
  - the repair head is accepted for parent rerun
  - the repair is rejected
  - the task is blocked
- If any worker starts touching non-owned surfaces, the parent must terminate that lane and keep the session in the primary parent lane.
- The parent remains the only source of truth for run-state artifacts and the only agent allowed to declare completion.

## 9. Tests And Acceptance

### 9.1 Kickoff integrity checklist

- [ ] `feat/m40-plus` confirmed at repo root
- [ ] kickoff SHA recorded as `991012f3f2112507d3ed9943eb96dacfc8bfa9be`
- [ ] `ws/spec-m46-integration` confirmed at `ccefca8`
- [ ] authoritative execution root recorded exactly
- [ ] `baseline.json`, `authority-freeze.json`, and `contract-freeze.json` written
- [ ] `in-scope-files.txt` and `out-of-scope-files.txt` written

### 9.2 Proof-wall checklist

- [ ] `cargo test -p spec-core -- --color never` run and captured
- [ ] `cargo test -p spec-cli --test cli -- --color never` run and captured
- [ ] aligned packet TypeScript test run and captured
- [ ] unsupported near miss TypeScript test run and captured
- [ ] `apply_tax.unit.spec` TypeScript test run and captured
- [ ] `discount_plus_tax.test.spec` TypeScript test run and captured
- [ ] TypeScript `spec status` JSON run and captured
- [ ] expected-versus-observed verdict written for all seven commands
- [ ] status JSON assertions checked explicitly
- [ ] intentional non-green status result called out explicitly

### 9.3 Path-decision checklist

- [ ] Option A or Option B selected explicitly
- [ ] one-line reason recorded in `acceptance.md`
- [ ] one-line reason recorded in `merge-log.md`
- [ ] `run-state.json` updated with `path_choice`
- [ ] blocked-stop selected instead of improvisation if the defect is not bounded

### 9.4 Landing checklist

- [ ] pre-landing `feat/m40-plus` SHA captured
- [ ] exact landing command captured in `validation/landing/landing-command.txt`
- [ ] landing performed by fast-forward only
- [ ] exact landed SHA captured
- [ ] `merge-log.md` updated with before and after SHAs
- [ ] no cherry-pick reconstruction used

### 9.5 Post-landing parity checklist

- [ ] landed-branch `apply_tax.unit.spec` TypeScript rerun executed
- [ ] landed-branch TypeScript status JSON rerun executed
- [ ] both captures written under `validation/post-landing/`
- [ ] canonical green surface matches integration truth
- [ ] intentional non-green status surface matches integration truth
- [ ] parity result recorded in `acceptance.md`
- [ ] parity result recorded in `closeout.md`

### 9.6 Closeout completeness checklist

- [ ] `acceptance.md` contains exact proof-wall command outcomes
- [ ] `closeout.md` contains exact integrated SHA `ccefca8`
- [ ] `closeout.md` contains exact landed SHA
- [ ] canonical green surface command is named exactly
- [ ] intentional non-green surface command is named exactly
- [ ] `.test.spec --target-language typescript` remaining unsupported is stated explicitly
- [ ] `run-state.json`, `merge-log.md`, `acceptance.md`, and `closeout.md` agree
- [ ] final acceptance decision is truthfully recorded

## 10. Contingency Matrix

| Condition | Parent action | Worker action | Result |
| --- | --- | --- | --- |
| proof wall matches expected truth exactly | select Option A | closeout drafting worker may optionally assist | land `ccefca8`, run parity, finalize closeout |
| one command fails due to obvious environment noise | allow one documented retry | none required | remain in current task |
| one real unfinished M46 defect appears and is bounded | activate Option B | bounded repair worker may repair only named surfaces | parent reruns full proof wall against repair head |
| defect is not bounded | write `blocked.json` | no repair lane or immediate repair-lane shutdown | stop and re-scope |
| branch move exposes parity drift | stop landing session | no broad recovery lane | session remains incomplete until resolved |
| any recovery path requires cherry-pick reconstruction | reject path immediately | none | stop and re-scope |

### 10.1 Option B activation protocol

Option B activates only through this protocol:

1. The parent completes `task/m46-a2-proof-wall` and identifies one real unfinished M46 defect.
2. The parent records the trigger in `validation/repair/option-b-trigger.md`.
3. The parent updates `run-state.json` with:
   - `path_choice: option_b`
   - `option_b_trigger`
   - bounded defect summary
4. The parent updates `tasks.json` to set `task/m46-c-option-b-repair` to `ready`.
5. The parent creates or enables `ws/spec-m46-option-b-repair` at `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/option-b-repair`.
6. The parent gives the repair worker only:
   - the exact defect statement
   - the exact allowed touch surfaces
   - the exact focused repro command
   - the explicit ban on scope expansion
7. The repair worker produces one bounded continuation head on top of `ccefca8` or returns blocked.
8. The parent records the proposed repair SHA in `validation/repair/repair-head.txt`.
9. The parent reruns the full frozen proof wall against the repair head before any landing action occurs.
10. Only if the repair head now matches the same expected truth may the parent proceed to `task/m46-d1-land-and-verify`.

### 10.2 Option B boundaries

The repair worker may own only:

- the smallest set of defect-specific surfaces explicitly named by the parent
- focused repro and repair validation captures
- one continuation head on top of `ccefca8`

The repair worker may not own:

- branch movement on `feat/m40-plus`
- run-state truth
- final proof interpretation
- milestone-scope changes
- documentation rewrites unrelated to the bounded defect
- broad cleanup

### 10.3 Blocked-stop protocol

If the defect is not bounded, if more than one substantive defect appears, or if the repair worker cannot stay within the approved surfaces:

- the parent must stop the session
- `blocked.json` must be written
- `queue.json` must show blocked state
- `tasks.json` must mark the blocking task as `blocked`
- `closeout.md` must remain non-final
- no landing action occurs

## 11. Assumptions

- `PLAN.md` is the current and authoritative M46 completion plan.
- `ccefca8` is the authoritative integrated M46 head at kickoff.
- `/Users/spensermcconnell/__Active_Code/atomize-hq/.worktrees/spec-m46/integration` is the authoritative execution root at kickoff.
- `feat/m40-plus` at kickoff is `991012f3f2112507d3ed9943eb96dacfc8bfa9be`.
- `.runs/m46_helper_aware_monotone_up_typescript/` remains the canonical run-artifact root for this session.
- The happy path is operational only and should not require product-source edits.
- The expected M46 truth still includes:
  - one canonical green TypeScript surface
  - one intentional non-green TypeScript status surface
- The closeout drafting lane is optional and support-only.
- The Option B repair lane is disabled by default and exists only if the parent explicitly activates it.
- If any fact here changes during execution, the parent must update run-state artifacts and continue to honor the hard guards rather than silently drifting scope.
