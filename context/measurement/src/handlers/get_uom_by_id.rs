use axum::{extract::Path, extract::State, Json};
use serde::Deserialize;
use utils::{ApiResponse, AppError, SharedState};
use uuid::Uuid;

use crate::{domain::uom::UomDto, service::uom_service::UomService};

/// Get UOM by ID request (uses path param)
#[derive(Debug, Deserialize)]
pub struct GetUomByIdRequest {
    pub id: Uuid,
}

/// Get UOM by ID response
pub type GetUomByIdResponse = UomDto;

/// Get a UOM by ID
#[utoipa::path(
    get,
    path = "/uoms/get_uom_by_id/{id}",
    params(
        ("id" = Uuid, Path, description = "The unique identifier of the UOM to retrieve")
    ),
    responses(
        (status = 200, description = "UOM found", body = UomDto),
        (status = 404, description = "UOM not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Measurement"
)]
#[axum::debug_handler]
pub async fn get_uom_by_id(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> ApiResponse<Json<GetUomByIdResponse>> {
    let service = UomService::new();

    match service.find_by_id(&state.db, id).await {
        Ok(Some(uom)) => Ok(Json(uom)),
        Ok(None) => Err(AppError::not_found("UOM")),
        Err(err) => Err(AppError::from(err)),
    }
    .into()
}
