use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use utils::{no_content, ApiResponse, AppError, SharedState};
use uuid::Uuid;

use crate::service::catalog_service::CatalogService;

/// Delete category by ID
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/categories/delete_category/{id}",
    params(
        ("id" = Uuid, Path, description = "Category ID")
    ),
    responses(
        (status = 204),
        (status = 404),
        (status = 500),
    ),
    tag = "Catalog",
    summary = "Delete category by ID",
    description = "Delete a category by its ID"
)]
pub async fn delete_category(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> ApiResponse<StatusCode> {
    let service = CatalogService::new();

    match service.delete(&state.db, id).await {
        Ok(true) => Ok(no_content()),
        Ok(false) => Err(AppError::not_found("Category")),
        Err(err) => Err(AppError::from(err)),
    }
    .into()
}
