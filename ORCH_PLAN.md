# I3.5 Orchestration Plan

Status: **authoritative execution runbook**
Milestone: **I3.5 authority alignment and repo-root contract freeze**
Last rewritten: **2026-05-20**

## Authority

Use these files as the only milestone authority for this runbook:

- `.runs/i3_5_authority_alignment/authority-plan.snapshot.md`
- `.runs/i3_5_authority_alignment/phase2-freeze.json`

This runbook does not inherit facts from the older I3 mechanics runbook.

## Goal

I3.5 is the alignment milestone between the landed I3 benchmark mechanics and
the future I4 regression wall.

The job is narrow:

- keep benchmark-root commands as the proof-authoritative default
- freeze repo-root `status` as `inventory_only`
- freeze repo-root `export` as unsupported with `SPEC_UNSUPPORTED_SCOPE`
- make README, example docs, changelog, and runbooks teach the same command wall

## Frozen Command Wall

These commands are the public contract after I3.5:

```bash
cargo run -p spec-cli -- status examples/ecommerce/units --format json
cargo run -p spec-cli -- export examples/ecommerce/units
cargo run -p spec-cli -- status examples/ecommerce/units/pricing --format json
cargo run -p spec-cli -- status examples/ecommerce/units/pricing/apply_discount.unit.spec --format json
cargo run -p spec-cli -- status . --format json
```

Interpretation rules:

- `status examples/ecommerce/units --format json` is the fresh-clone and CI
  proof wall
- `export examples/ecommerce/units` is the benchmark-root export wall
- namespace and single-file `status` are diagnostic-only partial scopes
- `status . --format json` stays supported, but only as broad inventory with
  scope authority `inventory_only`
- `export .` is unsupported for this workspace shape and must fail with
  `SPEC_UNSUPPORTED_SCOPE`

## Worker Lanes

The freeze record splits execution into two bounded implementation lanes:

- `task/i3_5-b-cli-hardening`
  - branch: `codex/i3-5-lane-cli`
  - owns CLI and fixture behavior
- `task/i3_5-c-doc-hardening`
  - branch: `codex/i3-5-lane-docs`
  - owns `README.md`, `examples/ecommerce/README.md`,
    `docs/rust_v1_contract_stack.md`, `ORCH_PLAN.md`, and `CHANGELOG.md`

Merge order is fixed:

1. `task/i3_5-b-cli-hardening`
2. `task/i3_5-c-doc-hardening`

## Non-Negotiable Rules

- Do not reopen I3 benchmark mechanics.
- Do not teach repo-root `status` as proof authority.
- Do not describe repo-root `export` as a supported aggregate bundle.
- Do not reintroduce stale local-user planning paths into repo docs.
- Do not let docs and help text disagree about the trusted command wall.

## Acceptance Checks

Documentation and runbook work is only done when these conditions are true:

- public docs point fresh-clone users at
  `cargo run -p spec-cli -- status examples/ecommerce/units --format json`
- repo-root `status . --format json` is described as `inventory_only`
- repo-root `export .` is described as unsupported with
  `SPEC_UNSUPPORTED_SCOPE`
- the contract-stack index teaches the ladder `M65-M68 -> I3 -> I3.5 -> I4`
- no stale local-user planning paths remain in the owned docs

Required audit commands:

```bash
rg "spec status \\." README.md examples/ecommerce/README.md ORCH_PLAN.md docs/rust_v1_contract_stack.md CHANGELOG.md
rg "SPEC_UNSUPPORTED_SCOPE|inventory_only|examples/ecommerce/units" README.md examples/ecommerce/README.md ORCH_PLAN.md docs/rust_v1_contract_stack.md CHANGELOG.md
```
