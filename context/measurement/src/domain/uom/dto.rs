use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use super::entity::Model;

/// Input type for creating a new UOM
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUomInput {
    pub name: String,
}

/// Response type for a single UOM
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct UomDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Model> for UomDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_at: model.created_at.to_utc(),
            updated_at: model.updated_at.map(|dt| dt.to_utc()),
        }
    }
}
