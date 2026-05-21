# Acceptance

- Final command wall must be green on the integrated branch.
- `BENCH-ECOM` must still pass and explicitly require `pricing/discount_strategy_checkout_flow`.
- `BENCH-CROSSLIB` must be complete and still zero-credit.
- The supported-boundary suite must be green and repo-root `status . --format json` must remain `inventory_only`.
- Snapshot commands must produce no new diff after Lane D lands.
