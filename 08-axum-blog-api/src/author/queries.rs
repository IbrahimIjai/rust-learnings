use crate::app::state::SharedState;
use crate::author::models::Author;
use crate::error::AppError;
use sqlx::QueryBuilder;
use uuid::Uuid;

pub async fn insert_author(state: &SharedState, name: &str, email: &str) -> Result<Author, AppError> {
    let author_id = Uuid::new_v4();

    let author = sqlx::query_as::<_, Author>(
        "INSERT INTO authors (id, name, email) VALUES ($1, $2, $3) RETURNING id, name, email, created_at, updated_at",
    )
    .bind(author_id)
    .bind(name)
    .bind(email)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|err| {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.code().as_deref() == Some("23505") {
                return AppError::UnProcessableEntity {
                    field: "email".to_string(),
                    message: "Email already exists".to_string(),
                };
            }
        }
        AppError::InternalServerError(format!("Failed to create author: {err}"))
    })?;

    Ok(author)
}

pub async fn list_authors(
    state: &SharedState,
    limit: i64,
    offset: i64,
) -> Result<Vec<Author>, AppError> {
    let mut builder = QueryBuilder::new(
        "SELECT id, name, email, created_at, updated_at FROM authors",
    );

    builder
        .push(" ORDER BY created_at DESC LIMIT ")
        .push_bind(limit)
        .push(" OFFSET ")
        .push_bind(offset);

    let authors = builder
        .build_query_as::<Author>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|err| AppError::InternalServerError(format!("Failed to fetch authors: {err}")))?;

    Ok(authors)
}

pub async fn find_author_by_id(state: &SharedState, id: Uuid) -> Result<Author, AppError> {
    let author = sqlx::query_as::<_, Author>(
        "SELECT id, name, email, created_at, updated_at FROM authors WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("Failed to fetch author: {err}")))?
    .ok_or_else(|| AppError::NotFound(format!("Author not found: {id}")))?;

    Ok(author)
}

pub async fn update_author(
    state: &SharedState,
    id: Uuid,
    name: Option<&str>,
    email: Option<&str>,
) -> Result<Author, AppError> {
    let author = sqlx::query_as::<_, Author>(
        "UPDATE authors SET name = COALESCE($2, name), email = COALESCE($3, email), updated_at = NOW() WHERE id = $1 RETURNING id, name, email, created_at, updated_at",
    )
    .bind(id)
    .bind(name)
    .bind(email)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|err| {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.code().as_deref() == Some("23505") {
                return AppError::UnProcessableEntity {
                    field: "email".to_string(),
                    message: "Email already exists".to_string(),
                };
            }
        }
        AppError::InternalServerError(format!("Failed to update author: {err}"))
    })?
    .ok_or_else(|| AppError::NotFound(format!("Author not found: {id}")))?;

    Ok(author)
}

pub async fn delete_author(state: &SharedState, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM authors WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|err| AppError::InternalServerError(format!("Failed to delete author: {err}")))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Author not found: {id}")));
    }

    Ok(())
}

pub async fn author_exists(state: &SharedState, id: Uuid) -> Result<bool, AppError> {
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM authors WHERE id = $1)")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|err| AppError::InternalServerError(format!("Failed to check author: {err}")))?;

    Ok(exists)
}
