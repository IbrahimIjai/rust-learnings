use crate::app::state::SharedState;
use crate::error::AppError;
use crate::post::models::{CreatePostRequest, PostFilter, PostListResponse, PostResponse, UpdatePostRequest};
use crate::post::queries;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

pub async fn create_post(
    State(state): State<SharedState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, AppError> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(AppError::UnProcessableEntity {
            field: "title".to_string(),
            message: "Title cannot be empty".to_string(),
        });
    }

    let content = payload.content.trim();
    if content.is_empty() {
        return Err(AppError::UnProcessableEntity {
            field: "content".to_string(),
            message: "Content cannot be empty".to_string(),
        });
    }

    let author_id = parse_uuid_field("author_id", &payload.author_id)?;

    let post = queries::insert_post(&state, title, content, author_id).await?;
    Ok((StatusCode::CREATED, Json(PostResponse::from(post))))
}

pub async fn get_posts(
    State(state): State<SharedState>,
    Query(filter): Query<PostFilter>,
) -> Result<Json<PostListResponse>, AppError> {
    let limit = filter.limit.unwrap_or(20).clamp(1, 100);
    let offset = filter.offset.unwrap_or(0).max(0);

    let author_id = match filter.author_id {
        Some(raw) => Some(parse_uuid_field("author_id", &raw)?),
        None => None,
    };

    let posts = queries::list_posts(&state, author_id, limit, offset).await?;
    let total = posts.len();
    let data = posts.into_iter().map(PostResponse::from).collect();

    Ok(Json(PostListResponse { data, total }))
}

pub async fn get_post_by_id(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<PostResponse>, AppError> {
    let post_id = parse_uuid_field("id", &id)?;
    let post = queries::find_post_by_id(&state, post_id).await?;
    Ok(Json(PostResponse::from(post)))
}

pub async fn patch_post(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>, AppError> {
    let post_id = parse_uuid_field("id", &id)?;

    let title = payload.title.as_deref().map(str::trim);
    if let Some(title) = title {
        if title.is_empty() {
            return Err(AppError::UnProcessableEntity {
                field: "title".to_string(),
                message: "Title cannot be empty".to_string(),
            });
        }
    }

    let content = payload.content.as_deref().map(str::trim);
    if let Some(content) = content {
        if content.is_empty() {
            return Err(AppError::UnProcessableEntity {
                field: "content".to_string(),
                message: "Content cannot be empty".to_string(),
            });
        }
    }

    if title.is_none() && content.is_none() {
        return Err(AppError::UnProcessableEntity {
            field: "body".to_string(),
            message: "At least one of title or content must be provided".to_string(),
        });
    }

    let post = queries::update_post(&state, post_id, title, content).await?;
    Ok(Json(PostResponse::from(post)))
}

pub async fn delete_post(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let post_id = parse_uuid_field("id", &id)?;
    queries::delete_post(&state, post_id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Post deleted successfully"
        })),
    ))
}

fn parse_uuid_field(field: &str, value: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(value).map_err(|_| AppError::UnProcessableEntity {
        field: field.to_string(),
        message: "Invalid UUID".to_string(),
    })
}
