use rust_decimal::Decimal;

/// Hand-written Rust baseline for the canonical `billing/discount_strategy` seam.
#[derive(Clone, Debug, PartialEq)]
pub enum DiscountStrategy {
    Declined,
    Percentage { rate: Decimal },
    FixedCredit { amount: Decimal },
}

impl DiscountStrategy {
    pub fn discount_amount(&self, subtotal: Decimal) -> Decimal {
        match self {
            Self::Declined => Decimal::ZERO,
            Self::Percentage { rate } => subtotal * *rate,
            Self::FixedCredit { amount } => (*amount).min(subtotal),
        }
    }

    pub fn discounted_subtotal(&self, subtotal: Decimal) -> Decimal {
        subtotal - self.discount_amount(subtotal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_percentage_discount_without_generated_helpers() {
        let policy = DiscountStrategy::Percentage {
            rate: Decimal::new(10, 2),
        };

        assert_eq!(
            policy.discount_amount(Decimal::new(10000, 2)),
            Decimal::new(1000, 2)
        );
        assert_eq!(
            policy.discounted_subtotal(Decimal::new(10000, 2)),
            Decimal::new(9000, 2)
        );
    }

    #[test]
    fn caps_fixed_credit_at_subtotal_without_generated_helpers() {
        let policy = DiscountStrategy::FixedCredit {
            amount: Decimal::new(2000, 2),
        };

        assert_eq!(
            policy.discount_amount(Decimal::new(1500, 2)),
            Decimal::new(1500, 2)
        );
        assert_eq!(
            policy.discounted_subtotal(Decimal::new(1500, 2)),
            Decimal::ZERO
        );
    }
}
