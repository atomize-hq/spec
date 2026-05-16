# ORCH Review Checklist

Use this after Stage 5 and before accepting the draft.

## Contract checks

- The summary is grounded in the live branch and current repo-root `PLAN.md`.
- `PLAN.md` is treated as the only authority source.
- Historical orchestration docs are used for shape only, not for milestone facts.

## Structure checks

- Hard guards are explicit and milestone-specific.
- The parent-agent critical path is explicit.
- Parallel lanes are backed by real ownership or dependency splits.
- Context-control rules are present.
- Tests and acceptance gates are concrete.
- Assumptions are stated plainly.

## Rejection checks

Reject and send a correction prompt if:

- stale milestone ids, branch names, or worktree paths leaked in
- the draft says "follow `PLAN.md`" where a runbook section should be explicit
- hard guards, workstream ownership, tests/acceptance, or assumptions are missing
- a parallel lane touches the same module boundary without calling out the conflict
- the draft never names the stop conditions or blocked-state behavior
