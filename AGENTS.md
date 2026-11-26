# Engineering rules for payments-service

These are the rules a user story never states. CodeRabbit reads this file automatically.

1. Money is `i64` minor units from `payments-contracts`. Never `f32`/`f64`, never ad-hoc rounding.
2. Every mutating operation requires an idempotency key (contract: `IDEMPOTENCY_HEADER`).
   Do not default, generate, or skip it.
3. Always call `payments_contracts::validate_charge` — never re-implement limits locally.
4. No `unwrap()`/`expect()` in non-test code; return typed errors.
5. Never log raw customer ids, cards, or full request structs. Use `tracing` + `redact_id`.
6. Zero-decimal currencies (JPY) must be handled via `Currency::minor_unit_exponent`.
