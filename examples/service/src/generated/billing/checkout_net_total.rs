use rust_decimal::Decimal;

use crate::billing::apply_membership_discount::apply_membership_discount;
use crate::billing::apply_regional_fee::apply_regional_fee;

/// Return the net checkout total after discounting the subtotal and then applying the regional fee.
pub fn checkout_net_total(subtotal: Decimal, membership_rate: Decimal, regional_rate: Decimal) -> Decimal {
    let discounted = apply_membership_discount(subtotal, membership_rate);
    apply_regional_fee(discounted, regional_rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkout_net_total_basic() {
        assert!(checkout_net_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(5, 2)) == Decimal::new(9450, 2));
    }
}
