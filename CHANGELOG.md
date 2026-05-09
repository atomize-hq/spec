# Changelog

## Unreleased

### Added

- **Helper-surface runtime support for the live `money/round` wedge** — `spec-core` now routes the current zero-dep, one-Decimal-input helper shape through `function.helper.identity_passthrough.v1` instead of dropping it to generic unsupported-function truth.

### Changed

- **Fresh helper proof now stays truthful on read-side surfaces** — the checked-in `examples/shared-spec` passport and `spec status` / `spec export` now preserve supported helper semantic-review truth for `money/round` instead of surfacing `unsupported.function.v1`.
- **Family-analysis operator truth now treats the helper wedge as supported-unpromoted substrate** — `cargo xtask family inventory --format json` publishes `function.helper.identity_passthrough.v1` as runtime-supported, coverage moves the three helper units into `supported_unpromoted_family_units`, and recommendation output no longer surfaces `helper_surface_not_promotable` as live unsupported pressure.

## 0.14.0 - 2026-05-07

### Added

- **Bounded second-language proof surfaces for promoted families** — monotone-up and wrapper-pipeline now ship committed packet truth, starter fixtures, and Rust plus TypeScript prove/certify coverage without claiming full second-language backend support.
- **Family-analysis decision kernel and verifier contracts** — `xtask` now derives bounded corpus-program decisions, verifies basis and decision parity, freezes helper-surface durable-hold tuples, and ships `family verify-decision-contract` as a standing truth wall.
- **Repo-local next-milestone operator tooling** — the repo now includes a `next-milestone` skill, live signal collector, and review rubric for choosing the next honest milestone from checkpoints, authority plans, and current family-analysis outputs.

### Changed

- **Read-side semantic review now routes through explicit backend-execution and portability layers** — passports, status/export projection, escape-hatch gates, and semantic review handling share bounded backend and portability surfaces instead of re-deriving lowering assumptions in each consumer.
- **Recommendation artifacts are now maintainer-facing and decision-explicit** — analysis outputs now carry durable `recommended`, `blocked_for_now`, and `not_recommended` verdicts, blocker evidence, normalized proof fingerprints, and follow-on action contracts for corpus-program decisions.
- **CI now gates promoted-family proof walls directly** — the main workflow adds monotone-up and wrapper-pipeline pilot jobs, and the repo docs/examples now teach the prove/certify loop as part of the maintained product surface.
- **Planning docs now freeze the shared-core follow-on boundary explicitly** — the current authority plan, orchestration plan, and semantic-family guides distinguish core semantic-review product work from family-analysis servant work and pin the exact trigger table for any later shared-core extraction.

### Fixed

- **Canonical example truth stays aligned across promoted-family and portability changes** — checked-in passports, regression fixtures, semantic-family packets, and cross-library examples now reflect the landed monotone-up, wrapper-pipeline, and preserve-mode portability contracts.
- **Helper-surface follow-on helpers no longer leave release CI red under `clippy -D warnings`** — dead runtime predicates were trimmed to the test-only surface so the workspace passes the same warning gate the release workflow enforces.

## 0.13.1 - 2026-04-29

### Added

- **Workspace-owned semantic-family promotion commands** — the repo now ships `xtask` as a first-class workspace member with locked `family new`, `smoke`, `prove`, and `certify` flows for promoted packets.
- **Two real promoted family packets beyond the original wrapper seed** — `function.wrapper.pipeline.chain3.v1` and `function.arithmetic_leaf.monotone_up.v1` now ship committed packet truth, scaffold contracts, fixture corpora, and attested proof artifacts.
- **Targeted promotion regression coverage** — xtask lock tests, runtime classifier/routing regressions, CLI truth-surface regressions, and monotone-up corpus fixtures now exercise the promotion workflow end to end.

### Changed

- **Unsupported-function proof now survives preserve-mode only when it is still fresh** — passport and export projection keep truthful `unsupported.function.v1` review plus rewrite hints on read-side surfaces until authored truth changes, then drop the stale proof instead of minting new unsupported truth.
- **Semantic-family artifact schema is now v3** — `prove.latest.json`, certify attempts, and `certification.report.json` now write `schema_version = 3`, and prove `overall_status` reflects only that artifact's `required_gates` instead of implicitly failing on `gate_d`.
- **xtask routing diagnostics now distinguish packet-local and registry-global truth** — selected-family manifest routing still requires exact equality with its locked harness entry, while registry-global coherence checks only registered families plus terminal `unsupported.function.v1`.
- **Repo truth and agent docs now teach the promoted-family workflow directly** — `README.md`, `AGENTS.md`, `semantic-families/README.md`, and the M24/M26 planning docs now point at the real smoke/prove/certify loop and the current supported-family inventory.

### Fixed

- **`prove.latest.json` no longer overstates failure on successful prove runs** — a passing prove artifact can now honestly serialize `overall_status = "pass"` even when `gate_d` remains informationally `fail`.
- **Canonical ecommerce proof surfaces are fresh again for the promoted families** — checked-in passports, unsupported-truth fixtures, and read-side status fixtures now match the current monotone-down, monotone-up, and wrapper-family contract.

## 0.13.0 - 2026-04-26

### Added

- **M19 semantic-review falsification pack** — `spec` now proves supported function review against unseen Family A and Family B examples instead of relying on canonical pricing names.
- **Family B argument-flow proof** — wrapper pipeline review now rejects swapped, duplicated, dropped, and mis-threaded argument paths instead of false-greening shape-compatible wrappers.
- **Function semantic freshness coverage** — supported function proof now goes stale when intent, deps, body Rust, or routing-relevant authored contract cues change.

### Changed

- **Unsupported function near misses stay read-side neutral** — `spec test` may record additive unsupported metadata, while `spec build`, `spec status`, and `spec export` keep official health surfaces neutral for unsupported cases.
- **Semantic review projection now uses family compatibility keys** — preserve-mode keeps only matching supported-family truth and drops stale exact-id or unsupported review metadata on read-side surfaces.
- **The M19 plan and docs now state the supported-vs-unsupported function contract directly** — maintainers can see what the evaluator proves today and what remains intentionally out of scope.

### Fixed

- **Preserve-mode proof no longer survives semantic edits as current truth** — status and export reproject freshness before surfacing stored semantic review.
- **Canonical ecommerce semantic-review artifacts are fresh again** — checked-in passports and molecule evidence now match the current Family A and Family B proof model.

## 0.12.0 - 2026-04-22

### Added

- **M14 proof freshness and truth surfaces** — passports, status, and export now distinguish authored-truth freshness from backend-execution freshness instead of collapsing both into one opaque stale signal.
- **Plan acceptance closure in machine-readable output** — `spec plan validate --format json` and plan export now surface `acceptance_closure`, so authored acceptance lists are checked against computed impact instead of trusted at face value.
- **Canonical M14 regression coverage** — the CLI test suite now includes focused regressions for stale-proof projection, escape-hatch gate behavior, legacy passport freshness fallback, backend-only drift, and plan-closure honesty.

### Changed

- **`spec build` and `spec generate` now preserve prior proof state** — non-test passport writes keep existing evidence and freshness anchors on disk, then reproject current freshness/marker state instead of erasing prior `spec test` proof.
- **Escape-hatch proof gates are now live truth surfaces** — marked seam units project backend markers, required proof surfaces, and gate state consistently through stored passports, `spec status`, and `spec export`.
- **The canonical `pricing/discount_policy` seam now proves real branch breadth** — the example wedge carries direct atom proof for `none`, `percentage`, `fixed_amount`, and capped fixed-amount behavior, plus molecule proof that closes the escape-hatch gate.
- **Agent and repo docs now teach the M14 trust loop** — `README.md` and `AGENTS.md` describe freshness as authored/backend truth, not just legacy contract-hash drift, and point at the current milestone framing.

### Fixed

- **Legacy passports now project freshness honestly** — units written before M14 still resolve authored drift correctly even when they only carry the old top-level contract hash and no freshness block.
- **Export no longer overclaims molecule proof coverage** — canonical seam proof coverage only includes the `molecule` surface when current molecule evidence is both present and passing.
- **Backend-only drift now reopens proof gates without inventing authored drift** — status and export agree when lowering changes but shared seam meaning does not, which prevents fake-green review surfaces.

## 0.11.0 - 2026-04-21

### Added

- **M13 sum seams** — `spec` can now author one top-level `kind: sum` seam with explicit ordered variants, named payload fields, seam-owned methods, and seam-owned local tests, then lower it into generated Rust as one `enum + impl`.
- **Canonical ecommerce M13 wedge** — `examples/ecommerce` now ships the `pricing/discount_policy` migration wedge as both a hand-written Rust baseline and an authored `kind: sum` seam, plus a mixed-kind molecule proof that composes the new seam with the existing `pricing/checkout_quote` data seam and `pricing/apply_tax` function unit.

### Changed

- **Mixed-kind trust surfaces now stay aligned through one shared dep/import projection seam** — graph edges, exact-unit single-file `spec test`, molecule imports, passport deps, and export deps all project `function`, `data`, and `sum` units through the same top-level dependency story instead of re-deriving kind-specific behavior in multiple places.
- **Validation, normalization, and code generation now treat `kind: sum` as a first-class authored shape** — schema rules, semantic validation, normalization, lowering, and generated Rust all preserve shared seam truth while keeping Rust-specific details inside method lowering and backend derives.
- **Passport, export, and status now expose truthful additive `sum` metadata** — sum seams participate in contract-hash staleness, passport/export authored-shape projection, mixed-tree status reporting, and checked-in example artifacts without widening nested variants into separate tracked nodes.
- **The docs and plan now teach the exact M13 loop** — `PLAN.md`, `README.md`, `AGENTS.md`, and `examples/ecommerce/README.md` all point at the `discount_policy` validate/build/test/status flow, record the adversarial wedge score table, and explain the `kind: sum` seam boundary explicitly.

### Fixed

- **Projected Rust identifier failures now stop at validation time** — sum variants that normalize into invalid or colliding Rust names are rejected before code generation instead of leaking through as later compiler failures.
- **Molecule imports for mixed-kind seams are now warning-clean and explicit** — the new M13 molecule coverage uses explicit imports and the shared projection helper, so the ecommerce example keeps compiling cleanly under strict consumers while preserving truthful coverage metadata.
- **Canonical ecommerce proof artifacts are fresh again** — the checked-in passports and molecule evidence now match the landed M13 contract hashes, provenance, and mixed-kind example flow instead of drifting behind the branch.

## 0.10.0 - 2026-04-20

### Added

- **M12 data seams** — `spec` can now author one top-level `kind: data` seam with explicit shared fields, declarative constructors, inherent methods, and seam-owned local tests, then lower it into generated Rust as one `struct + impl`.
- **Canonical ecommerce migration wedge** — `examples/ecommerce` now includes a real `kind: data` seam at `units/pricing/checkout_quote.unit.spec` plus a hand-written Rust baseline at `src/raw_baseline/pricing/checkout_quote.rs` for side-by-side comparison.

### Changed

- **The full CLI pipeline now treats a data seam as one truthful top-level unit** — kind-aware normalization, graph loading, generation, passport hashing, export, and status all preserve seam-level truth without promoting nested constructors or methods into separate tracked nodes.
- **Validation and generation now understand data-seam semantics end to end** — method deps flow through graph/export/passport surfaces, Rust lowering emits seam derives and inherent methods, and identical cross-method dep reuse is allowed when it is semantically the same callable.
- **Checkout molecule coverage now includes the migrated seam** — `pricing/checkout_flow.test.spec` covers `pricing/checkout_quote` and asserts the data seam agrees with the existing function-based checkout flow.
- **Docs now show the exact M12 command loop** — `README.md`, `AGENTS.md`, `PLAN.md`, and `examples/ecommerce/README.md` document the repo-root `validate`, `build`, `test`, and `status` commands for the checkout quote migration wedge.

### Fixed

- **Data-seam validator failures are now caught earlier and more precisely** — the CLI rejects invalid method contracts, callable collisions, unsupported scoped generation, bad body shapes, and shared-semantic escape hatches with stable machine-readable diagnostics instead of leaking later generator/compiler failures.
- **Workspace-scoped generation and plan scanning are less brittle** — scoped generation cleanup and library path rebasing stay rooted correctly, and hidden scratch units no longer pollute plan scans.
- **Shipped ecommerce truth artifacts stay fresh** — the checked-in passports and molecule evidence now match the landed M12 contract hashes and provenance instead of drifting behind the branch.

## 0.9.0 - 2026-04-17

### Added

- **M11 molecule evidence artifacts** — Molecule tests now persist observed results in co-located `*.test.evidence.json` files, separate from unit passports.
- **Single-file molecule execution** — `spec test path/to/file.test.spec` now runs exactly one molecule test and writes only that target evidence artifact.
- **Checked-in ecommerce plan example** — `examples/ecommerce/plans/refactors/checkout-tax-refactor.plan.spec` is now the canonical in-repo plan example path.

### Changed

- **`spec status --format json` now emits `schema_version: 3`** — Status is grouped by discovered library root, keeps units and molecule tests in separate planes, preserves a flattened top-level `units` compatibility view, and treats zero discovered roots as a non-green result.
- **Rust toolchain is now pinned in-repo** — Added `rust-toolchain.toml` pinned to Rust `1.89.0` to match the crate metadata and CI configuration.

### Fixed

- **Repo-root status discovery now stays library-bounded** — `spec status` can discover multiple library roots under a parent/repo path without collapsing duplicate local ids across roots.
- **Molecule status no longer contaminates unit health** — failing, stale, or missing molecule evidence affects only the molecule-test plane, not the covered units' passport status.
- **Canonical ecommerce example now ships molecule evidence** — the checked-in `pricing/*.test.evidence.json` artifacts keep `spec status .` truthful on a fresh clone instead of leaving the shipped M11 example non-green by default.

## 0.8.0 - 2026-04-17

### Added

- **M10 plan artifacts** — `spec plan validate <file>` and `spec plan export <file>` now let you author a single `.plan.spec` change-intent file, validate it, and export a dedicated machine-readable plan bundle without mutating the existing `spec export` contract.

### Changed

- **Plan impact is now derived, local-library, and truthful** — M10 plan files author intended changes plus structured acceptance targets, while `computed_impact` is derived from the current local graph. `modify` and `remove` reuse current graph impact, `add` stays honest with unresolved output, and cross-library plan refs remain out of scope.
- **Plan commands reuse the repo trust boundary** — plan root resolution now anchors to the enclosing library root instead of the plan file directory, keeps scans repo-bounded, and emits stable JSON diagnostics for the new plan failure modes.
- **Machine-readable docs now cover the plan contract** — README and AGENTS document the single-file plan workflow, the local-library-only rules, the dedicated export shape, and the new `SPEC_PLAN_*` error codes.

### Fixed

- **Plan validation now rejects duplicate and impossible authored changes cleanly** — missing `modify/remove` targets, duplicate `changes[].unit` entries, symlink escapes, and invalid local molecule-test loading all fail with explicit contract errors instead of leaking ambiguous loader behavior.
- **M10 regression coverage is now complete at the CLI boundary** — end-to-end tests cover nested plan root resolution, mixed add/modify impact, cross-library rejection, missing modify targets, duplicate change units, symlink escapes, and dedicated plan export stability.

## 0.7.0 - 2026-04-16

### Added

- **M9 direct cross-library deps** — Root specs can now declare direct sibling-library deps like `shared::money/round` via `[libraries]` in `spec.toml`. The CLI resolves only the root library's direct aliases, keeps root config authoritative, and validates the full direct dep shape before generation or export.
- **In-repo shared-library proof example** — Added `examples/shared-spec`, `examples/shared-crate`, and `examples/crosslib-app` to show the end-to-end sibling-library flow: generate shared units into a Rust crate, generate the consuming app, then `cargo check` and `cargo test` the result.

### Changed

- **Cross-library dep identity is now typed end-to-end** — Validator, generator, graph, and export all consume the same parsed dep identity instead of stringly local IDs. Local deps stay backward compatible, duplicate unit IDs remain library-local, and imported libraries do not recursively widen the graph.
- **Generated imports now honor library aliases** — External deps emit `use <alias>::...` imports, and the CLI rejects missing Cargo dependency aliases or callable-name collisions instead of generating ambiguous Rust.
- **`spec export` now emits `schema_version: 3`** — Dep edges and unit deps use structured `{ library, id }` refs so mixed local and external deps are no longer ambiguous in exported bundles.
- **Status/validate handling for library loader failures is now truthful** — `spec validate --format json` and `spec status --format json` keep `[libraries]` resolution failures machine-readable, and `spec status` routes imported-library loader failures to top-level `loader_errors` instead of misreporting root units.

### Fixed

- **Cross-library cycle and cover failures now fail with stable contract errors** — Direct A→B→A library cycles raise `SPEC_CROSS_LIBRARY_CYCLE`, cross-library `.test.spec` covers are rejected explicitly, and transitive library aliases stay unresolved instead of silently loading extra graphs.
- **Example ecommerce passports now record observed `pass` evidence** — Re-ran `spec test examples/ecommerce/units` so the checked-in regression passports match the current M9 test-name and provenance behavior instead of leaving local tests at `unknown`.

## 0.6.0 - 2026-04-15

### Added

- **M8 declared graph API in `spec-core`** — `SpecGraph` is now a public declared-relationship query surface, re-exported from `spec-core` alongside `SpecEdge`, `UnitNode`, `MoleculeTestNode`, and `ImpactSet`. Query methods `units()`, `molecule_tests()`, `edges()`, `reverse_deps()`, `tests_covering()`, and `impact()` expose local-library declared graph relationships.
- **`ImpactSet` return type for impact analysis** — `impact(unit_id)` returns the local declared retest closure as `{ units, molecule_tests }`. `reverse_deps()` returns direct dependents, `tests_covering()` returns directly covering molecule tests, and all three query methods return `None` for unknown unit IDs. This is advisory planning data only, not observed runtime status, cross-library graph identity, or `spec status` propagation.

### Changed

- **Export now consumes the graph through accessor methods** — `spec export` continues to emit sorted declared dep and covers edges, but now projects them from the public `SpecGraph` surface instead of reaching into graph internals. This locks in the M8 contract that export is a projection layer, not the owner of graph state.

## 0.5.3 - 2026-04-13

### Added

- **Molecule tests (`.test.spec`)** — First-class support for multi-unit integration tests. Author `.test.spec` files alongside your unit specs to declare test interactions. `spec validate`, `spec generate`, `spec build`, `spec test`, and `spec export` all handle them. Each molecule test declares which units it `covers` and provides a full Rust block body; generated `#[test]` functions are placed in `molecule_tests.rs` per namespace.
- **`SpecGraph` in spec-core** — Minimal typed graph (`UnitNode`, `MoleculeTestNode`, `SpecEdge`) built from loaded specs and molecule tests. Foundation for M8 full graph layer.

### Breaking

- **`spec export` schema_version bumped to 2** — `ExportEdge` changed from a flat struct `{from, to}` to a tagged enum. Dep edges now serialize as `{"kind":"dep","from":"…","to":"…"}` and covers edges as `{"kind":"covers","test":"…","unit":"…"}`. The `molecule_tests` array is also added to the bundle. Consumers reading schema_version 1 bundles must update to handle the `kind` field.

### Fixed

- **Single-file CLI scope no longer breaks on sibling molecule tests** — `spec validate <file.unit.spec>`, `spec generate <file.unit.spec>`, and `spec export <file.unit.spec>` now stay scoped to the requested unit. Sibling `.test.spec` files are only loaded for directory invocations, preserving the exact-unit authoring loop used by agents and README workflows.

### Breaking (continued)

- **`spec status` non-valid units now exit `1`** — `untested`, `incomplete`, and `failing` units now produce a non-zero exit code alongside `invalid` and `stale`. Consumers that previously treated those states as soft-success need to update.
- **`stale: bool` removed from `spec status --format json` units** — Machine consumers should switch to the `status` and `reason` fields instead of reading a separate stale flag.
- **`spec status --format json` now emits `schema_version: 2`** — Parsers should gate on version `2` for the new health-state contract.

Migration: for `spec status`, treat any unit whose `status` is not `"valid"` as a failing health result, and replace any `stale: bool` logic with `status`/`reason` handling.

## 0.5.2 - 2026-04-12

### Changed

- **Default output path is now `{crate_root}/src/generated`** — `spec generate`, `spec build`, and `spec test` no longer require `--output`. When omitted, the output directory is derived from the crate root (via `spec.toml` or ancestor `Cargo.toml` walk) and defaults to `src/generated` inside that crate. Projects using the old default `generated/spec` can pass `--output generated/spec` explicitly to preserve prior behavior.
- **`generated_module_prefix` config key** — Add `[pipeline] generated_module_prefix = "custom::prefix"` to `spec.toml` for non-standard output layouts where auto-derivation produces a wrong module path.
- **`Verbosity` enum in pipeline API** — `run_cargo_build` and `run_cargo_test` now accept a `Verbosity` parameter. `Verbosity::Normal` preserves existing `spec: running cargo …` stderr output; `Verbosity::Silent` suppresses it (reserved for future `--format json` mode).

### Fixed

- **Module prefix evidence mismatch** — `spec test` was computing the generated module prefix twice: once for the cargo filter and once for evidence lookup, using different values. All tests showed `status: "unknown"` when the two derivations disagreed. Fixed by computing the effective prefix once and passing it to both sites.

## 0.5.1 - 2026-04-11

### Added

- **Pipeline timeout** — Configure `[pipeline] timeout_secs` in `spec.toml` to bound `spec build` and `spec test` execution time. Hung cargo processes are killed after the deadline; passports record `build_status: "timeout"` so downstream agents see a clean signal instead of a stale run.
- **Git provenance in passports and exports** — `spec test` now records the current `git_commit_sha` in passport evidence when run inside a git repository. `spec export` includes top-level provenance in the bundle. Passports from pre-provenance runs deserialize cleanly (field is optional, absent passports remain valid).
- **Concurrent write warning** — When multiple `spec` processes write passports at the same time (a risk in multi-agent CI), a warning is emitted to stderr. The guard is advisory (warn-only, no blocking lock), matching the M5 trust-not-lock design.

### Changed

- **Stable `SPEC_*` JSON error codes** — `spec validate --format json` and `spec status --format json` now emit stable, namespaced error codes (`SPEC_MISSING_DEP`, `SPEC_INVALID_CONTRACT_TYPE`, etc.) instead of bare CamelCase names. Machine consumers can write stable matchers against these codes. `AGENTS.md` updated to reflect the new contract.
- **`schema_version` is now a JSON integer** — Export bundles and JSON status/validate responses emit `"schema_version": 1` (integer) instead of `"schema_version": "1.0"` (string). Consumers that compare against the string `"1.0"` need to update to compare against the integer `1`.
- **Faster cargo test result lookups** — `parse_cargo_test_output` now returns a `HashMap` instead of `BTreeMap`, reducing evidence-building overhead for large test suites.

### Fixed

- **Timeout process tree** — After killing cargo on timeout, pipe reader threads are no longer joined. Grandchildren (rustc, test binaries) that inherit pipe write-ends no longer cause `spec` to hang past the configured timeout.

## 0.5.0 - 2026-04-06

### Added

- **`spec status [path]`** — New command showing per-unit validation, passport, and staleness status in both human-readable and `--format json` modes. AI agents read this to know what to work on.
- **`--format json` on `validate` and `status`** — Structured JSON output on stdout with `schema_version`, `status`, `errors[]`, and `warnings[]` fields. Each error carries a `path` field pointing directly to the source `.unit.spec` file. Loader errors surface in `status --format json` via a `loader_errors` field so AI agents receive a single parseable signal even when files are malformed.
- **`spec test [path]` single-unit scoping** — Pass a `.unit.spec` file path to `spec test` to run only that unit's cargo tests using a derived module path filter (`pricing::apply_tax::tests::`). New passport evidence is written per-run.
- **Contract hash in passports** — `spec test` now writes a `contract_hash` (SHA-256 of the serialized contract) to the passport. `spec status` compares the live contract hash against the stored hash to detect stale units — those where the contract changed but tests haven't been re-run.
- **AGENTS.md spec workflow** — Real agent workflow guide added to AGENTS.md: a 5-step validate → edit → build → test → check loop for AI coding agents working with spec units.
- **Companion gstack skill** — `.claude/skills/spec/SKILL.md` teaches any Claude Code session the spec workflow, common validation errors, and how to interpret passport evidence.
- **ICP definition** — Who spec v0.5 is for: a solo engineer or 2-5 person team using AI coding assistants daily where contract clarity and correctness matter. Written in DECISIONS.md.
- **Golden JSON fixture tests** — `spec-cli/tests/fixtures/` contains reference JSON outputs for `spec validate --format json` and `spec status --format json`. Shape breakage = test failure.

### Fixed

- **Zero-tests detection** — `spec test` now correctly detects when a filter matches 0 tests in a multi-binary crate (checks all binaries, returns true only when none ran matching tests). Previously would silently write evidence with empty test results.
- **JSON status loader errors** — `spec status --format json` no longer emits text diagnostics to stdout when loader errors occur; errors now appear in the JSON response's `loader_errors` field.
- **`status_command` zero-unit edge case** — `spec status` no longer incorrectly prints "0 units found" when loader errors are present.
- **JSON error field completeness** — Several `SpecError` variants (`RustKeyword`, `DepCollision`, `BodyRustMustBeBlock`, `LocalTestExpectNotExpr`) now correctly populate all JSON error fields instead of emitting `null` for known values.

### Migration

- **No authored unit format change** — `.unit.spec` authors should continue using `spec_version: "0.3.0"`.
- **Passport schema v3** — Passports may now include an optional `contract_hash` field (SHA-256, prefixed `sha256:`). Parsers should tolerate its absence; missing hash means "no stale detection available for this unit."

## 0.4.0 - 2026-04-05

### Added

- **Pipeline commands** — `spec build` now runs validate → generate → `cargo build`, and `spec test` runs the same pipeline followed by `cargo test`.
- **JSON export** — `spec export` emits a machine-readable bundle with units, passports, graph edges, and export warnings.

### Changed

- **Generated Rust doc comments** — `spec generate` now emits `///` doc comments from each unit's `intent.why` field above the generated function.
- **Passport runtime evidence** — `spec test` now records observed build/test results under an optional `evidence` field in co-located passports.

### Breaking

- **Passport schema v2** — Passport JSON may now include an optional `evidence` field containing locally observed runtime results.

### Migration

- **Passport evidence is additive** — No file migration is required. Parsers should tolerate absent `evidence` and treat it as "no runtime evidence available".
- **Authored unit format version remains `0.3.0`** — The crate release is `0.4.0`, but `.unit.spec` authors should continue using `spec_version: "0.3.0"` because the unit-file wire format did not change in this release.

## 0.3.0 - 2026-04-04

### Added

- **Passport generation** — `spec generate` now emits a `.spec.passport.json` file co-located with each `.unit.spec` source file. Passports are static knowledge artifacts containing the unit's id, intent, contract, deps, local tests, and generation timestamp. They are written atomically only after all Rust code generation succeeds, and gitignored automatically via an appended `**/*.spec.passport.json` entry.
- **`spec_version` field** — Units can now declare `spec_version: "0.3.0"` to indicate which format version they were authored for. `spec validate` and `spec generate` emit a `MissingSpecVersion` warning for units without this field, guiding authors to add it.
- **Cycle detection** — `spec validate` and `spec generate` now detect circular dependencies in the dep graph using DFS. A cycle like `A → B → A` is reported as `❌ cycle detected: A → B → A` and blocks generation.
- **Contract type validation** — `contract.inputs` values and `contract.returns` are now validated as syntactically valid Rust types using `syn`. Invalid types (e.g., `Vec<`) are caught at `spec validate` time. Parameter names (keys) are validated as valid Rust identifiers, catching reserved keywords like `type` or hyphenated names like `bad-name` before they reach codegen.
- **CUE trigger conditions** — DECISIONS.md now documents the explicit conditions under which CUE adoption is warranted, preventing indefinite deferral.

### Changed

- **`body.rust` is now a block expression** — The function body is now specified as a Rust block expression (`{ ... }`, braces included) rather than a complete function declaration. `spec generate` synthesizes the `pub fn` signature from `contract.inputs` and `contract.returns`. This eliminates fn name drift and makes contracts the authoritative source of the function's interface.
- **`contract.inputs` uses ordered map** — Input parameters now preserve YAML declaration order in generated code, using `IndexMap` instead of `HashMap`.
- **`spec generate <file.unit.spec>`** — Single-file generate now correctly writes `.gitignore` to the spec file's parent directory instead of failing with a path error.

### Breaking

- **`body.rust` format** — Units authored for 0.2.x with a full `pub fn` declaration will fail `spec validate` with a migration error. Strip the `pub fn name(params) -> ReturnType` line, keep only the `{ ... }` block, and move parameters into `contract.inputs` and return type into `contract.returns`. See the migration guide in README.md.

## 0.2.2 - 2026-04-03

### Added

- **`--no-strict` flag for `spec validate`** — Downgrades missing-dep errors to warnings and exits 0. Useful for partial-graph workflows where not all deps are present in the local spec set. `spec generate` explicitly rejects `--no-strict` with a helpful error.
- **`spec.toml` workspace config** — Supports `[validation] allow_unsafe_local_test_expect = true` to permit block, unsafe, closure, and other complex Rust expressions in `local_tests[].expect` for trusted environments. Config is discovered by walking ancestors from the target path (same convention as `.gitignore`).
- **`SpecWarning` type** — New non-fatal diagnostic type. Currently emitted for: symlink cycles skipped during directory traversal (`SymlinkCycleSkipped`) and missing deps in non-strict mode (`MissingDep`). Warnings print to stderr and appear in the success message count.

### Fixed

- **Symlink cycle handling** — Directory traversal no longer errors on symlink cycles. Cycles emit a `SymlinkCycleSkipped` warning and traversal continues with the rest of the tree. Previously, a cycle caused `spec validate` and `spec generate` to hard-fail.
- **`safe_output_path` consolidation** — `clean_output_dir` and `ensure_output_marker` previously used divergent path-containment logic (`normalized_absolute_path` lexical vs `canonicalize` symlink-following). Both now use a single `safe_output_path` utility that canonicalizes existing ancestors and rejects paths outside the project root.
- **`local_tests[].id` uniqueness** — Duplicate IDs within a single unit's `local_tests` are now caught at validation time. Previously, duplicate IDs would silently generate duplicate `fn test_{id}()` functions and cause a Rust compile error downstream.
- **`BodyRustSingleItemNotFn` error** — When `body.rust` contains exactly one top-level item that is not a function, the error now says "found 1 item (not a function)" instead of the misleading "found 0 items".

### Internal

- `load_directory_report` promoted from `pub(crate)` test helper to public API. Returns `DirectoryLoadReport` with `specs`, `errors`, `warnings`, and `total_files`.
- `validate_full`, `validate_semantic`, and `validate_deps_exist` each now have `_with_options` variants accepting `ValidationOptions`. The originals are kept as strict-mode convenience wrappers.

## 0.2.1 - 2026-04-03

### Security

- **`is_safe_expect_expr` now recurses into sub-expressions** — Previously, the expression whitelist in `spec validate` only inspected the top-level AST node. A call like `f({ unsafe { ... } })` would pass because the outer `Call` was whitelisted without checking its arguments. All Arms (Binary, Call, MethodCall, Field, Index, Unary, Paren, Cast) now recurse into every sub-expression; `unsafe`, block, closure, and control-flow forms are rejected wherever they appear in the tree. Error message updated from "simple expression" framing to "block, unsafe, closure" framing to accurately describe what is and isn't blocked.

### Testing

- Added 4 regression tests covering recursion through Field, Index, Unary, and Cast arms.

## 0.2.0 - 2026-04-02

### Added
- **`imports` field** — specs can now declare external `use` statements directly (e.g., `imports: [rust_decimal::Decimal]`), which are emitted as `use` items in the generated file. Previously, all external types had to be brought into scope manually via the crate using the generated code.
- **`local_tests` codegen** — specs can embed inline test cases with an `expect` expression; `spec generate` now produces a `#[cfg(test)]` block with one `#[test]` function per entry. Block, unsafe, closure, and control-flow expressions are rejected to keep expect values safe to embed.
- **Body validation** — `body.rust` is now parsed with `syn` at validation time. `spec validate` reports errors for: wrong function name, multiple top-level items, `&self` receivers, and contract input names that don't match function parameters.
- **Ecommerce example** — `examples/ecommerce/` ships a working multi-unit spec set (money/round, pricing/apply_discount, pricing/apply_tax, pricing/calculate_total) with `cargo check` and `cargo test` verified in CI.
- **Cargo check integration test** — the CLI test suite runs `cargo check` and `cargo test` against the ecommerce example on every `cargo test --workspace`, ensuring generated Rust stays valid end-to-end.

### Changed
- **Output safety hardened** — `spec generate --output <dir>` now validates the output path before writing: rejects paths outside the project root (including symlink traversal), rejects non-empty directories missing the `.spec-generated` marker, and cleans orphaned `.rs` files from prior runs. Previously the output directory was trusted as-is.
- **Duplicate ID reporting** — `spec validate` now reports all files with duplicate IDs in a single pass instead of stopping at the first pair.

### Fixed
- `spec validate` and `spec generate` no longer silently pass specs with unresolved internal deps.

### Breaking
- **Strict dep validation** — `validate` and `generate` now exit 1 for specs with unresolved internal deps. Previously these passed silently. Ensure all deps are defined in the same spec set before upgrading. A `--no-strict` flag for partial-graph workflows is planned for M3.
