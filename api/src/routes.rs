use axum::{
    routing::{get, post},
    Router,
};
use measurement::handlers::{create_uom, delete_uom, get_uom_by_id, list_uoms};
use utils::SharedState;

/// Create the main API router
pub fn api_router() -> Router<SharedState> {
    Router::new().nest("/uoms", uom_router())
    // Add more context routers here as needed
}

/// Create a router for UOM operations
fn uom_router() -> Router<SharedState> {
    Router::new()
        .route("/list_uoms", get(list_uoms))
        .route("/create_uom", post(create_uom))
        .route("/get_uom_by_id/{id}", get(get_uom_by_id))
        .route("/delete_uom/{id}", post(delete_uom))
}
