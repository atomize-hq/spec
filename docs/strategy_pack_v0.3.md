# spec — North Star Vision
**Version:** v0.2  
**Status:** Draft  
**Date:** 2026-03-28

## Purpose

This document defines the long-term product vision for **spec**, a semantic-unit development system for authoring, validating, compiling, and evolving software as structured semantic records rather than as loosely related text files.

It is meant to keep vision, architecture, and sequencing aligned as the system grows from a narrow prototype into a durable development model.

This is a living document. It should be stable enough to guide roadmap decisions, but flexible enough to evolve as we learn.

---

## Vision Statement

**Software should be authored, validated, and evolved as a graph of semantic units rather than as loosely structured source files.**

Each meaningful unit of software should carry:

- a stable identity
- a concise statement of intent
- a contract
- a native implementation
- local verification
- links to broader verification
- machine-readable evidence about how it behaves

From that source of truth, the system should be able to generate:

- native production code
- tests
- documentation
- machine-readable semantic indexes
- verification and traceability artifacts

The long-term outcome is a development model where humans and AI both operate on the same explicit structure, and where implementation, verification, and intent remain continuously aligned.

---

## The Problem We Are Solving

Modern software development is still largely **text-first**.

The core structure of a system is usually implicit rather than explicit:

- intent is scattered across tickets, PRs, comments, and memory
- tests are weakly linked to the code they are meant to verify
- documentation drifts away from implementation
- change impact is hard to reason about at a granular level
- AI coding tools operate on files and snippets, often without enough structure or verification

The result is a familiar set of failures:

- brittle edits
- shallow retrieval
- overconfident code generation
- stale docs
- incomplete tests
- weak traceability between plan, code, and behavior

spec exists to replace that with a model that is:

- more explicit
- more granular
- more verifiable
- more machine-readable
- more resilient to both human and AI mistakes

---

## North Star

In the mature version of spec:

1. **Every meaningful code unit has a durable semantic record.**  
   A function, module, class, workflow step, or other unit is represented as a structured source artifact with intent, contract, implementation, dependencies, and verification.

2. **Verification is attached to the unit graph, not loosely implied.**  
   Local tests are owned by the unit. Broader tests link across units. The system can distinguish declared coverage from observed coverage.

3. **Planning and implementation connect through the same semantic model.**  
   Plans describe proposed graph changes and acceptance criteria. Implementation realizes those changes through unit updates. Verification confirms alignment.

4. **Machine-readable artifacts are derived automatically.**  
   Passports, docs, embeddings, compact fingerprints, dependency graphs, and diagnostics are outputs of the system, not manual side channels.

5. **Humans keep native code.**  
   The source model is not intended to replace Rust, TypeScript, or other target languages. It should generate readable native code and allow escape hatches where needed.

6. **AI is forced into a slower, safer, verify-as-it-builds workflow.**  
   AI should not freehand large stretches of unstructured code. It should propose or update semantic units, run validation, compile, test, observe failures, and iterate.

---

## Product Thesis

The core thesis is:

> If code is authored as a network of typed semantic units with explicit intent and verification, then software systems become easier to understand, safer to change, easier to search, and significantly more reliable for AI-assisted development.

This is not just a new representation format. It is a new development operating model.

---

## What Success Looks Like

### User-level success

A developer can:

- open a unit and immediately understand why it exists
- see the contract, dependencies, and local verification in one place
- know which broader tests actually exercise it
- understand what changed, why, and what else is impacted
- edit a system at the semantic-unit level rather than by hunting through scattered files

An AI system can:

- propose work in unit-sized increments
- validate required fields before generation
- compile and test continuously
- inspect failures against explicit contracts
- avoid broad hallucinated edits because the system shape constrains the work

### System-level success

The platform can:

- compile semantic source into multiple outputs
- maintain traceability from plan to code to tests to evidence
- surface drift between logic, intent, and verification
- build compact machine indexes for retrieval and analysis
- serve as a durable backbone for future planning, search, automation, and governance features

---

## Core Principles

### 1. Semantic units are the source of truth
The primary authored object is not a raw source file. It is a semantic unit with explicit shape and constraints.

### 2. Native code remains a first-class output
The system should generate readable, conventional target-language code. This is not a bid to replace general-purpose languages.

### 3. Intent, logic, and evidence are distinct
The system must model what a unit is supposed to do, how it is implemented, and how that behavior is verified as separate but linked concerns.

### 4. Verification is not optional metadata
Tests and evidence are first-class. The system should treat missing or weak verification as a product concern, not a documentation concern.

### 5. Human readability on the surface, machine rigor underneath
The authoring experience should be easy to read and edit. The validation and normalization layers should be strict and unambiguous.

### 6. Source artifacts and derived artifacts must stay separate
Embeddings, compact fingerprints, coverage results, and diagnostics are generated outputs, not primary authored content.

### 7. Planning is separate from implementation
Planning should describe intended graph changes, constraints, and acceptance criteria. It should not be the source of truth for implementation details.

### 8. Granularity should be semantic, not syntactic
We are not aiming for one file per AST node. We are aiming for one file per meaningful semantic unit.

### 9. The system must be useful before it is universal
The architecture should support a narrow, high-value MVP and grow toward broader generality over time.

---

## Product Boundary

### spec owns the semantic source system
spec should own:

- semantic unit source files
- schemas and validation
- compiler and lowering pipeline
- dependency graph
- unit/test linking model
- passports and machine indexes
- verification and evidence artifacts

### Experience layers sit on top of spec
Interactive tools built around spec may eventually own:

- graph-oriented editing experiences
- unit inspection and review
- plan visualization
- diagnostics and failure surfacing
- navigation across units, tests, docs, and evidence

### Planning is a separate layer
Planning should produce:

- intended graph changes
- work decomposition
- acceptance criteria
- dependency ordering
- validation requirements

Planning should not own the semantic source or generated code.

---

## The Semantic Unit Model

A semantic unit should be the smallest artifact that has:

- a coherent reason to exist
- a stable public contract
- a bounded implementation
- local verification
- a meaningful review surface

Examples:

- a pure leaf function may be its own unit
- a stateful component with private helpers may be one unit
- tiny helpers should remain nested until they deserve promotion

The goal is not maximal fragmentation. The goal is explicit, useful boundaries.

---

## Source Shape

The author-facing format should remain spec-shaped and readable.

Primary source artifacts should look like:

- `*.unit.spec`
- `*.test.spec`

These files are **YAML by content** and should be treated as YAML throughout parsing, validation, formatting, and editor support. The `.spec` suffix signals domain intent; it does not imply a new programming language.

---

## Testing Model

The system should support a layered verification model:

- **Atom tests** — local, unit-owned tests that live with the semantic unit
- **Molecule tests** — integration tests covering a few collaborating units
- **Organism tests** — broader feature or workflow tests spanning a domain slice

The unit should own atom tests directly and link to broader tests indirectly.

This allows the platform to represent both:

- what a unit claims should verify it
- what execution evidence shows actually verifies it

That distinction is strategically important.

---

## Semantic Passports

Each unit should eventually produce a **passport**: a machine-readable summary of implementation, intent, verification, dependencies, and observed evidence.

A passport may include:

- stable unit ID
- symbol path
- normalized logic form
- dependency summary
- declared test links
- observed test links
- coverage evidence
- compact fingerprints
- embeddings
- drift or coherence scores

Passports are a derived layer, not the authored source.

---

## Why This Matters for AI

The system is not just about better code organization. It is about making software development legible to machine systems without making it illegible to humans.

Today, AI often works too broadly and too quickly:

- it edits too much at once
- it assumes missing constraints
- it invents implied behavior
- it under-verifies
- it loses the relationship between plan, code, and tests

A semantic-unit workflow forces a better loop:

1. understand the target unit or create a new one
2. fill required fields
3. update implementation
4. compile
5. run local verification
6. inspect diagnostics
7. link broader tests
8. finalize only when evidence supports the change

That slower loop is a feature, not a bug.

---

## Non-Goals

At this stage, this system is **not** intended to:

- create a new general-purpose programming language
- eliminate native code as a human-readable artifact
- formalize every helper into its own top-level source file
- guarantee complete semantic equivalence across arbitrary implementations
- deliver full formal verification in the first versions
- replace all existing repo structures and workflows immediately

---

## Strategic Advantages

If executed well, this architecture creates a durable moat:

### 1. Traceability by construction
Intent, code, and tests are linked at the unit level rather than stitched together after the fact.

### 2. Better AI reliability
AI can operate against explicit schemas, contracts, and verification checkpoints.

### 3. Better retrieval
Search can target semantic units, not just text blobs or file chunks.

### 4. Better review
Reviews can happen at the unit boundary with explicit contract and evidence context.

### 5. Multi-output value
One authored source can generate code, docs, machine indexes, and diagnostics.

### 6. Drift detection
The platform can identify when implementation, tests, and intent stop agreeing.

---

## Design Commitments for the Near Term

For the first phase, we are intentionally constraining the shape:

- author-facing source files remain simple and readable
- native code lives inside structured semantic unit files
- schemas and constraints sit beneath the authoring surface
- local tests are supported first
- broader tests remain linked artifacts
- derived machine artifacts are generated automatically
- code generation targets a small number of languages first

This keeps the architecture ambitious while keeping the implementation tractable.

---

## Long-Term Direction

Over time, the system should evolve toward:

- stronger contracts and invariants
- richer unit-level evidence
- better plan-to-implementation traceability
- deeper semantic indexing
- smarter AI edit and review loops
- drift and coverage health scoring
- reverse-ingestion of conventional repos into semantic units
- richer graph-aware development environments built on top of spec

---

## The North Star in One Sentence

**Build a semantic-unit-native development system where implementation, intent, and verification are explicit, linked, and machine-operable from the start.**


---

# spec — High-Level Technical Architecture
**Version:** v0.2  
**Status:** Draft  
**Date:** 2026-03-28

## Purpose

This document captures the initial technical architecture for **spec**, a semantic-unit system for authoring, validating, compiling, linking, and verifying software as structured semantic records.

It is intentionally high level. The goal is to define system boundaries, core abstractions, and the first stable implementation shape without overcommitting to details too early.

This architecture is designed to support:

- semantic-unit source authoring
- strict validation and normalization
- compilation into native code
- layered verification
- machine-readable passports and indexes
- future planning and AI orchestration on top

---

## Architectural Summary

The broader ecosystem around spec has three major layers:

1. **Planning Layer**  
   Produces proposed graph changes, acceptance criteria, and dependency ordering.

2. **Semantic Source Layer (spec core)**  
   Owns the source-of-truth unit graph, schemas, validation, compilation, linking, and evidence.

3. **Experience Layer**  
   Provides the user-facing environment for editing, inspection, review, and navigation.

The core technical focus for the first releases is the **Semantic Source Layer**.

---

## System Boundary

### In scope for spec

- semantic unit source format
- test source format
- schema and policy definitions
- validation and normalization pipeline
- graph resolution
- code generation and lowering
- test assembly and execution hooks
- evidence collection
- passport generation
- docs and JSON export

### Out of scope for spec

- freeform planning UX
- long-horizon project management
- final code review UX
- generalized IDE replacement
- broad repo analytics beyond semantic units

Those experiences may exist, but they sit above or around spec rather than inside the core semantic compiler.

---

## Core Concepts

### 1. Semantic Unit
A semantic unit is the primary authored object.

A semantic unit is the smallest source artifact that has:

- a stable identity
- a clear intent
- a contract
- a bounded body of implementation
- local verification
- explicit dependencies

Examples:

- a pure function
- a stateful service component
- a domain module
- a transformation or workflow step

A semantic unit is **semantic**, not merely syntactic. It should align to a meaningful boundary.

### 2. Test Artifact
A test artifact represents verification at one of several levels:

- atom
- molecule
- organism

Atom tests are usually owned directly by a unit. Molecule and organism tests are separate artifacts that may cover multiple units.

### 3. Unit Passport
A unit passport is a derived artifact containing machine-readable information about the unit’s structure, verification, dependencies, and observed evidence.

### 4. Plan Artifact
A plan artifact is a future-facing object that describes intended graph changes. It is not part of the semantic source of truth.

---

## Authoring Format

### Decision
Use **YAML as the underlying authoring format** and **`.spec` as the domain-specific file extension**.
For **0.1.x–0.2.x**, validate with **JSON Schema** (see `DECISIONS.md`). **CUE** remains a candidate for **0.3+**
when cross-file constraints and policy composition justify the complexity. Do not design against CUE until then.

### Why
YAML gives a readable, familiar surface that works well for spec-shaped documents and embedded native code blocks.
JSON Schema is the currently implemented path for validation and keeps the project aligned with shipped behavior. CUE
may become valuable later if/when the system needs stronger cross-file constraint composition.

### Practical interpretation
Authors primarily edit:

- `*.unit.spec`
- `*.test.spec`

These files are YAML documents by content. The build system should parse them as YAML, validate them against JSON
Schema, and then run any additional semantic/policy validation before normalization/compilation.

### Tooling implication
Editor support should associate `*.spec` with YAML syntax and (optionally) attach JSON Schema tooling for authoring.

---

## Source Model

### Semantic unit source
A semantic unit source file should contain human-authored truth:

- `id`
- `kind`
- `intent`
- `contract`
- `deps`
- `body`
- `local_tests`
- `links`

### Test source
A non-local test file should contain:

- `id`
- `tier`
- `covers`
- `scenario`
- optional setup and fixtures metadata

### Derived artifacts
The following should not be hand-authored in source files:

- embeddings
- normalized nameless logic forms
- compact fingerprints
- coverage counts
- drift and coherence scores
- generated docs
- generated native code

These belong in the build output.

---

## Example Repository Shape

```text
spec/
  schema/
    unit.cue
    test.cue
    policies/
  units/
    pricing/
      apply_discount.unit.spec
      apply_tax.unit.spec
    checkout/
      finalize_total.unit.spec
  tests/
    pricing/
      discount_plus_tax.test.spec
    checkout/
      final_total_flow.test.spec
  build/
    generated/
    passports/
    docs/
    diagnostics/
```

This shape is illustrative, not final.

---

## Example Unit File

```yaml
kind: function
id: pricing/apply_discount
intent:
  why: Apply a percentage discount while preserving nonnegative money values.
contract:
  inputs:
    subtotal: Money
    rate: Decimal
  returns: Money
  invariants:
    - output <= subtotal
    - output >= 0
deps:
  - money/round
body:
  rust: |
    pub fn apply_discount(subtotal: Decimal, rate: Decimal) -> Decimal {
        let discounted = subtotal - subtotal * rate;
        round_money(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: happy_path
    expect: apply_discount(100, 0.10) == 90
  - id: zero_rate
    expect: apply_discount(100, 0.0) == 100
links:
  molecule_tests:
    - pricing/discount_plus_tax
```

---

## Example Test File

```yaml
kind: test
tier: molecule
id: pricing/discount_plus_tax
covers:
  - pricing/apply_discount
  - pricing/apply_tax
scenario:
  description: Discount is applied before tax in final pricing flow.
```

---

## Primary Pipeline

The semantic source layer should follow a deterministic multi-stage pipeline.

### 1. Ingest
Load unit and test spec files from configured roots.

### 2. Validate
Apply JSON Schema validation plus any additional semantic/policy rules (0.1/0.2; see `DECISIONS.md`).

Validation includes:

- required fields
- kind-specific shapes
- allowed extra fields
- referential integrity checks
- basic contract consistency
- test tier rules
- link shape validation

### 3. Normalize
Convert validated spec files into an internal canonical representation.

Normalization should:

- resolve defaults
- canonicalize IDs
- standardize dependency references
- normalize test links
- prepare data for code generation and indexing

### 4. Resolve Graph
Build the semantic dependency graph.

Graph resolution includes:

- unit-to-unit dependencies
- unit-to-test declared links
- ownership rules for local tests
- reverse references
- cycle diagnostics where relevant

### 5. Lower and Generate
Generate target-language artifacts from the internal representation.

Potential outputs:

- Rust source
- TypeScript source
- test harness code
- docs stubs
- machine-readable JSON

### 6. Compile and Execute
Compile generated native code using the appropriate target toolchain and run selected tests.

### 7. Collect Evidence
Gather observed results such as:

- compile diagnostics
- test results
- coverage data
- execution-to-unit mappings

### 8. Emit Derived Artifacts
Generate:

- unit passports
- docs
- JSON exports
- diagnostics
- search and index inputs

---

## Architectural Components

### A. Source Loader
Responsibilities:

- discover source files
- read YAML-backed `.spec` files
- track file paths and origin metadata
- support incremental change detection later

### B. Schema and Policy Engine
Responsibilities:

- host schema definitions (JSON Schema for 0.1/0.2; see `DECISIONS.md`)
- validate source files
- enforce closed and open shape policy
- apply defaults and normalization hints

### C. Internal IR Builder
Responsibilities:

- convert validated source into canonical internal structures
- separate authored content from derived content
- provide a stable API for downstream stages

### D. Graph Resolver
Responsibilities:

- create dependency graph
- attach local tests to units
- attach broader test references
- detect unresolved IDs and illegal edges

### E. Code Generator and Lowerer
Responsibilities:

- map semantic units into target-language artifacts
- generate readable source
- preserve source maps back to unit IDs where possible

### F. Test Assembler
Responsibilities:

- materialize local atom tests
- assemble broader test suites
- route tests to appropriate native framework adapters

### G. Evidence Collector
Responsibilities:

- collect test outcomes
- collect compile diagnostics
- collect observed coverage
- associate execution evidence back to unit IDs

### H. Passport Builder
Responsibilities:

- generate machine-readable per-unit summaries
- merge declared and observed verification links
- prepare data for downstream indexing and retrieval

### I. Export Layer
Responsibilities:

- write generated code
- write docs
- write JSON artifacts
- write diagnostics and reports

---

## Verification Model

### Atom tests
Atom tests are local to the unit. They should be lightweight and close to the implementation.

Typical use cases:

- simple examples
- edge cases
- invariants
- property-style checks where supported

### Molecule tests
Molecule tests cover the interaction of a small number of units.

### Organism tests
Organism tests cover broader feature or workflow behavior.

### Declared vs observed links
The architecture should maintain both:

- **declared links** — what source files say should be covered
- **observed links** — what runtime evidence shows was executed

This distinction is important for future drift and coverage diagnostics.

---

## Unit Passport Shape

A first-pass passport may contain:

- `unit_id`
- `kind`
- `symbol_path`
- `intent_summary`
- `contract_summary`
- `declared_deps`
- `declared_test_links`
- `observed_test_links`
- `coverage_summary`
- `build_status`
- `generated_artifacts`

Later versions may add:

- normalized logic forms
- compact fingerprints
- embeddings
- drift and coherence scores
- call graph summaries
- effect signatures

Passports are versioned derived data.

---

## Compiler Outputs

The architecture should support at least four output classes:

### 1. Native code output
Readable Rust or TypeScript (or other target language) source.

### 2. Verification output
Materialized tests, test reports, and coverage artifacts.

### 3. Human-readable documentation
Rendered docs from semantic unit metadata.

### 4. Machine-readable output
JSON, passports, and indexable artifacts for search and analysis.

---

## Integration with External Layers

spec should be usable as a backend semantic system rather than as a UI framework.

Upstream or adjacent tools should eventually be able to:

- edit and inspect unit files
- navigate unit graph relationships
- display contract and intent beside generated code
- surface validation failures and test evidence
- show diffs at semantic-unit granularity
- render plan-to-unit mapping when the planning layer exists

The core principle is:

- spec computes truth
- surrounding tools present and manipulate it

---

## Incremental and Reverse Flows

The architecture should support two flows over time.

### Forward flow
Author semantic units and compile them into native outputs.

### Reverse flow
Ingest conventional repos and derive semantic-unit candidates from existing code, tests, and documentation.

Reverse flow is not required in the first release, but the internal model should not prevent it.

---

## Initial Technical Priorities

The first implementation should prioritize:

1. `.spec` plus JSON Schema validation for a narrow unit schema (0.1/0.2; see `DECISIONS.md`)
2. one or two target languages only
3. local atom tests first
4. simple broader test linking
5. readable code generation over aggressive optimization
6. clear diagnostics and referential integrity
7. stable unit IDs and graph construction
8. build outputs that are easy to inspect and diff

This keeps the system tractable while still proving the core thesis.

---

## Risks and Mitigations

### Risk 1: Over-designing the language surface
If the unit format becomes too expressive too early, the project turns into a language design effort.

**Mitigation:** Keep the authored format narrow and let native code blocks carry most implementation complexity at first.

### Risk 2: Repo explosion from over-granularity
If every tiny helper becomes a top-level unit, the system becomes noisy and hard to manage.

**Mitigation:** Define semantic-unit heuristics and allow nested implementation inside a unit.

### Risk 3: Schema logic leaks everywhere
If validation rules are split across naming conventions, compiler code, and ad hoc scripts, the system becomes brittle.

**Mitigation:** Centralize shape and policy rules in the schema + validator layer and keep the compiler focused on lowering and evidence.

### Risk 4: Generated code is unreadable
If output code is hard to inspect, trust in the system will drop.

**Mitigation:** Optimize for readable code generation first, compact semantic forms second.

### Risk 5: Weak test linkage
If test relationships are only declared and never observed, the evidence model will be weak.

**Mitigation:** Add observed coverage mapping as early as feasible.

### Risk 6: Extension friction in tooling
If editors, schema tools, and CLI steps do not consistently treat `.spec` files as YAML, the experience will feel brittle.

**Mitigation:** Standardize loader behavior, provide editor associations, and make YAML interpretation explicit in build tooling.

---

## Open Questions

The architecture intentionally leaves several questions open:

- What exact kinds of semantic units are supported in v1?
- How much contract expressiveness should be allowed in source?
- How should local tests map into native testing frameworks?
- How should source maps from generated code back to unit IDs work?
- What is the minimal useful passport schema?
- When should reverse ingestion of existing repos begin?
- How much of the planning system should share the same file model versus a separate plan artifact model?

These should be resolved through roadmap milestones and targeted decision records.

---

## Minimal Architecture in One Sentence

**Validate structured semantic units, resolve them into a graph, lower them into native code and tests, execute them, and emit evidence-rich artifacts that humans and machines can both operate on.**


---

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
- JSON-Schema-backed validation and policy enforcement (0.1/0.2; see `DECISIONS.md`)
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
- prove that `.spec` files can be parsed as YAML and validated through JSON Schema (0.1/0.2; see `DECISIONS.md`)
- define the minimum viable semantic unit and test schemas
- establish stable normalization and deterministic diagnostics

### Big-picture deliverables
- project/repo conventions for spec
- source loader for `*.spec` files
- YAML parsing and normalization pipeline
- JSON Schema + semantic/policy validation rules v1 (0.1/0.2; see `DECISIONS.md`)
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

2. **YAML/schema validation track**  
   `.spec` loader behavior, JSON Schema validation flow (0.1/0.2; see `DECISIONS.md`), editor support, schema organization,
   and normalization rules. CUE remains a candidate for 0.3+ when cross-file constraints and policy composition justify it.

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
