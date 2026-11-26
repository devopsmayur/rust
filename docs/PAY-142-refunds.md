# PAY-142: Support partial refunds

**As a** support agent **I want** to refund part of a charge **so that** customers get money back quickly.

Acceptance criteria
- Refund up to the original charge amount.
- Customer sees the refund within 5 business days.

Note what is NOT written here: integer money, idempotency, PII-safe logging, JPY handling,
contract limits. That knowledge lives in AGENTS.md, path instructions, and learnings.
