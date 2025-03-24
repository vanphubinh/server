use axum::{extract::Query, extract::State, Json};
use utils::{ApiResponse, PaginatedResponse, PaginationParams, SharedState};

use crate::{domain::uom::UomDto, service::UomService};

/// List UOMs request (uses pagination params from query)
pub type ListUomsRequest = PaginationParams;

/// List UOMs response
pub type ListUomsResponse = PaginatedResponse<UomDto>;

/// List all UOMs with pagination
#[utoipa::path(
    get,
    path = "/uoms/list_uoms",
    params(
        ("page" = Option<u64>, Query, description = "Page number for pagination"),
        ("page_size" = Option<u64>, Query, description = "Number of items per page")
    ),
    responses(
        (status = 200, response = inline(PaginatedResponse<UomDto>)),
        (status = 500, description = "Internal server error")
    ),
    tag = "Measurement",
    summary = "Get all UOMs with pagination",
    description = "Get a paginated list of all UOMs"
)]
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
