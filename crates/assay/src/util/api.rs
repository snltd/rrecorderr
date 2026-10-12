use crate::util::types::PossibleValues;
use anyhow::{Context, bail};
use bigdecimal::BigDecimal;
use serde_json::Value;
use std::env;
use std::str::FromStr;

pub const USER_AGENT: &str = "Assay/1.0 +https://tech.id264.net";

// Gives us the price Discogs thinks a CONDITION copy of this thing is worth. It's
// probably miles out.
pub fn marketplace_suggestion(uri: String) -> anyhow::Result<Option<PossibleValues>> {
    tracing::info!("fetching {uri}");
    let discogs_token = env::var("DISCOGS_TOKEN").expect("no DISCOGS_TOKEN");
    let condition = env::var("DEFAULT_CONDITION").unwrap_or("Very Good Plus (VG+)".to_owned());

    let body = ureq::get(uri)
        .header("Authorization", format!("Discogs token={discogs_token}"))
        .header("user_agent", USER_AGENT)
        .call()?
        .body_mut()
        .read_to_string()?;

    let json: Value = serde_json::from_str(&body)?;

    if let Some(value) = json.get(&condition).and_then(|u| u.get("value")) {
        match value {
            Value::Number(n) => {
                let bd_value =
                    BigDecimal::from_str(&n.to_string()).context(format!("could not parse {n}"))?;
                tracing::debug!("marketplace suggestion: {bd_value:.2}");
                Ok(Some(PossibleValues::VgPlusValue(bd_value)))
            }
            _ => {
                bail!("could not get '{condition}' price")
            }
        }
    } else {
        Ok(None)
    }
}
