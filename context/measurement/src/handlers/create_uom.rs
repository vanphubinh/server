use axum::{extract::State, http::StatusCode, Json};
use utils::{created, ApiResponse, AppError, SharedState};

use crate::{
    domain::uom::{CreateUomInput, UomResponse},
    service::UomService,
};

/// Create UOM request
pub type CreateUomRequest = CreateUomInput;

/// Create UOM response
pub type CreateUomResponse = UomResponse;

/// Create a new UOM
#[axum::debug_handler]
pub async fn create_uom(
    State(state): State<SharedState>,
    Json(req): Json<CreateUomRequest>,
) -> ApiResponse<(StatusCode, Json<CreateUomResponse>)> {
    let service = UomService::new(state.db.clone());

    service
        .create(req)
        .await
        .map(|uom| created(uom))
        .map_err(AppError::from)
        .into()
}
