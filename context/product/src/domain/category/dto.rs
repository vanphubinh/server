use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

/// Response type for a single category
#[derive(Debug, Serialize, ToSchema, DerivePartialModel, FromQueryResult, ToResponse)]
#[sea_orm(entity = "<crate::domain::category::entity::Model as ModelTrait>::Entity")]
#[serde(rename_all = "camelCase")]
pub struct Category {
  pub id: Uuid,
  pub name: String,
  pub parent_category_id: Option<Uuid>,
}

// Implement From for Model to Category conversion
impl From<crate::domain::category::entity::Model> for Category {
  fn from(model: crate::domain::category::entity::Model) -> Self {
    Self {
      id: model.id,
      name: model.name,
      parent_category_id: model.parent_category_id,
    }
  }
}
