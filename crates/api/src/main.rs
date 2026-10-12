mod handlers;
mod helpers;
mod types;
use axum::http::{HeaderValue, Method, header};
use axum::{Router, routing::get};
use std::env;
use std::time::Duration;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("couldn't load env file");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let origins: Vec<HeaderValue> = env::var("CORS_ORIGINS")
        .unwrap_or("http://localhost:5173".to_owned())
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<_, _>>()
        .expect("failed to set up CORS origins");

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    let pool = sqlx::PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("failed to connect to database");

    let app = Router::new()
        .route("/initials/artists", get(handlers::initials::artists))
        .route("/initials/labels", get(handlers::initials::labels))
        .route("/artists/{prefix}", get(handlers::list_by_prefix::artists))
        .route("/labels/{prefix}", get(handlers::list_by_prefix::labels))
        .route("/discog/artist/{id}", get(handlers::discography::artist))
        .route("/discog/label/{id}", get(handlers::discography::label))
        .route("/media", get(handlers::media_types::list))
        .route(
            "/most_expensive/{medium}/{limit}",
            get(handlers::most_expensive::most_expensive),
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3456").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
