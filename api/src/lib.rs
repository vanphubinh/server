use std::net::SocketAddr;

use axum::{routing::get, Router};
use config::Config;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
pub async fn start() {
    // Initialize configuration
    let config = Config::load().expect("Failed to load configuration");

    // Initialize tracing
    setup_tracing(&config);

    // Build the application router
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], config.server.port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn setup_tracing(config: &Config) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.log.level.clone()));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Tracing initialized");
}
