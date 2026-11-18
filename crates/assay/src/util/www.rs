use crate::util::api::USER_AGENT;
use crate::util::types::PossibleValues;
use anyhow::bail;
use bigdecimal::BigDecimal;
use scraper::{Html, Selector};
use std::env;
use std::str::FromStr;

pub fn cheapest_available(id: &str) -> anyhow::Result<Option<PossibleValues>> {
    let uk_url = uk_marketplace_url(id);
    tracing::info!("fetching UK prices from {uk_url}");
    let uk_raw = raw_page(&uk_url)?;

    if let Some(uk_price) = extract_price_uk(&uk_raw, id)? {
        tracing::debug!("found UK price of {uk_price}");
        return Ok(Some(PossibleValues::CheapestUk(uk_price)));
    }

    tracing::info!("{id} has no UK price");
    let world_url = world_marketplace_url(id);
    tracing::info!("fetching world prices from {world_url}");
    let world_raw = raw_page(&world_url)?;

    if let Some(world_price) = extract_price_world(&world_raw, id)? {
        tracing::info!("found world price of {world_price}");
        return Ok(Some(PossibleValues::CheapestWorld(world_price)));
    }

    Ok(None)
}

fn raw_page(url: &str) -> anyhow::Result<String> {
    let discogs_token = env::var("DISCOGS_TOKEN").expect("no DISCOGS_TOKEN");
    let body = ureq::get(url)
        .header("Authorization", format!("Discogs token={discogs_token}"))
        .header("user_agent", USER_AGENT)
        .call()?
        .body_mut()
        .read_to_string()?;
    Ok(body)
}

fn uk_marketplace_url(id: &str) -> String {
    format!("https://www.discogs.com/sell/release/{id}?ships_from=United+Kingdom")
}

fn world_marketplace_url(id: &str) -> String {
    format!("https://www.discogs.com/sell/release/{id}")
}

// Gets the cheapest price from the given HTML page
// /html/body/div[1]/div[3]/div[3]/div/div[2]/div[2]/table/tbody/tr[1]/td[5]/div[1]
fn _extract_price(html: &str, selector_str: &str) -> anyhow::Result<Option<BigDecimal>> {
    tracing::debug!("parsing HTML");
    let document = Html::parse_document(html);

    if let Ok(selector) = Selector::parse(selector_str) {
        if let Some(element) = document.select(&selector).next() {
            Ok(price_from_str(
                &element.text().collect::<Vec<_>>().join(" "),
            ))
        } else {
            tracing::debug!("selector matched nothing");
            Ok(None)
        }
    } else {
        bail!("Error parsing document");
    }
}

fn price_from_str(s: &str) -> Option<BigDecimal> {
    println!("EXTRACTING FROM {s}");
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();

    let cleaned = cleaned.trim_matches('.');
    println!("CLEANED IS {cleaned}");
    BigDecimal::from_str(cleaned).ok()
}

fn extract_price_uk(html: &str, id: &str) -> anyhow::Result<Option<BigDecimal>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!(
        r#"tr.shortcut_navigable[data-release-id="{id}"] span.price"#
    ))
    .map_err(|e| anyhow::anyhow!("bad selector: {e}"))?;

    dbg!(document.select(&selector).next());

    Ok(document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("data-pricevalue"))
        .and_then(price_from_str))
}

fn extract_price_world(html: &str, id: &str) -> anyhow::Result<Option<BigDecimal>> {
    let document = Html::parse_document(html);

    let selector = Selector::parse(&format!(
        r#"tr.shortcut_navigable[data-release-id="{id}"] span.converted_price"#
    ))
    .map_err(|e| anyhow::anyhow!("bad selector: {e}"))?;

    Ok(document
        .select(&selector)
        .next()
        .and_then(|el| price_from_str(&el.text().collect::<String>())))
}

#[cfg(test)]
mod test {
    use super::*;
    use snltest::load_fixture;
    use std::str::FromStr;

    #[test]
    fn test_extract_price_world() {
        assert_eq!(
            BigDecimal::from_str("25.42").unwrap(),
            extract_price_world(&load_fixture!("31853303-world.html"), "31853303")
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn test_extract_price_uk() {
        assert_eq!(
            BigDecimal::from_str("17.50").unwrap(),
            extract_price_uk(&load_fixture!("31853303-uk.html"), "31853303")
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn test_price_from_str() {
        assert_eq!(None, price_from_str("no price"));
        assert_eq!(
            BigDecimal::from_str("12.34").unwrap(),
            price_from_str("£12.34").unwrap()
        );
        assert_eq!(
            BigDecimal::from_str("9.99").unwrap(),
            price_from_str(" about  £9.99 ").unwrap()
        );
    }
}
