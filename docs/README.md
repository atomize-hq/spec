# Documentation Map

This repo has two different kinds of docs:

- current authority docs that describe what `spec` does today
- versioned design drafts that preserve historical planning context

If two docs disagree, trust the current authority docs first:

1. [`README.md`](../README.md)
2. [`core_mechanisms_guide_v0.1.md`](./core_mechanisms_guide_v0.1.md)
3. [`CHANGELOG.md`](../CHANGELOG.md)
4. [`PLAN.md`](../PLAN.md)
5. [`DECISIONS.md`](../DECISIONS.md)

That ordering keeps the repo's newcomer story, shipped behavior, active plan,
and durable decisions aligned.

## Start Here

- [`README.md`](../README.md)
  Repo entry point. What `spec` is, the first command loop, and the day-1 vocabulary.
- [`core_mechanisms_guide_v0.1.md`](./core_mechanisms_guide_v0.1.md)
  The mental model doc. Use this when you need to separate authored truth, proof, semantic review, families, and benchmarks.
- [`kind_coverage_map_v0.1.md`](./kind_coverage_map_v0.1.md)
  The progress-map doc. Use this when you need to know which kinds and per-kind categories are shipped, deferred, or still unnamed.
- [`function_category_matrix_v0.1.md`](./function_category_matrix_v0.1.md)
  The detailed `kind:function` matrix. Use this when you need per-category truth for families, TypeScript, proof, and benchmark role.
- [`data_category_taxonomy_v0.1.md`](./data_category_taxonomy_v0.1.md)
  The detailed `kind:data` taxonomy. Use this when you need the exact supported data descriptor, its benchmark role, and the remaining unnamed pressure inside seam categories.
- [`sum_category_taxonomy_v0.1.md`](./sum_category_taxonomy_v0.1.md)
  The detailed `kind:sum` taxonomy. Use this when you need the exact supported sum descriptor, its benchmark role, and the current split between canonical detector wording and the broader service sibling surface.
- [`category_truth_contract_v0.1.md`](./category_truth_contract_v0.1.md)
  The cross-cutting honesty contract for category-backed read-side claims. Use this when you need to know how benchmark, status, export, and future consumers should decide support and positive-credit eligibility without inferring from partial truth.
- [`category_truth_contract_correction_v0.1.md`](./category_truth_contract_correction_v0.1.md)
  The corrective note for M101. Use this when you need the explicit decision that category qualification must consume stored semantic truth rather than minting fresh read-side category truth.
- [`examples/ecommerce/README.md`](../examples/ecommerce/README.md)
  The canonical concrete walkthrough. Use it when you want one real example root to run and inspect.
- [`AGENTS.md`](../AGENTS.md)
  Exact workflow rules for editing `.unit.spec`, `.test.spec`, and `.plan.spec` files.

## Current Authority

- [`CHANGELOG.md`](../CHANGELOG.md)
  Shipped changes and repo-facing behavior changes.
- [`PLAN.md`](../PLAN.md)
  Active implementation plan and current milestone authority.
- [`DECISIONS.md`](../DECISIONS.md)
  Durable decisions that should not be re-litigated casually.
- [`ORCH_PLAN.md`](../ORCH_PLAN.md)
  Current orchestration runbook for the active plan.
- [`rust_v1_contract_stack.md`](./rust_v1_contract_stack.md)
  The Rust V1 command-wall and benchmark-claim index.

## Vision And Architecture

These docs are still useful, but they are design context, not the day-to-day
source of shipped CLI truth.

- [`north_star_v0.2.md`](./north_star_v0.2.md)
  Long-term product vision. Read this for where the system is trying to go.
- [`high_level_technical_architecture_v0.2.md`](./high_level_technical_architecture_v0.2.md)
  High-level system design. Good for boundaries and component thinking.
- [`roadmap_and_release_shape_v0.1.md`](./roadmap_and_release_shape_v0.1.md)
  Early sequencing logic. Useful for why the repo grew in this order.
- [`diagrams.md`](../diagrams.md)
  Visual map of the write path, read surfaces, family-analysis lane, and planning boundary.

## Semantic Family And Recommendation Docs

- [`semantic-families/README.md`](../semantic-families/README.md)
  Packet rules, promotion workflow boundaries, and corpus-analysis context.
- [`semantic_family_capability_corpus_guide_v0.1.md`](./semantic_family_capability_corpus_guide_v0.1.md)
  Deeper explanation of family capability, coverage, and recommendation surfaces.
- [`recommendation_corpus_expansion_program_v0.1.md`](./recommendation_corpus_expansion_program_v0.1.md)
  Corpus growth and recommendation-pressure design context.
- [`ai_promotion_and_multilanguage_milestones_v0.1.md`](./ai_promotion_and_multilanguage_milestones_v0.1.md)
  Milestone framing for family promotion and second-language work.

## Historical Design Packets

These are useful when you are reconstructing why a milestone exists or how a
specific bounded wedge was scoped. They are not the right starting point for a
new reader.

- `docs/m26_*`
- `docs/m27_5_*`
- `docs/m52_*`

Some of these older design packets preserve machine-local path references from
the original authoring session. Read them as historical design records unless
they are re-grounded by current authority docs above.
