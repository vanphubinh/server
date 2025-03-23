use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

/// Pagination request parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
        }
    }
}

/// Pagination response - a generic wrapper for paginated data
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct PaginatedResponse<T: ToSchema> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct PaginationMeta {
    pub total: u64,
    pub page: u64,
    #[serde(rename = "perPage")]
    pub per_page: u64,
    #[serde(rename = "totalPages")]
    pub total_pages: u64,
}

impl<T: ToSchema> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, meta: PaginationMeta) -> Self {
        Self { data, meta }
    }
}

impl PaginationMeta {
    pub fn new(total: u64, page: u64, per_page: u64) -> Self {
        let total_pages = if total > 0 {
            (total + per_page - 1) / per_page
        } else {
            0
        };

        Self {
            total,
            page,
            per_page,
            total_pages,
        }
    }
}
