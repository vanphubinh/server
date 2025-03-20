use axum::{extract::Path, extract::State, Json};
use serde::Deserialize;
use utils::{ApiResponse, AppError, SharedState};
use uuid::Uuid;

use crate::{domain::uom::UomResponse, service::UomService};

/// Get UOM by ID request (uses path param)
#[derive(Debug, Deserialize)]
pub struct GetUomByIdRequest {
    pub id: Uuid,
}

/// Get UOM by ID response
pub type GetUomByIdResponse = UomResponse;

/// Get a UOM by ID
#[axum::debug_handler]
pub async fn get_uom_by_id(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> ApiResponse<Json<GetUomByIdResponse>> {
    let service = UomService::new(state.db.clone());

    match service.find_by_id(id).await {
        Ok(Some(uom)) => Ok(Json(uom)),
        Ok(None) => Err(AppError::not_found("UOM")),
        Err(err) => Err(AppError::from(err)),
    }
    .into()
}
