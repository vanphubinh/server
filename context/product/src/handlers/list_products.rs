use axum::{
  extract::{Query, State},
  Json,
};
use utils::{ApiResponse, PaginatedResponse, PaginationParams, SharedState};

use crate::{domain::product::dto::Product, service::product_service::ProductService};

/// List products request (uses pagination params from query)
pub type ListProductsRequest = PaginationParams;

/// List products response
pub type ListProductsResponse = PaginatedResponse<Product>;

/// List all products with pagination
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/products/list_products",
    params(
      ("page" = Option<u64>, Query, description = "Page number"),
      ("per_page" = Option<u64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, response = inline(PaginatedResponse<Product>)),
        (status = 500, description = "Internal server error")
    ),
    tag = "Product",
    summary = "Get all products with pagination",
    description = "Get a paginated list of all products"
)]
pub async fn list_products(
  State(state): State<SharedState>,
  Query(req): Query<ListProductsRequest>,
) -> ApiResponse<Json<ListProductsResponse>> {
  let service = ProductService::new();

  service
    .list(&state.db, req)
    .await
    .map(Json)
    .map_err(utils::AppError::from)
    .into()
}
