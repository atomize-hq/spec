---
name: spec
description: >
  Use for the spec workflow: inspect .unit.spec and .test.spec files, validate with structured JSON, regenerate and test units, and interpret passports, molecule evidence, and status output while editing spec-authored source.
---

# spec

Use this skill when working inside a `spec` repository and the task is to author, validate, generate, or test `.unit.spec` or `.test.spec` files with the current AI-native loop.

## Core Loop

1. Run `spec status .` to identify units or molecule tests that are invalid, stale, or missing observed evidence.
2. Run `spec validate [path] --format json` to get machine-readable failures and warnings.
3. Edit the `.unit.spec` file only. Keep generated Rust and passports derived.
4. Run `spec build [path]` to catch Rust type and generation errors.
5. Run `spec test [path]` to execute tests, write passport evidence, and repeat until the unit is valid, fresh, and evidenced.

If you are working on a molecule test, edit the `.test.spec` file and run `spec test path/to/file.test.spec` to refresh only that test's `*.test.evidence.json` artifact.

## `.unit.spec` Anatomy

Each unit is a YAML document with these required fields:

- `id`: hierarchical unit id such as `pricing/apply_tax`
- `kind`: currently `function`
- `intent.why`: the reason the unit exists
- `body.rust`: the function body as a Rust block expression

Common optional fields:

- `contract`
- `deps`
- `imports`
- `local_tests`
- `links`

`body.rust` must contain only the body block, not a full function declaration. The generator synthesizes the `pub fn` signature from `contract.inputs` and `contract.returns`.

## Validation JSON

`spec validate --format json` emits structured output with `schema_version`, `status`, `errors`, and `warnings`. Error objects use stable `SPEC_*` machine codes, not prose.

Treat `errors[]` as the exact list of fix targets. Use the structured fields (`dep`, `field`, `value`, `id`, `cycle`, `path2`) instead of scraping prose when they are present.

`spec status --format json` is a separate contract. It emits `schema_version: 3` and groups results under `roots[]`, with separate `units[]` and `molecule_tests[]` planes per discovered library root.

## `local_tests.expect`

`local_tests[].expect` must parse as a Rust expression. In normal mode, blocks, closures, unsafe blocks, and similar forms are rejected. If the workspace explicitly enables trusted validation overrides, the expression still has to parse, but some otherwise-rejected forms may be accepted.

## Passports and Stale Meaning

`spec generate` and `spec test` write a co-located `.spec.passport.json` file for each unit. The passport records the authored unit metadata and, after `spec test`, runtime evidence such as build status, per-test results, and `observed_at`.

`spec test` also writes co-located `*.test.evidence.json` files for molecule tests. Molecule failures stay on the molecule-test plane and do not poison unit health.

A passport becomes stale when the stored contract hash no longer matches the current contract in the `.unit.spec` file. In `spec status`, stale units are marked with `~` and should be revalidated and retested before being treated as done.
