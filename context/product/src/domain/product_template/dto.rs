use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use super::entity::Model;
use crate::product_template::{ProductSubtype, ProductType};

/// Response type for a single product
#[derive(Debug, Serialize, ToSchema, DerivePartialModel, FromQueryResult, ToResponse)]
#[sea_orm(entity = "<crate::domain::product_template::entity::Model as ModelTrait>::Entity")]
#[serde(rename_all = "camelCase")]
pub struct ProductTemplate {
  pub id: Uuid,
  pub name: String,
  pub uom_id: Uuid,
  pub product_subtype: ProductSubtype,
  pub product_type: ProductType,
  pub description: String,
  pub category_id: Option<Uuid>,
}
impl From<Model> for ProductTemplate {
  fn from(model: Model) -> Self {
    Self {
      id: model.id,
      name: model.name,
      uom_id: model.uom_id,
      product_subtype: model.product_subtype,
      description: model.description,
      product_type: model.product_type,
      category_id: model.category_id,
    }
  }
}
