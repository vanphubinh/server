use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

/// Response type for a single unit of measurement
#[derive(Debug, Serialize, ToSchema, DerivePartialModel, FromQueryResult, ToResponse)]
#[sea_orm(entity = "<crate::domain::uom::entity::Model as ModelTrait>::Entity")]
#[serde(rename_all = "camelCase")]
pub struct Uom {
  pub id: Uuid,
  pub name: String,
}

// Implement From for Model to UnitOfMeasurement conversion
impl From<crate::domain::uom::entity::Model> for Uom {
  fn from(model: crate::domain::uom::entity::Model) -> Self {
    Self {
      id: model.id,
      name: model.name,
    }
  }
}
