use crate::util::types::{PossibleValues, RecordToAssay};
use sqlx::PgPool;

pub async fn update_api_price(
    pool: &PgPool,
    id: i32,
    price: Option<PossibleValues>,
) -> anyhow::Result<()> {
    match price {
        Some(PossibleValues::VgPlusValue(price)) => {
            tracing::info!("updating {id} with suggested price {:.2}", price);
            sqlx::query!(
                "UPDATE rec SET discogs_vgplus_value = $1, discogs_updated = NOW() WHERE id = $2",
                price,
                id,
            )
            .execute(pool)
            .await?;
        }
        _ => {
            tracing::info!("{id} has no suggested price");
        }
    }

    Ok(())
}

pub async fn update_www_price(
    pool: &PgPool,
    id: i32,
    price: Option<PossibleValues>,
) -> anyhow::Result<()> {
    if let Some(cheapest_price) = price {
        match cheapest_price {
            PossibleValues::VgPlusValue(value) => {
                tracing::info!("setting {id} VGPlusValue to {value:.2}",);
                sqlx::query!(
                    "UPDATE rec
                    SET discogs_vgplus_value = $1, discogs_updated = NOW()
                    WHERE id = $2",
                    value,
                    id
                )
                .execute(pool)
                .await?;
            }
            PossibleValues::CheapestUk(value) => {
                tracing::info!("setting {id} Cheapest UK Value to {value:.2}",);
                sqlx::query!(
                    "UPDATE rec
                    SET discogs_cheapest_uk = $1, discogs_updated = NOW()
                    WHERE id = $2",
                    value,
                    id
                )
                .execute(pool)
                .await?;
            }
            PossibleValues::CheapestWorld(value) => {
                tracing::info!("setting {id} Cheapest World Value to {value:.2}",);
                sqlx::query!(
                    "UPDATE rec
                    SET discogs_cheapest_world = $1, discogs_updated = NOW()
                    WHERE id = $2",
                    value,
                    id
                )
                .execute(pool)
                .await?;
            }
        }
    } else {
        tracing::info!("{id} has no cheapest price");
    }

    Ok(())
}

pub async fn update_discogs_scraped(pool: &PgPool, id: i32) -> anyhow::Result<()> {
    sqlx::query!("UPDATE rec SET discogs_updated = NOW() WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn records_to_assay(
    pool: &PgPool,
    batch_size: usize,
) -> anyhow::Result<Vec<RecordToAssay>> {
    tracing::debug!("fetching {batch_size} records from database");

    let result = sqlx::query_as!(
        RecordToAssay,
        "SELECT id, discogs_uri, title FROM rec ORDER BY discogs_updated LIMIT $1",
        batch_size as i64
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
