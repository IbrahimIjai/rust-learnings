mod handlers;
pub mod models;
mod queries;

use crate::app::state::SharedState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn post_routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::create_post).get(handlers::get_posts))
        .route(
            "/{id}",
            get(handlers::get_post_by_id)
                .patch(handlers::patch_post)
                .delete(handlers::delete_post),
        )
}
