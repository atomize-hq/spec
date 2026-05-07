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

Do not collapse these six family names into looser prose. They are part of the output contract.

### 4. Auto-decide, do not bounce the choice back to the user

This skill is closer to `/autoplan` than to brainstorming.

- Recommend one next move.
- Include up to two alternates.
- If the top choice is uncertain, still choose it and explain the uncertainty.
- Only ask the user a question if required inputs are missing or the repo is in a contradictory state.

Mechanical ambiguity should not become a user question. Resolve it with source hierarchy and explicit guardrails.

### 4.5 Decide the handoff artifact

Do not stop at "what milestone next?" Also decide what artifact should exist next and whether `/autoplan` is the correct immediate next tool.

This skill is read-only. It must not clear, replace, archive, or rewrite `PLAN.md` or `ORCH_PLAN.md`. Its job is to classify those files correctly, not mutate them.

You must decide:

- `Implementation readiness: <ready-now | needs_artifact_first>`
- `Next artifact kind: <design_doc | authority_plan_draft | authority_plan>`
- `Autoplan ready: <yes | no>`
- `Authority file states:`
  - `PLAN.md: <completed_authority_context | active_execution_contract | draft_next_artifact | stale_historical_artifact | none>`
  - `ORCH_PLAN.md: <completed_authority_context | active_execution_contract | draft_next_artifact | stale_historical_artifact | none>`

Use these rules:

- If `Implementation readiness = ready-now`:
  - default to `Next artifact kind: authority_plan`
  - default to `Autoplan ready: yes`
  - the handoff may go straight into `/autoplan`
- If `Implementation readiness = needs_artifact_first`:
  - default to `Autoplan ready: no`
  - choose `Next artifact kind: design_doc` when the seam, trigger table, or proof gate framing is still under-defined and needs problem-shaping first
  - choose `Next artifact kind: authority_plan_draft` when the source hierarchy already gives enough structure to draft the bounded authority plan directly
- Only emit `Autoplan ready: yes` for `needs_artifact_first` when the required draft artifact already exists, is clearly the file `/autoplan` should review, and passes the artifact-readiness check below

Authority file state classification:

- `completed_authority_context`
  - use this when the latest relevant closeout proves that milestone landed and the file still describes that completed milestone
  - a finished `ORCH_PLAN.md` for a closed planning run usually belongs here once its execution job is done
  - completed authority context is evidence, not the next review target
- `active_execution_contract`
  - use this when the file still governs an in-flight run or current execution lane and is not yet closed out
- `draft_next_artifact`
  - use this only when the file is the actual next draft artifact for the next milestone and is reviewable as-is
  - a current `PLAN.md` can stay here even after the planning run that authored it closes out, but only if the closeout shows the draft was authored or refined and implementation remains gated
- `stale_historical_artifact`
  - use this when the file is older residue that is neither the current active contract nor the next draft artifact
- `none`
  - use this when the file does not exist

Never assume `PLAN.md` or `ORCH_PLAN.md` are always the previous landed milestone.
Never assume they are always the next draft artifact either.
Classify them from repo truth each time.

Artifact-readiness check for `authority_plan_draft`:

- it defines the scoped contract directly rather than saying its job is to author the plan
- it names the candidate seam or architecture boundary concretely enough to review
- it names the trigger table or equivalent gating conditions concretely enough to review
- it names the proof gates or evidence thresholds that separate planning authorization from implementation authorization
- it names explicit non-goals
- it is reviewable as-is by `/autoplan` without asking `/autoplan` to invent the artifact boundary first
- it is not just completed authority context from the last landed milestone

If any of those are missing, `Autoplan ready` must stay `no` even if a draft file already exists.

Positive signals for `authority_plan_draft` / `Autoplan ready: yes`:

- the latest relevant closeout says the planning run authored or refined the draft artifact and still left implementation gated
- the file already carries the seam or boundary definition, trigger table, proof gates, non-goals, and any needed future parallelization or execution split directly
- the file no longer asks a later agent to decide what artifact should exist before review can begin

Automatic negative signals for `draft_next_artifact` / `Autoplan ready: yes`:

- if the file says `draft planning candidate for /autoplan review`, treat that as evidence it is still a draft candidate, not automatically a ready review target
- if the file says `author the ... plan`, treat that as evidence the artifact is still partly meta
- if the file says `run /autoplan on this plan candidate`, treat that as evidence the artifact boundary is not yet fully settled
- if the latest closeout proves the file's milestone already landed and the file now serves only as historical context, classify it as `completed_authority_context`, not `draft_next_artifact`
- do not auto-demote a current `PLAN.md` draft just because the planning run that authored it has a closeout; first check whether the draft now passes the readiness criteria as the exact review target

Never hand off a planning-follow-on recommendation as "proceed to `/autoplan` based on these findings." `/autoplan` reviews a plan artifact. It should not be asked to invent the artifact boundary from a milestone memo alone.

### 5. Return a short decision memo

Use this output shape:

```markdown
NEXT MILESTONE

Milestone family: <semantic-review-substrate | rust-family-promotion | corpus-recommendation-policy | shared-core-portability | second-language-backend | operator-consumer-tooling>
Implementation readiness: <ready-now | needs_artifact_first>
Next artifact kind: <design_doc | authority_plan_draft | authority_plan>
Autoplan ready: <yes | no>
Authority file states:
- PLAN.md: <completed_authority_context | active_execution_contract | draft_next_artifact | stale_historical_artifact | none>
- ORCH_PLAN.md: <completed_authority_context | active_execution_contract | draft_next_artifact | stale_historical_artifact | none>
Recommendation: <one-line recommendation>

Why this wins:
- <product-core reason>
- <proof / truth reason>
- <boundedness reason>

Why not the others:
- <alt 1>
- <alt 2>

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
If implementation is not ready, make that explicit in `Implementation readiness` and in the handoff rather than turning planning into the milestone recommendation.
If the answer depends on current authority docs, say "authority context" or "current milestone authority" rather than calling those docs "live signals."
If a command did not run, failed, or was unavailable, say that plainly. Do not backfill a fake live-signal claim from memory or nearby docs.
If `Autoplan ready: no`, the handoff must name the artifact that gates the recommended milestone and must not tell the user to run `/autoplan` yet.
If `Autoplan ready: yes`, name the exact file `/autoplan` should review.
If `Next artifact kind: authority_plan_draft` and `Autoplan ready: yes`, briefly justify why the draft passes the artifact-readiness check and unblocks the recommended milestone.
If an authority file is classified as `completed_authority_context`, do not target it with `/autoplan`.
If an authority file is classified as `active_execution_contract`, do not target it with `/autoplan` unless the next milestone is explicitly to review that same in-flight execution contract.

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
- If `required_next_action` still points at the current planning artifact, do not synthesize a later M41-style follow-on from that artifact's future trigger table or authorization gate.
- A draft file existing on disk is not enough by itself to make `/autoplan` ready. The draft must already behave like a reviewable authority plan, not a plan-to-write-a-plan.
- If the latest closeout proves an orchestration run landed, `ORCH_PLAN.md` should usually classify as `completed_authority_context`, not `draft_next_artifact`.
- If that same closeout proves the run authored or refined the current next authority-plan draft while leaving implementation gated, `PLAN.md` may still classify as `draft_next_artifact`.
- If a closeout or plan says `implementation still gated`, treat that as evidence against recommending the gated successor milestone right now.
- Do not turn a future trigger-table row, `M41` gate branch, or `allowed if...` clause into the next milestone unless current evidence proves the trigger fired or a higher-priority source explicitly names that follow-on.
- Do not treat repo-root `PLAN.md` as authoritative by default. It must earn that role by clearly being the current branch's active plan and not merely prior landed work.
- If the latest closeout shows the current `ORCH_PLAN.md` execution contract is complete, treat it as completed authority context, not the next milestone contract.
- If the latest closeout shows the current `PLAN.md` draft is the authored output of that completed planning run, decide between `draft_next_artifact` and `completed_authority_context` using the artifact-readiness check rather than auto-demoting it.
- If the best honest answer is "planning milestone next, implementation later," say exactly that.
- If a live signal is branch-local noise but the frozen decision surfaces are stable, call it secondary evidence, not the primary reason.
- Do not label `PLAN.md`, `ORCH_PLAN.md`, or closeout files as live signals. They are authority context or closeout context.
- Do not say "touch `PLAN.md` first" unless repo convention and current authority context clearly make repo-root `PLAN.md` the next milestone authority file.
- If `PLAN.md` is only likely, say "author the next milestone authority plan, likely in `PLAN.md`."
- Do not say "replace `PLAN.md`" unless repo convention clearly requires replacement rather than a new authority artifact.
- Do not emit a plan artifact whose main purpose is to say "write the plan." The artifact itself must carry the scoped contract once it exists.

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
