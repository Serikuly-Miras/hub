use crate::core::domain::health::HealthStatus;
use async_trait::async_trait;

#[async_trait]
pub trait HealthPort {
    async fn get_health_status(&self) -> HealthStatus;
}
