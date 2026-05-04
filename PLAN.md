<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260503-232225.md -->
# M30 - Add Second Bounded TypeScript Family Proof

Status: **authoritative implementation plan**
Base branch: **main**
Working branch: **feat/corpus-expansion**
Last rewritten: **2026-05-03**
Supersedes: **M29R - Additive Body Contract Recovery**
Design authority: **`/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260503-231926.md`**
Related roadmap: **`docs/ai_promotion_and_multilanguage_milestones_v0.1.md`**
Public proof baseline: **GitHub Actions run `25296654428` on `6c100d05902519634dc5002445036a050b506934`**
Execution note: **This plan does not require a fresh `ORCH_PLAN.md` up front. Add one only if implementation expands beyond the closed surface or needs parallel worktrees.**

## Decision

M29R and the narrow CI truth-repair follow-on answered the first real question:
the shared `kind:function` authoring boundary can carry additive
`body.typescript` truth without hidden `spec_version` routing and without
breaking the Rust-default lane.

M30 should answer the next question and only that question:

> Does the same explicit authored TypeScript contract hold on a second,
> materially different promoted `kind:function` family?

The chosen forcing function is `function.wrapper.pipeline.v1`.

M30 is not a TypeScript corpus-analysis milestone. It is not repo-wide
target-language support. It is one second family proof.

## Problem Statement

The branch now has public evidence that the first TypeScript pilot is real:

1. `kind:function` specs can author additive `body.typescript` through the
   shared path.
2. `cargo xtask family prove/certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript`
   pass.
3. Ordinary CI and the monotone-up pilot both passed publicly on
   GitHub Actions run `25296654428`.

The remaining uncertainty is cross-family generalization.

The repo already contains a promoted wrapper family packet under
`semantic-families/function.wrapper.pipeline.v1/**`, complete with Rust prove
and certify suites, read-side regressions, and a registered family harness.
But the current target-language gate in `xtask/src/family/prove.rs` still says:

- `Rust` is accepted for all registered families
- `Typescript` is accepted only for
  `function.arithmetic_leaf.monotone_up.v1`

That means the repo currently has:

- one shared family registry
- one Rust-only maintainer corpus manifest
- one explicit TypeScript proof family

It does **not** yet have proof that the same authored TypeScript contract can
survive the wrapper-pipeline family shape with supported dep threading and
packet-local leaf dependencies.

There is one repo-local truth wrinkle worth naming up front:
`spec-cli/tests/m14_regressions.rs` still contains a monotone-up helper that
copies fixtures and injects TypeScript bodies at test time when they are
missing. M30 must not copy that pattern forward as the second-family template.

## Done Means

M30 is complete only when all of these are true:

1. `semantic-families/function.wrapper.pipeline.v1/**` remains the authoritative
   packet root for the second proof. No alternate packet root is introduced.
2. The committed wrapper packet fixtures truthfully carry additive
   `body.typescript` where the second-language proof depends on it, across all
   four required buckets:
   `aligned`, `drift`, `under_specified`, `unsupported_near_miss`.
3. The wrapper packet stays self-contained. If the wrapper proof depends on
   packet-local discount and tax leaf behavior, those packet-local units also
   carry truthful additive TypeScript bodies rather than relying on hidden
   external target assumptions.
4. Rust prove/certify for `function.wrapper.pipeline.v1` still pass unchanged:
   - `cargo xtask family prove function.wrapper.pipeline.v1`
   - `cargo xtask family certify function.wrapper.pipeline.v1`
5. TypeScript prove/certify for `function.wrapper.pipeline.v1` pass:
   - `cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript`
   - `cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript`
6. The existing monotone-up TypeScript proof still passes. M30 must not earn a
   second family by regressing the first.
7. Wrapper read-side proof surfaces stay honest:
   - copied wrapper packet fixtures load truthfully through the shared loader
   - wrapper status/export projections still classify buckets correctly
   - unsupported near-miss remains additive-only and neutral
   - copied-fixture proof does not depend on new test-time TypeScript injection
     helpers
8. The second proof does not require new hidden family-specific routing
   metadata, new packet roots, or repo-wide `spec build/test --target-language typescript`.
9. Ordinary CI remains green on the exact pushed SHA, and public CI also proves
   the wrapper TypeScript path.
10. Closeout can truthfully say one shared authored TypeScript contract now
    holds across **two** promoted `kind:function` families with different
    shapes:
    `function.arithmetic_leaf.monotone_up.v1` and
    `function.wrapper.pipeline.v1`.

## NOT in Scope

The following work was considered and is explicitly deferred:

- `semantic-families/corpus/typescript-function.toml`
  Reason: corpus-analysis symmetry is not the question M30 is answering.
- Repo-wide `spec build --target-language typescript`
  Reason: M30 is family-local proof, not repo-wide CLI support.
- Repo-wide `spec test --target-language typescript`
  Reason: same blast-radius control.
- TypeScript support for `kind:data`
  Reason: M30 remains bounded to promoted `kind:function` families only.
- TypeScript support for `kind:sum`
  Reason: same boundary.
- TypeScript support for `.test.spec`
  Reason: molecule tests remain Rust-only.
- Third TypeScript family proof
  Reason: two families is the complete M30 claim.
- New packet roots such as `semantic-families-typescript/`
  Reason: one packet root per family must remain authoritative.
- Recommendation / coverage analysis changes
  Reason: the Rust maintainer corpus stays unchanged in M30.
- npm publishing, runtime distribution, or external TypeScript packaging
  Reason: this is still an internal proof milestone.

## What Already Exists

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Shared authored TypeScript contract for `kind:function` | `spec-core/src/schema/unit.spec.json`, `spec-core/src/types.rs`, `spec-core/src/semantic_review.rs` | Reuse unchanged unless wrapper proof demonstrates a real gap. |
| Registered wrapper family harness | `xtask/src/family/harness.rs` | Reuse. Extend target-language proof, do not redesign the registry. |
| Wrapper packet root and Rust fixtures | `semantic-families/function.wrapper.pipeline.v1/**` | Reuse. Add additive TypeScript truth inside the existing packet. |
| Wrapper Rust prove/certify suites | `xtask/src/family/harness.rs`, `xtask/src/family/prove.rs`, `xtask/src/family/certify.rs` | Reuse. Keep Rust behavior stable while widening the explicit target-language gate. |
| Wrapper read-side regression coverage | `spec-cli/tests/m14_regressions.rs` | Reuse. Extend copied-fixture proof rather than inventing a new test harness. |
| Monotone-up TypeScript pilot pattern | `xtask/src/family/prove.rs`, `.github/workflows/ci.yml`, `spec-core/src/semantic_review.rs`, `semantic-families/function.arithmetic_leaf.monotone_up.v1/**` | Reuse as the template, not the permanent one-family exception. |
| Rust maintainer corpus analysis | `semantic-families/corpus/rust-function.toml`, `xtask/src/family/coverage.rs`, `xtask/src/family/recommend.rs` | Reuse unchanged. M30 does not widen corpus analysis. |

## Step 0 - Scope Challenge

### Premises

1. The next uncertainty is cross-family generalization, not shared-core truth.
   Verdict: **accept**
2. `function.wrapper.pipeline.v1` is the best second proof because it exercises
   a materially different promoted `kind:function` shape.
   Verdict: **accept**
3. A TypeScript corpus manifest is the wrong next move. It adds a new workflow
   before the second family proof exists.
   Verdict: **accept**
4. M30 should extend the existing pilot architecture, not replace it with
   repo-wide target-language machinery.
   Verdict: **accept**

### Minimum Change That Still Counts

The minimum honest M30 diff is:

1. add truthful additive TypeScript bodies to the committed
   `function.wrapper.pipeline.v1` packet surfaces that the proof depends on
2. widen the target-language prove/certify gate to allow
   `function.wrapper.pipeline.v1`
3. add wrapper-specific read-side and semantic-review proof that authored
   TypeScript survives the second family path
4. prove the result publicly in CI without widening repo-wide TypeScript
   command support
5. reuse existing prove/certify suite and artifact surfaces instead of
   inventing target-specific report paths or a second suite registry

Anything smaller turns M30 into naming theater.

### Complexity Check

This plan likely touches more than 8 files. Normally that is a smell.

Here it is still the smallest honest surface because a second family proof has
to align four layers:

- packet-local authored truth
- wrapper-family semantic-review proof
- target-language prove/certify gate
- public CI proof

No new subsystem is introduced. The milestone is still sequential and bounded.

### TODOS Cross-Reference

`TODOS.md` does not contain an open item that blocks M30 directly.

The most relevant standing theme is the post-M23 discipline note:
do not reduce packet ceremony or generalize target-language support before the
repo has enough real family proofs. M30 is exactly the milestone that earns the
right to revisit that later.

### Completeness Check

The obvious shortcut is to add wrapper-family target-language support without
putting truthful additive TypeScript into the packet-local fixtures.

Reject it.

That would prove a second routing carveout, not a second authored contract.

M30 should take the complete bounded option:

- truthful packet-local additive TypeScript
- wrapper-family read-side proof
- explicit target-language widening only for the second family
- ordinary CI and public pilot proof on the pushed SHA

### Distribution Check

No new user-facing artifact type is introduced.

Internal delivery surfaces only:

- committed wrapper packet truth
- `cargo xtask` prove/certify selectors
- regression and semantic-review test surfaces
- CI verification
- milestone closeout

## Dream State Delta

Today the repo can only claim one promoted `kind:function` family with public
TypeScript prove/certify proof.

If M30 lands cleanly, the repo can claim:

- two promoted `kind:function` families with materially different shapes share
  one explicit authored TypeScript contract
- the second proof reused the existing family registry, wrapper suites, and
  prove/certify artifact surfaces
- CI exposes each family proof separately, so failure attribution stays obvious

It still will **not** claim:

- TypeScript corpus analysis
- repo-wide `spec build/test --target-language typescript`
- TypeScript support for seam kinds or molecule tests

## Architecture Diagram

```text
Committed wrapper packet truth
semantic-families/function.wrapper.pipeline.v1/**
    │
    ├── wrapper packet-local units
    │     pricing_discount_leaf_*
    │     pricing_tax_leaf_*
    │     pricing_total_wrapper_*
    │
    └── additive authored bodies
          body.rust + body.typescript
                    │
                    v
Shared loader / validator / normalized function packet
spec-core shared kind:function path
                    │
                    ├── Rust-default semantic review and prove/certify
                    └── TypeScript target-language pilot for wrapper family
                              │
                              v
                    xtask family prove/certify
                              │
                              ├── spec-core wrapper classifier suites
                              ├── spec-cli wrapper corpus/status/export regressions
                              └── CI public proof on pushed SHA
```

## Closed Implementation Surface

### Primary implementation files

- `semantic-families/function.wrapper.pipeline.v1/**`
- `spec-core/src/semantic_review.rs`
- `spec-cli/tests/m14_regressions.rs`
- `xtask/src/family/prove.rs`
- `xtask/src/family/certify.rs`
- `xtask/src/family/harness.rs`
- `xtask/src/family/scaffold.rs`
- `xtask/src/lib.rs`
- `.github/workflows/ci.yml`
- `PLAN.md`

### Allowed mechanical spillover

Because wrapper-family proof is exercised through existing prove/certify and test
helpers, small mechanical updates are allowed only when all of the following are
true:

1. the edit is a direct compile-fix or expectation-fix caused by the M30 closed
   surface
2. the edit does not widen the family registry beyond
   `function.wrapper.pipeline.v1`
3. the edit does not introduce repo-wide TypeScript support
4. the edit does not change maintainer corpus analysis behavior

Likely spillover sites, only if forced:

- `spec-core/src/types.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/passport.rs`

If M30 needs semantic changes outside the primary surface, stop and rewrite the
plan before implementation continues.

## Workstreams

### WS-1 Packet-local additive TypeScript truth

Goal: make the wrapper packet itself truthful for second-language proof.

Required work:

- add additive `body.typescript` to the wrapper family fixtures that participate
  in the proof
- keep the packet self-contained rather than relying on hidden external target
  assumptions
- update any scaffold smoke contract that claims to represent the wrapper packet

Acceptance:

- committed wrapper packet fixtures show explicit additive TypeScript truth
- smoke and packet-layout expectations still pass

### WS-2 Wrapper read-side and semantic proof

Goal: prove the shared loader and wrapper semantic-review route read the new
packet truth honestly.

Required work:

- add at least one explicit wrapper semantic-review assertion that authored
  `body.typescript` is preserved on the second-family path
- extend copied-wrapper fixture regressions in
  `spec-cli/tests/m14_regressions.rs` so the shared loader path proves additive
  TypeScript survives for all four buckets
- retire or bypass the existing test-time TypeScript injection pattern in
  `spec-cli/tests/m14_regressions.rs` so wrapper proof is grounded in committed
  packet bytes
- preserve existing wrapper bucket health outcomes

Acceptance:

- wrapper classifier suites stay green
- wrapper corpus/status/export regressions stay green

### WS-3 Target-language prove/certify widening

Goal: widen the existing TypeScript pilot only far enough to include the second
family.

Required work:

- extend `xtask` target-language validation to allow
  `function.wrapper.pipeline.v1`
- preserve the current explicit rejection for every other family
- keep Rust-default prove/certify unchanged
- keep prove/certify artifact paths and suite ownership exactly as they are;
  the second-family proof should extend wrapper-owned suites rather than
  introduce target-specific suite names or output trees

Acceptance:

- wrapper TypeScript prove/certify pass
- non-wrapper, non-monotone-up families still fail fast on
  `--target-language typescript`
- existing wrapper suite slugs and prove/certify artifact surfaces remain
  authoritative

### WS-4 CI and public proof

Goal: make the second family publicly provable on the pushed SHA.

Required work:

- keep the ordinary workspace lane green
- preserve the existing monotone-up TypeScript pilot proof
- add a dedicated `wrapper_pipeline_pilot` CI job
- keep `monotone_up_pilot` as its own job; do not collapse both families into
  one opaque multi-family TypeScript lane

Acceptance:

- public CI on the pushed SHA shows ordinary lane green
- public CI shows monotone-up pilot green
- public CI shows wrapper pipeline TypeScript proof green

## Test and Failure Map

```text
M30 proof paths
================

[+] Packet-local authored truth
    ├── aligned wrapper fixture carries additive TS body
    ├── drift wrapper fixture carries additive TS body
    ├── under_specified wrapper fixture carries additive TS body
    └── unsupported_near_miss wrapper fixture carries additive TS body

[+] Shared read-side loader
    ├── copied wrapper aligned fixture loads and projects valid
    ├── copied wrapper drift fixture loads and projects failing
    ├── copied wrapper under_specified fixture loads and projects incomplete
    └── copied wrapper unsupported_near_miss stays additive-only and neutral

[+] Wrapper semantic-review target path
    ├── authored TS body preserved in wrapper semantic packet
    └── wrapper Rust-default route still classifies the same family correctly

[+] xtask target-language gate
    ├── wrapper prove/certify typescript accepted
    ├── monotone_up prove/certify typescript still accepted
    └── every other family still rejected

[+] Public proof
    ├── ordinary workspace lane green
    ├── monotone_up pilot green
    └── wrapper pipeline pilot green
```

Critical silent-failure risks M30 must prevent:

1. Wrapper fixtures gain TypeScript bodies in only one bucket, so the packet
   looks truthful on the happy path and lies on regressions.
2. Wrapper target-language gate is widened, but copied packet fixtures are not
   exercised through the shared loader path.
3. CI bundles both families into one opaque pilot command without preserving
   clear failure attribution.

## Error and Rescue Registry

| Risk | Where it shows up | Early signal | Rescue path |
|---|---|---|---|
| Wrapper fixtures gain partial TS coverage only | `semantic-families/function.wrapper.pipeline.v1/fixtures/**` | one bucket passes TS prove while another still lacks `body.typescript` | stop and finish packet truth across all four buckets before touching CI |
| Wrapper proof depends on new ad hoc test rewriting | `spec-cli/tests/m14_regressions.rs` | new helper injects or mutates wrapper TS bodies at test time | reject the helper, move the truth into committed fixture bytes, re-run read-side coverage |
| TS widening leaks beyond the intended families | `xtask/src/family/prove.rs`, `xtask/src/family/certify.rs` | non-wrapper families start accepting `--target-language typescript` | revert to explicit allowlist of `monotone_up` + `wrapper_pipeline` only |
| CI hides which family broke | `.github/workflows/ci.yml` | one TypeScript pilot job runs both families and emits one red badge | split back into `monotone_up_pilot` and `wrapper_pipeline_pilot` before calling the milestone done |

## Failure Modes Registry

| Failure mode | Severity | Why it matters | Mitigation in M30 |
|---|---|---|---|
| Shared contract looks generalized but only because tests patched fixtures in memory | High | maintainer proof becomes misleading and the packet stops being the source of truth | committed wrapper fixtures must carry additive TS directly |
| Wrapper family passes TS prove but on a bespoke suite/output path | High | repo learns nothing about whether the existing harness generalizes | require reuse of wrapper-owned suites and current artifact surfaces |
| Combined TypeScript CI lane masks the failing family | Medium | diagnosis slows down and maintainers lose confidence in the pilot signal | dedicated wrapper pilot job with monotone-up kept separate |
| Scaffold claims wrapper starter truth that does not match the committed packet | Medium | future packet refreshes drift and reintroduce hidden cleanup work | update `xtask/src/family/scaffold.rs` in the same milestone if starter output claims TypeScript support |

## Parallelization Strategy

Sequential implementation, no parallelization opportunity.

The likely touch set is small enough and coupled enough that parallel worktrees
would create merge friction for little gain:

| Step | Modules touched | Depends on |
|---|---|---|
| WS-1 packet truth | `semantic-families/function.wrapper.pipeline.v1/`, `xtask/src/family/scaffold.rs`, `xtask/src/lib.rs` | — |
| WS-2 read-side proof | `spec-core/src/semantic_review.rs`, `spec-cli/tests/`, `xtask/src/family/harness.rs` | WS-1 |
| WS-3 target gate | `xtask/src/family/prove.rs`, `xtask/src/family/certify.rs`, `xtask/src/family/harness.rs` | WS-2 |
| WS-4 CI proof | `.github/workflows/ci.yml` | WS-3 |

## DX Review

### DX Scorecard

| Dimension | Score | Why |
|---|---|---|
| Discoverability | 8/10 | family ids and harness slugs are explicit in `xtask/src/family/harness.rs`, but TS support is still gated in one place that contributors have to find |
| Local iteration speed | 7/10 | targeted wrapper and monotone-up suites exist, but the ordinary verification floor is still heavyweight |
| Failure attribution | 9/10 | dedicated wrapper pilot plus preserved monotone-up pilot keeps the red badge actionable |
| Fixture truthfulness | 6/10 | current monotone-up injection helper proves the repo can still cheat accidentally |
| Reversibility | 9/10 | M30 remains allowlist-based and can be backed out family by family |
| Test ergonomics | 8/10 | existing suite slugs are well-factored for selective execution |
| Consistency | 8/10 | one shared family registry and one packet root per family stay intact |
| Onboarding clarity | 7/10 | `PLAN.md` can make the milestone legible, but code comments/tests still need to carry the boundary clearly |

DX overall: **7.8/10**

### Developer Journey Map

```text
Contributor asks "how do I add TS proof to a second family?"
    │
    ├── reads PLAN.md and design doc
    ├── finds current gate in xtask/src/family/prove.rs
    ├── inspects wrapper packet fixtures
    ├── runs targeted wrapper semantic + spec-cli regressions
    ├── runs wrapper prove/certify in Rust and Typescript
    └── pushes and reads separate pilot jobs in CI
```

### Developer Empathy Narrative

The tired maintainer failure case is obvious here. They add
`--target-language typescript` for wrapper, CI goes red, and now they have to
guess whether the problem is fixture truth, semantic review, prove gating, or
one huge combined pilot job.

M30 should remove that guesswork. The packet bytes should already be truthful.
The wrapper-owned suites should stay the same. The CI job name should tell you
which family broke. That is the difference between "I can land this in an
evening" and "I am spelunking a one-off pilot exception at 11:30pm."

### TTHW

Current time-to-honest-wrapper-proof: **~20 minutes**

Why:
- contributors must inspect the gate, harness, packet fixtures, and CI wiring
- current monotone-up injection precedent creates doubt about where truth is
- verification spans both targeted suites and the ordinary lane

Target after M30: **~10 minutes**

How:
- one committed wrapper packet with additive TS in all required buckets
- one explicit allowlist expansion in `xtask/src/family/prove.rs`
- one dedicated `wrapper_pipeline_pilot` job
- no new suite names or artifact paths to discover

### DX Implementation Checklist

- [ ] packet bytes, not tests, author the wrapper TypeScript truth
- [ ] `xtask` allowlist names exactly two TS-proof families
- [ ] wrapper suite slugs stay unchanged
- [ ] wrapper pilot has its own CI job name
- [ ] verification commands in this plan stay sufficient for a newcomer to reproduce the result

## Explicit Verification Sequence

Run this exact merged-state verification sequence before calling M30 done:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo run -p spec-cli -- generate examples/ecommerce/units --output examples/ecommerce/src/generated
cargo check --manifest-path examples/ecommerce/Cargo.toml
cargo test -p spec-core wrapper_pipeline_classifier_ -- --color never
cargo test -p spec-cli --test m14_regressions wrapper_pipeline_ -- --color never
cargo test -p spec-cli --test m14_regressions monotone_up_ -- --color never
cargo xtask family smoke function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1
cargo xtask family certify function.wrapper.pipeline.v1
cargo xtask family prove function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family certify function.wrapper.pipeline.v1 --target-language typescript
cargo xtask family prove function.arithmetic_leaf.monotone_up.v1 --target-language typescript
cargo xtask family certify function.arithmetic_leaf.monotone_up.v1 --target-language typescript
```

These commands are the verification floor for M30.

## Cross-Phase Themes

**Truth over convenience** — the same theme shows up in scope, architecture,
and DX. Every shortcut that makes M30 easier in the moment, test-time fixture
injection, combined CI pilot lanes, target-specific suite names, weakens the
one thing this milestone is supposed to prove.

**Boring reuse is the win** — the right shape is not more TypeScript machinery.
It is proving that the existing registry, packet root, wrapper suites, and
prove/certify surfaces already generalize one family further.

## Closeout and Verdict

M30 closes with exactly one verdict:

- `EXPAND` if the second family proof passes cleanly, ordinary CI is green on
  the pushed SHA, and no new hidden target-language carveout was required
- `NARROW` if the wrapper family works but one additional bounded follow-on is
  needed before broader TypeScript expansion is honest
- `STOP` if the second family proof requires a new packet root, repo-wide
  target-language support, or hidden family-specific metadata to pass

The closeout must answer plainly:

1. Did one shared authored TypeScript contract survive on two promoted
   `kind:function` families?
2. Did ordinary CI and both pilot lanes pass on the pushed SHA?
3. What exact question is still unanswered after M30?

Anything softer is not a real closeout.

<!-- AUTONOMOUS DECISION LOG -->
## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|-------|----------|----------------|-----------|-----------|----------|
| 1 | CEO | Make `function.wrapper.pipeline.v1` the M30 target instead of adding a TypeScript corpus manifest | Scope | Incremental over revolutionary | It answers the next unknown directly without creating a new maintainer-analysis lane first. | `typescript-function.toml` first |
| 2 | ENG | Require committed wrapper packet fixtures to carry additive `body.typescript` across all four buckets | Truth boundary | Make the change easy, then make the easy change | The packet has to remain the source of truth or the proof becomes synthetic. | test-time fixture mutation |
| 3 | ENG | Reuse existing wrapper prove/certify suites and artifact paths | Architecture | Boring by default | A second-family proof should demonstrate harness reuse, not new target-specific plumbing. | new suite registry or TS-specific output tree |
| 4 | ENG | Require a dedicated `wrapper_pipeline_pilot` CI job and preserve `monotone_up_pilot` | Observability | Systems over heroes | Separate jobs keep failure attribution obvious for maintainers and future contributors. | one combined multi-family pilot lane |
| 5 | DX | Keep the TS allowlist explicitly limited to `monotone_up` and `wrapper_pipeline` in M30 | Blast radius | Reversibility preference | The smallest reversible expansion gives the repo a real signal without spending more innovation tokens. | repo-wide TS prove/certify widening |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAN | 1 proposal, 1 accepted, 0 deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAN | 3 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | no UI scope in M30 |

- **DX:** 1 review path through `/autoplan`, clean. TTHW: ~20 min -> ~10 min.
- **UNRESOLVED:** 0
- **VERDICT:** CEO + ENG CLEARED — ready to implement M30 on the closed surface above.
