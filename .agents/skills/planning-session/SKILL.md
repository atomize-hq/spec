---
name: planning-session
description: Run the repo's full planning loop from one pasted implementation closeout. Use when the user wants to take the last implementation-phase message and, in one session, produce the next-milestone memo, a fresh gstack design doc, a fresh repo-root PLAN.md, a second PLAN.md polish pass, and a fresh ORCH_PLAN.md. Each stage must run on a fresh GPT-5.4 subagent at reasoning high; the ORCH stage may reuse its own agent only for correction passes.
---

# Planning Session

## Goal

Take one prior implementation-phase last message and run the whole planning loop end to end:

1. next milestone memo
2. fresh design doc
3. fresh `PLAN.md`
4. one more `PLAN.md` solidification pass
5. fresh `ORCH_PLAN.md`

This is an execution skill, not a brainstorming skill.

## Input Contract

- Required input: the exact prior implementation last message.
- Do not ask for the milestone number, design doc path, `PLAN.md`, or `ORCH_PLAN.md` path unless the closeout text is missing.
- Treat repo-root `PLAN.md` and repo-root `ORCH_PLAN.md` as existing authority context until stages 3-5 intentionally replace them.
- If `spawn_agent` is unavailable, stop and report that the wrapper cannot satisfy the required fresh-subagent contract.
- Infer the previous milestone number from the pasted closeout text, then compute the next milestone as `previous + 1`.
- If the closeout does not expose a milestone number clearly enough to compute `+1`, stop and report that exact blocker instead of guessing.

## Setup

1. Run `.agents/skills/planning-session/scripts/init_session.sh`.
2. Save the pasted closeout text to the emitted `PRIOR_MESSAGE_FILE`.
3. Read:
   - `references/prompt_contract.md`
   - `references/orch_review_checklist.md`
4. Keep all intermediate artifacts in the emitted `SESSION_DIR`.

## Fresh Subagent Rule

- Stages 1-4 each use a new `spawn_agent` call with:
  - `model: gpt-5.4`
  - `reasoning_effort: high`
  - `fork_context: false`
- Stage 5 uses one fresh `gpt-5.4` / `high` subagent for the initial ORCH draft.
- The wrapper-owned Stage 5 agent is itself the required fresh session for the initial ORCH draft, so this rule is enforced by the wrapper and does not need to be repeated inside the Stage 5 prompt text.
- If the ORCH draft is insufficient, reuse that same Stage 5 subagent with `send_input` for correction rounds.
- Do not reuse Stage 1-4 agents for later stages.
- Do not parallelize this wrapper. The stages are serialized by artifact dependency.

## Prompt Fidelity Rule

- The stage prompts come from `references/prompt_contract.md`.
- Use them exactly as written.
- The only allowed changes are:
  - insert the pasted prior implementation last message
  - compute the next milestone number as `previous milestone + 1`
  - insert the exact next-milestone memo returned by Stage 1
  - insert the exact design-doc path produced by Stage 2
- Do not add wrapper narration, extra guardrails, or rewritten wording inside the stage prompts.

## Workflow

### 1. Next milestone

- Compute `NEXT_MILESTONE_NUMBER` from the pasted closeout before building the prompt.
- Spawn a fresh agent.
- Give it the Stage 1 prompt from `references/prompt_contract.md` with the saved closeout text inserted verbatim.
- Wait for completion.
- Save the final memo to `NEXT_MILESTONE_FILE`.
- Do not rewrite the returned memo into a different format.

### 2. Design doc

- Spawn a fresh agent.
- Give it the Stage 2 prompt from `references/prompt_contract.md` with the Stage 1 memo inserted verbatim.
- Ask it to return the exact design-doc path it created.
- Save the returned summary to `DESIGN_DOC_SUMMARY_FILE`.
- If the path is missing, resolve it with `.agents/skills/planning-session/scripts/resolve_latest_design_doc.sh` and record that fallback in the session log.

### 3. Fresh PLAN

- Spawn a fresh agent.
- Give it the Stage 3 prompt from `references/prompt_contract.md`, pointing at the resolved design-doc path.
- The target is a new or fully refreshed repo-root `PLAN.md`.
- Save its returned summary to `PLAN_PASS1_FILE`.

### 4. PLAN solidification pass

- Spawn a fresh agent.
- Give it the Stage 4 prompt from `references/prompt_contract.md`, pointing at the same design doc and the current repo-root `PLAN.md`.
- The target is the same repo-root `PLAN.md`, tightened into one cohesive execution contract with the normal `plan-eng-review` rigor and the parallelization section preserved.
- Save its returned summary to `PLAN_PASS2_FILE`.

### 5. ORCH plan draft and review loop

- Spawn one fresh agent.
- Before prompting it, read:
  - repo-root `PLAN.md`
  - `docs/m26_orchestration_kickoff_prompt.md`
  - repo-root `ORCH_PLAN.md` if it exists, only as structure history and not as authority
- Give the Stage 5 prompt from `references/prompt_contract.md`.
- Review the returned draft against `references/orch_review_checklist.md`.
- If the draft misses required sections or leaks stale milestone-specific details from older orchestration docs, send one precise correction prompt back to the same agent.
- Repeat at most twice. If the draft is still weak after two correction rounds, stop and report the missing pieces instead of silently accepting it.
- Once the draft passes, write the new repo-root `ORCH_PLAN.md`.

## Parent Review Standard For Stage 5

Reject the draft if any of the following is true:

- it hand-waves with "follow `PLAN.md`" where ownership, commands, or acceptance should be explicit
- it omits hard guards, context-control rules, tests/acceptance, or assumptions
- it copies stale milestone names, branches, or worktree paths from an old example without grounding them in the current `PLAN.md`
- it claims parallel workstreams without a module-ownership split
- it lacks a clear parent-agent critical path

## Completion

At the end, return the concrete outputs:

- session directory
- next milestone memo path
- design doc path
- repo-root `PLAN.md`
- repo-root `ORCH_PLAN.md`

If blocked, report the exact stage and the missing artifact or failed contract.
