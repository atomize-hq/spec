# M32 - One Bounded Second-Language Promotion Path

Status: **authoritative implementation plan**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-04**
Supersedes: **M31 - Shared-Core Extraction And Escape-Hatch Containment**
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260504-143928.md`**
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**
Execution note: **Do not create `ORCH_PLAN.md` up front. Create it only if the post-foundation lanes below are actually split into separate worktrees.**
Foundation precondition: **Start from `ws/m31-int` at `945284ea7ab6bf788d7202ff674b81581afd47c6` or a merged equivalent before doing any M32 implementation work.**

## Objective

Make the repository able to say one precise, enforceable thing:

> one existing promoted family can complete the full promotion proof path in a
> second-language lane, and the repo can describe that result honestly on the
> same public truth surfaces it already asks users to trust.

This is the full M32 claim.

M32 is not broad TypeScript support.

M32 is one bounded proof:

- one already-known function family
- one second-language lane
- one set of promotion artifacts
- one set of read-side truth surfaces
- one explicit closeout of what remained shared versus target-specific

## Decision

M32 ships as a single-family pilot centered on
`function.arithmetic_leaf.monotone_up.v1`.

That means:

1. `function.arithmetic_leaf.monotone_up.v1` is the only family that must pass
   the full M32 pilot contract.
2. `function.wrapper.pipeline.v1` stays as regression pressure only. Its
   existing suites must stay green, but it is not a second certification target
   for this milestone.
3. Acceptance requires both:
   - the existing Rust-default `prove` and `certify` path staying green
   - the bounded TypeScript `prove` and `certify` path going green on the same
     family
4. M32 reuses the M31 portability boundary. It does not reopen seam portability
   semantics or invent cross-language truth for `kind:data`, `kind:sum`, or
   `.test.spec`.

## Problem Statement

The repo already has important pieces of the second-language story:

- `xtask` exposes `--target-language` on `family prove` and `family certify`
- `xtask/src/family/prove.rs` already allows `typescript` for
  `function.arithmetic_leaf.monotone_up.v1` and `function.wrapper.pipeline.v1`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/` already carries
  additive `body.typescript` fixtures
- `spec-core/src/semantic_review.rs` already reads authored `body.typescript`
  when building semantic-review truth
- `spec-core/src/passport.rs`, `spec-core/src/export.rs`, and
  `spec-cli/src/commands.rs` already project public read-side truth

Good. That means the repo is not starting from zero.

But the path is still incomplete and partly misleading:

- promotion execution artifacts still hard-code `target_language = rust` in
  `xtask/src/family/promotion_artifacts.rs`
- there is no frozen parent-usable runtime command surface yet for emitting
  `promotion.execution.json` or `blocker.report.json`
- certification reports do not record which target-language lane produced the
  artifact
- the current promotion-artifact chain can still point at a stale
  `recommendation.latest.json` that does not describe the monotone-up pilot
- the plan does not currently lock whether Rust-default proof must remain green
  while TypeScript proof is added
- there is no single milestone contract that says which read-side surfaces must
  agree on the same second-language pilot result
- the current branch still points at pre-closeout M31 planning rather than an
  M32 authority document

That is exactly the kind of half-true state that produces fake confidence.

## Locked Decisions

These decisions remove the remaining ambiguity. They are part of the milestone
contract, not suggestions.

### 1. M31 is a hard prerequisite

No M32 implementation begins on stale pre-M31 state.

Start from:

- `ws/m31-int` at `945284ea7ab6bf788d7202ff674b81581afd47c6`, or
- a merged equivalent that already contains the M31 portability contract

If that precondition is not met, stop and re-anchor before doing anything else.

### 2. One primary family only

`function.arithmetic_leaf.monotone_up.v1` is the only primary M32 packet.

Do not add:

- a new third family
- a second primary pilot family
- a cross-family promotion milestone

### 3. Rust-default and TypeScript lanes are both required

M32 is only complete when the monotone-up family passes:

- `cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1`
- `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1`
- `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1`
- `cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
- `cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript`

This is the complete version and the cost delta is tiny compared to the value
of proving the second-language lane did not quietly regress the original one.

### 4. Wrapper pipeline is comparator pressure, not a second pilot

`function.wrapper.pipeline.v1` remains a required regression surface in tests.

It is not required to pass a second M32 certify lane. Its job is to catch
shared-kernel regressions, not to widen the milestone.

### 5. Public truth must stay on existing surfaces first

Default stance: reuse the repo's existing public truth surfaces and make them
target-language-aware where needed:

- promotion execution artifacts
- certification reports
- passport
- `spec status`
- `spec export`
- semantic-review summaries and citations

Add a new public surface only if a concrete truth gap remains after the reused
surfaces are made honest.

### 6. Artifact truth must stop pretending everything is Rust

If an artifact participates in M32 closeout, it must record the actual
target-language lane.

At minimum, the M32 implementation must make target language explicit in:

- prove/certify reports
- the monotone-up promotion recommendation artifact used by closeout
- promotion execution artifacts
- artifact validation rules

### 7. Read-side truth is part of acceptance, not follow-up polish

The same pilot path must leave honest truth on:

- passport output
- `spec status`
- `spec export`
- semantic-review summaries and citations

If the commands go green but the read-side story is still vague or stale, M32
is not done.

### 8. Scope stays function-only

M32 does not widen support for:

- `kind:data`
- `kind:sum`
- `.test.spec` target-language execution
- general target-language lowering policy outside the chosen function family

### 9. The closeout must distinguish shared versus target-specific residue

The repo must exit M32 able to state, with evidence:

- what remained genuinely shared
- what stayed target-specific
- whether the M31 containment boundary held under the pilot

That summary belongs in repo truth, not hidden maintainer context.

## Done Means

M32 is complete only when all of the following are true:

1. the branch or integration target already includes the validated M31 boundary
2. `function.arithmetic_leaf.monotone_up.v1` passes `smoke`, Rust `prove`,
   Rust `certify`, TypeScript `prove`, and TypeScript `certify`
3. prove/certify artifacts and validation logic no longer hard-code
   `target_language = rust` for M32-relevant outputs
4. the monotone-up pilot still keeps existing wrapper-pipeline regression suites
   green
5. public read-side truth surfaces stay honest for the same pilot path:
   passport, status, export, semantic-review summaries
6. the repo can explain what stayed shared versus what remained
   target-specific without relying on chat-only interpretation
7. no new family, no new broad TypeScript support claim, and no seam-kind
   widening was needed to land the milestone
8. the roadmap and this plan describe the same `M31 -> M32` sequence
9. the tests prove the second-language lane rather than only compiling through
   the flag plumbing
10. the plan's parallelization and failure-mode sections still match the actual
    landed implementation shape

## NOT in Scope

The following work was considered and is explicitly deferred:

- Broad repo-wide "TypeScript is supported" messaging
  Reason: M32 proves one bounded family path only.
- A new third function family
  Reason: that changes both the package and the belt at the same time.
- `kind:data` or `kind:sum` second-language execution semantics
  Reason: that would reopen M31 and widen the ontology too early.
- `.test.spec` target-language execution
  Reason: molecule tests remain Rust-only in current validator policy.
- A replacement for `xtask` proof commands
  Reason: `smoke`, `prove`, and `certify` are already the deterministic kernel.
- A new standalone public CLI command for target-language closeout
  Reason: existing read-side and artifact surfaces should carry the truth first.
- Recommendation-engine target-language expansion
  Reason: M27/M27.5 artifacts are still intentionally Rust-scoped today.
- A second certify lane for `function.wrapper.pipeline.v1`
  Reason: useful pressure, wrong milestone.

## What Already Exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| CLI target-language flag | `xtask/src/lib.rs` defines `FamilyTargetLanguage::{Rust, Typescript}` and threads it into `family prove` and `family certify` | Reuse the existing flag surface. Do not invent a second CLI entrypoint. |
| Bounded TypeScript gate admission | `xtask/src/family/prove.rs` already allows `typescript` only for `function.arithmetic_leaf.monotone_up.v1` and `function.wrapper.pipeline.v1` | Reuse the existing admission rule and tighten the proof/reporting around it. |
| Certify flow reuse of prove | `xtask/src/family/certify.rs` already runs `prove::execute_in(...)` and layers gate D routing checks on top | Reuse the same kernel. Do not fork a second certify implementation for TypeScript. |
| Locked monotone-up harness | `xtask/src/family/harness.rs` already defines monotone-up smoke contracts, prove suites, certify suites, routing precedence, and regression suite names | Reuse as the pilot harness surface. Extend only where M32 truth requires it. |
| Committed pilot packet | `semantic-families/function.arithmetic_leaf.monotone_up.v1/` already exists with additive `body.typescript` starter fixtures in all four buckets | Reuse the packet instead of creating a fresh family. |
| Semantic-review authored TypeScript visibility | `spec-core/src/semantic_review.rs` already reads authored `body.typescript` and cites it in monotone-up and wrapper-family tests | Reuse this as the read-side truth foundation, not as proof that the full lane is already done. |
| Passport truth surface | `spec-core/src/passport.rs` already projects freshness, markers, proof state, and semantic review into the public passport | Reuse the passport path as one of the required M32 honesty surfaces. |
| Export truth surface | `spec-core/src/export.rs` already enriches passports for export and projects current semantic truth | Reuse and keep it aligned with passport/status. |
| Status truth surface | `spec-cli/src/commands.rs` already emits structured `spec status` health including freshness and semantic review | Reuse as the live health/read-side surface. |
| Monotone-up regression fixtures | `spec-cli/tests/m14_regressions.rs` already contains monotone-up truth-surface and corpus regressions with additive TypeScript bodies | Reuse as the main spec-cli regression bed rather than creating a new test fixture universe. |
| Promotion artifact framework | `xtask/src/family/promotion_artifacts.rs` already owns execution and blocker artifact schemas | Reuse, but extend it so M32 artifacts can tell the truth about target language instead of forcing `rust`. |

## Step 0 - Scope Challenge

This milestone touches more than 8 files. Normally that is a smell.

Here it is justified because the gap is cross-surface truth:

- the proof kernel already accepts the flag
- the committed family packet already contains additive TypeScript bodies
- the read-side surfaces already expose semantic truth
- the artifact layer still says "rust"

Reducing below that surface would leave one of these stories false:

- the proof commands
- the proof artifacts
- the read-side truth surfaces
- the public roadmap

The minimum honest implementation surface is:

- the existing `xtask` prove/certify/report/artifact path
- the existing monotone-up packet and harness
- the existing read-side truth surfaces
- the roadmap and this plan

Anything smaller is a partial patch that still leaves the repo flattering
itself.

## Closed Implementation Surface

### Primary modules

- `xtask/src/lib.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/family/report.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/harness.rs`
- `spec-core/src/semantic_review.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`
- `semantic-families/README.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`
- `PLAN.md`

### Allowed mechanical spillover

Only if compile- or fixture-forced:

- `xtask/src/family/paths.rs`
- `xtask/src/family/routing.rs`
- `spec-core/src/types.rs`
- `spec-core/src/lib.rs`
- `spec-core/src/validator.rs`

If implementation needs broader semantics outside this surface, stop and rewrite
the plan before continuing.

## Architecture

### Current shape

```text
committed family packet
        |
        +--> family smoke
        |
        +--> family prove/certify
              |
              +--> accepts --target-language typescript for 2 families
              |
              `--> writes reports/artifacts that still assume rust-first truth

spec-core / spec-cli read-side surfaces
passport -> export -> status -> semantic review summaries
        |
        `--> already expose public truth, but not yet locked to one M32 pilot contract
```

### Target shape

```text
M31-integrated repo base
        |
        v
monotone-up committed packet + locked harness
        |
        +--> smoke contract stays stable
        |
        +--> Rust prove/certify lane stays green
        |
        `--> TypeScript prove/certify lane goes green
                |
                +--> target-language-aware report + execution artifacts
                +--> explicit shared-vs-target-specific closeout notes
                |
                +-----------+--------------+--------------+
                            v              v              v
                        passport         spec export    spec status
                            \              |              /
                             \             |             /
                              `------ semantic review --'
```

The important change is not just "TypeScript commands pass."

The important change is that the same bounded pilot can be read honestly from
both the proof artifacts and the public truth surfaces.

## Pilot Contract

M32 owns one bounded second-language contract.

### Primary pilot

- family: `function.arithmetic_leaf.monotone_up.v1`
- target language: `typescript`
- baseline lane: `rust`
- smoke lane: scaffold contract only, no target-language flag

### Comparator pressure

- keep wrapper-pipeline suites green in `spec-core` and `spec-cli`
- do not add a second full certify requirement for wrapper-pipeline

### Artifact contract

The prove/certify path must emit target-language-aware machine truth.

The exact Rust type names may differ, but the ownership boundary cannot:

- `CertificationReport` must record the target-language lane
- the promotion chain must refresh a monotone-up recommendation artifact before
  emitting closeout artifacts
- promotion execution artifacts must record the target language
- artifact validation must accept the new truthful shape
- closeout notes must distinguish shared semantics from target-specific residue

## Read-Side Truth Rules

These rules are locked.

### Packet and harness truth

- `family smoke` remains the scaffold contract for the committed monotone-up
  packet
- additive `body.typescript` stays part of the committed packet truth
- the TypeScript lane must not require packet-local cheats hidden outside the
  packet, harness, or deterministic proof kernel

### Rust baseline truth

- the existing Rust-default monotone-up `prove` and `certify` path must stay
  green
- M32 is not allowed to break the original lane in order to make the
  second-language lane look green

### Semantic-review truth

- `spec-core/src/semantic_review.rs` must continue to cite authored
  `body.typescript` honestly
- existing supported-function and unsupported-function verdict vocabulary stays
  unchanged unless a compile-local fix is forced

### Passport, export, and status truth

- the same monotone-up pilot path must project coherently through passport,
  export, and status
- these surfaces must not imply broad repo-wide TypeScript execution support
- if a lane stays target-specific or requires bounded exceptions, that truth
  must remain visible rather than smoothed over

## Implementation Plan

### Step 1 - Re-anchor on the validated M31 base and freeze the M32 pilot contract

Goal: start from a truthful baseline and remove branch-history ambiguity.

Files:

- `PLAN.md`
- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`

Required work:

- confirm the working implementation base already includes `ws/m31-int` or
  merge it first
- replace the stale M31 authority plan with this M32 authority plan
- lock the primary pilot family, acceptance commands, comparator policy, and
  stop conditions before touching code

Exit condition:

- there is one unambiguous M32 authority document and it matches the design doc

### Step 2 - Make prove/certify/report/artifact truth target-language-aware

Goal: stop the proof artifact layer from pretending the pilot is Rust-only.

Files:

- `xtask/src/lib.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/family/report.rs`
- `xtask/src/family/promotion_artifacts.rs`

Required work:

- preserve the current CLI target-language flag shape
- thread the chosen target language into prove/certify reporting
- update report and promotion-artifact schemas so M32-relevant outputs can say
  `typescript` truthfully
- update artifact validation to accept the new truthful schema
- keep Rust-default behavior unchanged when the flag is omitted

Exit condition:

- the prove/certify path can produce machine-readable artifacts that explicitly
  identify the Rust lane versus the TypeScript lane

### Step 3 - Lock the monotone-up pilot packet and harness around the M32 contract

Goal: make the chosen family the authoritative bounded pilot, not just a loose
  example.

Files:

- `xtask/src/family/harness.rs`
- `semantic-families/function.arithmetic_leaf.monotone_up.v1/**`

Required work:

- keep the committed monotone-up scaffold truth aligned with `family smoke`
- ensure the prove/certify suites named in the harness still describe the pilot
  accurately once target-language reporting becomes explicit
- keep the packet-local TypeScript bodies additive and truthful
- leave public packet wording to the final docs-closeout lane so packet docs
  describe the landed pilot rather than an intermediate assumption

Exit condition:

- the committed packet, harness, and smoke contract still agree on the same
  monotone-up pilot story

### Step 4 - Re-prove the read-side truth surfaces against the same pilot

Goal: make public repo truth match the proof artifact story.

Files:

- `spec-core/src/semantic_review.rs`
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/m14_regressions.rs`

Required work:

- preserve monotone-up authored TypeScript visibility in semantic review
- prove that passport, export, and status stay aligned for the pilot fixtures
- verify that the repo can surface target-specific residue honestly instead of
  implying broad support
- keep wrapper-pipeline regression surfaces green while doing this work

Exit condition:

- the read-side public surfaces tell the same bounded M32 story as the proof
  artifacts

### Step 5 - Close the public wording and milestone sequencing

Goal: make the roadmap say what the code now actually means.

Files:

- `docs/ai_promotion_and_multilanguage_milestones_v0.1.md`

Required work:

- describe M31 as the landed boundary extraction prerequisite
- describe M32 as the first bounded second-language promotion path
- avoid any broad "TypeScript is now supported" claim
- align roadmap wording with the actual monotone-up pilot contract and
  comparator policy

Exit condition:

- the roadmap, the plan, and the landed code use the same milestone language

## Code Path Diagram

```text
[1] Monotone-up committed packet
    semantic-families/function.arithmetic_leaf.monotone_up.v1/**
        |
        +-- family smoke
        |      |
        |      +-- exact scaffold contract still matches
        |      `-- additive body.typescript remains committed truth
        |
        +-- family prove (rust)
        |
        +-- family certify (rust)
        |
        +-- family prove (typescript)
        |
        `-- family certify (typescript)
                |
                +-- report.rs writes target-language-aware certify/prove artifacts
                +-- promotion_artifacts.rs validates/records target-language-aware execution truth
                |
                v
[2] Public read-side surfaces
    semantic_review.rs
    passport.rs
    export.rs
    spec status / spec-cli/src/commands.rs
        |
        +-- authored body.typescript remains visible
        +-- no false broad-support claim is introduced
        `-- monotone-up pilot truth stays aligned across surfaces

[3] Comparator pressure
    wrapper_pipeline suites stay green
        |
        `-- proves the shared kernel was not broken while landing the bounded pilot
```

Every branch above needs tests.

## Test and Proof Plan

100% of the new M32 codepaths must be covered. This milestone is easy to fake
with one green TypeScript command and a vague closeout note. That is not good
enough.

### Code path coverage

```text
XTASK TARGET-LANGUAGE TRUTH
===========================
[+] xtask/src/family/prove.rs
    ├── [GAP] target-language lane recorded explicitly in prove output/report path
    ├── [TEST] unsupported family + typescript still rejected
    └── [TEST] rust default behavior unchanged when flag omitted

[+] xtask/src/family/certify.rs
    ├── [GAP] certify attempt/certification artifacts record target language
    ├── [TEST] rust lane still certifies normally
    └── [TEST] typescript lane propagates prove/routing failures truthfully

[+] xtask/src/family/promotion_artifacts.rs
    ├── [GAP] execution artifact currently rust-only
    ├── [TEST] validator accepts truthful typescript execution artifact
    └── [TEST] legacy rust artifacts still validate

PACKET / HARNESS PILOT
======================
[+] xtask/src/family/harness.rs + semantic-families/function.arithmetic_leaf.monotone_up.v1/**
    ├── [TEST] family smoke still enforces the committed monotone-up scaffold
    ├── [TEST] committed packet keeps additive body.typescript in all buckets
    └── [TEST] prove/certify suite ownership remains locked

READ-SIDE TRUTH
===============
[+] spec-core/src/semantic_review.rs
    ├── [TEST] authored body.typescript still appears in semantic citations
    ├── [TEST] monotone-up supported truth remains supported
    └── [TEST] wrapper-pipeline regression suites stay green

[+] spec-core/src/passport.rs / export.rs / spec-cli/src/commands.rs
    ├── [TEST] passport, export, and status agree on the pilot fixture set
    ├── [TEST] read-side truth does not imply broad target-language support
    └── [TEST] target-specific residue stays visible when present

REGRESSION PRESSURE
===================
[+] spec-cli/tests/m14_regressions.rs
    ├── [TEST] monotone_up_truth_surface_* stays green
    ├── [TEST] monotone_up_corpus_* stays green
    ├── [TEST] monotone_up_regression_* stays green
    └── [TEST] wrapper-pipeline regressions stay green
```

### User-flow coverage

```text
PROMOTION FLOW
==============
[+] Maintainer runs monotone-up smoke/prove/certify
    ├── [TEST] Rust lane still works without any flag
    ├── [TEST] TypeScript lane works with --target-language typescript
    └── [GAP] Artifact outputs distinguish which lane actually ran

READ-SIDE FLOW
==============
[+] User inspects the repo after the pilot
    ├── [TEST] passport still surfaces the same monotone-up semantic truth
    ├── [TEST] spec status still reports the same truth
    ├── [TEST] spec export still reports the same truth
    └── [GAP] closeout wording must not overclaim broad TypeScript support

COMPARATOR FLOW
===============
[+] Shared-kernel regression check
    ├── [TEST] wrapper pipeline suites remain green
    └── [TEST] monotone-up addition did not shadow or weaken existing family routing
```

### Required regression tests

Add or preserve tests proving:

- `prove` and `certify` keep Rust as the default lane when no target-language
  flag is passed
- `prove` and `certify` artifacts record `typescript` truthfully when that lane
  is selected
- promotion-artifact validation accepts truthful TypeScript execution artifacts
- monotone-up smoke still enforces the committed additive TypeScript scaffold
- monotone-up semantic-review tests still cite authored `body.typescript`
- passport, export, and status agree on the monotone-up pilot fixtures
- wrapper-pipeline regression suites stay green as comparator pressure
- unsupported families still reject `--target-language typescript`

### Failure modes by codepath

| Codepath | Realistic production failure | Test required | Error handling / visible truth |
|---|---|---|---|
| `prove` target-language plumbing | TypeScript lane silently writes an artifact that still claims `rust` | Yes, xtask unit/integration test | Artifact must show the real lane |
| `certify` target-language plumbing | Rust and TypeScript certify attempts overwrite or blur each other | Yes, xtask report/artifact test | Reports must distinguish lanes explicitly |
| Promotion artifact validation | New truthful TypeScript artifact shape is rejected as invalid | Yes, validator regression | Schema must accept the new honest shape |
| Monotone-up harness contract | Packet drifts away from smoke expectations while still compiling | Yes, smoke regression | `family smoke` must catch scaffold drift |
| Semantic review | Authored `body.typescript` stops appearing in citations | Yes, semantic-review regression | Truth surface must stay explicit |
| Passport/export/status | One surface implies broad target-language support while the others stay bounded | Yes, cross-surface regression | Surfaces must agree on the same bounded claim |
| Wrapper comparator | Shared-kernel change breaks wrapper pipeline while monotone-up stays green | Yes, existing regression suites | Wrapper pressure must remain visible |
| Roadmap wording | Docs claim TypeScript support broadly after one bounded pilot | Yes, closeout review | Docs must stay narrow and truthful |

Critical gap rule:

If any path above lacks both a regression test and a truthful public surface,
the milestone is not done.

### Commands to run

Run at minimum:

```bash
cargo xtask family smoke function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo test -p xtask monotone_up
cargo test -p spec-core monotone_up_
cargo test -p spec-core wrapper_pipeline_
cargo test -p spec-cli --test cli monotone_up_
cargo test -p spec-cli --test cli wrapper_pipeline_
cargo test
```

The narrow loop can be smaller while implementing. Done still requires the full
set above plus workspace `cargo test`.

## Performance Review

There is no meaningful runtime hot-path risk in M32. This is a proof-integrity
and truth-surface milestone.

The real performance risks are engineering-performance risks:

- duplicating target-language truth in both reports and read-side surfaces
- adding a second pilot family "for confidence" and doubling review scope
- inventing a generic multi-language abstraction before the repo has even proven
  one honest second-language lane

Recommendation: be boring by default.

Make the current bounded path explicit. Do not spend an innovation token on a
premature general multi-language framework.

## Distribution Surface

M32 introduces no new binary, package, or container.

Its distribution surface is repo truth:

- the monotone-up packet and harness
- target-language-aware prove/certify artifacts
- passport, export, status, and semantic-review truth on the same pilot
- the roadmap and plan closeout

Code without those truth surfaces is not a real M32 ship.

## Worktree Parallelization Strategy

This plan has real parallelization opportunity, but only after the target-language
artifact shape is frozen.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| A. Target-language artifact foundation | `xtask/src/`, especially prove/certify/report/promotion-artifacts | - |
| B. Monotone-up packet + harness lock-in | `xtask/src/family/`, `semantic-families/` | A |
| C. Read-side truth alignment | `spec-core/src/`, `spec-cli/src/`, `spec-cli/tests/` | A |
| D. Roadmap + packet-doc closeout | `docs/`, `semantic-families/README.md` | B, C |

### Parallel lanes

- Lane A: `A`
  Sequential foundation lane. Freeze artifact/report truth first.
- Lane B: `B`
  Runs after Lane A. Owns the monotone-up packet and harness contract.
- Lane C: `C`
  Runs after Lane A in parallel with Lane B. Owns passport/export/status and
  semantic-review truth alignment.
- Lane D: `D`
  Runs after B + C. Docs and closeout last so they describe the actual landed
  implementation.

### Execution order

Launch Lane A first.

After Lane A is merged or otherwise stabilized, launch Lane B and Lane C in
parallel worktrees.

Merge B + C, then do Lane D last.

### Conflict flags

- Lanes B and C both depend on the final target-language artifact shape from
  Lane A. Freeze that shape before splitting.
- `xtask/src/lib.rs` belongs to Lane A. Do not let Lane B or Lane C take
  opportunistic ownership of the CLI parsing layer.
- `semantic-families/README.md` belongs to Lane D unless a packet-local doc
  change is compile- or review-forced earlier.

If the work is not split into worktrees, execute sequentially in the same
order:

```text
A -> B -> C -> D
```

## Completion Summary

- Step 0: Scope Challenge
  Accepted as-is, because the minimum honest surface already spans proof
  artifacts plus read-side truth surfaces.
- Architecture Review
  One bounded monotone-up pilot, one artifact-truth foundation, one read-side
  truth alignment pass.
- Code Quality Review
  Reuse the existing CLI, harness, packet, and truth surfaces. Extend them
  explicitly rather than inventing a new framework.
- Test Review
  Explicit second-language coverage diagram plus mandatory regressions above.
- Performance Review
  No runtime bottleneck. Truth drift and over-abstraction are the real risks.
- NOT in scope
  Written.
- What already exists
  Written.
- Failure modes
  Written.
- Parallelization
  Four steps, two post-foundation lanes that can run in parallel.
- Distribution
  Explicitly limited to repo truth surfaces.

## Implementation Guardrail

If implementation discovers that the monotone-up pilot cannot be made truthful
without widening target-language execution semantics for seam kinds, molecule
tests, or a second family, stop.

That is not "small spillover." That is M33-or-later work trying to leak into a
bounded M32 milestone.
