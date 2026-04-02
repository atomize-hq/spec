# Changelog

## 0.2.0

- **Breaking:** `validate` and `generate` now exit 1 for specs with unresolved internal deps. Previously these passed silently. Ensure all deps are defined in the same spec set before upgrading.
