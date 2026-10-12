use crate::helpers;
use crate::types::{Artist, Medium, ReleaseType};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ValuedRecord {
    pub id: i32,
    pub title: Option<String>,
    pub label_id: i32,
    pub label_name: Option<String>,
    pub artists: sqlx::types::Json<Vec<Artist>>,
    pub release_type: ReleaseType,
    pub medium: Medium,
    pub released: Option<i16>,
    #[serde(serialize_with = "helpers::round_to_pence")]
    pub value_estimated: Option<BigDecimal>,
    #[serde(serialize_with = "helpers::round_to_pence")]
    pub value_cheapest_uk: Option<BigDecimal>,
    #[serde(serialize_with = "helpers::round_to_pence")]
    pub value_cheapest_world: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct Params {
    pub medium: Medium,
    pub limit: i64,
}

pub async fn most_expensive(
    State(pool): State<PgPool>,
    Path(params): Path<Params>,
) -> Result<Json<Vec<ValuedRecord>>, StatusCode> {
    let discog = sqlx::query_as!(
        ValuedRecord,
        r#"
        SELECT 
            r.id,
            r.title,
            l.name as label_name,
            l.id as label_id,
            r.medium AS "medium!: Medium",
            r.release_type AS "release_type!: ReleaseType",
            r.released,
            r.discogs_vgplus_value as value_estimated,
            r.discogs_cheapest_uk as value_cheapest_uk,
            r.discogs_cheapest_world as value_cheapest_world,
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
        WHERE medium = $1::medium
        GROUP BY r.id, r.title, l.name, l.id
        ORDER BY r.discogs_cheapest_uk DESC
        LIMIT $2
        "#,
        &params.medium as _,
        params.limit,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(discog))
}
