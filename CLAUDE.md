## Skill routing

When the user's request matches an available skill, ALWAYS invoke it using the Skill
tool as your FIRST action. Do NOT answer directly, do NOT use other tools first.
The skill has specialized workflows that produce better results than ad-hoc answers.

Key routing rules:
- Product ideas, "is this worth building", brainstorming → invoke office-hours
- Bugs, errors, "why is this broken", 500 errors → invoke investigate
- Ship, deploy, push, create PR → invoke ship
- QA, test the site, find bugs → invoke qa
- Code review, check my diff → invoke review
- Update docs after shipping → invoke document-release
- Weekly retro → invoke retro
- Design system, brand → invoke design-consultation
- Visual audit, design polish → invoke design-review
- Architecture review → invoke plan-eng-review

## M20 semantic review note

For unsupported-function semantic review, branch on `semantic_review.support_status == "unsupported"`. The public M20 fields are `semantic_review.support_status`, `semantic_review.unsupported_reason_codes`, and `semantic_review.rewrite_hints`; legacy reviews without `support_status` still fall back to `evaluator_scope` plus `unsupported.*.v1` inference.
