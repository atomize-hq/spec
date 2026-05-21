use rust_decimal::Decimal;

/// Return the subtotal after applying the membership discount rate and clamping at zero.
pub fn apply_membership_discount(subtotal: Decimal, rate: Decimal) -> Decimal {
    (subtotal - subtotal * rate).max(Decimal::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_membership_discount_basic() {
        assert!(apply_membership_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2));
    }
}
