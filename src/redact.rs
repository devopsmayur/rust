/// Mask an identifier for logs: keep prefix + last 2 chars.
pub fn redact_id(id: &str) -> String {
    if id.len() <= 6 {
        return "***".to_string();
    }
    format!("{}***{}", &id[..4], &id[id.len() - 2..])
}
