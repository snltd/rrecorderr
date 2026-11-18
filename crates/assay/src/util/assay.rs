use crate::util::types::RecordToAssay;
use crate::util::{api, db, www};
use regex::Regex;
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

        if let Some(id) = release_id(discogs_uri) {
            let api_uri = format!("https://api.discogs.com/marketplace/price_suggestions/{id}");
            let suggested_price = api::marketplace_suggestion(api_uri)?;
            db::update_api_price(pool, record.id, suggested_price).await?;
            let cheapest_price = www::cheapest_available(&id)?;
            db::update_www_price(pool, record.id, cheapest_price).await?;
            db::update_discogs_scraped(pool, record.id).await?;
        } else {
            tracing::warn!("Could not get discogs ID for '{title}'");
        }
    } else {
        tracing::warn!("No discogs_uri for {title} [{}]", record.id);
    }

    Ok(())
}

pub fn release_id(discogs_uri: &str) -> Option<String> {
    tracing::debug!("looking for release ID in {discogs_uri}");
    let re = Regex::new(r"/release/(\d+)").unwrap();
    re.captures(discogs_uri)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_owned()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_release_id() {
        assert_eq!(
            "1618085".to_owned(),
            release_id("/Quintron-IF-001-011/release/1618085").unwrap()
        );
        assert_eq!(
            "31522313".to_owned(),
            release_id("/release/31522313-Melt-Banana-35").unwrap()
        );
    }
}
