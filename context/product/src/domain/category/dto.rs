use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use super::entity::Model;

/// Input type for creating a new category
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCategoryInput {
    pub name: String,
    pub parent_category_id: Option<Uuid>,
}

/// Response type for a single category
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct CategoryDto {
    pub id: Uuid,
    pub name: String,
    pub parent_category_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Model> for CategoryDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            parent_category_id: model.parent_category_id,
            created_at: model.created_at.to_utc(),
            updated_at: model.updated_at.map(|dt| dt.to_utc()),
        }
    }
}
