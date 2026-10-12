use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use sqlx::PgPool;

#[axum::debug_handler]
pub async fn artists(State(pool): State<PgPool>) -> Result<Json<Vec<String>>, StatusCode> {
    // Not sure why sqlx thinks this is nullable. It's not.
    let list = sqlx::query_scalar!(
        r#"SELECT DISTINCT LOWER(LEFT(name, 1)) AS "initial!: String"
        FROM artist
        ORDER BY 1"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(list))
}

#[axum::debug_handler]
pub async fn labels(State(pool): State<PgPool>) -> Result<Json<Vec<String>>, StatusCode> {
    let list = sqlx::query_scalar!(
        r#"SELECT DISTINCT LOWER(LEFT(name, 1)) AS "initial!: String"
        FROM label
        ORDER BY 1"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(list))
}
