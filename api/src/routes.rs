use axum::{
    routing::{get, post},
    Router,
};
use measurement::handlers::{create_uom, delete_uom, get_uom_by_id, list_uoms};
use product::handlers::{
    create_category, create_product, delete_category, get_category_by_id, list_categories,
};
use utils::SharedState;

/// Create the main API router
pub fn api_router() -> Router<SharedState> {
    Router::new()
        .nest("/uoms", uom_router())
        .nest("/categories", category_router())
        .nest("/products", product_router())
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

/// Create a router for category operations
fn category_router() -> Router<SharedState> {
    Router::new()
        .route("/list_categories", get(list_categories))
        .route("/create_category", post(create_category))
        .route("/get_category_by_id/{id}", get(get_category_by_id))
        .route("/delete_category/{id}", post(delete_category))
}

/// Create a router for product operations
fn product_router() -> Router<SharedState> {
    Router::new().route("/create_product", post(create_product))
}
