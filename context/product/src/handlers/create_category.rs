use axum::{extract::State, http::StatusCode, Json};
use utils::{created, ApiResponse, AppError, CreateResponse, SharedState};

use crate::service::catalog_service::{CatalogService, CreateCategoryInput};

/// Create category request
pub type CreateCategoryRequest = CreateCategoryInput;

/// Create category response
pub type CreateCategoryResponse = CreateResponse;

/// Create a new category
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/categories/create_category",
    request_body = CreateCategoryRequest,
    responses(
        (status = 201, response = CreateCategoryResponse),
        (status = 400),
        (status = 500),
    ),
    tag = "Catalog",
    summary = "Create a new category",
    description = "Create a new category with the given name and optional parent category"
)]
pub async fn create_category(
  State(state): State<SharedState>,
  Json(req): Json<CreateCategoryRequest>,
) -> ApiResponse<(StatusCode, Json<CreateCategoryResponse>)> {
  let service = CatalogService::new();

  service
    .create(&state.db, req)
    .await
    .map(|category| created(CreateCategoryResponse { id: category.id }))
    .map_err(AppError::from)
    .into()
}
