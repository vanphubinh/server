use axum::{extract::Path, extract::State, Json};
use serde::Deserialize;
use utils::{ApiResponse, AppError, SharedState};
use uuid::Uuid;

use crate::{domain::category::CategoryDto, service::catalog_service::CatalogService};

/// Get category by ID request (uses path param)
#[derive(Debug, Deserialize)]
pub struct GetCategoryByIdRequest {
    pub id: Uuid,
}

/// Get category by ID response
pub type GetCategoryByIdResponse = CategoryDto;

/// Get a category by ID
#[utoipa::path(
    get,
    path = "/categories/get_category_by_id/{id}",
    params(
        ("id" = Uuid, Path, description = "The unique identifier of the category to retrieve")
    ),
    responses(
        (status = 200, description = "Category found", body = CategoryDto),
        (status = 404, description = "Category not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Catalog"
)]
#[axum::debug_handler]
pub async fn get_category_by_id(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> ApiResponse<Json<GetCategoryByIdResponse>> {
    let service = CatalogService::new(state.db.clone());

    match service.find_by_id(id).await {
        Ok(Some(category)) => Ok(Json(category)),
        Ok(None) => Err(AppError::not_found("Category")),
        Err(err) => Err(AppError::from(err)),
    }
    .into()
}
