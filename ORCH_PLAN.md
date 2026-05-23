# I7 Orchestration Plan

Status: **authoritative execution runbook and closeout record**  
Milestone: **I7 Rust V1 scope-decision closure**  
Plan authority: **[`PLAN.md`](./PLAN.md)**  
Contract-stack authority: **[`docs/rust_v1_contract_stack.md`](./docs/rust_v1_contract_stack.md)**  
Frozen command-wall authority: **`.runs/i3_5_authority_alignment/authority-plan.snapshot.md` and `.runs/i3_5_authority_alignment/phase2-freeze.json`**  
Primary workspace: **`/home/azureuser/__Active_Code/atomize-hq/spec`**  
Frozen basis commit: **`9bec150c596148f5bd03f048d154c59f137bb0cf`**  
Last rewritten: **2026-05-22**

## Summary

- Execute from `/home/azureuser/__Active_Code/atomize-hq/spec`.
- Treat I7 as a scope-decision closure milestone, not a broad feature milestone.
- Keep the critical path local to the parent for:
  - basis freeze
  - packet acceptance
  - decision freeze
  - authority merge
  - final I8 handoff
- Parallelize only the two independent decision packets before freeze:
  - Lane A: bounded generics
  - Lane B: async/IO
- Launch Lane C only after decision freeze.
- Launch Lane D only if the frozen decision admits one newly bounded slice that
  still needs code proof before I8.

I7 is complete only when the repo can answer, without inference:

- what Rust V1 includes
- what Rust V1 defers to `V1.1`
- what exact wall I8 must prove

## Frozen Outcome

- bounded generics defer to `V1.1`
- Rust V1 remains synchronous-only, so async/IO also defer to `V1.1`
- `BENCH-CROSSLIB` remains the active companion-negative wall
- Lane D is absent because no new bounded Rust V1 slice was admitted
- I8 inherits the existing five-command validation wall unchanged

## Milestone Posture

- I7 closes post-I6 ambiguity; it does not reopen I3.5 command semantics.
- I3.5 remains frozen authority for the public command wall.
- I8 is the final proof-run milestone.
- There is no implied checked-in `I9`.
- `BENCH-ECOM`, `BENCH-SERVICE`, and the current `BENCH-CROSSLIB` role remain
  frozen starting truth unless I7 records an explicit scoped change.
- Narrow, explicit deferral is a successful I7 outcome.

## Parent And Worker Responsibilities

### Parent owns

- the frozen basis commit and basis packet
- all canonical run-state under `.runs/i7/`
- acceptance or rejection of lane packets
- the single decision-freeze record
- the single authority merge
- the final validation wall
- the final I8 handoff record

### Workers own

- narrow branch-local implementation or drafting work for their lane only
- short return summaries: changed files, commands run, blockers, assumptions

### Workers must not own

- `.runs/i7/**` in the primary checkout
- basis freeze decisions
- final scope admission decisions
- cross-lane reconciliation
- final validation or milestone closeout

## Hard Guards

- Do not change the I3.5 command wall.
- Do not redesign benchmark mechanics.
- Do not widen Rust V1 support by prose drift.
- Do not let Lane A or Lane B edit checked-in authority docs directly before
  decision freeze.
- Do not merge ratification docs before the parent records the frozen decisions.
- Do not create Lane D unless the parent has already frozen one exact admitted
  slice and its proof burden.
- Do not close I7 while any V1 admission still lacks exact proof commands.
- Do not imply a checked-in `I9`.

## Worktree And Branch Layout

The parent checkout is the canonical run root.

- Parent basis branch: `main`
- Parent working branch: `codex/i7-v1-scope-closure`
- Parent workspace: `/home/azureuser/__Active_Code/atomize-hq/spec`
- Worktree root: `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7`

Parent branch creation:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec checkout main
git -C /home/azureuser/__Active_Code/atomize-hq/spec pull --ff-only
git -C /home/azureuser/__Active_Code/atomize-hq/spec checkout -b codex/i7-v1-scope-closure
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7
```

Worker worktrees after Gate 0 basis freeze:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-a -b codex/i7-lane-a-generics codex/i7-v1-scope-closure
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-b -b codex/i7-lane-b-async codex/i7-v1-scope-closure
```

Worker worktrees after Gate 2 decision freeze:

```bash
DECISION_FREEZE_COMMIT="$(git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse HEAD)"
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-c -b codex/i7-lane-c-ratify "$DECISION_FREEZE_COMMIT"
```

Conditional worker only if I7 admits a new bounded slice that needs code proof:

```bash
DECISION_FREEZE_COMMIT="$(git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse HEAD)"
git -C /home/azureuser/__Active_Code/atomize-hq/spec worktree add /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-d -b codex/i7-lane-d-proof "$DECISION_FREEZE_COMMIT"
```

Concurrency policy:

- maximum concurrent workers before decision freeze: `2`
- maximum concurrent workers after decision freeze: `1` by default
- Lane C and Lane D may overlap only if Lane C is limited to prose that cannot
  affect the final validation wall or proof-command list

## Canonical Run State

The parent-owned source of truth for this milestone is:

- `I7_RUN_ROOT=/home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7`
- `tasks.json` at `$I7_RUN_ROOT/tasks.json`
- session log at `$I7_RUN_ROOT/session-log.md`
- basis record at `$I7_RUN_ROOT/basis.json`
- decision packet receipt log at `$I7_RUN_ROOT/packet-receipts.json`
- decision freeze at `$I7_RUN_ROOT/decision-freeze.json`
- authority merge record at `$I7_RUN_ROOT/authority-merge.json`
- final I8 handoff at `$I7_RUN_ROOT/i8-handoff.json`

Parent also preserves raw basis evidence under:

- `$I7_RUN_ROOT/basis/examples-ecommerce.status.json`
- `$I7_RUN_ROOT/basis/examples-ecommerce.export.json`
- `$I7_RUN_ROOT/basis/examples-service.status.json`
- `$I7_RUN_ROOT/basis/examples-service.export.json`
- `$I7_RUN_ROOT/basis/workspace.status.json`

Recommended parent-owned lane return mirrors:

- `$I7_RUN_ROOT/lanes/lane-a-bounded-generics.md`
- `$I7_RUN_ROOT/lanes/lane-b-async-io.md`
- `$I7_RUN_ROOT/lanes/lane-c-ratification.md`
- `$I7_RUN_ROOT/lanes/lane-d-proof.md`

Workers may draft branch-local notes, but only the parent writes canonical
files under `.runs/i7/` in the primary checkout.

## Workstream Matrix

| Workstream | Task id | Owner | Start gate | Owned write set | Deliverable |
| --- | --- | --- | --- | --- | --- |
| WS-PARENT-0 | `task/i7-p0-basis-freeze` | parent | run start | `.runs/i7/**` | frozen basis packet plus raw evidence |
| WS-A | `task/i7-a1-bounded-generics-packet` | worker lane A | Gate 0 green | worker-local packet draft only | one bounded-generics recommendation packet |
| WS-B | `task/i7-b1-async-io-packet` | worker lane B | Gate 0 green | worker-local packet draft only | one async/IO recommendation packet |
| WS-PARENT-1 | `task/i7-p1-packet-acceptance-and-decision-freeze` | parent | WS-A and WS-B returned | `.runs/i7/packet-receipts.json`, `.runs/i7/decision-freeze.json` | frozen milestone posture |
| WS-D | `task/i7-d1-conditional-proof` | worker lane D | Gate 2 admits one bounded slice | only frozen proof surfaces named in `decision-freeze.json` | exact bounded proof or honest blocker |
| WS-C | `task/i7-c1-contract-ratification` | worker lane C | Gate 2 green | checked-in authority docs only | repo-facing I7 ratification diff |
| WS-PARENT-2 | `task/i7-p2-integration-and-closeout` | parent | WS-C done and WS-D done if present | parent branch, `.runs/i7/authority-merge.json`, `.runs/i7/i8-handoff.json` | merged authority, validation rerun, I8 handoff freeze |

## Approval And Gate Model

I7 does not need M26-style human approval pauses by default. Its approval model
is parent-gated milestone control.

- Gate 0 freezes starting truth.
- Gate 1 accepts or rejects the two decision packets.
- Gate 2 freezes the single milestone posture.
- Gate 3 admits or rejects any conditional proof lane.
- Gate 4 merges ratified authority.
- Gate 5 freezes the final I8 handoff and closes the run.

If a lane recommends widening Rust V1 beyond the current narrow baseline and
the parent cannot express that widening as one bounded, nameable, provable
slice using existing authority inputs, the parent halts and rejects the packet
back to that lane.

## Workstream Plan

### WS-PARENT-0 (`task/i7-p0-basis-freeze`) — parent only, sequential

Owned write set:

- `.runs/i7/**`

Required commands:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/basis
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse --abbrev-ref HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse HEAD
cargo run -p spec-cli -- status examples/ecommerce/units --format json | tee /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/basis/examples-ecommerce.status.json
cargo run -p spec-cli -- export examples/ecommerce/units | tee /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/basis/examples-ecommerce.export.json
cargo run -p spec-cli -- status examples/service/units --format json | tee /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/basis/examples-service.status.json
cargo run -p spec-cli -- export examples/service/units | tee /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/basis/examples-service.export.json
cargo run -p spec-cli -- status . --format json | tee /home/azureuser/__Active_Code/atomize-hq/spec/.runs/i7/basis/workspace.status.json
```

Acceptance:

- `basis.json` records:
  - `basis_branch`
  - `basis_commit`
  - the frozen I3.5 authority paths
  - the five-command validation wall
  - current `BENCH-ECOM`, `BENCH-SERVICE`, and `BENCH-CROSSLIB` starting truth
  - the current bounded-generics and async/IO deferred posture from milestone
    authority
- raw basis command outputs exist under `.runs/i7/basis/`
- Lane A and Lane B fork only from `basis_commit`

### WS-A (`task/i7-a1-bounded-generics-packet`) — worker lane A

Branch and worktree:

- branch: `codex/i7-lane-a-generics`
- worktree: `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-a`

Owned write set:

- worker-local draft packet only:
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-a/.runs/i7-drafts/task-i7-a1-bounded-generics.md`
- no checked-in authority docs before Gate 2

Required commands:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-a/.runs/i7-drafts
git -C /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-a rev-parse HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-a merge-base --is-ancestor "$(git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse codex/i7-v1-scope-closure)" HEAD
```

Required packet contents:

- recommendation: `admit` or `defer`
- user-value rationale
- exact authored boundary
- exact proof burden
- rejected broader expansions
- exact repo surfaces that would change if admitted

Acceptance:

- the packet recommends exactly one outcome
- the boundary fits in one paragraph
- the proof burden is believable on top of the frozen command wall
- the packet does not quietly widen into broad trait, lifetime, macro, or
  abstraction-heavy support
- the packet cites the frozen basis commit and the relevant authority inputs
- the packet returns changed files, commands run, blockers, and assumptions in a
  parent-readable summary

### WS-B (`task/i7-b1-async-io-packet`) — worker lane B

Branch and worktree:

- branch: `codex/i7-lane-b-async`
- worktree: `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-b`

Owned write set:

- worker-local draft packet only:
  - `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-b/.runs/i7-drafts/task-i7-b1-async-io-packet.md`
- no checked-in authority docs before Gate 2

Required commands:

```bash
mkdir -p /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-b/.runs/i7-drafts
git -C /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-b rev-parse HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-b merge-base --is-ancestor "$(git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse codex/i7-v1-scope-closure)" HEAD
```

Required packet contents:

- recommendation: `admit` or `defer`
- user-value rationale
- exact authored boundary
- exact proof burden
- rejected broader expansions
- exact repo surfaces that would change if admitted

Acceptance:

- the packet recommends exactly one outcome
- the boundary does not collapse Rust V1 into normal backend Rust
- the proof burden does not require a mechanics rewrite
- the packet does not quietly import framework lifecycle semantics as baseline
- the packet cites the frozen basis commit and the relevant authority inputs
- the packet returns changed files, commands run, blockers, and assumptions in a
  parent-readable summary

### WS-PARENT-1 (`task/i7-p1-packet-acceptance-and-decision-freeze`) — parent only, sequential

Owned write set:

- `.runs/i7/packet-receipts.json`
- `.runs/i7/decision-freeze.json`

Required actions:

- read Lane A and Lane B returns against frozen basis
- compare both packets against `PLAN.md`, `docs/rust_v1_contract_stack.md`, and
  the frozen I3.5 authority snapshot
- reject any packet that still says "maybe"
- reject any packet whose admitted slice cannot be named, bounded, and proven
- reject any pair of packets that disagree on shared milestone truth
- record accepted packet summaries in `packet-receipts.json`
- freeze the single final posture in `decision-freeze.json`

`decision-freeze.json` must record:

- `basis_commit`
- bounded-generics outcome
- async/IO outcome
- `BENCH-CROSSLIB` disposition
- whether Lane D exists
- if Lane D exists:
  - admitted slice name
  - owned code surfaces
  - exact required proof commands

Acceptance:

- bounded generics has one frozen outcome
- async/IO has one frozen outcome
- there is no remaining ambient scope question for I7
- Lane D is either explicitly absent or explicitly bounded

### WS-D (`task/i7-d1-conditional-proof`) — conditional worker lane D

Start condition:

- this lane exists only if `decision-freeze.json` admits one new bounded slice
  and names exact proof commands
- if `decision-freeze.json` does not name exact proof commands, Lane D does not
  start and Gate 2 is still open

Branch and worktree:

- branch: `codex/i7-lane-d-proof`
- worktree: `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-d`

Owned write set:

- only the code, tests, examples, and benchmark surfaces named in
  `decision-freeze.json`

Forbidden touches:

- `PLAN.md`
- `ORCH_PLAN.md`
- `docs/rust_v1_contract_stack.md`
- `.runs/i7/**`
- any surface outside the admitted bounded slice

Acceptance:

- the admitted slice proves only the frozen boundary
- the commands named in `decision-freeze.json` actually run and are sufficient
- no additional broad support is smuggled in as proof scaffolding
- if the proof fails honestly, Lane D returns a blocker instead of widening the
  slice or inventing new milestone scope

### WS-C (`task/i7-c1-contract-ratification`) — worker lane C after Gate 2

Branch and worktree:

- branch: `codex/i7-lane-c-ratify`
- worktree: `/home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-c`

Owned write set:

- `PLAN.md`
- `ORCH_PLAN.md`
- `docs/rust_v1_contract_stack.md`
- any additional repo-facing closeout docs explicitly required by the frozen
  I7 decisions

Dependencies:

- must start from the exact `decision_freeze_commit`
- if Lane D exists, Lane C may draft prose early but the parent must not merge
  Lane C until Lane D proves the final command list

Required commands:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-c rev-parse HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/.worktrees/spec-i7/lane-c merge-base --is-ancestor "$(git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse codex/i7-v1-scope-closure)" HEAD
```

Acceptance:

- all checked-in authority surfaces teach the same frozen I7 story
- `I7 -> I8` is explicit everywhere
- deferred `V1.1` surfaces are explicit everywhere
- no doc teaches a checked-in `I9`
- no doc teaches post-I6 ambiguity as current truth
- the final checked-in validation section matches the frozen proof wall exactly

### WS-PARENT-2 (`task/i7-p2-integration-and-closeout`) — parent only, sequential

Owned write set:

- integration branch on `codex/i7-v1-scope-closure`
- `.runs/i7/authority-merge.json`
- `.runs/i7/i8-handoff.json`

Required actions:

- verify no authority input drifted since Gate 0 without being consciously
  re-frozen
- merge Lane D first if it exists
- merge Lane C after proof-command truth is final
- resolve only straightforward merge mechanics
- bounce disagreements back to the owning lane instead of resolving creatively
- rerun the validation wall
- append any exact conditional proof commands to the validation wall before
  closeout
- freeze the final I8 handoff

Required commands:

```bash
git -C /home/azureuser/__Active_Code/atomize-hq/spec rev-parse HEAD
git -C /home/azureuser/__Active_Code/atomize-hq/spec diff --name-only
git -C /home/azureuser/__Active_Code/atomize-hq/spec merge --no-ff codex/i7-lane-d-proof
git -C /home/azureuser/__Active_Code/atomize-hq/spec merge --no-ff codex/i7-lane-c-ratify
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- status . --format json
```

If Lane D does not exist, skip the `codex/i7-lane-d-proof` merge and run the
existing five-command wall unchanged.

Acceptance:

- merged repo truth matches `decision-freeze.json`
- the validation wall is enumerable and current
- the final I8 handoff is a proof run, not another discovery milestone
- `authority-merge.json` records exactly which authority surfaces changed

## Execution Order

1. Parent runs Gate 0 basis freeze locally.
2. Lane A and Lane B run in parallel from the frozen basis commit.
3. Parent accepts or rejects both packets, then records Gate 2 decision freeze.
4. If both topics defer cleanly, launch Lane C only.
5. If one bounded slice is admitted and still needs proof, launch Lane D first
   or overlap Lane C only on prose that cannot affect proof commands.
6. Parent merges proof work, then ratification work, then reruns the final wall.
7. Parent records the I8 handoff and closes the run.

## Halt, Bounce, And Reopen Rules

Parent must halt the run if:

- the live command wall no longer matches the I3.5 authority snapshot
- `basis_commit` changes after Gate 0 but before Lane A and Lane B fork
- any existing validation-wall command fails unexpectedly during basis capture
- `PLAN.md`, `docs/rust_v1_contract_stack.md`, or the I3.5 authority files
  drift under the parent during the run and that drift was not consciously
  re-frozen

Parent must bounce a lane back to its owner if:

- the packet recommends more than one outcome
- the packet relies on "common Rust expectations" instead of named repo truth
- the packet names no exact proof burden
- Lane A and Lane B disagree on the shared milestone boundary and the conflict
  is not settled in `decision-freeze.json`
- Lane C rewrites authority before decision freeze
- Lane C teaches a proof wall that Lane D has not yet proven
- Lane D touches surfaces outside the admitted slice
- any worker edits `.runs/i7/**` in the parent checkout

Parent must reopen Gate 0 if:

- basis evidence is missing, stale, or captured from mixed commits
- a worker did not branch from `basis_commit`

Parent must reopen Gate 2 if:

- ratified docs conflict with frozen decisions
- Lane D proves a different slice than the one frozen
- new proof work changes the exact command list required for closeout

Parent must reopen Gate 4 if:

- the merged tree no longer teaches one coherent `I7 -> I8` story
- the final validation wall omits newly required proof commands
- authority merge reveals a lane conflict that cannot be resolved literally from
  `decision-freeze.json`

## Context-Control Rules

- Parent keeps live context limited to:
  - `PLAN.md`
  - `docs/rust_v1_contract_stack.md`
  - `.runs/i7/tasks.json`
  - `.runs/i7/decision-freeze.json` once it exists
  - the active integration diff summary
- Worker prompts must include exactly:
  - task id
  - lane branch and worktree
  - fork commit
  - owned write set
  - forbidden touch surfaces
  - exact relevant `PLAN.md` excerpt
  - any frozen proof commands already known
  - required return format
- Each worker receives only:
  - its owned write set
  - the exact relevant `PLAN.md` excerpt
  - frozen authority rules
  - required commands
  - forbidden touch surfaces
  - the commit it must fork from
- Each worker returns only:
  - changed files
  - commands run and exit codes
  - blockers
  - assumptions
- Parent summarizes worker returns into `.runs/i7/lanes/*.md` instead of
  pulling full transcripts into working context.
- Workers do not write canonical `.runs/i7/**` state in the primary checkout.
- Close each worker after merge or rejection.
- If a worker needs broader context than its prompt packet, the parent pauses
  and explicitly decides whether to widen that context instead of letting the
  lane self-expand.

## Tests, Validation, And Acceptance

### Basis Integrity

- `basis.json` must cite:
  - `basis_branch`
  - `basis_commit`
  - the exact five-command wall
  - the current deferred bounded-generics and async/IO posture
  - the starting `BENCH-CROSSLIB` role
- raw basis evidence must exist for all five frozen commands
- all worker branches must descend from `basis_commit`

### Packet Integrity

- Lane A and Lane B must each return exactly one recommendation
- each packet must name:
  - boundary
  - rationale
  - proof burden
  - rejected broader expansions
  - repo surfaces impacted if admitted
- neither packet may rely on ambient future work or an implied `I9`

### Decision Integrity

- `decision-freeze.json` must be the only frozen milestone posture
- it must record:
  - bounded-generics outcome
  - async/IO outcome
  - `BENCH-CROSSLIB` disposition
  - whether Lane D exists
  - any exact added proof commands
- if Lane D exists, its allowed write set and trigger condition must be frozen
  before that lane starts

### Ratification Integrity

- `PLAN.md`, `ORCH_PLAN.md`, and `docs/rust_v1_contract_stack.md` must agree on:
  - what is in Rust V1
  - what defers to `V1.1`
  - the `I7 -> I8` ladder
  - the absence of a checked-in `I9`
- repo-facing closeout docs updated in I7 must teach the same frozen story

### Conditional Proof Integrity

- if Lane D does not exist, I7 remains a decision-and-ratification milestone
- if Lane D exists:
  - it must prove only the admitted bounded slice
  - its commands must be appended before closeout
  - failure must return an honest blocker or force decision reopen; it must not
    silently widen scope

### Integration And Closeout Integrity

- parent merges only after packet acceptance and decision freeze
- parent resolves only straightforward merge mechanics
- lane disagreements on truth are bounced back, not papered over in integration
- `authority-merge.json` and `i8-handoff.json` must reflect the merged truth
- the final run must leave I8 enumerable as a proof wall, not a discovery loop

The minimum frozen validation wall for I7 closeout is:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- status examples/service/units --format json
cargo run -p spec-cli -- export examples/service/units
cargo run -p spec-cli -- status . --format json
```

Closeout rules:

- these five commands must remain present at closeout
- if I7 admits a new bounded slice, the parent must append that slice's exact
  proof commands before declaring I7 done
- the appended commands must be recorded in:
  - `.runs/i7/decision-freeze.json`
  - `.runs/i7/i8-handoff.json`
  - the final checked-in `ORCH_PLAN.md` validation wall if it changes

Final acceptance for I7:

- the repo has one explicit answer on bounded generics
- the repo has one explicit answer on async/IO
- the final Rust V1 line is truthful and bounded
- `BENCH-CROSSLIB` is either preserved or explicitly re-justified
- the command wall still matches the ratified claim
- `I8` is explicitly the final proof-run milestone
- no checked-in doc implies a checked-in `I9`
- `.runs/i7/` contains enough parent-owned evidence to replay how the decision
  was frozen

## Assumptions

- `main` is the correct frozen basis branch when the run starts.
- The current I3.5 authority snapshot remains the command-wall source of truth.
- I7 is allowed to finish with explicit `V1.1` deferrals and no new code work.
- Lane D is exceptional and should not exist unless the parent can name one
  newly admitted bounded slice and its exact proof burden.
- Repo-facing ratification after decision freeze may include docs beyond
  `PLAN.md`, `ORCH_PLAN.md`, and `docs/rust_v1_contract_stack.md` only when the
  frozen decisions make that update necessary.
- Worker-local packet drafts are disposable execution aids, not checked-in
  deliverables.

## Immediate Next Move

If starting the run fresh:

1. create `codex/i7-v1-scope-closure`
2. capture Gate 0 basis evidence under `.runs/i7/basis/`
3. fork Lane A and Lane B from `basis_commit`
4. freeze the decisions before any authority doc merge
5. create Lane D only if the frozen decision admits one exact bounded slice
6. ratify the repo-facing I7 story
7. rerun the validation wall and freeze the I8 handoff
