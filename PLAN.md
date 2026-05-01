<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-spec/feat-corpus-expansion-autoplan-restore-20260501-192409.md -->
# M27.8R - Fixture-Owned Harness Truth Repair

Status: **implementation contract**  
Base branch: **main**  
Working branch: **feat/corpus-expansion**  
Nearest source-truth branches: **`ws/m27_8-int`, `ws/m27_8-lane-a`, `ws/m27_8-lane-b`**  
Last rewritten: **2026-05-01**

## Summary

The old M27.8 plan is obsolete.

The integrated M27.8 run already proved the intended product truth on the
`ws/m27_8-*` worktree branches. It stopped only because the final seeded `xtask`
command-path lock replayed a different world and observed
`coverage.function_coverage.promoted_family_units == 10` instead of the locked
integrated truth `15`.

This follow-up is not another corpus-expansion milestone. It is a harness-truth
repair milestone.

The job is:

1. recover the already-proven lane-A authored source truth from the `ws/m27_8-*`
   branches
2. keep the ranked command-path assertions from the blocked lane-B work
3. repair the seeded workspace so it copies the promoted packet truth that the
   failing command-path coverage path actually consumes
4. rerun the exact proof loop until the final `cargo test -p xtask -- --color never`
   locks the same truth already observed in the blocked integration run

If this lands, the repo can finally trust the final lock again. If it does not,
stop and re-plan from the next mismatch with captured seeded evidence, not with a
second guess.

## Plan Authority

This file supersedes the earlier "Crosslib Arithmetic Confirmation Pack" plan.

Primary sources:

- [spensermcconnell-feat-corpus-expansion-design-20260501-191122.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-spec/spensermcconnell-feat-corpus-expansion-design-20260501-191122.md)
- `.runs/m27_8/acceptance.md`
- `.runs/m27_8/merge-log.md`
- `.runs/m27_8/contract-freeze.json`
- `xtask/src/lib.rs`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/inventory.rs`
- `semantic-families/corpus/rust-function.toml`

Repo truth checked while writing this plan:

- current branch `feat/corpus-expansion` does **not** contain the recovered
  `examples/crosslib-app/units/pricing/apply_tax.unit.spec` source truth
- the blocked run artifacts show that the integrated truth existed on
  `ws/m27_8-int`
- `xtask/src/lib.rs` on the current branch still contains the stale
  `recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate()`
  lock and a seeded workspace copier that omits
  `semantic-families/function.wrapper.pipeline.v1`
- the failing command-path test calls `recommend::run_with_writer()`, which flows
  through `coverage::collect_latest()`, the corpus manifest, and the promoted
  family inventory, not through cross-library `spec.toml` loading

Durable truth lives in the blocked run artifacts:

- `.runs/m27_8/acceptance.md`
- `.runs/m27_8/merge-log.md`
- `.runs/m27_8/contract-freeze.json`

The `ws/m27_8-*` branches are recovery sources for authored files, not the only
authority.

## Problem Statement

The repo already learned the important product fact:

- integrated coverage truth was `28 / 15 / 0 / 13`
- integrated recommendation truth was `ranked`
- arithmetic was first and `ready`
- `money/round` remained second and `hold` for `unknown_overlap_family`

That truth was observed before the stop in `.runs/m27_8/acceptance.md`.

What failed was narrower:

- the final seeded command-path test in `xtask/src/lib.rs`
- specifically, the copied mini-repo world did not reproduce the same promoted
  family count as the integrated run

The strongest concrete mechanism is now visible in code:

1. `recommendation_command_path_writes_same_bytes...` calls
   `recommend::run_with_writer()`
2. that path flows through `coverage::collect_latest()`
3. `collect_latest()` loads:
   - the corpus manifest `semantic-families/corpus/rust-function.toml`
   - the promoted family inventory from `xtask/src/family/inventory.rs`
4. promoted families are derived from which promoted packet roots exist in the
   seeded workspace
5. `seed_locked_recommendation_workspace()` currently copies:
   - `semantic-families/function.wrapper.pipeline.chain3.v1`
   - `semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1`
   - `semantic-families/function.arithmetic_leaf.monotone_up.v1`
6. but it does **not** copy `semantic-families/function.wrapper.pipeline.v1`

That omission explains the observed `15 -> 10` promoted-family drop cleanly:
five wrapper-family units lose promoted status when the promoted packet root is
missing from the seeded workspace.

Wild. The branch that proved the feature is not the branch the final copied test
actually simulates.

## Milestone Outcome

When M27.8R lands, the repo can truthfully claim:

- the already-proven M27.8 source truth has been recovered from the `ws/m27_8-*`
  branches onto `feat/corpus-expansion`
- the seeded command-path harness now copies the promoted packet root required to
  model the same promoted-family inventory the integrated run used
- the final `xtask` command-path lock reproduces the same ranked truth already
  observed in the blocked integration run
- the repo did not widen scope into recommendation policy, coverage policy,
  schema changes, or broader test-architecture redesign

M27.8R does **not** claim:

- a new corpus experiment
- new recommendation logic
- new coverage logic
- new artifact schemas
- a repo-wide replacement of copied workspace fixtures
- M28 shared-core extraction

## Step 0 - Scope Challenge

### What already exists

| Sub-problem | Existing code or artifact | Reuse decision |
|---|---|---|
| Exact authored `apply_tax` source truth | `.runs/m27_8/contract-freeze.json`, merge commit `ab11249`, branch `ws/m27_8-lane-a` | Reuse literally. Do not re-invent the unit. |
| Ranked command-path assertion shape | merge commit `7ae58ae`, branch `ws/m27_8-lane-b` | Reuse as the starting point. Repair the harness around it. |
| Integrated acceptance truth | `.runs/m27_8/acceptance.md` | Reuse as the contract oracle. |
| Seeded workspace helper seam | `xtask/src/lib.rs::seed_locked_recommendation_workspace()` | Reuse and repair. Do not introduce a new harness framework. |
| Promoted-family inventory model | `xtask/src/family/inventory.rs` | Reuse as-is. Feed it the missing promoted packet root. |
| Existing proof loop order | `.runs/m27_8/contract-freeze.json.required_build_order` | Reuse exactly. |

### Minimum honest change

The smallest complete diff is still three tracked source files:

1. `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
2. `examples/crosslib-app/units/.gitignore`
3. `xtask/src/lib.rs`

But the meaning of those three files has changed:

- files 1 and 2 are recovery of already-proven source truth from `ws/m27_8-lane-a`
- file 3 is no longer "update the lock to ranked truth"
- file 3 is now "preserve the ranked truth from `ws/m27_8-lane-b`, then repair the
  seeded promoted-packet inputs so the copied workspace actually reproduces it"

Anything larger is scope creep.

Anything smaller leaves the repo in the same bad state where the integrated run
and the final lock disagree about reality.

### Alternatives considered

| Alternative | Why deferred |
|---|---|
| Real temp checkout instead of copied workspace harness | Bigger test-model change than this follow-up needs. Only justified if the seeded packet-root fix plus diagnostic gate still leave unexplained drift. |
| Checked-in seed manifest for every copied input | Legitimate follow-on hardening, but too much ceremony for the immediate three-file repair. |
| Artifact-hash-only lock without reconstructing the world | Too weak for this milestone because the failure is about the reconstructed world disagreeing with integrated truth. |

### Complexity check

This plan touches exactly three tracked source files and introduces zero new Rust
modules, services, or harness frameworks.

Good. Boring. Keep it that way.

### Search check

- **[Layer 1]** Reuse the existing blocked-run artifacts and `ws/m27_8-*` commits
  instead of inventing a new recovery path.
- **[Layer 1]** Reuse the existing seeded workspace helper instead of building a new
  fixture system.
- **[Layer 3]** The important insight is not "config is missing." It is that the
  command-path test consumes promoted packet presence through inventory, and the
  seeded workspace omits `semantic-families/function.wrapper.pipeline.v1`.

### TODOS cross-reference

`TODOS.md` contains no deferred item that blocks this plan directly.

This plan also does not justify a new TODO yet. If the harness still disagrees
after copying `function.wrapper.pipeline.v1`, that follow-up becomes a fresh plan
item with captured evidence, not a vague TODO.

### Completeness check

Do the complete version now:

- recover the exact lane-A source truth
- preserve the ranked lane-B assertions
- copy the missing promoted packet root into the seeded workspace
- add an explicit diagnostic stop gate if promoted-family count is still wrong
- rerun the exact proof loop
- require the final `xtask` test to match the integrated truth already proven

Do **not** do the shortcut version where a plausible but unproven input gets
patched first and only then investigated. The complete fix here is to target the
seeded input the failing codepath actually consumes, then require a diagnostic
diff if the count still disagrees.

### Distribution check

No new artifact type is introduced.

This is repo-internal governance work. The maintainer-facing surface stays:

- `cargo xtask family coverage --format json`
- `cargo xtask family recommend --format json`
- `cargo xtask family validate-artifact <path>`
- `cargo test -p xtask -- --color never`

## Scope

### In Scope

- recover the exact lane-A authored files from `ws/m27_8-lane-a` or merge commit
  `ab11249`
- preserve the ranked lock shape from `ws/m27_8-lane-b` or merge commit `7ae58ae`
- repair `seed_locked_recommendation_workspace()` so it copies:
  - `semantic-families/function.wrapper.pipeline.v1`
- add a diagnostic stop gate:
  - if `promoted_family_units` is still not `15`, dump and diff the seeded
    inventory and coverage outputs before any further edits
- keep the locked ranked assertions aligned with the blocked integrated truth
- rerun the exact integrated proof loop from `.runs/m27_8/contract-freeze.json`
- stop again on the first mismatch

### NOT In Scope

- editing `semantic-families/corpus/rust-function.toml`
- changing `xtask/src/family/coverage.rs`
- changing `xtask/src/family/recommend.rs`
- changing `xtask/src/family/promotion_artifacts.rs`
- changing recommendation or coverage schemas
- rewriting `.runs/m27_8/*` historical artifacts
- replacing copied-workspace harnesses repo-wide
- M28 work

## Architecture Review

### Architecture recommendation

Use the blocked run as the oracle and repair the existing harness seam in place.

Do not add a new fixture layer. Do not generalize this into a new test framework.
This repo already has the seam it needs.

### Architecture ASCII diagram

```text
CURRENT BRANCH
==============
feat/corpus-expansion
    │
    ├── missing recovered lane-A source truth
    │   ├── apply_tax.unit.spec absent
    │   └── apply_tax passport whitelist absent
    │
    └── stale xtask harness
        ├── stale no_strong_candidate test name + baseline
        └── seeded workspace omits wrapper.pipeline.v1 packet root

RECOVERY PATH
=============
ws/m27_8-lane-a / ab11249
    └── recover exact authored apply_tax source truth

ws/m27_8-lane-b / 7ae58ae
    └── preserve ranked command-path assertions

HARNESS REPAIR
==============
xtask/src/lib.rs
    └── seed_locked_recommendation_workspace()
        ├── copy corpus unit trees
        ├── copy wrapper.pipeline.chain3.v1
        ├── copy arithmetic promoted packets
        ├── copy wrapper.pipeline.v1                  <-- add
        └── run the same recommend path as the lock

PROOF LOOP
==========
shared-spec build
    -> exact apply_tax proof
    -> crosslib build
    -> crosslib tests
    -> coverage json
    -> recommendation json
    -> artifact validation
    -> xtask command-path lock
    -> if still red: dump seeded inventory + coverage diff, then stop
```

### Architecture findings

- **[P1] (confidence: 10/10) `xtask/src/lib.rs:3707` +
  `xtask/src/family/inventory.rs:116`** — the seeded workspace helper omits
  `semantic-families/function.wrapper.pipeline.v1`, and promoted-family inventory is
  derived from which promoted packet roots exist. Recommendation: copy the missing
  promoted packet root first. This is the cleanest explanation for the exact
  `15 -> 10` promoted-family drop.
- **[P1] (confidence: 9/10) `.runs/m27_8/acceptance.md:1` + current branch** —
  the source truth proven in `ws/m27_8-int` is not present on the working branch.
  Recommendation: recover the exact lane-A authored files from the `ws/*` source
  branch or merge commit, not by fresh manual re-authoring.
- **[P1] (confidence: 9/10) plan boundary** — even the promoted-packet explanation
  is still a hypothesis until the final lock reruns. Recommendation: if promoted
  family count remains wrong after adding `function.wrapper.pipeline.v1`, stop and
  diff seeded inventory/coverage outputs before editing anything else.

### Realistic production failure scenarios

| Codepath | Failure scenario | Accounted for? |
|---|---|---|
| seeded workspace copy list | future maintainer omits one promoted packet root and silently demotes a family | Yes, if the ranked command-path test stays the lock and the seed list comment explains promoted packet truth |
| recovered `apply_tax` source truth | maintainer "recreates" the unit with a slightly different body or intent and silently changes cluster semantics | Yes, by reusing exact source truth from `ab11249` / contract freeze |
| ranked lock assertions | maintainer updates ranked output expectations without reproducing integrated proof | Yes, by binding every assertion to `.runs/m27_8/acceptance.md` truth |
| proof loop rerun | crosslib exact-unit proof passes but final lock still seeds a divergent world | Yes, this is the exact condition the plan exists to catch |

## Code Quality Review

### Recommendation

Bias hard toward explicit over clever.

Use one literal path addition in the seeded copy list and a short adjacent comment
explaining why that packet root is required. Do not introduce a fixture bundle type,
path registry abstraction, or helper that tries to "discover" promoted packet roots.

### Findings

- **[P2] (confidence: 9/10) `xtask/src/lib.rs`** — the current seed list hides a
  non-obvious dependency: promoted-family truth depends on promoted packet roots
  existing in the copied workspace. Recommendation: add a short comment above the
  seed list saying the command-path workspace must copy both corpus units and every
  promoted packet root the inventory path expects.
- **[P2] (confidence: 8/10) plan scope** — recovering the lane-A diff and repairing
  the seed helper are structural and behavioral changes, but they stay isolated to
  three files. Recommendation: keep them in one milestone because the recovered
  source truth is a prerequisite for the harness assertion meaning anything.

### DRY / engineering-enough rule

- Reuse the exact authored lane-A file contents. No duplicated "temporary" spec.
- Reuse the ranked lane-B assertions. No parallel new command-path test.
- Reuse the existing seed helper. No second helper that only differs by one packet path.

## Test Review

100% coverage is the goal here, and this plan can get there without adding a second
test family.

### Code path coverage

```text
CODE PATH COVERAGE
==================
[+] examples/crosslib-app/units/pricing/apply_tax.unit.spec
    │
    ├── [PLAN] recover exact frozen authored shape from ab11249 / contract-freeze.json
    └── [PLAN] prove via `cargo run -p spec-cli -- test .../apply_tax.unit.spec`

[+] xtask/src/lib.rs :: seed_locked_recommendation_workspace()
    │
    ├── [GAP CLOSED] copy semantic-families/function.wrapper.pipeline.v1
    ├── [PLAN] comment that promoted packet roots are inventory inputs
    └── [PLAN] reuse existing copy helper, no new harness abstraction

[+] xtask command-path recommendation lock
    │
    ├── [PLAN] first run stdout bytes == written artifact bytes
    ├── [PLAN] second run stdout bytes == second written artifact bytes
    ├── [PLAN] first run bytes == second run bytes
    ├── [PLAN] coverage source ids == locked five-source order
    ├── [PLAN] coverage source counts == 6 / 12 / 9 / 1 / 2
    ├── [PLAN] function coverage == 28 / 15 / 0 / 13
    ├── [PLAN] recommendation status == ranked
    ├── [PLAN] first candidate == arithmetic ready
    └── [PLAN] second candidate == money/round hold

[+] Diagnostic fallback
    │
    ├── [PLAN] if promoted-family count still != 15, dump seeded inventory bytes
    ├── [PLAN] diff seeded coverage output against integrated artifact truth
    └── [PLAN] stop before any second speculative fix

INTEGRATED PROOF COVERAGE
=========================
[+] shared-spec build
    └── [PLAN] must pass before crosslib proof

[+] crosslib exact-unit proof
    └── [PLAN] apply_tax exact-unit test must pass

[+] crosslib build + crate tests
    └── [PLAN] shared dep wiring must compile and test clean

[+] artifact validation
    ├── [PLAN] coverage.latest.json validates
    └── [PLAN] recommendation.latest.json validates

─────────────────────────────────
COVERAGE: 11/11 planned paths covered
  Code paths: 8/8
  Integrated proof paths: 3/3
QUALITY: ★★★: 11  ★★: 0  ★: 0
GAPS: 0 remaining if plan is executed literally
─────────────────────────────────
```

### Regression rule

This is a regression repair.

The branch already had a full integrated proof that succeeded, but the final
command-path lock still modeled an older seeded world. Therefore the command-path
test itself is the mandatory regression surface. No debate.

### Test requirements

1. Reuse the exact command-path regression test from `7ae58ae`, not a new test.
2. Extend the seeded workspace inputs so the missing promoted packet root is
   covered by the existing ranked command-path lock.
3. Re-run the exact proof loop from `.runs/m27_8/contract-freeze.json`.
4. If promoted-family count still diverges, dump and diff seeded inventory/coverage
   outputs before any further edits.
5. Treat any remaining divergence between integrated proof and final lock as a stop signal.

### Failure modes

| Codepath | Realistic failure | Test covers it? | Error handling exists? | User-visible or silent? | Critical gap? |
|---|---:|---:|---:|---|---:|
| `apply_tax.unit.spec` recovery | wrong body copied back onto branch | Yes | N/A | visible in exact-unit proof | No |
| `.gitignore` recovery | passport not intentionally tracked | Yes | N/A | visible in rerun artifact diff | No |
| seed helper copy list | promoted packet root still omitted | Yes | No | visible in final xtask test, not silent | No |
| diagnostic fallback | maintainer makes a second speculative fix without capturing seeded truth | Yes | N/A | prevented by explicit stop gate | No |
| ranked lock assertions | counts updated without real proof | Yes | No | visible in xtask failure | No |
| artifact validation | output bytes differ from written files | Yes | No | visible in validation / diff failure | No |

No failure mode in this plan is both silent and untested. Good.

## Performance Review

No meaningful runtime performance risk is introduced.

This work only changes test-harness setup and authored spec recovery. The added
copy operation is one packet directory. The expensive commands were already part
of the blocked proof loop.

The only performance rule here is operational: do not add repeated proof passes
or duplicate cargo invocations beyond the locked command sequence.

## DX Review

DX scope exists because this milestone is about maintainer truth, not end-user UI.

### Developer journey map

| Stage | Maintainer action | Expected outcome |
|---|---|---|
| 1 | read `PLAN.md` | understands this is a harness-truth repair, not new corpus work |
| 2 | inspect `.runs/m27_8/acceptance.md` | sees exact blocked invariant |
| 3 | inspect `.runs/m27_8/merge-log.md` | finds lane-A and lane-B source truth branches/commits |
| 4 | recover lane-A files | branch now contains the already-proven authored source truth |
| 5 | update `xtask/src/lib.rs` | seeded workspace copies `function.wrapper.pipeline.v1` and ranked assertions stay intact |
| 6 | run exact proof loop | integrated truth is reproduced locally |
| 7 | if still red, inspect dumped seeded inventory/coverage diff | sees the next real mismatch instead of guessing |
| 8 | inspect final diff | sees only three tracked source files plus expected derived artifacts |
| 9 | land or stop | either merge cleanly or re-plan from the next mismatch |

### Developer empathy narrative

I am the tired maintainer on Friday night. I do not want a philosophical essay
about test strategy. I want one file telling me which branch had the last known
truth, which three source files matter, which command-path input is actually wrong,
and what exact command to run if the first fix fails. This plan should let me do
that without spelunking old chat history.

### DX scorecard

| Dimension | Score | Note |
|---|---:|---|
| Problem framing | 9/10 | sharply narrowed to harness truth |
| Entry point clarity | 9/10 | blocked artifacts and `ws/*` branches named explicitly |
| Command clarity | 9/10 | exact proof loop preserved |
| Error interpretability | 9/10 | explicit diagnostic fallback now turns future mismatches into concrete diffs |
| Change locality | 10/10 | three tracked source files only |
| Reversibility | 9/10 | stop on first mismatch, no broad redesign |
| Surprise factor | 9/10 | actual failing input path is now named explicitly |
| Time-to-truth | 8/10 | still cargo-heavy, but path is deterministic |

### DX implementation checklist

- [ ] recover lane-A source truth from `ab11249` or `ws/m27_8-lane-a`
- [ ] preserve ranked lock semantics from `7ae58ae` or `ws/m27_8-lane-b`
- [ ] add `semantic-families/function.wrapper.pipeline.v1` to `seed_locked_recommendation_workspace()`
- [ ] if still red, dump seeded inventory + coverage diff before any second fix
- [ ] keep the proof loop order from `.runs/m27_8/contract-freeze.json`

### TTHW assessment

Current TTHW for a maintainer who starts from the current branch is too high,
because the truth is split across a blocked run, missing source files, and stale
test harness assumptions.

Target TTHW after this plan lands: under 10 minutes to understand what happened,
which branch/commit contains the source truth, which seeded input is missing, and
what commands reproduce the decision-grade result.

## Exact File Contract

### Tracked source files

These are the only tracked source files this follow-up should change:

1. `examples/crosslib-app/units/pricing/apply_tax.unit.spec`
2. `examples/crosslib-app/units/.gitignore`
3. `xtask/src/lib.rs`

### Non-touch source surfaces

- `semantic-families/corpus/rust-function.toml`
- `xtask/src/family/coverage.rs`
- `xtask/src/family/recommend.rs`
- `xtask/src/family/promotion_artifacts.rs`
- `xtask/src/family/inventory.rs`
- `docs/recommendation_corpus_expansion_program_v0.1.md`
- `.runs/m27_8/*`

No config or corpus source files should be edited in this follow-up. The harness
repair is about copying the missing promoted packet root, not changing source truth.

### Expected derived artifact churn

- `examples/crosslib-app/units/pricing/apply_tax.spec.passport.json` (new or refreshed)
- `examples/crosslib-app/units/pricing/apply_discount.spec.passport.json`
- `examples/shared-spec/units/money/round.spec.passport.json`
- `.semantic-family-artifacts/family-promotion/analysis/coverage.latest.json`
- `.semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json`

## Implementation Steps

1. Recover `examples/crosslib-app/units/pricing/apply_tax.unit.spec` exactly from
   `ab11249` or `ws/m27_8-lane-a`.
2. Recover the `!pricing/apply_tax.spec.passport.json` whitelist line exactly from
   `ab11249` or `ws/m27_8-lane-a`.
3. Start from the ranked command-path test shape from `7ae58ae` or
   `ws/m27_8-lane-b`.
4. In `seed_locked_recommendation_workspace()`, add this copied input:
   - `semantic-families/function.wrapper.pipeline.v1`
5. Add one short comment above the seed list noting that promoted packet roots are
   part of command-path inventory truth.
6. Keep the ranked assertions bound to this exact locked truth:
   - source ids: `examples_ecommerce`, `m19_semantic_falsification_pack`,
     `m20_unsupported_truth_pack`, `examples_shared_spec`, `examples_crosslib_app`
   - source counts: `6 / 12 / 9 / 1 / 2`
   - function coverage: `28 / 15 / 0 / 13`
   - recommendation status: `ranked`
   - arithmetic cluster first and `ready`
   - `money/round` second and `hold`
7. If the final lock still reports promoted-family count other than `15`, dump and
   diff seeded inventory and coverage outputs before any other change.
8. Run the locked proof loop below in order.

## Locked Proof Loop

Run these commands in order:

```bash
git status --short

cargo run -p spec-cli -- build examples/shared-spec/units --output examples/shared-crate/src/generated
cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- build examples/crosslib-app/units --output examples/crosslib-app/src/generated
cargo test --manifest-path examples/crosslib-app/Cargo.toml

cargo xtask family coverage --format json > /tmp/m27_8r-coverage.stdout.json
cmp -s /tmp/m27_8r-coverage.stdout.json .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json || { diff -u .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json /tmp/m27_8r-coverage.stdout.json || true; exit 1; }
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/coverage.latest.json

cargo xtask family recommend --format json > /tmp/m27_8r-recommend.stdout.json
cmp -s /tmp/m27_8r-recommend.stdout.json .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json || { diff -u .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json /tmp/m27_8r-recommend.stdout.json || true; exit 1; }
cargo xtask family validate-artifact .semantic-family-artifacts/family-promotion/analysis/recommendation.latest.json

cargo test -p xtask -- --color never
```

If the final `xtask` command still fails on promoted-family count, run these before
making any second speculative edit:

```bash
cargo xtask family inventory --format json > /tmp/m27_8r-seeded-inventory.json
cat /tmp/m27_8r-seeded-inventory.json
cat /tmp/m27_8r-coverage.stdout.json
```

Then stop and re-plan from those concrete seeded outputs.

## Worktree Parallelization Strategy

Sequential implementation, no parallelization opportunity worth taking.

Reason:

- this is a three-file repair
- two of the files are exact source-truth recovery from existing `ws/*` commits
- the only new reasoning is in `xtask/src/lib.rs`
- parallel worktrees increase branch-recovery risk more than they buy speed here

Use one working branch with a `git status --short` preflight before the proof loop.

## Test Plan Artifact

Primary QA surface for this milestone is command and artifact truth, not pages.

- `cargo run -p spec-cli -- test examples/crosslib-app/units/pricing/apply_tax.unit.spec`
  — prove the recovered cross-library unit is valid and executable
- `cargo xtask family coverage --format json`
  — verify the coverage artifact reproduces the blocked integrated truth `28 / 15 / 0 / 13`
- `cargo xtask family recommend --format json`
  — verify the recommendation artifact stays `ranked` with arithmetic first and `money/round` held second
- `cargo test -p xtask -- --color never`
  — verify the final copied-workspace command-path lock reproduces the same truth as the integrated proof loop

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | Scope | Replace old corpus-expansion plan with harness-truth repair plan | Mechanical | Explicit over clever | The blocked run proved product truth already; pretending this is still corpus exploration is false | Keeping the old plan and patching around it |
| 2 | Scope | Treat `ws/m27_8-*` branches and blocked artifacts as source truth | Mechanical | Pragmatic | The current branch is missing the proven files; the `ws/*` branches hold the real authored result | Re-authoring from memory |
| 3 | Architecture | Reuse the existing seed helper instead of inventing a fixture framework | Mechanical | Boring by default | One explicit seam already exists in `xtask/src/lib.rs` | New fixture registry abstraction |
| 4 | Architecture | Copy `semantic-families/function.wrapper.pipeline.v1` because inventory consumes promoted packet roots | Mechanical | Explicit over clever | The failing codepath reads promoted packet presence, and the missing wrapper packet explains the exact 5-unit delta | Speculative config-root patching |
| 5 | Tests | Add a diagnostic stop gate if promoted-family count is still wrong after the packet-root fix | Mechanical | Choose completeness | The next mismatch should produce concrete seeded truth, not another guess | Generic "re-plan" without captured evidence |
| 6 | Execution | Keep implementation serial on one branch | Mechanical | Pragmatic | This is a three-file repair with existing `ws/*` source truth; parallel worktrees add more operational risk than value | Multi-worktree recovery replay |

## Completion Summary

- Step 0: Scope Challenge — scope reduced from corpus-expansion follow-up to harness-truth repair
- Architecture Review: 3 issues found
- Code Quality Review: 2 issues found
- Test Review: diagram produced, 0 remaining gaps if executed literally
- Performance Review: 0 issues found
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 0 items proposed
- Failure modes: 0 critical gaps flagged
- Outside voice: unavailable, auth missing
- Parallelization: sequential, 0 useful parallel lanes
- Lake Score: 6/6 recommendations chose the complete option

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | advisory only | Strategy pass agreed with the reframing but required a stronger diagnostic gate, alternatives section, and more durable truth notes |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR | Engineering pass rejected the original `spec.toml` theory and pointed the plan at the inventory-backed promoted packet input that the failing path actually reads |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped | No UI scope |

**UNRESOLVED:** 0

**VERDICT:** ENG CLEARED — this file is now a fresh harness-truth implementation
contract with the causality model corrected to match the actual command-path code.
