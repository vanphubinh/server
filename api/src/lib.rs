use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router};
use config::Config;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::signal;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utils::AppState;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use utoipauto::utoipauto;

mod routes;

#[tokio::main]
pub async fn start() -> Result<()> {
    /// Generate the OpenAPI documentation for the API
    #[utoipauto(paths = "./utils/src from utils, ./context/measurement/src from measurement")]
    #[derive(OpenApi)]
    #[openapi(info(
        title = "Van Phu Binh API",
        version = "1.0.0",
        description = "API for managing Van Phu Binh Internal System"
    ))]
    pub struct ApiDoc;

    /// Serve the OpenAPI specification in JSON format
    pub async fn serve_openapi() -> impl axum::response::IntoResponse {
        axum::Json(ApiDoc::openapi())
    }

    // Initialize configuration
    let config = Config::load().expect("Failed to load configuration");

    // Initialize tracing
    setup_tracing(&config);

    // Connect to the database
    let db = setup_database(&config).await?;

    // Create app state
    let state = AppState::new(db).into_shared();

    // Build the application router
    let app = Router::new()
        .route("/openapi.json", get(serve_openapi))
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .merge(routes::api_router())
        .with_state(state);

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], config.server.port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutdown completed");
    Ok(())
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

async fn setup_database(config: &Config) -> Result<DatabaseConnection> {
    let db_url = &config.database.url;

    // Create connection options
    let mut opt = ConnectOptions::new(db_url.to_owned());

    // Enable SQL query logging only in development mode
    if cfg!(debug_assertions) {
        opt.sqlx_logging(true);
    } else {
        opt.sqlx_logging(false);
    }

    let db = Database::connect(opt).await?;

    // Run migrations if enabled
    if config.database.run_migrations {
        info!("Running database migrations");
        migration::Migrator::up(&db, None).await?;
    }

    info!("Database connection established");
    Ok(db)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, starting graceful shutdown");
        },
        _ = terminate => {
            info!("Received terminate signal, starting graceful shutdown");
        },
    }
}
