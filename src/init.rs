use std::{str::FromStr, time::Duration};

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up Logging")
}

pub async fn database_connection() -> PgPool {
    tracing::debug!("Setting up database connection");
    let db_url = dotenvy::var("DATABASE_URL").expect("Failed to read DATABASE_URL");

    let options = PgConnectOptions::from_str(&db_url).expect("Failed to parse url");

    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("Failed to connect to database");

    tracing::debug!("Successfully connected");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to migrate");

    tracing::debug!("Successfully migrated");

    pg_pool
}
