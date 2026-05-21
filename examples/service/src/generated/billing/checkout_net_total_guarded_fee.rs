use rust_decimal::Decimal;

use crate::billing::apply_membership_discount::apply_membership_discount;
use crate::billing::apply_regional_fee::apply_regional_fee;

/// Return the net checkout total after discounting the subtotal and applying a nonnegative regional fee only.
pub fn checkout_net_total_guarded_fee(subtotal: Decimal, membership_rate: Decimal, regional_rate: Decimal) -> Decimal {
    let discounted = apply_membership_discount(subtotal, membership_rate);
    apply_regional_fee(discounted, regional_rate.max(Decimal::ZERO))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkout_net_total_guarded_fee_clamps_negative_rates() {
        assert!(checkout_net_total_guarded_fee(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(-5, 2)) == Decimal::new(9000, 2));
    }
}
