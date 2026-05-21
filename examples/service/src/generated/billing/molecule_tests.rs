use rust_decimal::Decimal;
use crate::billing::checkout_net_total_guarded_fee::checkout_net_total_guarded_fee;
use crate::billing::pricing_quote::PricingQuote;
use crate::billing::apply_membership_discount::apply_membership_discount;
use crate::billing::apply_regional_fee::apply_regional_fee;
use crate::billing::checkout_net_total::checkout_net_total;

#[test]
fn test_checkout_declined_discount_flow() {
    let subtotal = Decimal::new(10000, 2);
    let declined = crate::billing::discount_strategy::DiscountStrategy::Declined;
    let quote = PricingQuote::new(subtotal, Decimal::ZERO, Decimal::ZERO);
    let guarded_total =
        checkout_net_total_guarded_fee(subtotal, Decimal::ZERO, Decimal::new(-5, 2));

    assert_eq!(declined.discount_amount(subtotal), Decimal::ZERO);
    assert_eq!(declined.discounted_subtotal(subtotal), subtotal);
    assert_eq!(quote.discounted_subtotal(), subtotal);
    assert_eq!(quote.total(), subtotal);
    assert_eq!(guarded_total, subtotal);
    assert_eq!(guarded_total, quote.total());
}

#[test]
fn test_checkout_success_flow() {
    let quote = PricingQuote::new(
        Decimal::new(10000, 2),
        Decimal::new(10, 2),
        Decimal::new(5, 2),
    );
    let total = checkout_net_total(
        Decimal::new(10000, 2),
        Decimal::new(10, 2),
        Decimal::new(5, 2),
    );

    assert_eq!(
        quote.discounted_subtotal(),
        apply_membership_discount(Decimal::new(10000, 2), Decimal::new(10, 2))
    );
    assert_eq!(quote.total(), total);
    assert_eq!(
        apply_regional_fee(quote.discounted_subtotal(), Decimal::new(5, 2)),
        total
    );
    assert!(quote.total() > quote.discounted_subtotal());
}

#[test]
fn test_discount_strategy_quote_flow() {
    let subtotal = Decimal::new(10000, 2);
    let raw_declined = crate::raw_baseline::billing::discount_strategy::DiscountStrategy::Declined;
    let generated_declined = crate::billing::discount_strategy::DiscountStrategy::Declined;
    assert_eq!(raw_declined.discount_amount(subtotal), generated_declined.discount_amount(subtotal));
    assert_eq!(
        raw_declined.discounted_subtotal(subtotal),
        generated_declined.discounted_subtotal(subtotal)
    );

    let raw_percentage =
        crate::raw_baseline::billing::discount_strategy::DiscountStrategy::Percentage {
            rate: Decimal::new(10, 2),
        };
    let generated_percentage =
        crate::billing::discount_strategy::DiscountStrategy::Percentage {
            rate: Decimal::new(10, 2),
        };
    assert_eq!(
        raw_percentage.discount_amount(subtotal),
        generated_percentage.discount_amount(subtotal)
    );
    assert_eq!(
        raw_percentage.discounted_subtotal(subtotal),
        generated_percentage.discounted_subtotal(subtotal)
    );

    let raw_fixed =
        crate::raw_baseline::billing::discount_strategy::DiscountStrategy::FixedCredit {
            amount: Decimal::new(1250, 2),
        };
    let generated_fixed =
        crate::billing::discount_strategy::DiscountStrategy::FixedCredit {
            amount: Decimal::new(1250, 2),
        };
    assert_eq!(
        raw_fixed.discount_amount(Decimal::new(5000, 2)),
        generated_fixed.discount_amount(Decimal::new(5000, 2))
    );
    assert_eq!(
        raw_fixed.discounted_subtotal(Decimal::new(5000, 2)),
        generated_fixed.discounted_subtotal(Decimal::new(5000, 2))
    );

    let quote = PricingQuote::new(
        subtotal,
        Decimal::new(10, 2),
        Decimal::new(5, 2),
    );
    let percentage_discounted = generated_percentage.discounted_subtotal(subtotal);

    assert_eq!(percentage_discounted, Decimal::new(9000, 2));
    assert_eq!(percentage_discounted, quote.discounted_subtotal());
    assert_eq!(apply_regional_fee(percentage_discounted, Decimal::new(5, 2)), quote.total());
    assert_eq!(
        generated_fixed.discounted_subtotal(Decimal::new(5000, 2)),
        Decimal::new(3750, 2)
    );
}
