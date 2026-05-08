# M40+ Orchestration Plan

Status: **authoritative execution contract for M40+**
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**
Milestone family: **operator-consumer-tooling**
Post-fix replay winner target: **shared-core-portability**
Live checkout: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**
Live branch: **`feat/m40-plus`**
Live HEAD at rewrite time: **`ed93e453471ccf1d6b57a8a733d5790b4ec40799`**
Review base: **`main`**
Last rewritten: **`2026-05-08`**
Canonical run root for an executed M40+ session: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_selector_contract_hardening`**
Execution note: **`PLAN.md` is the sole authority. This file operationalizes that authority from kickoff through closeout. The pre-refresh repo-root orchestration context remains replay input and must be captured before any future execution relies on this file.**

## Summary

- M40+ is a contract-hardening milestone, not an infrastructure milestone.
- The implementation surface is intentionally closed around:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md`
- The current failure is one contradictory selector rule plus an under-specified output contract. The fix is to remove planning-as-output, force one product-lane winner, and push planning only into readiness and handoff.
- Replay validation is mandatory. The fix is incomplete if the captured `feat/m40-plus` branch truth does not still resolve to `shared-core-portability` as the winner while keeping planning only in readiness and handoff.
- The parent agent remains the sole integrator, scope owner, gate owner, acceptance owner, and closeout author.
- Honest concurrency is narrow. There is no real parallel write window. At most, a read-only support lane can collect evidence while the parent edits one of the two contract files.

## Hard Guards

- `PLAN.md` wins over this file, memory, prior notes, prior run artifacts, and stale repo-root orchestration context if they disagree.
- The parent owns only this authored file in the repo during this rewrite. Future M40+ implementation work may touch only the two closed-surface contract files unless a concrete gap proves otherwise.
- The following paths are read-only unless a concrete, documented contract gap is proven first:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/scripts/collect_signals.sh`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/agents/openai.yaml`
- If the selector cannot be fixed while preserving a product-lane winner and keeping planning only in readiness and handoff, the session stops. Do not widen scope to collector changes, YAML changes, or code changes as a first reaction.
- No edits are authorized to:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
  - any Rust source
  - any `xtask` command path
  - any historical `.runs/**` artifact
  - any replay input outside the canonical run root for this session
- Hard-banned final outputs after the fix remain:
  - `planning`
  - `planning milestone next`
  - `author a plan`
  - `no milestone`
  - `more evidence`
- Replay acceptance must use captured branch truth, not this rewritten file as self-proof.
- The parent must preserve unrelated local work. Current branch state is already dirty at draft time; do not normalize or revert other actors' changes.

## Closed Implementation Surface

| Path | Role in M40+ | Ownership | Allowed action |
|---|---|---|---|
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` | active authority artifact | read-only authority | no writes |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md` | execution contract | parent only | authored edit for this rewrite only |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md` | primary selector contract | future M40+ implementation surface | edit |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md` | scoring mirror and hard gates | future M40+ implementation surface | edit after `SKILL.md` freeze |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/scripts/collect_signals.sh` | live signal collector | read-only unless concrete gap proven | read by default |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/agents/openai.yaml` | secondary prompt summary | read-only unless concrete divergence proven | read by default |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_selector_contract_hardening/**` | canonical run-state for an executed session | parent only | run-state writes only |

Rules:

- The closed implementation surface above is the full honest blast radius for M40+.
- `SKILL.md` freezes first. `rubric.md` mirrors second. Replay validation happens last.
- Any edit outside the two contract files is out of contract unless the parent first records the proven gap, the blocked replay condition, and the exact scope-expansion rationale in canonical run-state.
- Changes to `collect_signals.sh` or `agents/openai.yaml` are exception paths, not planned workstreams.

## Branch, Worktree, And Ownership Layout

Repository root:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

Execution layout:

| Role | Branch | Worktree | Writes allowed |
|---|---|---|---|
| parent integrator | `feat/m40-plus` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `ORCH_PLAN.md` now; later M40+ execution surface only |
| optional support lane `S1` | none required | same checkout, read-only | no repo writes |

Rules:

1. No worktree split is required for this milestone.
2. There is no honest parallel editing lane because `rubric.md` depends on the exact wording frozen in `SKILL.md`.
3. The only real parallel window is optional read-only evidence collection by `S1` while the parent authors or reviews one contract file.
4. The parent remains the sole merge point, stale-lane invalidator, and final gate owner.
5. If the branch head or the active `PLAN.md` content changes during execution, the parent must re-freeze authority before continuing.

## Execution Topology

- One repository checkout only:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec`
- No dedicated worker branches.
- No dedicated worker worktrees.
- All workers, if spawned, run in the same checkout under parent control.
- This is deliberate, not omitted:
  - the milestone is contract-hardening over two tightly coupled files
  - introducing extra worktrees would create merge overhead without creating a real parallel write window
  - replay inputs must reflect one branch truth snapshot, not per-worktree variants

## Execution Actor Model

### Actor inventory

| Actor ID | Role | May write repo files | May write canonical run-state | Spawn conditions | Mandatory close condition |
|---|---|---|---|---|---|
| `P0` | parent integrator and gate owner | yes, but only within milestone-owned surface | yes | always active | closes only after final closeout or blocked stop |
| `R1` | optional read-only baseline and replay worker | no | no | parent needs help capturing branch truth or replay evidence | closes immediately after returning captured facts |
| `W1` | optional `SKILL.md` rewrite worker | yes, `SKILL.md` only | no | parent decides to delegate drafting after authority freeze | closes before `gate-m40p-15-skill-readback` begins |
| `W2` | optional `rubric.md` parity worker | yes, `rubric.md` only | no | parent has frozen `SKILL.md` wording and wants delegated parity editing | closes before `gate-m40p-25-rubric-readback` begins |

### Actor rules

- `P0` is the sole integrator, queue owner, gate owner, acceptance owner, and closeout author.
- `R1` is read-only. It may inspect `HEAD`, replay inputs, restore snapshots, checkpoint context, and collector outputs. It may not edit repo files or claim any gate passed.
- `W1` may exist only after `gate-m40p-05-authority-freeze` passes. It may touch only:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md`
- `W2` may exist only after `gate-m40p-15-skill-readback` passes. It may touch only:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md`
- `W1` and `W2` are serialized. They are never live at the same time.
- Workers never own gates, never advance `queue.json`, and never write `blocked.json`, `run-state.json`, `acceptance.md`, or `closeout.md`.
- Any worker becomes stale immediately if:
  - branch HEAD changes
  - working-tree `PLAN.md` changes
  - `P0` changes the frozen contract wording the worker was following
- Stale workers must be closed, their output re-reviewed by `P0`, and the affected gate restarted from the last valid freeze point.

## Canonical Run-State

Canonical run root for an executed M40+ session:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_selector_contract_hardening
```

Canonical parent-owned files:

- `baseline.json`
- `authority-freeze.json`
- `run-state.json`
- `tasks.json`
- `queue.json`
- `session-log.md`
- `replay-inputs/active-plan.md`
- `replay-inputs/head-orch-plan.md`
- `replay-inputs/restore-point.md`
- `replay-inputs/m39-closeout.md`
- `validation/collect-signals.stdout.txt`
- `validation/contract-readback.stdout.txt`
- `validation/ban-language.stdout.txt`
- `validation/replay-summary.md`
- `validation/diff-scope.stdout.txt`
- `blocked.json`
- `acceptance.md`
- `closeout.md`

Required contents:

- `baseline.json`
  - branch name
  - HEAD SHA
  - dirty-state summary
  - whether working-tree `PLAN.md` differs from `HEAD`
  - whether `HEAD:ORCH_PLAN.md` was captured as replay input
  - whether the restore snapshot named in `PLAN.md` is reachable
- `authority-freeze.json`
  - active authority path
  - closed implementation surface snapshot
  - replay winner target
  - exact stop rule if planning reappears as the final answer
- `replay-inputs/active-plan.md`
  - exact working-tree authority file used for execution
- `replay-inputs/head-orch-plan.md`
  - exact pre-refresh repo-root orchestration context from `HEAD`, not the post-refresh working copy
- `replay-inputs/restore-point.md`
  - the restore snapshot named in the HTML comment at the top of `PLAN.md`
- `replay-inputs/m39-closeout.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m39_verification_consumer_probe/closeout.md`
- `validation/replay-summary.md`
  - the captured-truth winner
  - readiness result
  - handoff result
  - ranked alternates and loser reasons
  - explicit statement that planning did not become the winner
- `closeout.md`
  - final verdict
  - touched files
  - replay result
  - explicit statement that the fix remained within contract

## Queue And Gate Model

Canonical queue:

| Order | ID | Kind | Owner | Success outputs |
|---|---|---|---|---|
| 1 | `gate-m40p-00-baseline-freeze` | gate | parent | `baseline.json`, `session-log.md`, `run-state.json` |
| 2 | `gate-m40p-05-authority-freeze` | gate | parent | `authority-freeze.json`, `replay-inputs/*`, `tasks.json`, `queue.json` |
| 3 | `task-m40p-10-skill-contract` | task | parent | updated `SKILL.md`, `session-log.md` |
| 4 | `gate-m40p-15-skill-readback` | gate | parent | `validation/contract-readback.stdout.txt`, `acceptance.md` |
| 5 | `task-m40p-20-rubric-parity` | task | parent | updated `rubric.md`, `session-log.md` |
| 6 | `gate-m40p-25-rubric-readback` | gate | parent | `validation/ban-language.stdout.txt`, `acceptance.md` |
| 7 | `gate-m40p-30-replay-validation` | gate | parent, optional read-only support from `S1` | `validation/collect-signals.stdout.txt`, `validation/replay-summary.md` |
| 8 | `gate-m40p-35-diff-scope` | gate | parent | `validation/diff-scope.stdout.txt`, `acceptance.md` |
| 9 | `gate-m40p-40-closeout` | gate | parent | `closeout.md`, final `run-state.json`, final `queue.json` |

Queue rules:

- This queue is strictly serialized for writes.
- `rubric.md` work cannot start until `SKILL.md` language is frozen.
- Replay validation cannot start until both contract files pass readback.
- If any gate fails, the parent writes `blocked.json`, marks the queue blocked at that gate, and stops without informal continuation.
- The parent alone may advance `queue.json`.

## Blocked-State And Restart Semantics

When any gate fails, `P0` must write both `blocked.json` and `run-state.json` before stopping.

Required `blocked.json` fields:

- `gate_id`
- `failed_at`
- `branch`
- `head_sha`
- `authority_path`
- `replay_winner_target`
- `failure_class`
- `blocking_evidence`
- `resume_from_gate`
- `requires_reauthority`
- `notes`

Required `run-state.json` fields on failure:

- `status: blocked`
- `active_gate`
- `last_passing_gate`
- `branch`
- `head_sha`
- `authority_frozen`
- `worker_state`
- `next_action`

Restart rules:

- If `gate-m40p-00-baseline-freeze` fails, restart from the beginning.
- If `gate-m40p-05-authority-freeze` fails, restart from `gate-m40p-00-baseline-freeze`.
- If any later gate fails and branch HEAD plus working-tree `PLAN.md` are unchanged, resume from the failed gate after parent review.
- If branch HEAD changes after `gate-m40p-05-authority-freeze`, all in-flight worker output is stale and restart must begin at `gate-m40p-00-baseline-freeze`.
- If working-tree `PLAN.md` changes at any point after authority freeze, restart must begin at `gate-m40p-00-baseline-freeze`.
- If replay failure proves the winner target changed, do not resume. Stop and require a new authority artifact.
- If a read-only exception path would need to become writable, do not resume inside the same run. Stop and require re-authority or follow-on planning.

## Parent-Owned Integration Rules

- The parent is the only actor allowed to interpret `PLAN.md`, freeze the winner-vs-handoff split, or declare the replay winner acceptable.
- `S1`, if used at all, is limited to read-only evidence collection:
  - run `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/scripts/collect_signals.sh`
  - capture grep readback outputs
  - summarize concrete divergence findings for the parent
- `S1` may not:
  - edit repo files
  - update canonical run-state
  - declare a gate passed
  - reinterpret the milestone winner independently
- Integration happens only at the parent gate boundaries:
  1. freeze authority
  2. land `SKILL.md`
  3. mirror `rubric.md`
  4. run replay
  5. inspect diff scope
  6. close out

## Workstream Plan

### WS0 - Baseline And Replay Freeze

IDs:

- `gate-m40p-00-baseline-freeze`
- `gate-m40p-05-authority-freeze`

Purpose:

- capture the live branch truth that acceptance must replay against
- preserve the pre-refresh orchestration context before any later M40+ contract edits
- freeze the active working-tree `PLAN.md` as the sole authority input

Required actions:

1. record branch, HEAD, and dirty-state facts
2. snapshot working-tree `PLAN.md` to `replay-inputs/active-plan.md`
3. snapshot `HEAD:ORCH_PLAN.md` to `replay-inputs/head-orch-plan.md`
4. copy the restore snapshot named in the HTML comment at the top of `PLAN.md`
5. copy `.runs/m39_verification_consumer_probe/closeout.md`

Task-level command list:

```bash
git branch --show-current
git rev-parse HEAD
git status --short
git show HEAD:ORCH_PLAN.md
sed -n '1,40p' /Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md
cp /Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_selector_contract_hardening/replay-inputs/active-plan.md
git show HEAD:ORCH_PLAN.md > \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_selector_contract_hardening/replay-inputs/head-orch-plan.md
cp /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260508-105337.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_selector_contract_hardening/replay-inputs/restore-point.md
cp /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m39_verification_consumer_probe/closeout.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_plus_selector_contract_hardening/replay-inputs/m39-closeout.md
```

Worker usage:

- `R1` may be spawned here to gather branch facts and replay inputs.
- `R1` must close before `gate-m40p-05-authority-freeze` is marked passed.

Gate passes only if:

- the parent can identify one active authority file
- replay inputs preserve branch truth without using the post-fix contract as evidence
- the replay winner target remains `shared-core-portability`

Acceptance for `gate-m40p-00-baseline-freeze`:

- branch, HEAD, and dirty-state facts are captured exactly once in `baseline.json`
- current working-tree `PLAN.md` is frozen as the active authority input
- `HEAD:ORCH_PLAN.md` is preserved as pre-refresh orchestration truth

Acceptance for `gate-m40p-05-authority-freeze`:

- the restore snapshot named in `PLAN.md` is captured into canonical run-state
- `.runs/m39_verification_consumer_probe/closeout.md` is captured into canonical run-state
- `authority-freeze.json` records the replay winner target and stop conditions
- any worker used for baseline capture is closed

Blocked-path behavior:

- if the active authority cannot be frozen, stop
- if the pre-refresh orchestration context cannot be recovered, stop
- if branch truth already implies the winner changed, stop and re-authority rather than improvising

### WS1 - Rewrite The Selector Contract

IDs:

- `task-m40p-10-skill-contract`
- `gate-m40p-15-skill-readback`

Purpose:

- fix the primary contract in `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md`

Required edits:

1. remove the planning-as-output escape hatch
2. make the winner-vs-handoff split explicit
3. require `Executable wedge` and `Confidence`
4. require ranked alternates with loser reasons
5. state that `required_next_action = author_*_plan` affects readiness and handoff only
6. state that `recommendation_status = no_strong_candidate` does not authorize a null final answer
7. anchor replay expectations for the captured `feat/m40-plus` truth

Task-level command list:

```bash
rg -n "planning milestone|best honest answer|Executable wedge|Confidence" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
sed -n '240,340p' \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md
```

Worker usage:

- `W1` may be spawned only after `gate-m40p-05-authority-freeze` passes.
- `W1` may draft or patch `SKILL.md` only.
- `W1` must close before `gate-m40p-15-skill-readback`.

Readback gate passes only if:

- no contradictory planning-as-output wording remains
- the output block is stable and exact enough to force one winner
- the winner remains a concrete product-surface wedge, not a planning label

Validation commands:

```bash
rg -n "planning milestone|best honest answer|Executable wedge|Confidence" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
rg -n "Ranked alternates:|Implementation readiness:|Next artifact kind:|Autoplan ready:" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md
```

Acceptance for `task-m40p-10-skill-contract`:

- `SKILL.md` is the only contract file touched during this task
- the planning-as-output escape hatch is removed or rewritten into a ban-compatible rule
- the output block now names executable wedge, confidence, readiness, artifact kind, and ranked alternates

Acceptance for `gate-m40p-15-skill-readback`:

- grep/readback outputs show one stable winner-oriented contract shape
- readback shows planning moved entirely into readiness and handoff semantics
- `W1`, if used, is closed and its output accepted by `P0`

Blocked-path behavior:

- if the parent cannot delete or rewrite the contradictory rule without changing the milestone taxonomy, stop
- if fixing `SKILL.md` appears to require collector or YAML edits, stop and prove the gap before widening scope

### WS2 - Mirror The Rubric Contract

IDs:

- `task-m40p-20-rubric-parity`
- `gate-m40p-25-rubric-readback`

Purpose:

- make `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md` match the frozen selector contract exactly enough to remove ambiguity

Required edits:

1. mirror the hard bans
2. force one winner and explicit loser reasons
3. state that blocked readiness does not demote the winner into planning
4. state that future trigger rows and not-yet-fired authorization branches cannot win
5. preserve the existing scoring axes unless `SKILL.md` readback proves a mismatch

Task-level command list:

```bash
sed -n '1,260p' \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
rg -n "planning milestone next|author a plan|no milestone|more evidence|Ranked alternates|winner" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
```

Worker usage:

- `W2` may be spawned only after `gate-m40p-15-skill-readback` passes.
- `W2` may patch `rubric.md` only.
- `W2` must close before `gate-m40p-25-rubric-readback`.

Readback gate passes only if:

- `rubric.md` and `SKILL.md` agree on final-answer shape
- no new scoring engine, artifact type, or command path appears
- planning remains handoff-only when readiness is blocked

Validation commands:

```bash
rg -n "planning milestone next|author a plan|no milestone|more evidence|Ranked alternates|winner" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
rg -n "blocked readiness|future trigger|not-yet-triggered|forced-ranking|loser" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
```

Acceptance for `task-m40p-20-rubric-parity`:

- `rubric.md` mirrors the frozen contract instead of inventing a new one
- no new scoring machinery or command paths appear
- `W2`, if used, touched only `rubric.md`

Acceptance for `gate-m40p-25-rubric-readback`:

- parity readback shows the same hard bans and one-winner requirement in both files
- readback shows blocked readiness cannot demote the winner into planning
- `W2`, if used, is closed and its output accepted by `P0`

Blocked-path behavior:

- if parity requires widening beyond the two contract files, stop
- if the readback still allows a planning winner, stop

### WS3 - Replay Validation Against Captured Branch Truth

ID:

- `gate-m40p-30-replay-validation`

Purpose:

- prove the contract fix preserves the intended product-lane winner for the captured branch truth

Replay inputs:

- `replay-inputs/active-plan.md`
- `replay-inputs/head-orch-plan.md`
- `replay-inputs/restore-point.md`
- `replay-inputs/m39-closeout.md`
- live output from `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/scripts/collect_signals.sh`

Expected replay result:

- winning family stays product-lane and resolves to `shared-core-portability`
- the output names a concrete executable wedge
- `Implementation readiness` stays blocked honestly
- planning appears only in `Next artifact kind`, `Autoplan ready`, and `Handoff`
- loser reasons are explicit
- evidence cites both:
  - `pivot_to_architecture_shared_core_follow_on`
  - `author_architecture_follow_on_plan`

Validation commands:

```bash
/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/scripts/collect_signals.sh
rg -n "Executable wedge:|Confidence:|Ranked alternates:|Implementation readiness:" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md
```

Replay verdict procedure:

1. `P0` captures live collector output into `validation/collect-signals.stdout.txt`.
2. `P0` compares collector truth against:
   - `replay-inputs/active-plan.md`
   - `replay-inputs/head-orch-plan.md`
   - `replay-inputs/restore-point.md`
   - `replay-inputs/m39-closeout.md`
3. `P0` writes `validation/replay-summary.md` with:
   - winner family
   - concrete executable wedge
   - implementation readiness
   - next artifact kind
   - autoplan readiness
   - ranked alternates and loser reasons
   - evidence lines proving planning remained handoff-only
4. `P0` records pass or block in `acceptance.md` and advances or blocks the queue.

Worker usage:

- `R1` may be respawned here as read-only replay support.
- `R1` must close before `gate-m40p-30-replay-validation` is marked passed or blocked.

Gate passes only if:

- replay yields a product-lane winner
- readiness remains blocked without becoming the answer
- no replay evidence requires collector or YAML changes

Acceptance for `gate-m40p-30-replay-validation`:

- `validation/replay-summary.md` names `shared-core-portability` as the winner
- the replay result includes a concrete executable wedge rather than a planning label
- the replay result cites both `pivot_to_architecture_shared_core_follow_on` and `author_architecture_follow_on_plan`
- planning appears only in readiness, artifact kind, autoplan readiness, or handoff
- any replay support worker is closed

Blocked-path behavior:

- if replay produces another planning-winner answer, stop
- if replay can be made truthful only by changing read-only files, stop and record the exact missing signal or divergence
- if the winner is no longer `shared-core-portability`, stop and request new authority

### WS4 - Diff Scope And Closeout

IDs:

- `gate-m40p-35-diff-scope`
- `gate-m40p-40-closeout`

Purpose:

- verify that the landed implementation stayed inside the closed surface
- finish with one unambiguous contract result

Required checks:

```bash
git diff --stat -- \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
git diff --stat -- /Users/spensermcconnell/__Active_Code/atomize-hq/spec
git status --short -- /Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

Diff gate passes only if:

- the only intentional M40+ implementation edits are the two contract files
- this orchestration file remains the only repo edit for the present rewrite session
- no read-only file changed without a recorded scope-expansion decision

Closeout requirements:

- `closeout.md` must name one successful verdict only:
  - `selector contract hardening complete`
- `closeout.md` must also state:
  - replay winner stayed product-lane
  - planning remained in readiness and handoff only
  - no unauthorized files changed
  - any exception path was blocked rather than smuggled in

Closeout artifact list:

- `validation/diff-scope.stdout.txt`
- final `acceptance.md`
- final `run-state.json`
- final `queue.json`
- final `closeout.md`

Acceptance for `gate-m40p-35-diff-scope`:

- diff-scope output isolates the two contract files for executed M40+ implementation work
- current rewrite session changed only `ORCH_PLAN.md`
- no read-only file appears in the intentional edit set

Acceptance for `gate-m40p-40-closeout`:

- `closeout.md` contains the single allowed success verdict
- `closeout.md` lists touched files and replay outcome explicitly
- `run-state.json` ends in a terminal `complete` or `blocked` state with no ambiguous in-between status
- all optional workers are closed

Blocked-path behavior:

- if diff scope widened, stop
- if closeout wording implies a collector, YAML, or code-path fix landed when it did not, stop

## Context-Control Rules

- Primary authority is intentionally narrow:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md`
  - the restore snapshot named in `PLAN.md`
  - `.runs/m39_verification_consumer_probe/closeout.md`
- Read `collect_signals.sh` and `agents/openai.yaml` only to confirm they do not already contradict the frozen contract.
- Do not reopen unrelated code inspection. M40+ is not a Rust design or implementation milestone.
- Do not fabricate extra lanes, new artifact kinds, or new prompt surfaces to make the plan look busier than it is.
- Every acceptance claim must tie back to a concrete file snapshot, command output, or replay result captured under the canonical run root.

## Tests And Acceptance

### Mandatory content acceptance

- This file operationalizes the active M40+ `PLAN.md`, not the stale M40 orchestration contract.
- It preserves the winner-vs-handoff split as the central contract change.
- It keeps the closed implementation surface limited to `SKILL.md` and `rubric.md`, with documented read-only exceptions only.
- It names the replay winner target as `shared-core-portability`.
- It makes the parent the sole integrator and gate owner.
- It tells the operator exactly when to stop on replay failure or scope drift.

### Mandatory implementation acceptance for an executed M40+ session

1. `SKILL.md` no longer contains a planning-as-output escape hatch.
2. `SKILL.md` includes `Executable wedge:` and `Confidence:`.
3. `rubric.md` mirrors the same hard bans and forced-ranking rules.
4. Replay against captured branch truth yields a product-lane winner and blocked readiness, not a planning answer.
5. No file outside the closed implementation surface changed unless a proven gap was documented and the run was blocked for re-authority.

### Mandatory validation commands

```bash
/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/scripts/collect_signals.sh
rg -n "planning milestone|best honest answer|Executable wedge|Confidence" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
rg -n "planning milestone next|author a plan|no milestone|more evidence" \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/SKILL.md \
  /Users/spensermcconnell/__Active_Code/atomize-hq/spec/.agents/skills/next-milestone/references/rubric.md
git diff --stat -- /Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

### Completion conditions

M40+ is complete only when all of the following are true:

1. the selector contract forces one product-lane winner
2. the rubric no longer leaves room for planning as the answer
3. replay preserves `shared-core-portability` as the winner on captured branch truth
4. readiness and handoff carry the planning artifact honestly
5. no unauthorized scope expansion landed
6. closeout records one clean success verdict

## Assumptions

- The working-tree `PLAN.md` is the active authority even though it is already modified relative to `HEAD`.
- `HEAD:ORCH_PLAN.md` is the correct historical repo-root orchestration context to preserve for replay after this refresh.
- `shared-core-portability` remains the correct replay winner until a new authority artifact says otherwise.
- `collect_signals.sh` already exposes the fields needed to keep planning in readiness and handoff only.
- `agents/openai.yaml` already reflects the desired winner-vs-handoff split closely enough that it should remain untouched unless replay proves a concrete divergence.
- The weakest point in the plan is replay capture hygiene: if the pre-refresh orchestration context or restore snapshot is not frozen correctly, a later run could accidentally use the post-fix contract as its own evidence. That is a stop condition, not something to paper over.
