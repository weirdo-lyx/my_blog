use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use crate::models::{CreatePost, Post};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}


// --- Route Handlers ---

pub async fn get_posts(State(state): State<AppState>) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as::<_, Post>("SELECT id, title, content FROM posts ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(posts))
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(input): Json<CreatePost>,
) -> Result<(StatusCode, Json<Post>), StatusCode> {
    let post = sqlx::query_as::<_, Post>("INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id, title, content")
        .bind(&input.title)
        .bind(&input.content)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, StatusCode> {
    sqlx::query_as::<_, Post>("SELECT id, title, content FROM posts WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map(Json)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<CreatePost>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("UPDATE posts SET title = $1, content = $2 WHERE id = $3")
        .bind(&input.title)
        .bind(&input.content)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::OK)
    }
}

pub async fn delete_post(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
