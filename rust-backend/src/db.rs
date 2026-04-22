//! PostgreSQL connection pool.
//!
//! **Invoice `metadata` (JSONB)** — today the API stores a small opaque object and does not
//! filter on it in SQL. Do not add JSONB indexes until a real `WHERE` / `ORDER BY` / `JOIN`
//! pattern lands in application code; see `../usdc-payment-link-tool/migrations/003_invoice_metadata_jsonb_index_plan.sql`
//! and the product README for the decision record and index-type cheat sheet.

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::Config as PgConfig;

use crate::config::Config;

pub fn create_pool(config: &Config) -> anyhow::Result<Pool> {
    let pg = config.database_url.parse::<PgConfig>()?;
    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(pg, tokio_postgres::NoTls, manager_config);
    Ok(Pool::builder(manager)
        .runtime(Runtime::Tokio1)
        .max_size(16)
        .build()?)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn invoice_metadata_plan_migration_documents_index_policy() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../usdc-payment-link-tool/migrations/003_invoice_metadata_jsonb_index_plan.sql");
        let sql =
            std::fs::read_to_string(path).expect("read 003_invoice_metadata_jsonb_index_plan.sql");
        assert!(
            sql.contains("COMMENT ON COLUMN invoices.metadata"),
            "plan should register a catalog comment for operators"
        );
        assert!(
            sql.contains("jsonb_path_ops") && sql.contains("GIN"),
            "plan should mention GIN operator class options when metadata is queried"
        );
        assert!(
            sql.contains("Policy: do not CREATE INDEX"),
            "plan should warn against speculative indexes"
        );
        for line in sql.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with("--") {
                continue;
            }
            assert!(
                !t.to_uppercase().starts_with("CREATE INDEX"),
                "003 must not create speculative metadata indexes: {t}"
            );
        }
    }
}
