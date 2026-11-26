use crate::{fees::platform_fee_minor, redact::redact_id};
use payments_contracts::{validate_charge, ChargeRequest, ContractError};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChargeResponse {
    pub charge_id: String,
    pub fee_minor: i64,
}

#[derive(Default)]
pub struct ChargeService {
    seen: HashMap<String, ChargeResponse>,
}

impl ChargeService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Idempotent per contract: same key => same response.
    pub fn charge(&mut self, req: &ChargeRequest) -> Result<ChargeResponse, ContractError> {
        validate_charge(req)?;
        if let Some(prev) = self.seen.get(&req.idempotency_key) {
            return Ok(prev.clone());
        }
        let resp = ChargeResponse {
            charge_id: format!("ch_{}", self.seen.len() + 1),
            fee_minor: platform_fee_minor(&req.amount),
        };
        tracing::info!(customer = %redact_id(&req.customer_id), charge = %resp.charge_id, "charge created");
        self.seen.insert(req.idempotency_key.clone(), resp.clone());
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payments_contracts::{Currency, Money};

    fn req(key: &str) -> ChargeRequest {
        ChargeRequest {
            idempotency_key: key.into(),
            customer_id: "cus_123456".into(),
            amount: Money { amount_minor: 5_000, currency: Currency::USD },
        }
    }

    #[test]
    fn replay_returns_same_charge() {
        let mut svc = ChargeService::new();
        let a = svc.charge(&req("k1")).unwrap();
        let b = svc.charge(&req("k1")).unwrap();
        assert_eq!(a, b);
    }
}
