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
Use **YAML as the underlying authoring format**, **`.spec` as the domain-specific file extension**, and **CUE as the schema, validation, normalization, and policy layer**.

### Why
YAML gives a readable, familiar surface that works well for spec-shaped documents and embedded native code blocks. CUE provides stronger guarantees around required fields, closed shapes, defaults, constraints, and cross-file composition.

### Practical interpretation
Authors primarily edit:

- `*.unit.spec`
- `*.test.spec`

These files are YAML documents by content. The build system should parse them as YAML, validate them against CUE definitions, and normalize them before compilation.

### Tooling implication
Because CUE’s CLI infers file type from filename suffix by default, a custom `.spec` suffix should be handled by either:

- a source loader that reads `*.spec` files as YAML before they reach downstream stages, or
- explicit CUE input qualifiers such as `yaml:` in CLI invocations

Editor support should also associate `*.spec` with YAML syntax and schema tooling.

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
Apply CUE schema and policy validation.

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

- host CUE definitions
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

1. `.spec` plus CUE validation for a narrow unit schema
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

**Mitigation:** Centralize shape and policy rules in CUE and keep the compiler focused on lowering and evidence.

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
