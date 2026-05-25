# spec

`spec` is a Rust-first semantic-source workflow for code that you want to be
explicit, machine-checkable, and provable.

You author units in `*.unit.spec` files, run a tight validate/build/test loop,
generate readable Rust, and keep proof artifacts that say what actually passed.
The project is built for AI-assisted and human-assisted development where "it
probably works" is not a good enough answer.

## What spec is

At a high level, `spec` gives you:

- semantic source files instead of raw implementation as the only source of truth
- generated Rust from explicit contracts
- atom and molecule proof surfaces with tracked evidence
- machine-readable `validate`, `status`, and `export` outputs for agents and tools
- a bounded, honest product claim surface instead of fuzzy support language

This repo is the implementation of that workflow, plus the example libraries and
proof walls used to keep the claims honest.

## What you need

- Rust `1.89.0`
- `cargo`
- this repo checked out locally

The pinned toolchain lives in [`rust-toolchain.toml`](rust-toolchain.toml).

If you want a local binary instead of `cargo run`, install from the repo root:

```bash
cargo install --path spec-cli
```

## The core loop

If you are new here, do this first.

From the repo root:

```bash
cargo run -p spec-cli -- validate examples/ecommerce/units/pricing/apply_tax.unit.spec --format json
cargo run -p spec-cli -- build examples/ecommerce/units --output examples/ecommerce/src/generated
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/apply_tax.unit.spec
cargo run -p spec-cli -- test examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec
cargo run -p spec-cli -- status examples/ecommerce/units --format json
```

What each step tells you:

1. `validate` answers: is the authored source legal `spec` input?
2. `build` answers: can the backend lower and compile it?
3. `test` answers: did it actually prove, and did it refresh the proof record?
4. `status` answers: what does the repo project as true right now?

Files that change during that loop:

- generated Rust under `examples/ecommerce/src/generated/`
- unit proof in `*.spec.passport.json`
- molecule proof in `*.test.evidence.json`

`src/generated/` is ephemeral output and is gitignored in this repo. The
canonical example's proof artifacts are tracked on purpose so `spec status` stays
truthful on a fresh clone. Re-running `spec generate` on an unchanged tree
should not rewrite tracked passports just to bump `generated_at`.

If you install the binary, the command name is `spec`, but the repo-native
`cargo run -p spec-cli -- ...` flow is the safest place to start.

## A tiny example

This is the shape of a simple function unit:

```yaml
id: pricing/apply_tax
kind: function
intent:
  why: Add sales tax to a subtotal.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        subtotal + subtotal * rate
    }
```

That source is not just documentation. It is the authored input to validation,
generation, proof, and read-side status/export surfaces.

## Vocabulary starter

These are the words that matter on day 1.

| Term | Meaning | Why you care |
| --- | --- | --- |
| unit | One authored semantic source file, usually `*.unit.spec` | It is the basic building block. |
| kind | The top-level unit shape: currently `function`, `data`, or `sum` | It tells you what the unit is structurally. |
| seam | A non-function shared semantic shape such as `kind:data` or `kind:sum` | It helps explain why seam behavior is different from function-family behavior. |
| atom test | A unit-local test from `local_tests` | It proves one unit in isolation. |
| molecule test | A `.test.spec` interaction test across multiple units | It proves behavior across unit boundaries. |
| passport | The co-located `.spec.passport.json` proof record for a unit | It stores current unit proof truth and freshness anchors. |
| semantic review | The bounded classifier that can understand some `kind:function` meanings | It is intentionally narrow, not general code understanding. |
| family | A shipped semantic-review bucket such as `function.wrapper.pipeline.v1` | It tells you which supported function meaning shape a unit matches. |
| benchmark | A curated proof wall used for bounded public product claims | It answers what the repo can honestly claim today. |

## The mental model

Most confusion in this repo comes from asking one overloaded "is this supported?"
question. Break it into four smaller questions:

1. Is the authored source valid?
2. Can the backend lower and run it?
3. Do we have current proof?
4. What, if anything, can the repo publicly claim from that proof?

Those are different layers.

The clean split is:

- write-path truth: authored specs, validation, generation, build/test, passports, molecule evidence
- read-side interpretation: semantic review, status/export projection, benchmark accounting, recommendation analysis

If you keep that split in your head, the rest of the docs get much easier.

## What to ignore at first

If you are just authoring or fixing a unit, do not start with:

- benchmark roster details
- milestone history
- recommendation and corpus-program decisions
- the full bounded TypeScript lane contract

Start with the core loop. Come back to the deeper surfaces once you need to
explain a status projection, a semantic-review result, or a public product claim.

## Current project shape

This is the honest short version of what the repo currently is:

- Rust-first and proof-first
- semantic source for `function`, `data`, and `sum` units
- atom and molecule proof with tracked evidence
- machine-readable CLI surfaces for validation, status, export, and plan artifacts
- a bounded TypeScript execution lane that is additive, not the default
- benchmark-backed public claims instead of broad "maybe supported" language

The repo's current public claim surface is intentionally narrow. That is a
feature, not a gap. The project prefers small honest boundaries over large
hand-wavy ones.

## Repo layout

- `spec-core/`: parsing, validation, normalization, generation, proof, export primitives
- `spec-cli/`: the `spec` CLI
- `examples/ecommerce/`: the canonical single-library example
- `examples/service/`: the service benchmark example
- `examples/shared-spec/`, `examples/shared-crate/`, `examples/crosslib-app/`: direct sibling-library examples
- `docs/`: explanation docs, contract stacks, and deeper mechanism guides

## Where to read next

Read these in this order if you want the shortest path to fluency:

1. [`docs/README.md`](docs/README.md)
   Use this if you want the categorized docs map and the "current authority
   versus historical draft" split.
2. [`docs/core_mechanisms_guide_v0.1.md`](docs/core_mechanisms_guide_v0.1.md)
   Start here for the repo mental model and the main vocabulary boundaries.
3. [`examples/ecommerce/README.md`](examples/ecommerce/README.md)
   Use this for the canonical end-to-end example and the concrete command loop.
4. [`AGENTS.md`](AGENTS.md)
   Use this if you are editing `.unit.spec` or `.test.spec` files and want the exact workflow.
5. [`docs/rust_v1_contract_stack.md`](docs/rust_v1_contract_stack.md)
   Use this if you need the benchmark-backed Rust V1 claim boundary and command-wall semantics.

Project-state docs:

- [`CHANGELOG.md`](CHANGELOG.md): shipped history
- [`DECISIONS.md`](DECISIONS.md): durable project decisions
- [`PLAN.md`](PLAN.md): active implementation roadmap
- [`TODOS.md`](TODOS.md): backlog and follow-up inventory

## CLI surface

```bash
spec validate     # validate .unit.spec files
spec status       # project per-unit and molecule health
spec generate     # generate Rust from .unit.spec files
spec build        # validate, generate, and cargo build
spec test         # validate, generate, cargo build, cargo test
spec export       # export a machine-readable bundle
spec benchmark    # write derived benchmark snapshots
spec plan         # validate and export .plan.spec files
```

From the repo root, the same commands are available through:

```bash
cargo run -p spec-cli -- <command> ...
```

## Working rules that matter

- Edit source specs, not generated Rust or observed proof artifacts.
- Treat `spec validate --format json` as the main machine-readable feedback channel.
- Use atom tests for one-unit behavior and molecule tests for cross-unit behavior.
- Do not assume semantic review understands every valid unit. Valid proof and semantic support are different things.
- Do not assume repo-root surfaces are the proof-authoritative default. Some broad scopes are inventory or diagnostic surfaces only.

## Why this project exists

The project is trying to make one development style real:

- author intent explicitly
- make the contract machine-readable
- generate readable implementation
- attach proof to what actually passed
- give humans and agents the same truthful surfaces to reason from

That is the whole game.
