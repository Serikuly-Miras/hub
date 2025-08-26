use crate::core::env::EnvVars;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    pub _diesel_pool: bb8::Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
    pub env: EnvVars,
}

impl AppState {
    pub async fn from_env() -> anyhow::Result<Self> {
        let env = EnvVars::from_env()?;

        // Diesel connection for raw queries
        tracing::debug!("🔌 establishing Diesel connection to database");
        let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
            &*env.database_url,
        );
        let _diesel_pool = Pool::builder()
            .connection_timeout(Duration::from_secs(env.db_aquire_timeout))
            .max_size(env.max_db_connections)
            .build(config)
            .await?;
        tracing::debug!("💾 successfully established diesel connection");

        Ok(Self { _diesel_pool, env })
    }
}
