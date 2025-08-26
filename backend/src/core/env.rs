use anyhow::bail;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct EnvVars {
    pub database_url: Cow<'static, str>,
    pub port: u16,
    pub max_db_connections: u32,
    pub db_aquire_timeout: u64,
}

impl EnvVars {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: match dotenvy::var("DATABASE_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing DATABASE_URL: {err}"),
            },
            port: match dotenvy::var("PORT") {
                Ok(port) => port.parse()?,
                _ => 8000,
            },
            max_db_connections: match dotenvy::var("MAX_DB_CONNECTIONS") {
                Ok(max) => max.parse()?,
                _ => 3,
            },
            db_aquire_timeout: match dotenvy::var("DB_ACQUIRE_TIMEOUT") {
                Ok(timeout) => timeout.parse()?,
                _ => 5,
            },
        })
    }
}
