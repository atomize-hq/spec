<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m20-autoplan-restore-20260427-122129.md -->
# M21 - Semantic Family Promotion Harness

Status: **Implementation ready on `feat/m20`** (reviewed via `/autoplan` on 2026-04-27).

M19 proved that a bounded positive semantic slice can survive unseen examples. M20 made the
unsupported-function path honest on read-side truth surfaces. M21 turns those wins into one
repeatable promotion loop: propose one bounded function family, prove it against unseen examples
and truth-surface rules, then certify it without teaching repo tooling to become semantic engine
number two.

UI scope: **no**. This is a backend-only semantic-review milestone for repo-owned family packets,
`xtask` orchestration, one promoted function family, and explicit certification gates.

## Source Inputs

- Current branch: `feat/m20`
- Base branch: `main`
- Restore point:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m20-autoplan-restore-20260427-122129.md`
- Review artifact:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-m20-reviews.jsonl`
- Test plan artifact:
  `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m20-test-plan-20260427-123346.md`
- Relevant landed proof:
  - M19 unseen falsification pack
  - M20 unsupported-function truth surface
- Relevant code seams:
  - `spec-core/src/semantic_review.rs`
  - `spec-core/src/passport.rs`
  - `spec-core/src/export.rs`
  - `spec-cli/src/commands.rs`
  - `spec-cli/tests/cli.rs`
  - `spec-cli/tests/m14_regressions.rs`

## Milestone Summary

```text
M21a  Lock a repo-owned semantic family packet contract                    required
M21b  Add an xtask crate for scaffold / prove / certify orchestration      required
M21c  Encode a reusable certification gate for candidate → prove → certify required
M21d  Make the gate enforce M19 / M20 truth-surface honesty                required
M21e  Promote `function.wrapper.pipeline.chain3.v1` through the harness    required
M21f  Leave the public CLI surface unchanged                               required
```

**Lake to boil in M21**

- Promotion must become repeatable without becoming generic-Rust theater.
- The harness must prove itself immediately on one real family.
- The promoted family must unlock a real three-step pricing / checkout change loop.
- Truth-surface honesty stays part of the gate, not a later cleanup.
- The repo gains process leverage, not just a new directory tree.

## User Outcome

An engineer working on the repo can propose a new semantic `kind:function` family and run one
consistent workflow:

1. define the family packet under `semantic-families/<family>/`
2. run `cargo xtask family new|prove|certify <family>`
3. fill aligned, drift, under-specified, and unsupported-near-miss fixtures
4. implement the runtime classifier in `spec-core`
5. prove the family survives a true unseen corpus
6. prove the family obeys the M19 / M20 refresh, preserve, and stale truth-surface rules
7. certify the family with a reproducible report instead of "tests passed, trust me"

The output is a certification result with provenance. Either the family earned promotion, or the
milestone fails honestly and says why.

## Step 0: Scope Challenge

### Current system state

| Surface | Already proved | Still missing | M21 implication |
|---|---|---|---|
| `spec-core/src/semantic_review.rs` routing | Current function families have a real, ordered evaluator | Adding the next family is still maintainer-memory work | Reuse the router. Do not add a parallel xtask classifier path. |
| `spec-core/src/passport.rs` / `spec-core/src/export.rs` / `spec-cli/src/commands.rs` truth loop | `spec test` refreshes proof and read-side flows preserve projected truth | New families could still greenlight stale or shadowed proof unless the promotion loop checks it | Gate C must certify refresh / preserve / stale behavior, not just classifier alignment. |
| `spec-cli/tests/cli.rs` and `spec-cli/tests/m14_regressions.rs` | The repo already has a command-matrix and wedge harness for truth surfaces | Family promotion still lacks a reusable certify path | Reuse the existing matrix. Add M21 family-specific coverage, not another end-to-end harness. |
| M19 unseen falsification pack | Non-canonical aligned / drift / under-specified proof is real | There is no reusable packet contract for future families | Reuse the pack shape as the starting template for M21 packets. |
| Workspace tooling | Existing Cargo workspace and CI are enough for repo-internal tooling | There is no repo-owned orchestration crate | Add `xtask`, but keep it as orchestration only. |

### What already exists

| Sub-problem | Existing code / flow | M21 reuse decision |
|---|---|---|
| Runtime family routing | `spec-core/src/semantic_review.rs` | Reuse directly. New families must still plug into explicit ordered routing. |
| Refresh / preserve truth projection | `spec-core/src/passport.rs`, `spec-core/src/export.rs`, `spec-cli/src/commands.rs` | Reuse as Gate C target. Do not mint a second truth surface in xtask. |
| Command-matrix proof | `spec-cli/tests/cli.rs`, `spec-cli/tests/m14_regressions.rs` | Reuse as the core read-side honesty harness. |
| Unseen corpus structure | `spec-cli/tests/fixtures/m19/semantic_falsification_pack/` | Reuse as the fixture-template shape for future family packets. |
| Public contract wording | `README.md`, `AGENTS.md`, `CLAUDE.md` | Add repo-process docs only. No new end-user command docs in M21. |

### Minimum diff that still solves the problem

- Add one repo-owned packet contract under `semantic-families/`.
- Add one `xtask` crate to scaffold, prove, and certify family packets.
- Add one promoted function family, `function.wrapper.pipeline.chain3.v1`.
- Extend existing proof surfaces for Gate C and Gate D coverage.
- Add repo-process docs and certification-report conventions.

Anything beyond that is scope drift for this milestone.

### Complexity check

M21 will touch more than eight files, but the blast radius stays inside one subsystem:

- workspace root `Cargo.toml`
- new `xtask/`
- new `semantic-families/`
- `spec-core/src/semantic_review.rs`
- targeted `spec-core` tests
- targeted `spec-cli` tests and fixtures
- docs

That is still a boilable lake because it is one feature slice with one new internal crate and no
new infrastructure. What is **not** allowed is widening from `kind:function` into a general
function/data/sum promotion framework in the same milestone.

### TODOS cross-reference

- The existing TODO about the Cargo-heavy CLI harness still matters. M21 should keep most new
  semantic proof in `spec-core` tests and use CLI wedges only where read-side truth must be proven.
- No new TODO is required to land this plan. The current backlog already holds larger follow-ups
  like harness cleanup, broader semantic breadth, and future eval work.

### Completeness check

The complete version is: packet contract + orchestration crate + one real promoted family + Gate C
truth-surface checks + Gate D non-regression + provenance + docs.

Rejected shortcuts:

- harness-only with no promoted family
- family-only with no reusable certify path
- classifier-only proof with no stale / preserve / export / status validation
- checked-in full certification artifacts that create stale source-of-truth churn

### Distribution check

M21 does not introduce a new user-facing binary, package, or artifact type. Existing Cargo and CI
distribution remain enough.

The only distribution requirement is that CI can publish certification reports as build artifacts.
Those reports are repo outputs, not release assets.

## Approved Scope

- Family promotion remains **function-family-only** in M21.
- The promoted exemplar is `function.wrapper.pipeline.chain3.v1`.
- `family.toml` is the machine-readable packet manifest.
- `candidate.md` is human-readable review context only.
- `xtask` owns orchestration only.
- Runtime semantic truth remains in `spec-core`.
- Full certification output lives under `.semantic-family-artifacts/` locally and as CI artifacts.
- Public `spec` CLI surface stays unchanged.

## Architecture Review

### Ownership split

| Concern | Owner | Why |
|---|---|---|
| Runtime family truth | `spec-core` | The shipped classifier must stay in product code. |
| Refresh / preserve / stale projection | `spec-core` + `spec-cli` | Read-side truth already lives here. |
| Family packet manifest parsing | `xtask` | Repo orchestration belongs outside product semantics. |
| Family review rationale | `candidate.md` | Human-readable context is useful, but cannot be executable truth. |
| Certification output | `.semantic-family-artifacts/` + CI artifacts | Reproducible, non-authoritative build output. |

### Trust-loop diagram

```text
semantic-families/<family>/candidate.md
        │
        ├── semantic-families/<family>/family.toml
        │      └── machine manifest consumed by xtask
        │
        ├── semantic-families/<family>/fixtures/
        │      ├── aligned/
        │      ├── drift/
        │      ├── under_specified/
        │      └── unsupported_near_miss/
        │
        ├── cargo xtask family new <family>
        │      └── scaffold packet + fixture skeletons
        │
        ├── cargo xtask family prove <family>
        │      ├── run targeted spec-core family tests
        │      ├── run targeted spec-cli truth-surface tests
        │      └── validate packet completeness before certification
        │
        ├── cargo xtask family certify <family>
        │      ├── Gate A: packet + core-shape integrity
        │      ├── Gate B: unseen corpus survival
        │      ├── Gate C: refresh / preserve / stale honesty
        │      ├── Gate D: cross-family non-regression
        │      └── write certification.report.json with provenance
        │
        └── product code under pressure
               ├── spec-core/src/semantic_review.rs
               ├── spec-core/src/passport.rs
               ├── spec-core/src/export.rs
               ├── spec-cli/src/commands.rs
               └── spec-cli/tests/{cli,m14_regressions}.rs
```

### Family packet contract

Packet layout:

```text
semantic-families/<family>/
  candidate.md
  family.toml
  fixtures/
    aligned/
      Cargo.toml
      src/main.rs
      units/**/*.unit.spec
    drift/
      Cargo.toml
      src/main.rs
      units/**/*.unit.spec
    under_specified/
      Cargo.toml
      src/main.rs
      units/**/*.unit.spec
    unsupported_near_miss/
      Cargo.toml
      src/main.rs
      units/**/*.unit.spec
```

`family.toml` must be versioned and machine-readable. It carries:

- family id
- one-sentence semantic claim
- supported scope (`kind:function` only in M21)
- authored invariants
- executable body-shape grammar
- helper / dependency topology rules
- drift taxonomy
- under-specified taxonomy
- unsupported-near-miss taxonomy
- routing precedence expectations
- unseen-example requirements
- compatibility key

`candidate.md` is not executable truth. If a rule matters to certification, it must exist in
`family.toml`, runtime code, or both.

### Appendix A. Locked `family.toml` schema for M21

M21 does **not** leave the manifest shape open. `family.toml` schema version `1` is:

```toml
schema_version = 1
family = "function.wrapper.pipeline.chain3.v1"
kind = "function"
compatibility_key = "function.wrapper.pipeline.chain3.v1"
summary = "Straight-line three-call wrapper pipeline over supported function deps."

[routing]
precedence = 1
must_not_shadow = [
  "function.wrapper.pipeline.v1",
  "function.arithmetic_leaf.monotone_down_nonnegative.v1",
  "function.arithmetic_leaf.monotone_up.v1",
]

[shape]
dep_count = 3
control_flow = "straight_line_only"
return_style = "let_then_return_or_direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift", "under_specified", "unsupported_near_miss"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
```

Field rules:

- `schema_version`
  - required
  - integer
  - M21 accepts only `1`
- `family`
  - required
  - must equal the packet directory name exactly
  - regex: `^[a-z0-9]+(\\.[a-z0-9_]+)+\\.v[0-9]+$`
- `kind`
  - required
  - must equal `"function"` in M21
- `compatibility_key`
  - required
  - must equal `family`
- `summary`
  - required
  - one line only
- `routing.precedence`
  - required
  - positive integer
  - unique across supported function families
- `routing.must_not_shadow`
  - required
  - non-empty array for every M21 family
- `shape.dep_count`
  - required
  - exact integer
  - exemplar family requires `3`
- `shape.control_flow`
  - required enum
  - M21 accepts only `"straight_line_only"`
- `shape.return_style`
  - required enum
  - M21 accepts `"direct_return"` or `"let_then_return_or_direct_return"`
- `shape.loops`, `shape.branching`, `shape.requires_supported_function_deps`
  - required booleans
- `args.threading`
  - required enum
  - M21 accepts only `"ordered_passthrough"`
- `args.allow_nested_argument_expressions`, `args.allow_literal_only_extra_args`
  - required booleans
- `corpus.required_buckets`
  - required
  - must be exactly `["aligned", "drift", "under_specified", "unsupported_near_miss"]`
- `corpus.min_cases_per_bucket`
  - required
  - integer `>= 1`
- `truth_surface.*`
  - all required
- `gates.*`
  - all required
  - all must be `true` for M21

Rejected in M21:

- arbitrary extra top-level keys
- family-specific escape-hatch manifest fields
- Markdown-only metadata that is not repeated in `family.toml`
- packet-local routing rules that disagree with runtime routing order

### Appendix B. Locked fixture contract for M21

Each bucket is a self-contained crate-root fixture, not a partial fragment. This is deliberate. It
duplicates small `Cargo.toml` and `src/main.rs` files, but it makes the implementation boring and
removes hidden assembly logic from xtask.

Required bucket shape:

```text
fixtures/<bucket>/
  Cargo.toml
  src/main.rs
  units/<namespace>/<case_name>.unit.spec
```

Allowed bucket names:

- `aligned`
- `drift`
- `under_specified`
- `unsupported_near_miss`

Rules:

- each bucket must exist
- each bucket must contain at least one `.unit.spec`
- each `.unit.spec` filename must be unique within the packet
- `Cargo.toml` and `src/main.rs` are checked in for every bucket
- xtask must reject symlinks anywhere under `fixtures/`
- xtask must reject any non-`.unit.spec` file under `units/`

Case naming rule:

- aligned cases end with `_aligned.unit.spec`
- drift cases end with `_drift.unit.spec`
- under-specified cases end with `_under_specified.unit.spec`
- unsupported near misses end with `_unsupported_near_miss.unit.spec`

The exemplar family must use namespace `pricing/` and case ids prefixed with `checkout_` or
`pricing_`. No generic `example_1` names.

Exact bucket expectations for the exemplar family:

- `aligned`
  - valid three-step wrapper pipeline
  - uses exactly three supported function deps
- `drift`
  - shape is wrapper-like but semantic argument flow is wrong
- `under_specified`
  - authored truth is too weak to prove ordered passthrough
- `unsupported_near_miss`
  - body is close, but falls outside the admitted family by control flow or dep topology

`candidate.md` must list every fixture file once under these four headings, but xtask must treat
the file system as the source of truth.

### Certification gate contract

**Gate A: Core-shape integrity**

- bounded family scope
- explicit runtime matcher
- explicit drift and under-specified reasons
- explicit unsupported-near-miss exclusions
- explicit precedence expectations

**Gate B: True unseen-example survival**

- aligned examples
- drift examples
- under-specified examples
- unsupported-near-miss examples
- non-canonical names and alternate ids

**Gate C: Truth-surface honesty**

- `spec test` is the only proof refresh path
- `spec build`, `spec generate`, `spec status`, and `spec export` preserve but do not mint proof
- stale proof demotes correctly after semantic changes
- read-side surfaces agree on projected truth

**Gate D: Cross-family non-regression**

- existing Family A examples stay Family A
- existing Family B examples stay Family B
- unsupported near misses do not accidentally promote
- runtime routing order matches packet precedence expectations

### Appendix C. Locked routing precedence for M21

Function-family routing order is locked for this milestone:

1. `function.wrapper.pipeline.chain3.v1`
2. `function.wrapper.pipeline.v1`
3. `function.arithmetic_leaf.monotone_down_nonnegative.v1`
4. `function.arithmetic_leaf.monotone_up.v1`
5. `unsupported.function.v1`

Why this order:

- `chain3` is the most specific wrapper family added by M21
- `pipeline.v1` remains the existing two-dep wrapper family
- arithmetic leaves are structurally separate but still ordered explicitly
- unsupported remains the final catch-all

Required runtime behavior:

- routing code must encode this order explicitly in `spec-core/src/semantic_review.rs`
- the packet manifest `routing.precedence` value for `chain3` must be `1`
- Gate D must fail if runtime order and manifest order diverge
- no family may rely on hash-map or iteration-order behavior

### Appendix D. Locked command matrix and report contract

`cargo xtask family new <family>`

- inputs
  - one family id matching the manifest regex
- writes
  - packet directory only if it does not already exist
- exit codes
  - `0` success
  - `2` invalid family id or unsafe path
  - `3` packet already exists
  - `4` write failure

`cargo xtask family prove <family>`

- must run these steps in order
  1. validate family id and packet path safety
  2. parse `family.toml`
  3. validate packet layout and bucket completeness
  4. run `cargo test -p spec-core m21_chain3_classifier_ -- --nocapture`
  5. run `cargo test -p spec-cli --test cli m21_chain3_truth_surface_ -- --nocapture`
  6. run `cargo test -p spec-cli --test m14_regressions m21_chain3_corpus_ -- --nocapture`
- required naming convention
  - every new M21 prove-level test must begin with one of:
    - `m21_chain3_classifier_`
    - `m21_chain3_truth_surface_`
    - `m21_chain3_corpus_`
- outputs
  - always write `.semantic-family-artifacts/semantic-families/<family>/prove.latest.json`
- exit codes
  - `0` all prove steps passed
  - `2` invalid packet or manifest
  - `3` suite failure
  - `4` artifact write failure

`cargo xtask family certify <family>`

- must run `family prove <family>` first
- then must run these extra suites in order
  7. run `cargo test -p spec-core m21_chain3_regression_ -- --nocapture`
  8. run `cargo test -p spec-cli --test m14_regressions m21_chain3_regression_ -- --nocapture`
- required naming convention
  - every new M21 certify-only regression test must begin with one of:
    - `m21_chain3_regression_`
- outputs
  - always write `.semantic-family-artifacts/semantic-families/<family>/attempt-<timestamp>.json`
  - write or replace `.semantic-family-artifacts/semantic-families/<family>/certification.report.json`
    **only when all gates pass**
- exit codes
  - `0` all gates passed
  - `2` invalid packet or manifest
  - `3` prove-level suite failure
  - `4` certify-level suite failure or gate failure
  - `5` artifact write failure

Locked gate-to-suite mapping:

| Gate | Source of truth | Pass condition |
|---|---|---|
| Gate A | manifest validation + `m21_chain3_classifier_` | manifest valid and classifier suite green |
| Gate B | `m21_chain3_corpus_` | all four buckets classified as expected |
| Gate C | `m21_chain3_truth_surface_` | refresh / preserve / stale assertions all green |
| Gate D | `m21_chain3_regression_` | Family A / B stability and no shadowing proven |

Locked report schema:

```json
{
  "schema_version": 1,
  "family": "function.wrapper.pipeline.chain3.v1",
  "manifest_schema_version": 1,
  "git_commit_sha": "abc1234",
  "rust_toolchain": "rustc 1.89.0",
  "generated_at": "2026-04-27T00:00:00Z",
  "overall_status": "pass",
  "gates": {
    "gate_a": { "status": "pass" },
    "gate_b": { "status": "pass" },
    "gate_c": { "status": "pass" },
    "gate_d": { "status": "pass" }
  },
  "suites": [
    {
      "name": "spec-core:m21_chain3_classifier_",
      "command": ["cargo", "test", "-p", "spec-core", "m21_chain3_classifier_", "--", "--nocapture"],
      "exit_code": 0,
      "status": "pass"
    }
  ],
  "fixture_digests": [
    {
      "bucket": "aligned",
      "path": "fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec",
      "sha256": "..."
    }
  ]
}
```

Required report enums:

- `overall_status`: `pass | fail`
- `gate_*.status`: `pass | fail`
- `suite.status`: `pass | fail`

No optional gate objects. No free-form verdict strings. No parsing plain cargo stdout to infer gate
names after the fact.

### Code seams under pressure

- `spec-core/src/semantic_review.rs`
  Add the new family matcher, routing precedence, and family-local test coverage.
- `spec-core/src/passport.rs` / `spec-core/src/export.rs`
  Touch only if Gate C reveals a missing projection or provenance surface. Do not widen these
  files preemptively.
- `spec-cli/src/commands.rs`
  Touch only if the existing proof / preserve flow needs a new structured hook for M21 tests.
- `xtask/src/...`
  Keep the orchestration pipeline explicit and comment it with an inline ASCII pipeline diagram if
  the command wiring grows beyond a straight line.

### Error & Rescue Registry

| Risk | Failure shape | Rescue |
|---|---|---|
| Harness-first drift | M21 ships packet ceremony without proving one useful family | Require one promoted family in the same milestone and fail if it cannot certify. |
| Hidden manifest DSL | `candidate.md` or one-off TOML fields become de facto runtime truth | Keep `family.toml` minimal and versioned, and reject family-specific escape-hatch fields. |
| xtask becomes semantic engine #2 | Certification logic reimplements classifier semantics | Consume structured runtime outputs only. Keep semantic truth in `spec-core`. |
| Precedence regression | New family steals Family A / B matches | Gate D is required, not optional. |
| Non-reproducible certification | Two engineers get different green answers | Report SHA, toolchain, manifest version, fixture digests, and exit codes. |

## Code Quality Review

M21 should bias toward explicit and boring:

- one manifest format, not layered Markdown parsing
- one routing authority, not a shadow xtask classifier
- one certification report format, not test-text scraping
- one promoted family, not a framework that guesses future kinds

The main complexity hotspots already exist:

- `spec-core/src/semantic_review.rs` is a large ordered classifier
- `spec-cli/tests/cli.rs` is a large integration-matrix file

That means M21 should avoid:

- new per-family exceptions in packet schema
- clever generic abstractions that hide routing order
- duplicated truth-surface rules in both product code and xtask
- widening `spec-cli` command behavior when tests alone would do

Inline ASCII comments are worth adding in these implementation files if the final diff becomes
non-trivial:

- `xtask/src/family/certify.rs` or equivalent certification pipeline file
- `spec-core/src/semantic_review.rs` near the routing order for the promoted family
- any new xtask manifest-validation module if it accumulates multiple gate phases

## Test Review

### Code path coverage to add

```text
CODE PATH COVERAGE
===========================
[+] cargo xtask family new <family>
    ├── valid scaffold
    ├── invalid family id
    ├── path traversal attempt
    └── packet root symlink rejection

[+] family.toml manifest validation
    ├── valid manifest
    ├── missing required field
    ├── bad schema version
    └── contradictory precedence / scope metadata

[+] cargo xtask family prove <family>
    ├── targeted suite selection
    ├── missing packet assets
    ├── empty fixture bucket
    ├── duplicate fixture ids
    └── interrupted or failing proof run

[+] cargo xtask family certify <family>
    ├── all gates pass
    ├── Gate A fails
    ├── Gate B fails
    ├── Gate C fails
    ├── Gate D fails
    └── failed run preserves last known good success artifact

[+] function.wrapper.pipeline.chain3.v1 runtime routing
    ├── aligned examples
    ├── drift examples
    ├── under-specified examples
    ├── unsupported near misses
    └── precedence against Family A / Family B

[+] read-side truth surfaces
    ├── spec test refreshes proof
    ├── spec build preserves but does not mint proof
    ├── spec generate preserves but does not mint proof
    ├── spec status projects fresh vs stale honestly
    └── spec export projects fresh vs stale honestly
```

### Operator-flow coverage

```text
MAINTAINER FLOW
===============
Author chooses family
    │
    ├── cargo xtask family new <family>
    │       └── scaffold is correct, safe, and repeatable
    │
    ├── author fills family.toml + fixtures
    │       └── manifest and fixture completeness are validated
    │
    ├── runtime classifier added in spec-core
    │       └── aligned / drift / under-specified / unsupported behavior proven
    │
    ├── cargo xtask family prove <family>
    │       └── targeted suites fail loudly on missing or stale proof
    │
    └── cargo xtask family certify <family>
            └── emits reproducible report or fails with gate-local reason
```

### Required test split

- `xtask`
  - scaffold generation
  - manifest parsing and validation
  - artifact-path safety
  - no-overwrite-on-failure behavior
  - provenance fields
- `spec-core`
  - promoted family classifier tests
  - precedence and shadowing tests against Family A / Family B
  - stale invalidation when semantic inputs change
- `spec-cli`
  - refresh / preserve / stale command matrix
  - export / status projection assertions
  - unseen corpus integration coverage
- fixtures
  - aligned
  - drift
  - under-specified
  - unsupported near miss

Locked test naming contract:

- prove-level `spec-core` tests: `m21_chain3_classifier_*`
- prove-level `spec-cli` truth-surface tests: `m21_chain3_truth_surface_*`
- prove-level `m14_regressions` corpus tests: `m21_chain3_corpus_*`
- certify-level regression tests in `spec-core` or `m14_regressions`: `m21_chain3_regression_*`

If a new M21 test does not follow this prefix scheme, it is outside the xtask command matrix and
therefore outside the boring implementation path for this milestone.

### Test plan artifact

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-m20-test-plan-20260427-123346.md`

### Verification loop

Use this as the implementation-time proof loop:

```bash
cargo test -p xtask
cargo test -p spec-core semantic_review -- --nocapture
cargo test -p spec-cli --test cli -- --nocapture
cargo test -p spec-cli --test m14_regressions -- --nocapture
cargo xtask family prove function.wrapper.pipeline.chain3.v1
cargo xtask family certify function.wrapper.pipeline.chain3.v1
```

If `family prove` or `family certify` require the entire repo test matrix every local run, the plan
is wrong and needs to narrow the suite split before landing.

## Performance Review

The real performance risk is developer-loop drag, not runtime latency.

If `cargo xtask family prove` reruns the entire semantic and CLI matrix on every small fixture
change, engineers will bypass it. M21 therefore needs a deliberate split:

- `family prove`
  - impacted, family-local suites
  - fast enough for local iteration
- `family certify`
  - full family-local proof
  - full Gate C truth-surface matrix
  - full Gate D cross-family non-regression
- CI / nightly
  - broader regression confirmation if needed

The goal is explicit: local proof should stay targeted; certification should stay complete.

## Security and Trust Boundary Review

This is repo tooling, but it still reads manifests and writes files. Treat it like real code:

- reject invalid family ids
- reject path traversal
- reject symlink packet roots
- keep writes inside `semantic-families/` or `.semantic-family-artifacts/`
- treat packet inputs as trusted repo-owned source, not arbitrary external uploads
- avoid shelling out with interpolated unvalidated paths

No security theater. One bad path join here is enough to make local proof tooling sketchy.

## NOT in Scope

- generic automatic family discovery
- LLM-generated family design without human review
- public `spec family ...` commands
- second-language implementation work
- `kind:data` or `kind:sum` promotion
- semantic ontology redesign
- multiple new families in the same milestone
- unsupported-path redesign beyond the already-landed M20 truth surface
- checked-in full certification reports

## Implementation Order

### M21a. Lock the packet contract and scaffold the exemplar packet

Deliver:

- `semantic-families/README.md` or equivalent packet-contract doc
- `semantic-families/function.wrapper.pipeline.chain3.v1/`
- `family.toml`
- `candidate.md`
- fully scaffolded aligned / drift / under-specified / unsupported-near-miss crate buckets

Acceptance:

- packet layout is versioned and documented
- exemplar family packet exists before runtime code changes begin
- no family-specific manifest escape-hatch fields are required
- all four fixture buckets are self-contained crate roots with checked-in `Cargo.toml` and `src/main.rs`

### M21b. Add the `xtask` crate and scaffold / validate flow

Deliver:

- workspace member in root `Cargo.toml`
- `cargo xtask family new <family>`
- manifest parsing and validation
- safe packet-path creation
- exact exit-code behavior from Appendix D

Acceptance:

- `family new` creates the right tree
- invalid ids and unsafe paths fail with no writes
- xtask remains orchestration-only

### M21c. Encode the reusable certification gate

Deliver:

- `cargo xtask family prove <family>`
- `cargo xtask family certify <family>`
- certification report schema
- provenance
- no-overwrite-on-failure semantics
- Gate A / B / C / D evaluation driven by structured suite outputs
- exact command matrix and stable test-prefix selection from Appendix D

Acceptance:

- a failed gate cannot emit a false success artifact
- certification output is reproducible
- xtask consumes structured runtime proof and does not reimplement semantic truth

### M21d. Make the gate enforce M19 / M20 truth-surface honesty

Deliver:

- Gate C CLI and projection coverage
- stale invalidation coverage
- export / status read-side assertions
- refresh / preserve semantics wired into `prove` / `certify`

Acceptance:

- `spec test` is the only refresh path
- `spec build`, `spec generate`, `spec status`, and `spec export` do not mint proof
- stale proof drops or demotes exactly where M19 / M20 rules require

### M21e. Promote the exemplar family through the harness

Deliver:

- runtime matcher for `function.wrapper.pipeline.chain3.v1`
- explicit routing precedence relative to Family A / Family B, using Appendix C order
- aligned / drift / under-specified / unsupported evaluator tests
- complete exemplar fixture pack
- Family A / Family B non-regression coverage

Acceptance:

- the family classifies the exemplar corpus correctly
- precedence is explicit and test-backed
- the new family does not steal existing matches
- the exemplar certifies end to end through `family prove` and `family certify`

### M21f. Final docs and repo-process polish

Deliver:

- README / AGENTS process docs for future family promotion
- stable report-location convention
- final milestone gate wording in the plan and docs

Acceptance:

- a maintainer can propose the next family without reverse-engineering M21 history
- docs describe repo workflow, not new public CLI product surface

## Failure Modes Registry

| Codepath | Real failure | Test coverage required | Error handling required | User signal | Critical gap if omitted |
|---|---|---|---|---|---|
| `family new` | writes outside packet root or follows symlink | yes | yes | explicit CLI failure | yes |
| manifest validation | contradictory or incomplete family contract passes | yes | yes | explicit CLI failure | yes |
| runtime routing | new family shadows Family A / B | yes | yes | failing proof / certify gate | yes |
| Gate C | stale proof still looks fresh on `status` / `export` | yes | yes | failing CLI tests and certify gate | yes |
| Gate D | unsupported near miss promotes into new family | yes | yes | failing certify gate | yes |
| certification report write | failed run overwrites last known good artifact | yes | yes | explicit certify failure | yes |
| local prove loop | prove is so slow people stop using it | yes | partial | developer friction, not silent | no, but milestone quality drops |

Any row in the table above that ships without test coverage **and** without explicit failure
signaling is a red milestone, not a paper cut.

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
|---|---|---|
| Packet contract + exemplar scaffold | `semantic-families/`, docs | — |
| xtask scaffold / validate flow | `xtask/`, workspace root | Packet contract + exemplar scaffold |
| Runtime family implementation | `spec-core/`, `semantic-families/fixtures/` | Packet contract + exemplar scaffold |
| CLI truth-surface matrix | `spec-cli/`, `spec-cli/tests/`, `spec-cli/tests/fixtures/` | Runtime family implementation |
| Certification aggregation | `xtask/`, `.semantic-family-artifacts/` convention | xtask scaffold / validate flow, CLI truth-surface matrix |
| Final docs and report polish | docs, `PLAN.md` | Certification aggregation |

### Parallel lanes

Lane 0: packet contract + exemplar scaffold  
Sequential because every other lane depends on the manifest shape.

Lane A: xtask scaffold / validate flow  
Independent after Lane 0. Touches `xtask/` and workspace root only.

Lane B: runtime family implementation  
Independent after Lane 0. Touches `spec-core/` and the exemplar family fixtures.

Lane C: CLI truth-surface matrix  
Sequential after Lane B because it depends on the promoted family existing in runtime routing.

Lane D: certification aggregation  
Sequential after Lane A + Lane C because it consumes both xtask command plumbing and proven suite
outputs.

Lane E: final docs and report polish  
Sequential after Lane D.

### Execution order

1. Launch Lane 0 first.
2. After Lane 0 lands, launch Lane A and Lane B in parallel worktrees.
3. After Lane B lands, run Lane C.
4. After Lane A and Lane C both land, run Lane D.
5. Finish with Lane E.

### Conflict flags

- Lane A and Lane D both touch `xtask/`. Keep them sequential.
- Lane B and Lane C both depend on the promoted family fixtures and runtime behavior. Keep them
  sequential.
- Lane 0 must stay single-owner. Packet schema churn during parallel work would create fake
  conflicts everywhere.

## Green Gate

M21 is green only if all of these are true:

- `semantic-families/function.wrapper.pipeline.chain3.v1/` exists with the locked packet layout
- `family.toml` is the machine-readable contract
- `candidate.md` is review-only context
- all four fixture buckets are self-contained crate roots with at least one `.unit.spec` each
- `cargo xtask family new|prove|certify` all exist and fail safely
- the promoted family classifies aligned / drift / under-specified / unsupported unseen examples
- Gate C proves refresh / preserve / stale honesty
- Gate D proves no Family A / Family B shadowing
- certification output includes provenance
- public `spec` CLI surface did not expand

## Red Gate

M21 is red if any of these happen:

- `family.toml` requires family-specific one-off fields to support the exemplar
- xtask reimplements semantic truth instead of consuming runtime proof
- the promoted family cannot certify without special-case packet behavior
- runtime routing order differs from Appendix C
- stale proof still reads as current on `status` or `export`
- the new family steals existing Family A or Family B matches and no bounded precedence fix exists
- local `family prove` is so broad that the intended developer loop is unusable

## Decision Audit Trail

| # | Phase | Decision | Classification | Rationale |
|---|---|---|---|---|
| 1 | Intake | Replace the stale M20 draft with an M21 implementation contract | Mechanical | Reviewing the wrong milestone would have invalidated the whole pass. |
| 2 | CEO | Keep `xtask` as orchestration-only | Mechanical | Semantic truth already has clear product homes. |
| 3 | CEO | Keep the harness milestone, but add external-value pressure and kill criteria | User challenge absorbed into plan | The milestone needed stronger outcomes, not a total reframing. |
| 4 | Eng | Narrow M21 to `kind:function` only | Mechanical | Current evaluator architecture supports function families cleanly; seam-family generalization does not belong here. |
| 5 | Eng | Choose `function.wrapper.pipeline.chain3.v1` as the exemplar | Taste resolved | It adds real pricing / checkout leverage without forcing seam refactors. |
| 6 | Eng | Make `family.toml` authoritative and `candidate.md` review-only | Mechanical | Avoids hidden Markdown DSL drift. |
| 7 | Eng | Keep full certification output out of checked-in source | Mechanical | Avoids stale truth and merge churn while preserving reproducible artifacts. |
| 8 | Eng | Add Gate D as a required certification gate | Mechanical | Ordered routing makes shadowing a correctness risk, not a nice-to-have. |

## Completion Summary

- Step 0: Scope Challenge — accepted with one narrowing: function-family-only
- Architecture Review — integrated into the plan with locked ownership and gate boundaries
- Code Quality Review — integrated, with explicit anti-duplication and anti-DSL constraints
- Test Review — coverage diagram written, required suites and verification loop locked
- Performance Review — `prove` vs `certify` split locked
- NOT in scope — written
- What already exists — written
- TODOS.md updates — none required for this milestone pass
- Failure modes — critical gaps identified up front and turned into required gates
- Outside voice — ran via `/autoplan` (`codex+subagent`)
- Parallelization — 6 steps, 2 parallel lanes, 4 sequential lanes
- Lake Score — complete version chosen over shortcut in every material scope decision

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR via `/autoplan` | mode: `SELECTIVE_EXPANSION`, 0 critical gaps |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR via `/autoplan` | 0 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | skipped, no UI scope |

**CROSS-MODEL:** CEO voices converged on stronger kill criteria and leverage pressure. Eng voices
converged on function-only scope, machine-manifest authority, provenance, and required
non-regression coverage.

**UNRESOLVED:** 0

**VERDICT:** CEO + ENG CLEARED — ready to implement.
