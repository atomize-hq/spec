use rust_decimal::Decimal;

/// Hand-written Rust baseline for the canonical `pricing/pricing_quote` seam.
#[derive(Clone, Debug, PartialEq)]
pub struct PricingQuote {
    pub subtotal: Decimal,
    pub discount_rate: Decimal,
    pub tax_rate: Decimal,
}

impl PricingQuote {
    pub fn new(subtotal: Decimal, discount_rate: Decimal, tax_rate: Decimal) -> Self {
        Self {
            subtotal,
            discount_rate,
            tax_rate,
        }
    }

    pub fn discounted_subtotal(&self) -> Decimal {
        let discounted = self.subtotal - self.subtotal * self.discount_rate;
        discounted.max(Decimal::ZERO)
    }

    pub fn total(&self) -> Decimal {
        let discounted = self.discounted_subtotal();
        discounted + discounted * self.tax_rate
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
            Decimal::new(725, 4),
        );

        assert_eq!(quote.discounted_subtotal(), Decimal::new(9000, 2));
        assert_eq!(quote.total(), Decimal::new(96525, 3));
    }

    #[test]
    fn computes_rounding_sensitive_totals_without_generated_helpers() {
        let quote = PricingQuote::new(
            Decimal::new(1001, 2),
            Decimal::new(3333, 4),
            Decimal::new(725, 4),
        );

        assert_eq!(quote.discounted_subtotal(), Decimal::new(6673667, 6));
        assert_eq!(quote.total(), Decimal::new(71575078575, 10));
    }
}
