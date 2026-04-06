mod handlers;
pub mod models;
mod queries;

use crate::app::state::SharedState;
use crate::error::AppError;
use axum::{
    Router,
    routing::{get, post},
};
use uuid::Uuid;

pub fn author_routes() -> Router<SharedState> {
    Router::new()
        .route(
            "/",
            post(handlers::create_author).get(handlers::get_authors),
        )
        .route(
            "/{id}",
            get(handlers::get_author_by_id)
                .patch(handlers::patch_author)
                .delete(handlers::delete_author),
        )
}

pub async fn author_exists(state: &SharedState, id: Uuid) -> Result<bool, AppError> {
    queries::author_exists(state, id).await
}
