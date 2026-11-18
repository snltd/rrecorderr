use crate::types::{Artist, Medium, ReleaseType};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RecordWithArtists {
    pub id: i32,
    pub title: Option<String>,
    pub label_id: i32,
    pub label_name: Option<String>,
    pub artists: sqlx::types::Json<Vec<Artist>>,
    pub release_type: ReleaseType,
    pub medium: Medium,
    pub colour: Option<String>,
    pub cat_no: Option<String>,
    pub released: Option<i16>,
    pub notes: Option<String>,
    pub value: Option<f64>,
}

pub async fn artist(
    State(pool): State<PgPool>,
    Path(artist_id): Path<i32>,
) -> Result<Json<Vec<RecordWithArtists>>, StatusCode> {
    let discog = sqlx::query_as!(
        RecordWithArtists,
        r#"
        SELECT 
            r.id,
            r.title,
            l.name as label_name,
            l.id as label_id,
            r.medium AS "medium!: Medium",
            r.release_type AS "release_type!: ReleaseType",
            r.colour,
            r.cat_no,
            r.released,
            r.notes,
            CAST(r.discogs_vgplus_value AS FLOAT8) as value,
            jsonb_agg(
                jsonb_build_object(
                    'id', a.id,
                    'name', a.name
                ) ORDER BY a.name
            ) AS "artists!: sqlx::types::Json<Vec<Artist>>"
        FROM rec r
        JOIN label l ON l.id = r.label_id
        JOIN perf p ON p.rec_id = r.id
        JOIN artist a ON a.id = p.artist_id
        WHERE r.id IN (
            SELECT rec_id FROM perf WHERE artist_id = $1
        )
        GROUP BY r.id, r.title, l.name, l.id
        ORDER BY r.released, r.cat_no COLLATE "und-x-icu"
        "#,
        artist_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(discog))
}

pub async fn label(
    State(pool): State<PgPool>,
    Path(label_id): Path<i32>,
) -> Result<Json<Vec<RecordWithArtists>>, (StatusCode, String)> {
    let discog = sqlx::query_as!(
        RecordWithArtists,
        r#"
        SELECT 
            r.id,
            l.name as label_name,
            l.id as label_id,
            r.title AS "title!",
            r.medium AS "medium!: Medium",
            r.release_type AS "release_type!: ReleaseType",
            r.colour,
            r.cat_no,
            r.released,
            r.notes,
            CAST(r.discogs_vgplus_value AS FLOAT8) as value,
            jsonb_agg(
                jsonb_build_object(
                    'id', a.id,
                    'name', a.name
                ) ORDER BY a.name
            ) AS "artists!: sqlx::types::Json<Vec<Artist>>"
        FROM rec r
        JOIN perf p ON p.rec_id = r.id
        JOIN label l ON l.id = r.label_id
        JOIN artist a ON a.id = p.artist_id
        WHERE r.label_id = $1
        GROUP BY r.id, r.title, r.medium, l.name, l.id
        ORDER BY r.released, r.cat_no COLLATE "und-x-icu"
        "#,
        label_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(discog))
}
