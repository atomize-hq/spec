Summary: Added end-to-end M10 coverage and public docs. The CLI suite now covers directory rejection, nested plan root resolution, outside-root failure, symlink escape rejection, cross-library plan rejection, modify/remove impact semantics, mixed modify/add partial impact, checked-in JSON fixtures for plan validate/export, and a guard that `spec export` remains unchanged. README and AGENTS now document the plan-command contract and `SPEC_PLAN_*` diagnostics.

Files changed:
- `spec-cli/tests/cli.rs`
- `spec-cli/tests/fixtures/plan-validate-valid-mixed.json`
- `spec-cli/tests/fixtures/plan-export-valid-mixed.json`
- `README.md`
- `AGENTS.md`

Commands run:
- `cargo test -p spec-cli`
- `cargo test -p spec-core`
- `cargo test --all`

Result:
- Passed.

Blockers:
- None.
