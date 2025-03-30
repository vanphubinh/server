use sea_orm::{prelude::Decimal, DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

/// Response type for a single product
#[derive(Debug, Serialize, ToSchema, DerivePartialModel, FromQueryResult, ToResponse)]
#[sea_orm(entity = "<crate::domain::product::entity::Model as ModelTrait>::Entity")]
#[serde(rename_all = "camelCase")]
pub struct Product {
  pub id: Uuid,
  pub product_template_id: Uuid,
  pub price: Decimal,
  pub cost: Decimal,
}

// Implement From for Model to Product conversion
impl From<crate::domain::product::entity::Model> for Product {
  fn from(model: crate::domain::product::entity::Model) -> Self {
    Self {
      id: model.id,
      product_template_id: model.product_template_id,
      price: model.price,
      cost: model.cost,
    }
  }
}
