use crate::semantic_review::{SemanticReview, SemanticSupportStatus};
use serde::{Deserialize, Serialize};

pub const CATEGORY_TRUTH_SCHEMA_VERSION: u8 = 1;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct CategoryTruthRegistry {
    pub schema_version: u8,
    pub categories: &'static [CategoryTruthRow],
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct CategoryTruthRow {
    pub category_id: &'static str,
    pub kind: CategoryKind,
    pub contract_support_status: ContractSupportStatus,
    pub alias_sibling_policy: AliasSiblingPolicy,
    pub descriptor_set: DescriptorSet,
    pub positive_credit_policy: PositiveCreditPolicy,
    pub notes: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CategoryKind {
    Sum,
    Data,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractSupportStatus {
    Supported,
    Unsupported,
}

impl From<ContractSupportStatus> for SemanticSupportStatus {
    fn from(value: ContractSupportStatus) -> Self {
        match value {
            ContractSupportStatus::Supported => SemanticSupportStatus::Supported,
            ContractSupportStatus::Unsupported => SemanticSupportStatus::Unsupported,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AliasSiblingPolicy {
    CanonicalOnly,
    UnsupportedTerminal,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct DescriptorIdentity {
    pub descriptor_id: &'static str,
    pub representative_unit_id: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct DescriptorSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_descriptor: Option<DescriptorIdentity>,
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub approved_siblings: &'static [DescriptorIdentity],
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct PositiveCreditPolicy {
    pub eligible: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerKind {
    Benchmark,
    Status,
    Export,
    Snapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CategoryQualification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor_id: Option<String>,
    pub claim_status: ClaimStatus,
    pub positive_credit_eligibility: PositiveCreditEligibility,
    pub reason_code: QualificationReasonCode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClaimStatus {
    SupportedQualified,
    UnsupportedQualified,
    Unqualified,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PositiveCreditEligibility {
    Eligible,
    Ineligible,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QualificationReasonCode {
    Qualified,
    SemanticReviewMissing,
    RegistryRowMissing,
    DescriptorIdMissing,
    DescriptorNotApproved,
    SupportStatusMismatch,
    PositiveCreditDisallowed,
}

const EMPTY_DESCRIPTOR_SET: DescriptorSet = DescriptorSet {
    canonical_descriptor: None,
    approved_siblings: &[],
};

const DISCOUNT_STRATEGY_ECOMMERCE_DESCRIPTOR: DescriptorIdentity = DescriptorIdentity {
    descriptor_id: "discount_strategy.ecommerce.v1",
    representative_unit_id: "pricing/discount_strategy",
};

const PRICING_QUOTE_ECOMMERCE_DESCRIPTOR: DescriptorIdentity = DescriptorIdentity {
    descriptor_id: "pricing_quote.ecommerce.v1",
    representative_unit_id: "pricing/pricing_quote",
};

const CATEGORY_ROWS: [CategoryTruthRow; 4] = [
    CategoryTruthRow {
        category_id: "sum.discount_strategy.v1",
        kind: CategoryKind::Sum,
        contract_support_status: ContractSupportStatus::Supported,
        alias_sibling_policy: AliasSiblingPolicy::CanonicalOnly,
        descriptor_set: DescriptorSet {
            canonical_descriptor: Some(DISCOUNT_STRATEGY_ECOMMERCE_DESCRIPTOR),
            approved_siblings: &[],
        },
        positive_credit_policy: PositiveCreditPolicy { eligible: true },
        notes: "canonical ecommerce descriptor only; service sibling remains visible but unqualified until producer truth is widened explicitly",
    },
    CategoryTruthRow {
        category_id: "data.pricing_quote.v1",
        kind: CategoryKind::Data,
        contract_support_status: ContractSupportStatus::Supported,
        alias_sibling_policy: AliasSiblingPolicy::CanonicalOnly,
        descriptor_set: DescriptorSet {
            canonical_descriptor: Some(PRICING_QUOTE_ECOMMERCE_DESCRIPTOR),
            approved_siblings: &[],
        },
        positive_credit_policy: PositiveCreditPolicy { eligible: true },
        notes: "canonical ecommerce descriptor only; service sibling remains visible but unqualified until producer truth is widened explicitly",
    },
    CategoryTruthRow {
        category_id: "unsupported.sum.v1",
        kind: CategoryKind::Sum,
        contract_support_status: ContractSupportStatus::Unsupported,
        alias_sibling_policy: AliasSiblingPolicy::UnsupportedTerminal,
        descriptor_set: EMPTY_DESCRIPTOR_SET,
        positive_credit_policy: PositiveCreditPolicy { eligible: false },
        notes: "unsupported seam sum truth remains visible, additive, and never positive-credit eligible",
    },
    CategoryTruthRow {
        category_id: "unsupported.data.v1",
        kind: CategoryKind::Data,
        contract_support_status: ContractSupportStatus::Unsupported,
        alias_sibling_policy: AliasSiblingPolicy::UnsupportedTerminal,
        descriptor_set: EMPTY_DESCRIPTOR_SET,
        positive_credit_policy: PositiveCreditPolicy { eligible: false },
        notes: "unsupported seam data truth remains visible, additive, and never positive-credit eligible",
    },
];

const CATEGORY_TRUTH_REGISTRY: CategoryTruthRegistry = CategoryTruthRegistry {
    schema_version: CATEGORY_TRUTH_SCHEMA_VERSION,
    categories: &CATEGORY_ROWS,
};

pub fn category_truth_registry() -> &'static CategoryTruthRegistry {
    &CATEGORY_TRUTH_REGISTRY
}

impl CategoryTruthRegistry {
    pub fn find_by_category_id(&self, category_id: &str) -> Option<&'static CategoryTruthRow> {
        self.categories
            .iter()
            .find(|row| row.category_id == category_id)
    }
}

pub fn is_seam_category_claim_candidate(review: &SemanticReview) -> bool {
    matches!(
        review.compatibility_key.as_str(),
        "unsupported.sum.v1" | "unsupported.data.v1"
    ) || review.compatibility_key.starts_with("sum.")
        || review.compatibility_key.starts_with("data.")
}

pub fn qualify_category_claim(
    _consumer: ConsumerKind,
    semantic_review: Option<&SemanticReview>,
    _unit_id: Option<&str>,
) -> CategoryQualification {
    let Some(review) = semantic_review else {
        return CategoryQualification {
            category_id: None,
            descriptor_id: None,
            claim_status: ClaimStatus::Unqualified,
            positive_credit_eligibility: PositiveCreditEligibility::Ineligible,
            reason_code: QualificationReasonCode::SemanticReviewMissing,
        };
    };

    let descriptor_id = review.descriptor_id.clone();
    let Some(row) = category_truth_registry().find_by_category_id(&review.compatibility_key) else {
        return CategoryQualification {
            category_id: None,
            descriptor_id,
            claim_status: ClaimStatus::Unqualified,
            positive_credit_eligibility: PositiveCreditEligibility::Ineligible,
            reason_code: QualificationReasonCode::RegistryRowMissing,
        };
    };

    if review.effective_support_status() != row.contract_support_status.into() {
        return CategoryQualification {
            category_id: Some(row.category_id.to_string()),
            descriptor_id,
            claim_status: ClaimStatus::Unqualified,
            positive_credit_eligibility: PositiveCreditEligibility::Ineligible,
            reason_code: QualificationReasonCode::SupportStatusMismatch,
        };
    }

    if matches!(
        row.contract_support_status,
        ContractSupportStatus::Supported
    ) {
        let Some(descriptor_id_value) = descriptor_id.clone() else {
            return CategoryQualification {
                category_id: Some(row.category_id.to_string()),
                descriptor_id: None,
                claim_status: ClaimStatus::Unqualified,
                positive_credit_eligibility: PositiveCreditEligibility::Ineligible,
                reason_code: QualificationReasonCode::DescriptorIdMissing,
            };
        };

        if !descriptor_is_approved(&row.descriptor_set, &descriptor_id_value) {
            return CategoryQualification {
                category_id: Some(row.category_id.to_string()),
                descriptor_id: Some(descriptor_id_value),
                claim_status: ClaimStatus::Unqualified,
                positive_credit_eligibility: PositiveCreditEligibility::Ineligible,
                reason_code: QualificationReasonCode::DescriptorNotApproved,
            };
        }

        return CategoryQualification {
            category_id: Some(row.category_id.to_string()),
            descriptor_id: Some(descriptor_id_value),
            claim_status: ClaimStatus::SupportedQualified,
            positive_credit_eligibility: PositiveCreditEligibility::Eligible,
            reason_code: QualificationReasonCode::Qualified,
        };
    }

    CategoryQualification {
        category_id: Some(row.category_id.to_string()),
        descriptor_id,
        claim_status: ClaimStatus::UnsupportedQualified,
        positive_credit_eligibility: PositiveCreditEligibility::Ineligible,
        reason_code: QualificationReasonCode::PositiveCreditDisallowed,
    }
}

fn descriptor_is_approved(descriptor_set: &DescriptorSet, descriptor_id: &str) -> bool {
    descriptor_set
        .canonical_descriptor
        .as_ref()
        .is_some_and(|descriptor| descriptor.descriptor_id == descriptor_id)
        || descriptor_set
            .approved_siblings
            .iter()
            .any(|descriptor| descriptor.descriptor_id == descriptor_id)
}

pub fn is_first_scope_seam_unit_id(unit_id: &str) -> bool {
    matches!(
        unit_id,
        "pricing/discount_strategy"
            | "billing/discount_strategy"
            | "pricing/pricing_quote"
            | "billing/pricing_quote"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_review::{
        EvaluatorScope, SemanticReview, SemanticSupportStatus, SemanticVerdict,
    };

    fn review(
        compatibility_key: &str,
        descriptor_id: Option<&str>,
        support_status: Option<SemanticSupportStatus>,
        evaluator_scope: EvaluatorScope,
    ) -> SemanticReview {
        SemanticReview {
            verdict: SemanticVerdict::Aligned,
            compatibility_key: compatibility_key.to_string(),
            descriptor_id: descriptor_id.map(str::to_string),
            support_status,
            unsupported_reason_codes: vec![],
            rewrite_hints: vec![],
            reason_codes: vec![],
            summary: String::new(),
            authored_surfaces: vec![],
            executable_surfaces: vec![],
            evaluator_scope,
        }
    }

    #[test]
    fn registry_exposes_first_scope_rows() {
        let registry = category_truth_registry();
        assert_eq!(registry.schema_version, CATEGORY_TRUTH_SCHEMA_VERSION);
        assert_eq!(registry.categories.len(), 4);
        assert!(
            registry
                .find_by_category_id("sum.discount_strategy.v1")
                .is_some()
        );
        assert!(
            registry
                .find_by_category_id("data.pricing_quote.v1")
                .is_some()
        );
        assert!(registry.find_by_category_id("unsupported.sum.v1").is_some());
        assert!(
            registry
                .find_by_category_id("unsupported.data.v1")
                .is_some()
        );
    }

    #[test]
    fn canonical_ecommerce_sum_qualifies_as_supported() {
        let qualification = qualify_category_claim(
            ConsumerKind::Benchmark,
            Some(&review(
                "sum.discount_strategy.v1",
                Some("discount_strategy.ecommerce.v1"),
                None,
                EvaluatorScope::SupportedSumSurface,
            )),
            Some("pricing/discount_strategy"),
        );

        assert_eq!(qualification.claim_status, ClaimStatus::SupportedQualified);
        assert_eq!(
            qualification.positive_credit_eligibility,
            PositiveCreditEligibility::Eligible
        );
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::Qualified
        );
    }

    #[test]
    fn canonical_ecommerce_data_qualifies_as_supported() {
        let qualification = qualify_category_claim(
            ConsumerKind::Status,
            Some(&review(
                "data.pricing_quote.v1",
                Some("pricing_quote.ecommerce.v1"),
                None,
                EvaluatorScope::SupportedDataSurface,
            )),
            Some("pricing/pricing_quote"),
        );

        assert_eq!(qualification.claim_status, ClaimStatus::SupportedQualified);
        assert_eq!(
            qualification.positive_credit_eligibility,
            PositiveCreditEligibility::Eligible
        );
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::Qualified
        );
    }

    #[test]
    fn service_sum_sibling_stays_visible_but_unqualified() {
        let qualification = qualify_category_claim(
            ConsumerKind::Export,
            Some(&review(
                "sum.discount_strategy.v1",
                Some("discount_strategy.service.v1"),
                None,
                EvaluatorScope::SupportedSumSurface,
            )),
            Some("billing/discount_strategy"),
        );

        assert_eq!(
            qualification.category_id.as_deref(),
            Some("sum.discount_strategy.v1")
        );
        assert_eq!(qualification.claim_status, ClaimStatus::Unqualified);
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::DescriptorNotApproved
        );
    }

    #[test]
    fn service_data_sibling_stays_visible_but_unqualified() {
        let qualification = qualify_category_claim(
            ConsumerKind::Snapshot,
            Some(&review(
                "data.pricing_quote.v1",
                Some("pricing_quote.service.v1"),
                None,
                EvaluatorScope::SupportedDataSurface,
            )),
            Some("billing/pricing_quote"),
        );

        assert_eq!(
            qualification.category_id.as_deref(),
            Some("data.pricing_quote.v1")
        );
        assert_eq!(qualification.claim_status, ClaimStatus::Unqualified);
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::DescriptorNotApproved
        );
    }

    #[test]
    fn unsupported_sum_row_qualifies_only_as_unsupported() {
        let qualification = qualify_category_claim(
            ConsumerKind::Benchmark,
            Some(&review(
                "unsupported.sum.v1",
                None,
                None,
                EvaluatorScope::UnsupportedSurface,
            )),
            Some("billing/discount_strategy"),
        );

        assert_eq!(
            qualification.claim_status,
            ClaimStatus::UnsupportedQualified
        );
        assert_eq!(
            qualification.positive_credit_eligibility,
            PositiveCreditEligibility::Ineligible
        );
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::PositiveCreditDisallowed
        );
    }

    #[test]
    fn unsupported_data_row_qualifies_only_as_unsupported() {
        let qualification = qualify_category_claim(
            ConsumerKind::Benchmark,
            Some(&review(
                "unsupported.data.v1",
                None,
                None,
                EvaluatorScope::UnsupportedSurface,
            )),
            Some("billing/pricing_quote"),
        );

        assert_eq!(
            qualification.claim_status,
            ClaimStatus::UnsupportedQualified
        );
        assert_eq!(
            qualification.positive_credit_eligibility,
            PositiveCreditEligibility::Ineligible
        );
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::PositiveCreditDisallowed
        );
    }

    #[test]
    fn missing_semantic_review_is_explicit_failure() {
        let qualification = qualify_category_claim(ConsumerKind::Status, None, None);

        assert_eq!(qualification.claim_status, ClaimStatus::Unqualified);
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::SemanticReviewMissing
        );
    }

    #[test]
    fn missing_descriptor_id_is_explicit_failure_for_supported_rows() {
        let qualification = qualify_category_claim(
            ConsumerKind::Export,
            Some(&review(
                "sum.discount_strategy.v1",
                None,
                None,
                EvaluatorScope::SupportedSumSurface,
            )),
            Some("pricing/discount_strategy"),
        );

        assert_eq!(qualification.claim_status, ClaimStatus::Unqualified);
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::DescriptorIdMissing
        );
    }

    #[test]
    fn unknown_seam_registry_row_fails_explicitly() {
        let qualification = qualify_category_claim(
            ConsumerKind::Status,
            Some(&review(
                "sum.unknown.v1",
                Some("sum.unknown.ecommerce.v1"),
                None,
                EvaluatorScope::SupportedSumSurface,
            )),
            Some("pricing/discount_strategy"),
        );

        assert_eq!(qualification.claim_status, ClaimStatus::Unqualified);
        assert_eq!(
            qualification.reason_code,
            QualificationReasonCode::RegistryRowMissing
        );
    }
}
