<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260502-150826.md -->
# M28 - Shared-Core Boundary Extraction + Escape-Hatch Containment

Status: **implementation contract**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-02**
Supersedes: **M27.9B - money/round Durable-Hold Resolution**
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-150831.md`**
Execution authority: **`/Users/spensermcconnell/__Active_Code/atomize-hq/spec/ORCH_PLAN.md`**

## Summary

M27.9B closed the family-choice lane honestly.

The repo now says:

- `recommendation_status = "no_strong_candidate"`
- `money/round` remains visible under `unsupported_function_surface-e40675da6fa0`
- that surface is durable-held under `helper_surface_not_promotable`
- corpus run `1` remains unspent and unauthorized by default

So M28 is not an evidence milestone.

It is an architecture milestone.

M28 does one bounded thing:

> extract one explicit shared backend-execution boundary inside `spec-core`,
> route the current seam consumers through it, and prove that Rust-specific
> escape hatches are boxed instead of silently mixed into shared semantic truth.

This milestone does **not** add language two.
It does **not** reopen recommendation policy.
It does **not** spend corpus run `1`.
It does **not** rewrite `xtask` unless the audit proves `xtask` is the actual
portability choke point.

That is the whole milestone.

## Done Means

M28 is complete only when all of the following are true together:

1. `spec-core/src/backend_execution.rs` exists as the single shared runtime
   boundary for seam backend-execution markers, marker summaries, and
   backend-execution digests.
2. `spec-core/src/passport.rs` no longer hand-rolls seam backend marker or
   digest scans.
3. `spec-core/src/escape_hatch.rs` no longer classifies Rust lowering markers
   independently of the shared boundary.
4. `spec-core/src/semantic_review.rs` consumes the same shared summary when
   deciding aligned, backend-only-preserved, and backend-only-leaked truth.
5. `spec-core/src/export.rs` and `spec-cli/src/commands.rs` preserve the same
   read-side truth for the same fixtures.
6. the targeted regression loop stays green for the current Rust repo behavior.
7. `cargo xtask family coverage --format json` remains byte-stable against
   `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
   unless recommendation semantics intentionally change, which they must not in
   M28.
8. no recommendation, corpus, or `money/round` governance semantics change.
9. no second-language runtime, lowering, packet, fixture, or scaffold work
   lands.
10. M28 closes with an explicit M29 verdict:
    - proceed to a scoped pilot, or
    - stop and write a kill memo.

## Current Repo Truth

### Locked baseline from M27.9B

- `function_coverage = 28 / 17 / 0 / 11`
- `recommendation_analysis_schema_version = 3`
- `recommendation_status = "no_strong_candidate"`
- visible candidate id:
  `z-unsupportedfunctionsurface-unsupported_function_surface-e40675da6fa0`
- durable hold reason: `helper_surface_not_promotable`
- `next_step_status = "durable_hold"`
- `next_step_detail = "helper_surface_not_promotable"`
- corpus run `1` remains unspent and unauthorized by default

### The actual M28 leak

The portability problem is already in shipped code paths.

Four direct runtime consumers inspect or project Rust-specific execution
surfaces for seam units today:

- `spec-core/src/passport.rs`
  - computes backend-execution digests
  - computes seam marker lists
- `spec-core/src/escape_hatch.rs`
  - classifies `methods.*.lowering.rust.body`
  - distinguishes domain lowering from proof-helper lowering
  - computes required proof surfaces
- `spec-core/src/semantic_review.rs`
  - decides whether executable lowering:
    - agrees with authored semantics
    - preserves meaning but stays backend-only
    - leaks backend-only semantics into the shared semantic surface
- `spec-core/src/export.rs`
  - projects read-side gate and semantic-review truth for downstream consumers

The repo therefore has one conceptual boundary but more than one code owner.

That duplication is the M28 wedge.

## Authority And Evidence

Primary decision inputs:

- `PLAN.md`
- `ORCH_PLAN.md`
- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260502-150831.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `docs/high_level_technical_architecture_v0.2.md`
- `.runs/m27_9b/final-proof.json`
- `.runs/m27_9b/closeout.md`
- `spec-core/src/passport.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/export.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/validator.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/cli.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/report.rs`

If an older note implies that more corpus is the default next move, or that M28
should start with a second-language pilot, this file wins.

## Step 0 - Scope Challenge

### Premise challenge

1. The branch is no longer blocked on next-family ambiguity.
   Verdict: **accept**
2. The highest-leverage M28 leak is duplicated backend-execution logic around
   seam markers, digests, and backend-only semantic classification.
   Verdict: **accept**
3. A docs-only M28 would be fake progress because the leak is already in
   runtime truth surfaces.
   Verdict: **accept**
4. `xtask` should be audit-only unless the audit proves a real Rust-specific
   proof-surface leak.
   Verdict: **accept**
5. M28 must unlock a real M29 decision instead of ending as cleanup-only
   architecture work.
   Verdict: **accept**

### What already exists

| Sub-problem | Existing code / truth | Plan decision |
|---|---|---|
| Shared authored-truth hashing | `spec-core/src/passport.rs` already computes authored-truth digests separately from backend execution | Reuse. Do not redesign freshness. |
| Backend-only execution marker classification | `spec-core/src/escape_hatch.rs` already distinguishes domain lowering, proof-helper lowering, and backend derives | Extract. Make it the single shared source. |
| Supported seam semantic drift classification | `spec-core/src/semantic_review.rs` already projects aligned vs backend-only-preserved vs backend-only-leaked truth | Reuse. Rewire to the extracted boundary module. |
| Seam escape-hatch policy | `spec-core/src/validator.rs` already confines seam escape hatches to `methods[].lowering.rust.body` and `backends.rust.derives` | Treat as authoritative. Do not widen policy. |
| Seam code generation | `spec-core/src/generator.rs` already lowers seams from authored truth plus backend details | Reuse. M28 does not rewrite lowering. |
| CLI status projection | `spec-cli/src/commands.rs` already demotes health from semantic drift and open escape-hatch gates | Reuse. Preserve behavior, tighten data source. |
| Export read-side projection | `spec-core/src/export.rs` already projects passport / semantic-review / gate truth for downstream consumers | Reuse. Preserve parity with `status`, tighten data source. |
| Proof workflow / promotion loop | `xtask` already owns approval, prove, certify, and family recommendation artifacts | Audit only. Do not rewrite unless a real Rust-only proof leak is demonstrated. |
| Coverage reporting | `xtask/src/family/coverage.rs` already summarizes family coverage and recommendation surfaces | Audit only. Freeze no-drift output unless recommendation semantics intentionally change. |

### Minimum honest change

The smallest complete M28 diff is:

1. freeze the portability-consumer inventory so the branch proves this seam is
   the real choke point
2. add one new shared `spec-core` boundary module for backend-execution markers
   and digest surfaces
3. route current seam consumers through it
4. update CLI truth-surface regressions and status/export parity coverage
5. audit `xtask` without rewriting it by habit
6. force a post-M28 M29 go / no-go gate
7. keep `PLAN.md` and `ORCH_PLAN.md` aligned so the docs tell one story

Anything smaller leaves the leak in place.
Anything larger risks turning M28 into a rewrite.

### Closed file contract

Runtime implementation scope is closed to exactly these files:

- `spec-core/src/backend_execution.rs` **new**
- `spec-core/src/passport.rs`
- `spec-core/src/escape_hatch.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/cli.rs`

Planning scope is closed to:

- `PLAN.md`
- `ORCH_PLAN.md`

`xtask/src/family/*` is read-only in M28.
If making M28 green requires an `xtask` code edit, halt and split a bounded
follow-on plan instead of expanding this one.

### Complexity check

Implementation scope is capped at **9 runtime/test files** plus the two active
plan files.

That is at the smell threshold, not beyond it.

It is acceptable because:

- the blast radius stays concentrated in one seam
- no new infrastructure is introduced
- no new command family is introduced
- no new language runtime or artifact type is introduced

### Search and boring-tech rule

- **[Layer 1]** Reuse the seam policy already encoded in `validator.rs`. Do not
  invent a second escape-hatch policy layer.
- **[Layer 1]** Reuse the current health projection in `spec-cli/src/commands.rs`.
  Do not build a second status interpreter.
- **[Layer 1]** Reuse existing M14 regression surfaces instead of inventing a
  new portability-only fixture lane.
- **[EUREKA]** The blocker is not "lack of portability support." The blocker is
  that the repo still computes Rust-specific execution meaning in multiple
  places. Portability work starts by making that one thing explicit.

### TODOS cross-reference

No existing `TODOS.md` item blocks M28 directly.

M28 may create one follow-on TODO only if the `xtask` audit proves a real
Rust-specific proof-surface leak. That follow-on must not be silently expanded
into this milestone.

### Completeness check

The shortcut version of M28 would be "write docs, defer the extraction."

That saves almost nothing with AI assistance and leaves the actual
portability leak in place.

This plan chooses the complete version:

- real shared boundary extraction
- real regression coverage
- real `xtask` audit
- real M29 decision gate

### Distribution check

M28 introduces no new distributed artifact type.

No release pipeline work is required.
No package publishing work is required.
No new install surface is required.

## Alternatives Considered

| Approach | Summary | Effort | Risk | Verdict |
|---|---|---:|---:|---|
| A. Docs-only M28 | Write down the boundary and defer code | S | High | Rejected |
| B. Runtime boundary extraction first | Extract one shared backend-execution module and route current consumers through it | M | Low-Med | **Recommended** |
| C. Full portability-kernel rewrite | Refactor `spec-core`, `spec-cli`, and `xtask` together before any future language work | L | High | Rejected |
| D. Shadow portability probe first | Try one cheap portability probe before extracting the boundary | S-M | Med | Rejected as a standalone milestone, retained as the closeout forcing function |

## Dream State

```text
CURRENT
  Shared semantic intent exists,
  but Rust-specific execution markers are interpreted in multiple places.
        |
        v
THIS PLAN (M28)
  One shared backend-execution boundary
  -> seam consumers reuse it
  -> CLI truth stays aligned
  -> xtask audited, not rewritten by habit
        |
        v
12-MONTH IDEAL
  Shared semantic core is explicit,
  backend adapters are boxed,
  proof workflow is language-portable enough for one honest M29 pilot.
```

## Architecture Review

### Architecture decision

Add exactly one new module:

- `spec-core/src/backend_execution.rs`

This module owns:

- backend-execution marker collection
- marker classification:
  - domain lowering
  - proof-helper lowering
  - backend derives
- shared marker summaries
- backend-execution digest computation for seam units
- shared helper/example identity surface reused by downstream consumers

This module is seam-local.

It is **not** a general portability kernel.
It is **not** a second-language abstraction layer.
It is the one explicit place where Rust-specific backend execution is collected
and summarized before other consumers reason about it.

### Portability-relevant consumer inventory

| Consumer | Why it matters | Decision criticality |
|---|---|---|
| `spec-core/src/passport.rs` | authored/backend digest truth and freshness projection | high |
| `spec-core/src/escape_hatch.rs` | proof-surface containment gate truth | high |
| `spec-core/src/semantic_review.rs` | preserved-vs-leaked backend-only meaning | high |
| `spec-core/src/export.rs` | downstream export truth must stay aligned with `status` truth | high |
| `spec-cli/src/commands.rs` | user-visible health/status projection | high |
| `xtask/src/family/coverage.rs` | frozen coverage/recommendation surface that can reveal semantic drift even when runtime code stays green | medium, audit-only |
| `xtask/src/family/report.rs` | proof/report wording can expose Rust-only assumptions | medium, audit-only |

If the extraction does not simplify every high-criticality consumer above,
halt before merge and re-scope.

### Dependency graph

```text
AUTHORED SPEC
   |
   +--> validator.rs
   |      `- freezes where escape hatches are allowed
   |
   +--> generator.rs
   |      `- lowers shared authored truth + backend details
   |
   `--> backend_execution.rs   [NEW SHARED BOUNDARY]
           |
           +--> passport.rs
           |      `- authored/backend digests, markers
           |
           +--> escape_hatch.rs
           |      `- proof-surface gate
           |
           +--> semantic_review.rs
           |      `- aligned / preserved / leaked truth
           |
           +--> export.rs
           |      `- read-side bundle projection
           |
           `--> spec-cli/src/commands.rs
                  `- final status / health projection
```

### Runtime data flow

```text
seam unit
  -> validator confirms escape-hatch policy
  -> backend_execution collects and classifies backend markers
  -> passport records authored digest + backend digest
  -> escape_hatch computes required proof surfaces
  -> semantic_review decides aligned / preserved / leaked
  -> export projects read-side bundle truth
  -> spec-cli projects final status truth
```

### Exact file responsibilities

| File | Responsibility | Must not happen |
|---|---|---|
| `spec-core/src/backend_execution.rs` | Single shared source for seam backend-execution markers, summaries, helper/example identity, and backend digests | Do not bake in second-language abstractions or new runtime routing policy. |
| `spec-core/src/passport.rs` | Reuse shared boundary for backend markers and digests | Do not change authored-truth hashing semantics. |
| `spec-core/src/escape_hatch.rs` | Reuse shared boundary for marker summaries and gate logic | Do not loosen required proof surfaces. |
| `spec-core/src/semantic_review.rs` | Reuse shared boundary for preserved-vs-leaked decisions | Do not change supported family routing or recommendation policy. |
| `spec-core/src/lib.rs` | Export the new boundary module | Do not become a grab-bag for unrelated refactors. |
| `spec-core/src/export.rs` | Preserve truthful export projection against the refactored boundary | Do not let export/status truth drift apart. |
| `spec-cli/src/commands.rs` | Preserve truthful health projection against the refactored boundary | Do not rewrite unrelated status rules. |
| `spec-cli/tests/m14_regressions.rs` | Lock regressions around contained vs leaked backend execution | Do not weaken current M14 truth surfaces. |
| `spec-cli/tests/cli.rs` | Lock CLI read-side truth after the refactor | Do not add unrelated fixture churn. |

### Production failure scenario per codepath

| Codepath | Realistic failure | Accounted for in this plan? |
|---|---|---|
| `backend_execution.rs` marker classification | helper-only lowering gets misclassified as domain lowering and silently demotes healthy seams | Yes, targeted regression coverage required |
| `passport.rs` digest reuse | authored-truth and backend-execution digests collapse back together and stale/fresh truth lies | Yes, digest-specific regression required |
| `escape_hatch.rs` summary reuse | open gate closes incorrectly because marker summary loses proof-helper distinction | Yes, current-surface and marker regressions required |
| `semantic_review.rs` shared summary | backend-only preserved seams get reported as drift, or leaked semantics get treated as preserved | Yes, aligned/preserved/leaked regressions required |
| `export.rs` read-side reuse | export bundle truth drifts from `status` truth for the same fixture | Yes, status/export parity regressions required |
| `spec-cli/src/commands.rs` health projection | read-side health changes without an implementation truth change | Yes, CLI JSON/text regressions required |

## Code Quality Review

### Primary DRY issue this milestone fixes

The repo currently has one conceptual boundary but more than one code owner:

- backend-only marker collection lives in more than one consumer
- backend-only digest truth is computed separately from marker truth
- helper/example identity can be re-derived differently by different consumers
- the meaning of "backend-only preserved" vs "backend-only leaked" can drift

That is the core architectural duplication.

M28 removes it by extracting one obvious module instead of adding a clever
framework.

### Quality bar

- prefer free functions and plain structs over trait-heavy indirection
- keep naming literal and boring:
  - `BackendExecutionMarker`
  - `BackendExecutionMarkerKind`
  - `BackendExecutionSummary`
  - `compute_backend_execution_digest`
- keep helper/example identity as an explicit exported API, not an implicit
  re-derivation hidden inside `escape_hatch` or `semantic_review`
- keep the boundary module seam-local, not a fake universal portability layer
- avoid unrelated cleanup in touched files unless it directly clarifies the new
  boundary

### Inline diagram rule

If `backend_execution.rs` lands with more than one classification path, add a
small inline ASCII comment in that file showing:

```text
method lowering / derives
  -> collect markers
  -> classify marker kind
  -> summarize
  -> digest / review / gate consumers
```

## Test Review

### Code path coverage target

```text
CODE PATH COVERAGE TARGET
=========================
[+] spec-core/src/backend_execution.rs
    │
    ├── collect_backend_execution_markers()
    │   ├── [REQUIRED] non-seam units produce no markers
    │   ├── [REQUIRED] backend derives only
    │   ├── [REQUIRED] proof-helper lowering only
    │   └── [REQUIRED] domain lowering only
    │
    ├── summarize_backend_execution_markers()
    │   ├── [REQUIRED] domain + helper summary stay distinct
    │   └── [REQUIRED] derive marker remains visible
    │
    └── compute_backend_execution_digest()
        ├── [REQUIRED] seam digest present when lowering/derives exist
        ├── [REQUIRED] digest absent for seam units without backend execution
        ├── [REQUIRED] backend-only lowering change flips backend freshness only
        └── [REQUIRED] authored-only seam change leaves backend freshness alone

[+] Runtime consumer coverage
    │
    ├── passport.rs
    │   ├── [REQUIRED] authored digest unchanged
    │   ├── [REQUIRED] backend digest semantics unchanged
    │   ├── [REQUIRED] authored-only seam edits do not change backend freshness
    │   └── [REQUIRED] backend-only seam edits do not change authored freshness
    │
    ├── escape_hatch.rs
    │   ├── [REQUIRED] helper-only marker keeps gate semantics
    │   └── [REQUIRED] domain lowering still marks domain execution
    │
    ├── semantic_review.rs
    │   ├── [REQUIRED] preserved backend-only meaning stays preserved
    │   ├── [REQUIRED] leaked backend-only meaning stays failing
    │   └── [REQUIRED] aligned supported seams stay aligned
    │
    ├── export.rs
    │   ├── [REQUIRED] export gate truth matches status truth
    │   └── [REQUIRED] export semantic-review truth matches status truth
    │
    ├── spec-cli/src/commands.rs
    │   ├── [REQUIRED] CLI health still demotes open escape-hatch gates
    │   ├── [REQUIRED] CLI health still reports semantic drift correctly
    │   └── [REQUIRED] status/read-side wording remains stable where intended
    │
    └── xtask/src/family/coverage.rs   [AUDIT-ONLY]
        └── [REQUIRED] frozen no-drift coverage JSON under unchanged recommendation semantics
```

### Operator-flow coverage target

```text
OPERATOR FLOW COVERAGE TARGET
=============================
[+] Maintainer changes seam lowering
    ├── [REQUIRED] stale/fresh projection remains truthful
    └── [REQUIRED] preserved-vs-leaked review remains truthful

[+] Maintainer runs status/export/read-side commands
    ├── [REQUIRED] open escape-hatch gate remains incomplete, not valid
    ├── [REQUIRED] semantic drift remains failing, not preserved
    └── [REQUIRED] export and status agree on the same fixture truth

[+] Maintainer audits portability readiness for M29
    ├── [REQUIRED] one shared runtime boundary exists to inspect
    └── [REQUIRED] coverage output remains frozen under unchanged semantics
```

### Required test additions

1. `spec-core` unit tests for the new boundary module:
   - non-seam units produce no markers
   - helper-only lowering stays helper-only
   - domain lowering stays domain
   - backend derives remain visible independently
2. `spec-core` passport regressions:
   - authored digest unchanged by extraction
   - backend digest semantics unchanged
   - authored-only seam edit does not change backend freshness
   - backend-only seam edit changes backend freshness without changing authored freshness
3. `spec-core` escape-hatch regressions:
   - proof-helper marker does not silently become domain lowering
   - required proof surfaces stay `atom` + `molecule`
4. `spec-core` semantic-review regressions:
   - aligned supported seams stay aligned
   - backend-only preserved meaning stays preserved
   - backend-only leaked meaning stays failing
5. `spec-core` export regressions:
   - export gate truth matches `status` truth for the same fixture
   - export semantic-review truth matches `status` truth for the same fixture
6. `spec-cli/tests/m14_regressions.rs`:
   - leaked backend-only semantics still fail
   - preserved backend-only meaning remains additive-only where expected
7. `spec-cli/tests/cli.rs`:
   - CLI JSON/text health reasons remain truthful after extraction
8. frozen audit regression:
   - `cargo xtask family coverage --format json` matches
     `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`

### Regression rule

This milestone touches existing runtime behavior.

These are **critical regressions**, not optional improvements:

- backend-only preserved vs leaked classification
- escape-hatch gate open/closed read-side projection
- passport authored/backend digest distinction
- status/export parity for the same fixture
- frozen family coverage under unchanged recommendation semantics

### Test plan artifact

QA-oriented artifact for this plan:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-eng-review-test-plan-20260502-151200.md`

## Performance And Operational Review

### Performance

This milestone must preserve an **O(method count)** scan over seam units.

Do not introduce:

- global caches
- repeated serialization inside inner loops
- cross-command memoization
- extra repo scans from `spec-cli`

The new module may compute summaries and digests, but every consumer should
reuse one shared extraction path instead of each re-scanning the unit
differently.

### Operational quality

- no new command family
- no new build flag
- no new generated artifact class
- no new release workflow
- no new fixture family outside targeted regressions

### Read-only `xtask` audit

M28 includes an explicit audit of current `xtask` proof, report, and coverage
surfaces only.

If the audit proves a real Rust-specific semantic leak in:

- `xtask/src/family/report.rs`
- `xtask/src/family/coverage.rs`
- prove/certify artifact wording
- promotion artifact schemas

then halt M28 and split a follow-on M28.x plan.

Do not expand this milestone opportunistically.

## Failure Modes Registry

| Codepath | Realistic failure | Test required? | Error handling exists? | Silent if missed? | Critical gap? |
|---|---|---|---|---|---|
| `backend_execution.rs` helper/domain classification | helper-only lowering treated as domain lowering | yes | no runtime fallback | yes | **yes** |
| `passport.rs` digest reuse | stale/fresh projection lies after extraction | yes | no | yes | **yes** |
| `escape_hatch.rs` summary reuse | required proof surfaces close incorrectly | yes | partial read-side demotion only | yes | **yes** |
| `semantic_review.rs` shared summary | preserved backend-only meaning becomes failing drift, or leaked meaning becomes preserved | yes | yes, but only if correctly classified | yes | **yes** |
| `export.rs` read-side reuse | export truth diverges from `status` truth for the same fixture | yes | no | yes | **yes** |
| `spec-cli/src/commands.rs` health projection | CLI JSON/text status silently regresses | yes | no | yes | **yes** |

## NOT in scope

- spending corpus run `1`
  reason: M27.9B already closed the evidence lane for now
- changing recommendation policy or recommendation artifacts
  reason: M28 is not a family-choice milestone
- changing supported function routing order
  reason: M28 is not a routing milestone
- rewriting `generator.rs` lowering semantics
  reason: the goal is boundary extraction, not lowering redesign
- adding a second target language
  reason: that is M29 at the earliest
- preemptively rewriting `xtask`
  reason: audit-only unless a real proof-surface leak is proven
- broad docs cleanup outside `PLAN.md` and `ORCH_PLAN.md`
  reason: keep the blast radius bounded

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| A. Boundary module + passport rewire | `spec-core/src/` | — |
| B. Escape-hatch + semantic-review rewire | `spec-core/src/` | A |
| C. Export/status parity + CLI regressions | `spec-core/src/`, `spec-cli/src/`, `spec-cli/tests/` | A, B |
| D. Plan / orchestration docs | repo root plan files only | — |
| E. `xtask` audit | `xtask/src/family/` read-only | A, B |

### Parallel lanes

- **Lane A:** A -> B -> C
  sequential, shared `spec-core/src/` ownership
- **Lane B:** D
  independent, docs only
- **Lane C:** E
  independent read-only audit, but blocked from code mutation

### Execution order

1. Launch Lane A and Lane B in parallel worktrees.
2. Launch Lane C in parallel as a read-only audit.
3. Merge Lane B at any time.
4. Merge Lane A only after the targeted proof loop is green.
5. Process Lane C before final closeout:
   - if `no_leak_found`, record the read-only result
   - if `leak_found_follow_on_required`, halt and split a follow-on

### Conflict flags

- Steps A and B both touch `spec-core/src/` and are therefore intentionally
  sequential
- Lane C must remain read-only; if it requires edits in `xtask/src/family/`,
  that is a scope break, not a merge conflict to improvise through
- Lane B must not back-edit runtime scope after Lane A merges; doc drift is a
  parent-integration problem, not a worker freedom

## Implementation Plan

### Step 0 - Freeze the consumer inventory

Before the first runtime edit:

1. enumerate every portability-relevant consumer
2. confirm that every high-criticality consumer is covered by this extraction
3. record any uncovered high-criticality consumer as a blocker, not as a TODO
4. confirm the closed file contract still matches `ORCH_PLAN.md`

Exit gate:

- the consumer inventory in this file still matches the actual planned diff

### Step 1 - Create the shared boundary module

Add `spec-core/src/backend_execution.rs` with:

- marker kind enum
- marker struct
- summary struct
- shared helper/example identity surface
- marker collection function
- summary function
- backend-execution digest function

Invariants locked here:

- authored-only seam edits do **not** change backend-execution freshness
- backend-only lowering / derives edits do **not** change authored freshness

Exit gate:

- the new module is exported from `spec-core/src/lib.rs`
- no second consumer still needs to invent marker classification from scratch

### Step 2 - Rewire `passport` and `escape_hatch`

Update:

- `spec-core/src/passport.rs`
- `spec-core/src/escape_hatch.rs`

Required outcome:

- backend-execution digest truth comes from the shared boundary
- seam marker truth comes from the shared boundary
- helper vs domain lowering distinction is preserved exactly
- required proof surfaces remain `atom` + `molecule`

Exit gate:

- no independent backend marker scan remains in either file

### Step 3 - Rewire `semantic_review`, `export`, and `status`

Update:

- `spec-core/src/semantic_review.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`

Required outcome:

- aligned / preserved / leaked decisions consume the shared summary
- export truth and status truth stay aligned for the same fixture
- open escape-hatch gates still demote to `incomplete`
- semantic drift still demotes to `failing`
- no wording drift is introduced unless explicitly intended and tested

Exit gate:

- one fixture can no longer show different truth in `status` vs `export`

### Step 4 - Land targeted regressions

Add or update tests in:

- `spec-core` unit tests for the new boundary module
- `spec-cli/tests/m14_regressions.rs`
- `spec-cli/tests/cli.rs`

Required outcome:

- all critical regressions listed in the test review are covered
- helper-only vs domain-lowering classification stays stable
- passport freshness invariants stay stable
- status/export parity is locked

Exit gate:

- the required proof loop is green

### Step 5 - Run the `xtask` audit

Audit prove, certify, report, and coverage surfaces for real Rust-only
semantic leakage.

Allowed dispositions:

1. `no_leak_found`
   - `xtask` remains read-only for M28
2. `leak_found_follow_on_required`
   - halt this milestone
   - write the finding into run-state / closeout notes
   - split a bounded follow-on plan

Disallowed disposition:

- "small enough to fix now"

Exit gate:

- the audit result is explicit and recorded

### Step 6 - Keep `PLAN.md` and `ORCH_PLAN.md` aligned

The plan and orchestration docs must tell the same story on:

- closed file contract
- worktree lanes
- proof loop
- halt conditions
- final acceptance gate

Exit gate:

- no stale M27.9B or pre-rewrite M28 residue remains in either file

### Step 7 - Force the M29 decision

M28 does not close with "refactor complete."

It closes with exactly one of these two outcomes:

1. **Proceed to M29**
   - all high-criticality runtime consumers now share the same boundary path
   - no `xtask` proof leak blocks the pilot
   - the exact closeout probe passes
2. **Kill memo**
   - a larger portability blocker still dominates, or
   - `xtask` / proof surfaces remain the real choke point, or
   - the exact closeout probe fails

No third option.

### Exact M29 closeout probe

Use the existing helper-only and domain-lowering seam fixtures already covered
by the targeted regressions.

Run exactly this command path:

```bash
cargo test -p spec-core --lib -- --color never
cargo test -p spec-cli --test m14_regressions -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo xtask family coverage --format json >/tmp/m28.coverage.actual.json
diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m28.coverage.actual.json
```

Pass:

- helper-only fixture remains helper-only / additive-only where expected
- domain-lowering fixture still reports the same failing or gated truth where expected
- `status` and `export` agree on both fixtures
- coverage JSON stays byte-stable against the frozen baseline

Fail:

- any helper-only fixture becomes domain-lowering
- any domain-lowering fixture becomes silently preserved
- `status` and `export` disagree on the same fixture
- coverage JSON drifts without an intentional recommendation-policy change

## Proof Loop

### Required proof loop

```bash
cargo test -p spec-core --lib -- --color never
cargo test -p spec-cli --test m14_regressions -- --color never
cargo test -p spec-cli --test cli -- --color never
cargo xtask family coverage --format json >/tmp/m28.coverage.actual.json
diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m28.coverage.actual.json
```

### Optional full confirmation

Run this only if the diff expands within `spec-cli` beyond the targeted
read-side surfaces:

```bash
cargo test -p spec-cli -- --color never
```

### Halt rule

If any `xtask/src/family/*` file must change to make M28 green, stop and split a
follow-on plan instead of broadening this milestone.

## Acceptance Criteria

The milestone is accepted only if all of these are true together:

1. a single shared runtime boundary module exists for seam backend execution and
   helper/example identity
2. the four direct runtime consumers reuse it and `spec-cli/src/commands.rs`
   stays aligned with that shared path
3. current Rust status/export truth stays green under targeted regressions
4. backend-execution freshness invariants are proven
5. no recommendation/corpus semantics changed
6. no second-language runtime landed
7. `xtask` either:
   - stayed read-only, or
   - triggered a documented halt and follow-on split
8. frozen coverage JSON stayed byte-stable under unchanged recommendation semantics
9. the closeout explicitly says whether M29 should proceed

## Halt Conditions

Halt immediately if any of these happen:

- `xtask` edit becomes necessary
- validator policy widening appears necessary
- recommendation/corpus semantics drift
- runtime proof loop requires changes outside the closed file contract
- second-language work starts sneaking in through fixtures or packet scaffolds
- the milestone cannot produce a credible M29 go / no-go decision

## Completion Summary

- Step 0: Scope Challenge, accepted as written
- Architecture Review: shared-boundary design locked with one explicit module
- Code Quality Review: central DRY seam named and bounded
- Test Review: coverage targets and critical regressions enumerated
- Performance Review: one runtime constraint locked, no new infra
- NOT in scope: written
- What already exists: written
- `xtask` audit: explicitly bounded to read-only or halt
- Parallelization: 3 lanes, 2 parallel + 1 read-only audit lane
- Final gate: explicit M29 proceed / kill decision required
- Lake Score: this is the complete version, not the docs-only shortcut

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Move to M28 now instead of another corpus run | Mechanical | P1, P6 | The branch already has an honest `no_strong_candidate` result and a durable hold; more evidence is no longer the default blocker. | another corpus milestone |
| 2 | CEO | Keep M28 as one milestone with serial runtime phases | Taste | P1, P3 | One milestone keeps the branch story coherent while still forcing explicit gates and a real closeout decision. | splitting immediately into M28A/M28B |
| 3 | CEO | Force M28 to end with an M29 go / no-go decision | Mechanical | P1, P6 | Internal cleanup without a roadmap decision would be elegant drift, not progress. | cleanup-only closeout |
| 4 | Eng | Add one new shared `spec-core` boundary module | Mechanical | P5 | Four direct runtime consumers already prove the seam is shared; one explicit module is clearer than hidden helpers. | burying helpers in one existing file |
| 5 | Eng | Keep `xtask` audit-only unless a real leak is proven | Mechanical | P2, P4 | Rewriting proof workflow by habit would spend scope on a problem not yet demonstrated. | preemptive `xtask` rewrite |
| 6 | Eng | Cap runtime implementation scope at 9 files | Mechanical | P3, P5 | Architectural cleanup only pays off if the blast radius stays bounded. | full portability-kernel rewrite |
