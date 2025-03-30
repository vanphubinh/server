use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use super::entity::Model;

/// Response type for a single category
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct CategoryDto {
    pub id: Uuid,
    pub name: String,
    pub parent_category_id: Option<Uuid>,
}

impl From<Model> for CategoryDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            parent_category_id: model.parent_category_id,
        }
    }
}
