use payments_contracts::Money;

/// Platform fee: 2.9% in basis points, integer math, rounded half-up.
pub const PLATFORM_FEE_BPS: i64 = 290;

pub fn platform_fee_minor(amount: &Money) -> i64 {
    (amount.amount_minor * PLATFORM_FEE_BPS + 5_000) / 10_000
}

#[cfg(test)]
mod tests {
    use super::*;
    use payments_contracts::Currency;

    #[test]
    fn fee_rounds_half_up() {
        let m = Money { amount_minor: 1_000, currency: Currency::USD };
        assert_eq!(platform_fee_minor(&m), 29);
    }
}
