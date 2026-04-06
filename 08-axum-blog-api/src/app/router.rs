use crate::app::state::SharedState;
use crate::author::author_routes;
use crate::post::post_routes;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::get,
};
use serde_json::json;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .nest("/authors", author_routes())
        .nest("/posts", post_routes())
        .route("/", get(hello))
        .route("/health", get(health_handler))
        .with_state(state)
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn health_handler() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Server is running"
        })),
    )
}
