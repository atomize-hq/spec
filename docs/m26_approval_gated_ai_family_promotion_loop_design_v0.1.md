# Design: M26 Approval-Gated AI Family Promotion Loop

**Version:** v0.1  
**Status:** Draft  
**Date:** 2026-04-29

> This design doc is a first pass. It is grounded in the current repo thesis:
> AI is supposed to operate the promotion loop under hard proof gates, while
> humans approve the family target and the final promoted result.

## Purpose

Define the first milestone that turns family promotion from a manually driven
engineering exercise into an approval-gated AI workflow.

This is not yet the multi-language milestone. It is the milestone that makes
future family expansion and future language expansion operable at the right
abstraction level.

## Problem Statement

After M24, the repo has proven that packetized family promotion can work for:

- a wrapper family
- one arithmetic leaf family
- its direct sibling arithmetic leaf family

That is useful, but it is still too manual.

Today, the workflow assumes a human can hold too much of the ceremony in their
head:

- what the next family should be
- how to scaffold the packet truthfully
- what the four canonical buckets should look like
- which tests must exist for classifier, corpus, truth-surface, and regression proof
- what counts as a blocker versus a fixable failure

That is the wrong operator model for the repo thesis.

The desired model is:

1. AI recommends the next family candidate.
2. Human approves or rejects that recommendation.
3. AI performs the promotion loop inside hard gates.
4. Human reviews the final promotion report and approves or rejects the result.

If the repo cannot support that loop, it is not yet ready for high-surface family
expansion, and it is definitely not ready for multi-language growth.

## Goal

Make one full semantic-family promotion operable by AI with only two human
approval points:

- approval of the next target family
- approval of the final promoted output

Everything in between should be machine-driven, observable, and bounded by
existing proof commands.

## Non-Goals

This milestone is not trying to:

- make family selection globally optimal
- genericize all family templates
- make the promotion kernel language-agnostic yet
- land a second-language backend
- solve broad reverse-ingestion
- remove all manual editorial judgment from packet curation

It is a workflow milestone, not a universality milestone.

## Success Criteria

M26 is successful only if all of the following are true:

1. AI can produce a structured recommendation for the next family candidate.
2. After human approval, AI can run the family promotion loop without ad hoc
   human steering between steps.
3. The loop terminates in one of two honest ways:
   - green `smoke`, `prove`, and `certify`
   - a precise blocker report naming what the system could not infer or satisfy
4. The final output includes a machine-readable and human-readable promotion
   report explaining what was proved.
5. Human interaction is limited to the two approval boundaries, not hidden
   rescue work inside the loop.

## Operator Model

### Human responsibilities

- approve the proposed target family
- approve or reject the final promoted output

### AI responsibilities

- inspect current repo truth
- recommend the next family candidate
- scaffold the promotion workspace
- author or curate packet content
- add or refine family-owned tests
- run proof gates
- diagnose failures
- iterate toward green
- emit final report

### Hard rule

If AI needs more than the two planned approvals to finish ordinary promotion
work, the milestone has not landed cleanly.

## Inputs

The loop should operate from explicit repo truth, not hidden chat memory.

### Required inputs

- current promoted family registry
- current unsupported or unpromoted shape inventory
- existing family packet conventions
- existing `xtask family new/smoke/prove/certify` commands
- current semantic review and CLI regression surfaces

### Human approval input

The human chooses from an AI recommendation packet that should include:

- proposed family id
- short statement of semantic boundary
- why this family is the right next target
- expected leverage
- expected risk

## Proposed Outputs

### 1. Family recommendation packet

A structured artifact that says:

- candidate family id
- summary
- corpus evidence
- relationship to existing promoted families
- expected buckets
- likely blockers

### 2. Promotion execution report

A structured artifact that records:

- approved family id
- files created or changed
- commands run
- proof gate results
- failures encountered and how they were resolved
- final status

### 3. Final approval bundle

A compact human review surface containing:

- semantic boundary claimed
- exact proof surfaces added
- routing and shadowing consequences
- whether `smoke`, `prove`, and `certify` are green
- unresolved concerns, if any

## Workflow Shape

### Phase A — Recommend

AI inspects repo truth and emits a short ranked candidate list.

Minimum output:

- top candidate
- one backup candidate
- why the top candidate wins now

Human then approves the next family target.

### Phase B — Promote

After approval, AI performs the promotion loop:

1. create or refresh scaffold
2. curate packet truth
3. add classifier proof
4. add corpus proof
5. add truth-surface proof
6. add regression proof
7. run `smoke`
8. run `prove`
9. run `certify`
10. iterate until green or blocked

### Phase C — Report

AI emits the final promotion report and approval bundle.

Human approves or rejects the final result.

## Architecture Consequences

M26 should bias the repo toward explicit machine-operable artifacts.

That likely means adding or firming up:

- a structured next-family recommendation surface
- a structured promotion report surface
- clearer mapping between proof gates and expected tests
- clearer distinction between fixable failures and true blockers

The milestone should prefer explicit files and machine-readable reports over
chat-only reasoning. If the loop only works because the AI "remembers how this
repo likes to do things," that is fake automation.

## Risks

### Risk 1: The AI can only scaffold, not curate

This would mean the current packet conventions are still too implicit.

### Risk 2: Proof failures are not diagnosable enough

If `smoke`, `prove`, or `certify` fail without precise machine-actionable
feedback, the AI loop will stall or thrash.

### Risk 3: Recommendation quality is weak

If the candidate recommendation logic is too shallow, the human approval step
becomes noise instead of governance.

### Risk 4: Rust assumptions leak into the workflow core

This is acceptable in M26 to a point, but the design should avoid deepening
Rust-specific assumptions in the shared loop where possible.

## Acceptance Gates

M26 should be considered landed only when:

- one real family promotion has been completed through the approval-gated AI loop
- the only human approvals were target selection and final output approval
- the final promoted family is green on:
  - `cargo xtask family smoke <family>`
  - `cargo xtask family prove <family>`
  - `cargo xtask family certify <family>`
- the loop emitted durable recommendation and promotion reports
- at least one failed iteration was handled by the AI loop without extra human
  steering, unless the first run happened to go green directly

## Open Questions

- Should the recommendation packet live in-repo, under `.semantic-family-artifacts/`,
  or both?
- Should AI recommend exactly one family or a ranked top-N list?
- What is the minimum machine-readable blocker schema?
- Should the final approval bundle embed fixture digests and report digests, or
  just link to them?
- How much of the recommendation logic belongs in `xtask` versus higher-level AI
  orchestration?

## Recommended Next Step

Use this design as the basis for a narrower implementation plan that names:

- the new artifacts to add
- the exact commands the AI operator is allowed to run
- the expected recommendation and report schemas
- the single real family that will be used as the first end-to-end M26 proof
