use axum::{extract::Query, extract::State, Json};
use utils::{ApiResponse, PaginatedResponse, PaginationParams, SharedState};

use crate::{domain::uom::UomResponse, service::UomService};

/// List UOMs request (uses pagination params from query)
pub type ListUomsRequest = PaginationParams;

/// List UOMs response
pub type ListUomsResponse = PaginatedResponse<UomResponse>;

/// List all UOMs with pagination
#[axum::debug_handler]
pub async fn list_uoms(
    State(state): State<SharedState>,
    Query(req): Query<ListUomsRequest>,
) -> ApiResponse<Json<ListUomsResponse>> {
    let service = UomService::new(state.db.clone());

    service
        .find_all(req)
        .await
        .map(Json)
        .map_err(utils::AppError::from)
        .into()
}
