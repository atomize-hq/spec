# spec — Roadmap & Release Shape
**Version:** v0.1  
**Status:** Draft  
**Date:** 2026-03-29

## Purpose

This document translates the current **North Star vision** and **high-level technical architecture** into an execution sequence.

It is intentionally framed around **maturity gates**, not calendar dates. At this stage, the goal is not to predict exact timing. The goal is to define a coherent order of operations so that:

- the roadmap follows the product thesis
- each milestone builds on the previous one
- the first release train proves the core system before broader expansion
- planning, research, and implementation can be broken down against a stable backbone

This document should be read as the bridge between:

1. the long-term vision of spec as a semantic-unit-native development system, and
2. the near-term work needed to build a credible first version.

---

## How to Read This Roadmap

The roadmap is split into two layers:

1. **Large milestones**  
   These are the major capability thresholds the product needs to cross.

2. **Early release shape**  
   These are the first release-sized packages that move the system through those milestones in a practical sequence.

The milestones are durable. The release boundaries may move as we learn.

---

## Sequencing Principles

The roadmap is guided by a few principles derived from the North Star and architecture documents.

### 1. Prove the source model before the workflow model
spec only works if the semantic unit and test model are stable, readable, and enforceable. Fancy editing, planning, and AI loops should come later.

### 2. Prove forward authoring before reverse ingestion
The first job is to show that authored `*.unit.spec` and `*.test.spec` files can reliably produce real code, tests, docs, and machine artifacts. Reverse import of conventional repos should wait.

### 3. Prove verification before intelligence
Passports, similarity, retrieval, and AI assistance become meaningful only after there is a trustworthy validation/build/test/evidence loop.

### 4. Keep source truth separate from derived truth
Human-authored source stays narrow. Passports, diagnostics, coverage data, embeddings, compact fingerprints, and similar artifacts are derived outputs.

### 5. Start narrow on language targets and test adapters
The system should go deep on one target language and one testing path before broadening to multiple languages or frameworks.

### 6. Planning connects to spec; it does not replace it
Planning should operate against semantic units, acceptance criteria, and graph changes only after the underlying source model and evidence model are proven.

---

## Roadmap at a Glance

### Milestone M1 — Semantic Source Foundations
Create the first stable authored source model for units and tests.

### Milestone M2 — Graph Resolution & Native Lowering
Turn validated source artifacts into an internal graph and then into readable native code and test artifacts.

### Milestone M3 — Verification & Evidence
Compile, execute, and observe the generated system so the platform can distinguish declared verification from observed verification.

### Milestone M4 — Passports, Docs, & Machine Artifacts
Emit machine-readable and human-readable derived outputs that make the system inspectable and operable.

### Milestone M5 — Planning-Connected Development
Allow planning artifacts to describe intended graph changes and connect them to implementation and evidence.

### Milestone M6 — Reverse Ingestion & Repo Intelligence
Import conventional repos into draft semantic units and begin supporting bidirectional development workflows.

### Milestone M7 — Advanced Semantics & Automation
Add richer passports, drift analysis, compact semantic fingerprints, and increasingly capable AI-assisted development loops.

---

## Large Milestones

## M1 — Semantic Source Foundations

### Goal
Establish the first credible authored format for semantic units and tests.

### Why it matters
This is the foundation of the entire system. If the source model is unclear, too loose, or too expressive too early, everything above it becomes brittle.

### Capabilities
- `*.unit.spec` and `*.test.spec` as canonical author-facing file conventions
- YAML-by-content parsing with `.spec` as the domain-specific extension
- CUE-backed schema validation and policy enforcement
- first-pass source loader and normalization pipeline
- stable IDs and referential rules for units and tests
- an initial repo shape and project configuration model
- a narrow but usable set of supported authored fields

### Success criteria
- a user can author valid semantic unit and test files without needing internal implementation knowledge
- invalid source fails clearly and deterministically
- validated source normalizes into a canonical internal representation
- the authored format is readable enough to serve as a durable editing surface

### Notes
This milestone should stay intentionally narrow. It is better to validate one strong unit shape than to support too many speculative forms.

---

## M2 — Graph Resolution & Native Lowering

### Goal
Resolve validated source into a semantic graph and lower that graph into readable native code and executable test artifacts.

### Why it matters
spec only becomes real when the semantic source can produce working code. Without this milestone, the system remains an interesting schema layer rather than a development model.

### Capabilities
- internal IR for normalized units and tests
- graph resolution for dependencies and links
- ownership rules for atom tests
- first target-language lowerer
- first test framework adapter
- readable generated native code
- source-to-unit traceability metadata where practical

### Success criteria
- a small but real spec-authored project can be built into native code
- the generated code is readable and inspectable
- link integrity and dependency problems are surfaced before generation or compilation
- atom tests can be materialized into native test artifacts

### Notes
The primary emphasis here should be **clarity**, not optimization. Generated output should be easy to inspect and reason about.

---

## M3 — Verification & Evidence

### Goal
Execute the generated system and attach observed evidence back to semantic units and tests.

### Why it matters
This milestone is where spec stops being declarative only and becomes accountable to runtime evidence.

### Capabilities
- compile and test execution pipeline
- test result ingestion
- coverage or equivalent execution mapping for the first target language
- declared test links vs observed test links
- diagnostics tied back to unit IDs where possible
- baseline health checks around missing tests, broken links, and unexercised code paths

### Success criteria
- the system can report which units were intended to be covered and which units were actually exercised
- failed tests and build errors can be traced back to semantic units
- evidence is stable enough to feed derived artifacts

### Notes
This is one of the most strategically important milestones. It gives spec a truth layer that ordinary config systems do not have.

---

## M4 — Passports, Docs, & Machine Artifacts

### Goal
Emit the first durable derived artifacts that make semantic units useful to both humans and machines beyond the compile step.

### Why it matters
The source model alone is not the product. The product becomes more valuable when it can generate machine-operable outputs and human-readable system views from the same source of truth.

### Capabilities
- unit passports v1
- docs generation v1
- JSON exports of normalized units, tests, links, and evidence
- graph inspection and impact outputs
- first indexable machine artifact bundle
- first clear separation between authored source and build-derived artifacts

### Success criteria
- each meaningful unit has a machine-readable passport
- the platform can emit docs and JSON from the same build
- downstream systems can consume exported artifacts without re-parsing raw source files

### Notes
Embeddings and compact semantic fingerprints may begin experimentally here, but they should not be required to complete this milestone.

---

## M5 — Planning-Connected Development

### Goal
Connect planning artifacts to the semantic source system without collapsing planning into implementation.

### Why it matters
The architecture assumes planning is separate but compatible. This milestone makes that boundary operational.

### Capabilities
- plan artifact schema
- proposed graph change representation
- acceptance criteria linked to units and tests
- semantic diff and change-set model
- impact-aware planning views
- workflow gates that require validate/build/test/evidence before change completion

### Success criteria
- plans can name intended unit changes and acceptance criteria in a structured way
- implementation can be evaluated against those planned changes
- review can see plan, implementation, and evidence in one connected flow

### Notes
This milestone should focus on structural linkage and workflow rigor, not on broad PM tooling.

---

## M6 — Reverse Ingestion & Repo Intelligence

### Goal
Support import and analysis of conventional repos into draft semantic structures.

### Why it matters
This is what allows spec to become an adoption path rather than only a greenfield system.

### Capabilities
- reverse ingestion from existing code, tests, and docs
- candidate unit extraction
- candidate test linkage
- draft intent generation
- partial graph reconstruction
- repo health analysis based on semantic-unit gaps

### Success criteria
- an existing codebase can be partially mapped into draft semantic units
- the system can identify likely missing intent, weak tests, and structural gaps
- reverse-ingested artifacts can be edited and then recompiled forward through the same core pipeline

---

## M7 — Advanced Semantics & Automation

### Goal
Layer richer semantic intelligence and AI workflows on top of the proven core.

### Why it matters
This is where the longer-term moat begins to compound, but it should rest on real source, graph, and evidence foundations.

### Capabilities
- richer passport schema
- compact logic fingerprints
- embeddings and retrieval layers
- drift and coherence scoring across intent, logic, and tests
- AI-assisted unit creation and editing loops
- automated impact-aware review suggestions
- stronger semantic duplicate and reuse detection

### Success criteria
- advanced machine artifacts measurably improve search, diagnostics, or AI reliability
- automation operates against trustworthy constraints and evidence rather than guesswork

---

## Release Mapping

The first release train should primarily move the product through **M1 → M5**.

### Suggested release sequence
- **Release 0.1** — Source Foundations
- **Release 0.2** — Compiler MVP
- **Release 0.3** — Verification & Passports
- **Release 0.4** — Broader Verification & Exports
- **Release 0.5** — Plan-Aware Workflow

This sequence keeps the center of gravity on the spec core instead of prematurely expanding into reverse-ingestion, advanced semantic retrieval, or generalized interfaces.

---

## Release 0.1 — Source Foundations

### Theme
Make the source model real.

### Primary objectives
- prove that `*.unit.spec` and `*.test.spec` are a viable authoring surface
- prove that `.spec` files can be parsed as YAML and validated through CUE
- define the minimum viable semantic unit and test schemas
- establish stable normalization and deterministic diagnostics

### Big-picture deliverables
- project/repo conventions for spec
- source loader for `*.spec` files
- YAML parsing and normalization pipeline
- CUE schema and policy layer v1
- unit schema v1 for a narrow set of semantic unit kinds
- test schema v1 for atom and broader test artifacts
- CLI or build entrypoints for validation and normalization
- example projects demonstrating authoring conventions
- authoring guidance for local tests, links, and contracts

### Explicit non-goals
- production-grade code generation
- coverage or runtime evidence
- advanced passports
- plan integration
- multi-language support

### Exit signal
A user can author a small spec project, run validation, and get reliable normalized output and diagnostics.

---

## Release 0.2 — Compiler MVP

### Theme
Generate readable native code from semantic units.

### Primary objectives
- turn normalized source into a resolved unit graph
- lower that graph into one target language and one test adapter path
- produce readable code and test artifacts that can compile

### Big-picture deliverables
- internal IR v1
- graph resolver for dependencies and links
- local atom test ownership and materialization rules
- first target-language lowerer
- first test harness adapter
- readable generated native source
- generated test files or harness output
- compile-oriented diagnostics and source-to-unit mapping where practical
- build command that runs validation plus generation

### Explicit non-goals
- molecule and organism test orchestration
- strong evidence model
- multiple target languages
- reverse ingestion
- advanced docs and passports

### Exit signal
A small but real project can be authored in spec, lowered to native code, and compiled successfully.

---

## Release 0.3 — Verification & Passports

### Theme
Add runtime truth.

### Primary objectives
- execute generated tests
- attach observed evidence back to semantic units
- produce the first passport layer

### Big-picture deliverables
- compile and test execution pipeline
- test result ingestion and reporting
- declared vs observed test link model
- first coverage or execution mapping for the primary target language
- unit passports v1
- evidence-rich diagnostics
- baseline build health checks
- missing-test and broken-link warnings

### Explicit non-goals
- plan artifacts
- reverse ingestion
- advanced semantic retrieval
- second target language

### Exit signal
Each unit can show a first-pass passport containing identity, intent summary, dependencies, declared links, observed links, and build/test status.

---

## Release 0.4 — Broader Verification & Exports

### Theme
Expand beyond local units into system-level outputs.

### Primary objectives
- support broader test layers beyond atom tests
- generate docs and machine-readable exports from the same build
- make the graph and evidence easier to inspect and consume

### Big-picture deliverables
- molecule test support
- organism test support
- broader test suite assembly rules
- docs generation v1
- JSON export v1 for units, tests, graph, and evidence
- passport export bundle v1
- impact graph and dependency inspection outputs
- improved diagnostics/reporting structure
- optional second adapter decision point (either second test framework or second target language)

### Explicit non-goals
- full planning workflow
- reverse ingestion
- mature embedding or similarity systems

### Exit signal
A spec-authored system can generate code, tests, docs, passports, and JSON exports while supporting layered verification.

---

## Release 0.5 — Plan-Aware Workflow

### Theme
Connect planning to implementation without collapsing the boundary.

### Primary objectives
- define the first structured plan artifact model
- link plans to intended graph changes and acceptance criteria
- make implementation progress measurable against structured intent

### Big-picture deliverables
- plan artifact schema v1
- proposed graph change model
- acceptance criteria model tied to units and tests
- semantic diff/change-set representation
- validate/build/test gates against planned work
- impact-aware change views
- traceability from plan to unit updates to evidence
- workflow hooks suitable for future interactive and AI-driven experiences

### Explicit non-goals
- full project management suite
- generalized IDE replacement
- reverse-ingestion parity with forward authoring

### Exit signal
A planned change can be expressed structurally, implemented through spec units, and evaluated against acceptance criteria and evidence.

---

## What Should Probably Wait Until After 0.5

The following areas feel important, but should probably remain outside the first release train unless something earlier proves easier than expected:

- reverse ingestion from conventional repos
- compact logic fingerprints as a central feature
- semantic embeddings as a hard dependency
- generalized multi-language expansion
- rich graph editing environments
- deep AI automation beyond the validate/build/test loop
- cross-repo analysis and governance layers

These are high-leverage, but they will be much more valuable once the forward authoring and evidence pipeline is proven.

---

## Release-to-Milestone Mapping

| Release | Primary milestone coverage | Secondary milestone coverage |
|---|---|---|
| 0.1 | M1 | prepares M2 |
| 0.2 | M2 | deepens M1 |
| 0.3 | M3 | starts M4 |
| 0.4 | M4 | extends M3 |
| 0.5 | M5 | builds on M4 |
| post-0.5 | M6, M7 | expands whole system |

---

## Recommended Next Planning Layer

Once this roadmap is accepted, the next layer should break work into **research and decision tracks** before detailed implementation planning.

The most important early tracks appear to be:

1. **Source model track**  
   Supported unit kinds, field shapes, contract scope, ID rules, and repo conventions.

2. **YAML/CUE toolchain track**  
   `.spec` loader behavior, validation flow, editor support, schema organization, and normalization rules.

3. **Language target track**  
   Choose the first target language and first test adapter path.

4. **Lowering and IR track**  
   Internal canonical representation, graph resolution rules, and code generation strategy.

5. **Verification track**  
   Atom/molecule/organism representation, test assembly, compile/test execution flow, and coverage strategy.

6. **Passport track**  
   Define the minimum useful passport schema and export contract.

7. **Planning integration track**  
   Define plan artifact boundaries, change-set modeling, and acceptance criteria linkage.

8. **Experience/API track**  
   Determine which interfaces should exist first: CLI, library API, machine-readable build artifacts, and future editor hooks.

That work can then be converted into decision records, research notes, and implementation plans release by release.

---

## Minimal Summary

**Build spec in this order: define the source model, compile it into real code, attach evidence to it, emit durable artifacts from it, then connect planning and higher-order automation on top of it.**
