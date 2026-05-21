use rust_decimal::Decimal;

/// Hand-written Rust baseline for the canonical `billing/pricing_quote` seam.
#[derive(Clone, Debug, PartialEq)]
pub struct PricingQuote {
    pub subtotal: Decimal,
    pub membership_rate: Decimal,
    pub regional_rate: Decimal,
}

impl PricingQuote {
    pub fn new(subtotal: Decimal, membership_rate: Decimal, regional_rate: Decimal) -> Self {
        Self {
            subtotal,
            membership_rate,
            regional_rate,
        }
    }

    pub fn discounted_subtotal(&self) -> Decimal {
        (self.subtotal - self.subtotal * self.membership_rate).max(Decimal::ZERO)
    }

    pub fn total(&self) -> Decimal {
        let discounted = self.discounted_subtotal();
        discounted + discounted * self.regional_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_happy_path_totals_without_generated_helpers() {
        let quote = PricingQuote::new(
            Decimal::new(10000, 2),
            Decimal::new(10, 2),
            Decimal::new(5, 2),
        );

        assert_eq!(quote.discounted_subtotal(), Decimal::new(9000, 2));
        assert_eq!(quote.total(), Decimal::new(9450, 2));
    }

    #[test]
    fn computes_declined_discount_totals_without_generated_helpers() {
        let quote = PricingQuote::new(
            Decimal::new(10000, 2),
            Decimal::ZERO,
            Decimal::ZERO,
        );

        assert_eq!(quote.discounted_subtotal(), Decimal::new(10000, 2));
        assert_eq!(quote.total(), Decimal::new(10000, 2));
    }
}
