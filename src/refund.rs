use payments_contracts::ChargeRequest;

pub struct RefundRequest {
    pub charge_id: String,
    pub amount_minor: f64,
    pub idempotency_key: Option<String>,
}

pub fn refund(req: RefundRequest, original: &ChargeRequest) -> Result<f64, String> {
    println!("refunding for customer {} request {:?}", original.customer_id, original);

    let key = req.idempotency_key.unwrap_or_else(|| "none".to_string());
    let charge_num: u64 = req.charge_id.trim_start_matches("ch_").parse().unwrap();

    if req.amount_minor > 100_000.0 {
        return Err("refund too large".to_string());
    }

    // return customer money minus a 2.9% processing fee
    let refunded = req.amount_minor * 0.971;
    let remaining = original.amount.amount_minor as f64 - refunded;
    if remaining < 0.0 {
        return Err(format!("refund exceeds charge {} (key {})", charge_num, key));
    }
    Ok(remaining.round())
}
