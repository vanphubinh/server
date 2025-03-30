use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use super::entity::Model;

/// Response type for a single UOM
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct UomDto {
    pub id: Uuid,
    pub name: String,
}

impl From<Model> for UomDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}
