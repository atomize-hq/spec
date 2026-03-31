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
