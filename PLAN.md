<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m20-autoplan-restore-20260427-122129.md -->
# M21 - Semantic Family Promotion Harness

Status: **rough draft for `/autoplan` review on `feat/m20`**.

UI scope: **no**. This is a backend semantic-review milestone for plan artifacts, repo-owned
orchestration, fixture generation, promotion gates, and one newly promoted semantic function
family.

M19 proved a narrow but real positive slice. M20 made the negative path honest. M21 should turn
that into a repeatable promotion system so future semantic-family expansion is deliberate instead
of ad hoc.

M21 is **not** "auto-generate new semantic understanding." It is a promotion harness that helps
humans define, implement, falsify, and certify one bounded **function** family at a time.

## Source Inputs

- Current branch: `feat/m20`
- Base branch: `main`
- Restore point: `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m20-autoplan-restore-20260427-122129.md`
- Relevant landed proof:
  - M19 unseen falsification pack
  - M20 unsupported-function truth surface
- Relevant current code:
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`

## Milestone Summary

```text
M21a  Introduce a repo-owned semantic family packet layout                    required
M21b  Add an xtask crate to orchestrate family promotion workflows           required
M21c  Encode the reusable proof gate for candidate → prove → certify         required
M21d  Make the gate check M19/M20 truth-surface rules, not just classifiers  required
M21e  Promote `function.wrapper.pipeline.chain3.v1` through the harness       required
M21f  Leave public CLI product surface unchanged in M21                      required
```

## Problem Statement

Right now the repo has real semantic-review machinery, but the process for adding the next family
still lives mostly in human memory plus a successful prior milestone.

That does not scale.

If the project wants to keep expanding semantic-review coverage toward a meaningful subset of Rust,
and later prove the shared model is broad enough to support another language, it needs a repeatable
promotion system with the same honesty constraints that made M19 and M20 credible.

The real M21 question:

How do we make future semantic-family expansion repeatable, reviewable, and semi-automatable
without turning the repo into a fake generic-Rust-classifier factory, while still proving one
more externally meaningful slice of supported Rust behavior?

## User Outcome

An engineer working on the repo can propose a new semantic **function** family and run one
consistent workflow:

1. define the family packet in `semantic-families/<family>/`
2. run `cargo xtask family new|prove|certify ...`
3. generate the right fixture skeletons and proof checklist
4. implement the family classifier in `spec-core`
5. prove the family matches the current semantic-core contract
6. prove the family survives a true unseen example corpus
7. certify that the new family refreshes and projects truth the same way M19/M20 families do

The output is not just "tests passed." It is a certification result that says whether the family
earned promotion or should be rejected, plus evidence that the promoted family unlocked a real
change-loop shape the current surface could not support cleanly.

## Current Thesis

- M19 and M20 together prove the current semantic-review substrate is honest enough to expand.
- Expansion must stay bounded and family-based, not drift into generic language understanding.
- Unsupported-function reporting does not need another milestone right now; that landed cleanly.
- The next bottleneck is repeatable function-family promotion, not more unsupported-path polish.
- Second-language work remains downstream of more proven positive-path breadth.

## What Already Exists

| Sub-problem | Existing code | Reuse in M21 |
|---|---|---|
| Function family routing | `spec-core/src/semantic_review.rs` | New families still plug into explicit routing, not ad hoc file ids |
| Preserve vs refresh truth loop | `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs` | Family certification must validate that refresh/preserve behavior stays honest |
| CLI fixture harness | `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs` | Reuse for command-matrix and stale/read-side proof |
| Unseen corpus pattern | `spec-cli/tests/fixtures/m19/semantic_falsification_pack/` | Template for future family-specific falsification packs |
| Public contract wording | `README.md`, `AGENTS.md`, `CLAUDE.md` | M21 should add process docs, not new end-user commands |

## Proposed Deliverable Shape

### 1. Repo-owned family packet directory

Add:

```text
semantic-families/<family>/
  candidate.md
  family.toml
  fixtures/
    aligned/
    drift/
    under_specified/
    unsupported_near_miss/
```

`candidate.md` is human-readable review context only.

`family.toml` is the machine-readable manifest consumed by `xtask`:

- family name
- one-sentence semantic claim
- authored truth requirements
- executable body-shape grammar
- required invariants
- allowed dep/helper topology
- drift taxonomy
- under-specified taxonomy
- unsupported-near-miss taxonomy
- unseen-example requirements
- compatibility key
- routing precedence expectations
- family scope (`kind:function` only in M21)
- second-language litmus note

`xtask` validates `family.toml` before `prove` or `certify`. Markdown is not the executable truth
surface.

### 2. `xtask` orchestration crate

Add an `xtask` crate for repo-owned workflow orchestration, not product semantics.

Expected commands:

```text
cargo xtask family new <family>
cargo xtask family prove <family>
cargo xtask family certify <family>
```

Responsibilities:

- scaffold `semantic-families/<family>/`
- generate checklist/templates for fixtures and certification
- run the expected unit and CLI proof suites
- aggregate results into `.semantic-family-artifacts/semantic-families/<family>/certification.report.json`
- fail loudly when a family skips any required gate

Non-responsibilities:

- do not contain classifier truth itself
- do not become a shadow semantic engine
- do not create public CLI surface in M21

Trust boundary:

- family packets are trusted repo source, not untrusted user input
- `xtask` must reject path traversal, symlinks, and invalid family ids
- writes stay inside repo-relative packet paths or `.semantic-family-artifacts/`
- `prove` / `certify` are safe on trusted branches and CI, not a general-purpose runner for
  arbitrary external artifacts

### 3. Reusable proof gate

Every family must pass three gates:

#### Gate A: Core-shape integrity

Does the new family follow the same core contract as M19/M20?

- bounded family
- explicit authored-side claim
- explicit executable-side matcher
- explicit drift reasons
- explicit under-specified reasons
- explicit unsupported-near-miss exclusion
- explicit routing precedence and non-shadowing expectations

#### Gate B: True unseen-example survival

Does the family survive a non-canonical corpus?

- alternate ids
- non-canonical naming
- aligned examples
- drift examples
- under-specified examples
- unsupported near-miss examples

#### Gate C: Truth-surface honesty

Does the family behave honestly across product surfaces?

- `spec test` refreshes family proof
- `spec build`, `spec generate`, `spec status`, and `spec export` preserve but do not mint proof
- stale proof degrades correctly
- read-side health remains coherent

#### Gate D: Cross-family non-regression

Does the promoted family avoid stealing or breaking existing classifications?

- Family A current examples still classify as Family A
- Family B current examples still classify as Family B
- unsupported near-miss examples do not accidentally promote into the new family
- packet precedence matches runtime routing order

## Approaches Considered

### Approach A: Promotion Harness + One Real Function Family

Summary: build the reusable promotion system, then immediately run one new family all the way
through it.

Effort: M
Risk: Low

Pros:
- makes expansion repeatable without ceremony-only process
- forces the harness to prove itself immediately
- gives the repo both infrastructure and new semantic breadth

Cons:
- more moving parts than a one-off manual family addition
- requires discipline about what lives in xtask versus product code

### Approach B: Framework First, No New Family

Summary: ship only templates, orchestration, and certification structure.

Effort: M
Risk: Med

Pros:
- clean process skeleton
- lower immediate classifier risk

Cons:
- easy to ship process theater
- does not prove the harness is useful on real semantic work

### Approach C: New Family First, Standardize Later

Summary: add the next family manually, then extract the process afterward.

Effort: S-M
Risk: Med

Pros:
- fastest visible semantic expansion
- simplest first implementation path

Cons:
- bakes in another ad hoc milestone
- makes future standardization fuzzier

## Recommended Approach

Choose **Approach A: Promotion Harness + One Real Function Family**.

Reason: M21 should improve the rate and honesty of future family promotion, not just ship another
one-off family. But the harness must be forced to prove itself on one real family or it becomes
process garnish.

## Scope

M21 includes:

1. `semantic-families/` repo layout.
2. `xtask` crate with `family new`, `family prove`, and `family certify`.
3. `family.toml` machine manifest plus `candidate.md` human review context.
4. Reusable unseen-fixture and certification-report conventions.
5. Required truth-surface and cross-family non-regression gates that reuse the M19/M20
   command-matrix discipline.
6. One newly promoted function family, `function.wrapper.pipeline.chain3.v1`, run through the full
   harness.
7. Docs explaining repo-owned family promotion workflow.
8. Kill criteria and outcome metrics that can fail the milestone honestly.

## Not In Scope

- generic automatic family discovery
- LLM-generated family design without human review
- public `spec family ...` commands
- second-language implementation work
- `kind:data` or `kind:sum` family promotion in M21
- broad semantic ontology redesign
- another unsupported-path-specific milestone
- multiple new families in the same milestone unless one proves trivially incremental

## Chosen Promoted Family

M21 explicitly promotes:

`function.wrapper.pipeline.chain3.v1`

Shape:

- straight-line three-step wrapper pipeline
- three supported dep callables
- decimal-in / decimal-out contract
- no branching
- no loops
- explicit arg threading from authored inputs through exactly three supported calls

Why this family:

- it stays inside `kind:function`, which matches the current evaluator architecture
- it meaningfully expands real pricing/checkout change loops beyond the current two-step wrapper
- it creates a stronger falsification surface than another arithmetic-leaf variant
- it pressures routing precedence and unseen-corpus proof without forcing a seam refactor

Family-selection rubric used:

- real change-loop leverage
- boundedness
- strong falsification surface
- minimal evaluator-architecture expansion
- future portability signal

## Architecture Review

### Dependency graph

```text
semantic-families/<family>/candidate.md
        │
        ├── semantic-families/<family>/family.toml
        │      └── machine-readable manifest for xtask validation
        │
        ├── cargo xtask family new
        │      └── scaffold family packet + fixture skeletons
        │
        ├── cargo xtask family prove
        │      ├── run spec-core family tests
        │      ├── run spec-cli command-matrix tests
        │      └── check unseen fixture completeness
        │
        ├── cargo xtask family certify
        │      ├── evaluate Gate A / Gate B / Gate C / Gate D
        │      └── write .semantic-family-artifacts/.../certification.report.json
        │
        └── product code touched by the promoted family
               ├── spec-core/src/semantic_review.rs
               ├── spec-core/src/passport.rs        [if projection rules need extension]
               ├── spec-core/src/export.rs          [if export projection needs extension]
               ├── spec-cli/src/commands.rs         [if status/read-side behavior needs tests]
               └── spec-cli/tests/...               [fixture + command-matrix proof]
```

### Boundary rule

`xtask` owns orchestration only.

Product truth remains in:

- `spec-core`
- `spec-cli`
- checked-in manifests and fixtures

If a rule matters to the shipped product, it cannot live only in `xtask`.

Packet authority rule:

- `family.toml` drives orchestration inputs
- runtime semantic truth still lives in `spec-core`
- `candidate.md` exists for human review, rationale, and certification commentary only

Artifact ownership rule:

- checked in: `candidate.md`, `family.toml`, fixture sources
- not checked in: full `certification.report.json`
- CI may publish full certification reports as artifacts
- `.semantic-family-artifacts/` is gitignored repo-local state, not Cargo build output
- a stable checked-in summary is allowed only if it is tiny and schema-versioned

## Test Review

M21 needs two test layers.

### Harness tests

- scaffold generation tests for `cargo xtask family new`
- prove/certify happy-path tests
- missing-artifact and partial-packet failure tests
- malformed manifest tests
- path traversal / symlink rejection tests
- duplicate fixture id tests
- do-not-overwrite-last-known-good-report-on-failure tests
- dirty-worktree and concurrent-run behavior tests

### Promoted-family tests

- `spec-core` classifier tests
- unseen aligned/drift/under-specified/unsupported fixture tests
- `spec-cli` command-matrix tests for refresh/preserve/stale behavior
- export/status assertions for read-side honesty
- cross-family precedence regression tests
- stale-proof invalidation tests when classifier shape or manifest version changes

## Success Criteria

- `semantic-families/<family>/` exists with a documented packet schema.
- `family.toml` exists and is versioned; `candidate.md` is review-only context.
- `xtask` crate exists and can scaffold, prove, and certify a family packet.
- M21 defines one reusable certification gate covering core shape, unseen examples,
  truth-surface honesty, and cross-family non-regression.
- `function.wrapper.pipeline.chain3.v1` is promoted through the harness end to end.
- The new family proves real unseen-example survival.
- The new family obeys M19/M20 refresh/preserve honesty rules.
- The new family does not shadow or break Family A / Family B.
- Public CLI surface remains unchanged in M21.
- Documentation explains how future families are proposed and certified.
- External-value proxy: the promoted family unlocks at least one realistic three-step pricing /
  checkout flow currently outside the shipped supported subset.
- Kill criteria: if the harness requires family-specific schema escape hatches, or if the promoted
  family cannot be certified without duplicating product semantics in xtask, M21 is red.

## Failure Modes to Guard Against

| Failure mode | Why it matters | M21 guard |
|---|---|---|
| xtask becomes a shadow semantic engine | Product truth drifts into private orchestration | Keep semantic rules in `spec-core` or checked-in family packets |
| Harness certifies incomplete packets | Teams can mint fake families | Gate fails on missing aligned/drift/under-specified/unsupported corpora |
| New family passes classifier tests but breaks read-side truth | Product surface lies | Gate C required for certification |
| New family steals Family A/B matches | Existing truth silently changes | Gate D precedence regression suite |
| Family packet schema becomes bureaucracy | Slows real work | Keep packet minimal and centered on proof |
| Packet docs drift from runtime truth | Engineers trust the wrong source | Machine manifest plus runtime-owned truth boundary |
| Certification is not reproducible | Two engineers certify different things | Report provenance with SHA, toolchain, schema version, and fixture digests |

## Distribution

No new end-user distribution channel is needed.

This is repo-owned promotion machinery plus product-internal semantic expansion. Existing Cargo,
CI, and release workflows remain enough.

## Outcome Metrics and Kill Criteria

### Outcome Metrics

- M21 certifies one real new family using one repeatable command path.
- The promoted family unlocks at least one realistic three-step pricing or checkout flow not
  representable by current Family B.
- The next function family packet after M21 should require materially less custom test scaffolding
  than M19 did.

### Kill Criteria

- if `family.toml` needs family-specific one-off fields to support the chosen exemplar, M21 is red
- if `xtask` must infer semantic truth by reimplementing classifier rules, M21 is red
- if the promoted family breaks Family A or Family B precedence and no bounded routing fix exists,
  M21 is red
- if the external-value proxy is still hand-wavy by the end of the milestone, M21 is red

## Initial Next Steps

1. Lock the family packet schema under `semantic-families/`.
2. Add `family.toml` as the required machine-readable manifest and keep `candidate.md` review-only.
3. Define the exact outputs of `family new`, `family prove`, and `family certify`.
4. Scaffold `function.wrapper.pipeline.chain3.v1` as the promoted exemplar.
5. Write the M21 certification gate before implementation starts.

## CEO Review

### 0A. Premise Challenge

1. **Premise:** the next repo bottleneck is repeatable semantic-family promotion.
   Status: challenged.
   Why: current evidence proves a narrow semantic substrate, but does not yet prove that promotion
   process is the next user-visible bottleneck.

2. **Premise:** one new family plus a harness is the best next milestone shape.
   Status: plausible, but not yet proven.
   Why: this is the most complete internal move, but it risks process-first work unless M21 also
   yields a clearer adoption wedge or measurable external progress.

3. **Premise:** `xtask` plus `semantic-families/` is the right boundary.
   Status: supported.
   Why: existing product semantics are centralized in `spec-core` / `spec-cli`, and there is no
   current orchestration crate to misuse or unwind.

4. **Premise:** the next family should be chosen for semantic distinctness alone.
   Status: rejected.
   Why: distinctness matters, but M21 should also optimize for external leverage and real change
   frequency.

### 0B. Existing Code Leverage Map

| Sub-problem | Existing code | Reuse decision |
|---|---|---|
| Family evaluation entrypoint | `spec-core/src/semantic_review.rs` | Reuse directly, do not mirror logic in xtask |
| Refresh/preserve truth projection | `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs` | Reuse as Gate C certification target |
| Fixture and command-matrix proof | `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs` | Reuse as the core proof harness |
| Prior unseen corpus shape | `spec-cli/tests/fixtures/m19/semantic_falsification_pack/` | Reuse as the template for future family packets |
| Workspace packaging | root `Cargo.toml` | Add `xtask` as a workspace member, no new infra needed |

### 0C. Dream State Diagram

```text
CURRENT
  Narrow but real semantic families.
  Honest refresh/preserve behavior.
  Next family addition still lives in maintainer memory.

THIS PLAN
  Formal family packet layout.
  xtask orchestration for scaffold → prove → certify.
  One new family promoted through the same honest gate.

12-MONTH IDEAL
  Family promotion is fast and boring.
  New families are chosen for real repo leverage, not neatness.
  `spec` proves trustworthy change loops on real codebases, not just internal classifier rigor.
```

### 0C-bis. Alternatives Table

| Approach | Summary | Effort | Risk | Why choose it | Why reject it |
|---|---|---:|---:|---|---|
| A. Promotion Harness + One Real Family | Standardize future family promotion and prove it immediately | M | Low | Best internal rigor and repeatability | Can become process-first unless tied to external value |
| B. Framework First, No New Family | Build only process scaffolding | M | Med | Cleanest orchestration skeleton | High risk of shipping process theater |
| C. New Family First, Standardize Later | Manually add one more family before automation | S-M | Med | Fastest visible semantic expansion | Repeats ad hoc milestone shape |
| D. Adoption Wedge First | Pick one real external change loop and only build the minimum harness needed to support it | M-L | High | Best external proof if it lands | Higher milestone ambiguity and broader blast radius |

### 0D. Mode Selection

Mode: `SELECTIVE_EXPANSION`

Reason: keep the user’s M21 direction intact, but expand it enough to include outcome metrics,
kill criteria, and family-selection rules tied to leverage, not just elegance.

### 0E. Temporal Interrogation

- **Hour 1:** scaffold `xtask`, define packet schema, choose the promoted family.
- **Hour 6:** one family packet exists, but if certification rules are still vague, the milestone is
  already drifting into paperwork.
- **End of milestone:** success only matters if the harness both certifies one family and shortens
  the path for the next one in a measurable way.
- **6 months later:** this looks good if it became the shortest path to more real semantic
  coverage. It looks bad if it became a careful internal bureaucracy with no adoption wedge.

### 0F. CEO Review Findings

- The milestone needs explicit **kill criteria** so it can fail strategically instead of only
  succeeding procedurally.
- The promoted family should be chosen by **real leverage**, not by semantic neatness alone.
- Success criteria need at least one **external-value proxy**, not just artifact completion.
- The harness must stay **repo-owned orchestration**, not a second semantic engine.

### CEO Dual Voices

#### CODEX SAYS (CEO - strategy challenge)

- The plan may be optimizing the family factory before proving the outputs matter enough.
- Leaving the family choice open while standardizing the harness is backwards.
- Zero public surface change makes the milestone strategically invisible unless it also improves an
  external adoption story.
- The plan needs kill criteria and at least one benchmark for real user-value movement.

#### CLAUDE SUBAGENT (CEO - strategic independence)

- The stronger framing may be adoption wedge first: make `spec` the fastest trustworthy way to
  change real codebases with AI, then automate family promotion only as needed.
- The current success criteria measure internal artifact completion, not product progress.
- The plan needs premise evidence and falsifiers, not just confident assertions.
- Competitive pressure is real enough that slow internal bureaucracy would be a strategic mistake.

#### CEO DUAL VOICES - CONSENSUS TABLE

| Dimension | Claude | Codex | Consensus |
|---|---|---|---|
| Premises valid? | challenged | challenged | DISAGREE with current draft |
| Right problem to solve? | maybe too internal | maybe too internal | DISAGREE with current draft |
| Scope calibration correct? | too governance-first | too governance-first | DISAGREE with current draft |
| Alternatives sufficiently explored? | no | no | DISAGREE with current draft |
| Competitive / market risks covered? | weak | weak | DISAGREE with current draft |
| 6-month trajectory sound? | only with kill criteria | only with kill criteria | DISAGREE with current draft |

### Error & Rescue Registry

| Risk | Failure shape | Rescue |
|---|---|---|
| Harness-first drift | We ship scaffolding before proving the next family matters | Require one promoted family plus outcome metrics in the same milestone |
| Wrong next family | We choose the neat family instead of the useful one | Add family-selection rubric weighted by real leverage and frequency |
| Strategic invisibility | M21 ships with no externally legible value movement | Add at least one adoption or capability proxy metric |
| Bureaucratic packet design | The packet schema grows faster than families do | Keep packet minimal and fail any non-essential metadata expansion |

### Cross-Phase Themes

- **Truth before theater:** the repo should not confuse process completion with product progress.
- **Leverage over neatness:** the promoted family should unlock something meaningful, not just look
  clean in the taxonomy.

## Design Review

Skipped, no UI scope.

## Eng Review

### 0. Scope Challenge

The codebase supports a **function-family-only** M21 cleanly. It does **not** support a general
"any future family kind" harness at the same risk level.

Why:

- function-family routing is centralized in `spec-core/src/semantic_review.rs`
- preserve / refresh truth projection is centralized in `spec-core/src/passport.rs`,
  `spec-core/src/export.rs`, and `spec-cli/src/commands.rs`
- `kind:data` and `kind:sum` support are still unit-specific and would require a broader evaluator
  refactor than this milestone should absorb

Engineering decision:

- M21 stays on `kind:function`
- promoted exemplar is `function.wrapper.pipeline.chain3.v1`
- `xtask` is orchestration-only

### 0.5. Eng Dual Voices

#### CODEX SAYS (eng - architecture challenge)

- choose the promoted family now, not after designing the harness
- do not let packets pretend to be runtime truth unless there is a real ingestion model
- add explicit precedence / shadowing checks because runtime routing is ordered
- current risk was understated given `semantic_review.rs` and `spec-cli/tests/cli.rs` are already
  large monoliths

#### CLAUDE SUBAGENT (eng - independent review)

- move machine meaning out of Markdown and into a versioned manifest
- make certification reproducible with provenance
- define a clear trust boundary for packet inputs and `xtask` writes
- keep full certification output out of checked-in source to avoid stale truth and merge churn

#### ENG DUAL VOICES - CONSENSUS TABLE

| Dimension | Claude | Codex | Consensus |
|---|---|---|---|
| Architecture sound? | conditional | conditional | CONFIRMED with narrowing |
| Test coverage sufficient? | not yet | not yet | DISAGREE with current draft |
| Performance risks addressed? | partially | partially | DISAGREE with current draft |
| Security threats covered? | weak | weak | DISAGREE with current draft |
| Error paths handled? | weak | weak | DISAGREE with current draft |
| Deployment risk manageable? | yes with provenance | yes with provenance | CONFIRMED with fixes |

### 1. Architecture Review

The architecture is acceptable **only** with these decisions locked:

- `family.toml` is the machine-readable input
- `candidate.md` is human-readable context only
- runtime semantic truth remains in `spec-core`
- `xtask` consumes structured outputs; it does not infer semantic truth by scraping broad test text
- M21 is function-family-only

This keeps the blast radius bounded to:

- workspace root `Cargo.toml`
- new `xtask` crate
- new `semantic-families/function.wrapper.pipeline.chain3.v1/`
- `spec-core/src/semantic_review.rs`
- targeted `spec-cli` tests and fixtures

### 2. Code Quality Review

What already exists is reusable, but the repo has two large pressure points:

- `spec-core/src/semantic_review.rs` is already a large, ordered classifier file
- `spec-cli/tests/cli.rs` is already a large integration-matrix file

That means M21 should avoid:

- new ad hoc branching that widens monolith complexity without abstraction discipline
- duplicating gate semantics inside `xtask`
- adding packet-schema exceptions for the exemplar family

### 3. Test Review

Test plan artifact:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m20-test-plan-20260427-123346.md`

Key required coverage:

- `family new` scaffold success and failure paths
- manifest validation failures
- promoted-family aligned / drift / under-specified / unsupported unseen corpus
- refresh / preserve / stale command matrix
- precedence non-regression versus Family A / Family B
- report provenance and no-overwrite-on-failure behavior

This section is now a real gate, not a suggestion. Missing any of those means the harness can
greenlight the wrong family or record stale proof as current.

### 4. Performance Review

Main risk is not runtime user latency. It is developer-loop and CI drag.

If `family prove` reruns the entire `spec-core` and CLI matrix every time, people will bypass it.
So M21 needs:

- impacted-suite selection for `prove`
- broader matrix for `certify`
- full cross-family regression in CI / nightly, not every local prove run

### 5. Security and Trust Boundary Review

This is repo tooling, but it still executes and writes files. So the trust boundary has to be
explicit:

- reject invalid family ids
- reject path traversal
- reject symlink packet roots
- keep writes inside repo packet paths or `.semantic-family-artifacts/`
- treat packet inputs as trusted repo-owned source, not arbitrary external uploads

### 6. Failure Modes Registry

| Failure mode | Severity | Fix |
|---|---|---|
| Packet schema grows hidden DSL behavior | High | machine manifest only; Markdown review-only |
| Certification is non-reproducible | High | report provenance with SHA, toolchain, digests, exit codes |
| New family shadows Family A / B | High | Gate D required |
| xtask becomes semantic engine #2 | High | consume structured outputs only |
| Full certification output creates stale checked-in artifacts | Medium | keep full report in `.semantic-family-artifacts/` or CI artifacts |
| `prove` becomes too slow and gets bypassed | Medium | impacted-suite selection and caching |

### 7. Eng Completion Summary

```text
+====================================================================+
|                      M21 ENG REVIEW SUMMARY                        |
+====================================================================+
| Scope                 | function-family-only                        |
| Exemplar family       | function.wrapper.pipeline.chain3.v1         |
| Packet authority      | family.toml machine / candidate.md human    |
| Report ownership      | .semantic-family-artifacts + CI artifacts    |
| Core risk             | precedence, provenance, xtask drift          |
| Required new gate     | Gate D cross-family non-regression           |
| Test artifact         | written                                      |
| UI review             | skipped, no UI scope                         |
+====================================================================+
```

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | Intake | Replace stale M20 plan with rough M21 draft before review | Mechanical | Pragmatic | Reviewing the wrong milestone would waste the whole session | Reviewing M20 again |
| 2 | CEO | Keep `xtask` as orchestration-only boundary | Mechanical | Explicit over clever | Central semantic logic already has clear product homes | Hiding semantic truth in xtask |
| 3 | CEO | Add adoption-pressure concerns instead of silently auto-approving current framing | User Challenge | User sovereignty | Both outside voices challenged the milestone framing itself | Treating it as a taste-only disagreement |
| 4 | CEO | Accept user option A and keep the harness milestone with stronger constraints | Mechanical | Bias toward action | Preserves the user's direction while absorbing the strategic critique | Reframing the whole milestone around adoption first |
| 5 | Eng | Narrow M21 to `kind:function` only | Mechanical | Explicit over clever | Current evaluator architecture supports function-family promotion cleanly, seam-family promotion does not | One general harness for function, data, and sum in M21 |
| 6 | Eng | Choose `function.wrapper.pipeline.chain3.v1` as the exemplar family | Taste | Completeness | Stronger leverage and falsification surface than another arithmetic leaf, without forcing seam refactors | Seam-local family, trivial arithmetic variant |
| 7 | Eng | Add `family.toml` and demote `candidate.md` to review-only context | Mechanical | Explicit over clever | Avoids hidden Markdown DSL and gives xtask a stable manifest | Markdown-only packet |
| 8 | Eng | Keep full certification output out of checked-in source | Mechanical | Pragmatic | Avoids stale truth and merge churn while preserving reproducible CI artifacts | Checked-in full reports |
| 9 | Eng | Add Gate D for precedence and non-regression | Mechanical | Completeness | Ordered routing makes shadowing a real correctness risk | Per-family proof only |
