use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// AppState holds shared resources that can be accessed by request handlers
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    /// Create a new AppState with the given database connection
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Create a shareable AppState by wrapping it in an Arc
    pub fn into_shared(self) -> SharedState {
        Arc::new(self)
    }
}

/// SharedState is an Arc-wrapped AppState that can be shared across threads
pub type SharedState = Arc<AppState>;
