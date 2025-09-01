use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HealthStatus {
    pub status: String,
}

pub struct HealthService;

impl HealthService {
    pub fn new() -> Self {
        Self
    }

    pub fn check_health(&self) -> HealthStatus {
        HealthStatus {
            status: "ok".to_string(),
        }
    }
}
