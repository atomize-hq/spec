/// Represent mutually exclusive membership discount strategies for a service checkout flow.
#[derive(Clone, Debug, PartialEq)]
pub enum DiscountStrategy {
    Declined,
    Percentage {
        rate: rust_decimal::Decimal,
    },
    FixedCredit {
        amount: rust_decimal::Decimal,
    },
}

impl DiscountStrategy {
    /// Return the discount amount to subtract from the subtotal.
    pub fn discount_amount(&self, subtotal: rust_decimal::Decimal) -> rust_decimal::Decimal {
    match self {
        Self::Declined => rust_decimal::Decimal::ZERO,
        Self::Percentage { rate } => subtotal * *rate,
        Self::FixedCredit { amount } => (*amount).min(subtotal),
    }
}

    /// Return the subtotal after applying the selected discount strategy.
    pub fn discounted_subtotal(&self, subtotal: rust_decimal::Decimal) -> rust_decimal::Decimal {
    subtotal - self.discount_amount(subtotal)
}

    /// Support direct atom proof for the canonical declined-discount example.
    pub fn declined_example_holds(&self) -> bool {
    let policy = Self::Declined;
    policy.discount_amount(rust_decimal::Decimal::new(10000, 2))
        == rust_decimal::Decimal::ZERO
        && policy.discounted_subtotal(rust_decimal::Decimal::new(10000, 2))
            == rust_decimal::Decimal::new(10000, 2)
}

    /// Support direct atom proof for the canonical percentage discount example.
    pub fn percentage_example_holds(&self) -> bool {
    let policy = Self::Percentage {
        rate: rust_decimal::Decimal::new(10, 2),
    };
    policy.discount_amount(rust_decimal::Decimal::new(10000, 2))
        == rust_decimal::Decimal::new(1000, 2)
        && policy.discounted_subtotal(rust_decimal::Decimal::new(10000, 2))
            == rust_decimal::Decimal::new(9000, 2)
}

    /// Support direct atom proof for capped fixed-credit behavior.
    pub fn fixed_credit_capped_behavior_holds(&self) -> bool {
    let policy = Self::FixedCredit {
        amount: rust_decimal::Decimal::new(2000, 2),
    };
    policy.discount_amount(rust_decimal::Decimal::new(1500, 2))
        == rust_decimal::Decimal::new(1500, 2)
        && policy.discounted_subtotal(rust_decimal::Decimal::new(1500, 2))
            == rust_decimal::Decimal::ZERO
}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_declined() {
        assert!(DiscountStrategy::Declined.declined_example_holds());
    }

    #[test]
    fn test_variant_percentage() {
        assert!(DiscountStrategy::Declined.percentage_example_holds());
    }

    #[test]
    fn test_behavior_fixed_credit_capped() {
        assert!(DiscountStrategy::Declined.fixed_credit_capped_behavior_holds());
    }
}
