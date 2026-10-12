use crate::util::types::RecordToAssay;
use crate::util::{api, db, helpers, www};
use sqlx::PgPool;

pub async fn assay_list(pool: &PgPool, list_of_records: Vec<RecordToAssay>) -> anyhow::Result<()> {
    for record in list_of_records {
        assay_record(pool, record).await?;
    }

    Ok(())
}

async fn assay_record(pool: &PgPool, record: RecordToAssay) -> anyhow::Result<()> {
    let title = record.title.unwrap_or("<untitled>".to_owned());

    if let Some(discogs_uri) = &record.discogs_uri {
        tracing::info!("considering {title} [{}]", record.id);

        if let Some(id) = helpers::release_id(discogs_uri) {
            let api_uri = format!("https://api.discogs.com/marketplace/price_suggestions/{id}");
            let suggested_price = api::marketplace_suggestion(api_uri)?;
            db::update_api_price(pool, record.id, suggested_price).await?;
            let cheapest_price = www::cheapest_available(&id)?;
            db::update_www_price(pool, record.id, cheapest_price).await?;
        } else {
            tracing::error!("Could not get ID");
        }
    } else {
        tracing::warn!("No discogs_uri for {title} [{}]", record.id);
    }

    Ok(())
}
