use crate::billing::apply_membership_discount::apply_membership_discount;
use crate::billing::apply_regional_fee::apply_regional_fee;

/// Quote a service checkout total from subtotal plus membership discount and regional fee rates.
#[derive(Clone, Debug, PartialEq)]
pub struct PricingQuote {
    pub subtotal: rust_decimal::Decimal,
    pub membership_rate: rust_decimal::Decimal,
    pub regional_rate: rust_decimal::Decimal,
}

impl PricingQuote {
    /// Create a quote from explicit subtotal and rates.
    pub fn new(subtotal: rust_decimal::Decimal, membership_rate: rust_decimal::Decimal, regional_rate: rust_decimal::Decimal) -> Self {
        Self {
            subtotal: subtotal,
            membership_rate: membership_rate,
            regional_rate: regional_rate,
        }
    }

    /// Return the subtotal after the membership discount and before the regional fee.
    pub fn discounted_subtotal(&self) -> rust_decimal::Decimal {
    apply_membership_discount(self.subtotal, self.membership_rate)
}

    /// Return the final checkout total after the membership discount and regional fee.
    pub fn total(&self) -> rust_decimal::Decimal {
    apply_regional_fee(self.discounted_subtotal(), self.regional_rate)
}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discounted_subtotal_basic() {
        assert!(PricingQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(5, 2)).discounted_subtotal() == rust_decimal::Decimal::new(9000, 2));
    }

    #[test]
    fn test_total_basic() {
        assert!(PricingQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(5, 2)).total() == rust_decimal::Decimal::new(9450, 2));
    }
}
