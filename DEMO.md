# CodeRabbit demo: Rust, GitHub, cross-repo context

**Story in one line:** a "simple" refunds ticket (PAY-142) ships code that compiles and passes
tests, but breaks rules the ticket never mentions. CodeRabbit catches them because it has
(1) cross-repo context, (2) path instructions + guidelines, (3) learnings, and leaves (4) a review record.

## Repos
| Repo | Role |
|---|---|
| `devopsmayur/rust` | `payments-service` (consumer) |
| `devopsmayur/rust-shared-contracts` | `payments-contracts` crate (source of truth) |

Each has `.coderabbit.yaml` with `knowledge_base.linked_repositories` pointing at the other.

## Setup (once)
1. Install the CodeRabbit GitHub App on **both** repos (same installation).
2. Run `./push.sh` (see below). Contracts repo is pushed first and tagged `v0.1.0`.
3. Open the two demo PRs (branches are pre-pushed):
   - **rust** PR: `feature/refund-endpoint` -> `main`, title `feat: partial refunds (PAY-142)`
   - **contracts** PR: `feature/rename-amount-field` -> `main`, title `breaking: rename amount_minor (PAY-150)`
4. Optional: create GitHub issue "PAY-142 Support partial refunds" with the story from
   `docs/PAY-142-refunds.md` and reference `Closes #1` in the PR for Linked Issue Assessment.

## Act 1: Rust PR (refunds) — what the ticket didn't say
Seeded defects in `src/refund.rs`; expected CodeRabbit findings:
| Defect | Why only context catches it |
|---|---|
| `amount_minor: f64`, `* 0.971` | Contract + AGENTS.md rule 1: integer minor units. Compiles fine. |
| `Option<String>` key, defaulted to `"none"` | Contract requires idempotency; all refunds collapse onto one key. |
| Local `100_000.0` limit | Contract `MAX_CHARGE_MINOR`; `validate_charge` bypassed. |
| `.parse().unwrap()` on charge id | AGENTS.md rule 4; panics in a payments path. |
| `println!("{:?}", original)` | PII in logs; rule 5 (use `tracing` + `redact_id`). |
| `Result<_, String>` | Contract has typed `ContractError`. |
| Clippy | Clippy integration surfaces lint-level issues alongside. |

Talk track: "Ticket says *refund up to the charge*. Nothing about floats, idempotency or logs.
That knowledge is in AGENTS.md and path instructions for `src/refund*.rs`, so the review applies it."

## Act 2: Cross-repo, the other direction
Contracts PR renames `Money.amount_minor` -> `amount_cents` with no semver bump, no CHANGELOG,
no serde alias. CodeRabbit (linked repo `devopsmayur/rust`) should name the consumer files that
break (`src/fees.rs`, `src/charge.rs`, tests) and request bump + changelog + alias.
Talk track: "The author of this PR never opened the service repo. The review did."

## Act 3: Learnings
In the rust PR, reply to a CodeRabbit comment:
```
@coderabbitai remember: refunds in this repo must also emit an audit event via tracing target "audit"; flag any refund path without it.
@coderabbitai remember: JPY is zero-decimal; any hard-coded * 100 or / 100 on Money is a bug.
```
Then push a small follow-up commit or open a second PR: the learning is applied and cited.
View/manage under *CodeRabbit dashboard -> Learnings* (`approval_delay` lets admins approve first;
`scope: local|global|auto` controls repo vs org reach).

## Act 4: The review record as evidence
Answer to "how do we prove this change was reviewed against our standards?":
- PR **walkthrough + summary** (what changed, sequence/file table)
- Inline comments tied to rules (AGENTS.md, path instructions, contract file refs)
- **Pre-merge checks** (title, description, linked issue assessment) as pass/warn on the PR
- Resolved/ignored threads with who overrode and why; commit history showing fixes
- Dashboard/reports for org-level trends (verify availability on your plan)
Show it: the PR conversation tab *is* the audit trail, attached to the commit SHA and the ticket.

## SaaS vs self-hosted — be ready
(Verify specifics against current docs before presenting.)
| Topic | SaaS (cloud) | Self-hosted |
|---|---|---|
| Ops | Zero infra, instant updates | You run the container, upgrades, scaling |
| Code residency | Code processed ephemerally in CodeRabbit's cloud | Stays inside your network/VPC |
| LLMs | Managed by CodeRabbit | Bring your own provider endpoint/keys |
| Git hosts | GitHub, GitLab, Bitbucket, Azure DevOps | Includes GHES / GitLab self-managed / Bitbucket DC |
| Plan | All tiers | Enterprise |
| Config | `.coderabbit.yaml`, linked repos, learnings identical in both | same |
Likely questions: data retention/training (no training on customer code per policy), SOC 2,
air-gapped, cross-repo access (the app/token must be able to read every linked repo in both models).

## Reset between runs
Close PRs, delete learnings you added, `git push` the branches again.
