use crate::types::{Artist, Label};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sqlx::PgPool;

#[axum::debug_handler]
pub async fn artists(
    State(pool): State<PgPool>,
    Path(prefix): Path<String>,
) -> Result<Json<Vec<Artist>>, StatusCode> {
    let list = sqlx::query_as!(
        Artist,
        r#"SELECT id, name
            FROM artist
            WHERE lower(name) LIKE $1
            ORDER BY name COLLATE "und-x-icu""#,
        format!("{}%", prefix.to_lowercase())
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(list))
}

#[axum::debug_handler]
pub async fn labels(
    State(pool): State<PgPool>,
    Path(prefix): Path<String>,
) -> Result<Json<Vec<Label>>, StatusCode> {
    let list = sqlx::query_as!(
        Label,
        r#"SELECT id, name
            FROM label
            WHERE lower(name) LIKE $1
            ORDER BY name COLLATE "und-x-icu""#,
        format!("{}%", prefix.to_lowercase())
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(list))
}
