use crate::core::{
    app_error::AppError,
    domain::health::{HealthService, HealthStatus},
    ports::health::HealthPort,
};
use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::TransformOperation;
use async_trait::async_trait;
use axum::Json;

pub struct HealthWebAdapter {
    health_service: HealthService,
}

impl HealthWebAdapter {
    pub fn new() -> Self {
        Self {
            health_service: HealthService::new(),
        }
    }
}

#[async_trait]
impl HealthPort for HealthWebAdapter {
    async fn get_health_status(&self) -> HealthStatus {
        self.health_service.check_health()
    }
}

#[axum::debug_handler]
async fn health_check_handler() -> Result<Json<HealthStatus>, AppError> {
    let adapter = HealthWebAdapter::new();
    let health_status = adapter.get_health_status().await;
    Ok(Json(health_status))
}

pub fn health_check_handler_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get health status.").tag("health")
}

pub fn health_check_routes() -> ApiRouter {
    ApiRouter::new().api_route(
        "/",
        get_with(health_check_handler, health_check_handler_docs),
    )
}
