<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m40-plus-autoplan-restore-20260508-105337.md -->
# M40+ - `/next-milestone` Forced-Ranking Contract Hardening

Status: **authority plan**
Milestone family: **operator-consumer-tooling**
Implementation readiness: **ready-now**
Next artifact kind: **authority_plan**
Autoplan ready: **yes**
Base branch: **main**
Working branch: **feat/m40-plus**
Last rewritten: **2026-05-08**
Supersedes: **M40 - Family-Analysis Shared-Core Follow-On Authority Plan** as the repo-root branch authority only. Historical M40 closeout evidence remains valid context and is not being revoked.
Execution note: **This rewritten `PLAN.md` is now the active branch authority artifact for `feat/m40-plus`. `ORCH_PLAN.md` remains replay context only for this milestone unless a later milestone explicitly refreshes it.**
Post-fix replay winner target: **shared-core-portability**

## Executive Verdict

`/next-milestone` already has most of the right machinery. The failure is contract-level, not infrastructure-level.

The skill already recovers the right sources, already has a six-family taxonomy, already has a bounded live-signal collector, and already says the winner should be a concrete product-surface milestone. It still fails because one contradictory rule in `SKILL.md` re-allows planning-as-output, and the output contract is not strict enough to force one executable wedge plus loser explanations.

This milestone fixes that by rewriting the selector contract in place. No new command path. No new artifact type. No new scoring engine. Just a tighter authored contract that makes the existing selector do its actual job.

This plan belongs to `operator-consumer-tooling` because the thing being changed is a maintainer-facing truth-selection surface. The replay target after the fix is still `shared-core-portability`, because that is the winning product lane the selector should name for the captured branch state. Those are different layers. That split is intentional.

## Live Validated Basis

Revalidated on the live `feat/m40-plus` tree on 2026-05-08.

Commands run:

```bash
.agents/skills/next-milestone/scripts/collect_signals.sh
rg -n "planning milestone|best honest answer|Executable wedge|Confidence" \
  .agents/skills/next-milestone/SKILL.md \
  .agents/skills/next-milestone/references/rubric.md
```

Observed truth:

- Latest checkpoint says semantic review is still the product core and family-analysis governance is servant work.
- Live family-analysis signals still say:
  - `recommendation_status = "no_strong_candidate"`
  - `decision_action = "pivot_to_architecture_shared_core_follow_on"`
  - `required_next_action = "author_architecture_follow_on_plan"`
  - `overall_verdict = "pass"`
- `SKILL.md` already says, "choose the concrete product-surface milestone, not the prerequisite planning task."
- `rubric.md` already says, "recommend the product milestone, not the prerequisite planning step."
- `SKILL.md:283` still contradicts both by saying, "If the best honest answer is 'planning milestone next, implementation later,' say exactly that."
- `agents/openai.yaml` is already aligned with the desired winner-vs-handoff split and does not need a new prompt surface unless implementation uncovers a real divergence.
- Before this rewrite, repo-root branch authority context was stale for `feat/m40-plus`. The restore snapshot named in the HTML comment preserves that pre-pass state for replay purposes. This file now replaces that stale `PLAN.md` as the active branch authority artifact.
- Current `ORCH_PLAN.md` is still historical replay context, not an implementation target for this milestone.

## Replay Baseline

Acceptance for M40+ is anchored to the branch truth captured on 2026-05-08, not to whatever this plan says after the rewrite.

Use these as the replay baseline:

- live signal snapshot from `.agents/skills/next-milestone/scripts/collect_signals.sh`
- latest checkpoint at validation time
- `.runs/m39_verification_consumer_probe/closeout.md`
- the restore snapshot named in the HTML comment at the top of this file
- current `ORCH_PLAN.md` as historical authority context

Rules:

- Do not use this rewritten `PLAN.md` itself as proof that the selector fix worked.
- Do not let the existence of this file turn a blocked product winner into a planning winner.
- The selector must still name the same product lane when replaying the captured pre-pass truth.

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Plan decision |
|---|---|---|
| context recovery and source hierarchy | `.agents/skills/next-milestone/SKILL.md` | keep and tighten, do not replace |
| scoring model and hard-gate vocabulary | `.agents/skills/next-milestone/references/rubric.md` | keep and tighten, do not replace |
| live evidence collection | `.agents/skills/next-milestone/scripts/collect_signals.sh` | keep unchanged unless a concrete signal gap is proven during implementation |
| operator-facing prompt summary | `.agents/skills/next-milestone/agents/openai.yaml` | leave unchanged by default because it already encodes the correct split |
| replay inputs for the captured branch truth | latest checkpoint, `.runs/m39_verification_consumer_probe/closeout.md`, restore snapshot of the pre-pass `PLAN.md`, current `ORCH_PLAN.md` | use as regression inputs, not as implementation targets |
| desired replacement behavior | `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m40-plus-design-20260508-102457.md` | treat as source-of-truth design intent for this branch |

### Existing contradiction to remove

The selector has one job-killing contradiction:

```text
good rule already present:
  choose the concrete product-surface milestone,
  not the prerequisite planning task

bad rule still present:
  if the best honest answer is "planning milestone next,
  implementation later," say exactly that
```

That second rule turns the selector back into a permit office. It must be deleted or rewritten so the handoff artifact can never become the milestone recommendation itself.

### Minimum complete change

M40+ is complete only if all of the following land together:

1. `SKILL.md` explicitly separates milestone selection from artifact handoff.
2. `SKILL.md` hard-bans planning, `no milestone`, and `more evidence` as final answers.
3. `SKILL.md` output shape adds `Executable wedge` and `Confidence`.
4. `SKILL.md` requires ranked alternates with loser reasons.
5. `rubric.md` hard gates match the rewritten `SKILL.md` contract exactly.
6. The captured `feat/m40-plus` branch truth replays to one product-lane winner instead of another planning answer.

### Complexity and scope result

This is a deliberately small plan.

- Files intentionally changed:
  - `.agents/skills/next-milestone/SKILL.md`
  - `.agents/skills/next-milestone/references/rubric.md`
- Files read but not changed by default:
  - `.agents/skills/next-milestone/scripts/collect_signals.sh`
  - `.agents/skills/next-milestone/agents/openai.yaml`
  - `CLAUDE.md`
  - `ORCH_PLAN.md`
  - the restore snapshot named in the HTML comment
- New classes or services: `0`
- New command paths: `0`
- New artifact types: `0`
- Distribution work: none beyond landing the repo-local skill contract

If implementation starts touching `collect_signals.sh`, `agents/openai.yaml`, routing files, or any `xtask` code, that is a smell and must be justified explicitly before widening scope.

## NOT in scope

- changing `xtask` family-analysis commands or JSON schemas
- changing checkpoint format, closeout format, or `.runs/` layout
- changing `CLAUDE.md` routing rules
- replacing the six milestone-family taxonomy
- building a new prompt-eval harness or test framework
- shared-core extraction, TypeScript backend work, or corpus-policy changes
- rewriting `ORCH_PLAN.md`
- revoking or rewriting historical M39 or M40 closeout evidence
- adding a second prompt surface when the existing YAML already says the right thing

## Architecture Surface

### Dependency graph

```text
checkpoint + frozen docs + authority context + live signals
                      │
                      ▼
        next-milestone/SKILL.md
          workflow + source hierarchy
          + authority-file classification
          + output contract
                      │
                      ▼
        references/rubric.md
          scoring + hard gates
          + tie-breakers
                      │
                      ▼
        decision memo
          one winner
          ranked losers
          readiness split
          exact handoff

outside the change boundary
  collect_signals.sh
  agents/openai.yaml
  CLAUDE.md routing
  xtask family commands
```

### Why this boundary is right

- The bug is in the authored selector contract, not in evidence collection.
- `collect_signals.sh` already surfaces the exact live fields that prove the current failure mode.
- `rubric.md` already carries the right mental model, but its gates need to become stricter and more explicit.
- `agents/openai.yaml` already says the right thing, which means a second prompt surface is not the current blocker.
- `xtask` outputs are already the upstream truth source. Changing them here would spend scope on the wrong layer.

This is the whole change. No wider.

### Closed implementation surface

| Path | Role in M40+ | Allowed action |
|---|---|---|
| `.agents/skills/next-milestone/SKILL.md` | primary selector contract | edit |
| `.agents/skills/next-milestone/references/rubric.md` | scoring mirror and hard gates | edit after `SKILL.md` contract is frozen |
| `.agents/skills/next-milestone/scripts/collect_signals.sh` | evidence collector | read-only unless a concrete signal gap is proven |
| `.agents/skills/next-milestone/agents/openai.yaml` | secondary prompt summary | read-only unless live divergence is proven |
| `ORCH_PLAN.md` | historical authority context | read-only replay input |
| restore snapshot in the HTML comment | frozen pre-pass branch authority | read-only replay input |

Rules:

- The table above is the full honest implementation surface for M40+.
- Any edit outside the two contract files is out of contract unless implementation first proves the selector cannot be fixed in authored prompt/rubric space alone.
- `SKILL.md` freezes first. `rubric.md` mirrors second. Replay happens last.

## Chosen Contract To Land

### Exact selector split

The landed contract must force the skill to answer two questions in order:

1. What is the next executable wedge?
2. What artifact, if any, must exist next to support that wedge honestly?

Question 1 chooses the milestone.

Question 2 chooses the handoff.

Question 2 is never allowed to replace Question 1.

### Exact final output shape

The landed `SKILL.md` output block must include this shape:

```markdown
NEXT MILESTONE

Milestone family: <one of the six fixed family names>
Executable wedge: <one concrete product-surface wedge, never planning>
Confidence: <high | medium | low>
Implementation readiness: <ready-now | needs_artifact_first>
Next artifact kind: <design_doc | authority_plan_draft | authority_plan>
Autoplan ready: <yes | no>
Authority file states:
- PLAN.md: <completed_authority_context | active_execution_contract | draft_next_artifact | stale_historical_artifact | none>
- ORCH_PLAN.md: <completed_authority_context | active_execution_contract | draft_next_artifact | stale_historical_artifact | none>
Recommendation: <one-line winner>

Why this wins:
- ...

Ranked alternates:
1. <family / wedge> - why it loses now
2. <family / wedge> - why it loses now

Evidence used:
- Checkpoint: ...
- Frozen decision docs: ...
- Authority context: ...
- Live signals: ...

Handoff:
1. ...
2. ...
3. ...
```

### Hard bans

The landed selector must reject these as final answers:

- `planning`
- `planning milestone next`
- `author a plan`
- `no milestone`
- `more evidence`

If the honest status is blocked, the selector still chooses the product-lane winner and pushes the block into:

- `Implementation readiness`
- `Next artifact kind`
- `Autoplan ready`
- `Handoff`

### Current-state replay expectation

When replaying the captured 2026-05-08 branch truth, the rewritten contract must produce:

- a winner in `shared-core-portability`
- a concrete wedge description, not a planning label
- `Implementation readiness: needs_artifact_first`
- ranked losers with explicit reasons
- evidence attribution that cites both:
  - `pivot_to_architecture_shared_core_follow_on`
  - `author_architecture_follow_on_plan`

It must not produce another "planning milestone next" style answer.

### Exact file edit contract

| File | Required adds | Required removals | Must stay unchanged |
|---|---|---|---|
| `.agents/skills/next-milestone/SKILL.md` | explicit winner-vs-handoff split, `Executable wedge`, `Confidence`, ranked alternates, replay expectation for captured `feat/m40-plus` truth | the planning-as-output escape hatch and any equivalent null-final-output wording | source hierarchy, family taxonomy, existing evidence attribution rules that are already correct |
| `.agents/skills/next-milestone/references/rubric.md` | mirrored hard bans, forced-ranking requirement, "blocked readiness does not demote the winner" rule, "future trigger row cannot win" rule | any residual ambiguity that lets planning become the answer | existing scoring axes and tie-breaker order unless Step 1 proves a real mismatch |
| `.agents/skills/next-milestone/agents/openai.yaml` | nothing by default | nothing by default | keep unchanged unless readback proves the YAML contradicts the landed contract |
| `.agents/skills/next-milestone/scripts/collect_signals.sh` | nothing | nothing | keep unchanged unless replay proves a concrete missing signal makes the selector impossible to answer honestly |

## Implementation Plan

### Step 1 - Rewrite `SKILL.md`

Owner: parent lane

Touch only these sections unless implementation proves a missing dependency:

- workflow section for candidate selection
- handoff-artifact rules
- decision memo output block
- planning-vs-implementation guardrails

Required edits:

1. Remove the contradictory planning-as-output allowance at `SKILL.md:283`.
2. Add `Executable wedge` and `Confidence` to the memo shape.
3. Rename "Why not the others" to a ranked alternates block with loser reasons.
4. State explicitly that `required_next_action = author_*_plan` affects readiness and handoff only.
5. State explicitly that `recommendation_status = no_strong_candidate` does not authorize `no milestone` as the final answer.
6. Add one short replay-expectation subsection for the captured `feat/m40-plus` truth so the intended behavior stays anchored in-repo.
7. Clarify that this rewritten `PLAN.md` is current branch authority now, while the replay baseline comes from the restore snapshot and current closeout context.

Acceptance for Step 1:

- no planning-as-output wording remains
- output contract is exact, stable, and enum-shaped
- replay expectation is present
- readback shows no conflict between winner selection, readiness, and `/autoplan` handoff wording

### Step 2 - Tighten `rubric.md`

Owner: parent lane

Required edits:

1. Mirror the `SKILL.md` hard bans exactly.
2. Add an explicit forced-ranking requirement: one winner, up to two losers, no null final output.
3. Add a hard gate that blocked readiness never demotes the winning family into planning-as-output.
4. Add a hard gate that future trigger rows and not-yet-triggered authorization branches cannot win.
5. Keep the current scoring axes and tie-breakers unless Step 1 exposes a real mismatch.

Acceptance for Step 2:

- `rubric.md` and `SKILL.md` no longer disagree on allowed final outputs
- no new scoring machinery was introduced
- loser explanation requirement is explicit
- future-trigger rows are treated as evidence context, not as current winners

### Step 3 - Replay current repo truth

Owner: parent lane

Use the already-shipped collector and the captured branch context.

Replay target:

- branch truth captured on `2026-05-08`
- latest checkpoint at validation time
- live corpus decision:
  - `decision_action = pivot_to_architecture_shared_core_follow_on`
  - `required_next_action = author_architecture_follow_on_plan`
- current historical authority context:
  - restore snapshot of the pre-pass repo-root `PLAN.md`
  - repo-root `ORCH_PLAN.md`
  - `.runs/m39_verification_consumer_probe/closeout.md`

Acceptance for Step 3:

- winner remains product-lane, not planning-lane
- planning artifact appears only in readiness and handoff
- `ORCH_PLAN.md` is not targeted as the next `/autoplan` review surface
- alternates are present and lose for explicit reasons
- nothing in the output implies that a future trigger row already fired

### Replay interpretation matrix

| Evidence source | Expected interpretation | Forbidden interpretation |
|---|---|---|
| latest checkpoint | semantic review is core; family-analysis governance is servant work | corpus-policy work is automatically the next milestone |
| `recommendation_status = no_strong_candidate` | another Rust family push is not currently the honest winner | no milestone can be emitted |
| `decision_action = pivot_to_architecture_shared_core_follow_on` | `shared-core-portability` is the winning product lane | begin extraction now |
| `required_next_action = author_architecture_follow_on_plan` | readiness is blocked and handoff must name the gating artifact | planning becomes the milestone |
| stale historical authority context | replay input only | active execution target |

## Code Quality Guardrails

This milestone is tiny. That makes code-quality discipline more important, not less.

Rules:

- edit existing sections instead of appending a second layer of duplicated rules
- keep one vocabulary set across both files for winner, readiness, handoff, and alternates
- do not fork the taxonomy into another reference file, appendix, or YAML-only variant
- do not add clever abstractions or pseudo-schema machinery when plain contract language is enough
- if a rule must exist in both files, mirror it once and verify wording parity rather than improvising near-duplicates

DRY risk to avoid:

- `agents/openai.yaml` already carries the right summary. Do not "fix" the wrong surface and leave the primary contract ambiguous.
- `SKILL.md` and `rubric.md` must agree, but they should not each invent different examples, enum names, or winner/readiness terminology.

## Test Review

### Test framework detection

This repo has a real Rust test framework, but this milestone does not touch Rust runtime code. It touches authored skill-contract files under `.agents/skills/`.

No repo-local automated eval harness was discovered for `next-milestone` prompt behavior. Because of that, this plan requires a checked replay matrix as the regression surface, not a fake claim of automated coverage.

### Coverage diagram

```text
CONTRACT PATH COVERAGE
======================
[+] .agents/skills/next-milestone/SKILL.md
    │
    ├── source recovery + classification rules
    │   └── [REQUIRED] Preserve existing source hierarchy
    │
    ├── milestone selection
    │   └── [GAP] Must reject planning/no-milestone outputs explicitly
    │
    ├── handoff classification
    │   └── [GAP] Must keep blocked readiness out of winner selection
    │
    └── output memo
        └── [GAP] Must emit executable wedge + confidence + ranked losers

[+] .agents/skills/next-milestone/references/rubric.md
    │
    ├── hard gates
    │   └── [GAP] Must mirror `SKILL.md` contract exactly
    │
    └── tie-breakers
        └── [REQUIRED] Keep existing scoring model, no net-new engine

REPLAY COVERAGE
===============
[+] Captured feat/m40-plus truth
    ├── [REQUIRED] winner is a product family, not planning
    ├── [REQUIRED] readiness is blocked honestly
    ├── [REQUIRED] loser reasons are explicit
    └── [REQUIRED] historical authority context is not treated as the active review target
```

### Required regression checks

Implementation is not done until all of these are true:

1. `rg -n "planning milestone next|author a plan|no milestone|more evidence" .agents/skills/next-milestone/SKILL.md .agents/skills/next-milestone/references/rubric.md`
   - only legitimate ban-language mentions remain
2. `SKILL.md` contains `Executable wedge:` and `Confidence:`
3. `rubric.md` contains a hard gate requiring one winner and loser explanations
4. A manual replay against the captured `feat/m40-plus` truth yields a product-lane winner with blocked readiness rather than a planning answer
5. `git diff --stat -- .agents/skills/next-milestone/SKILL.md .agents/skills/next-milestone/references/rubric.md`
   - only those two files changed unless an explicitly justified scope expansion was accepted first

### Acceptance matrix

| Check | Command or method | Pass condition | Failure means |
|---|---|---|---|
| ban-language readback | `rg -n "planning milestone next|author a plan|no milestone|more evidence" ...` | only explicit ban references remain | planning can still leak into final output |
| output-shape readback | `rg -n "Executable wedge:|Confidence:|Ranked alternates:" .agents/skills/next-milestone/SKILL.md` | all required fields exist in one stable memo block | the output contract is still under-specified |
| rubric parity readback | read `rubric.md` after Step 2 | hard gates match `SKILL.md` wording closely enough to remove ambiguity | prompt/rubric drift will reintroduce non-determinism |
| captured-truth replay | manual `/next-milestone` dry run or structured readback against the replay baseline | winner is `shared-core-portability`, readiness is blocked, planning stays in handoff only | the selector still behaves like a permit office |
| scope check | `git diff --stat` | no unexpected files touched | the fix widened beyond the honest blast radius |

Regression rule:

The current failure is a behavioral regression in the selector contract. The landed implementation must preserve a replayable current-state example so this exact failure mode does not silently come back.

## Failure Modes Registry

| Codepath | Realistic failure | Required coverage | Error handling | User-visible impact |
|---|---|---|---|---|
| milestone selection | planning answer still emitted because one contradictory sentence survived | captured-truth replay | explicit hard ban in both files | operator gets paperwork instead of the next wedge |
| authority classification | historical `ORCH_PLAN.md` treated as the next `/autoplan` target | captured-truth replay | explicit stale-context rule | operator reviews the wrong artifact |
| ranking | winner emitted without loser reasons | output-block readback | required ranked alternates block | choice stays opaque and unstable |
| readiness split | blocked readiness demotes winner into `no milestone` | captured-truth replay | explicit readiness-vs-winner rule | selector fails exactly when evidence is messy |
| contract drift | `SKILL.md` and `rubric.md` disagree after edit | readback review | same-day sync in one milestone | future runs become non-deterministic |
| scope drift | implementation touches collector or YAML with no proven need | diff-scope check | closed implementation-surface rule | a two-file fix turns into fake progress |

Critical gap to avoid:

If the replay expectation is not checked into the touched contract surfaces, this milestone can look correct in prose and still regress silently. That is not acceptable.

## Performance Review

There is no runtime performance risk in the usual sense, but there is prompt-surface complexity risk.

Rules:

- do not add a third scoring layer
- do not add a second collector
- do not duplicate the taxonomy in multiple new places
- prefer editing existing sections over appending a long appendix
- keep the prompt footprint roughly flat while increasing decisiveness

The right outcome is a sharper contract with roughly the same prompt footprint, not a longer prompt that says the same thing three times.

## Worktree Parallelization Strategy

Sequential implementation, no honest parallelization opportunity.

The file count is small, but the semantics are tightly coupled. `rubric.md` cannot be finalized until `SKILL.md` freezes the winner/readiness/output contract, and replay validation depends on both.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| rewrite selector contract | `.agents/skills/next-milestone/` | — |
| tighten scoring gates | `.agents/skills/next-milestone/references/` | rewrite selector contract |
| replay and acceptance | `.agents/skills/next-milestone/`, repo-root authority context, restore snapshot | rewrite selector contract, tighten scoring gates |

### Parallel lanes

- Lane A: rewrite selector contract -> tighten scoring gates -> replay and acceptance

### Execution order

Launch one lane only. Finish `SKILL.md`, then `rubric.md`, then replay the captured branch truth.

### Conflict flags

- `SKILL.md` and `rubric.md` are semantically coupled even though they are different files.
- Replay interpretation depends on the exact wording frozen in both files.
- Parallel edits would create contract-drift risk for no real gain.

## TODOS.md Impact

No new `TODOS.md` entry is required for the plan as written.

Rules:

- if implementation stays within the two contract files, this milestone should land complete, not deferred
- if implementation proves that collector or YAML changes are necessary, stop and author a follow-on milestone or TODO explicitly instead of smuggling that expansion into this one

## Completion Summary

- Step 0: Scope Challenge - scope accepted as-is
- Architecture Review - 0 infrastructure issues, 1 contract contradiction identified
- Code Quality Review - 1 DRY risk flagged, keep `agents/openai.yaml` unchanged unless proven necessary
- Test Review - diagram produced, 5 required regression checks identified
- Performance Review - 1 prompt-surface complexity guardrail set
- NOT in scope - written
- What already exists - written
- Failure modes - 1 critical gap called out if replay coverage is omitted
- Parallelization - 1 lane, 0 parallel, 1 sequential
- TODOS.md impact - none unless scope expands
- Lake score - complete contract hardening, not a wording patch
