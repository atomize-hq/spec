# M40 Orchestration Plan

Status: **authoritative execution contract for the M40 session**  
Authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`**  
Owned authored artifact: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**  
Live checkout: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec`**  
Live branch: **`feat/corpus-expansion`**  
Live HEAD at draft time: **`de096096e6093eaea771af1f6b95f208ca3e7e44`**  
Review base: **`main`**  
Last rewritten: **`2026-05-07`**  
Run root for an executed M40 session: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_authority_artifact_freeze`**  
Artifact root: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts`**  
Execution note: **M40 is an authority-artifact milestone only. It does not authorize extraction, code motion, seam implementation, corpus-run spend, schema widening, or stealth cleanup outside the authored orchestration file.**

## Summary

- This session exists to operationalize **M40 - Family-Analysis Shared-Core Follow-On Authority Plan** without widening scope beyond authored authority.
- `PLAN.md` is the milestone source of truth. `ORCH_PLAN.md` is the execution contract for carrying that source of truth through one honest session to completion.
- M40 is sequential and parent-owned on purpose. There is no honest multi-lane edit split for a one-file authority artifact.
- The parent agent is the sole:
  - baseline capturer
  - authority interpreter
  - `ORCH_PLAN.md` author
  - scope gatekeeper
  - acceptance gatekeeper
  - final verifier
  - closeout author
- Optional delegation is capped at read-only fact collection before final artifact acceptance. No delegate edits repo files, run-state, or authored authority in M40.
- The critical path is fixed:
  1. capture baseline and acknowledge live branch state
  2. read `PLAN.md` and freeze the exact M40 truth that must be preserved
  3. author `ORCH_PLAN.md`
  4. self-audit the draft against M40 boundaries
  5. verify diff scope and acceptance
  6. stop
- The semantic floor remains exactly what `PLAN.md` already froze:
  - the repo is not yet authorized to extract a shared family-analysis core
  - corpus run `1` remains unspent
  - the helper-surface wedge remains a durable non-promotable hold
  - `xtask/src/family/verify.rs` is a real consumer of the bounded decision contract
  - no command output currently authorizes extraction

## Hard Guards

- `PLAN.md` wins over `ORCH_PLAN.md`, notes, memory, or stale run artifacts if they disagree.
- `ORCH_PLAN.md` is the only repo file owned by this session.
- This session does not edit:
  - `PLAN.md`
  - any code under `xtask/src/**`
  - any file under `.semantic-family-artifacts/**`
  - any prior `.runs/**` artifact
- M40 must not introduce:
  - local seam extraction
  - cross-crate extraction
  - a new shared crate
  - command-wiring moves into the seam
  - helper-surface reclassification
  - a synthetic second consumer
  - a synthetic second durable wedge
  - public fingerprint-field expansion
  - corpus run `1` activation
  - closeout wording that implies implementation happened
- The candidate seam remains exactly:
  - helper-surface durable-hold classifier
  - bounded corpus-program decision derivation
  - normalized decision proof-fingerprint helpers
- The following must remain local even after any future seam move:
  - `xtask` CLI wiring
  - artifact latest-path lookup
  - command-specific JSON rendering
  - proof-wall file locations
  - milestone-specific closeout wording
- If any newly gathered live evidence contradicts `PLAN.md` on trigger truth, extraction status, or allowed next milestones, the session stops. The fix is a new authority decision, not silent ORCH drift.
- Existing local edits by other actors must be preserved. The parent may adapt around them or stop, but does not overwrite or revert them.

## Closed Implementation Surface

Only the following surfaces are in contract for an executed M40 session:

| Path | Role in M40 | Ownership | Allowed write mode |
|---|---|---|---|
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md` | milestone authority | read-only authority | no writes |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md` | parent-authored orchestration contract | parent only | authored edit |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_authority_artifact_freeze/**` | execution run-state, proof captures, acceptance, closeout | parent only | run-state writes only |
| `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.semantic-family-artifacts/**` | derived proof inputs | read-only derived input | no writes |

Rules:

- The table above is the entire closed implementation surface for M40.
- Any edit outside this closed surface is out of contract.
- Writes under `.runs/m40_authority_artifact_freeze/**` are execution artifacts only. They are never treated as authored source, implementation work, or extraction evidence by themselves.
- `.semantic-family-artifacts/**` may be read to confirm live truth, but no file under that tree may be hand-edited.

## Authored Artifacts vs Run Artifacts

### Authored authority

Authored authority for M40 is limited to:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`

Rules:

- `PLAN.md` already exists and is authoritative.
- `ORCH_PLAN.md` is parent-authored only.
- `PLAN.md` is read-only for this session.
- `ORCH_PLAN.md` is complete only when it faithfully operationalizes `PLAN.md` without adding implementation scope.

### Run artifacts

Run artifacts for an executed M40 session are derived, replaceable, and non-authoritative:

- `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_authority_artifact_freeze/**`
- command stdout/stderr captures from the M40 proof floor
- diff evidence
- acceptance notes
- closeout notes

Rules:

- Run artifacts document execution. They never override authored authority.
- Run artifacts may confirm that the proof floor stayed green, but they do not create extraction authorization.
- If the session is executed later, preserve the final proof-floor outputs and acceptance notes under the M40 run root. Do not treat those files as authored source.

## Branch, Worktree, And Concurrency Layout

Repository root:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec
```

M40 layout:

| Role | Branch | Worktree | Concurrency |
|---|---|---|---|
| Live authority authoring and acceptance | `feat/corpus-expansion` | `/Users/spensermcconnell/__Active_Code/atomize-hq/spec` | `1` |

Rules:

1. M40 does not require a worktree split.
2. M40 does not launch editing lanes.
3. M40 does not create a merge surface separate from the live checkout because the only owned repo artifact is `ORCH_PLAN.md`.
4. Optional read-only help, if used at all, may inspect facts in parallel but may not write files, create branches, or claim acceptance.
5. If the live branch head moves during authoring, the parent rechecks whether `PLAN.md` still matches the intended M40 truth before finishing.

## Canonical Run-State

Canonical run root for an executed M40 session:

```text
/Users/spensermcconnell/__Active_Code/atomize-hq/spec/.runs/m40_authority_artifact_freeze
```

Run-state rules:

- Everything in this section is execution metadata, not authored source.
- The parent owns every canonical file in this run root.
- Optional support lane `S1` writes none of these files.
- If a gate fails, the parent records the failure in canonical run-state before stopping.

Canonical parent-owned files:

- `baseline.json`
- `authority-freeze.json`
- `authority-snapshot/PLAN.md`
- `authority-snapshot/ORCH_PLAN.md`
- `run-state.json`
- `tasks.json`
- `queue.json`
- `session-log.md`
- `proof-floor/verify-decision-contract.stdout.json`
- `proof-floor/corpus-decision.stdout.json`
- `proof-floor/cargo-test-p-xtask.stdout.txt`
- `proof-floor/summary.json`
- `acceptance.md`
- `blocked.json`
- `closeout.md`

Required contents:

- `baseline.json`
  - live branch name
  - live HEAD SHA
  - dirty-state summary
  - whether `PLAN.md` is already modified
  - whether `ORCH_PLAN.md` was empty at start
  - whether any `.runs/m40*` path already exists
  - current proof-input paths under `.semantic-family-artifacts`
- `authority-freeze.json`
  - exact authority paths
  - closed implementation surface snapshot
  - frozen M40 truth summary copied from `PLAN.md`
  - explicit statement that no extraction is authorized
  - explicit statement that only one successful closeout verdict is allowed
- `authority-snapshot/PLAN.md`
  - exact `PLAN.md` snapshot used for the run
- `authority-snapshot/ORCH_PLAN.md`
  - exact accepted `ORCH_PLAN.md` snapshot for the run
- `run-state.json`
  - active gate id
  - gate status
  - current phase
  - whether the run is `active`, `blocked`, or `complete`
  - latest accepted verdict if any
- `tasks.json`
  - canonical ordered tasks and gates with status fields
  - owner for each item
  - required outputs for each item
- `queue.json`
  - execution order
  - current item pointer
  - completed item list
  - blocked item if the run stops
- `session-log.md`
  - ordered operator notes with timestamps
  - baseline capture note
  - authority-freeze note
  - any stop or restart rationale
- `proof-floor/verify-decision-contract.stdout.json`
  - raw stdout capture from `cargo xtask family verify-decision-contract --format json`
- `proof-floor/corpus-decision.stdout.json`
  - raw stdout capture from `cargo xtask family corpus-decision --format json`
- `proof-floor/cargo-test-p-xtask.stdout.txt`
  - raw stdout capture from `cargo test -p xtask`
- `proof-floor/summary.json`
  - pass/fail summary for the three commands
  - extracted high-signal fields needed for closeout
- `acceptance.md`
  - parent acceptance against M40 content and scope requirements
  - exact statement that `ORCH_PLAN.md` is accepted or blocked
- `blocked.json`
  - failing gate id
  - failing condition
  - blocking evidence paths
  - whether restart is required
- `closeout.md`
  - final outcome
  - exact allowed verdict line
  - proof-floor summary reference
  - explicit statement that no implementation or extraction was authorized

## Queue And Task Model

The executed session must use one canonical ordered queue. No item may be skipped silently.

Canonical ordered entries:

| Order | ID | Kind | Owner | Required outputs on success |
|---|---|---|---|---|
| 1 | `gate-m40-00-baseline-capture` | gate | parent | `baseline.json`, `session-log.md`, `run-state.json` |
| 2 | `gate-m40-10-authority-freeze` | gate | parent | `authority-freeze.json`, `authority-snapshot/PLAN.md`, `authority-snapshot/ORCH_PLAN.md`, `run-state.json` |
| 3 | `task-m40-20-author-orch-plan` | task | parent | updated `ORCH_PLAN.md`, `tasks.json`, `session-log.md` |
| 4 | `gate-m40-30-self-audit` | gate | parent | updated `acceptance.md`, `authority-snapshot/ORCH_PLAN.md`, `run-state.json` |
| 5 | `gate-m40-40-proof-floor` | gate | parent, optional read-only support from `S1` | `proof-floor/*`, `proof-floor/summary.json`, `run-state.json` |
| 6 | `gate-m40-50-final-closeout` | gate | parent | `closeout.md`, final `acceptance.md`, final `run-state.json`, final `queue.json` |

Queue rules:

- `tasks.json` is the canonical task catalog.
- `queue.json` is the canonical ordered execution pointer.
- Gates are mandatory checkpoints. A later item cannot pass until all earlier items are complete.
- If a gate fails, `queue.json` must mark the run blocked at that item and `blocked.json` must be written before stopping.
- The parent is the only actor allowed to advance `queue.json`.

## Parent vs Delegated Ownership

### Parent-only

The parent alone may:

- interpret `PLAN.md`
- decide the M40 run root naming
- author and edit `ORCH_PLAN.md`
- declare the candidate seam, local-only exclusions, and trigger table operationalized
- decide whether to rerun the proof floor during execution
- inspect final diff scope
- declare acceptance or stop the run

### Delegation allowed only as read-only support

Optional support work may:

- inspect prior `.runs/` naming patterns
- confirm current branch, head, and dirty-state facts
- capture proof-floor outputs for parent review

Restrictions:

- no delegate edits any repo file
- no delegate writes `.runs/m40_authority_artifact_freeze/**`
- no delegate updates orchestration truth
- no delegate reinterprets triggers independently of the parent
- no delegate claims the run is complete

### Optional support lane `S1`

Mission:

- provide read-only support while the parent authors `ORCH_PLAN.md`

Allowed work:

- capture prior `.runs/` naming facts
- capture proof-floor command outputs for parent review
- summarize only the fields the parent asks for

Forbidden work:

- editing any repo file
- writing canonical run-state
- changing queue order
- declaring any gate passed
- declaring acceptance

Return contract:

- findings return to the parent only
- the parent decides whether to write those findings into canonical run-state

### Honest concurrency rule

M40 uses no editing lanes because there is no honest way to parallelize authorship of one authority artifact without manufacturing coordination work. `S1` is the only allowed parallel support lane, and it owns no files.

## Workstream Plan

### WS0 - Baseline Capture

Task and gate IDs:

- `gate-m40-00-baseline-capture`

Purpose:

- capture the live branch, head, and dirty-state context
- confirm `PLAN.md` is present and authoritative
- confirm `ORCH_PLAN.md` is the only intended owned output
- record whether any pre-existing local edits overlap authored authority

Expected findings for the current draft session:

- branch is `feat/corpus-expansion`
- `PLAN.md` is already modified and must be treated as current authority, not as M40-owned scope
- `ORCH_PLAN.md` starts empty and is safe to author
- no existing `.runs/m40*` naming collision is present

Gate `M40-G0` passes only if the parent can state the live baseline without ambiguity.

Required artifacts on pass:

- write `baseline.json`
- append baseline note to `session-log.md`
- set `run-state.json` to active gate `gate-m40-10-authority-freeze`
- mark `gate-m40-00-baseline-capture` complete in `tasks.json` and `queue.json`

Stop conditions:

- `PLAN.md` missing
- another actor is concurrently editing `ORCH_PLAN.md`
- baseline facts contradict the intended milestone identity

Blocked-state behavior:

- write `blocked.json` with gate id `gate-m40-00-baseline-capture`
- set `run-state.json` to `blocked`
- stop without editing `ORCH_PLAN.md`

### WS1 - Authority Extraction From `PLAN.md`

Task and gate IDs:

- `gate-m40-10-authority-freeze`

Purpose:

- extract the exact M40 truths that must survive into the orchestration contract
- freeze the non-goals so the session cannot slide into implementation
- freeze the allowed M41 outcomes and trigger table

The parent must carry forward all of the following without drift:

- M40 is an authority artifact, not an implementation milestone
- extraction is not yet authorized
- the seam is limited to the three candidate elements already named in `PLAN.md`
- the five local-only surfaces stay local even after any future seam move
- the only future authorization triggers are:
  - one additional non-`recommend.rs` and non-`promotion_artifacts.rs` in-tree consumer beyond `verify.rs` for local extraction
  - one non-`xtask` crate consumer for cross-crate extraction
  - one second durable non-promotable wedge for a multi-wedge layer
  - one real external consumer for public fingerprint fields
- the only allowed M41 outcomes are:
  - local implementation milestone
  - cross-crate implementation milestone
  - further evidence milestone
  - no new milestone

Gate `M40-G1` passes only if the frozen boundary written into `ORCH_PLAN.md` exactly matches `PLAN.md` on these points.

Stop conditions:

- any trigger is widened
- any local-only surface is pulled into the seam
- any new next-milestone type is introduced

Required artifacts on pass:

- write `authority-freeze.json`
- snapshot `PLAN.md` to `authority-snapshot/PLAN.md`
- snapshot current `ORCH_PLAN.md` state to `authority-snapshot/ORCH_PLAN.md`
- update `run-state.json` to active item `task-m40-20-author-orch-plan`
- mark `gate-m40-10-authority-freeze` complete in `tasks.json` and `queue.json`

Blocked-state behavior:

- write `blocked.json` with gate id `gate-m40-10-authority-freeze`
- append mismatch reason to `session-log.md`
- set `run-state.json` to `blocked`
- stop

### WS1A - Optional Read-Only Support

Task ID:

- `task-m40-s1-read-only-support`

Purpose:

- let `S1` gather proof-floor captures or naming facts in parallel while the parent authors `ORCH_PLAN.md`

Rules:

- `S1` writes no canonical file
- `S1` owns no gate
- `S1` may be omitted entirely with no effect on M40 validity

### WS2 - Author `ORCH_PLAN.md`

Task and gate IDs:

- `task-m40-20-author-orch-plan`
- `gate-m40-30-self-audit`

Purpose:

- write a fresh execution-grade orchestration contract for M40
- make stopping rules, validation, ownership, and future implementation orchestration explicit

Required sections in the authored file:

- title and current-run metadata
- summary
- hard guards
- authored-artifact vs run-artifact distinction
- branch/worktree/concurrency layout
- parent vs delegated ownership
- workstream plan
- context-control rules
- tests and acceptance
- assumptions

Required M40-specific content:

- explicit statement that M40 itself is sequential
- explicit statement that no hidden extraction work is authorized
- explicit statement that `verify.rs` is real consumer pressure but still insufficient alone
- explicit statement that no command output authorizes extraction yet
- explicit future implementation parallelization section aligned to `PLAN.md`:
  - Lane A first
  - Lane B and Lane C in parallel after A
  - Lane D after B

Task `task-m40-20-author-orch-plan` completes when:

- `ORCH_PLAN.md` reflects the current M40 execution contract
- the file includes the closed surface, canonical run-state, queue model, and gate-writing discipline
- `tasks.json` and `queue.json` show the authoring task complete

Gate `M40-G2` passes only if the resulting file can guide the full M40 session without relying on unstated operator judgment.

Required artifacts on pass:

- update `authority-snapshot/ORCH_PLAN.md` to the accepted draft
- append acceptance rationale to `acceptance.md`
- update `run-state.json` to active gate `gate-m40-40-proof-floor`
- mark `gate-m40-30-self-audit` complete in `tasks.json` and `queue.json`

Stop conditions:

- the draft reads like notes instead of an execution contract
- the draft implies code changes are part of M40
- the future parallelization section is omitted or contradicts `PLAN.md`

Blocked-state behavior:

- write `blocked.json` with gate id `gate-m40-30-self-audit`
- append failure rationale to `acceptance.md`
- set `run-state.json` to `blocked`
- stop

### WS3 - Validation And Completion

Task and gate IDs:

- `gate-m40-40-proof-floor`
- `gate-m40-50-final-closeout`

Purpose:

- confirm the finished orchestration artifact still sits on the live proof floor
- confirm authored scope stayed honest
- stop without widening work

Required proof floor for a fully executed M40 session:

```bash
cargo xtask family verify-decision-contract --format json
cargo xtask family corpus-decision --format json
cargo test -p xtask
```

Expected interpretation:

- `verify-decision-contract` must still pass the bounded decision contract
- `corpus-decision` must still report planning-follow-on truth, not extraction authorization
- `cargo test -p xtask` must stay green

Required scope checks:

- inspect the final diff for `ORCH_PLAN.md`
- confirm `PLAN.md` was not edited by this session
- confirm no code or derived artifact path was touched

Gate `M40-G3` passes only if:

- `ORCH_PLAN.md` is complete
- proof-floor commands are green when run
- no unauthorized file changes exist

Required artifacts on proof-floor pass:

- write `proof-floor/verify-decision-contract.stdout.json`
- write `proof-floor/corpus-decision.stdout.json`
- write `proof-floor/cargo-test-p-xtask.stdout.txt`
- write `proof-floor/summary.json`
- update `run-state.json` to active gate `gate-m40-50-final-closeout`
- mark `gate-m40-40-proof-floor` complete in `tasks.json` and `queue.json`

Stop conditions:

- proof-floor output contradicts `PLAN.md`
- any source file outside `ORCH_PLAN.md` changes due to this session
- closeout text implies M40 implemented the seam

Blocked-state behavior:

- write `blocked.json` with gate id `gate-m40-40-proof-floor`
- record failing proof path in `session-log.md`
- set `run-state.json` to `blocked`
- stop

Final closeout gate requirements:

- `closeout.md` must contain exactly one successful verdict line:
  - `authority artifact complete`
- any other outcome is blocked or incomplete
- `closeout.md` must also state:
  - proof floor stayed within planning authority
  - no implementation or extraction was authorized
  - no file outside the closed implementation surface was edited

Required artifacts on final closeout pass:

- write final `closeout.md`
- finalize `acceptance.md`
- set `run-state.json` to `complete`
- mark `gate-m40-50-final-closeout` complete in `tasks.json` and `queue.json`

Blocked-state behavior:

- write `blocked.json` with gate id `gate-m40-50-final-closeout`
- set `run-state.json` to `blocked`
- stop without any alternate success wording

## Future Implementation Parallelization

This section is inert during M40 itself. It exists only so the first later-authorized implementation milestone starts from a frozen, honest split.

Trigger prerequisite:

- one of the `PLAN.md` authorization triggers must become true first

Future lane contract:

| Lane | Purpose | Ownership | Starts when | Blocks |
|---|---|---|---|---|
| `Lane A` | freeze local seam interface | lane-owned implementation surface only | first | everything else |
| `Lane B` | rewire in-tree consumers | lane-owned in-tree consumer surfaces only | after `Lane A` | `Lane D` |
| `Lane C` | docs and closeout sync | docs and closeout artifacts only | after `Lane A` | none beyond parent merge discipline |
| `Lane D` | command-surface adoption | command-facing local surfaces only | after `Lane B` | final verification |

Execution order:

1. launch `Lane A`
2. after `Lane A` lands, launch `Lane B` and `Lane C` in parallel
3. after `Lane B` lands, run `Lane D`

Frozen rule:

- `Lane A` must freeze the local seam interface before any consumer rewiring or command-surface adoption starts.
- The parent remains the sole integrator, stale-lane invalidator, and final verifier.
- If the authority basis changes before that future milestone begins, every pre-launched lane is stale and must be recreated from the new freeze point.
- No future lane may edit the authority artifact that authorizes its milestone after freeze.

Non-negotiable boundaries for the future milestone:

- `xtask` CLI wiring stays local
- artifact latest-path lookup stays local
- command-specific JSON rendering stays local
- proof-wall file locations stay local
- milestone-specific closeout wording stays local

## Context-Control Rules

- Primary source set for M40 authoring is intentionally small:
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/PLAN.md`
  - `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`
  - top-level `.runs/` names for naming realism
  - prior `ORCH_PLAN.md` files only as structure bars, not as content templates
  - current branch/head/status facts
- The parent should not reopen deep code inspection to write this plan. Code paths named in `PLAN.md` are boundary anchors, not M40 edit targets.
- Do not chase optional ideas that are not required for M40 authority:
  - no crate design
  - no module API sketching
  - no implementation TODO expansion
  - no cleanup inventory
- If proof-floor commands are rerun and reveal drift, stop and record the drift explicitly. Do not rewrite the orchestration file to hide it.
- If another actor changes `PLAN.md` before closeout, the parent must reread it and confirm whether the drafted `ORCH_PLAN.md` is still valid.
- Keep all acceptance reasoning tied to explicit files and commands. Narrative confidence is not a substitute for the proof floor.

## Tests And Acceptance

### Mandatory content acceptance

- `ORCH_PLAN.md` exists and is non-empty.
- It operationalizes the current `PLAN.md` rather than restating an older milestone.
- It explicitly distinguishes authored authority from run artifacts.
- It preserves the exact candidate seam and exact local-only exclusions.
- It preserves the exact trigger table and exact M41 outcomes from `PLAN.md`.
- It states that M40 itself is sequential and parent-owned.
- It includes the future A then B+C then D split for the first later-authorized implementation milestone.
- It defines canonical run-state files and an ordered queue model.

### Mandatory scope acceptance

- This session edits only `/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`.
- `PLAN.md` remains untouched by this session even if already dirty.
- No source or derived artifact file outside `ORCH_PLAN.md` changes.

### Mandatory proof-floor acceptance for a fully executed session

- `cargo xtask family verify-decision-contract --format json`
- `cargo xtask family corpus-decision --format json`
- `cargo test -p xtask`

All three are green, and none of their outputs imply extraction authorization.

### Completion conditions

M40 is complete only when all of the following are true:

1. `ORCH_PLAN.md` is execution-grade and specific to M40.
2. The artifact makes it obvious when to stop and what would invalidate the run.
3. The artifact does not authorize implementation work.
4. The proof floor remains the same planning-authority floor described in `PLAN.md`.
5. `closeout.md` ends with the only allowed successful verdict: `authority artifact complete`.
6. The parent can end the session without any open "might as well" follow-on edits.

## Assumptions

- The current contents of `PLAN.md` are intentional and authoritative for M40 even though `PLAN.md` is already locally modified.
- `feat/corpus-expansion` remains the live branch for this work.
- `de096096e6093eaea771af1f6b95f208ca3e7e44` is the live head at draft time only; later execution should recapture baseline rather than rely on this line.
- `ORCH_PLAN.md` began empty for this session.
- No existing `.runs/m40*` artifact root currently constrains the naming chosen here.
- No live command output currently authorizes extraction unless a fresh proof-floor run explicitly proves otherwise.
