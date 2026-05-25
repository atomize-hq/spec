# Decisions

This file records project-level decisions that should remain stable across the repo.

---

## ICP (v0.5)

The ICP for v0.5 is a solo engineer or a small team of roughly 2-5 people who use AI coding assistants every day, work on systems where correctness matters, and want generated code they can trust because it is governed by explicit spec contracts and machine-readable evidence rather than terminal scraping or guesswork.

---

## 2026-05-22 — Rust V1 Scope Closure (I7)

**Decision**

Rust V1 stays narrow, synchronous, and benchmark-backed for the I8 proof run:

- bounded generics defer to `V1.1`
- async flows, runtime adapters, and IO-owned boundaries defer to `V1.1`
- `BENCH-CROSSLIB` stays active as companion negative proof and never earns positive supported credit
- `I8` uses the existing five-command proof wall without any new slice-specific commands

**Rationale**

- The existing `BENCH-ECOM` and `BENCH-SERVICE` walls already prove one honest narrow-core Rust claim.
- No bounded-generic or async/IO slice could be named in one paragraph with believable proof commands on top of the frozen I3.5 wall.
- Explicit deferral is safer than widening the Rust V1 claim by prose drift or by importing runtime/framework expectations as ambient support.

**Revisit when**

- A future milestone can name one bounded generic or async/IO slice, its exact repo surfaces, and the exact added proof commands required to prove it honestly.

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
- Co-located proof artifacts may still be tracked by examples or fixtures, so no-op `spec generate` runs should avoid timestamp-only passport rewrites when no proof or authored truth changed.

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

---

## 2026-04-11 — schema_version: Integer, Not String (0.5.1 decision record)

**Decision:** `schema_version` in export bundles and JSON status/validate responses is emitted as the JSON integer `1`, not the string `"1.0"`.

**Rationale**

- Integers are unambiguous for version comparisons and require no string parsing. A consumer checking `schema_version === 1` is correct; a consumer checking `schema_version === "1.0"` already silently broke when the patch digit changed.
- Using a bare integer removes the misleading minor-version component: schema shape changes are never patch-level, so `"1.0"` vs `"1.1"` would have implied a meaningful distinction that does not exist.
- The integer form matches common practice in protocol versioning (e.g., HTTP/2 version fields in JSON APIs).

**Breaking impact**

- Consumers that string-match `"schema_version": "1.0"` must update to match against the integer `1`. Consumers that parse the field as a number are unaffected.

**Revisit when**

- We need to distinguish major vs minor schema variants within a single major version (not anticipated before 1.0).

---

## 2026-04-11 — Concurrent Passport Writes: Warn, Not Lock (0.5.1 decision record)

**Decision:** When multiple `spec` processes attempt to write passports for the same unit at the same time, `spec` emits a warning to stderr and continues. No blocking lock is taken.

**Rationale**

- The passport write itself is atomic (write to a temp file, then rename). The warn-only guard detects the collision window but does not prevent it; the last writer wins.
- A blocking lock introduces the risk of deadlock or indefinite stall in CI environments where agents are killed without cleanup, leaving stale lock files.
- Passport evidence is append-friendly: the worst outcome of a concurrent write is that one run's evidence overwrites another's. Both runs were valid observations; neither is silently lost from the overall system because the next `spec test` will regenerate fresh evidence.
- This matches the M5 "trust, not lock" design philosophy: prefer observable warnings and human/agent follow-up over hard serialization at the tool level.

**Revisit when**

- We see real data loss or correctness failures from last-writer-wins behavior in production multi-agent pipelines.
- A CI orchestrator provides a better coordination primitive (e.g., a shared artifact store with CAS semantics) that makes advisory locks unnecessary.
