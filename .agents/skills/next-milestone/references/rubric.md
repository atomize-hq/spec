# Next Milestone Rubric

Before scoring, classify each realistic candidate into exactly one milestone family:

- `semantic-review-substrate`
  Base `spec` / semantic-review capability growth beyond the current narrow family set.
- `rust-family-promotion`
  A new promoted Rust family or a bounded proof wedge that lands more shipped family truth.
- `corpus-recommendation-policy`
  Corpus expansion, recommendation-policy hardening, or decision-contract work.
- `shared-core-portability`
  Shared-core extraction, seam boundary hardening, and portability honesty.
- `second-language-backend`
  Real non-Rust backend execution support such as first-class TypeScript generate/build/test.
- `operator-consumer-tooling`
  Maintainer-facing truth consumers, verification readers, orchestration surfaces, or other honest operator tooling.

The milestone family is the strategic lane.
The candidate itself is the concrete product-surface milestone inside that lane.
Prerequisite planning may gate a candidate, but it is not itself a candidate milestone.
Every scoring pass must force-rank one winner and explain why each alternate loses.

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
- `contract_alignment`
  - `3`: frozen decision docs name this exact action or clearly imply it
  - `2`: consistent with frozen decision docs
  - `1`: not ruled out, but not named
  - `0`: conflicts with explicit next-action wording
- `churn_penalty`
  - subtract `3`: likely to reopen family-analysis or recommendation-governance churn
  - subtract `2`: broad decision work with thin proof
  - subtract `1`: some ambiguity or likely follow-on drift
  - subtract `0`: focused

## Hard gates

Apply these before comparing totals.

- If a candidate does not create new product truth within one milestone, it should almost never win.
- Recommend the product milestone, not the prerequisite planning step.
- If the winning milestone still needs a plan artifact first, keep the recommendation on that milestone and treat planning as gating handoff only.
- Hard-banned final outputs: `planning`, `planning milestone next`, `author a plan`, `no milestone`, `more evidence`.
- If sources say `recommendation_status = no_strong_candidate`, still pick the strongest honest product-surface candidate and explain why the field is insufficient to overturn the forced ranking.
- If a frozen source names a `required_next_action`, do not skip over it casually.
- If sources say `required_next_action = author_*_plan`, move that constraint into readiness and handoff only. It does not authorize a planning winner or a null answer.
- If `required_next_action` still points at the current planning artifact, do not jump ahead to a later milestone named only inside that artifact's future trigger table or gate section.
- A trigger-table row marked `not yet triggered` cannot win until live evidence shows the trigger fired or a higher-priority source explicitly says to author that follow-on now.
- A future trigger row, authorization branch, or not-yet-fired follow-on cannot beat a current product-lane winner just because the winner is blocked on readiness.
- Blocked readiness does not demote the winner into planning. It changes `Implementation readiness`, `Next artifact kind`, and `Autoplan ready`, not the recommendation itself.
- If a candidate requires the user to make multiple subjective choices just to start, it should usually lose to a more bounded path.
- If current repo signals say `no_strong_candidate`, more corpus or recommendation work should lose unless the blocker is plainly "missing evidence we can collect in one tight pass."
- If the candidate is first-class TypeScript backend support, it must beat the Rust wedge on product leverage and boundedness. "Interesting" is not enough.

## Default interpretation for this repo

- `semantic-review-substrate` is for broadening what the base reviewer or `spec` core can truthfully classify. It can win when the next blocker is substrate capability itself, not "which family next?"
- `rust-family-promotion` starts with a structural advantage because it is closest to product-core truth and existing machinery, but it loses if frozen decision surfaces explicitly say there is no strong next family move.
- `corpus-recommendation-policy` is usually support work, not the headline next move. It wins only when frozen decision surfaces still say evidence quality or decision honesty is the blocker.
- `shared-core-portability` can still be the winning milestone family when frozen decision surfaces say the next honest step is to author the architecture/shared-core follow-on plan first; in that case the milestone is blocked, not replaced by planning.
- For the captured `feat/m40-plus` branch truth, if evidence still shows `pivot_to_architecture_shared_core_follow_on` plus `author_architecture_follow_on_plan`, keep `shared-core-portability` as the winner and record planning only as the gating artifact.
- A closed planning run for `shared-core-portability` does not by itself authorize an `operator-consumer-tooling` or implementation follow-on when the same sources still say `implementation still gated`.
- `second-language-backend` is valid, but expensive. It wins only when the repo has clearly earned real backend expansion rather than another Rust wedge or portability-boundary hardening.
- `operator-consumer-tooling` is real milestone work when the missing capability is an honest maintainer-facing consumer of repo truth, not a hidden helper or dashboard garnish.
- `operator-consumer-tooling` should lose when it is inferred only from a future trigger row and no source has yet named that consumer as the current next action.

## Tie-breakers

If scores are close:

1. Pick the higher `proof_yield`.
2. Then pick the higher `contract_alignment`.
3. Then pick the higher `boundedness`.
4. Then pick the option that reduces future "what next?" ambiguity.
5. Then pick the simpler path that still ships the full lake.
