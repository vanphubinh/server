use axum::{extract::Path, extract::State, http::StatusCode};
use serde::Deserialize;
use utils::{no_content, ApiResponse, AppError, SharedState};
use uuid::Uuid;

use crate::service::UomService;

/// Delete UOM request (uses path param)
#[derive(Debug, Deserialize)]
pub struct DeleteUomRequest {
    pub id: Uuid,
}

/// Delete UOM has no response body (204 No Content)
pub type DeleteUomResponse = ();

/// Delete a UOM by ID
#[axum::debug_handler]
pub async fn delete_uom(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> ApiResponse<StatusCode> {
    let service = UomService::new(state.db.clone());

    match service.delete(id).await {
        Ok(true) => Ok(no_content()),
        Ok(false) => Err(AppError::not_found("UOM")),
        Err(err) => Err(AppError::from(err)),
    }
    .into()
}
