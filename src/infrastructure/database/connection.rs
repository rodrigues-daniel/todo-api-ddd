use anyhow::{Context, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};

/// Cria pool de conexões com PostgreSQL
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Falha ao conectar ao banco de dados")
}
