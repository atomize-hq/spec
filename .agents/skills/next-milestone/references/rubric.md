# Next Milestone Rubric

Score each realistic candidate on these dimensions from `0` to `3`.

- `product_core_leverage`
  - `3`: directly expands semantic review or other core product truth
  - `2`: strongly enables near-term core work
  - `1`: indirect support
  - `0`: mostly local cleanup or side work
- `proof_yield`
  - `3`: produces new supported proof, fresh semantic review truth, or a clearly stronger shipped capability
  - `2`: unlocks proof quickly in the next step
  - `1`: mostly analysis
  - `0`: no new truth
- `boundedness`
  - `3`: one milestone, one wedge, obvious done shape
  - `2`: moderate scope, still finishable without reopening architecture
  - `1`: broad or fuzzy
  - `0`: ocean
- `reuse_of_live_machinery`
  - `3`: uses existing M26-style or shipped repo machinery directly
  - `2`: light extension of existing machinery
  - `1`: meaningful new machinery
  - `0`: mostly net-new stack
- `signal_pressure`
  - `3`: current checkpoint, docs, and live repo signals all point here
  - `2`: two of the three point here
  - `1`: weak signal
  - `0`: mostly speculative
- `churn_penalty`
  - subtract `3`: likely to reopen family-analysis or recommendation-governance churn
  - subtract `2`: broad decision work with thin proof
  - subtract `1`: some ambiguity or likely follow-on drift
  - subtract `0`: focused

## Hard gates

Apply these before comparing totals.

- If a candidate does not create new product truth within one milestone, it should almost never win.
- If a candidate requires the user to make multiple subjective choices just to start, it should usually lose to a more bounded path.
- If current repo signals say `no_strong_candidate`, more corpus or recommendation work should lose unless the blocker is plainly "missing evidence we can collect in one tight pass."
- If the candidate is first-class TypeScript backend support, it must beat the Rust wedge on product leverage and boundedness. "Interesting" is not enough.

## Default interpretation for this repo

- Rust semantic-review wedge expansion starts with a structural advantage because it is closest to product-core truth and existing machinery.
- Reusable seam semantic-review expansion can win if it unlocks repeated semantics across more than one real example without demanding a large new framework.
- First-class TypeScript backend work is valid, but expensive. It wins only when the repo has clearly exhausted the next useful Rust wedge or the product docs explicitly elevate multi-language execution now.
- Family-analysis and corpus-governance work should usually be support work, not the next headline milestone.

## Tie-breakers

If scores are close:

1. Pick the higher `proof_yield`.
2. Then pick the higher `boundedness`.
3. Then pick the option that reduces future "what next?" ambiguity.
4. Then pick the simpler path that still ships the full lake.
