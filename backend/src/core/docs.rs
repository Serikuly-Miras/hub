use crate::core::appstate::AppState;
use aide::swagger::Swagger;
use aide::{
    axum::{
        ApiRouter, IntoApiResponse,
        routing::{get, get_with},
    },
    openapi::OpenApi,
};
use axum::{Extension, Json, response::IntoResponse};
use std::sync::Arc;

pub fn docs_routes(state: AppState) -> ApiRouter {
    aide::generate::infer_responses(true);

    let router: ApiRouter = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Swagger::new("/api/docs/private/api.json")
                    .with_title("Swagger API Docs")
                    .axum_handler(),
                |op| op.description("This documentation page.").tag("docs"),
            ),
            |p| p,
        )
        .route("/private/api.json", get(serve_docs))
        .with_state(state);

    aide::generate::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(&*api).into_response()
}
