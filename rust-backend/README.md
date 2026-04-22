# ASTROpay Rust Backend

This service is the beginning of the backend migration out of Next.js route handlers.

What it currently owns:

- merchant registration, login, logout, and cookie-backed sessions
- invoice creation, listing, detail lookup, and status lookup
- Horizon-backed reconciliation for pending invoices
- webhook-driven payment marking (`/api/webhooks/stellar`)
- a Rust migration runner that reuses the existing SQL migrations

What is intentionally not faked yet:

- buyer XDR generation/submission for checkout
- merchant settlement cron

Those routes return `501 Not Implemented` in the Rust service until the Stellar transaction logic is ported properly.

## Database migrations

SQL lives in `../usdc-payment-link-tool/migrations/`. Apply with `cargo run --bin migrate` from `rust-backend/`. The runner errors clearly if the migrations directory is missing or if a file’s SQL fails.

**Invoice `metadata` (JSONB):** migration `003_invoice_metadata_jsonb_index_plan.sql` records the indexing policy—no speculative GIN until real filter queries exist—and sets `COMMENT ON COLUMN invoices.metadata` for DB catalog visibility. See the Next.js README for the same guidance.

**Verification:** `cargo test` (includes a guard that 003 stays comment/plan-only without `CREATE INDEX`).

## Run locally

```bash
cd rust-backend
cargo run --bin migrate
cargo run
```

The service reads env vars from:

- `rust-backend/.env.local`
- `rust-backend/.env`
- `../usdc-payment-link-tool/.env.local`
- `../usdc-payment-link-tool/.env`
