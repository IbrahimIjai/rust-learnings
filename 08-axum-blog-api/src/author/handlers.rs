use crate::app::state::SharedState;
use crate::author::models::{AuthorFilter, AuthorListResponse, AuthorResponse, CreateAuthorRequest, UpdateAuthorRequest};
use crate::author::queries;
use crate::error::AppError;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

pub async fn create_author(
    State(state): State<SharedState>,
    Json(payload): Json<CreateAuthorRequest>,
) -> Result<impl IntoResponse, AppError> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::UnProcessableEntity {
            field: "name".to_string(),
            message: "Name cannot be empty".to_string(),
        });
    }

    let email = payload.email.trim().to_lowercase();
    if !is_valid_email(&email) {
        return Err(AppError::UnProcessableEntity {
            field: "email".to_string(),
            message: "Invalid email format".to_string(),
        });
    }

    let author = queries::insert_author(&state, name, &email).await?;
    Ok((StatusCode::CREATED, Json(AuthorResponse::from(author))))
}

pub async fn get_authors(
    State(state): State<SharedState>,
    Query(filter): Query<AuthorFilter>,
) -> Result<Json<AuthorListResponse>, AppError> {
    let limit = filter.limit.unwrap_or(20).clamp(1, 100);
    let offset = filter.offset.unwrap_or(0).max(0);

    let authors = queries::list_authors(&state, limit, offset).await?;
    let total = authors.len();
    let data = authors.into_iter().map(AuthorResponse::from).collect();

    Ok(Json(AuthorListResponse { data, total }))
}

pub async fn get_author_by_id(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<AuthorResponse>, AppError> {
    let author_id = parse_uuid_field("id", &id)?;
    let author = queries::find_author_by_id(&state, author_id).await?;
    Ok(Json(AuthorResponse::from(author)))
}

pub async fn patch_author(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAuthorRequest>,
) -> Result<Json<AuthorResponse>, AppError> {
    let author_id = parse_uuid_field("id", &id)?;

    let name = payload.name.as_deref().map(str::trim);
    if let Some(name) = name {
        if name.is_empty() {
            return Err(AppError::UnProcessableEntity {
                field: "name".to_string(),
                message: "Name cannot be empty".to_string(),
            });
        }
    }

    let email_owned = payload.email.map(|value| value.trim().to_lowercase());
    if let Some(email) = &email_owned {
        if !is_valid_email(email) {
            return Err(AppError::UnProcessableEntity {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
            });
        }
    }

    let email = email_owned.as_deref();

    if name.is_none() && email.is_none() {
        return Err(AppError::UnProcessableEntity {
            field: "body".to_string(),
            message: "At least one of name or email must be provided".to_string(),
        });
    }

    let author = queries::update_author(&state, author_id, name, email).await?;
    Ok(Json(AuthorResponse::from(author)))
}

pub async fn delete_author(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let author_id = parse_uuid_field("id", &id)?;
    queries::delete_author(&state, author_id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Author deleted successfully"
        })),
    ))
}

fn parse_uuid_field(field: &str, value: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(value).map_err(|_| AppError::UnProcessableEntity {
        field: field.to_string(),
        message: "Invalid UUID".to_string(),
    })
}

fn is_valid_email(email: &str) -> bool {
    let mut parts = email.split('@');
    let local = parts.next().unwrap_or("");
    let domain = parts.next().unwrap_or("");

    !local.is_empty() && domain.contains('.') && parts.next().is_none()
}
