use crate::types::Medium;
use axum::Json;
use strum::IntoEnumIterator;

#[axum::debug_handler]
pub async fn list() -> Json<Vec<String>> {
    Json(Medium::iter().map(|m| m.to_string()).collect())
}
