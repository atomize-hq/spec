# Decisions

This file records project-level decisions that should remain stable across the repo.

---

## 2026-04-02 — Validation Strategy (0.1.x–0.2.x): JSON Schema

**Decision**

For **0.1** and **0.2**, **JSON Schema is the implementation path** for validation. **CUE remains a candidate
for 0.3+** when cross-file constraints and policy composition justify the complexity.

**Do not design against CUE until then.**

**Rationale**

- The current codebase validates with JSON Schema, and the CLI and examples are built around that workflow.
- Keeping one clear source of truth prevents doc-driven design drift and avoids building against deferred tooling.

**Revisit when**

- We need first-class **cross-file constraints** that JSON Schema cannot express cleanly.
- We introduce **policy composition** / layered rule sets that benefit from a richer constraint language.
- We want schema-level defaults/closures to become a central authoring UX feature (and JSON Schema becomes painful).

---

## 2026-04-03 — CUE Trigger Condition

**Decision:** CUE remains deferred. Implement CUE when one of these triggers is met:
(a) Cross-library dep validation requires user-configurable policy that JSON Schema cannot express, OR
(b) Teams need team-authored constraint overlays on top of core schema.

Until then: JSON Schema + semantic validation in Rust is the implementation path. Do not design against CUE until a trigger condition is explicitly met.

