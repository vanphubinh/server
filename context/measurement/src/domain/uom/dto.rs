use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entity::Model;

/// Input type for creating a new UOM
#[derive(Debug, Deserialize)]
pub struct CreateUomInput {
    pub name: String,
}

/// Input type for updating an existing UOM
#[derive(Debug, Deserialize)]
pub struct UpdateUomInput {
    pub name: String,
}

/// Response type for a single UOM
#[derive(Debug, Serialize)]
pub struct UomResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

impl From<Model> for UomResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
