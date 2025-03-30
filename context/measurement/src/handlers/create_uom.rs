use axum::{extract::State, http::StatusCode, Json};
use utils::{created, ApiResponse, AppError, CreateResponse, SharedState};

use crate::service::uom_service::{CreateUomInput, UomService};

/// Create UOM request
pub type CreateUomRequest = CreateUomInput;

/// Create UOM response
pub type CreateUomResponse = CreateResponse;

/// Create a new UOM
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/uoms/create_uom",
    request_body = CreateUomRequest,
    responses(
        (status = 201, response = CreateUomResponse),
        (status = 400),
        (status = 500),
    ),
    tag = "Measurement",
    summary = "Create a new UOM",
    description = "Create a new UOM with the given name"
)]
pub async fn create_uom(
    State(state): State<SharedState>,
    Json(req): Json<CreateUomRequest>,
) -> ApiResponse<(StatusCode, Json<CreateUomResponse>)> {
    let service = UomService::new();

    service
        .create(&state.db, req)
        .await
        .map(|uom| created(CreateUomResponse { id: uom.id }))
        .map_err(AppError::from)
        .into()
}
