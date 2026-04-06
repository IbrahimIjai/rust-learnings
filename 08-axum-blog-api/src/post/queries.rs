use crate::app::state::SharedState;
use crate::author;
use crate::error::AppError;
use crate::post::models::Post;
use sqlx::QueryBuilder;
use uuid::Uuid;

pub async fn insert_post(
    state: &SharedState,
    title: &str,
    content: &str,
    author_id: Uuid,
) -> Result<Post, AppError> {
    let has_author = author::author_exists(state, author_id).await?;
    if !has_author {
        return Err(AppError::UnProcessableEntity {
            field: "author_id".to_string(),
            message: "Author does not exist".to_string(),
        });
    }

    let post_id = Uuid::new_v4();

    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (id, title, content, author_id) VALUES ($1, $2, $3, $4) RETURNING id, title, content, author_id, created_at, updated_at",
    )
    .bind(post_id)
    .bind(title)
    .bind(content)
    .bind(author_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("Failed to create post: {err}")))?;

    Ok(post)
}

pub async fn list_posts(
    state: &SharedState,
    author_id: Option<Uuid>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, AppError> {
    let mut builder = QueryBuilder::new(
        "SELECT id, title, content, author_id, created_at, updated_at FROM posts",
    );

    if let Some(author_id) = author_id {
        builder.push(" WHERE author_id = ");
        builder.push_bind(author_id);
    }

    builder
        .push(" ORDER BY created_at DESC LIMIT ")
        .push_bind(limit)
        .push(" OFFSET ")
        .push_bind(offset);

    let posts = builder
        .build_query_as::<Post>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|err| AppError::InternalServerError(format!("Failed to fetch posts: {err}")))?;

    Ok(posts)
}

pub async fn find_post_by_id(state: &SharedState, id: Uuid) -> Result<Post, AppError> {
    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, author_id, created_at, updated_at FROM posts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("Failed to fetch post: {err}")))?
    .ok_or_else(|| AppError::NotFound(format!("Post not found: {id}")))?;

    Ok(post)
}

pub async fn update_post(
    state: &SharedState,
    id: Uuid,
    title: Option<&str>,
    content: Option<&str>,
) -> Result<Post, AppError> {
    let post = sqlx::query_as::<_, Post>(
        "UPDATE posts SET title = COALESCE($2, title), content = COALESCE($3, content), updated_at = NOW() WHERE id = $1 RETURNING id, title, content, author_id, created_at, updated_at",
    )
    .bind(id)
    .bind(title)
    .bind(content)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|err| AppError::InternalServerError(format!("Failed to update post: {err}")))?
    .ok_or_else(|| AppError::NotFound(format!("Post not found: {id}")))?;

    Ok(post)
}

pub async fn delete_post(state: &SharedState, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|err| AppError::InternalServerError(format!("Failed to delete post: {err}")))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Post not found: {id}")));
    }

    Ok(())
}
