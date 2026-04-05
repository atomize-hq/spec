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

---

## 2026-04-05 — Generated Output: Ephemeral by Default (0.4.0 decision record)

**Decision:** `spec build` and `spec test` treat generated Rust as ephemeral output. They regenerate into the `--output` directory on each run, assume that directory is spec-owned, and do not require the generated files to be committed.

**Rationale**

- The pipeline commands are optimized for validate → generate → cargo execution in one local flow.
- The existing `.spec-generated` marker and output cleanup rules already model the output tree as spec-owned.
- Teams that want committed generated code for diffs or IDE discoverability can continue to use `spec generate` directly and commit that output intentionally.

**Revisit when**

- We add an explicit committed-output mode such as `--no-regen`.
- CI or editor workflows show that ephemeral-only pipeline behavior creates more friction than it removes.

---

## 2026-04-05 — Cross-Library Dep Schema: Namespace Prefix (0.4.0 decision record)

**Decision:** Cross-library deps will use a namespace-prefixed form:

- Local dep: `money/round`
- Cross-library dep: `shared::money/round`

The future config contract for this syntax is:

```toml
[libraries]
shared = "../shared-spec"
```

`shared` is a namespace alias defined by the consuming workspace. The mapped path points at the
root of another spec library. D6 is design-only in M4: no parser, validator, generator, or CLI
behavior changes ship as part of this decision record. M5 will implement cross-library resolution,
validation, use-path generation, and cycle detection against this contract.

**Tradeoff matrix**

| Candidate | Readability | Backward compatibility | Parser/config complexity | Versioning story | Registry dependence |
|---|---|---|---|---|---|
| `shared::money/round` | High: clearly distinguishes external deps while preserving existing unit-id shape | Strong: existing local deps stay unchanged | Low: one namespace separator plus a simple `[libraries]` mapping | Deferred cleanly; versioning can be added later without polluting v1 syntax | None |
| `money/round@1.2` | Medium: compact, but version pin is mixed into the authored dep | Weak: turns every dep string into a version-aware contract even for local/private use | Medium: parser must split path vs version and validation must define semver policy | Immediate, but premature for the first cross-library cut | None |
| `org/shared/money/round` | Medium: fully qualified, but noisy for normal authoring | Medium: new global path shape replaces the existing local mental model | High: implies registry or org namespace governance beyond local config | Possible, but only with added registry semantics | High |

**Rationale**

- Namespace prefix wins because it preserves the current slash-delimited unit id format for local
  deps, introduces the minimum new syntax needed to identify external libraries, and avoids forcing
  registry infrastructure or version policy into the first cross-library design.
- Versioned paths are rejected for M4/M5 planning because they make version policy part of the
  first schema decision before there is any registry, lockfile, or compatibility story to support
  it. Version pinning is deferred until there is real pressure to resolve multiple library versions.
- Registry paths are rejected for M4/M5 planning because they imply a globally qualified naming
  system and discovery model that the product does not have yet.

**Contract**

- `deps` supports two authored forms:
  - local unit id: `<unit-id>`
  - cross-library unit id: `<namespace>::<unit-id>`
- `namespace` uses lowercase snake-case: `[a-z][a-z0-9_]*`
- `<unit-id>` keeps the existing slash-delimited shape already used by local deps
- Positive examples:
  - `money/round`
  - `shared::money/round`
  - `pricing_shared::tax/apply_tax`
- Invalid examples for the chosen contract:
  - `money/round@1.2`
  - `org/shared/money/round`
  - `Shared::money/round`

**Revisit when**

- Teams need version pinning or multi-version resolution across libraries.
- A registry or organization-wide namespace becomes a real product requirement.
- Cross-library config needs stronger ownership or trust semantics than a local `[libraries]`
  mapping can provide.
