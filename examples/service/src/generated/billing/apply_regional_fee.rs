use rust_decimal::Decimal;

/// Return the discounted subtotal after applying the regional fee rate.
pub fn apply_regional_fee(subtotal: Decimal, rate: Decimal) -> Decimal {
    subtotal + subtotal * rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regional_fee_basic() {
        assert!(apply_regional_fee(Decimal::new(10000, 2), Decimal::new(5, 2)) == Decimal::new(10500, 2));
    }
}
