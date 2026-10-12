// use bigdecimal::BigDecimal;
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// #[derive(Debug, sqlx::FromRow, Serialize)]
// pub struct Record {
// pub id: i32,
// pub artist_id: i32,
// publartist_name: <String>,
// pub title: Option<String>,
// #[sqlx(rename = "type")]
// pub rec_type: RecType,
// pub format: RecFormat,
// pub colour: Option<String>,
// pub label_name: i32,
// pub label_id: Option<String>,
// pub cat_no: Option<String>,
// pub released: Option<i16>,
// pub ltd_no: Option<String>,
// pub notes: Option<String>,
// pub cost: Option<BigDecimal>,
// pub created_at: DateTime<Utc>,
// pub discogs_updated: DateTime<Utc>,
// pub discogs_uri: Option<String>,
// pub discogs_vgplus_value: Option<BigDecimal>,
// pub discogs_cheapest_uk: Option<BigDecimal>,
// pub discogs_cheapest_world: Option<BigDecimal>,
// }

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, strum::EnumIter, strum::Display)]
#[sqlx(type_name = "medium", rename_all = "lowercase")]
pub enum Medium {
    #[serde(rename = "10 inch vinyl")]
    #[sqlx(rename = "10 inch vinyl")]
    #[strum(serialize = "10 inch vinyl")]
    Vinyl10,
    #[serde(rename = "12 inch vinyl")]
    #[sqlx(rename = "12 inch vinyl")]
    #[strum(serialize = "12 inch vinyl")]
    Vinyl12,
    #[serde(rename = "3 inch cd")]
    #[sqlx(rename = "3 inch cd")]
    #[strum(serialize = "3 inch cd")]
    Vinyl3,
    #[serde(rename = "5 inch vinyl")]
    #[sqlx(rename = "5 inch vinyl")]
    #[strum(serialize = "5 inch vinyl")]
    Vinyl5,
    #[serde(rename = "6 inch vinyl")]
    #[sqlx(rename = "6 inch vinyl")]
    #[strum(serialize = "6 inch vinyl")]
    Vinyl6,
    #[serde(rename = "7 inch vinyl")]
    #[sqlx(rename = "7 inch vinyl")]
    #[strum(serialize = "7 inch vinyl")]
    Vinyl7,
    #[serde(rename = "8 inch vinyl")]
    #[sqlx(rename = "8 inch vinyl")]
    #[strum(serialize = "8 inch vinyl")]
    Vinyl8,
    #[serde(rename = "9 inch vinyl")]
    #[sqlx(rename = "9 inch vinyl")]
    #[strum(serialize = "9 inch vinyl")]
    Vinyl9,
    #[serde(rename = "11 inch vinyl")]
    #[sqlx(rename = "11 inch vinyl")]
    #[strum(serialize = "11 inch vinyl")]
    Vinyl11,
    #[serde(rename = "cassette")]
    #[sqlx(rename = "cassette")]
    #[strum(serialize = "cassette")]
    Cassette,
    #[serde(rename = "cd")]
    #[sqlx(rename = "cd")]
    #[strum(serialize = "cd")]
    Cd,
    #[serde(rename = "flexidisc")]
    #[sqlx(rename = "flexidisc")]
    #[strum(serialize = "flexidisc")]
    Flexidisc,
    #[serde(rename = "DVD")]
    #[sqlx(rename = "DVD")]
    #[strum(serialize = "DVD")]
    Dvd,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "release_type", rename_all = "lowercase")]
pub enum ReleaseType {
    #[serde(rename = "box set")]
    #[sqlx(rename = "box set")]
    BoxSet,
    #[serde(rename = "lp")]
    #[sqlx(rename = "lp")]
    Lp,
    #[serde(rename = "lp sampler")]
    #[sqlx(rename = "lp sampler")]
    LpSampler,
    #[serde(rename = "mini lp")]
    #[sqlx(rename = "mini lp")]
    LpMini,
    #[serde(rename = "single")]
    #[sqlx(rename = "single")]
    Single,
    #[serde(rename = "split single")]
    #[sqlx(rename = "split single")]
    Split,
    #[serde(rename = "split lp")]
    #[sqlx(rename = "split lp")]
    LpSplit,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
pub struct Label {
    pub id: i32,
    pub name: String,
}
