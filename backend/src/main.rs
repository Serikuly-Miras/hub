use crate::adapters::web::health::health_check_routes;
use crate::core::{app_error::AppError, docs::docs_routes};
use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use appstate::AppState;
use axum::{Extension, Json};
use core::appstate;
use reqwest::StatusCode;
use std::{env, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{CompressionLevel, compression::CompressionLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod adapters;
mod core;
mod schema;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "warn,sqlx=debug,tower_http=debug,{}=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize application state from env
    let state = AppState::from_env().await?;
    let listen_port = state.env.port;

    // Register routes
    aide::generate::on_error(|error| {
        println!("{error}");
    });
    aide::generate::extract_schemas(true);
    let mut api = OpenApi::default();
    let app = ApiRouter::new()
        .nest_api_service("/api/docs", docs_routes(state.clone()))
        .nest_api_service("/api/health", health_check_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
        .with_state(state);

    // Add compression layer (applies to all routes)
    let app = app.layer(
        CompressionLayer::new()
            .gzip(true)
            .br(true)
            .deflate(true)
            .quality(CompressionLevel::Fastest),
    );

    // Start the server
    let listener = TcpListener::bind(format!("0.0.0.0:{listen_port}")).await?;
    tracing::debug!("🚀 listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("API")
        .summary("Blog backend API")
        .default_response_with::<Json<AppError>, _>(|res| {
            res.example(AppError {
                error: "some error happened".to_string(),
                error_details: None,
                status: StatusCode::IM_A_TEAPOT,
            })
        })
}
