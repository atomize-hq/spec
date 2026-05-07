---
name: next-milestone
description: Auto-decide the next repo milestone or wedge from checkpoints, product docs, and live repo signals. Use when the user asks "what should be next", "what next", "pick the next milestone", "choose the next wedge", or wants an autoplan-like recommendation without answering intermediate questions.
---

# Next Milestone

## Overview

Pick one next move. Not three. Not "it depends."

This skill is the repo-local answer to milestone drift. It recovers the latest project context, reads the product spine, checks the current branch signals, scores the viable next moves with a fixed rubric, and returns one recommended milestone plus exact follow-up commands.

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

Only read these support docs when they are directly relevant to the current branch or checkpoint:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `docs/semantic_family_capability_corpus_guide_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/m26_implementation_plan_v0.1.md`
- `docs/m27_5_recommendation_quality_plan_v0.1.md`

### 2. Collect live repo signals

Run the bundled collector:

```bash
.agents/skills/next-milestone/scripts/collect_signals.sh
```

The script is intentionally narrow. It gives you the current branch, dirty state, recent commits, the latest checkpoint summary, and the current family-analysis signals without making you reconstruct them by hand.

### 3. Score the candidate next moves

Read `references/rubric.md` and score the realistic candidates. Start from the repo's usual candidate set:

1. Rust semantic-review wedge expansion
2. Reusable seam semantic-review expansion
3. First-class TypeScript function backend work
4. More family-analysis or corpus-governance work

Do not invent five more options unless the checkpoint or docs clearly introduce them.

### 4. Auto-decide, do not bounce the choice back to the user

This skill is closer to `/autoplan` than to brainstorming.

- Recommend one next move.
- Include up to two alternates.
- If the top choice is uncertain, still choose it and explain the uncertainty.
- Only ask the user a question if required inputs are missing or the repo is in a contradictory state.

### 5. Return a short decision memo

Use this output shape:

```markdown
NEXT MILESTONE

Recommendation: <one-line recommendation>

Why this wins:
- <product-core reason>
- <proof / truth reason>
- <boundedness reason>

Why not the others:
- <alt 1>
- <alt 2>

Evidence used:
- <checkpoint/doc/signal>
- <checkpoint/doc/signal>

Start here:
1. <first concrete step>
2. <second concrete step>
3. <exact command or file to touch>
```

## Decision rules

- Prefer product-core work over support machinery.
- Prefer work that creates new semantic-review truth over work that only reorganizes recommendation machinery.
- Prefer a bounded lake over an ocean. If something looks multi-quarter, it probably loses to a tighter wedge.
- Prefer reusing the live M26-style proof loop when it still fits.
- If the current recommendation surface says `no_strong_candidate`, treat that as evidence against spending another round on corpus steering unless missing evidence or stale evidence would clearly change the answer.
- If TypeScript is still metadata-plus-pilot and not a real `spec` backend, do not recommend TypeScript backend work unless the product docs or checkpoint explicitly say the repo is ready to pay that cost now.

## Repo-specific cautions

- Semantic review is the product core. Family-analysis governance is servant work.
- Do not confuse authored `body.typescript` support with first-class TypeScript target support. `spec generate/build/test` are still Rust-only unless the repo changes.
- Dead-code warnings or other local hygiene issues do not become the next milestone unless they block the recommended path.

## Resources

- Use `scripts/collect_signals.sh` for the fast repo snapshot.
- Use `references/rubric.md` for the scoring model and tie-breakers.
