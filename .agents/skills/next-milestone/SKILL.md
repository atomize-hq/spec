---
name: next-milestone
description: Auto-decide the next repo milestone or wedge from checkpoints, product docs, frozen decision docs, closeouts, and live repo signals. Use when the user asks "what should be next", "what next", "pick the next milestone", "choose the next wedge", or wants an autoplan-like recommendation without answering intermediate questions.
---

# Next Milestone

## Overview

Pick one next move. Not three. Not "it depends."

This skill is the repo-local answer to milestone drift. It recovers the latest project context, reads the product spine, checks the current branch signals, scores the viable product-surface moves with a fixed rubric, and returns one recommended milestone plus exact follow-up commands.

It must distinguish three different questions:

1. What strategic lane should the repo be in next?
2. What concrete product-surface milestone should ship next?
3. What prerequisite artifact, if any, is still needed before that milestone can start honestly?

## Workflow

### 1. Recover context first

Read the newest checkpoint before doing anything else:

```bash
eval "$($HOME/.codex/skills/gstack/bin/gstack-slug 2>/dev/null)" 2>/dev/null || true
PROJ="${GSTACK_HOME:-$HOME/.gstack}/projects/${SLUG:-atomize-hq-spec}"
find "$PROJ/checkpoints" -maxdepth 1 -name "*.md" -type f 2>/dev/null | xargs ls -1t 2>/dev/null | head -1
```

Read that checkpoint file fully.

Then read the three core product docs:

- `docs/north_star_v0.2.md`
- `docs/high_level_technical_architecture_v0.2.md`
- `docs/roadmap_and_release_shape_v0.1.md`

Then read the frozen decision surfaces that can override a naive wedge recommendation:

- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`

Only then read these support docs when they are directly relevant to the current branch or checkpoint:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `docs/m26_implementation_plan_v0.1.md`
- `docs/m27_5_recommendation_quality_plan_v0.1.md`

Treat repo-root `PLAN.md` and `ORCH_PLAN.md` as conditional sources, not default authority.

- Use them when they are clearly the active plan for the current branch or the currently in-flight milestone.
- Demote them when they describe prior landed work, a bounded already-shipped milestone, or a stale branch-specific contract.
- If a plan says `required_next_action = author_*_plan`, that is evidence that the winning product milestone is not implementation-ready yet. Do not turn the planning artifact into the milestone recommendation itself.

### 2. Collect live repo signals

Run the bundled collector:

```bash
.agents/skills/next-milestone/scripts/collect_signals.sh
```

The script is intentionally narrow. It gives you the current branch, dirty state, recent commits, the latest checkpoint summary, current authority-plan headers, the latest authority closeout context, and the live family-analysis command signals, including the bounded corpus-decision action contract, without making you reconstruct them by hand.

### 3. Score the candidate next moves

Read `references/rubric.md` and score the realistic candidates.

First classify each realistic candidate into exactly one milestone family from this fixed set:

1. `semantic-review-substrate`
   Base `spec` / reviewer capability growth.
   Use this when the next milestone is about broadening what the core semantic-review substrate can truthfully understand, not merely promoting one more Rust family.
2. `rust-family-promotion`
   New supported or promoted Rust semantic-review wedges.
   Use this when the next milestone is about proving and shipping another bounded Rust family-shaped capability.
3. `corpus-recommendation-policy`
   Corpus expansion, recommendation-policy hardening, or decision-contract work.
   Use this when the repo is still missing evidence or decision-surface honesty, not when it is ready for a bigger product-core move.
4. `shared-core-portability`
   Seam / shared-core / portability-boundary architecture.
   Use this when the next blocker is making the shared boundary honest before broader reuse or language expansion.
5. `second-language-backend`
   Real second-language backend execution support.
   Use this only for actual backend work like `spec generate/build/test` support, not metadata-only authored fields or bounded pilots that already landed.
6. `operator-consumer-tooling`
   Maintainer / operator / consumer truth surfaces.
   Use this for new honest consumers of repo truth, verification readers, orchestration surfaces, or maintainer-facing proof ergonomics.

Within the chosen family, choose the concrete product-surface milestone, not the prerequisite planning task.
The skill may still say that the milestone is blocked on an artifact, but it must not recommend "planning follow-on" as the milestone itself.
If sources say `required_next_action = author_*_plan`, keep the winner on the product lane and move that planning requirement into `Implementation readiness`, `Next artifact kind`, `Autoplan ready`, and `Handoff` only.
If sources say `recommendation_status = no_strong_candidate`, that does not authorize `planning`, `no milestone`, `more evidence`, or any other null final answer. Force-rank one honest product-surface winner and explain the loser reasons.

Do not collapse these six family names into looser prose. They are part of the output contract.

### 4. Auto-decide, do not bounce the choice back to the user

This skill is closer to `/autoplan` than to brainstorming.

- Recommend one next move.
- Include up to two alternates.
- If the top choice is uncertain, still choose it and explain the uncertainty.
- Only ask the user a question if required inputs are missing or the repo is in a contradictory state.
- The final answer must force one winner. Planning may appear only as a gating artifact, never as the recommended milestone.

Mechanical ambiguity should not become a user question. Resolve it with source hierarchy and explicit guardrails.

### 4.5 Decide the handoff artifact

Do not stop at "what milestone next?" Also decide what artifact should exist next and whether `/autoplan` is the correct immediate next tool.

This skill is read-only. It must not clear, replace, archive, or rewrite `PLAN.md` or `ORCH_PLAN.md`. Its job is to classify those files correctly, not mutate them.

Repo invariant for this project:

- `ORCH_PLAN.md` is always an execution contract, never a plan draft.
- By the time `/next-milestone` runs, repo-root `PLAN.md` and repo-root `ORCH_PLAN.md` are already completed authority context if they exist.
- During `/next-milestone`, treat those two repo-root files as evidence inputs, not active work targets.
- The handoff should point to a fresh artifact or refreshed authority file for the next move, not re-open the current repo-root `PLAN.md` or `ORCH_PLAN.md` as in-flight execution state.

You must decide:

- `Implementation readiness: <ready-now | needs_artifact_first>`
- `Next artifact kind: <design_doc | authority_plan_draft | authority_plan>`
- `Autoplan ready: <yes | no>`
- `Authority file states:`
  - `PLAN.md: <completed_authority_context | none>`
  - `ORCH_PLAN.md: <completed_authority_context | none>`

Use these rules:

- If `Implementation readiness = ready-now`:
  - default to `Next artifact kind: authority_plan`
  - default to `Autoplan ready: yes`
  - the handoff may go straight into `/autoplan`
- If `Implementation readiness = needs_artifact_first`:
  - default to `Autoplan ready: no`
  - choose `Next artifact kind: design_doc` when the next milestone should start with a fresh gstack design doc or when `/gstack-autoplan` can bootstrap the missing design doc from completed authority context
  - choose `Next artifact kind: authority_plan_draft` only when repo convention explicitly requires a fresh authority-plan draft before any `/gstack-autoplan` review and the intended draft shape is already concrete
- Emit `Autoplan ready: yes` for `needs_artifact_first` when `/gstack-autoplan` is the truthful immediate next tool, including the case where it will bootstrap a missing design doc via its inline prerequisite flow
- Emit `Autoplan ready: no` for `needs_artifact_first` only when the user truly must author a fresh artifact outside `/gstack-autoplan` first

Authority file state classification:

- `completed_authority_context`
  - use this for repo-root `PLAN.md` and repo-root `ORCH_PLAN.md` whenever those files exist during `/next-milestone`
  - `ORCH_PLAN.md` reaches this state as a completed execution contract
  - `PLAN.md` reaches this state as completed branch authority context for the last landed move
  - completed authority context is evidence, not the next review target
- `none`
  - use this when the file does not exist

For this repo, do not classify repo-root `PLAN.md` or repo-root `ORCH_PLAN.md` as `active_execution_contract`, `draft_next_artifact`, or `stale_historical_artifact` during `/next-milestone`.
Classify them from repo truth each time, but enforce the repo invariant above.

Artifact-readiness check for `authority_plan_draft`:

- it defines the scoped contract directly rather than saying its job is to author the plan
- it names the candidate seam or architecture boundary concretely enough to review
- it names the trigger table or equivalent gating conditions concretely enough to review
- it names the proof gates or evidence thresholds that separate planning authorization from implementation authorization
- it names explicit non-goals
- it is reviewable as-is by `/autoplan` without asking `/autoplan` to invent the artifact boundary first
- it is not just completed authority context from the last landed milestone

If any of those are missing, `Autoplan ready` must stay `no` even if a draft file already exists.

Artifact-readiness check for `design_doc` / `/gstack-autoplan` handoff:

- completed authority context plus current repo signals already define the next milestone tightly enough for `/gstack-autoplan` to start from them
- the next move benefits from the `/gstack-autoplan` review pipeline creating or tightening the design doc before implementation
- if no design doc exists yet, `/gstack-autoplan` can still be the immediate next tool because it offers `/office-hours` inline to create the prerequisite design doc
- the handoff names `/gstack-autoplan` explicitly instead of telling the user to author the design doc manually first

Positive signals for `authority_plan_draft` / `Autoplan ready: yes`:

- the latest relevant closeout says the planning run authored or refined the draft artifact and still left implementation gated
- the file already carries the seam or boundary definition, trigger table, proof gates, non-goals, and any needed future parallelization or execution split directly
- the file no longer asks a later agent to decide what artifact should exist before review can begin

Positive signals for `design_doc` / `Autoplan ready: yes`:

- the completed authority context already names the seam, trigger table, proof floor, non-goals, or milestone boundary tightly enough to seed `/gstack-autoplan`
- the next milestone is better served by a fresh gstack design doc than by reusing the completed repo-root authority files
- `/gstack-autoplan` is the desired immediate next tool because it can create the missing design doc through its inline prerequisite flow and then continue the review

Automatic negative signals for `draft_next_artifact` / `Autoplan ready: yes`:

- if the file says `draft planning candidate for /autoplan review`, treat that as evidence it is still a draft candidate, not automatically a ready review target
- if the file says `author the ... plan`, treat that as evidence the artifact is still partly meta
- if the file says `run /autoplan on this plan candidate`, treat that as evidence the artifact boundary is not yet fully settled
- if the latest closeout proves the file's milestone already landed and the file now serves only as historical context, classify it as `completed_authority_context`, not `draft_next_artifact`

Never hand off a planning-follow-on recommendation as "proceed to `/autoplan` based on these findings." `/autoplan` reviews a plan artifact. It should not be asked to invent the artifact boundary from a milestone memo alone.

### 5. Return a short decision memo

Use this output shape:

```markdown
NEXT MILESTONE

Milestone family: <semantic-review-substrate | rust-family-promotion | corpus-recommendation-policy | shared-core-portability | second-language-backend | operator-consumer-tooling>
Executable wedge: <one concrete product-surface wedge inside the winning family>
Confidence: <high | medium | low>
Implementation readiness: <ready-now | needs_artifact_first>
Next artifact kind: <design_doc | authority_plan_draft | authority_plan>
Autoplan ready: <yes | no>
Authority file states:
- PLAN.md: <completed_authority_context | none>
- ORCH_PLAN.md: <completed_authority_context | none>
Recommendation: <one-line recommendation naming the winning wedge itself; if blocked, append the gate as "gated on <artifact>" rather than telling the user to author or refresh the artifact here>

Why this wins:
- <product-core reason>
- <proof / truth reason>
- <boundedness reason>

Ranked alternates:
1. <runner-up wedge> - <why it loses to the winner right now>
2. <third-place wedge> - <why it loses to the winner right now>

Evidence used:
- Checkpoint: <checkpoint file or summary>
- Frozen decision docs: <frozen docs>
- Authority context: <current authority plan or closeout>
- Live signals: <command output or branch-local runtime signal>

Handoff:
1. <first concrete artifact or decision step>
2. <second concrete step>
3. <say whether `/autoplan` should run now or only after the artifact exists>
```

The recommendation must be a product-surface milestone, not "planning next."
`Executable wedge` must name the concrete winner, not a planning label, family label, or null answer.
`Confidence` must reflect the evidence quality after the winner is chosen. It is not permission to refuse ranking.
If implementation is not ready, make that explicit in `Implementation readiness` and in the handoff rather than turning planning into the milestone recommendation.
The `Recommendation` line must lead with the winning wedge itself. It may mention a blocking artifact only as a gate on that wedge, for example `Recommendation: pursue <wedge>, gated on <artifact>`.
The `Recommendation` line must not begin with or center on `author`, `write`, `refresh`, `draft`, `plan`, or `/autoplan`.
If the answer depends on current authority docs, say "authority context" or "current milestone authority" rather than calling those docs "live signals."
If a command did not run, failed, or was unavailable, say that plainly. Do not backfill a fake live-signal claim from memory or nearby docs.
If `Autoplan ready: no`, the handoff must name the artifact that gates the recommended milestone and must not tell the user to run `/autoplan` yet.
If `Autoplan ready: yes` and `Next artifact kind: design_doc`, tell the user to run `/gstack-autoplan` now and say that it should produce or tighten the fresh gstack design doc for the next milestone.
If `Autoplan ready: yes` and the target artifact already exists, name the exact file `/autoplan` should review.
If `Next artifact kind: authority_plan_draft` and `Autoplan ready: yes`, briefly justify why the draft passes the artifact-readiness check and unblocks the recommended milestone.
If an authority file is classified as `completed_authority_context`, do not target it with `/autoplan`.
Do not target repo-root `PLAN.md` or repo-root `ORCH_PLAN.md` with `/autoplan` during `/next-milestone`; recommend a fresh artifact or refreshed authority file instead.
Hard-banned final outputs: `planning`, `planning milestone next`, `author a plan`, `no milestone`, `more evidence`.
If `required_next_action = author_*_plan`, the only allowed place for that planning requirement is readiness and handoff semantics. It must not replace the winning wedge.
If the output still reads like "author a fresh plan before any implementation work" in the `Recommendation` slot, it is wrong and must be rewritten so the wedge stays the subject.
If the best truthful immediate next tool is `/gstack-autoplan`, do not hide that behind a manual-authoring handoff. Say so directly and set `Next artifact kind` / `Autoplan ready` to match.

## Decision rules

- Respect source hierarchy over enthusiasm.
- Prefer product-core work over support machinery.
- Prefer work that creates new semantic-review truth over work that only reorganizes recommendation machinery.
- Prefer a bounded lake over an ocean. If something looks multi-quarter, it probably loses to a tighter wedge.
- Prefer reusing the live M26-style proof loop when it still fits.
- If the current recommendation surface says `no_strong_candidate`, treat that as evidence against spending another round on corpus steering unless missing evidence or stale evidence would clearly change the answer.
- If TypeScript is still metadata-plus-pilot and not a real `spec` backend, do not recommend TypeScript backend work unless the product docs or checkpoint explicitly say the repo is ready to pay that cost now.

## Source hierarchy

When sources conflict, use this order:

1. Latest checkpoint
2. Frozen decision-contract docs and program trackers
3. Core product docs
4. Latest relevant closeout artifacts in `.runs/`
5. Current branch active plan, only if clearly active
6. Live command signals from `collect_signals.sh`
7. Historical milestone plans as background only

Do not let a noisy or stale lower-priority source override a cleaner higher-priority one.

## Evidence attribution guardrails

- Never say a command passed unless it was shown by `collect_signals.sh` or run in the current session.
- Never attribute `decision_action` or `required_next_action` to live signals unless `cargo xtask family corpus-decision --format json` actually ran.
- Never attribute decision-contract parity or pass/fail claims to anything except `cargo xtask family verify-decision-contract --format json`.
- If a live command is unavailable, failed, or returned inconsistent artifacts, say that explicitly instead of silently filling the gap from frozen docs.
- When frozen docs and live commands agree, say both. Do not collapse them into one evidence bucket.

## Planning vs implementation guardrails

- `semantic-review-substrate` is not the same lane as `rust-family-promotion`.
  "Support more semantic understanding at the base layer" and "promote one more Rust family" are different decisions and should not be merged casually.
- `shared-core-portability` is not the same lane as `second-language-backend`.
  "Make the seam boundary honest" and "ship real TypeScript or other backend support" are different decisions and should not be merged casually.
- `operator-consumer-tooling` is a real milestone family, not a misc bucket for leftovers.
  If the actual missing capability is a maintainer-facing truth consumer or verification surface, say so plainly.
- If sources say `pivot_to_architecture_shared_core_follow_on`, do not silently rewrite that into "begin shared-core extraction now."
- If sources say `required_next_action = author_architecture_follow_on_plan`, keep the milestone recommendation on the relevant product lane and mark it `needs_artifact_first` unless a higher-priority source explicitly upgrades that to ready-now implementation.
- If sources say `required_next_action = author_*_plan`, do not recommend the planning work as the milestone. Treat it as the gating artifact for the winning product milestone.
- If sources say `required_next_action = author_*_plan`, do not hand off directly to `/autoplan` unless that plan artifact already exists and is clearly the intended review target.
- Exception for this repo: if the desired next artifact is a fresh gstack design doc and `/gstack-autoplan` can bootstrap it from completed authority context plus inline `/office-hours`, hand off directly to `/gstack-autoplan` instead of requiring manual pre-authoring.
- If `required_next_action` still points at the current planning artifact, do not synthesize a later M41-style follow-on from that artifact's future trigger table or authorization gate.
- A draft file existing on disk is not enough by itself to make `/autoplan` ready. The draft must already behave like a reviewable authority plan, not a plan-to-write-a-plan.
- For this repo, `ORCH_PLAN.md` is always execution-only and should classify as `completed_authority_context` during `/next-milestone` if it exists.
- For this repo, repo-root `PLAN.md` should also classify as `completed_authority_context` during `/next-milestone` if it exists.
- If a closeout or plan says `implementation still gated`, treat that as evidence against recommending the gated successor milestone right now.
- When completed authority context already contains the seam, trigger table, proof floor, and non-goals for the next milestone, prefer `Next artifact kind: design_doc` plus `Autoplan ready: yes` over a manual `authority_plan_draft` handoff.
- Do not turn a future trigger-table row, `M41` gate branch, or `allowed if...` clause into the next milestone unless current evidence proves the trigger fired or a higher-priority source explicitly names that follow-on.
- Do not treat repo-root `PLAN.md` or repo-root `ORCH_PLAN.md` as the next review target during `/next-milestone`; they are completed authority context for the move that already landed.
- If a live signal is branch-local noise but the frozen decision surfaces are stable, call it secondary evidence, not the primary reason.
- Do not label `PLAN.md`, `ORCH_PLAN.md`, or closeout files as live signals. They are authority context or closeout context.
- Do not say "touch `PLAN.md` first" when you mean the current repo-root `PLAN.md`; ask for or recommend a fresh artifact instead.
- If repo convention suggests the next authority artifact will probably live at repo-root `PLAN.md`, say "refresh `PLAN.md` into the next authority artifact" rather than treating the current file as already active.
- Do not emit a plan artifact whose main purpose is to say "write the plan." The artifact itself must carry the scoped contract once it exists.
- For the captured `feat/m40-plus` branch truth, if evidence still includes `pivot_to_architecture_shared_core_follow_on` plus `author_architecture_follow_on_plan`, the winner stays in `shared-core-portability`; planning remains handoff-only and must not become the recommendation.

## Repo-specific cautions

- Semantic review is the product core. Family-analysis governance is servant work.
- Corpus / recommendation / decision-policy work is usually support work, not the headline next move, unless frozen decision surfaces clearly say evidence honesty is still the blocker.
- Do not confuse authored `body.typescript` support with first-class TypeScript target support. `spec generate/build/test` are still Rust-only unless the repo changes.
- Dead-code warnings or other local hygiene issues do not become the next milestone unless they block the recommended path.
- Do not recommend planning as the milestone. Planning exists only to enable the winning product milestone when readiness is not there yet.
- In this repo, `/autoplan` should review an actual design doc or authority-plan draft, not a milestone memo that still leaves the artifact boundary implicit.

## Resources

- Use `scripts/collect_signals.sh` for the fast repo snapshot.
- Use `references/rubric.md` for the scoring model and tie-breakers.
