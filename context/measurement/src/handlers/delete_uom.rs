use axum::{extract::Path, extract::State, http::StatusCode};
use utils::{no_content, ApiResponse, AppError, SharedState};
use uuid::Uuid;

use crate::service::uom_service::UomService;

/// Delete a UOM by ID
#[utoipa::path(
    post,
    path = "/uoms/delete_uom/{id}",
    params(
        ("id" = Uuid, Path, description = "The unique identifier of the UOM to delete")
    ),
    responses(
        (status = 204, description = "UOM successfully deleted"),
        (status = 404, description = "UOM not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Measurement"
)]
#[axum::debug_handler]
pub async fn delete_uom(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> ApiResponse<StatusCode> {
    let service = UomService::new();

    match service.delete(&state.db, id).await {
        Ok(true) => Ok(no_content()),
        Ok(false) => Err(AppError::not_found("UOM")),
        Err(err) => Err(AppError::from(err)),
    }
    .into()
}
