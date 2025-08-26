use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, JsonSchema, OperationIo)]
pub struct AppError {
    pub error: String,
    #[serde(skip)]
    pub status: StatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Value>,
}

impl AppError {
    #[allow(dead_code)]
    pub fn new(error: &str) -> Self {
        Self {
            error: error.to_string(),
            status: StatusCode::BAD_REQUEST,
            error_details: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    #[allow(dead_code)]
    pub fn with_details(mut self, details: Value) -> Self {
        self.error_details = Some(details);
        self
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = status;
        res
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            error: err.into().to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error_details: None,
        }
    }
}
