use crate::util::{assay, db};
use std::env;
use tracing_subscriber::EnvFilter;
mod util;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("couldn't load env file");

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_ansi(true)
        .init();

    let pool = sqlx::PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("failed to connect to database");

    let batch_size = std::env::var("BATCH_SIZE")
        .unwrap_or("10".to_owned())
        .parse::<usize>()
        .expect("invalid batch size");

    match db::records_to_assay(&pool, batch_size).await {
        Ok(list_of_records) => match assay::assay_list(&pool, list_of_records).await {
            Ok(_) => println!("OK"),
            Err(e) => {
                tracing::error!("ERROR assaying: {e}");
                std::process::exit(2);
            }
        },
        Err(e) => {
            tracing::error!("ERROR getting list: {e}");
            std::process::exit(2);
        }
    }
}
