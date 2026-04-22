<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/main-autoplan-restore-20260421-144420.md -->
# M15 — Semantic Governance + Eval

Status: **Draft, direction locked** (2026-04-22). M14 shipped at `v0.12.0` via
`feat: ship M14 proof freshness and truth surfaces` (`6519dbe`), so this section replaces M14 as
the current implementation contract. Source inputs are the shipped M14 plan and code in
`spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs`,
`spec-core/src/escape_hatch.rs`, `spec-core/src/validator.rs`, `spec-core/src/plan.rs`,
`examples/ecommerce/units/pricing/discount_policy.unit.spec`, and
`examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`.

This section is the M15 implementation contract. Historical roadmap material below it is record,
not current scope. Two separate implementers should be able to read this section and converge on
the same diff shape and the same proof obligations.

UI scope: **no**. This is a semantic-governance and review-surface milestone for evaluator
contracts, passports, status, export, and the canonical ecommerce seams.

## Milestone Summary

```text
M15a  Semantic alignment contract + verdicts      required
M15b  Evaluator inputs for function/data/sum      required
M15c  Passport/status/export eval truth surfaces  required
M15d  Canonical mismatch wedges + regression set  required
M15e  Escape-hatch semantic leak classification   required
M15f  Post-M15 decision gate                      required
```

**Lake to boil in M15**
- Make `green` mean something tighter than "the authored seam is fresh and the last proof passed."
- Teach the repo to ask the next real review question:
  - does `intent.why` still match what the executable lowering does?
  - does the authored contract still describe the behavior that was implemented?
  - did a backend-only edit preserve meaning or quietly change it?
  - is the authored truth under-specified enough that the evaluator cannot judge honestly?
- Keep the top-level seam node model from M14. Do not promote variants or methods into first-class
  graph/status nodes yet.
- Reuse the canonical `pricing/discount_policy` wedge again. The right M15 proof is not a toy
  semantic scorer. It is one real seam that can pass, fail, or be marked under-specified for
  reasons a reviewer can understand.
- Keep second-backend work out of scope until one seam can fail semantic review honestly.

**User job**
- An AI-heavy Rust maintainer edits one real unit or seam, runs
  `spec validate -> spec build -> spec test -> spec status -> spec export`, and can trust the
  resulting artifacts to answer:
  - does the authored semantic story still match the executable behavior?
  - is the evaluator saying "aligned", "under-specified", or "semantic drift"?
  - did the backend-only edit preserve meaning or leak new semantics through an escape hatch?
  - which seams now require human review because meaning, not freshness, is in doubt?

**Actual buyer**
- Primary buyer remains the AI-heavy Rust maintainer reviewing or making policy edits with agent
  help. M15 is still not for a hypothetical multi-backend platform buyer yet.

**Painful workflow this milestone must improve**
1. Edit a real unit or seam such as `pricing/discount_policy`.
2. Run the normal trust loop.
3. Decide whether the change preserved meaning, changed meaning, or revealed under-specified
   authored truth.
4. Decide whether the change is safe to accept without manually reverse-engineering raw Rust.

If M15 cannot make that workflow more honest and more localized, a second backend will just widen
the blast radius of semantic guesswork.

## Locked Boundary

- M15 adds **no new seam kind** and **no new backend**.
- M15 keeps the M14 top-level seam-node model. One seam still produces one unit row, one passport,
  and one export unit entry.
- M15 must make semantic review explicit and reusable across every truth surface:
  - authored semantic intent
  - executable lowering behavior
  - evaluator verdict and confidence class
- M15 may add additive metadata to passports, export, and status, but it must not promote nested
  variant or method behaviors into first-class graph nodes.
- M15 must support `kind: function`, `kind: data`, and `kind: sum`. It is not allowed to act like
  semantic review is only meaningful for the original function shape.
- Escape hatches remain allowed in M15, but their semantic effect must become classifiable:
  preserved meaning, leaked meaning, or under-specified truth.
- Explicitly not in M15:
  - second-backend implementation
  - first-class variant or method graph nodes
  - full sandboxing or elimination of `lowering.rust.body`
  - autonomous merge decisions based on evaluator output alone
  - new seam kinds or wider Rust item coverage

## Premises

1. M14 proved that the repo can separate authored freshness from backend freshness. It did **not**
   prove that the repo can judge whether executable behavior still matches authored meaning.
2. The next bottleneck is semantic alignment and under-specification detection, not another backend
   or another authored seam kind.
3. The genesis of this product was always the gap between `intent.why` and `body.rust`; M15 should
   finally turn that thesis into a first-class product surface instead of leaving it as a TODO.
4. `methods[].lowering.rust.body` remains the main semantic escape hatch for seam kinds. That is
   acceptable only if the repo can now say whether the escape hatch preserved or leaked meaning.
5. The default M16 question after this milestone should become: "is semantic review honest enough
   that backend travel would be reviewable?" not "can we add another generator now?"

## Dream State

```text
CURRENT (after M14)
  freshness is honest
  proof surfaces are explicit
  escape hatches are marked and gated
  meaning still requires human reverse-engineering

M15 TARGET
  authored intent/contract/deps and executable lowering are compared explicitly
  evaluator verdicts distinguish aligned, under-specified, and semantic drift
  backend-only edits can be classified as meaning-preserving vs meaning-changing
  reviewers can see why a seam failed semantic review without reading all raw Rust first

12-MONTH IDEAL
  semantic review is a first-class trust surface alongside freshness and proof
  AI edits fail fast when authored meaning and executable behavior diverge
  second-backend travel becomes a bounded lowering problem because semantic governance is already honest
```

## Implementation Alternatives

| Approach | What it does | Pros | Cons | Verdict |
|---|---|---|---|---|
| A. Backend-readiness now | Start formal second-backend prep on top of the M14 truth model | Flashiest roadmap story | Widens execution surface before the repo can judge semantic drift honestly | reject |
| B. Semantic governance + eval | Compare authored meaning to executable behavior and project the verdict through truth surfaces | Directly serves the product thesis and current buyer pain | Harder than a simple freshness extension because "under-specified" must be first-class | **chosen** |
| C. Lightweight heuristic lint only | Add string/AST hints without durable verdicts in truth surfaces | Smallest diff | Creates a fake semantic-review story without dependable product semantics | reject |

**Why B wins**
- M14 made freshness honest, which exposed the next real gap: reviewers still cannot tell whether
  code that compiles and passes tests still means what the authored unit says it means.
- The current buyer is Rust-first and review-first. Backend breadth is still a story; semantic
  alignment is the bottleneck in the actual workflow.
- The original product thesis was never "generate more code." It was "make intent, implementation,
  and evidence stay aligned under AI-assisted change."

## What Already Exists

| Sub-problem | Existing code surface | M15 reuse / correction |
|---|---|---|
| Authored-vs-backend truth partition | `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs` | Reuse M14 freshness partitions so semantic review can say whether drift is authored, backend-only, or both. |
| Escape-hatch gating | `spec-core/src/escape_hatch.rs`, passport/export/status markers | Reuse the marker and proof-surface contract, but add semantic leak classification instead of stopping at "escape hatch exists". |
| Canonical seam wedge | `examples/ecommerce/units/pricing/discount_policy.unit.spec`, `pricing/discount_policy_checkout_flow.test.spec` | Reuse the exact M13/M14 wedge and add semantic-alignment pass/fail fixtures instead of inventing a demo seam. |
| Plan and review truth surfaces | `spec-core/src/plan.rs`, export bundle, status JSON | Reuse the existing machine-readable review surfaces instead of inventing a side-channel evaluator output file first. |
| Deferred semantic-eval thesis | `TODOS.md` semantic contract-vs-body item | Close the longstanding product TODO by turning it into the current milestone rather than another deferred note. |

## Architecture Review

M15 is a semantic-governance milestone. The repo already knows how to say "fresh vs stale." What
it lacks is a coherent contract for saying whether the authored semantic story still matches the
executable behavior.

### Ownership split

| Layer | Owns | Must not own |
|---|---|---|
| Authored semantic truth | `intent.why`, contracts, declared deps, seam-owned structure, shared semantics | raw backend-only implementation details masquerading as authored meaning |
| Executable lowering truth | generated Rust shape, lowering bodies, derives, backend-only markers | pretending to define the semantic contract by itself |
| Semantic review truth | evaluator inputs, verdicts, reasons, under-specification markers | silent guesses or one-way "AI says so" authority |

### Truth surfaces

- **Passport** remains the co-located proof record, but it must now carry semantic review output in
  a way that survives the normal trust loop.
- **Status** remains the primary CLI trust loop, but it must project semantic review with a stable,
  human-readable verdict vocabulary instead of burying it in warnings.
- **Export** remains the AI / review bundle, but it must never serialize semantic review as an
  opaque score with no explanation.
- **Plan validation** stays out of the first M15 core unless it needs additive hooks later. The
  evaluator contract should be correct on live units before plans start depending on it.

### System architecture

```text
authored unit / seam
  │
  ├── semantic packet
  │     ├── intent.why
  │     ├── contract / receiver / signatures
  │     ├── deps / imports
  │     └── shared seam structure
  │
  ├── executable packet
  │     ├── generated callable shape
  │     ├── lowering.rust.body
  │     ├── backends.rust markers
  │     └── proof / freshness context
  │
  └── evaluator
        ├── alignment verdict
        ├── reason codes
        ├── under-specification markers
        └── escape-hatch semantic classification
               │
               ├── passport semantic review
               ├── status semantic review
               └── export semantic review
```

### M15 thesis

- A unit can be fresh and still semantically wrong.
- A backend-only change can be legitimate, but only if the evaluator can say why meaning was
  preserved.
- "Under-specified" is a first-class truthful result, not a failure of the system.
- Semantic review must produce a reason a maintainer can act on, not just a scalar score.

### Error & Rescue Registry

| Step | Failure | Detection | Rescue |
|---|---|---|---|
| Build semantic packet | authored truth is too sparse to judge behavior honestly | evaluator marks `under_specified` with missing fields/reason codes | preserve verdict as under-specified and require human review |
| Evaluate function unit | implementation behavior contradicts `intent.why` or `contract` | semantic drift verdict with rationale | fail semantic review truth surface and surface exact mismatch |
| Evaluate seam method | method lowering carries semantics not present in shared seam truth | escape-hatch semantic leak classification | mark drift or under-specification, not silent pass |
| Project status | evaluator returns opaque score without durable reason | status projection contract rejects score-only result | require named verdict + reason set before projecting |
| Export review bundle | export serializes verdict without rationale or context | export schema/assertion tests | attach verdict, reasons, and cited authored/executable surfaces |

### Security & Threat Model

The biggest security-like risk in M15 is not prompt injection. It is false semantic confidence.

- If the evaluator can say "aligned" without pointing at the authored and executable evidence it
  compared, the product will recreate fake-green trust at a higher layer.
- If "under-specified" is treated as pass, the repo will quietly reward vague authored truth.
- If escape-hatch-heavy seams can still be marked aligned without naming what semantics live only in
  lowering, backend travel will be built on a lie.
- M15 should not attempt general semantic proof of arbitrary Rust. That is an ocean. The lake here
  is: classify alignment truthfully, emit actionable reasons, and make uncertainty explicit.

### Data Flow & Interaction Edge Cases

Critical review questions:

- If `intent.why` is broad but the contract is narrow, should the evaluator say aligned,
  under-specified, or drift?
- If only wording changes in `intent.why`, when is that semantic drift versus harmless prose cleanup?
- If only `methods[].lowering.rust.body` changes, when can the system call it meaning-preserving?
- If a seam method depends on another unit whose behavior changed, what part of semantic review is
  local and what part is outside the first M15 boundary?
- If the evaluator cannot tell whether meaning changed, where does that surface and what should a
  reviewer do next?

Required M15 answers:

- Status must distinguish aligned, under-specified, and semantic-drift review states.
- Export must surface the authored fields and executable surfaces the evaluator relied on.
- Escape-hatch seams must be classifiable as meaning-preserving vs meaning-leaking, not just marked.
- Canonical wedges must include both pass and fail semantic-review fixtures.

## Code Quality Review

The quality risk in M15 is not "AI in the product." It is duplicated semantic judgment logic.

M15 must avoid:
- one verdict vocabulary in passports and another in status
- one evaluator path for `kind: function` and a hand-wave for seam kinds
- a single opaque score that cannot be mapped back to authored/executable evidence
- treating "could not tell" as success

One implementation seam should own semantic-eval projection and be reused by:
- passport writing
- status computation
- export projection
- canonical fixture expectations

## Implementation Slices

1. **Semantic alignment contract**
   - Primary modules: new/evolved semantic review helpers in `spec-core`, plus passport/export/status
   - Define one reusable verdict vocabulary:
     - `aligned`
     - `under_specified`
     - `semantic_drift`
     - `backend_only_meaning_preserved`
     - `backend_only_semantics_leaked`
   - Lock reason-code vocabulary once. Do not let passport, status, and export invent competing
     semantics.

2. **Evaluator input builder**
   - Primary modules: `validator`, normalization/truth helpers, seam projection code
   - Build stable semantic packets for `kind: function`, `kind: data`, and `kind: sum`.
   - Make the evaluator consume authored and executable packets, not ad hoc strings assembled per
     surface.

3. **Evaluator result projection**
   - Primary modules: `spec-core/src/passport.rs`, `spec-cli/src/commands.rs`,
     `spec-core/src/export.rs`
   - Persist semantic review into passports.
   - Make `spec status` consume the same contract and project stable verdicts and reasons.
   - Make export project verdict plus rationale, not just a score.

4. **Canonical pass / fail / under-specified wedges**
   - Primary modules: `examples/ecommerce`, evaluator fixtures, CLI/export fixtures
   - Add at least one canonical aligned seam, one semantic-drift seam fixture, and one
     under-specified authored-truth fixture.
   - Reuse `pricing/discount_policy` where possible; add only the minimum extra fixture surface
     needed to prove honest failure.

5. **Escape-hatch semantic leak classification**
   - Primary modules: `escape_hatch`, seam validation, passport/export/status projection
   - Distinguish "backend detail preserved meaning" from "backend-only semantics leaked beyond the
     authored model."
   - Require explicit reasons when a marked seam can still be considered aligned.

6. **Regression + verification pass**
   - Primary modules: `spec-cli/tests/cli.rs`, `spec-core` unit tests, example trust-loop commands
   - Re-run canonical trust loops and verify semantic-review truth surfaces for all three kinds.

## Test Review

### New codepaths

```text
SEMANTIC ALIGNMENT
  - authored semantic packet
  - executable lowering packet
  - alignment verdict + reason projection

UNDER-SPECIFICATION
  - missing or weak authored truth
  - evaluator cannot judge honestly

ESCAPE-HATCH SEMANTIC CLASSIFICATION
  - backend-only meaning preserved
  - backend-only semantics leaked

TRUTH-SURFACE PROJECTION
  - passport semantic review
  - status semantic review
  - export semantic review
```

### Coverage diagram

```text
[+] spec-core semantic review helpers
    ├── build semantic packet from authored truth
    ├── build executable packet from lowering truth
    └── emit stable verdict + reason codes

[+] spec-core/src/passport.rs
    ├── persist semantic review alongside freshness
    └── preserve the last review surface truthfully across non-test writes

[+] spec-cli/src/commands.rs
    ├── status projects semantic verdicts and reasons
    └── non-aligned semantic states stay explicit enough for review

[+] spec-core/src/export.rs
    ├── export emits semantic review with rationale
    └── AI/reviewer consumers can see what evidence the evaluator relied on

[+] examples/ecommerce
    ├── aligned seam fixture
    ├── semantic-drift fixture
    └── under-specified fixture
```

### Required test matrix

- Unit tests:
  - semantic packet builders for `function`, `data`, and `sum`
  - verdict vocabulary and reason-code projection
  - escape-hatch meaning-preserved vs meaning-leaked classification
- CLI integration tests:
  - `status --format json` for aligned semantic review
  - `status --format json` for semantic drift
  - `status --format json` for under-specified authored truth
  - `export` semantic review projection with rationale
- Example-backed tests:
  - canonical `discount_policy` seam aligned verdict under the normal trust loop
  - canonical mismatch fixture fails semantic review for an understandable reason
  - canonical under-specified fixture stays non-green without pretending drift was proven
- Regression tests:
  - M14 freshness semantics remain intact and distinct from semantic review
  - molecule status plane remains separate from unit semantic verdicts
  - all three unit kinds participate in one shared semantic-review contract

## Performance Review

No performance issue should drive M15 scope. The dominant risk is false semantic confidence, not
runtime cost. Semantic review can be slower than freshness projection if it remains deterministic,
explainable, and honest.

## Parallelization / Lanes

M15 is partially parallelizable, but only after the verdict vocabulary and evaluator contract are
locked.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| 1. Semantic alignment contract | semantic review helpers, truth-surface schema | - |
| 2. Evaluator input builder | validator/truth helpers, seam projection | 1 |
| 3. Evaluator result projection | passport, status, export, fixtures | 1, 2 |
| 4. Canonical pass/fail wedges | examples/ecommerce, regression fixtures | 1, 3 |
| 5. Escape-hatch semantic classification | escape_hatch, seam validation, truth surfaces | 1, 2, 3 |
| 6. Final regression + trust-loop verification | `spec-cli/tests`, `spec-core` regressions, example commands | 4, 5 |

### Parallel lanes

- **Gate 0, sequential:** Step 1 must land first. Every other slice depends on the same verdict and
  reason vocabulary.
- **Lane A, evaluator core:** Step 2 -> Step 3
  - Shared modules: semantic packet builders, passports, status, export
  - Keep this lane sequential because the same evaluator contract is reused end to end.
- **Lane B, canonical wedge lane:** Step 4
  - Independent once Gate 0 and the first evaluator/result contract exist
  - Focuses on pass/fail/under-specified example proof
- **Lane C, escape-hatch policy lane:** Step 5
  - Starts after Lane A has the verdict contract
  - Focuses on meaning-preserved vs meaning-leaked policy
- **Lane D, final integration lane:** Step 6
  - Runs last against merged evaluator truth surfaces, canonical wedges, and escape-hatch policy

### Conflict flags

- Keep `spec-cli/tests/cli.rs` mostly out of parallel lanes until the final integration pass. It
  is the natural conflict magnet for M15 too.
- Do not let canonical example work start from provisional verdict names. The example and docs must
  teach the final semantic-review contract, not an intermediate one.
- Avoid a separate evaluator output shape that is not the one passport/status/export finally use.

## Failure Modes

| Codepath | Failure mode | Test coverage required | Error handling | User outcome if missed | Critical gap? |
|---|---|---|---|---|---|
| semantic alignment | code is fresh but semantically wrong and still reports aligned | evaluator + CLI/export fixtures | semantic drift verdict with rationale | fake-green meaning review after real behavior drift | **yes** |
| under-specification | vague authored truth is treated as pass | under-specified fixtures | explicit under-specified verdict | repo rewards ambiguous specs instead of tightening them | **yes** |
| escape-hatch classification | backend-only semantics leak but are marked meaning-preserved | marker + policy fixtures | leaked-semantics verdict with rationale | backend travel starts from dishonest semantic boundaries | **yes** |
| export projection | verdict serialized without rationale | export schema/assertion fixtures | require verdict + reasons + cited surfaces | AI/reviewer consumes opaque semantic score | **yes** |
| kind coverage | evaluator works only for `function` and hand-waves seam kinds | kind-specific packet + status fixtures | shared evaluator contract | the core product thesis applies only to the easiest unit shape | **yes** |

## What NOT in M15 Scope

- second-backend implementation, because the repo still needs one seam that can fail semantic review honestly first
- first-class variant or method graph nodes, because that is a larger ontology change than M15 needs
- full sandboxing or elimination of `lowering.rust.body`, because explicit classification is the lake and containment is the ocean
- autonomous acceptance of a change based solely on evaluator output, because semantic review informs human judgment, it does not replace it
- new seam kinds or wider Rust item coverage, because M15 is about reviewing shipped authored shapes honestly

## Implementation Order

```text
1. Lock the semantic alignment contract and verdict vocabulary
2. Build shared authored/executable packets for function/data/sum
3. Teach passports, status, and export to project semantic review consistently
4. Add canonical aligned / semantic-drift / under-specified wedges
5. Add escape-hatch semantic leak classification
6. Re-run the trust loop and evaluate the post-M15 gate
```

## Success Criteria / Kill Metrics

M15 is successful only if all of these are true:

1. A maintainer can tell whether a unit is semantically aligned, under-specified, or semantically
   drifting.
2. `spec status` and `spec export` tell the same semantic-review story for the same unit.
3. `kind: function`, `kind: data`, and `kind: sum` all participate in one shared evaluator
   contract.
4. The canonical seam can fail semantic review for a reason a reviewer can understand without
   reverse-engineering all raw Rust first.
5. Escape-hatch seams can be classified as meaning-preserving vs meaning-leaking.

Kill the "second backend next" thesis for M16 if either of these happens:
- semantic review still collapses into opaque scoring or unexplained pass/fail output
- the canonical seam still requires reviewers to read raw lowering bodies to localize meaning drift

## Post-M15 Decision Gate

### Choose backend-readiness next if:

- semantic review is explicit, explainable, and honest across all three shipped unit kinds
- status/export/passport agree on semantic-review state
- escape-hatch semantics are classifiable enough that backend travel is a bounded policy question
- the canonical seam can pass and fail semantic review for understandable reasons

### Choose deeper governance next if:

- reviewers still cannot localize meaning drift without reading raw lowering bodies
- under-specification remains the dominant outcome because authored truth is still too weak
- the highest-value missing signal is still cross-unit semantic coherence rather than backend breadth

### Do not choose second backend unless:

- M12 + M13 seam families both pass the M15 semantic-review contract
- the repo can name which meaning is authored, which meaning is executable, and which mismatches are still uncertain
- escape-hatch usage is observable enough that backend travel is a bounded policy question instead of a semantic gamble

## Dream State Delta

If M15 lands cleanly, the project stops asking "can we add another backend soon?" and starts asking
the better question: "can an AI-assisted change fail semantic review honestly before it reaches a
human reviewer too late?"

That is the real leverage. M15 should make meaning reviewable.

## Review-Locked Decisions

- Choose semantic governance + eval for M15, not backend-readiness.
- Keep one top-level unit node per seam in M15. Variants and methods remain nested.
- Additive semantic-review metadata is allowed; first-class nested graph nodes are not.
- Semantic review must be shared across passport/status/export, not reimplemented separately.
- `kind: function`, `kind: data`, and `kind: sum` all stay in scope for one evaluator contract.
- `pricing/discount_policy` remains the canonical wedge and gains semantic pass/fail/under-specified review fixtures.
- Escape hatches remain allowed, but they become semantically classifiable, not just marked.
- Design review is skipped because there is no UI scope.

# M14 — Proof Freshness + Truth Surfaces

Status: **Draft, plan-solidified** (2026-04-21). M13 shipped at `v0.11.0` via `feat: ship M13 sum seams`
(`dca1009`), so this section replaces M13 as the current implementation contract. Source inputs are
the shipped M13 plan and code in `spec-core/src/passport.rs`, `spec-core/src/export.rs`,
`spec-cli/src/commands.rs`, `spec-core/src/plan.rs`, `spec-core/src/validator.rs`,
`examples/ecommerce/units/pricing/discount_policy.unit.spec`, and
`examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`.

This section is the M14 implementation contract. Historical roadmap material below it is record,
not current scope. Two separate implementers should be able to read this section and converge on
the same diff shape and the same proof obligations.

UI scope: **no**. This is a trust-loop and truth-surface milestone for the CLI, plan artifacts,
passports, status, export, and the canonical ecommerce seam.

## Milestone Summary

```text
M14a  Shared-vs-backend proof contract         required
M14b  Freshness-honest passports/status/export required
M14c  Plan acceptance closure against impact   required
M14d  Canonical seam proof expansion           required
M14e  Escape-hatch gate + truth markers        required
M14f  Post-M14 decision gate                   required
```

**Lake to boil in M14**
- Make `green` mean something tighter than "the authored seam still parses and the last test once
  passed."
- Separate three truths the repo currently conflates:
  - shared authored meaning
  - backend-specific lowering / execution behavior
  - observed proof coverage freshness
- Keep the top-level seam node model from M13. Do not turn variants into first-class graph/status
  nodes yet.
- Use the existing `pricing/discount_policy` wedge again. The right M14 proof is not a fresh
  ontology demo. It is proving that one real seam can be changed and reviewed honestly.
- Defer second-backend work until the repo can distinguish "shared meaning changed" from
  "backend lowering changed" without hand-waving.

**User job**
- An AI-heavy Rust maintainer edits one real policy seam, runs
  `spec validate -> spec build -> spec test -> spec status -> spec export`, and can trust the
  resulting artifacts to answer:
  - did the shared seam meaning change?
  - did only backend lowering change?
  - which proof surfaces are now stale or incomplete?
  - did the declared retest set in a `.plan.spec` actually cover the impacted blast radius?

**Actual buyer**
- Primary buyer remains the AI-heavy Rust maintainer reviewing or making policy edits with agent
  help. M14 is not for a hypothetical multi-backend platform buyer yet.

**Painful workflow this milestone must improve**
1. Edit a real seam such as `pricing/discount_policy`.
2. Run the normal trust loop.
3. Decide whether the change affected shared authored meaning, backend-only lowering, or both.
4. Decide whether the available local tests, molecule tests, export bundle, and plan acceptance set
   still prove enough.

If M14 cannot make that workflow more honest and more localized, a second backend will just widen
the blast radius of fake-green confidence.

## Locked Boundary

- M14 adds **no new seam kind** and **no new backend**.
- M14 keeps the M13 top-level seam-node model. One seam still produces one unit row, one passport,
  and one export unit entry.
- M14 must make three truths explicit and reusable across every truth surface:
  - authored seam truth
  - backend / execution truth
  - observed proof freshness
- M14 may add additive metadata to passports, export, status, and plan validation, but it must not
  promote nested variant or method behaviors into first-class graph nodes.
- `spec plan validate --format json` must stop trusting authored acceptance lists at face value.
  It must compare them against computed impact and surface omissions explicitly.
- `pricing/discount_policy` remains the canonical seam wedge. M14 deepens proof on that seam
  instead of introducing a fresh ontology demo.
- Escape hatches remain allowed in M14, but they stop being invisible. They must be marked and
  gated.
- Explicitly not in M14:
  - second-backend implementation
  - first-class variant or method graph nodes
  - full sandboxing or elimination of `lowering.rust.body`
  - semantic LLM contract-vs-body scoring
  - new seam kinds or wider Rust item coverage

## Premises

1. M13 proved that `spec` can represent a second seam shape in Rust. It did **not** prove the
   trust loop is precise enough for backend expansion.
2. The next bottleneck is proof freshness and truth partitioning, not another authored seam kind.
3. The repo should not claim backend-readiness while `methods[].lowering.rust.body` is still the
   primary escape hatch and its semantics are under-specified.
4. M14 should keep one top-level seam node per unit and add additive proof metadata before it even
   considers promoting variants or methods to first-class tracked nodes.
5. The default M15 question after this milestone should become: "is the core now honest enough for
   backend travel?" not "can we paper over honesty gaps with another generator?"

## Dream State

```text
CURRENT (after M13)
  authored seam proves shape
  status proves top-level freshness
  export serializes passports as found on disk
  plan acceptance may understate the real retest set

M14 TARGET
  authored truth, backend truth, and observed proof freshness are explicit
  stale export/passport state is surfaced, not silently serialized as current
  plan acceptance is checked against computed impact
  canonical seam proof covers all meaningful discount branches
  escape hatches are marked and gated before any backend story widens

12-MONTH IDEAL
  shared semantics and backend lowering are cleanly partitioned
  proof artifacts localize what changed and what remains unproven
  a second backend is a bounded lowering problem, not a trust-model rewrite
```

## Implementation Alternatives

| Approach | What it does | Pros | Cons | Verdict |
|---|---|---|---|---|
| A. Backend-readiness gate now | Formalize backend-safe fields and start preparing for another target | Fastest route to the multi-language story | Pretends the current truth model is already honest enough to travel | reject |
| B. Proof freshness + truth surfaces | Tighten status/export/passport/plan honesty around the shipped M13 seam | Directly serves the current buyer and makes future backend work safer | Less flashy than another backend story | **chosen** |
| C. Hybrid "small backend prep" | Mix truth-surface work with a little backend prep | Feels balanced | Usually turns a governance milestone into a fuzzy half-step | reject |

**Why B wins**
- Both outside voices converged on the same pressure: the repo is still better at proving authored
  shape integrity than backend freshness and proof completeness.
- The current buyer is Rust-first. Backend breadth is still a story; proof honesty is a current
  workflow bottleneck.
- M14 can still codify the backend boundary, but only as a gate and marker surface, not as new
  backend implementation.

## What Already Exists

| Sub-problem | Existing code surface | M14 reuse / correction |
|---|---|---|
| Top-level seam truth hash | `spec-core/src/passport.rs::compute_contract_hash` | Reuse the hash machinery, but split or annotate authored-vs-backend freshness instead of treating one top-level digest as the whole truth. |
| Health status | `spec-cli/src/commands.rs::compute_health_status` | Reuse the current status ladder, but teach it to reason about backend/execution freshness and export honesty. |
| Export projection | `spec-core/src/export.rs::build_export_bundle`, `load_passports_for_specs` | Reuse export bundle shape, but stop serializing stale passports as silently current truth. |
| Plan impact | `spec-core/src/plan.rs::build_plan_report`, `SpecGraph::impact` | Reuse computed impact. Add acceptance-vs-impact closure checks instead of trusting authored acceptance lists at face value. |
| Canonical seam wedge | `examples/ecommerce/units/pricing/discount_policy.unit.spec`, `pricing/discount_policy_checkout_flow.test.spec` | Reuse the exact M13 wedge and deepen its proof surface instead of inventing a new example. |
| Escape-hatch trust boundary | `methods[].lowering.rust.body`, `backends.rust`, validator seam rules | Reuse the bounded lowering shape, but make its policy explicit, observable, and reviewable. |

## Architecture Review

M14 is a truth-partitioning milestone. The repo already has the right raw surfaces. What it lacks
is a coherent contract for how those surfaces relate when the authored seam and the executable
backend drift in different ways.

### Ownership split

| Layer | Owns | Must not own |
|---|---|---|
| Authored seam truth | intent, sum/data/function structure, shared signatures, declared deps, seam-owned proof metadata | backend-only implementation detail |
| Backend lowering truth | Rust lowering body, derives, emitted-shape fingerprint, backend markers | pretending to be the only semantic source of truth |
| Observed proof truth | test evidence, stale/fresh markers, plan acceptance closure | hidden assumptions about impact coverage |

### Truth surfaces

- **Passport** remains the co-located unit proof record, but it must stop collapsing authored,
  backend, and observed proof freshness into one opaque story.
- **Status** remains the primary CLI trust loop, but it must project the same freshness contract as
  passport data instead of re-deriving a looser one.
- **Export** remains the AI / review bundle, but it must never serialize stale proof as if it were
  current.
- **Plan validation** remains the proof-closure surface for planned work, but it must compare
  authored acceptance against computed impact instead of trusting the authored list as complete.

### System architecture

```text
authored unit / plan
  │
  ├── authored truth digest
  ├── backend / execution fingerprint
  └── computed impact
        │
        ├── spec test / molecule evidence
        ├── passport freshness state
        ├── status freshness state
        └── export freshness / warning state
```

### M14 thesis

- `status` and `export` must tell the same freshness story about a unit.
- A plan that understates impacted validation or molecule proof is not "valid enough."
- The canonical seam's passport must reflect proof breadth that matches real branch behavior, not
  only the easiest local branch.
- Escape hatches remain allowed, but they stop being invisible.

### Error & Rescue Registry

| Step | Failure | Detection | Rescue |
|---|---|---|---|
| Edit seam lowering only | generated/backend behavior changes while passport still looks current | backend/execution fingerprint mismatch | mark stale and require fresh `spec test` evidence |
| Export after drift | `spec export` serializes a parseable but stale passport as if current | export freshness check against live unit | annotate stale passport or demote to warning |
| Validate plan | acceptance list omits impacted molecule tests | acceptance-vs-impact diff | warn or fail in strict mode with missing proof ids |
| Review canonical seam | branch-specific behavior remains molecule-only and under-localized | branch coverage summary in passport/export | expand atom tests and additive seam proof metadata |
| Expand backends later | backend-only semantics hide inside raw Rust escape hatch | explicit escape-hatch markers + policy gate | block backend expansion until the gate is satisfied |

### Security & Threat Model

The biggest security-like risk in M14 is not adversarial input. It is semantic dishonesty.

- `methods[].lowering.rust.body` is still trusted raw Rust. Today the validator mainly guarantees
  "this parses as a Rust block" for seam lowering. That is fine for M13, but it is too weak for a
  backend-readiness story.
- Escape hatches must become explicit proof state:
  - when a seam uses backend-only semantics, passports/export/status should be able to say so
  - the plan should state what tests are mandatory when that happens
- M14 should not attempt full sandboxing of lowering bodies. That is an ocean. The lake here is:
  mark the seam, gate it, and require proof that matches the escape-hatch class.

### Data Flow & Interaction Edge Cases

Critical review questions:

- If only `methods[].lowering.rust.body` changes, what should go stale?
- If only `backends.rust.derives` changes, what should go stale?
- If the authored seam stays the same but generator behavior changes, which artifact becomes
  untrustworthy first?
- If a `.plan.spec` names too few impacted molecule tests, where does that surface?
- If a seam has three meaningful branches but only one local atom proof, how does a reviewer see
  that gap without reading the raw spec manually?

Required M14 answers:

- Status must distinguish authored-truth freshness from backend/execution freshness.
- Export must never silently present stale passports as current truth.
- Plan validation must surface missing impacted proof, not just unknown proof ids.
- Canonical seam proof metadata must show whether each meaningful branch is covered by atom tests,
  molecule tests, or only implicit coverage.

## Code Quality Review

The repo already has the right direction: one graph surface, one export bundle, one status ladder.
The quality risk is duplicated truth logic drifting apart.

M14 must avoid:
- one freshness rule in `status` and another in `export`
- one proof-closure story in plans and another in review docs
- a second additive seam-proof projection shape that disagrees with passport/export/status

One implementation seam should own freshness projection and be reused by:
- passport writing
- status computation
- export projection
- plan proof-closure checks where relevant

## Implementation Slices

1. **Shared freshness contract**
   - Primary modules: `spec-core` truth helpers, `passport`, `export`, `status`
   - Define one reusable authored-vs-backend-vs-observed freshness model.
   - Lock stale-reason vocabulary once. Do not let passport, status, and export invent competing
     semantics.

2. **Passport + status honesty**
   - Primary modules: `spec-core/src/passport.rs`, `spec-cli/src/commands.rs`
   - Persist the richer freshness state into passports.
   - Make `spec status` consume that same contract and project specific stale reasons for authored
     drift, backend drift, and missing proof.

3. **Export freshness projection**
   - Primary modules: `spec-core/src/export.rs`, export fixtures
   - Make export consume the shared freshness contract and surface stale passports honestly.
   - Prefer explicit stale annotation over silent replay of outdated proof.

4. **Plan proof-closure checks**
   - Primary modules: `spec-core/src/plan.rs`, `spec-core/src/validator.rs`, plan CLI JSON fixtures
   - Compare `acceptance.validate[]` and `acceptance.molecule_tests[]` against computed impact.
   - Surface missing impacted proof deterministically in text and `--format json`.

5. **Canonical seam proof expansion**
   - Primary modules: `examples/ecommerce/units/pricing/discount_policy.unit.spec`,
     `examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`
   - Add direct atom proof for `none`, `percentage`, `fixed_amount`, and capped fixed-amount
     behavior.
   - Project enough proof metadata that a reviewer can tell which branch is directly proven.

6. **Escape-hatch markers + review gate**
   - Primary modules: seam validation, passport/export/status projection, docs
   - Mark backend-only semantics explicitly.
   - Add the policy hook that says escape-hatch use requires the matching proof path before any
     backend-readiness story widens.

7. **Regression + verification pass**
   - Primary modules: `spec-cli/tests/cli.rs`, `spec-core` unit tests, example trust-loop commands
   - Re-run the canonical seam trust loop, plan validation examples, and stale-proof regressions
     against the final contract.

## Test Review

### New codepaths

```text
PROOF FRESHNESS
  - authored truth digest vs backend/execution fingerprint
  - status stale reason projection
  - export stale/fresh projection

PLAN CLOSURE
  - computed impact vs authored acceptance lists
  - missing impacted molecule tests surfacing

CANONICAL SEAM PROOF BREADTH
  - atom coverage for none / percentage / fixed_amount
  - additive proof metadata for seam-localized review

ESCAPE-HATCH GATE
  - marker projection
  - policy enforcement and required proof path
```

### Coverage diagram

```text
[+] spec-core/src/passport.rs
    ├── split or annotate authored-truth and backend/execution freshness
    ├── project additive seam proof metadata
    └── carry escape-hatch markers explicitly

[+] spec-cli/src/commands.rs
    ├── compute_health_status reads the richer freshness state
    └── stale reasons stay specific enough for review

[+] spec-core/src/export.rs
    ├── freshness-aware passport loading / projection
    └── stale passports surface as explicit state or warnings

[+] spec-core/src/plan.rs
    ├── compare acceptance.validate[] to computed impacted units
    └── compare acceptance.molecule_tests[] to computed impacted molecule tests

[+] examples/ecommerce/units/pricing/discount_policy.unit.spec
    ├── add atom tests for percentage path
    └── add atom tests for fixed_amount normal and capped path
```

### Required test matrix

- Unit tests:
  - freshness-projection helpers
  - export freshness annotation / warning behavior
  - acceptance-vs-impact diff logic in plan reporting
  - escape-hatch marker projection
- CLI integration tests:
  - `status --format json` after authored-truth change
  - `status --format json` after backend-only lowering change
  - `export` when a passport is stale
  - `plan validate --format json` when acceptance omits impacted molecule tests
- Example-backed tests:
  - canonical `discount_policy` seam atom coverage for `none`, `percentage`, `fixed_amount`, and
    capped fixed amount
  - canonical seam stale/fresh behavior after targeted edits
- Regression tests:
  - existing function/data seam status semantics remain intact
  - molecule status plane stays separate from unit status unless M14 explicitly widens that rule
  - no divergent freshness logic between passport/status/export

### Test plan artifact

Primary artifact for this pass:
`~/.gstack/projects/atomize-hq-spec/spensermcconnell-main-m14-test-plan-20260421-145220.md`

## Performance Review

No performance issue should drive M14 scope. The dominant risk is semantic dishonesty, not runtime
cost. Recomputing freshness and comparing plan acceptance to computed impact are cheap relative to
the build/test loop already required.

## Parallelization / Lanes

M14 is partially parallelizable, but only after the freshness contract is locked.

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| 1. Shared freshness contract | `spec-core` truth helpers, passport truth model | - |
| 2. Passport + status honesty | `passport`, `spec-cli` status, CLI JSON fixtures | 1 |
| 3. Export freshness projection | `export`, export fixtures | 1 |
| 4. Plan proof-closure checks | `plan` validation, validator diagnostics, plan fixtures | 1 |
| 5. Canonical seam proof expansion | `examples/ecommerce`, example molecule coverage | 2, 3, 4 |
| 6. Escape-hatch markers + gate | seam validation, truth-surface projection, docs | 2, 3, 4 |
| 7. Final regression + trust-loop verification | `spec-cli/tests`, `spec-core` regressions, example commands | 5, 6 |

### Parallel lanes

- **Gate 0, sequential:** Step 1 must land first. Every other slice depends on the same
  authored-vs-backend-vs-observed contract.
- **Lane A, freshness consumers:** Step 2 -> Step 3
  - Shared modules: passport / status / export truth surfaces
  - Keep this lane sequential because the same freshness vocabulary and projection helpers are
    reused end to end.
- **Lane B, proof-closure lane:** Step 4
  - Independent once Gate 0 is locked
  - Focuses on plan impact, acceptance closure, and plan JSON diagnostics
- **Lane C, example + policy lane:** Step 5 -> Step 6
  - Starts after Lanes A and B converge, because the example proof and the escape-hatch gate need
    the final freshness and plan-closure contracts
- **Lane D, final integration lane:** Step 7
  - Runs last against the merged truth surfaces, plan closure, canonical seam proof, and policy
    gate

### Execution order

1. Run Gate 0 sequentially.
2. Launch Lane A and Lane B in parallel worktrees.
3. Merge Lane A and Lane B.
4. Run Lane C on top of the merged contract.
5. Run Lane D last for end-to-end regression and trust-loop verification.

### Conflict flags

- Lane A and Lane B both eventually consume the shared freshness vocabulary. If Step 1 is not
  locked first, they will drift and create merge churn.
- Keep `spec-cli/tests/cli.rs` mostly out of parallel lanes until the final integration pass. It
  is the natural conflict magnet for M14.
- Do not let Lane C start from provisional stale-reason names. The canonical example and docs must
  teach the final contract, not the intermediate one.

## Failure Modes

| Codepath | Failure mode | Test coverage required | Error handling | User outcome if missed | Critical gap? |
|---|---|---|---|---|---|
| freshness projection | backend-only change still reports `valid` | status + passport regression fixtures | explicit stale reason | fake-green proof after real behavior drift | **yes** |
| export projection | stale passport serialized as current truth | export freshness fixtures | stale annotation or warning | AI/reviewer consumes outdated truth | **yes** |
| plan proof closure | `acceptance.molecule_tests[]` understates impacted blast radius | plan validate JSON fixtures | warning or invalid in strict mode | plan says "done" before required proof ran | **yes** |
| canonical seam proof | percentage/fixed branches remain only indirectly proven | atom + molecule wedge tests | explicit proof metadata gap | reviewers overtrust one seam from incomplete local proof | no, but must close in M14 |
| escape-hatch gate | backend-only semantics stay invisible in truth artifacts | marker + policy fixtures | explicit marker and review gate | backend travel starts from dishonest semantics | **yes** |

## What NOT in M14 Scope

- second-backend implementation, because the current trust contract is not honest enough to travel
- first-class variant or method graph nodes, because that is a larger ontology change than M14
  needs
- semantic LLM contract-vs-body scoring, because proof freshness and truth partitioning come first
- full sandboxing or elimination of `lowering.rust.body`, because explicit gating is the lake and
  hard containment is the ocean
- new seam kinds or wider Rust item coverage, because M14 is about reviewing one shipped seam
  honestly

## Implementation Order

```text
1. Lock the shared freshness contract: authored truth, backend/execution truth, observed proof
2. Teach passports and status to project that contract consistently
3. Teach export to surface freshness honestly instead of replaying stale passports blindly
4. Add plan acceptance-vs-impact closure checks
5. Expand canonical discount_policy atom proof coverage
6. Add escape-hatch markers and the M14 review gate
7. Re-run the ecommerce trust loop and plan validation examples, then evaluate the post-M14 gate
```

## Success Criteria / Kill Metrics

M14 is successful only if all of these are true:

1. A maintainer can tell whether a seam edit changed authored meaning, backend lowering, or both.
2. `spec status` and `spec export` tell the same freshness story for the same unit.
3. `spec plan validate --format json` surfaces when authored acceptance lists understate the
   computed blast radius.
4. The canonical `pricing/discount_policy` seam has direct atom proof for all meaningful discount
   branches.
5. Escape-hatch use is explicitly marked and gated in the truth surfaces.

Kill the "second backend next" thesis for M15 if either of these happens:
- backend/execution freshness still cannot be surfaced honestly without collapsing back into one
  opaque top-level hash
- the canonical seam still requires reviewers to read raw Rust to know which branch is unproven

## Post-M14 Decision Gate

### Choose backend-readiness next if:

- authored truth and backend/execution freshness are clearly partitioned
- status/export/passport agree on stale/fresh state
- plan acceptance closure is honest enough to drive implementation review
- escape-hatch policy is explicit, enforced, and teachable

### Choose deeper governance next if:

- branch- or method-local proof still feels too coarse after additive proof metadata
- reviewers still cannot localize semantic drift without reading raw lowering bodies
- the highest-value missing signal is still "the code compiles, but the meaning is wrong"

### Do not choose second backend unless:

- M12 + M13 seam families both pass the M14 freshness contract
- the repo can name which truth is authored, which is backend, and which is observed proof
- escape-hatch usage is observable enough that backend travel is a bounded policy question

## Dream State Delta

If M14 lands cleanly, the project stops asking "can we serialize another backend soon?" and starts
asking the better question: "does the trust loop now tell the truth tightly enough that backend
travel would be honest?"

That is the real leverage. M14 should make green mean trustworthy.

## Review-Locked Decisions

- Choose truth-surface / governance refinement for M14, not backend-readiness.
- Keep one top-level unit node per seam in M14. Variants remain nested.
- Additive proof metadata is allowed; first-class variant nodes are not.
- Freshness must be shared across passport/status/export, not reimplemented separately.
- Plan acceptance must be checked against computed impact instead of trusted as authored gospel.
- `pricing/discount_policy` remains the canonical wedge and gains full atom proof coverage.
- Escape hatches remain allowed, but they become marked, gated, and part of the truth story.
- Design review is skipped because there is no UI scope.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | CEO | Choose M14 truth-surface refinement over backend-readiness | mechanical | P1 choose completeness | Current buyer pain is proof honesty, not another backend story | backend-readiness now |
| 2 | CEO | Keep top-level seam tracking in M14 | mechanical | P5 explicit over clever | Additive proof metadata solves the immediate review gap without redesigning graph ontology | variant-first tracked nodes |
| 3 | CEO | Reuse `pricing/discount_policy` as the canonical M14 wedge | mechanical | P4 DRY | The shipped M13 seam is the right place to prove review honesty | new demo wedge |
| 4 | Eng | Unify freshness logic across passport/status/export | mechanical | P5 explicit over clever | Divergent freshness semantics would recreate fake-green drift in three places | per-surface freshness rules |
| 5 | Eng | Add plan acceptance-vs-impact closure checks in M14 | mechanical | P1 choose completeness | A plan that omits impacted proof is not a trustworthy plan artifact | doc-only guidance |
| 6 | Eng | Make escape-hatch usage observable and gated, not banned outright | taste | P3 pragmatic | The repo already relies on lowering escape hatches; explicit markers and required proof are the lake | full sandboxing or silent trust |

## M14 Review Record (2026-04-21)

### Premise challenge

- The repo already proved `kind: sum` in Rust. It did **not** prove that the trust loop can
  localize or explain freshness once authored truth and backend truth drift separately.
- The buyer for the next milestone is still a Rust maintainer reviewing agent edits. A second
  backend is not yet a buyer problem. It is still a future story.
- The most expensive regret would be widening backend claims while `status` and `export` can still
  overstate freshness or proof completeness.

### CEO dual voices

CODEX SAYS (CEO — strategy challenge)
- Backend-readiness is mostly narrative right now.
- M14 should prove semantic truth and proof freshness for the real buyer before any backend travel.
- The underexplored alternatives are semantic proof localization, export freshness honesty, and an
  explicit escape-hatch gate.

CLAUDE SUBAGENT (CEO — strategic independence)
- The outside subagent agreed on the direction but cited stale worktree internals, so its repo
  specifics were not accepted as authority.
- The durable signal still matched the main review: truth-surface refinement beats backend
  readiness.

CEO DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Premises valid?                  mixed    yes     DISAGREE
  2. Right problem to solve?          yes      yes     CONFIRMED
  3. Scope calibration correct?       yes      yes     CONFIRMED
  4. Alternatives explored enough?    mixed    no      DISAGREE
  5. Competitive / market risks?      mixed    yes     DISAGREE
  6. 6-month trajectory sound?        yes      yes     CONFIRMED
═══════════════════════════════════════════════════════════════

### Eng dual voices

CODEX SAYS (eng — architecture challenge)
- The next bottleneck is semantic truth partitioning, not more codegen breadth.
- `status` and `export` still rely on coarse seam-level freshness, and `methods[].lowering.rust.body`
  remains an under-specified semantic backdoor.
- Backend-readiness is a governance milestone wearing a codegen costume.

CLAUDE SUBAGENT (eng — independent review)
- The subagent confirmed the same pressure from a different angle:
  - freshness is authored-hash only today
  - export can tell a greener story than status
  - plans do not yet enforce proof closure against computed impact
  - canonical seam proof is still too coarse at the branch level

ENG DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Architecture sound?              yes      yes     CONFIRMED
  2. Test coverage sufficient?        no       no      CONFIRMED
  3. Performance risks addressed?     yes      yes     CONFIRMED
  4. Security threats covered?        no       no      CONFIRMED
  5. Error paths handled?             mixed    no      DISAGREE
  6. Deployment risk manageable?      yes      yes     CONFIRMED
═══════════════════════════════════════════════════════════════

### Cross-phase themes

- **Theme: M14 is about proof honesty, not backend breadth.** Flagged independently in CEO and eng
  review.
- **Theme: escape-hatch policy must become explicit before backend travel.** Flagged independently
  in CEO and eng review.
- **Theme: canonical seam proof is still too coarse for review-grade trust.** Flagged independently
  in CEO and eng review.

### Completion summary

| Item | Status |
|---|---|
| Mode selected | SELECTIVE EXPANSION |
| Premise challenge | written |
| What already exists | written |
| Dream state delta | written |
| Error & rescue registry | written |
| Failure modes | written |
| NOT in scope | written |
| Implementation slices | written |
| Test plan artifact | written |
| Parallelization | written |
| Design phase | skipped, no UI scope |
| CEO outside voice | complete |
| Eng outside voice | complete |
| Current status | ready for implementation against this M14 section |
| Verdict | choose truth-surface / governance refinement |

# M13 — Orthogonal Core + Sum Seam

Status: **Draft, plan-solidified** (2026-04-21). Reviewed via `/autoplan` across the 2026-04-20
CEO pass and the 2026-04-21 engineering-solidification pass. Source inputs are the post-M12
office-hours design at
`~/.gstack/projects/atomize-hq-spec/spensermcconnell-main-design-20260420-220723.md`, the earlier
M13 shape study at
`~/.gstack/projects/atomize-hq-spec/spensermcconnell-main-design-20260420-215839.md`, and the
shipped M12 seam architecture already present in `spec-core/src/types.rs`,
`spec-core/src/normalizer.rs`, `spec-core/src/validator.rs`, `spec-core/src/generator.rs`,
`spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-core/src/graph.rs`, and
`spec-cli/src/commands.rs`.

This section is the M13 implementation contract. Historical roadmap material below it is record,
not current scope. Two separate implementers should be able to read this section and converge on
the same diff shape.

UI scope: **no**. This is a CLI/type-system milestone. Any design-review false positives from the
word `form` or `render` in older roadmap text do not count.

## Milestone Summary

```text
M13a  Preflight hardening + compatibility gate   required
M13b  Shared sum model + schema / validator      required
M13c  Rust lowering + enum generation            required
M13d  Seam-level truth surfaces stay honest      required
M13e  Canonical migration wedge + docs           required
M13f  Post-M13 decision gate                     required
```

**Lake to boil in M13**
- `spec` proves the core again with a second seam kind, not with wider Rust item coverage.
- The new seam stays orthogonal by construction: explicit shared variant semantics first, Rust
  enum lowering second.
- The canonical example is a migration of one real pricing choice seam, not a toy `Result<T, E>`
  demo.
- The user-outcome test is explicit: the migrated seam must be easier for an agent to inspect,
  branch on, modify, validate, and prove than the raw Rust file version.
- M13 must leave the project with a cleaner next question: backend-readiness or
  truth-surface/governance refinement.

**User job**
- A Rust maintainer can migrate one real choice-like pricing seam from freehand Rust into one
  authored semantic seam and keep the normal
  `spec validate -> spec build -> spec test -> spec status` loop.
- An AI agent can read one file and see variants, payloads, method signatures, local tests, and
  Rust-specific lowering details without reverse-engineering branching semantics from arbitrary
  `match` blocks.
- The system stays honest about what is and is not first-class: the sum seam is tracked as one
  node now, and variant-local behavior stays nested until a later milestone earns promotion.

**Actual buyer**
- Primary buyer: the AI-heavy Rust maintainer who owns pricing or policy logic and wants agents to
  make safe edits without spelunking through arbitrary enum branches by hand.

**Painful workflow this milestone must improve**
1. Find the handwritten Rust enum or branching policy surface.
2. Infer which variants matter, what payload they carry, and which methods branch on them.
3. Change the logic without breaking unrelated paths.
4. Prove the edit with the normal trust loop.

If M13 cannot make that workflow materially faster or safer, it should fail honestly. The point is
not "another seam kind exists." The point is "real branching policy edits get easier to author,
inspect, and verify."

## Locked Boundary

- M13 adds exactly one new authored top-level kind: `kind: sum`.
- The file extension stays `.unit.spec`. M13 is a new authored shape inside the current unit file
  family, not a parallel artifact type.
- One sum seam file owns one top-level unit ID such as `pricing/discount_policy`.
- Variants are explicit nested members of that seam file, but they are not first-class graph
  nodes, status rows, or passports in M13.
- Shared semantic meaning must be authored in explicit fields:
  - variant IDs
  - variant payload fields
  - method receiver mode
  - method signatures
- Rust-specific authored details are allowed only in namespaced lowering blocks and optional
  backend escape hatches. They may affect lowering only, not shared semantics.
- `lowering.rust.body` remains trusted raw Rust in M13. This milestone measures escape-hatch
  pressure. It does not claim sandboxed safety.
- M13 keeps passport/status/export changes additive-only where possible. If the code can support
  the seam without widening a truth surface, do not widen it just because enums feel special.
- The canonical example is a migration of one real pricing choice seam in `examples/ecommerce`,
  not a greenfield ADT showcase.

**Explicitly not in M13**
- full Rust enum breadth: tuple variants, generic bounds, visibility policy, macros, reprs,
  trait impl authoring, pattern-matching DSLs, or custom derives beyond the existing backend
  escape-hatch shape
- nested variant behaviors as first-class graph nodes, status rows, or passports
- second-language backends
- cross-library seam identity changes
- semantic evals / contract-vs-body scoring
- reverse ingestion, retrieval, or repo intelligence

## Canonical Migration Wedge

### Chosen seam

Use `pricing/discount_policy` as the M13 migration wedge.

Why this wedge:
- It is real pricing logic, not a tutorial prop.
- It pressures the core with mutually exclusive variants and payloads.
- It keeps the domain next to the M12 `checkout_quote` seam, so the example remains teachable.
- It is cross-language in shape. "No discount / percentage / fixed amount" is not Rust-native
  ontology.

### Adversarial calibration

`pricing/discount_policy` is the **teachable** wedge, not the only wedge.

Before schema lock, M13 must include one adversarial calibration pass:
- scan the repo and immediate target domain for the ugliest real branching surface available
- score candidate wedges by business frequency, failure cost, and branching complexity
- evaluate exactly these candidates:
  - `pricing/discount_policy` as the teachable default
  - the branching surface currently embodied by `pricing/checkout_quote`
  - the combined discount-plus-tax policy flow represented today by `pricing/discount_plus_tax`
- score each candidate on a 1-5 scale for:
  - business frequency
  - failure cost
  - branching complexity
  - escape-hatch pressure after mapping to explicit shared semantics
- treat lower escape-hatch pressure as the better score
- choose the highest total score
- tie-break in this order:
  - lower escape-hatch pressure
  - higher branching complexity
  - keep `pricing/discount_policy` if still tied
- record the score table in the M13 implementation notes or PR description
- default outcome for this plan: `pricing/discount_policy` remains the canonical wedge unless one
  of the named candidates beats it by at least 2 total points

If only the teachable wedge works and the adversarial wedge collapses into escape hatches, that is
evidence against the ontology-expansion thesis, not a detail to hand-wave away.

### Raw Rust baseline

The baseline should be one handwritten Rust enum in
`examples/ecommerce/src/raw_baseline/pricing/discount_policy.rs`:

```rust
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq)]
pub enum DiscountPolicy {
    None,
    Percentage { rate: Decimal },
    FixedAmount { amount: Decimal },
}

impl DiscountPolicy {
    pub fn discount_amount(&self, subtotal: Decimal) -> Decimal {
        match self {
            Self::None => Decimal::ZERO,
            Self::Percentage { rate } => subtotal * *rate,
            Self::FixedAmount { amount } => (*amount).min(subtotal),
        }
    }

    pub fn discounted_subtotal(&self, subtotal: Decimal) -> Decimal {
        subtotal - self.discount_amount(subtotal)
    }
}
```

### Authored schema (`kind: sum`)

The first cut should be boringly explicit:

```yaml
id: pricing/discount_policy
kind: sum
intent:
  why: Represent mutually exclusive discount strategies for checkout pricing.
sum:
  variants:
    none: {}
    percentage:
      fields:
        rate:
          type: rust_decimal::Decimal
    fixed_amount:
      fields:
        amount:
          type: rust_decimal::Decimal
methods:
  - id: discount_amount
    intent:
      why: Return the discount amount to subtract from the subtotal.
    receiver: shared_ref
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
      returns: rust_decimal::Decimal
    deps: []
    lowering:
      rust:
        body: |
          {
              match self {
                  Self::None => rust_decimal::Decimal::ZERO,
                  Self::Percentage { rate } => subtotal * *rate,
                  Self::FixedAmount { amount } => (*amount).min(subtotal),
              }
          }
  - id: discounted_subtotal
    intent:
      why: Return the subtotal after applying the selected discount strategy.
    receiver: shared_ref
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
      returns: rust_decimal::Decimal
    lowering:
      rust:
        body: |
          {
              subtotal - self.discount_amount(subtotal)
          }
local_tests:
  - id: fixed_amount_caps_at_subtotal
    expect: DiscountPolicy::FixedAmount { amount: rust_decimal::Decimal::new(2000, 2) }.discounted_subtotal(rust_decimal::Decimal::new(1500, 2)) == rust_decimal::Decimal::ZERO
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
```

`kind: sum` follows the same M12 rule as `kind: data`: shared seam field and contract types must
be fully qualified in the authored spec itself. No top-level `imports`.

### Authoring rules

- `id`, `kind`, and `intent.why` stay required for all unit kinds.
- `kind: sum` requires `sum.variants`.
- `sum.variants` is an ordered map keyed by variant name.
- Each variant may be unit-like (`{}`) or payload-bearing via `fields`.
- The first cut supports **named payload fields only**. No tuple variants.
- Methods remain seam-owned nested behaviors with:
  - `id`
  - `intent.why`
  - `receiver`
  - `contract`
  - optional `deps`
  - backend lowering block
- The first cut supports `receiver: shared_ref` only.
- `local_tests` remain seam-owned and compile inside the generated seam's `#[cfg(test)]` module.
- `backends.rust` is optional and additive only. The first cut supports `derives` there and
  nothing that can redefine shared meaning.

## What Already Exists

| Sub-problem | Existing code surface | M13 reuse / correction |
|---|---|---|
| Kind-aware authored unit parsing | `spec-core/src/types.rs` (`UnitKind`, `NormalizedUnit`, authored extensions) | Add one new top-level kind alongside `function` and `data`. Do not invent a parallel loader. |
| Kind-aware normalization | `spec-core/src/normalizer.rs::normalize_unit`, `spec-core/src/types.rs::NormalizedDataSeam::from_spec` | Mirror the same ownership split for `NormalizedSumSeam`; do not add a parallel loader. |
| Kind-aware semantic validation | `spec-core/src/validator.rs` dispatches `UnitKind::Function` vs `UnitKind::Data` | Extend the same dispatch model to `sum` instead of creating a side pipeline. |
| Rust lowering split | `spec-core/src/generator.rs` already lowers `NormalizedDataSeam` into `RustDataSeamLowering` | Mirror the same ownership split for a Rust enum seam. |
| Kind-aware top-level dep projection | `spec-core/src/graph.rs::top_level_deps`, `spec-cli/src/commands.rs::local_dep_ids` | Replace duplicated `data`-only branching with one shared helper before adding `sum`. |
| Molecule imports over mixed unit kinds | `spec-core/src/generator.rs::covered_unit_use_path`, `generate_molecule_tests_code` | Extend the same import projection path to `sum`, not ad hoc per call site. |
| Top-level truth surfaces | `spec-core/src/passport.rs::build_passport_with_evidence`, `compute_contract_hash`, `spec-core/src/export.rs::build_export_bundle`, `spec-cli/src/commands.rs::compute_health_status` | Keep one seam-level truth loop, but project authored `sum` metadata additively and hash it honestly. |
| Canonical migration wedge pattern | `examples/ecommerce/src/raw_baseline/pricing/checkout_quote.rs`, `examples/ecommerce/units/pricing/checkout_quote.unit.spec`, `examples/ecommerce/README.md` | Reuse the raw-vs-migrated teaching pattern for `discount_policy`, plus parity and molecule coverage. |
| Molecule evidence model | `.test.spec`, `*.test.evidence.json`, molecule status plane | Reuse the same end-to-end proof strategy for the new sum seam. |

## Architecture Review

M13 only stays orthogonal if each layer has one job. The existing repo already proves the failure
mode to avoid: `spec-core/src/graph.rs::top_level_deps`, `spec-cli/src/commands.rs::local_dep_ids`,
and `spec-core/src/generator.rs::covered_unit_use_path` each reconstruct kind-aware truth in
slightly different places. `sum` cannot ship safely on top of that duplication.

### Ownership split

| Layer | Purpose | Must own | Must not own |
|---|---|---|---|
| Raw authored form | Parse YAML into kind-aware authored structs | exact authored shape, file-facing schema, kind dispatch input | normalization shortcuts, Rust generation details |
| Normalized shared seam | shared semantic truth for one sum seam | variant list, payload fields, method signatures, seam-owned local tests | Rust enum syntax, derives, emitted `match` text |
| Rust lowering form | Rust-specific projection of the normalized seam | enum name, Rust variant casing, impl blocks, derives | source-of-truth semantics or hidden overrides |

### Type direction

```text
AuthoredUnit
  ├── FunctionUnitSpec (existing)
  ├── DataSeamSpec     (existing)
  └── SumSeamSpec      (new)

NormalizedUnit
  ├── Function(ResolvedSpec)
  ├── Data(NormalizedDataSeam)
  └── Sum(NormalizedSumSeam)

RustLoweredUnit
  ├── RustFunctionLowering
  ├── RustDataSeamLowering
  └── RustSumSeamLowering
```

**Locked architecture rule:** do not keep stretching the current function-native `ResolvedSpec`
until it secretly becomes an enum carrier. That is Rust-first expansion again.

### Dispatch rule

Centralize seam-kind dispatch in one place per subsystem:
- schema/parser dispatch once on `kind`
- validator dispatch once on normalized unit kind
- generator dispatch once on normalized unit kind
- export/passport/status project kind-aware contract data through the existing top-level truth
  surfaces

### Truth surfaces

M13 keeps the same high-level promise as M12:
- one `.unit.spec` source file
- one top-level unit ID
- one passport record
- one status row
- one export unit entry

The sum seam may contain multiple variants and multiple methods, but those stay nested until real
usage proves seam-level tracking too coarse.

### System architecture

```text
authored .unit.spec
    │
    ├── schema branch (`unit.spec.json`)
    ├── semantic validation (`validator.rs`)
    ├── normalize_unit(...)
    │     └── NormalizedSumSeam
    │
    ├── shared dep / import projection seam
    │     ├── graph edges / export deps
    │     ├── single-file `spec test` closure
    │     └── molecule import generation
    │
    ├── Rust lowering (`RustSumSeamLowering`)
    │     └── enum + impl + local tests
    │
    └── top-level truth surfaces
          ├── passport projection + contract hash
          ├── status health / stale detection
          └── export bundle projection
```

### Error & rescue registry

| Method / Codepath | What can go wrong | Failure class | Rescue action | User sees |
|---|---|---|---|---|
| `spec validate <sum.unit.spec>` | unknown seam fields, bad variant IDs, invalid payload types | schema / semantic validation error | fail fast with stable `SPEC_*` diagnostics | explicit validation failure |
| sum normalization | duplicate callable names, invalid variant map, bad receiver modes | normalization error | fail fast before generation | explicit validation/build failure |
| Rust lowering | invalid derive path, duplicate emitted names, malformed lowering body | generator error | fail fast with context naming seam + method | explicit build failure |
| `spec build` on mixed kinds | sum seam compiles but generated module graph drifts | cargo build failure | stop build, preserve failure evidence path | explicit build failure |
| `spec status` after contract change | stored hash does not match current sum contract | stale status | show stale row, require re-test | explicit `stale` status |

## Implementation Slices

### M13a — Preflight hardening + compatibility gate

Purpose: make M12's teaching surface explicit enough that M13 pressure does not create fake
confidence.

**Files**
- `spec-core/src/graph.rs`
- `spec-core/src/generator.rs`
- `spec-cli/src/commands.rs`
- `spec-cli/tests/cli.rs`
- fixture files as needed under `spec-cli/tests/fixtures/`

**Required work**
- Lock the canonical example posture: raw baseline, migrated seam, docs, and molecule evidence
  move together.
- Codify the M13 escape-hatch rule as an extension of the post-M11 TODO, not a vague future note.
- Add one canonical kind-aware dep/import projection helper so graph edges, exact-unit closure,
  and molecule import generation do not each re-derive top-level deps differently for `sum`.
- Add fixture coverage proving mixed `function` + `data` + `sum` trees report truthful
  validate/status/export/passport behavior.

**Exit condition**
- There is exactly one shared dep/import projection seam reused by graph, CLI exact-unit closure,
  and molecule generation.

### M13b — Shared sum model + schema / validator

Purpose: make `kind: sum` a first-class authored shape without letting Rust dictate the schema.

**Files**
- `spec-core/src/types.rs`
- `spec-core/src/schema/unit.spec.json`
- `spec-core/src/validator.rs`
- `spec-core/src/normalizer.rs`
- focused unit tests in `spec-core/`

**Required work**
- Extend authored types in `spec-core/src/types.rs`.
- Extend JSON schema for `.unit.spec` with an explicit `kind: sum` branch, `minProperties` /
  `required` rules for variants, and `not` guards that keep function-only top-level fields out of
  `sum`.
- Add semantic validation for:
  - ordered unique variant IDs
  - payload field typing
  - collision checks across variants, methods, and generated Rust-emitted type names
  - receiver rules
  - backend-lowering presence rules

**Exit condition**
- `kind: sum` validates as a first-class authored unit shape and rejects invalid mixed surfaces
  before generation.

### M13c — Rust lowering + enum generation

Purpose: lower the shared sum seam into bounded Rust enum output.

**Files**
- `spec-core/src/types.rs`
- `spec-core/src/generator.rs`
- `spec-core/src/normalizer.rs`
- generator-focused tests in `spec-core/`

**Required work**
- Add `NormalizedSumSeam` and `RustSumSeamLowering`.
- Generate Rust `enum + impl`.
- Support seam-owned local tests.
- Route enum lowering, single-file generation scope, and molecule-test imports through the shared
  dep/import projection helper instead of adding a third `kind: data` special case.
- Reject Rust-emitted-name collisions after projection, not only authored-name collisions.

**Exit condition**
- One valid `kind: sum` seam lowers to one readable Rust `enum + impl + #[cfg(test)]` block with
  no new ad hoc kind branches outside the planned seam.

### M13d — Seam-level truth surfaces stay honest

Purpose: keep the trust loop truthful across all three seam kinds.

**Files**
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- regression tests in `spec-core/` and `spec-cli/tests/cli.rs`

**Required work**
- Passport serialization for `kind: sum`.
- Status correctness for mixed trees.
- Export correctness for mixed trees.
- Contract-hash staleness on sum seams.
- Add additive `sum` projection in passport/export carrying ordered variant metadata, payload
  fields, methods, and derives as the exact machine-readable probe for "is seam-level truth too
  coarse?"
- Keep runtime evidence top-level only. Do not add per-variant runtime evidence in M13.

**Exit condition**
- Mixed `function` + `data` + `sum` trees stay truthful across validate, build, test, status,
  export, and passport flows.

### M13e — Canonical migration wedge + docs

Purpose: prove the seam with a real domain example instead of syntax theater.

**Files**
- `examples/ecommerce/src/raw_baseline/pricing/discount_policy.rs`
- `examples/ecommerce/src/raw_baseline/pricing/mod.rs`
- `examples/ecommerce/units/pricing/discount_policy.unit.spec`
- `examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`
- `examples/ecommerce/README.md`
- `README.md` and `AGENTS.md` only if command/workflow teaching must change

**Required work**
- Add the raw Rust baseline `discount_policy.rs`.
- Author `units/pricing/discount_policy.unit.spec`.
- Add local tests plus the molecule test
  `examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec`.
- That molecule test must cover exactly these unit IDs:
  - `pricing/discount_policy` for the new `sum` seam
  - `pricing/checkout_quote` for the existing `data` seam
  - `pricing/apply_tax` for one existing `function` unit
- The molecule body must prove one end-to-end parity story:
  - `DiscountPolicy::Percentage { rate }` and `DiscountPolicy::FixedAmount { amount }` both
    produce discounted subtotals that agree with the generated seam behavior expected by the
    migrated checkout flow
  - `CheckoutQuote` remains aligned with the same discounted subtotal for the percentage path
  - `apply_tax` still composes correctly on top of the discounted subtotal so the mixed-kind proof
    exercises `function` + `data` + `sum` together
- Refresh `examples/ecommerce/README.md`.
- Keep example commands fresh in AGENTS workflow text if needed.

**Exit condition**
- The raw baseline and authored seam stay semantically aligned under tests, and the example teaches
  the migration story without hand-waving.

### M13f — Post-M13 decision gate

Purpose: avoid a fuzzy M14.

**Files**
- `PLAN.md`
- `TODOS.md` only if a deferred item becomes newly explicit

**Required work**
- Keep the trigger table in this plan current.
- Name the two default follow-on paths:
  - backend-readiness gate
  - truth-surface / governance refinement

**Exit condition**
- M14 direction is chosen from evidence gathered in M13, not from roadmap inertia.

## Test Review

### New codepaths

```text
NEW AUTHORED SHAPE
  - parse and validate `kind: sum`
  - normalize variants + payloads + methods

NEW LOWERING PATH
  - lower `NormalizedSumSeam` to `RustSumSeamLowering`
  - generate enum code + impl + local tests

NEW TRUST PATHS
  - passport projection for sum seams
  - status truth for mixed function/data/sum trees
  - export bundle truth for mixed trees

NEW EXAMPLE PATHS
  - raw baseline vs migrated sum seam
  - molecule coverage in ecommerce pricing flow
```

### Coverage diagram

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/types.rs
    ├── add authored + normalized + lowered sum seam structs
    └── add post-projection emitted-name collision checks for enum type / variants / methods

[+] spec-core/src/schema/unit.spec.json + spec-core/src/validator.rs
    ├── explicit `kind: sum` schema branch with `required`, `not`, and empty-map rejection
    ├── variant/payload / receiver / lowering semantic validation
    └── mixed-kind regression tests for invalid authored surfaces

[+] spec-core/src/graph.rs + spec-cli/src/commands.rs + spec-core/src/generator.rs
    ├── shared top-level dep / import projection helper
    ├── single-file `spec test` scope includes local deps for `sum`
    └── molecule imports stay truthful for mixed function / data / sum trees

[+] spec-core/src/generator.rs
    ├── enum lowering happy path
    ├── duplicate emitted-name rejection
    └── seam-owned local tests compile under generated enum

[+] spec-core/src/passport.rs / export.rs / spec-cli/src/commands.rs
    ├── passport projection for sum seam
    ├── status stale/failing/untested for mixed trees
    └── additive authored `sum` projection for the M13 truth-surface probe

USER / AGENT FLOW COVERAGE
===========================
[+] Author choice-like seam
    ├── validate happy path
    ├── invalid variant payload types
    └── invalid lowering body / emitted-name collisions

[+] Build + prove canonical wedge
    ├── raw baseline and migrated seam stay aligned
    ├── local tests on enum seam
    └── `pricing/discount_policy_checkout_flow.test.spec` proves the mixed function / data / sum flow

[+] Trust loop
    ├── exact-unit `spec test units/pricing/discount_policy.unit.spec`
    ├── molecule `spec test units/pricing/discount_policy_checkout_flow.test.spec`
    ├── repo-root and library-root invocation parity
    ├── `spec status` after untouched build
    ├── `spec status` after contract drift without `spec test`
    └── `spec export` mixed-kind truth surface
```

### Required test matrix

- Unit tests:
  - authored type normalization for `kind: sum`
  - validator rejection cases
  - lowering and codegen cases
  - passport/export projection
  - emitted-name collision rejection after Rust projection
- CLI integration tests:
  - `validate --format json`
  - `build`
  - `test` on a directory
  - `test` on one `discount_policy.unit.spec` from repo root and library root
  - `test` on `discount_policy_checkout_flow.test.spec` from repo root and library root
  - `status --format json`
  - `export`
- Example-backed tests:
  - canonical wedge validate/build/test/status loop
  - raw baseline parity checks
  - mixed function/data/sum molecule import path
- Regression tests:
  - existing `kind: function` unaffected
  - existing `kind: data` unaffected
  - mixed tree status/export order and truth
  - top-level dep projection stays identical across graph, CLI scope building, and molecule codegen

### Test plan artifact

Primary artifact for this pass:
`~/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m13-m13-test-plan-20260421-070636.md`

## Performance Review

No material performance issue blocks M13. The existing hot-ish paths,
`ordered_unique_deps`, `local_dep_ids`, graph edge projection, and test-evidence correlation stay
small relative to one new seam kind and one canonical example. Do not widen M13 into a
performance refactor.

## Parallelization / Lanes

M13 is partially parallelizable, but only after the shared kind-aware dep/import seam is locked.

**Gate 0, do this first and sequentially**
- Lock the shared dep/import projection helper and the exact files that consume it.
- Do not start `sum` schema or truth-surface work until this seam exists.

**Lane A, authored-model lane**
- `spec-core/src/types.rs`
- `spec-core/src/schema/unit.spec.json`
- `spec-core/src/validator.rs`
- `spec-core/src/normalizer.rs`
- `spec-core/src/generator.rs`

Scope:
- authored `sum` types
- normalization
- schema validation
- semantic validation
- Rust lowering and enum generation

**Lane B, truth-surface lane**
- `spec-core/src/passport.rs`
- `spec-core/src/export.rs`
- `spec-cli/src/commands.rs`
- focused `spec-cli/tests/cli.rs` coverage for status/export/exact-unit proof

Scope:
- passport projection
- export projection
- status truth
- exact-unit `spec test` closure for mixed kinds

**Lane C, example/docs/regression lane**
- `examples/ecommerce/`
- `spec-cli/tests/cli.rs`
- `README.md`
- `AGENTS.md` only if workflow text must change

Scope:
- canonical wedge
- molecule coverage
- raw-baseline parity
- docs and command examples

**Execution order**
1. Gate 0 sequential.
2. Launch Lane A.
3. After Gate 0 is merged, Lane B may run in parallel with the latter part of Lane A.
4. Merge A + B.
5. Run Lane C last against the merged lowering and truth-surface contracts.

**Conflict rules**
- Lanes A and B may not begin from independent dep/import assumptions.
- Lane C must stay last because it consumes finalized lowering and truth-surface contracts.
- No lane is allowed to invent a second additive `sum` projection shape. Passport and export must
  share the same authored truth story.

## Failure Modes

| Codepath | Failure mode | Test coverage required | Error handling | User outcome if missed | Critical gap? |
|---|---|---|---|---|---|
| dep/import projection | `sum` deps show up in generation but disappear in graph, export, or single-file `spec test` scope | shared projection-helper regression suite across graph / CLI / generator | none today | false-green trust surfaces or missing exact-unit proof inputs | **yes** |
| Rust-emitted naming | authored variants normalize to colliding Rust names only after projection | validator + codegen rejection fixtures | build failure only | explicit compile failure | no |
| top-level trust surface | `compute_contract_hash` omits authored `sum` metadata and stale detection lies | passport hash regression tests | none today | stale edits look valid | **yes** |
| lowering trust boundary | `lowering.rust.body` uses raw Rust escape hatches to carry semantics the shared model does not express | fixture coverage + escape-hatch line-count metric | documented trust boundary only | hidden semantics in backend-only code | no, but tracked kill metric |
| canonical wedge parity | raw baseline and migrated seam drift apart on one branch | local + molecule parity tests | explicit test failure | teachable example becomes fake confidence | no |
| roadmap follow-through | M14 is chosen without explicit trigger evidence | plan/doc review | none | roadmap drift | **yes** |

## What NOT in M13 Scope

- tuple variants, generic enums, trait impl authoring, visibility matrices, macros, repr policy
  because Rust breadth is not the goal here
- variant-level passports, status rows, or graph nodes because top-level seam truth remains the
  M13 contract
- backend-specific lowering sandboxing beyond explicit trust-boundary docs and escape-hatch
  accounting because full Rust-body containment is an ocean, not this lake
- second-backend implementation or cross-library `sum` identity changes because M13 is still
  proving the authored core against Rust
- per-variant runtime evidence because M13 only needs enough additive authored projection to judge
  whether seam-level truth is too coarse
- semantic evals / contract-vs-body scoring because they still sit downstream of trustworthy
  authored surfaces
- reverse ingestion / retrieval / repo intelligence because they are product-adjacent but too early
  for this proof

## Implementation Order

```text
1. Land the shared dep/import projection seam and lock its call sites
2. Add authored `kind: sum` types plus schema/validator/normalizer support
3. Add `NormalizedSumSeam` + `RustSumSeamLowering` + enum codegen
4. Project `sum` truth through passport/export/status/contract-hash surfaces
5. Add the canonical ecommerce wedge, molecule coverage, and parity tests
6. Refresh README / AGENTS workflow text if the new example changes the teaching surface
7. Re-run the full exact-unit + mixed-kind trust loop and evaluate the post-M13 decision gate
```

**Acceptance commands**

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/discount_policy.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_policy.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_policy_checkout_flow.test.spec
cargo run -p spec-cli -- status examples/ecommerce --format json
cargo run -p spec-cli -- export examples/ecommerce/units
```

## Success Criteria / Kill Metrics

M13 is successful only if all of these are true:

1. A maintainer or agent can migrate one real branching pricing seam into `kind: sum` without
   widening into raw-Rust-first authoring for the important branching structure.
2. The migrated seam remains teachable **and** survives one adversarial wedge check without
   collapsing into special pleading.
3. Mixed `function` + `data` + `sum` trees stay truthful across validate, build, test, status,
   export, and passport flows.
4. The canonical example plus its raw baseline stay semantically aligned under test.
5. The sum seam improves the target workflow measurably. Track at least:
   - migration/edit time vs raw Rust baseline
   - agent edit success rate on the canonical seam
   - parity drift count between raw baseline and authored seam
   - escape-hatch line count required to make the wedge work

Kill the "expand ontology first" thesis for M14 if either of these happens:
- the wedge needs too much Rust-specific escape hatch to stay believable
- seam-level truth plus additive `sum` projection still cannot localize which branch is wrong

## Post-M13 Decision Gate

### Choose backend-readiness next if:

- M13 lands without forcing truth-surface redesign
- the escape-hatch boundary still looks contained
- the canonical example stays teachable without special pleading
- the authored core still feels obviously cross-language in shape

### Choose truth-surface / governance next if:

- M13 makes seam-level tracking feel fake or too coarse
- nested behaviors want to become first-class tracked truth
- status/passport/export start looking under-specified for real agent use
- the biggest remaining gap is "it compiles but the meaning is wrong"

### Do not choose second backend unless:

- both M12 and M13 seams lower cleanly from explicit shared semantics
- escape-hatch policy is written and enforced
- the team can name exactly which authored fields are shared-core versus backend-only

## Dream State Delta

If M13 lands cleanly, the project stops asking "can `spec` do anything beyond functions and one
record seam?" and starts asking the better question: "is the next leverage in another backend or
in stronger governance over the authored truth?"

That is the whole game. M13 should turn ontology anxiety into evidence.

## Review-Locked Decisions

- Use `kind: sum`, not `kind: enum`, so Rust does not retake the ontology.
- Keep seam truth top-level in M13. Variants stay nested.
- Use `pricing/discount_policy` as the teachable wedge, but require one adversarial wedge
  calibration pass before schema lock.
- Require one shared dep/import projection seam before any `sum`-specific code lands.
- Define the truth-surface probe as additive authored `sum` projection in passport/export, not
  per-variant runtime evidence.
- Treat `lowering.rust.body` as trusted raw Rust and track escape-hatch pressure explicitly.
- Require exact-unit and mixed-kind molecule regressions from both repo root and library root.
- Skip design review because this milestone has no real UI scope.

## M13 Review Record (2026-04-21)

**CEO pressure now baked into the plan**
- The milestone needed a named buyer and a painful workflow, not just "prove a second seam."
- The wedge needed adversarial calibration so the model can fail honestly if it only works on tidy
  examples.
- The competitive story is workflow speed, trust, and migration ergonomics, not schema novelty.
- The post-M13 gate needed hard kill metrics, not qualitative vibes.

**Engineering pressure now baked into the plan**
- The plan needed one canonical dep/import projection seam or `sum` would drift across graph,
  exact-unit proof, molecule imports, export, and status.
- Rust-emitted-name collision checks needed to happen after projection, not only at the authored
  name layer.
- The truth-surface probe needed an exact additive passport/export shape, not a vague future note.
- The test plan needed exact-unit isolated-generation coverage and mixed-kind molecule coverage
  from multiple working directories.

**Completion summary**
- Architecture direction: **locked** around `kind: sum`, top-level seam truth, and explicit
  lowering separation.
- What already exists: **written**
- Error/rescue registry: **written**
- Failure modes: **written**
- Test review: **written**
- Parallelization: **written**
- Design phase: **skipped, no UI scope**
- Current status: **ready for implementation against this plan section**

# Historical Roadmap (M6–M10)

Status: **M10 Delivered** (2026-04-17). `v0.8.0` ships the first local-library `.plan.spec`
contract, `spec plan validate`, `spec plan export`, and the dedicated plan export bundle.

Reviewed via `/autoplan` 2026-04-16 for the M10 solidification. Codex outside voices consulted;
delegated subagents were unavailable in this thread by session policy. M5 through M9 have
shipped before this branch. This plan now serves as the roadmap record for the shipped M6 through
M10 sequence plus the historical context that constrained it.

---

## Milestone Summary

```
M6a  Trust Gap Fixes          ✓ shipped
M6b  Health Model             ✓ shipped
     structural PR            ✓ shipped
M7   .test.spec + minimal graph ✓ shipped
M8   Full Graph Layer         ✓ shipped
M9   Cross-library Deps       ✓ shipped
M10  Planning Boundary as Data ✓ shipped
```

**Explicitly deferred (do not front-load):**
- TypeScript / Python / Go targets
- Semantic eval / embeddings
- LLM semantic contract-vs-body scoring
- Planning UX
- CUE
- Reverse ingestion

---

## M6a — Trust Gap Fixes

**Theme:** Make the pipeline truthful end-to-end. Close the confirmed bug where spec test
generates code to the wrong location, compiles different code than what it generated, and
produces all-"unknown" test results in passports.

### The Root Cause (confirmed by tracing commands.rs)

Default `--output generated/spec` is relative to CWD. Cargo runs in the resolved crate root.
These are different directories. `spec test examples/ecommerce/units` from the repo root:

```
BEFORE (broken):
  generates to:  {repo_root}/generated/spec/pricing/apply_tax.rs   ← gitignored, disconnected
  cargo sees:    examples/ecommerce/src/generated/                  ← prior run's code
  module prefix: "generated::spec"                                  ← wrong (has ::spec:: segment)
  test names:    "generated::spec::pricing::apply_tax::tests::..."  ← never found in cargo output
  result:        all local tests → status: "unknown"

AFTER (fixed):
  generates to:  {crate_root}/src/generated/pricing/apply_tax.rs   ← cargo sees THIS
  cargo sees:    examples/ecommerce/src/generated/                  ← freshly generated code
  module prefix: "generated"                                        ← derived from strip(crate_root/src/)
  test names:    "generated::pricing::apply_tax::tests::..."        ← found, matched
  result:        local tests → status: "pass"
```

### Changes

**1. Anchor default output to crate root (breaking behavior, correct fix)**

Change the default `--output` convention from CWD-relative `generated/spec` to
`{crate_root}/src/generated`. The crate root is already resolved via `workspace_root_for`
or `pipeline.crate_root` in spec.toml.

- Drop the `spec` subdirectory from the default. It added `::spec::` noise and no convention used it.
- New default: `{crate_root}/src/generated` (relative to resolved crate root, not CWD).
- Update `--output` default_value in all three command arg structs (generate, build, test).

**2. Auto-derive module prefix from output path relative to crate root**

Replace the current `output_module_prefix(output)` derivation (which uses the raw output path)
with derivation from `output.strip_prefix({crate_root}/src/)`:

```
output = {crate_root}/src/generated     →  prefix = "generated"
output = {crate_root}/src/generated/spec → prefix = "generated::spec"
output = {crate_root}/src/api/gen        → prefix = "api::gen"
```

The `src` strip is now anchored to the crate root, not guessed from the first path component.

**3. Add `[pipeline] generated_module_prefix` as explicit override**

For non-standard layouts (e.g., crate imports generated code via re-export rather than
direct `mod`), allow explicit override:

```toml
[pipeline]
generated_module_prefix = "my_custom_name"
```

When present, this overrides auto-derivation. When absent (the common case), auto-derive.

**4. Preserve evidence in write_passports**

Fix the TODOS item: `spec build` and `spec generate` currently overwrite `evidence` and
`contract_hash` fields in passports, silently erasing `spec test` results.

Fix: in `write_passports`, read the existing passport before writing. If the new call
provides `evidence = None` and `contract_hash = None`, carry forward the existing values.

**Important:** this does NOT manufacture false freshness. The 6-state model (M6b) ensures
a rebuilt unit is never shown as `valid` unless:
- `contract_hash` still matches (contract hasn't changed)
- Evidence exists and all tests show `pass` or `ok`

If the contract changed after `spec build`, status = `stale` (hash mismatch). Evidence is
preserved but the stale flag is accurate. M6a ships evidence preservation; M6b ships the
status model that makes it safe.

**5. Thread OutputFormat through pipeline.rs eprintln!**

`run_cargo_build` and `run_cargo_test` emit unconditional `eprintln!` status lines. These
will contaminate machine-readable output if `--format json` is ever added to build/test.
Fix now (XS, clear deadline):

```rust
// before: eprintln!("spec: running cargo build in {}", crate_root.display());
// after:
if matches!(format, OutputFormat::Text) {
    eprintln!("spec: running cargo build in {}", crate_root.display());
}
```

Thread `OutputFormat` parameter through `run_cargo_build` and `run_cargo_test`. One caller
each in commands.rs. No behavior change in Text mode.

**6. Nextest limitation documented**

Add to README under `## Pipeline`:
> `spec test` parses standard `cargo test` output format only. `cargo nextest` uses a
> different output format and is not supported. Running `spec test` in a project configured
> for nextest will produce `status: "unknown"` for all local tests. Use standard `cargo test`.

Close the TODOS item that has been outstanding since M4.

**7. Regenerate example ecommerce passports**

After all fixes land, run `spec test examples/ecommerce/units` and commit the resulting
passports. All local tests should show `status: "pass"` (not `"unknown"`). The committed
passports become a regression artifact proving the trust gap is closed.

### Dependency Order

```
1. Anchor default output + auto-derive prefix  (commands.rs + pipeline.rs/config.rs)
2. Evidence preservation in write_passports    (commands.rs)
3. eprintln! compat in pipeline.rs             (pipeline.rs)
4. Nextest doc                                 (README.md)
5. Regenerate + commit example passports       (examples/)
```

### Success Criteria

- `spec test examples/ecommerce/units` produces passports with all test results `pass`,
  not `unknown`. This is the regression test for the entire trust gap fix.
- A new integration test: `spec test <dir>` with `crate_root` configured correctly →
  `build_test_evidence` maps test names using the auto-derived prefix → results match.
- Existing tests all pass (`cargo test --all`).
- Fixture files updated if output path changes affect JSON snapshots.

### What NOT in M6a Scope

- Status state machine changes (M6b)
- schema_version bump (M6b)
- commands.rs split (structural PR, between M6a and M6b)
- ValidatedExpr newtype (structural PR)

---

## Structural PR (between M6a and M6b)

**Zero behavior change. All tests pass before and after.**

Split `spec-cli/src/commands.rs` (2433 lines) into a module directory:

```
spec-cli/src/commands/
  mod.rs          ← CLI dispatch (Cli::run match arm)
  validate.rs     ← validate_command
  generate.rs     ← generate_command + generate_specs + finalize_passports
  build.rs        ← build_command
  test.rs         ← test_command + build_test_evidence + passport_write_plan
  status.rs       ← status_command
  export.rs       ← export_command
  helpers.rs      ← output_module_prefix, expected_cargo_test_name,
                     cargo_test_filter_for, resolve_git_provenance,
                     rfc3339_now, timeout_suffix, etc.
```

Bundle `D5a ValidatedExpr` newtype into this PR:
- Replace `expect: String` in `ResolvedSpec` with `ValidatedExpr(syn::Expr)` newtype.
- `ValidatedExpr` wraps a parsed `syn::Expr` — eliminates double-parse in `generator.rs`.
- `generate_code` receives `ValidatedExpr`, calls `.into_token_stream()` directly.
- Removes the last gap where a direct `ResolvedSpec` constructor could bypass validation.

**Success criterion:** `cargo test --all` passes before and after. No new behavior.

---

## M6b — Health Model

**Theme:** Make `spec status` a real evidence-health surface, not just validation + staleness.

### 6-State Status Machine

```
  untested     no passport / no evidence field
      │
  incomplete   evidence exists but ≥1 test result is "unknown"
      │
  failing      build_status = "fail" OR "timeout" OR any test_result.status = "fail"
      │   ↘
  stale        contract_hash mismatch (contract changed since last spec test)
  valid        all: build_status pass, all tests pass/ok, hash matches, no unknowns
  invalid      validation errors (schema/semantic), regardless of evidence
```

**Precedence (highest to lowest):** invalid > failing > stale > incomplete > untested > valid

`valid` is only reached when ALL conditions are met: validation clean, build passed,
all test results observed (none "unknown"), all tests pass, contract hash matches.

### JSON Contract Change

This is a breaking change. Bump `schema_version` from 1 to 2.

Old (schema_version 1):
```json
{"status": "stale", "stale": true}
```

New (schema_version 2):
```json
{
  "schema_version": 2,
  "status": "incomplete",
  "reason": "1 local test not observed in cargo output"
}
```

**Migration plan:**
- Old passports (without `schema_version` or with `schema_version: 1`) deserialize with
  backward-compatible serde defaults. The status computation upgrades them on read.
- Mixed-version repos: each unit computes its own status from its own passport.
  No cross-unit version dependency.
- CLI consumers: the JSON `status` string values change (new values: `incomplete`, `untested`,
  `failing`). Bump `schema_version` in `spec status --format json` output so consumers can
  detect the change. Document in AGENTS.md and CHANGELOG.
- Old consumers reading `schema_version: 1` responses: existing `valid/invalid/stale` still
  valid state names. New state names are additive. Old code will see `schema_version: 2` and
  can guard on it.

### Human-readable `spec status` output

```
✓ money/round             valid       evidence:2026-04-12T02:56:17Z
✓ pricing/apply_tax       valid       evidence:2026-04-12T02:56:17Z
~ pricing/apply_discount  stale       contract changed since last test
? shipping/calculate      incomplete  1 test not observed
✗ auth/verify             failing     build failed
— new_unit/foo            untested    no evidence
✗ inventory/check         invalid     2 validation errors
```

### Success Criteria

- Each new state has at least one test that reaches it via a real code path.
- `spec status --format json` emits `schema_version: 2`.
- Fixture files updated for all new status values.
- Old passports still parse correctly (serde backward-compat test).
- AGENTS.md updated: document new state names and schema_version: 2 contract.

---

## M7 — .test.spec + Minimal Graph

**Theme:** First-class molecule tests with declared covers edges. Add just enough graph
structure to represent the unit/test/edge model without over-engineering it.

### .test.spec File Format

```yaml
# pricing.test.spec
id: pricing/checkout_flow
intent:
  why: "Verify discount + tax chain produces correct totals end-to-end."
covers:
  - pricing/apply_discount
  - pricing/apply_tax
  - money/round
imports:
  - rust_decimal::Decimal
  - crate::pricing::apply_discount::apply_discount
  - crate::pricing::apply_tax::apply_tax
body:
  rust: |
    {
      let discounted = apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2));
      let total = apply_tax(discounted, Decimal::new(725, 4));
      assert_eq!(total, Decimal::new(10725, 2));
    }
```

- `id`: same namespace as unit ids, conventionally `{namespace}/test_name`
- `intent`: why this molecule test exists
- `covers`: declared unit ids. spec validates all ids exist in the loaded spec set.
  These are programmer claims, not observed coverage — same epistemic status as `deps`.
- `imports`: optional Rust `use` paths for names the body needs in scope. Omit it only to rely on the temporary deprecated cover-derived fallback.
- `body.rust`: test function body. spec generates a `#[test]` function. This IS code
  injection — spec validates it compiles and the declared coverage/import metadata is coherent; it does not
  validate semantic correctness beyond that.

### Validation Rules

- All ids in `covers` must exist in the loaded spec set. Error: `SPEC_MOLECULE_COVERS_NOT_FOUND`.
- Duplicate `.test.spec` ids are rejected. Error: `SPEC_DUPLICATE_MOLECULE_ID`.
- Body validation: same `is_safe_expr` rules as local test `expect` (block expression,
  no unsafe).
- A `.test.spec` file that declares no `covers` is a warning, not an error.
- A `.test.spec` file that omits `imports` emits a deprecation warning because cover-derived implicit imports are transitional compatibility behavior.

### Generation

`spec generate` and `spec build` process `.test.spec` files alongside `.unit.spec` files.
Each molecule test generates a `#[test]` function in a dedicated `molecule_tests.rs` file
(or per-namespace `{namespace}/molecule_tests.rs`). `covers` is the semantic coverage list.
When `.test.spec` authors provide `imports`, generated Rust uses those imports exactly. When
`imports` is omitted, the generator temporarily falls back to cover-derived implicit imports and
validation emits a deprecation warning so authored molecule tests can migrate cleanly.

### Minimal Graph in spec-core

Rather than raw JSON arrays or a full graph abstraction, introduce a minimal `SpecGraph`
struct in `spec-core` that represents the current loaded world:

```rust
pub struct SpecGraph {
    pub units: Vec<UnitNode>,
    pub molecule_tests: Vec<MoleculeTestNode>,
    pub edges: Vec<SpecEdge>,
}

pub struct UnitNode { pub id: String, pub deps: Vec<String> }
pub struct MoleculeTestNode { pub id: String, pub covers: Vec<String> }

pub enum SpecEdge {
    Dep { from: String, to: String },
    Covers { test: String, unit: String },
}
```

This is not a full graph database. It's a typed representation of what the loader found.
It answers: what units? what molecule tests? what edges? M8 extends this.

### Export

`spec export` includes molecule tests and covers edges:

```json
{
  "schema_version": 2,
  "units": [...],
  "molecule_tests": [
    {
      "id": "pricing/checkout_flow",
      "intent": "...",
      "covers": ["pricing/apply_discount", "pricing/apply_tax", "money/round"]
    }
  ],
  "graph": {
    "edges": [
      {"kind": "dep",    "from": "pricing/apply_tax", "to": "money/round"},
      {"kind": "covers", "test": "pricing/checkout_flow", "unit": "pricing/apply_discount"}
    ]
  }
}
```

### Status Propagation Rule

Molecule test failure does NOT propagate to unit status. A failing molecule test changes
the molecule test's own status (in a future `spec status` extension for molecule tests).
Unit status is determined solely by:
- unit validation
- `spec test` evidence for that unit's local tests
- contract_hash staleness

This avoids the "five units fail because one molecule test failed" ambiguity Codex raised.
Document this boundary explicitly in AGENTS.md.

### Atom/Molecule Boundary

- **Atom tests**: inline `local_tests` in `.unit.spec`. Test one unit's behavior.
  Generated inside the unit's `#[cfg(test)]` module.
- **Molecule tests**: `.test.spec` files. Test interactions between units.
  Generated as standalone `#[test]` functions that call multiple units.
- **The boundary**: if a test needs to import more than one unit, it belongs in `.test.spec`.
  If it tests only the current unit's behavior, it belongs in `local_tests`.

### Success Criteria

- `spec validate`, `spec build`, `spec test`, `spec export` all handle `.test.spec` files.
- `covers` validation rejects unknown unit ids with a stable `SPEC_*` error code.
- Generated molecule test compiles and `cargo test` runs it.
- Export includes `molecule_tests` array and `covers` edges in `graph.edges`.
- At least two molecule tests added to `examples/ecommerce/`.
- Integration tests in `cli.rs` cover: valid molecule test, unknown covers id, generation,
  export shape.

---

## M8 — Full Graph Layer in spec-core

**Theme:** Promote the minimal M7 graph into a first-class **declared relationship contract**
that answers impact questions truthfully. M8 is not an observation system and not a status
engine. It is the clean declared-graph foundation that M9 and M10 can build on.

### Core Questions the Graph Must Answer

```
1. What are all the units?                    → graph.units()
2. What are all the molecule tests?           → graph.molecule_tests()
3. What edges exist (dep + covers)?           → graph.edges()
4. What is the reverse dependency set?        → graph.reverse_deps(unit_id)
5. What molecule tests cover a given unit?    → graph.tests_covering(unit_id)
6. What is the local declared blast radius?   → graph.impact(unit_id)
7. What is the authoritative relationship source? → deps + covers only
8. What export shape should reuse the graph?  → export projects from SpecGraph
```

### graph.build() Contract

`SpecGraph::build(loaded_units, molecule_tests)` constructs the graph from:
- Loaded `.unit.spec` files (units, deps, local_tests)
- Loaded `.test.spec` files (molecule tests, covers edges)

Graph source of truth: **the authored spec files**. In M8:
- `.unit.spec` `deps` are the only authoritative dependency edges
- `.test.spec` `covers` are the only authoritative molecule-test coverage edges
- passports are **not** graph input
- generated Rust is derived and ephemeral, never graph input

`links.molecule_tests` on unit specs is legacy metadata, not relationship truth. **Decision
(locked in M8 eng review 2026-04-15):** `build()` explicitly ignores it with a code comment;
a TODOS entry tracks the follow-up validator warning + field removal. It must not silently
compete with `.test.spec` `covers`.

### Invalidation Rules

The graph is rebuilt on each command invocation from the current spec files. No persistent
graph state between runs. This avoids staleness. The export bundle captures a snapshot.

### Impact Analysis (foundation for M10)

`graph.impact(unit_id)` returns the **local declared retest set** as a structured type:

```rust
pub struct ImpactSet {
    pub units: Vec<String>,          // unit IDs in the retest closure (seed + all reverse deps)
    pub molecule_tests: Vec<String>, // molecule tests covering any unit in that set
}

fn impact(&self, unit_id: &str) -> Option<ImpactSet>
// None  → seed unit not in graph
// Some  → ImpactSet (units always includes the seed; both vecs are sorted)
```

Unit IDs and molecule test IDs share the same string format, so the structured return type
is required to let callers (M10 plan artifact, AI agents) distinguish "units to re-implement"
from "molecule tests to run."

`impact()` returns **unit IDs**, not individual local test cases. The contract is: callers
pass unit IDs to `spec test`, which handles local tests per unit. Local test cases are
implicitly included through the unit ID.

`impact()` is implemented via BFS over `rev_dep_index` with a `HashSet<String>` for
deduplication (handles diamond dependencies). M8: local-library declared impact only.
Advisory planning data, not runtime status.

### API Contract (locked in M8 eng review 2026-04-15)

```rust
// SpecGraph fields are private. Accessor methods are the public API.
// build() assumes validated input (all dep IDs and covers IDs exist in the spec set).

fn units(&self) -> &[UnitNode]
fn molecule_tests(&self) -> &[MoleculeTestNode]
fn edges(&self) -> &[SpecEdge]           // sorted
fn reverse_deps(&self, unit_id: &str) -> Option<Vec<String>>
// None → unit not in graph; Some([]) → exists, no dependents; Some([...]) → sorted dependents
fn tests_covering(&self, unit_id: &str) -> Option<Vec<String>>
// None → unit not in graph; Some([]) → exists, no covering tests; Some([...]) → sorted
fn impact(&self, unit_id: &str) -> Option<ImpactSet>
// None → seed not in graph
```

Internal fields (`rev_dep_index`, `test_coverage_index`) are `HashMap<String, Vec<String>>`,
private to the struct. Export calls `graph.edges()` (not the field directly).

### Implementation Slices (locked for M8)

```text
LoadedSpec + LoadedMoleculeTest
        │
        ▼
SpecGraph::build()
  ├── sorted UnitNode / MoleculeTestNode vectors
  ├── sorted SpecEdge vector
  ├── rev_dep_index: unit_id -> direct dependents
  └── test_coverage_index: unit_id -> covering molecule tests
        │
        ├── accessors: units() / molecule_tests() / edges()
        ├── queries: reverse_deps() / tests_covering() / impact()
        └── export projection through graph.edges()
```

**Slice A. Graph core in `spec-core/src/graph.rs`**

- Keep `SpecGraph::build()` as the single constructor. It accepts validated input and stays infallible in M8.
- Make `units`, `molecule_tests`, and `edges` private. Add private `rev_dep_index` and `test_coverage_index`.
- Build all public vectors in deterministic order during construction:
  - `units` sorted by `id`
  - `molecule_tests` sorted by `id`
  - `edges` sorted lexicographically by enum payload
  - each index vec sorted and deduplicated once during `build()`
- `reverse_deps(unit_id)` returns **direct** dependents only. Transitive closure belongs to `impact()`, not this accessor.
- `tests_covering(unit_id)` returns molecule tests that directly declare the unit in `covers`.
- `impact(unit_id)` performs BFS over `rev_dep_index`, collecting the seed plus all transitive reverse deps, then unions molecule tests covering any unit in that closure.
- `build()` carries an explicit doc comment: "assumes validated input" and "does not read `links.molecule_tests`."

**Slice B. Public surface and file boundaries**

- `spec-core/src/lib.rs`: re-export `SpecGraph`, `SpecEdge`, `UnitNode`, `MoleculeTestNode`, and `ImpactSet`.
- `spec-core/src/export.rs`: remain a projection layer. It may call `graph.edges()`, but it must not read graph internals or serialize index state.
- `spec-core/src/types.rs`: no schema change in M8. `Links.molecule_tests` stays as legacy parsed metadata only; field removal is a later cleanup milestone.

**Slice C. Exact test work required before shipping M8**

- `spec-core/src/graph.rs` unit tests:
  - `reverse_deps_returns_direct_dependents_sorted`
  - `reverse_deps_unknown_unit_returns_none`
  - `tests_covering_returns_multiple_tests_sorted`
  - `tests_covering_unknown_unit_returns_none`
  - `impact_includes_seed_reverse_dep_closure_and_covering_tests`
  - `impact_includes_downstream_covering_tests_not_just_seed_tests`
  - `impact_deduplicates_diamond_reverse_deps`
  - `build_ignores_links_molecule_tests_legacy_metadata`
- `spec-core/src/export.rs` regression test:
  - export still projects sorted `graph.edges()` correctly after graph internals become private and indexed.
- End-of-milestone verification:
  - `cargo test -p spec-core`
  - `cargo test --all`

### Explicit Non-Goals for M8

- No `Declared | Observed` edge taxonomy
- No edge-level runtime evidence
- No `spec status` downstream stale propagation
- No cross-library node metadata (`library_id`, `scope`) before M9 defines typed dep identity
- No export schema growth beyond what current consumers need

### Success Criteria

- `SpecGraph` lives in `spec-core`, exposed from `lib.rs`.
- `SpecGraph::build()` consumes only loaded unit specs and loaded molecule tests.
- `spec export` uses `SpecGraph::build()` — already satisfied by M7 (`export.rs:92`).
- All M7 molecule test / covers edge behavior in `SpecGraph` confirmed as declared graph truth.
- `graph.reverse_deps()`, `graph.tests_covering()`, and `graph.impact()` ship for local-library declared relationships per the API contract above.
- `spec status` remains passport-driven in M8. No downstream stale propagation is added.
- `SpecGraph` fields are private; public API is accessor methods only.
- `ImpactSet` struct is public from `spec-core`.
- Tests cover: build contract, `reverse_deps()`, `tests_covering()`, `impact()` (including the downstream-covering-test case and diamond dedup case), relationship source-of-truth behavior, export projection regression, and unknown-unit-id contracts.
- `build()` doc comment explicitly states "assumes validated input" and "links.molecule_tests is explicitly not read."

### Delivery Status

**Delivered 2026-04-15 in v0.6.0.**

What shipped:
- `SpecGraph` now exposes the declared graph API from `spec-core`, including `reverse_deps()`, `tests_covering()`, and `impact()`.
- `ImpactSet` shipped as the structured return type for local declared blast-radius queries.
- Graph internals are private; export projects through the public graph surface.
- `links.molecule_tests` is explicitly ignored in `build()` as legacy metadata, with follow-up cleanup deferred.
- Graph and export regression coverage landed, including downstream-covering-test and diamond-dedup cases.

Post-ship verification:
- `cargo test --all` passed on the shipped branch.
- `spec export examples/ecommerce/units` emits `schema_version: 2` with 4 units, 2 molecule tests, and 11 graph edges.
- Example ecommerce passports were refreshed after ship so the checked-in regression artifacts now show `pass` rather than `incomplete`.

### M8 /autoplan Review (2026-04-14)

**Review scope:** `PLAN.md` M8 section, grounded against [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:1), [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:1), [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:1), [spec-core/src/passport.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/passport.rs:1), and [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:514).

**UI scope:** No. This is backend and data-model planning only, so Phase 2 design review is skipped.

#### Step 0A. Premise Challenge

1. The real user problem is not "we need a full graph layer." The real user problem is "we need trustworthy impact analysis for cross-library deps and plan artifacts." Right now M8 names the abstraction before it proves the user win.
2. `edge.kind (Declared | Observed)` is not supported by the current evidence model. Passports contain per-unit build and local-test evidence plus `contract_hash`; they do not contain edge-level runtime facts. Shipping "observed" edges in M8 would encode fake precision.
3. `spec status` currently computes truth from validation errors, passport evidence, and contract hash. Using `graph.impact()` to mark downstream stale units is a product-semantics change, not a plumbing cleanup. That deserves its own explicit contract.
4. M9's hard problem is typed cross-library dep identity and cycle truth, not `library_id` on nodes. Front-loading graph metadata before the dep identity model is fixed risks building the wrong foundation.
5. The schema still carries two relationship stories: `.test.spec` `covers` and `links.molecule_tests` on unit specs. M8 should not harden the graph until one relationship source of truth is chosen.

#### Step 0B. What Already Exists

| Sub-problem | Existing code | Reuse / implication |
|---|---|---|
| Declared unit and molecule-test edges | [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:1) | Reuse the current minimal graph as the seed, do not rebuild from scratch. |
| Export graph serialization | [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:83) | Existing consumer proves M8 already has one downstream caller. Keep export as a consumer, not the reason for extra schema growth. |
| Unit health and staleness truth | [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:514) | Reuse current passport-hash status model. Do not silently merge inferred blast radius into this surface in M8. |
| Molecule relationship validation | [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:1835) and [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:543) | Reuse current `covers` validation as the source of declared molecule-test edges. |
| Relationship schema debt | [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:61) | `links.molecule_tests` still exists. M8 must either deprecate or explicitly ignore it. |
| Cross-library dep identity | [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:10) | No typed dep identity exists yet. Current dep strings are local-only. This is an M9 blocker, not something graph metadata can wish away. |

#### Step 0C. Dream State Mapping

```text
CURRENT STATE                         THIS PLAN AS WRITTEN                     12-MONTH IDEAL
Minimal declared-edge graph           Broad "full graph layer" milestone       Trusted impact engine with explicit
used mostly by export.                mixing queries, future metadata,         declared relationships, typed cross-
Status truth lives in passports.      and implied status semantics.            library identities, and evidence-backed
                                      Some planned facts do not exist yet.     observations where instrumentation exists.
```

**Dream state delta:** M8 should move the repo from "graph as export helper" to "graph as trusted declared-relationship query layer." It should not jump all the way to observed edges or downstream status semantics before the evidence model and dep identity model exist.

#### Step 0C-bis. Implementation Alternatives

```text
APPROACH A: Query-Only Layer
  Summary: Keep SpecGraph minimal and private, add reverse lookup helpers over current local IDs.
  Effort:  S
  Risk:    Low
  Pros:    Small blast radius; unlocks impact queries quickly; minimal schema churn.
  Cons:    Leaves M9 to solve typed cross-library identity later; weak long-term contract; risks another rewrite.
  Reuses:  Existing graph.rs, export.rs, validator coverage.

APPROACH B: Contract-First Declared Graph (RECOMMENDED)
  Summary: Promote SpecGraph into a first-class declared-relationship model with explicit node/edge types and query APIs, while deferring observed edges and downstream stale propagation.
  Effort:  M
  Risk:    Medium
  Pros:    Gives M9/M10 a real foundation; avoids fake "observed" precision; keeps status semantics trustworthy.
  Cons:    Requires tighter contract decisions now; forces explicit deferrals in the roadmap.
  Reuses:  Existing SpecGraph, export consumer, current passport-based status model.

APPROACH C: Full Platform Graph Now
  Summary: Ship declared + observed edge taxonomy, cross-library-ready metadata, and status integration in one milestone.
  Effort:  L
  Risk:    High
  Pros:    Ambitious platform story; fewer future public API pivots if guessed correctly.
  Cons:    Encodes facts the repo cannot currently observe; couples M8 to unresolved M9 semantics; highest migration debt.
  Reuses:  Existing graph/export code only as scaffolding.
```

**Recommendation:** Choose **Approach B** because it is the complete version of what M8 can honestly promise today: trusted declared graph answers, not pretend observations.

#### Step 0D. SELECTIVE_EXPANSION Analysis

**Complexity check:** As written, M8 touches at least `spec-core/src/graph.rs`, `spec-core/src/export.rs`, `spec-core/src/lib.rs`, `spec-core/src/types.rs`, `spec-cli/src/commands.rs`, and integration/unit tests. That is already a medium-sized milestone. It should not also absorb status-semantics changes and future evidence concepts.

**Minimum set that achieves the goal:**
- Define the declared graph contract: node kinds, edge kinds, query methods, and rebuild rules.
- Migrate export and M7 molecule-test handling to the declared graph.
- Add `reverse_deps`, `tests_covering`, and `impact` for local-library declared relationships.
- Test the graph queries directly in `spec-core` plus one integration path through export.

**Expansion scan:**
- `library_id` and cross-library edge scope on public node/edge types.
- "Observed" edges sourced from runtime evidence.
- Downstream stale propagation in `spec status`.
- Additional graph queries such as SCC / topological ordering.
- Public export schema widening beyond what current consumers need.

**Cherry-pick decisions (auto-decided per /autoplan principles):**
- **Accepted into M8:** first-class declared graph API, local-library `impact()`, `reverse_deps()`, `tests_covering()`, export migration, and explicit rebuild/no-cache contract.
- **Deferred to M9:** typed cross-library dep identity, `library_id`, cross-library `scope`, and any graph semantics that depend on external libraries.
- **Deferred to later milestone:** observed edges, molecule-test runtime evidence, downstream stale propagation, and any export-schema expansion not needed by a named consumer.

#### Step 0E. Temporal Interrogation

- **HOUR 1 foundations:** decide whether M8's graph is declared-only or declared+observed. This cannot stay fuzzy.
- **HOUR 2-3 core logic:** decide the canonical relationship source. If `.test.spec` `covers` is truth, `links.molecule_tests` must be deprecated or explicitly non-authoritative.
- **HOUR 4-5 integration:** decide whether export consumes public graph structs or a projection. This affects schema churn and consumer stability.
- **HOUR 6+ polish/tests:** decide whether `impact()` is local-library only in M8. If that answer is "yes," the plan must say so plainly or implementers will overbuild for M9.

#### Step 0F. Mode Selection Confirmation

**Selected mode:** `SELECTIVE_EXPANSION`

**Chosen approach under this mode:** `APPROACH B: Contract-First Declared Graph`

**Premise gate outcome:** user selected the contract-first path and explicitly requested that
all cascades into M9 and M10 be reflected in `PLAN.md`.

This keeps the milestone complete, explicit, and honest:
- build the declared graph contract now
- do not ship fake observed edges
- do not mutate `spec status` semantics in the same milestone
- do not hard-block M9 on metadata that only M9 can define correctly

#### CEO Outside Voice

**CLAUDE SUBAGENT (CEO — strategic independence):** unavailable in this run. Session policy for this thread does not allow delegated sub-agents unless the user explicitly asks for delegation.

**CODEX SAYS (CEO — strategy challenge):**

- M8 is currently framed as a platform milestone, but the real unlock is trustworthy impact analysis for M9 and M10.
- `Declared | Observed` edges are premature because passport evidence has no edge-level runtime facts.
- Hard-blocking M9 and M10 on a "full graph layer" is likely over-scoping the abstraction before dep identity is solved.
- Reusing `graph.impact()` to mark downstream stale units would blend inferred blast radius with observed unit health and make `spec status` less trustworthy.
- `links.molecule_tests` remains unresolved schema debt and should not silently coexist with `.test.spec` `covers` as equal graph truth sources.

#### CEO Dual Voices — Consensus Table

```text
CEO DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Premises valid?                   N/A     No     single-model concern
  2. Right problem to solve?           N/A     No     single-model concern
  3. Scope calibration correct?        N/A     No     single-model concern
  4. Alternatives sufficiently explored?N/A    No     single-model concern
  5. Competitive/market risks covered? N/A     Partial single-model concern
  6. 6-month trajectory sound?         N/A     No     single-model concern
═══════════════════════════════════════════════════════════════
```

**Single-model verdict:** strong strategic signal to reframe M8 from "full graph layer" to "declared graph contract + impact queries."

#### NOT in Scope (CEO pass)

- Edge-level observed facts in M8, because the current evidence model cannot produce them truthfully.
- Downstream stale propagation in `spec status`, because that changes product semantics and should not piggyback on graph plumbing.
- Cross-library node metadata in M8, because typed dep identity is an M9 concern and is still undefined.
- Export schema growth beyond what current consumers require, because public schema churn without a named consumer is avoidable debt.

#### Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|-------|----------|----------------|-----------|-----------|----------|
| 1 | CEO | Review M8 as a contract-first graph milestone, not a generic platform rewrite | Taste | P1 + P5 | This preserves the real foundation while avoiding abstractions that the evidence model cannot support yet. | Treat M8 as a full graph platform milestone |
| 2 | CEO | Skip Phase 2 design review | Mechanical | P3 | M8 has no meaningful UI scope; design review would be noise. | Running UI/design review on backend graph planning |
| 3 | CEO | Recommend Approach B over A/C | Mechanical | P1 + P5 | It is the complete version that stays explicit and honest about current repo truth. | Query-only shortcut, full-platform overreach |
| 4 | CEO | Defer observed edges out of M8 | Mechanical | P5 | The repo has no edge-level observation artifact today. | Encoding fake "observed" precision from passports |
| 5 | CEO | Defer downstream stale propagation out of M8 | Mechanical | P3 + P5 | `spec status` currently reports observed unit truth; mixing inferred blast radius would muddy the contract. | Folding status semantics into the graph milestone |
| 6 | CEO | Cascade M8 scope changes into M9 and M10 prerequisites/success criteria | Mechanical | P1 | The roadmap must stay internally consistent or implementation will drift immediately. | Leaving later milestones on the old assumptions |
| 7 | CEO | Keep M10 local-library only even after M9 shipped | Taste | P3 + P5 | The repo has truthful local graph queries today, but not truthful cross-library query semantics. The complete near-term move is to prove the planning contract on one library before widening the blast radius. | Expanding M10 straight into cross-library planning |
| 8 | CEO | Reframe M10 around change intent + derived impact, not a passive YAML note | Mechanical | P1 | The user job is understanding what changed, why, and what else to retest. A file format alone does not solve that job. | Keeping M10 as a thin parseable note format |
| 9 | CEO | Replace authored `impacted` with derived `computed_impact` | Mechanical | P5 | Source and derived data must not share one field or the plan will rot immediately. | Authoring and exporting the same flat `impacted` list |
| 10 | Eng | Make acceptance criteria structured and machine-readable | Mechanical | P1 + P5 | Linking acceptance to unit ids and molecule tests gives AI and humans a real contract instead of YAML-shaped prose. | Free-text-only acceptance strings |
| 11 | Eng | Resolve plan graph scope from the enclosing library root, never from the plan file path | Mechanical | P5 | Existing file-path loaders are intentionally narrow. Reusing them for plans would under-report impact and drop sibling molecule tests. | Reusing single-file spec loading for plan impact |
| 12 | Eng | Define action-sensitive impact semantics: `modify/remove` = current graph, `add` = unknown | Mechanical | P5 | The graph can only answer questions about nodes that already exist. Fabricating impact for `add` would be a lie. | Pretending `graph.impact()` works for all actions |
| 13 | Eng | Use a dedicated `spec plan export` bundle instead of mutating `spec export` in M10 | Taste | P3 + P5 | The existing export bundle is already consumer-facing. A dedicated plan export is the smaller, cleaner first cut while the plan surface is still stabilizing. | Bumping the main export bundle schema for a single-plan feature |

---

## M9 — Cross-library Deps (Contract-First, Repo-Scoped)

**Theme:** Let one spec library reuse units from a sibling spec library in the same git repo
without copy-pasting code, while keeping `spec validate`, generated Rust imports, and export
truthful. M9 is not a package manager, not cross-library planning, and not a graph-query
expansion milestone.

**Milestone verdict:** M9 is the first truthful shared-library slice. It solves direct sibling
library reuse with one identity story across validation, generation, and export. It does **not**
expand planning semantics, graph-query scope, or trust boundaries beyond the repo.

**User job:**
- A root library can author `shared::money/round` and get real validation/build behavior,
  not stringly best-effort.
- A team can split shared units into a sibling spec library without losing trust in
  generated Rust or `spec validate`.
- M10 plan artifacts remain local-library only. Cross-library planning stays deferred.

**Prerequisite:** M8 declared graph contract complete. Do not implement M9 until local
`reverse_deps()` / `tests_covering()` / `impact()` semantics are locked and the graph has a
single source of relationship truth.

### Locked Boundary

- Only direct cross-library deps authored by the root library being validated/generated.
- `[libraries]` targets must resolve inside the same git repo as the invoking library.
- Only the root library's `spec.toml` is authoritative. Imported libraries do **not**
  recursively load their own `[libraries]` entries in M9.
- Cross-library `.test.spec` `covers` are out of scope and rejected loudly.
- `SpecGraph::reverse_deps()`, `tests_covering()`, and `impact()` stay local-library only in M9.
- M10 remains local-library only even after M9 lands.

### What Already Exists

| Sub-problem | Existing code | Reuse / implication |
|---|---|---|
| Author-facing cross-library syntax decision | [DECISIONS.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/DECISIONS.md:56) | Reuse the locked `shared::money/round` syntax. Do not reopen author-facing syntax in M9. |
| Local dep identity and duplicate-id validation | [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:13) and [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:303) | Current dep identity is plain local strings. M9 must add typed identity before it loads multiple libraries. |
| Local graph/export contract | [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:37) and [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:83) | Reuse the public graph/export boundary. Export stays a projection, not a second source of truth. |
| Generated import contract | [spec-core/src/generator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/generator.rs:475) and [README.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/README.md:226) | Local deps already rely on `use crate::...`. Cross-library imports must extend that model without inventing a second identity. |
| Root config loading | [spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1) | Current config lookup is single-root nearest-ancestor. Keep one authoritative root config in M9. |
| Cargo/crate-root truth | [spec-core/src/pipeline.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/pipeline.rs:37) | Reuse the existing "build what Cargo actually sees" principle. M9 must validate the Rust dependency alias before codegen lies. |

### Authoritative Contract

#### `spec.toml`

```toml
[libraries]
shared = "../shared-spec"
payments = "../../payments/spec"
```

The namespace alias is authoritative for:
- authored dep syntax (`shared::money/round`)
- generated Rust import paths (`use shared::money::round::round;`)
- root-scoped graph/export references in M9

M9 does **not** read a target crate's Cargo `[package] name` to invent a second identity.
If the consuming crate wants to import `shared::...`, its `Cargo.toml` must expose a dependency
named `shared`.

```toml
[dependencies]
shared = { path = "../shared-crate" }
payments = { path = "../../payments/crate" }
```

#### Authored dep syntax

```yaml
deps:
  - money/round              # local dep (same library)
  - shared::money/round      # cross-library dep
```

#### Typed identity

```rust
enum DepRef {
    Local { unit_id: String },
    External { library: String, unit_id: String },
}

struct QualifiedUnitRef {
    library: Option<String>, // None = root library, Some("shared") = external alias
    id: String,
}
```

- Local root-library units keep their existing slash-delimited unit ids.
- External refs use the root config's namespace alias plus the unit id.
- Canonicalized filesystem paths are used for trust checks and duplicate-root rejection,
  not as authored ids or generated Rust module names.
- The namespace alias is the only public cross-library identity in M9. Cargo package names,
  canonical paths, and inferred crate names remain implementation details.

### Architecture Review

```text
root spec library
    │
    ├── root spec.toml [libraries]
    │       │
    │       └── repo-scoped library resolver
    │               │
    │               ├── typed DepRef / QualifiedUnitRef
    │               ├── validator + cycle checks
    │               ├── generator import path selection
    │               └── export schema v3 projection
    │
    └── local graph queries remain local-only in M9
```

**Architecture constraints:**
- Root `spec.toml` is the only authoritative `[libraries]` config in M9.
- The same alias must satisfy authored syntax, generated `use <alias>::...` imports, and the
  consuming crate's Cargo dependency name.
- `SpecGraph` may carry typed dep refs internally, but public query semantics remain local-only.
- Recursive library discovery stays out of scope. One authoritative root config keeps validation,
  loading, and cycle detection deterministic.

### Validation

- Unknown library namespace → `SPEC_UNKNOWN_LIBRARY_NAMESPACE`
- Target library path missing on disk → `SPEC_LIBRARY_PATH_NOT_FOUND`
- Target library path escapes the repo root → `SPEC_LIBRARY_OUT_OF_ROOT`
- Alias points back to the root library → `SPEC_LIBRARY_ALIAS_SELF`
- Two aliases resolve to the same canonical library root → `SPEC_DUPLICATE_LIBRARY_ROOT`
- Cross-library dep id not found in target library → `SPEC_CROSS_LIBRARY_DEP_NOT_FOUND`
- Cross-library cycle across the direct library graph → `SPEC_CROSS_LIBRARY_CYCLE`
- Root crate lacks a Cargo dependency keyed by the same alias → `SPEC_LIBRARY_CRATE_ALIAS_MISSING`
- Legacy local deps (`money/round`) continue to work unchanged.
- Duplicate unit ids across different libraries are allowed. Duplicate ids within the same
  resolved library remain errors.

### Generator Contract

- Local deps keep the current `use crate::...` contract.
- Cross-library deps emit `use <alias>::...` where `<alias>` is the namespace key from
  the root library's `[libraries]` config.
- Cross-library callable-name collisions are rejected with a stable error in M9. Automatic
  import alias rewriting is deferred until the authored `body.rust` contract has a story for
  those alias names.

### Graph + Export Contract

M9 is where dep identity becomes typed. It is **not** where cross-library graph queries become
public API.

- Validator, generator, graph, and export all consume the same typed dep IR.
- `SpecGraph` may store typed cross-library dep refs internally, but public query semantics remain
  local-library only in M9.
- `spec export` bumps `schema_version` to 3 and encodes dep endpoints as structured refs:

```json
{
  "kind": "dep",
  "from": {"library": null, "id": "pricing/apply_tax"},
  "to": {"library": "shared", "id": "money/round"}
}
```

Export remains a projection over the public contract. It must not serialize raw graph internals.

### Implementation Plan

**Slice 1. Typed dep identity**
- Primary files: [spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:1), [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:303)
- Add typed dep IR to `spec-core` and normalize authored dep strings once.
- Keep the existing local-only dep path backward compatible.
- Make same-library duplicate-id validation stay local to the resolved library, while allowing
  the same unit id to exist in two different libraries.

**Slice 2. Root-owned library resolution**
- Primary files: [spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1), [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:1797)
- Extend `spec.toml` parsing with `[libraries]`.
- Add a repo-scoped resolver that canonicalizes library roots, rejects out-of-root targets,
  rejects alias-to-self, and rejects duplicate canonical roots.
- Keep only the invoking root library's config authoritative. Imported libraries do not recursively
  widen the graph in M9.

**Slice 3. Validation and cycle truth**
- Primary files: [spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:303), [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:1797)
- Resolve direct external libraries before dep-existence checks run.
- Extend cycle detection to the direct root-library plus imported-library graph.
- Reject cross-library `.test.spec` coverage loudly instead of silently treating it as local.

**Slice 4. Generator and compiler truth**
- Primary files: [spec-core/src/generator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/generator.rs:475), root `Cargo.toml` fixtures/examples
- Emit `use <alias>::...` imports for external deps.
- Validate that the consuming crate exposes the same alias in `Cargo.toml`.
- Reject callable-name collisions across local and external deps with a stable error. Do not try
  to invent automatic import alias rewriting in M9.

**Slice 5. Export and fixtures**
- Primary files: [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:83), spec CLI/export fixtures
- Bump export to `schema_version: 3` with structured dep refs.
- Add mixed local/cross-library dep fixtures and regression tests.
- Keep export a projection over the public typed dep contract. Do not leak graph internals.

**Slice 6. Example repo proof + verification**
- Add an in-repo second spec library and matching crate dependency alias proof.
- Verification commands:
  - `cargo test -p spec-core`
  - `cargo test -p spec-cli`
  - `cargo test --all`

### Parallelization / Lanes

M9 is only partially parallelizable. The first slice is the gate:

- **Gate:** `Slice 1` typed dep identity must land first. Validator, generator, export, and
  resolver work all need the same dep identity contract before they can move safely.

After `Slice 1` lands, split into two lanes:

- **Lane A. Resolution + validation**
  - `Slice 2` root-owned library resolution
  - `Slice 3` validation and direct cross-library cycle truth
  - Primary surfaces: `spec-cli/src/config.rs`, `spec-cli/src/commands.rs`,
    `spec-core/src/validator.rs`

- **Lane B. Generator + export**
  - `Slice 4` generator and compiler truth
  - `Slice 5` export schema v3 and fixtures
  - Primary surfaces: `spec-core/src/generator.rs`, `spec-core/src/export.rs`, export fixtures

Then reconverge for the final integration lane:

- **Lane C. Example proof + regression**
  - `Slice 6` example sibling library, Cargo alias proof, end-to-end regression coverage,
    and milestone verification commands

**Do not parallelize across these boundaries:**
- Do not start Lane A or Lane B before `Slice 1` lands.
- Do not run Lane C until Lane A and Lane B are both merged, because the example proof and
  regression suite need the final validator, generator, and export contracts together.

### Test Review

```text
CODE PATH COVERAGE
===========================
[+] spec-cli/src/config.rs
    ├── parse [libraries] table
    ├── alias-to-self rejection
    ├── duplicate canonical root rejection
    └── out-of-root path rejection

[+] spec-core/src/types.rs / validator.rs
    ├── typed dep IR parsing
    ├── same-library duplicate ids still rejected
    ├── same local id across two libraries allowed
    └── direct cross-library cycle detection

[+] spec-core/src/generator.rs
    ├── external deps emit use <alias>::...
    ├── missing Cargo dependency alias fails loudly
    └── callable-name collisions across local/external deps

[+] spec-core/src/export.rs
    ├── schema_version 3 dep ref encoding
    └── mixed local/cross-library fixture coverage
```

### Failure Modes Registry

| Codepath | Production failure mode | Planned handling | Silent? |
|---|---|---|---|
| `[libraries]` resolution | Path escapes repo root | `SPEC_LIBRARY_OUT_OF_ROOT` | no |
| `[libraries]` resolution | Alias resolves back to root library | `SPEC_LIBRARY_ALIAS_SELF` | no |
| `[libraries]` resolution | Two aliases resolve to the same canonical library root | `SPEC_DUPLICATE_LIBRARY_ROOT` | no |
| dep identity | Two libraries both define `money/round` | Typed `{library?, id}` contract keeps the dep target unambiguous | no |
| generator import path | Config alias does not match Cargo dependency name | `SPEC_LIBRARY_CRATE_ALIAS_MISSING` | no |
| generator import path | Local and external deps share the same callable name | Stable collision error, no auto alias rewriting | no |
| export | Cross-library dep serialized as a plain string edge | `schema_version: 3` structured dep refs | no |
| molecule coverage | External `.test.spec` cover silently treated as local | Dedicated rejection in M9 | no |

### Success Criteria

- `spec validate` accepts `shared::money/round` syntax with `[libraries]` config.
- `[libraries]` targets outside the repo root are rejected loudly.
- Cross-library deps generate `use <alias>::...` imports and fail validation if the root crate
  does not expose that alias in `Cargo.toml`.
- Cross-library cycle detection catches direct A→B→A across library boundaries.
- Export bumps to `schema_version: 3` and represents cross-library dep endpoints without ambiguity.
- Integration tests cover: valid direct cross-library dep, unknown namespace, missing dep,
  missing library path, out-of-root path, alias-to-self, duplicate canonical root, missing Cargo
  dependency alias, and direct cross-library cycle.
- Example project updated with a second spec library in-repo demonstrating the feature.

### Review-Locked Decisions

- Keep M9 as the next milestone, but narrow it to repo-scoped direct deps.
- Make the namespace alias the only public cross-library identity in M9.
- Keep root `spec.toml` authoritative for `[libraries]`.
- Keep cross-library graph queries out of M9.
- Reject cross-library callable-name collisions instead of inventing automatic aliases.
- Bump export to `schema_version: 3` for structured dep refs.

### What NOT in M9 Scope

- Out-of-repo libraries
- Recursive/transitive library discovery
- Cross-library `.test.spec` covers
- Cross-library `reverse_deps()` / `tests_covering()` / `impact()` semantics
- Package-name-derived import identity

---

## M10 — Planning Boundary as Data (Change Intent + Derived Impact)

**Theme:** Ship the first truthful plan contract after M9. M10 is not a planning UI and not
cross-library change intelligence. It is the minimal authored change-set artifact that lets a
human or AI say "these are the units I intend to change" and receive a derived local-library
retest set without scraping prose.

**Milestone verdict:** M10 should prove one clean boundary:
- authored plan source = intended changes + structured acceptance targets
- derived plan output = advisory impact, computed from the current local graph

That keeps planning explicit without pretending the repo already knows future state.

**User job:**
- A developer can author a local refactor plan and immediately see which existing units and
  molecule tests are in the current blast radius.
- An AI agent can parse one file, validate the intended changes, and get a machine-readable
  impact result instead of guessing from filenames and prose.
- The system stays honest about uncertainty: existing units get derived impact, new units do not.

**Prerequisite:** M9 shipped direct cross-library dep truth, but public graph queries are still
local-library only. M10 consumes the current local `SpecGraph` contract exactly as shipped in M8/M9.
If a future milestone wants cross-library plan impact, it must first define truthful
cross-library `reverse_deps()` / `impact()` semantics.

### Locked Boundary

- One plan file at a time. M10 validates or exports a single `.plan.spec` file by explicit path.
- The plan file must live under a resolved spec-library root. Directory-scoped graph loading is
  anchored to that library root, never to the plan file path.
- `changes[].unit` is local-library only in M10. Any authored `shared::...` unit ref is rejected.
- `computed_impact` is derived output only. It is not authored in `.plan.spec`.
- `modify` and `remove` compute current-graph impact. `add` reports `impact_status: unknown`
  unless a later milestone adds future-edge authoring.
- No plan execution, no progress tracking, no status mutation, no planning UI.
- Do not widen the existing `spec export` bundle contract in M10. Plan export gets its own bundle.

### Authored Schema (`.plan.spec`)

```yaml
# checkout-tax-refactor.plan.spec
id: checkout-tax-refactor
intent:
  why: "Refactor tax calculation to support tiered rates without losing checkout coverage."
changes:
  - unit: pricing/apply_tax
    action: modify
    acceptance:
      validate:
        - pricing/apply_tax
      molecule_tests:
        - pricing/checkout_flow
      notes:
        - "tiered-rate behavior is covered by checkout_flow"
  - unit: pricing/tiered_rate
    action: add
    acceptance:
      validate:
        - pricing/tiered_rate
notes:
  - "M10 plans are local-library only."
```

**Authoring rules:**
- `id` is unique per plan file.
- `intent.why` is required.
- `changes` must be non-empty.
- `changes[].unit` must be a valid local unit id, not a cross-library ref.
- `changes[].unit` values must be unique within one plan file.
- `action` is one of `add | modify | remove`.
- `modify` / `remove` require the unit to exist in the current library graph.
- `add` requires the unit id to be absent from the current library graph while still passing
  unit-id syntax validation.
- `acceptance.validate` lists unit ids that must validate when the work is done.
- `acceptance.molecule_tests` lists existing molecule-test ids that must still pass.
- `notes` fields are optional human guidance, not machine-derived truth.

### Derived Impact Output (`validate` / `export` only)

`computed_impact` is the machine-readable answer to "what current work should I re-check?"

```json
{
  "plan_id": "checkout-tax-refactor",
  "computed_impact": {
    "status": "partial",
    "units": ["pricing/apply_tax", "pricing/calculate_total"],
    "molecule_tests": ["pricing/checkout_flow"],
    "unresolved": [
      {
        "unit": "pricing/tiered_rate",
        "action": "add",
        "reason": "current graph has no node for action=add"
      }
    ]
  }
}
```

**Derived-impact contract:**
- `modify` / `remove` use `graph.impact(unit_id)` from the enclosing library root.
- Changed seed units stay in `computed_impact.units`. They are part of the retest set.
- `add` contributes an unresolved entry, not a fabricated impact set.
- Union impact across multiple changes is sorted and deduplicated.
- `computed_impact` is advisory planning data only. It does **not** mutate `spec status`.

### CLI Contract

`spec plan validate <file>`
- accepts one `.plan.spec` file path
- rejects directories
- resolves the enclosing library root before loading units or molecule tests
- validates authored shape plus action-specific rules
- computes per-change and union `computed_impact`
- should support `--format json` from the first cut so agents do not scrape terminal prose

`spec plan export <file>`
- emits a dedicated `PlanExportBundle`, not the existing `ExportBundle`
- includes the authored plan plus derived `computed_impact`
- keeps plan export schema evolution decoupled from the unit export contract

No plan discovery in M10. The caller passes one plan file explicitly.

### Dedicated Export Shape

```json
{
  "schema_version": 1,
  "spec_version": "0.3.0",
  "exported_at": "2026-04-16T00:00:00Z",
  "plan": { "...authored plan..." },
  "computed_impact": { "...derived output..." },
  "warnings": []
}
```

This is intentionally separate from `spec export`. The existing export bundle is already a
consumer-facing contract for units, molecule tests, passports, and graph edges. M10 should not
take on unrelated schema churn just to ship one plan artifact.

### What Already Exists

| Sub-problem | Existing code | Reuse / implication |
|---|---|---|
| Local declared impact queries | [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:49) | Reuse `ImpactSet` as the current-graph truth for `modify/remove`. Do not re-derive impact with ad hoc traversal in CLI code. |
| Workspace + repo boundary knowledge | [spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1) | Reuse resolved workspace and repo roots when anchoring plan scope. M10 should extend that trust boundary, not invent a second one. |
| Validation + JSON diagnostics contract | [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:56) | Mirror the existing `--format json` posture instead of inventing prose-only output. |
| Directory spec loading | [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:2162) | Reuse after adding a dedicated plan-root resolver. File-scoped loading is intentionally too narrow. |
| Molecule test loading | [spec-core/src/loader.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/loader.rs:232) | Reuse for local-library test discovery once the root is resolved. |
| Existing export versioning pattern | [spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:22) | Reuse the versioned bundle pattern, but keep M10 in a dedicated plan export surface. |

### Architecture Review

```text
.plan.spec
    │
    ├── authored change intent
    │       └── validate change ids + actions + acceptance targets
    │
    └── spec plan validate/export
            │
            ├── resolve enclosing library root (canonical, repo-bounded)
            ├── load units + molecule tests from that root
            ├── validate against current local graph
            ├── run graph.impact() per supported action
            └── emit PlanReport / PlanExportBundle
```

**Architecture constraints:**
- Plan scope resolution must reuse the existing workspace-root and repo-root truth from
  [spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1),
  but with a dedicated plan-root resolver instead of the current single-file spec loader.
- The plan layer consumes the current public `SpecGraph` contract from
  [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:56),
  not graph internals.
- Symlink traversal and out-of-root paths must be rejected or skipped explicitly during
  plan-root scanning. M10 cannot widen trust boundaries by accident.

### Error & Rescue Registry

| Scenario | What fails | User-visible rescue |
|---|---|---|
| Plan file sits outside any resolved library root | The command cannot know which units/tests define the local graph | Fail with an explicit path-to-root error and tell the caller to move the plan under a library root or pass a path inside one. |
| `changes[].unit` names a missing unit with `action=modify/remove` | The derived impact would be fiction | Fail validation with a stable machine code. No fallback. |
| `changes[].unit` names an already-existing unit with `action=add` | The authored intent conflicts with current graph truth | Fail validation and show the existing unit id. |
| `action=add` asks for impact on a not-yet-existing node | The graph has nothing truthful to traverse | Return `unresolved[]` with `reason`, keep the rest of the plan valid, and mark the overall impact `partial`. |
| Plan consumer wants one machine-readable bundle | Reusing `spec export` would create unrelated schema churn | Emit a dedicated `PlanExportBundle` from `spec plan export`. |

### Code Quality Review

- Keep the first cut explicit. Do not front-load a CLI refactor just to make room for `spec plan`.
  The command can land in the current CLI surface and move later if the command split happens.
- Keep authored plan types and derived-impact types separate. `computed_impact` must be derived
  data, not a field round-tripped through author input.
- Reuse existing JSON error and warning patterns. M10 is a new command surface, not a second
  diagnostics dialect.
- Prefer small dedicated plan types over widening generic export or graph types prematurely.
  The plan contract is new. The graph contract is already shipped.

### Implementation Slices

1. **Plan schema + parser contract**
   - Add typed `.plan.spec` structs for authored fields only.
   - Validate required keys, unique `changes[].unit`, and action enum shape before touching the graph.

2. **Plan-root resolution**
   - Resolve the enclosing library root from the plan file path.
   - Load the full local library spec set from that root, not from the plan file directory.
   - Reject directory input for `spec plan validate/export`; M10 is single-file invocation only.

3. **Action-sensitive validation + derived impact**
   - `modify/remove` require an existing local node and call `graph.impact(unit_id)`.
   - `add` requires a syntactically valid but currently missing unit id and emits unresolved impact.
   - Union and dedupe the per-change `ImpactSet` results deterministically.

4. **CLI contract + JSON output**
   - Add `spec plan validate <file>` with text and `--format json`.
   - Return stable machine-readable validation failures and a structured `computed_impact` payload.

5. **Plan export + docs**
   - Add `spec plan export <file>` with a dedicated versioned bundle.
   - Document the schema in AGENTS.md and README-level machine-readable docs.
   - Keep the existing `spec export` surface untouched.

6. **Regression suite**
   - Add integration tests for root resolution, symlink escape handling, cross-library rejection,
     add/modify/remove action semantics, and deterministic impact union/export ordering.

### Test Review

**Test diagram**

| Codepath / behavior | Test layer | Required coverage |
|---|---|---|
| Parse one `.plan.spec` file and reject directories | CLI integration | `spec plan validate <dir>` fails cleanly; `spec plan validate <file>` succeeds on a valid plan. |
| Resolve enclosing library root from nested plan path | CLI integration | Nested plan file still loads sibling units and molecule tests from the enclosing library root. |
| Validate `modify/remove` against current graph truth | CLI integration + unit | Missing unit id fails with a stable code; existing local unit id passes. |
| Validate `add` against absence-in-graph truth | CLI integration + unit | Existing unit id with `add` fails; missing id yields unresolved impact, not fabricated impact. |
| Reject cross-library `changes[].unit` refs | CLI integration | `shared::pricing/apply_tax` fails loudly in M10. |
| Derive union impact deterministically | spec-core unit + CLI integration | Changed seed units remain in the set, downstream units dedupe, molecule tests dedupe, ordering is stable. |
| Protect root/repo boundary on scan | CLI integration | Symlink escape or out-of-root path is rejected or skipped explicitly with warning/error coverage. |
| Export one plan bundle | CLI integration + fixture | Bundle schema, version, ordering, warnings, and `computed_impact` shape stay stable. |

**Test artifact:** [spensermcconnell-main-m10-test-plan-20260416-191129.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-m10-test-plan-20260416-191129.md)

### Performance Review

- The expensive operation in M10 is graph loading, not impact traversal. Keep work scoped to one
  resolved library root and build the graph once per invocation.
- `graph.impact()` already returns sorted, deduped `ImpactSet` data. Reuse it instead of
  recomputing traversals per export projection.
- Root scanning must stay repo-bounded. A fast command that silently walks outside the repo is
  worse than a slower truthful one.
- No caching layer in M10. The local-library graph is small enough, and caching would make root
  correctness harder to reason about in the first cut.

### Parallelization / Lanes

M10 is partially parallelizable, but only after the contract gate is locked.

**Gate 0, do this first and sequentially**
- Lock the authored schema, derived-impact shape, and plan-root resolution rules in the code and
  docs before splitting work.

**Lane A, spec-core contract lane**
- Plan structs and derived-impact types
- Plan export bundle + serializer
- Unit tests for action semantics and deterministic impact projection

**Lane B, spec-cli command lane**
- `spec plan validate/export` command wiring
- Plan-root resolver
- Validation diagnostics and `--format json`

**Join lane, run after A and B land**
- End-to-end integration tests
- README + AGENTS.md updates
- Fixture refresh and final CLI shape polish

**Do not parallelize across these boundaries**
- Do not let both lanes invent their own plan result types. The shared data contract is the gate.
- Do not start export fixtures before the validation payload and bundle schema are locked.
- Do not widen M10 into cross-library impact while Lane B is in flight. That collapses back into a
  sequential post-M9 graph-query milestone.

### Failure Modes

| Codepath | Failure mode | Test covers? | Error handling? | Silent? |
|---|---|---|---|---|
| plan root resolution | plan file outside any resolved library root | no | fail with explicit path/root error | no |
| plan root scan | symlink escapes the library or repo root | no | reject with `SPEC_PLAN_SYMLINK_ESCAPE` | **critical gap** |
| single-file invocation | graph built from the plan file path instead of the library root | no | dedicated resolver required | **critical gap** |
| `computed_impact` projection | authored and derived impact shapes drift | no | derived-only contract | **critical gap** |
| `action=add` | fake impact reported for a unit that is not yet in the graph | no | unresolved entry + partial status | no |
| plan export | existing unit export schema churns for one new artifact | yes (by contract choice) | separate bundle | no |
| conflicting changes | same unit listed twice with incompatible actions | no | fail validation | no |

### What NOT in M10 Scope

- Cross-library plan changes or cross-library impact queries
- Plan execution, task tracking, or planning UI
- Future-edge authoring for `action=add`
- Automatic plan discovery during `spec export`
- Local-test-level acceptance target identity

### Implementation Order

```text
1. Lock plan schema, derived-impact shape, and root-resolution contract
2. Implement plan structs + command parsing
3. Implement plan-root resolver and graph loading from enclosing library root
4. Implement action-sensitive validation and ImpactSet projection
5. Add `spec plan validate --format json`
6. Add dedicated `spec plan export` bundle
7. Land integration tests, fixtures, and docs
8. Re-review before widening scope beyond local-library truth
```

### Success Criteria

- `spec plan validate <file>` accepts one `.plan.spec` file and rejects directories.
- Plan validation resolves the enclosing library root before loading the graph.
- `modify` / `remove` require an existing local unit id.
- `add` requires a missing local unit id and reports derived impact as unresolved/unknown.
- Cross-library unit ids in `changes[].unit` are rejected in M10.
- `computed_impact` is derived-only, structured as `{status, units, molecule_tests, unresolved}`.
- `spec plan export <file>` emits a dedicated versioned plan export bundle.
- Schema is documented in AGENTS.md and README-level machine-readable docs, not only agent prompts.
- Integration tests cover:
  - valid local-only modify plan
  - valid mixed modify/add plan
  - unknown unit id for `modify`
  - duplicate/conflicting `changes[].unit`
  - cross-library unit ref rejected in a plan
  - single-file nested plan path still loads the full library graph
  - symlink escape rejected with `SPEC_PLAN_SYMLINK_ESCAPE`
  - impact union includes downstream molecule tests and keeps changed seed units
  - plan export schema/version behavior

---

## M10 Review Record (2026-04-16)

`/autoplan` was run against the refreshed M10 scope and grounded against
[docs/north_star_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/north_star_v0.2.md:101),
[docs/high_level_technical_architecture_v0.2.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/high_level_technical_architecture_v0.2.md:102),
[docs/roadmap_and_release_shape_v0.1.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/docs/roadmap_and_release_shape_v0.1.md:413),
[spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:56),
[spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:22),
[spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1),
[spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:56),
and [spec-core/src/loader.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/loader.rs:232).

Outcome:
- CEO correction: M10 should solve change intelligence for one library, not merely introduce a
  file extension.
- Eng correction: root resolution, symlink boundaries, action-sensitive impact semantics, and a
  dedicated plan export contract must be explicit in the milestone, not left to implementer taste.
- Design review skipped, no UI scope.
- Outside voice: Codex ran twice (CEO + Eng). Delegated subagents were unavailable in this thread
  by session policy.
- Test artifact: [spensermcconnell-main-m10-test-plan-20260416-191129.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-m10-test-plan-20260416-191129.md)
- Review-time taste choices are now resolved in the milestone text above:
  keep M10 local-library only, and ship a dedicated plan export bundle.

### CEO Dual Voices — Consensus Table

```text
CEO DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Premises valid?                   N/A     Partial single-model concern
  2. Right problem to solve?           N/A     No      single-model concern
  3. Scope calibration correct?        N/A     Partial taste disagreement
  4. Alternatives sufficiently explored?N/A    No      single-model concern
  5. Competitive/market risks covered? N/A     Partial single-model concern
  6. 6-month trajectory sound?         N/A     Partial taste disagreement
═══════════════════════════════════════════════════════════════
```

**CODEX SAYS (CEO — strategy challenge):**
- Do not ship YAML theater. M10 must change how developers and AI understand intended change.
- Free-text acceptance and authored `impacted` lists would rot immediately.
- The roadmap is more credible if M10 proves a local-library planning contract first, then opens a
  separate cross-library change-intelligence milestone.

**CLAUDE SUBAGENT (CEO — independent review):** unavailable in this run. Session policy for this
thread does not allow delegated sub-agents unless the user explicitly asks for delegation.

### Design Review

Skipped, no UI scope. M10 is a CLI/data-artifact milestone.

### ENG Dual Voices — Consensus Table

```text
ENG DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Architecture sound?               N/A     Partial single-model concern
  2. Test coverage sufficient?         N/A     No      single-model concern
  3. Performance risks addressed?      N/A     Yes     single-model positive
  4. Security threats covered?         N/A     No      single-model concern
  5. Error paths handled?              N/A     No      single-model concern
  6. Deployment risk manageable?       N/A     Yes     single-model positive
═══════════════════════════════════════════════════════════════
```

**CODEX SAYS (eng — architecture challenge):**
- Reusing single-file loaders for `spec plan validate <file>` would under-report sibling units and
  molecule tests.
- The plan layer widens a real trust boundary unless root-scoped path resolution and symlink
  handling are made explicit.
- `action=add` cannot truthfully use current-graph impact and must report uncertainty explicitly.
- Plan export needs a stable bundle contract now, not an implied future schema bump.

**CLAUDE SUBAGENT (eng — independent review):** unavailable in this run. Session policy for this
thread does not allow delegated sub-agents unless the user explicitly asks for delegation.

### Cross-Phase Themes

- **Truth before convenience** — both passes converged on the same rule: do not author or export
  derived impact as if it were source truth.
- **Scope from roots, not files** — both passes independently pushed the same implementation
  constraint: plan validation must resolve the library root first or it will lie.

### NOT in Scope (M10 pass)

- Cross-library plan changes or cross-library impact queries
- Plan execution, task tracking, or planning UI
- Future-edge authoring for `action=add`
- Automatic plan discovery during `spec export`
- Local-test-level acceptance target identity

### Completion Summary

```text
  +====================================================================+
  |                M10 /autoplan REVIEW — COMPLETION SUMMARY           |
  +====================================================================+
  | Mode selected        | SELECTIVE_EXPANSION                         |
  | Premise gate         | implicit via "solidify M10 after M9 landed" |
  | Section 1  (Arch)    | 4 contract issues fixed in-plan             |
  | Section 2  (Errors)  | failure modes updated, 3 critical gaps      |
  | Section 3  (Security)| 2 path/root boundary issues named           |
  | Section 4  (Data/UX) | skipped, no UI scope                        |
  | Section 5  (Quality) | 3 schema/contract drift issues fixed        |
  | Section 6  (Tests)   | diagram + QA artifact produced              |
  | Section 7  (Perf)    | no new runtime hotspot beyond root scan     |
  | Section 8  (Observ)  | skipped, no runtime surface in M10          |
  | Section 9  (Deploy)  | no deploy surface                           |
  | Section 10 (Future)  | post-M10 cross-library follow-on named      |
  | Section 11 (Design)  | SKIPPED (no UI scope)                       |
  +--------------------------------------------------------------------+
  | NOT in scope         | written (5 items)                           |
  | What already exists  | written                                     |
  | Failure modes        | 7 rows, 3 critical gaps                     |
  | Test artifact        | written                                     |
  | Outside voice        | ran (codex-only)                            |
  | Unresolved decisions | 2 taste choices, 0 blockers                 |
  +====================================================================+
```

The M10 section above is now the authoritative source of truth. This review record stays only as
historical evidence for why the boundary and contract were locked this way.

---

## M9 Review Record (2026-04-15)

`/autoplan` was run against the refreshed M9 scope and grounded against
[DECISIONS.md](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/DECISIONS.md:56),
[spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:1),
[spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:1),
[spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:1),
[spec-core/src/generator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/generator.rs:475),
[spec-core/src/validator.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/validator.rs:303),
[spec-cli/src/config.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/config.rs:1),
and [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:1797).

Outcome:
- CEO correction: keep M9 next, but narrow it to direct repo-scoped shared-library reuse.
- Eng correction: make dep identity, root-owned config, Cargo alias validation, and export schema
  v3 explicit in the milestone contract.
- Design review skipped, no UI scope.
- Outside voice: Codex ran, delegated subagents were unavailable in this thread by policy.
- Test artifact: [spensermcconnell-main-m9-test-plan-20260415-211200.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-m9-test-plan-20260415-211200.md)
- Unresolved plan decisions: 0

The M9 section above is now the authoritative source of truth. This review record stays only as
historical evidence for why the scope and boundary were locked this way.

## M8-M10 /autoplan Eng Review (2026-04-14)

**Review scope:** updated M8/M9/M10 roadmap sections, checked against current graph/export/status
implementation in [spec-core/src/graph.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/graph.rs:1),
[spec-core/src/export.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/export.rs:1),
[spec-core/src/types.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/types.rs:1),
[spec-core/src/passport.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-core/src/passport.rs:1),
and [spec-cli/src/commands.rs](/Users/spensermcconnell/__Active_Code/atomize-hq/spec/spec-cli/src/commands.rs:61).

### Architecture Review

**System architecture**

```text
Loaded .unit.spec + .test.spec
        │
        ▼
  validator (authoritative integrity gate)
        │
        ▼
 validated graph input
        │
        ▼
     SpecGraph
   ├── units
   ├── molecule_tests
   ├── declared dep edges
   ├── declared covers edges
   └── reverse indexes
        │
        ├── export projection
        ├── planning impact queries
        └── future M9 cross-library extension
```

**Architecture finding:** `SpecGraph::build()` should not become a public blind copier over raw
loaded specs. Today graph integrity checks live in CLI validation, not inside `graph.rs`. M8
must either build from validated input or return a fallible result.

**Architecture finding:** export must remain a projection over graph, not a serialization of
graph structs directly. Otherwise M9 graph evolution will become export-schema churn.

### Code Quality Review

- Current graph storage is flat vectors only. That is acceptable for M7 export, but not for the
  repeated `reverse_deps`, `tests_covering`, and `impact` queries M8/M10 want. The plan now
  needs reverse indexes baked into construction.
- The repo still carries `links.molecule_tests` as legacy metadata in `SpecStruct`. M8 must name
  its treatment explicitly so there is one relationship contract, not two.
- Cross-library dep parsing cannot stay stringly typed. M9 now explicitly owns a typed dep IR in
  `spec-core`, not a graph-only patch.

### Test Review

```text
CODE PATH COVERAGE
===========================
[+] spec-core/src/graph.rs
    │
    ├── [★   TESTED] build() creates dep + covers edges
    ├── [GAP]         reverse_deps() direct dependent lookup
    ├── [GAP]         reverse_deps() transitive closure
    ├── [GAP]         tests_covering() direct and multiple tests
    ├── [GAP]         impact() includes downstream units + their covering tests
    ├── [GAP]         unknown unit id contract (Result/Option vs silent empty)
    └── [GAP]         large fan-out/fan-in indexing behavior

[+] spec-core/src/export.rs
    │
    ├── [★★  TESTED] export builds graph edges through SpecGraph
    ├── [GAP]         export remains projection when graph adds new fields
    └── [GAP]         deterministic projection with graph query indexes present

[+] M9 cross-library dep layer
    │
    ├── [GAP]         parsed DepId IR round-trip from authored YAML
    ├── [GAP]         unknown namespace
    ├── [GAP]         missing canonicalized path
    ├── [GAP]         alias-to-self / duplicate canonical root
    ├── [GAP]         symlink-cycle external root
    └── [GAP]         cross-library cycle in graph + generator integration

[+] M10 plan artifact layer
    │
    ├── [GAP]         action=modify requires existing unit
    ├── [GAP]         action=add requires non-existent unit
    ├── [GAP]         graph scope resolves from enclosing spec-library root
    └── [GAP]         impact includes downstream molecule tests, not just direct seed tests

─────────────────────────────────
COVERAGE: existing tests prove seed graph construction and export projection basics.
GAPS: graph query semantics, typed dep identity, plan action validation, and external-library path trust boundaries.
─────────────────────────────────
```

### Performance Review

- Repeated graph queries over flat `Vec` scans will degrade once M9 loads multiple libraries.
  The plan now requires reverse indexes built once during graph construction.
- Deterministic ordering is part of the performance and correctness contract, because export
  snapshots and planning artifacts should not flap.

### ENG Dual Voices — Consensus Table

```text
ENG DUAL VOICES — CONSENSUS TABLE:
═══════════════════════════════════════════════════════════════
  Dimension                           Claude  Codex  Consensus
  ──────────────────────────────────── ─────── ─────── ─────────
  1. Architecture sound?               N/A     Partial single-model concern
  2. Test coverage sufficient?         N/A     No      single-model concern
  3. Performance risks addressed?      N/A     Partial single-model concern
  4. Security threats covered?         N/A     Partial single-model concern
  5. Error paths handled?              N/A     Partial single-model concern
  6. Deployment risk manageable?       N/A     Yes     single-model positive
═══════════════════════════════════════════════════════════════
```

**CODEX SAYS (eng — architecture challenge):**
- `impact()` was under-specified and would under-report downstream molecule tests.
- M10 `action: add` contradicted the existing-unit validation rule.
- M9 needed typed dep identity at the `spec-core` layer, not just extra graph metadata.
- Graph scope resolution for plan commands had to be anchored at the enclosing library root.
- Graph query APIs needed explicit unknown-id behavior and indexed internals.

**CLAUDE SUBAGENT (eng — independent review):** unavailable in this run. Session policy for
this thread does not allow delegated sub-agents unless the user explicitly asks for delegation.

### Cross-Phase Themes

- **Truth over convenience** — Phase 1 and Phase 3 both flagged the same risk: do not let M8
  pretend to know more than the repo can currently observe.
- **Type identity before metadata** — Phase 1 and Phase 3 both converged on the same M9 rule:
  cross-library identity must become a typed core contract before graph decorations land.

### Test Plan Artifact

- QA handoff written to [spensermcconnell-main-eng-review-test-plan-20260414-223534.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-main-eng-review-test-plan-20260414-223534.md)

### Completion Summary

```text
  +====================================================================+
  |            M8-M10 /autoplan REVIEW — COMPLETION SUMMARY            |
  +====================================================================+
  | Mode selected        | SELECTIVE_EXPANSION                         |
  | System Audit         | M8 reframed as declared graph contract      |
  | Step 0               | premise gate passed with user option A      |
  | Section 1  (Arch)    | 4 issues found                              |
  | Section 2  (Errors)  | failure modes updated, 3 critical gaps      |
  | Section 3  (Security)| 2 filesystem trust-boundary issues          |
  | Section 4  (Data/UX) | skipped, no UI scope                        |
  | Section 5  (Quality) | 3 contract-drift issues found               |
  | Section 6  (Tests)   | diagram produced, major gaps identified     |
  | Section 7  (Perf)    | 1 index/query-shape issue found             |
  | Section 8  (Observ)  | skipped, no new runtime surface in M8       |
  | Section 9  (Deploy)  | roadmap-only, no new deploy gate required   |
  | Section 10 (Future)  | M9/M10 cascades updated                     |
  | Section 11 (Design)  | SKIPPED (no UI scope)                       |
  +--------------------------------------------------------------------+
  | NOT in scope         | written and refreshed                       |
  | What already exists  | written and refreshed                       |
  | Error/rescue registry| failure modes table updated                 |
  | Failure modes        | 5 rows, 3 critical gaps                     |
  | TODOS.md updates     | roadmap TODO section updated in-plan        |
  | Scope proposals      | 3 evaluated, contract-first path accepted   |
  | CEO plan             | not externalized; review captured in plan   |
  | Outside voice        | ran (codex-only)                            |
  | Lake Score           | 6/6 major decisions chose complete option   |
  | Diagrams produced    | architecture, test coverage                 |
  | Stale diagrams found | 0                                           |
  | Unresolved decisions | 0 user-blocking, 2 roadmap clarifications   |
  +====================================================================+
```


## Failure Modes

| Codepath | Production failure mode | Test covers? | Error handling? | Silent? |
|---|---|---|---|---|
| Default output anchored to crate root | crate_root not resolved (no Cargo.toml) | yes (workspace_root_for tests) | bail with clear message | no |
| Evidence preservation in write_passports | passport file corrupted on disk | via serde deserialize | returns None, writes fresh | no |
| 6-state status transitions | clock skew between observed_at and now | N/A | timestamp is informational | no |
| .test.spec covers validation | covers unit deleted after test authored | yes (integration) | SPEC_MOLECULE_COVERS_NOT_FOUND | no |
| graph.impact() | downstream molecule tests omitted from retest set | yes (planned `impact_includes_downstream_covering_tests_not_just_seed_tests`) | `ImpactSet` contract + BFS closure over reverse deps | no |
| graph query API | unknown unit id returns empty and looks valid | yes (planned `*_unknown_unit_returns_none` tests) | explicit `Option` contract on all graph query methods | no |
| Cross-library dep resolution | [libraries] path not found on disk | partial | needs explicit test + loud error | **critical gap** |
| Cross-library dep resolution | alias resolves to self or duplicate canonical root | no | plan now requires rejection | **critical gap** |
| Plan artifact impact computation | graph built from file path instead of library root | no | plan now requires root resolution | **critical gap** |

**Critical gaps:**
- M9 needs explicit tests for missing library path, alias-to-self, duplicate canonical root,
  and symlink-looped external roots.
- M10 needs explicit tests proving plan validation resolves graph scope from the enclosing
  library root and handles `action: add` differently from `modify/remove`.

---

## NOT in Scope (Deferred)

- TypeScript / Python / Go generator targets (moved from M5 design doc; re-evaluate after M8)
- `ValidatedExpr` as a public library type (bundled into structural PR as internal refactor only)
- Observed coverage edges (molecule tests declare coverage; observation requires instrumentation)
- Molecule test passports / evidence tracking (molecule tests run via cargo test, but status
  tracking for them deferred until M8 graph is solid)
- Nextest support (detect nextest format and surface clear error rather than "unknown" — nice-to-have after M6a)
- LLM semantic contract-vs-body scoring
- CUE
- Reverse ingestion

---

## What Already Exists (reuse, don't rebuild)

The authoritative M10 reuse map now lives inside the milestone section above. Keep reusing:
- existing workspace + repo boundary resolution in `spec-cli/src/config.rs`
- local impact truth in `spec-core/src/graph.rs`
- versioned export-bundle patterns in `spec-core/src/export.rs`
- existing JSON fixture and CLI integration-test posture in `spec-cli/tests/`

---

## Worktree Parallelization

| Step | Modules touched | Depends on |
|---|---|---|
| Contract gate | `PLAN.md`, plan schema types, root-resolution contract notes | — |
| Lane A: spec-core plan contract | `spec-core` plan types, derived-impact types, plan export builder, unit tests | Contract gate |
| Lane B: spec-cli plan commands | `spec-cli` command wiring, plan-root resolver, validation diagnostics, CLI integration scaffolding | Contract gate |
| Join lane | integration tests, fixtures, README, AGENTS.md | Lane A + Lane B |

**Parallel lanes**
- `Lane A:` shared plan data contract in `spec-core`
- `Lane B:` CLI validate/export surface in `spec-cli`
- `Join lane:` end-to-end verification and docs after both land

**Execution order**
- Lock the schema and resolver contract first.
- Launch `Lane A` and `Lane B` in parallel only after that gate.
- Run the join lane last for integration coverage, fixture updates, and docs.

**Conflict flags**
- Both lanes depend on one shared `computed_impact` contract. Do not let each lane invent its own shape.
- Do not start fixture churn before the validate/export payloads are locked.
- If M10 scope expands into cross-library impact, stop parallelization and re-plan the milestone.

---

## TODOS.md Updates

This pass does not reopen shipped M6-M9 work. New M10-specific follow-ups to add:

- `[M10] Add stable error codes for plan outside library root, duplicate plan change ids,
  cross-library plan refs, modify/remove on missing unit, and add on existing unit.`
- `[M10] Add CLI fixtures for \`spec plan validate --format json\` and
  \`spec plan export\` schema_version 1 ordering.`
- `[post-M10] Decide whether future-edge authoring for \`action=add\` becomes a first-class plan
  feature or stays unresolved until a later graph-query milestone.`
- `[post-M10] Cross-library plan impact semantics need their own milestone after local-library
  plan truth is proven.`

---

## Implementation Order

**Current milestone: M10. M6a through M9 are shipped.**

```text
1. Lock M10 plan schema + root-resolution contract
   - single-file invocation only
   - local-library authored ids only
   - derived impact remains output-only

2. Implement spec-core plan contract
   - typed authored-plan structs
   - typed derived-impact structs
   - dedicated plan export bundle

3. Implement spec-cli plan commands
   - `spec plan validate <file>`
   - `spec plan export <file>`
   - root-scoped plan loading and validation diagnostics

4. Add regression suite
   - action-specific validation coverage
   - nested plan-path root resolution
   - symlink escape / root-boundary enforcement
   - deterministic impact union + export fixtures

5. Verification
   - cargo test -p spec-core
   - cargo test -p spec-cli
   - cargo test --all

6. Re-review before widening
   - keep M10 local-library scoped unless a later milestone expands query semantics

7. /ship when implementation lands
```

**Do not front-load into this PR:**
- Cross-library plan refs or cross-library impact
- Plan execution, task tracking, or planning UI
- Future-edge authoring for `action=add`
- Automatic plan discovery during `spec export`
- Local-test-level acceptance target identity

---

**Document version:** 2026-04-16
**Review status:** M10 consolidated into one implementation-ready plan section
**Next review checkpoint:** After M10 command surface lands, before any scope widening

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 5 | clean (PLAN via /autoplan) | M10 narrowed to truthful local-library change intent plus derived impact, not planning theater |
| Codex Review | `/codex review` | Independent 2nd opinion | 10+ | issues_found | M10: root resolution, action-sensitive impact semantics, dedicated plan export, and trust-boundary clarity |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 10 | **CLEAR (PLAN)** | M10 gaps made explicit: root-scoped loading, failure modes, test coverage, and parallelization |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |
| DX Review | `/plan-devex-review` | Developer experience gaps | 1 | issues_open | score: 5/10 → 7/10, TTHW: 5min-local/BLOCKED-external |

**CODEX (M10):** flagged the real missing pieces: root-scoped graph loading, explicit `action=add`
uncertainty, stable plan JSON/export contracts, and path-boundary handling that does not widen
the repo trust surface by accident.
**UNRESOLVED:** 0
**VERDICT:** PLAN LOCKED — start with the M10 schema and root-resolution contract, then land the
`spec-core` plan types, then the `spec-cli` validate/export surface, then the regression suite
before `/ship`.
