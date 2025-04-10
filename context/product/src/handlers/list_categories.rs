use axum::{
  extract::{Query, State},
  Json,
};
use utils::{ApiResponse, PaginatedResponse, PaginationParams, SharedState};

use crate::{domain::category::dto::Category, service::catalog_service::CatalogService};

/// List categories request (uses pagination params from query)
pub type ListCategoriesRequest = PaginationParams;

/// List categories response
pub type ListCategoriesResponse = PaginatedResponse<Category>;

/// Get all categories with pagination
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/categories/list_categories",
    params(
        ("page" = Option<u64>, Query, description = "Page number"),
        ("per_page" = Option<u64>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, response = inline(PaginatedResponse<Category>)),
        (status = 400),
        (status = 500),
    ),
    tag = "Catalog",
    summary = "Get all categories with pagination",
    description = "Get a paginated list of all categories"
)]
pub async fn list_categories(
  State(state): State<SharedState>,
  Query(req): Query<ListCategoriesRequest>,
) -> ApiResponse<Json<ListCategoriesResponse>> {
  let service = CatalogService::new();

  service
    .find_all(&state.db, req)
    .await
    .map(Json)
    .map_err(utils::AppError::from)
    .into()
}
