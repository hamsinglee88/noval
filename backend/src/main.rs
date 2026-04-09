use std::env;

use noval_backend::{app_state::AppState, create_router, db};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "noval_backend=debug,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        env::var("NOVAL_DB_URL").unwrap_or_else(|_| "sqlite://backend/data/noval.db".to_string());
    let address = env::var("NOVAL_API_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let db = db::create_pool(&database_url).await?;
    db::run_migrations(&db).await?;

    let app = create_router(AppState { db });
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("noval backend listening on {address}");
    axum::serve(listener, app).await?;

    Ok(())
}
